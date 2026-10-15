use std::path::Path;

use crate::model::{CleanFailure, CleanKind, CleanProgress, CleanReport, ScanReport};
use crate::safety::{self, Disposal};
use crate::size::older_than_days;

pub fn execute(
    home: &Path,
    report: &ScanReport,
    ids: &[String],
    progress: impl Fn(CleanProgress),
) -> Result<CleanReport, String> {
    let mut selected = Vec::new();
    for category in &report.categories {
        for item in &category.items {
            if ids.iter().any(|id| id == &item.id) {
                selected.push(item.clone());
            }
        }
    }
    if selected.is_empty() {
        return Err("No hay elementos válidos en la selección.".into());
    }

    let total = selected.len() as u32;
    let mut report = CleanReport {
        moved: 0,
        bytes: 0,
        failures: Vec::new(),
        emptied_trash: false,
        notes: Vec::new(),
    };

    let mut empty_trash = false;
    for (index, item) in selected.iter().enumerate() {
        progress(CleanProgress {
            name: item.name.clone(),
            done: index as u32,
            total,
        });
        if item.kind == CleanKind::EmptyTrash {
            empty_trash = true;
            continue;
        }

        let mut failed = false;
        if item.delete_paths.is_empty() {
            report.failures.push(CleanFailure {
                path: item.display_path.clone(),
                message: "El elemento no tiene archivos para mover.".into(),
            });
            continue;
        }
        for path in &item.delete_paths {
            match move_one(home, Path::new(path)) {
                Ok(MoveOutcome::Moved) | Ok(MoveOutcome::AlreadyGone) => {}
                Err(message) => {
                    failed = true;
                    report.failures.push(CleanFailure {
                        path: path.clone(),
                        message,
                    });
                }
            }
        }
        if !failed {
            report.moved += 1;
            report.bytes += item.bytes;
            if item
                .delete_paths
                .iter()
                .any(|path| safety::contains_container_metadata(Path::new(path)))
            {
                let note = "El resto está en la Papelera. macOS dejó un registro interno que no deja borrar.";
                if !report.notes.iter().any(|item| item == note) {
                    report.notes.push(note.into());
                }
            }
        }
    }

    if empty_trash {
        progress(CleanProgress {
            name: "Papelera".into(),
            done: total.saturating_sub(1),
            total,
        });
        match empty_user_trash() {
            Ok(()) => {
                report.emptied_trash = true;
                if let Some(item) = selected.iter().find(|item| item.kind == CleanKind::EmptyTrash) {
                    report.bytes += item.bytes;
                    report.moved += 1;
                }
            }
            Err(message) => report.failures.push(CleanFailure {
                path: "~/.Trash".into(),
                message,
            }),
        }
    }

    progress(CleanProgress {
        name: "Listo".into(),
        done: total,
        total,
    });
    Ok(report)
}

enum MoveOutcome {
    Moved,
    AlreadyGone,
}

fn move_one(home: &Path, path: &Path) -> Result<MoveOutcome, String> {
    if std::fs::symlink_metadata(path).is_err() {
        return Ok(MoveOutcome::AlreadyGone);
    }

    let downloads = home.join("Downloads");
    if path.parent() == Some(downloads.as_path()) && !older_than_days(path, 30) {
        return Err("El instalador cambió hace menos de 30 días. No se movió.".into());
    }

    let target = match safety::deletion_target(home, path) {
        Ok(target) => target,
        Err(message) if message.starts_with("Ya no está") => return Ok(MoveOutcome::AlreadyGone),
        Err(message) => return Err(message),
    };

    // Nunca la carpeta de inicio, ni una raíz permitida completa.
    if target == home || target.parent().is_none() {
        return Err("Ruta demasiado amplia.".into());
    }

    let steps = match safety::disposal_plan(home, path) {
        Ok(steps) => steps,
        Err(message) if message.starts_with("Ya no está") => return Ok(MoveOutcome::AlreadyGone),
        Err(message) => return Err(message),
    };

    let origin = safety::normalize(path);
    let mut trashed = false;
    let mut errors = Vec::new();
    for step in steps {
        match execute_step(&origin, step) {
            Ok(true) => trashed = true,
            Ok(false) => {}
            Err(message) => errors.push(message),
        }
    }
    if errors.is_empty() {
        return Ok(if trashed {
            MoveOutcome::Moved
        } else {
            MoveOutcome::AlreadyGone
        });
    }
    if trashed {
        return Err(format!(
            "Parte se movió a la Papelera, pero macOS bloqueó el resto: {}",
            errors.join(" ")
        ));
    }
    Err(errors.join(" "))
}

fn stays_inside(origin: &Path, path: &Path) -> bool {
    let origin = origin.canonicalize().unwrap_or_else(|_| origin.to_path_buf());
    let parent = path.parent().unwrap_or(path);
    let parent_root = parent.canonicalize().unwrap_or_else(|_| parent.to_path_buf());
    let candidate = match path.file_name() {
        Some(name) => parent_root.join(name),
        None => parent_root,
    };
    candidate == origin || candidate.starts_with(&origin)
}

