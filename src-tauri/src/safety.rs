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
///
/// La identidad es la app o el archivo bajo una raiz permitida, no cada
/// hijo. `com.apple.security.plist` dentro de UTM es una copia de esa app,
/// no el Llavero ni las preferencias del sistema.
pub fn is_apple_private(home: &Path, path: &Path) -> bool {
    let path = normalize(path);
    let caches = library(home).join("Caches");
    let logs = library(home).join("Logs");
    if path.starts_with(&caches) || path.starts_with(&logs) {
        return false;
    }
    if let Some(identity) = leftover_identity(home, &path) {
        return apple_name(&identity);
    }
    path.components().any(|component| apple_name(&component.as_os_str().to_string_lossy()))
}

fn leftover_identity(home: &Path, path: &Path) -> Option<String> {
    let lib = library(home);
    let prefs = lib.join("Preferences");
    if let Ok(rest) = path.strip_prefix(&prefs) {
        let mut parts = rest.components();
        let first = parts.next()?.as_os_str().to_string_lossy().to_string();
        if first.eq_ignore_ascii_case("ByHost") {
            return parts.next().map(|part| part.as_os_str().to_string_lossy().to_string());
        }
        if first.is_empty() {
            return None;
        }
        return Some(first);
    }
    for folder in [
        "Containers",
        "Group Containers",
        "Application Support",
        "Saved Application State",
        "HTTPStorages",
        "WebKit",
        "Application Scripts",
        "LaunchAgents",
    ] {
        let root = lib.join(folder);
        if let Ok(rest) = path.strip_prefix(&root) {
            return rest
                .components()
                .next()
                .map(|part| part.as_os_str().to_string_lossy().to_string())
                .filter(|name| !name.is_empty());
        }
    }
    None
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

/// Un paso para vaciar una carpeta sin seguir enlaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposal {
    /// Archivo o carpeta real, sin enlaces hacia fuera.
    Trash(PathBuf),
    /// Quitar el enlace. Nunca su destino.
    Unlink(PathBuf),
    /// Carpeta que ya no tiene hijos reales. No es la Papelera.
    RemoveEmpty(PathBuf),
}

/// Parte una ruta en pasos seguros. Una carpeta con enlaces hacia fuera
/// no se entrega entera a Finder: Finder puede rechazarla o resolverlos.
pub fn disposal_plan(home: &Path, root: &Path) -> Result<Vec<Disposal>, String> {
    let root = normalize(root);
    if !is_deletable(home, &root) || is_apple_private(home, &root) {
        return Err(format!("Ruta no permitida: {}", root.display()));
    }
    let mut steps = Vec::new();
    plan_node(home, &root, &root, &mut steps)?;
    Ok(steps)
}

fn plan_node(home: &Path, origin: &Path, path: &Path, steps: &mut Vec<Disposal>) -> Result<(), String> {
    let path = normalize(path);
    if path != *origin && !path.starts_with(origin) {
        return Err("La ruta sale de la carpeta elegida.".into());
    }

    let meta = std::fs::symlink_metadata(&path)
        .map_err(|_| format!("Ya no está: {}", path.display()))?;
    let kind = meta.file_type();
    if kind.is_symlink() {
        steps.push(Disposal::Unlink(path));
        return Ok(());
    }
    if kind.is_file() {
        if container_metadata(&path) {
            return Ok(());
        }
        steps.push(Disposal::Trash(checked_target(home, origin, &path)?));
        return Ok(());
    }
    if !kind.is_dir() {
        return Ok(());
    }

    // Un enlace o el registro de containermanagerd harían fallar el movimiento
    // de la carpeta entera. Se parte para no bloquear el resto.
    if !needs_split(&path)? {
        steps.push(Disposal::Trash(checked_target(home, origin, &path)?));
        return Ok(());
    }

    let entries = std::fs::read_dir(&path)
        .map_err(|err| format!("No se pudo revisar {}: {err}", path.display()))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("No se pudo revisar {}: {err}", path.display()))?;
        plan_node(home, origin, &entry.path(), steps)?;
    }
    steps.push(Disposal::RemoveEmpty(path));
    Ok(())
}

fn checked_target(home: &Path, origin: &Path, path: &Path) -> Result<PathBuf, String> {
    let target = deletion_target(home, path)?;
    let origin_root = origin.canonicalize().unwrap_or_else(|_| origin.to_path_buf());
    let target_root = target.canonicalize().unwrap_or_else(|_| target.clone());
    if target_root != origin_root && !target_root.starts_with(&origin_root) {
        return Err(format!(
            "La ruta sale de la carpeta elegida: {}",
            path.display()
        ));
    }
    Ok(target)
}

/// Registro que containermanagerd no deja mover. No es un archivo de la persona.
pub(crate) fn container_metadata(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    name.starts_with(".com.apple.containermanagerd.") && name.ends_with(".plist")
}

