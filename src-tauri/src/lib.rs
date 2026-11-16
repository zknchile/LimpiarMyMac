mod apps;
mod clean;
mod model;
mod place;
mod safety;
mod scan;
mod selection;
mod size;
pub mod view;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// Último informe y tarea en curso. La limpieza solo usa este informe.
pub struct Session {
    report: Mutex<Option<model::ScanReport>>,
    busy: AtomicBool,
}

impl Session {
    pub fn new() -> Self {
        Self {
            report: Mutex::new(None),
            busy: AtomicBool::new(false),
        }
    }

    pub fn try_begin(&self) -> Result<(), String> {
        if self.busy.swap(true, Ordering::SeqCst) {
            return Err("Ya hay una tarea en curso.".into());
        }
        Ok(())
    }

    pub fn end(&self) {
        self.busy.store(false, Ordering::SeqCst);
    }

    pub fn is_busy(&self) -> bool {
        self.busy.load(Ordering::SeqCst)
    }

    pub fn store_report(&self, report: model::ScanReport) {
        *self.report.lock().unwrap_or_else(|err| err.into_inner()) = Some(report);
    }

    pub fn report(&self) -> Option<model::ScanReport> {
        self.report
            .lock()
            .unwrap_or_else(|err| err.into_inner())
            .clone()
    }

    pub fn has_report(&self) -> bool {
        self.report
            .lock()
            .unwrap_or_else(|err| err.into_inner())
            .is_some()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

pub fn reveal(path: &Path) -> Result<(), String> {
    let home = dirs::home_dir().ok_or("No se encontró tu carpeta de usuario.")?;
    if std::fs::symlink_metadata(path).is_err() {
        return Err("Ese archivo ya no está.".into());
    }
    let check = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if !check.starts_with(&home) && !path.starts_with(&home) {
        return Err("Solo se pueden mostrar archivos de tu usuario.".into());
    }
    let status = std::process::Command::new("open")
        .args(["-R"])
        .arg(path)
        .status()
        .map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("No se pudo mostrar el archivo en el Finder.".into())
    }
}

pub fn open_trash() -> Result<(), String> {
    let home = dirs::home_dir().ok_or("No se encontró tu carpeta de usuario.")?;
    let status = std::process::Command::new("open")
        .arg(home.join(".Trash"))
        .status()
        .map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("No se pudo abrir la Papelera.".into())
    }
}