fn execute_step(origin: &Path, step: Disposal) -> Result<bool, String> {
    match step {
        Disposal::Trash(path) => {
            if !stays_inside(origin, &path) {
                return Err("La ruta sale de la carpeta elegida.".into());
            }
            let meta = match std::fs::symlink_metadata(&path) {
                Ok(meta) => meta,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(false),
                Err(err) => {
                    return Err(format!("No se pudo revisar {}: {err}", path.display()));
                }
            };
            if safety::container_metadata(&path) {
                return Ok(false);
            }
            if meta.file_type().is_symlink() {
                return Err(format!("No se mueve un enlace: {}", path.display()));
            }
            if meta.is_dir() && safety::needs_split(&path)? {
                return trash_children(origin, &path);
            }
            match move_to_trash(&path) {
                Ok(()) => Ok(true),
                Err(message) if meta.is_dir() && looks_like_permission(&message) => {
                    trash_children(origin, &path)
                }
                Err(message) => Err(message),
            }
        }
        Disposal::Unlink(path) => {
            if !stays_inside(origin, &path) {
                return Err("La ruta sale de la carpeta elegida.".into());
            }
            let meta = match std::fs::symlink_metadata(&path) {
                Ok(meta) => meta,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(false),
                Err(err) => {
                    return Err(format!("No se pudo revisar el enlace {}: {err}", path.display()));
                }
            };
            if !meta.file_type().is_symlink() {
                return Err(format!("Se esperaba un enlace: {}", path.display()));
            }
            match std::fs::remove_file(&path) {
                Ok(()) => Ok(false),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
                Err(err) => Err(format!(
                    "No se pudo quitar el enlace {}: {err}",
                    path.display()
                )),
            }
        }
        Disposal::RemoveEmpty(path) => {
            if !stays_inside(origin, &path) {
                return Err("La ruta sale de la carpeta elegida.".into());
            }
            match std::fs::remove_dir(&path) {
                Ok(()) => Ok(false),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
                Err(_) => Ok(false),
            }
        }
    }
}

/// Finder rechazó la carpeta. Se mueven los hijos reales y se deja el registro interno.
fn trash_children(origin: &Path, dir: &Path) -> Result<bool, String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|err| format!("No se pudo revisar {}: {err}", dir.display()))?;
    let mut trashed = false;
    let mut errors = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|err| format!("No se pudo revisar {}: {err}", dir.display()))?;
        let child = entry.path();
        let meta = match std::fs::symlink_metadata(&child) {
            Ok(meta) => meta,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
            Err(err) => {
                errors.push(format!("No se pudo revisar {}: {err}", child.display()));
                continue;
            }
        };
        let step = if meta.file_type().is_symlink() {
            Disposal::Unlink(child)
        } else if safety::container_metadata(&child) {
            continue;
        } else {
            Disposal::Trash(child)
        };
        match execute_step(origin, step) {
            Ok(true) => trashed = true,
            Ok(false) => {}
            Err(message) => errors.push(message),
        }
    }
    let _ = std::fs::remove_dir(dir);
    if errors.is_empty() {
        return Ok(trashed);
    }
    Err(errors.join(" "))
}

fn move_to_trash(path: &Path) -> Result<(), String> {
    if let Err(err) = trash::delete(path) {
        let message = err.to_string();
        if !looks_like_permission(&message) {
            return Err(human_trash_error(path, &message));
        }
        #[cfg(target_os = "macos")]
        {
            use trash::macos::{DeleteMethod, TrashContextExtMacos};
            let mut ctx = trash::TrashContext::new();
            ctx.set_delete_method(DeleteMethod::NsFileManager);
            if ctx.delete(path).is_ok() {
                return Ok(());
            }
        }
        return Err(human_trash_error(path, &message));
    }
    Ok(())
}

fn looks_like_permission(message: &str) -> bool {
    let lower = message.to_lowercase();
    lower.contains("permit")
        || lower.contains("permission")
        || lower.contains("permiso")
        || lower.contains("autoriz")
        || lower.contains("-1743")
        || lower.contains("-5000")
}

fn human_trash_error(path: &Path, message: &str) -> String {
    if looks_like_permission(message) {
        format!(
            "macOS no deja mover {}. En Ajustes del Sistema, Privacidad y seguridad, activa Acceso total al disco para Cristal y permite que controle Finder. Detalle: {message}",
            path.display()
        )
    } else {
        format!("No se pudo mover {}: {message}", path.display())
    }
}

fn empty_user_trash() -> Result<(), String> {
    let output = std::process::Command::new("osascript")
        .args(["-e", "tell application \"Finder\" to empty trash"])
        .output()
        .map_err(|err| format!("No se pudo hablar con Finder: {err}"))?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stderr = stderr.trim();
    if stderr.is_empty() {
        Err("Finder no vació la Papelera. Si macOS lo pide, permite que Cristal controle Finder.".into())
    } else {
        Err(stderr.to_string())
    }
}