/// Hay que partir la carpeta: contiene un enlace o un registro que macOS no deja mover.
pub(crate) fn needs_split(path: &Path) -> Result<bool, String> {
    let meta = std::fs::symlink_metadata(path)
        .map_err(|_| format!("Ya no está: {}", path.display()))?;
    if meta.file_type().is_symlink() || container_metadata(path) {
        return Ok(true);
    }
    if !meta.is_dir() {
        return Ok(false);
    }
    let entries = std::fs::read_dir(path)
        .map_err(|err| format!("No se pudo revisar {}: {err}", path.display()))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("No se pudo revisar {}: {err}", path.display()))?;
        if needs_split(&entry.path())? {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(crate) fn contains_container_metadata(path: &Path) -> bool {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return false;
    };
    if meta.file_type().is_symlink() {
        return false;
    }
    if container_metadata(path) {
        return true;
    }
    if !meta.is_dir() {
        return false;
    }
    let Ok(entries) = std::fs::read_dir(path) else {
        return false;
    };
    entries
        .flatten()
        .any(|entry| contains_container_metadata(&entry.path()))
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
        return Err(format!(
            "No se tocan datos del sistema de Apple: {}",
            path.display()
        ));
    }

    if meta.file_type().is_symlink() {
        return Ok(path);
    }

    if let Ok(canonical) = path.canonicalize() {
        let home_root = home.canonicalize().unwrap_or_else(|_| normalize(home));
        if !is_deletable(&home_root, &canonical) || is_apple_private(&home_root, &canonical) {
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

    #[test]
    fn container_plan_trashes_real_files_and_unlinks_escaping_links() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let home = std::env::temp_dir().join(format!("cristal-safety-{stamp}"));
        let container = home.join("Library/Containers/com.example.app");
        let documents = container.join("Data/Documents");
        let outside = home.join("Desktop");
        std::fs::create_dir_all(&documents).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(documents.join("nota.txt"), b"vm").unwrap();
        std::fs::write(outside.join("secreto.txt"), b"no tocar").unwrap();
        std::os::unix::fs::symlink(outside.join("secreto.txt"), container.join("Data/Desktop")).unwrap();

        let steps = disposal_plan(&home, &container).unwrap();
        let trashed: Vec<_> = steps
            .iter()
            .filter_map(|step| match step {
                Disposal::Trash(path) => Some(path.as_path()),
                _ => None,
            })
            .collect();
        let unlinked: Vec<_> = steps
            .iter()
            .filter_map(|step| match step {
                Disposal::Unlink(path) => Some(path.as_path()),
                _ => None,
            })
            .collect();

        assert!(trashed.iter().any(|path| path.ends_with("Data/Documents")));
        assert!(unlinked.iter().any(|path| path.ends_with("Data/Desktop")));
        assert!(steps.iter().all(|step| {
            let path = match step {
                Disposal::Trash(path) | Disposal::Unlink(path) | Disposal::RemoveEmpty(path) => path,
            };
            !path.starts_with(&outside)
        }));
        assert!(!steps.iter().any(|step| matches!(step, Disposal::Trash(path) if path == &container)));

        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn nested_apple_file_in_third_party_container_is_not_private() {
        let home = home();
        assert!(!is_apple_private(
            &home,
            Path::new("/Users/test/Library/Containers/com.utmapp.QEMUHelper/Data/Library/Preferences/com.apple.security.plist")
        ));
        assert!(is_apple_private(
            &home,
            Path::new("/Users/test/Library/Preferences/com.apple.security.plist")
        ));
        assert!(is_apple_private(
            &home,
            Path::new("/Users/test/Library/Preferences/ByHost/com.apple.finder.plist")
        ));
        assert!(is_apple_private(
            &home,
            Path::new("/Users/test/Library/Containers/com.apple.mail/Data/Library")
        ));
    }

    #[test]
    fn container_plan_skips_containermanager_metadata() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let home = std::env::temp_dir().join(format!("cristal-meta-{stamp}"));
        let container = home.join("Library/Containers/com.example.app");
        let documents = container.join("Data/Documents");
        std::fs::create_dir_all(&documents).unwrap();
        std::fs::write(documents.join("nota.txt"), b"vm").unwrap();
        std::fs::write(
            container.join(".com.apple.containermanagerd.metadata.plist"),
            vec![0; 30_000],
        )
        .unwrap();

        let steps = disposal_plan(&home, &container).unwrap();
        assert!(steps.iter().all(|step| {
            let path = match step {
                Disposal::Trash(path) | Disposal::Unlink(path) | Disposal::RemoveEmpty(path) => path,
            };
            !container_metadata(path)
        }));
        assert!(steps.iter().any(|step| matches!(
            step,
            Disposal::Trash(path) if path.ends_with("Data") || path.ends_with("Data/Documents")
        )));
        assert!(!steps.iter().any(|step| matches!(step, Disposal::Trash(path) if path == &container)));

        let _ = std::fs::remove_dir_all(&home);
    }
}
