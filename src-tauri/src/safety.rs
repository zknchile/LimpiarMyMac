//! Reglas de qué se puede proponer y qué se puede mover a la Papelera.
//!
//! El análisis y la limpieza usan las mismas reglas. Aunque la interfaz
//! pidiera una ruta arbitraria, aquí se rechaza.

use std::path::{Component, Path, PathBuf};

pub fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn library(home: &Path) -> PathBuf {
    home.join("Library")
}

/// Raíces donde Cristal puede proponer archivos. Nunca el directorio en sí.
fn allowed_roots(home: &Path) -> Vec<PathBuf> {
    let lib = library(home);
    vec![
        lib.join("Caches"),
        lib.join("Logs"),
        lib.join("Application Support"),
        lib.join("Preferences"),
        lib.join("Saved Application State"),
        lib.join("Containers"),
        lib.join("Group Containers"),
        lib.join("HTTPStorages"),
        lib.join("WebKit"),
        lib.join("Application Scripts"),
        lib.join("LaunchAgents"),
        lib.join("Developer/Xcode/DerivedData"),
        lib.join("Developer/Xcode/iOS DeviceSupport"),
        lib.join("Developer/CoreSimulator/Caches"),
        home.join(".npm/_cacache"),
        home.join(".gradle/caches"),
        home.join(".cargo/registry/cache"),
        home.join(".cache/pip"),
        home.join("Library/Caches/pip"),
    ]
}

fn denied_roots(home: &Path) -> Vec<PathBuf> {
    let lib = library(home);
    vec![
        home.join("Documents"),
        home.join("Desktop"),
        home.join("Pictures"),
        home.join("Movies"),
        home.join("Music"),
        home.join("Public"),
        home.join(".ssh"),
        home.join(".gnupg"),
        home.join(".aws"),
        home.join(".cargo"),
        lib.join("Keychains"),
        lib.join("Mail"),
        lib.join("Messages"),
        lib.join("Photos"),
        lib.join("Mobile Documents"),
        lib.join("CloudStorage"),
        lib.join("Accounts"),
        lib.join("Cookies"),
        lib.join("Calendars"),
        lib.join("Contacts"),
        lib.join("IdentityServices"),
        lib.join("HomeKit"),
        lib.join("Biome"),
        lib.join("IntelligencePlatform"),
        lib.join("PersonalizationPortrait"),
        lib.join("Safari"),
        lib.join("Metadata"),
        lib.join("Application Support/MobileSync"),
        lib.join("Application Support/AddressBook"),
        lib.join("Application Support/CloudDocs"),
        lib.join("Application Support/FileProvider"),
        lib.join("Application Support/CallHistoryDB"),
        lib.join("Application Support/Knowledge"),
    ]
}

fn is_cargo_cache(home: &Path, path: &Path) -> bool {
    path.starts_with(home.join(".cargo/registry/cache"))
}

/// Instaladores sueltos en Descargas: solo el archivo, no carpetas.
fn is_old_installer(home: &Path, path: &Path) -> bool {
    let downloads = home.join("Downloads");
    if path.parent() != Some(downloads.as_path()) {
        return false;
    }
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => {
            let ext = ext.to_ascii_lowercase();
            ext == "dmg" || ext == "pkg"
        }
        None => false,
    }
}

fn is_trash_child(home: &Path, path: &Path) -> bool {
    let trash = home.join(".Trash");
    path.starts_with(&trash) && path != trash
}

pub fn is_deletable(home: &Path, path: &Path) -> bool {
    let path = normalize(path);
    if !path.is_absolute() || path == home || !path.starts_with(home) {
        return false;
    }

    // El registro de Cargo sí, el resto de ~/.cargo (credenciales) no.
    let denied = denied_roots(home);
    let blocked = denied.iter().any(|root| path == *root || path.starts_with(root));
    if blocked && !is_cargo_cache(home, &path) {
        return false;
    }

    if is_old_installer(home, &path) || is_trash_child(home, &path) {
        return true;
    }

    allowed_roots(home)
        .iter()
        .any(|root| path.starts_with(root) && path != *root)
}

/// Datos de Apple fuera de cachés y registros. Esos no se proponen.
pub fn is_apple_private(home: &Path, path: &Path) -> bool {
    let caches = library(home).join("Caches");
    let logs = library(home).join("Logs");
    if path.starts_with(&caches) || path.starts_with(&logs) {
        return false;
    }
    path.components().any(|component| {
        let name = component.as_os_str().to_string_lossy().to_lowercase();
        name.starts_with("com.apple.") || name.starts_with("group.com.apple.")
    })
}

