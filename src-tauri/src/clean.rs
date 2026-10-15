use std::path::Path;

use crate::model::{CleanFailure, CleanKind, CleanProgress, CleanReport, ScanReport};
use crate::safety;
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

    trash::delete(&target).map_err(|err| err.to_string())?;
    Ok(MoveOutcome::Moved)
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
