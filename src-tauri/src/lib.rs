mod apps;
mod clean;
mod model;
mod safety;
mod scan;
mod size;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager, State};

struct AppState {
    report: Mutex<Option<model::ScanReport>>,
    busy: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            report: Mutex::new(None),
            busy: AtomicBool::new(false),
        })
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_background_color(Some(tauri::window::Color(0, 0, 0, 0)));
                #[cfg(target_os = "macos")]
                apply_liquid_glass(&window);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![scan, clean, reveal, open_trash, quit])
        .run(tauri::generate_context!())
        .expect("no se pudo abrir Cristal");
}

#[cfg(target_os = "macos")]
fn apply_liquid_glass(window: &tauri::WebviewWindow) {
    use window_vibrancy::{
        apply_liquid_glass, apply_vibrancy, LiquidGlassOptions, NSGlassEffectViewStyle,
        NSVisualEffectMaterial, NSVisualEffectState,
    };

    let options = LiquidGlassOptions::new(NSGlassEffectViewStyle::Regular)
        .radius(26.0)
        .opaque(false)
        .interactive(true);
    if let Err(err) = apply_liquid_glass(window, options) {
        eprintln!("Liquid Glass no disponible ({err}). Uso vibrancy.");
        let _ = apply_vibrancy(
            window,
            NSVisualEffectMaterial::HudWindow,
            Some(NSVisualEffectState::Active),
            Some(26.0),
        );
    }
}

#[tauri::command]
fn quit(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
async fn scan(app: AppHandle, state: State<'_, AppState>) -> Result<model::ScanReport, String> {
    if state.busy.swap(true, Ordering::SeqCst) {
        return Err("Ya hay una tarea en curso.".into());
    }
    let app_handle = app.clone();
    let scanned = tauri::async_runtime::spawn_blocking(move || {
        scan::run(|progress| {
            let _ = app_handle.emit("scan-progress", &progress);
        })
    })
    .await;

    state.busy.store(false, Ordering::SeqCst);
    let report = scanned.map_err(|err| err.to_string())??;
    *state.report.lock().unwrap_or_else(|err| err.into_inner()) = Some(report.clone());
    Ok(report)
}

#[tauri::command]
async fn clean(
    app: AppHandle,
    state: State<'_, AppState>,
    ids: Vec<String>,
) -> Result<model::CleanReport, String> {
    if ids.is_empty() {
        return Err("No hay nada seleccionado.".into());
    }
    if state.busy.swap(true, Ordering::SeqCst) {
        return Err("Ya hay una tarea en curso.".into());
    }

    let report = state
        .report
        .lock()
        .unwrap_or_else(|err| err.into_inner())
        .clone();
    let Some(report) = report else {
        state.busy.store(false, Ordering::SeqCst);
        return Err("Primero analiza la Mac.".into());
    };
    let home = match dirs::home_dir() {
        Some(home) => home,
        None => {
            state.busy.store(false, Ordering::SeqCst);
            return Err("No se encontró tu carpeta de usuario.".into());
        }
    };

    let app_handle = app.clone();
    let cleaned = tauri::async_runtime::spawn_blocking(move || {
        clean::execute(&home, &report, &ids, |progress| {
            let _ = app_handle.emit("clean-progress", &progress);
        })
    })
    .await;

    state.busy.store(false, Ordering::SeqCst);
    cleaned.map_err(|err| err.to_string())?
}

#[tauri::command]
fn reveal(path: String) -> Result<(), String> {
    let home = dirs::home_dir().ok_or("No se encontró tu carpeta de usuario.")?;
    let path = std::path::PathBuf::from(&path);
    if std::fs::symlink_metadata(&path).is_err() {
        return Err("Ese archivo ya no está.".into());
    }
    let check = path.canonicalize().unwrap_or(path.clone());
    if !check.starts_with(&home) && !path.starts_with(&home) {
        return Err("Solo se pueden mostrar archivos de tu usuario.".into());
    }
    let status = std::process::Command::new("open")
        .args(["-R"])
        .arg(&path)
        .status()
        .map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("No se pudo mostrar el archivo en el Finder.".into())
    }
}

#[tauri::command]
fn open_trash() -> Result<(), String> {
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