pub fn apple_name(name: &str) -> bool {
    let name = name.to_lowercase();
    name.starts_with("com.apple.") || name.starts_with("group.com.apple.")
}

const PROTECTED_SUPPORT: &[&str] = &[
    "addressbook",
    "animoji",
    "app store",
    "avatars",
    "bird",
    "caches",
    "calendar",
    "callhistorydb",
    "callhistorytransactions",
    "cloudkit",
    "clouddocs",
    "contacts",
    "controlcenter",
    "crashreporter",
    "differentialprivacy",
    "diskimages",
    "dock",
    "facetime",
    "fileprovider",
    "fonts",
    "gamekit",
    "homekit",
    "icdd",
    "icloud",
    "icloudmailagent",
    "identityservices",
    "identityservicesd",
    "knowledge",
    "mail",
    "messages",
    "metadata",
    "mobilesync",
    "music",
    "news",
    "notes",
    "passkit",
    "photos",
    "podcasts",
    "reminders",
    "siri",
    "shortcuts",
    "spotlight",
    "stickers",
    "stocks",
    "syncservices",
    "tv",
    "wallet",
    "appplaceholdersyncd",
    "contactsd",
    "homeenergyd",
    "intelligenceflow",
    "locationaccessstored",
    "networkserviceproxy",
    "privatecloudcomputed",
    "stickersd",
    "summary-events",
    "tipsd",
    "videosubscriptionsd",
    "sesstorage",
    "com.apple.sharedfilelist",
    "com.apple.controlcenter",
];

pub fn protected_support_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    if apple_name(name) || lower.starts_with("default.store") {
        return true;
    }
    PROTECTED_SUPPORT.iter().any(|item| *item == lower)
}

/// Confirma que una ruta del informe se puede mover. No sigue enlaces
/// que salgan de las carpetas permitidas.
pub fn deletion_target(home: &Path, path: &Path) -> Result<PathBuf, String> {
    let path = normalize(path);
    let meta = std::fs::symlink_metadata(&path).map_err(|_| {
        format!("Ya no está: {}", path.display())
    })?;

    if !is_deletable(home, &path) {
        return Err(format!("Ruta no permitida: {}", path.display()));
    }
    if is_apple_private(home, &path) {
        return Err("No se tocan datos del sistema de Apple.".into());
    }

    if meta.file_type().is_symlink() {
        return Ok(path);
    }

    if let Ok(canonical) = path.canonicalize() {
        if !is_deletable(home, &canonical) || is_apple_private(home, &canonical) {
            return Err(format!(
                "La ruta sale de una carpeta permitida: {}",
                path.display()
            ));
        }
        return Ok(canonical);
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> PathBuf {
        PathBuf::from("/Users/test")
    }

    #[test]
    fn allows_cache_child_and_rejects_documents() {
        let home = home();
        assert!(is_deletable(
            &home,
            Path::new("/Users/test/Library/Caches/Google")
        ));
        assert!(!is_deletable(&home, Path::new("/Users/test/Documents/nota.txt")));
        assert!(!is_deletable(&home, Path::new("/Users/test/Library/Keychains/login.keychain-db")));
        assert!(!is_deletable(&home, Path::new("/Users/test")));
        assert!(!is_deletable(&home, Path::new("/Users/test/Library/Caches")));
    }

    #[test]
    fn rejects_parent_escape_and_allows_installer() {
        let home = home();
        assert!(!is_deletable(
            &home,
            Path::new("/Users/test/Library/Caches/../Documents/secreto")
        ));
        assert!(is_deletable(
            &home,
            Path::new("/Users/test/Downloads/Instalador.dmg")
        ));
        assert!(!is_deletable(
            &home,
            Path::new("/Users/test/Downloads/vacaciones.zip")
        ));
        assert!(!is_deletable(
            &home,
            Path::new("/Users/test/Downloads/Carpeta/app.dmg")
        ));
    }

    #[test]
    fn apple_support_is_private_but_apple_cache_is_not() {
        let home = home();
        assert!(is_apple_private(
            &home,
            Path::new("/Users/test/Library/Containers/com.apple.mail")
        ));
        assert!(!is_apple_private(
            &home,
            Path::new("/Users/test/Library/Caches/com.apple.Safari")
        ));
        assert!(protected_support_name("MobileSync"));
        assert!(protected_support_name("com.apple.spotlight"));
        assert!(!protected_support_name("Slack"));
    }
}
