//! Análisis de la carpeta del usuario.
//!
//! Solo propone rutas que `safety` aceptaría borrar. Cachés y registros
//! vienen marcados. Restos de apps, perfiles y la Papelera no.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;

use crate::apps::{self, Inventory};
use crate::model::{Category, CleanItem, CleanKind, Risk, ScanProgress, ScanReport};
use crate::safety::{self, apple_name, protected_support_name};
use crate::size::{modified_secs, older_than_days, path_size};

const STAGE_COUNT: u32 = 7;
const MIN_PRESELECT: u64 = 512 * 1024;

const PROFILE_CACHES: &[&str] = &[
    "Cache",
    "Code Cache",
    "GPUCache",
    "Service Worker/CacheStorage",
    "Service Worker/ScriptCache",
    "DawnGraphiteCache",
    "DawnWebGPUCache",
    "DawnCache",
    "GrShaderCache",
    "ShaderCache",
];

const COOKIE_FILES: &[&str] = &[
    "Cookies",
    "Cookies-journal",
    "Network/Cookies",
    "Network/Cookies-journal",
];

struct ScanCtx<'a> {
    home: &'a Path,
    inv: &'a Inventory,
    progress: &'a dyn Fn(ScanProgress),
    found: u64,
    ids: u64,
    notes: Vec<String>,
}

impl ScanCtx<'_> {
    fn emit(&self, stage_index: u32, stage: &str, detail: &str) {
        (self.progress)(ScanProgress {
            stage: stage.to_string(),
            detail: detail.to_string(),
            stage_index,
            stage_count: STAGE_COUNT,
            found_bytes: self.found,
        });
    }

    fn next_id(&mut self, category: &str) -> String {
        self.ids += 1;
        format!("{category}-{}", self.ids)
    }
}

pub fn run(progress: impl Fn(ScanProgress)) -> Result<ScanReport, String> {
    let started = Instant::now();
    let home = dirs::home_dir().ok_or("No se encontró tu carpeta de usuario.")?;
    let inventory = apps::inventory();
    let mut ctx = ScanCtx {
        home: &home,
        inv: &inventory,
        progress: &progress,
        found: 0,
        ids: 0,
        notes: vec![
            "No se tocan Documentos, Fotos, el Llavero, contraseñas, marcadores ni el historial.".into(),
            "Lo que limpies va a la Papelera, salvo que elijas vaciarla.".into(),
        ],
    };

    ctx.emit(
        1,
        "Apps instaladas",
        &format!("{} apps para no confundirlas con restos", inventory.app_count),
    );

    ctx.emit(2, "Navegadores", "Cachés temporales, sin perfiles");
    let (browsers, claimed_caches, browser_roots) = scan_browsers(&mut ctx);

    ctx.emit(3, "Cachés", "Archivos que las apps pueden volver a crear");
    let caches = scan_caches(&mut ctx, &claimed_caches);

    ctx.emit(4, "Apps desinstaladas", "Carpetas que ya no coinciden con una app");
    let leftovers = scan_leftovers(&mut ctx, &browser_roots);

    ctx.emit(5, "Registros", "Logs que ya cumplieron su función");
    let logs = scan_logs(&mut ctx);

    ctx.emit(6, "Desarrollo", "Xcode, Homebrew, npm y similares");
    let dev = scan_dev(&mut ctx);

    ctx.emit(7, "Descargas y Papelera", "Instaladores viejos y lo que ya tiraste");
    let installers = scan_installers(&mut ctx);
    let trash = scan_trash(&mut ctx);

    let mut categories = vec![
        Category {
            id: "browsers".into(),
            title: "Navegadores".into(),
            description: "Caché y archivos temporales. No incluye contraseñas, marcadores ni historial.".into(),
            items: browsers,
        },
        Category {
            id: "caches".into(),
            title: "Cachés de apps".into(),
            description: "Datos temporales. Si una app los necesita, los vuelve a crear.".into(),
            items: caches,
        },
        Category {
            id: "leftovers".into(),
            title: "Restos de apps".into(),
            description: "Carpetas y preferencias de apps que no están instaladas. Revísalas: a veces guardan documentos.".into(),
            items: leftovers,
        },
        Category {
            id: "logs".into(),
            title: "Registros".into(),
            description: "Informes y logs. No son tus documentos.".into(),
            items: logs,
        },
        Category {
            id: "dev".into(),
            title: "Desarrollo".into(),
            description: "Cachés de Xcode, Homebrew, npm, Cargo y similares. No vienen marcados por si estás compilando.".into(),
            items: dev,
        },
        Category {
            id: "installers".into(),
            title: "Instaladores".into(),
            description: "Archivos .dmg y .pkg en Descargas con más de 30 días.".into(),
            items: installers,
        },
        Category {
            id: "trash".into(),
            title: "Papelera".into(),
            description: "Vaciarla libera espacio de verdad. Esta acción no se puede deshacer.".into(),
            items: trash,
        },
    ];

    for category in &mut categories {
        category.items.sort_by(|a, b| b.bytes.cmp(&a.bytes));
        category.items.retain(|item| item.bytes > 0 || item.kind == CleanKind::EmptyTrash);
    }
    categories.retain(|category| !category.items.is_empty());

    ctx.emit(STAGE_COUNT, "Listo", "Revisa la selección antes de limpiar");

    Ok(ScanReport {
        categories,
        notes: ctx.notes,
        elapsed_ms: started.elapsed().as_millis() as u64,
        apps_found: inventory.app_count,
    })
}

fn scan_browsers(ctx: &mut ScanCtx) -> (Vec<CleanItem>, HashSet<PathBuf>, HashSet<PathBuf>) {
    let mut claimed = HashSet::new();
    let mut roots = HashSet::new();
    let mut grouped: HashMap<String, (Vec<PathBuf>, Vec<PathBuf>)> = HashMap::new();

    for root in chromium_roots(ctx.home) {
        roots.insert(root.clone());
        let name = friendly_browser_name(&root);
        let entry = grouped.entry(name.clone()).or_default();
        collect_chromium(&root, &mut entry.0, &mut entry.1);
        for extra in extra_cache_dirs(ctx.home, &name) {
            claimed.insert(extra.clone());
            entry.0.push(extra);
        }
    }

    let safari_caches = [
        "Library/Caches/com.apple.Safari",
        "Library/Caches/com.apple.WebKit.Networking",
        "Library/Caches/com.apple.WebKit.WebContent",
        "Library/Caches/com.apple.WebKit.GPU",
    ];
    let mut safari = Vec::new();
    for relative in safari_caches {
        let path = ctx.home.join(relative);
        if path.exists() {
            claimed.insert(path.clone());
            safari.push(path);
        }
    }
    if !safari.is_empty() {
        grouped.entry("Safari".into()).or_default().0.extend(safari);
    }

    let firefox_root = ctx.home.join("Library/Application Support/Firefox");
    if firefox_root.exists() {
        roots.insert(firefox_root.clone());
    }
    let mut firefox_caches = Vec::new();
    let mut firefox_cookies = Vec::new();
    let firefox_cache = ctx.home.join("Library/Caches/Firefox");
    if firefox_cache.exists() {
        claimed.insert(firefox_cache.clone());
        firefox_caches.push(firefox_cache);
    }
    if let Ok(profiles) = fs::read_dir(firefox_root.join("Profiles")) {
        for profile in profiles.flatten() {
            let path = profile.path();
            for relative in ["cache2", "startupCache", "shader-cache"] {
                let cache = path.join(relative);
                if cache.exists() {
                    firefox_caches.push(cache);
                }
            }
            for relative in ["cookies.sqlite", "cookies.sqlite-wal", "cookies.sqlite-shm"] {
                let cookie = path.join(relative);
                if cookie.is_file() {
                    firefox_cookies.push(cookie);
                }
            }
        }
    }
    if !firefox_caches.is_empty() || !firefox_cookies.is_empty() {
        let entry = grouped.entry("Firefox".into()).or_default();
        entry.0.extend(firefox_caches);
        entry.1.extend(firefox_cookies);
    }

    let mut items = Vec::new();
    let mut names: Vec<_> = grouped.keys().cloned().collect();
    names.sort();
    for name in names {
        let (caches, cookies) = grouped.remove(&name).unwrap_or_default();
        push_group(
            ctx,
            &mut items,
            "browsers",
            &name,
            &caches,
            Risk::Safe,
            true,
            "Caché y datos temporales. La próxima visita los vuelve a crear.",
        );
        push_group(
            ctx,
            &mut items,
            "browsers",
            &format!("Cookies de {name}"),
            &cookies,
            Risk::Review,
            false,
            "Tendrás que volver a iniciar sesión en las páginas. No borra contraseñas guardadas ni marcadores.",
        );
    }

    (items, claimed, roots)
}

fn chromium_roots(home: &Path) -> Vec<PathBuf> {
    let support = home.join("Library/Application Support");
    let mut roots = Vec::new();
    for path in list_dir(&support) {
        if path.join("Local State").is_file() {
            roots.push(path);
            continue;
        }
        if !is_real_dir(&path) {
            continue;
        }
        for child in list_dir(&path) {
            if child.join("Local State").is_file() {
                roots.push(child);
            }
        }
    }
    roots.sort();
    roots.dedup();
    roots
}

fn friendly_browser_name(path: &Path) -> String {
    let raw = file_name(path);
    match raw.as_str() {
        "Chrome" => "Google Chrome".into(),
        "Chrome Canary" => "Chrome Canary".into(),
        "Brave-Browser" => "Brave".into(),
        "Microsoft Edge" | "Microsoft Edge Canary" => raw,
        "User Data" => path
            .parent()
            .map(file_name)
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| "Navegador".into()),
        other => other.to_string(),
    }
}

fn extra_cache_dirs(home: &Path, browser: &str) -> Vec<PathBuf> {
    let caches = home.join("Library/Caches");
    let names: &[&str] = match browser {
        "Google Chrome" | "Chrome Canary" => &["Google"],
        "Brave" => &["BraveSoftware"],
        "Microsoft Edge" => &["Microsoft Edge", "com.microsoft.Edge"],
        "Microsoft Edge Canary" => &["Microsoft Edge Canary"],
        "Arc" => &["Arc", "company.thebrowser.Browser"],
        "Firefox" => &["Firefox"],
        "Opera" => &["com.operasoftware.Opera"],
        "Vivaldi" => &["Vivaldi"],
        _ => &[],
    };
    names
        .iter()
        .map(|name| caches.join(name))
        .filter(|path| path.exists())
        .collect()
}

fn collect_chromium(root: &Path, caches: &mut Vec<PathBuf>, cookies: &mut Vec<PathBuf>) {
    for relative in ["ShaderCache", "GrShaderCache", "GraphiteDawnCache", "GPUCache"] {
        let path = root.join(relative);
        if path.exists() {
            caches.push(path);
        }
    }
    for profile in list_dir(root) {
        if !is_profile(&profile) {
            continue;
        }
        for relative in PROFILE_CACHES {
            let path = profile.join(relative);
            if path.exists() {
                caches.push(path);
            }
        }
        for relative in COOKIE_FILES {
            let path = profile.join(relative);
            if path.is_file() {
                cookies.push(path);
            }
        }
    }
}

fn is_profile(path: &Path) -> bool {
    if !is_real_dir(path) {
        return false;
    }
    let name = file_name(path);
    name == "Default"
        || name == "Guest Profile"
        || name == "System Profile"
        || name.starts_with("Profile ")
        || path.join("Preferences").is_file()
}

fn scan_caches(ctx: &mut ScanCtx, claimed: &HashSet<PathBuf>) -> Vec<CleanItem> {
    let root = ctx.home.join("Library/Caches");
    let paths: Vec<_> = list_dir(&root)
        .into_iter()
        .filter(|path| {
            let name = file_name(path);
            if name.starts_with('.')
                || name == "com.apple.Safari.SafeBrowsing"
                || matches!(
                    name.as_str(),
                    "Homebrew"
                        | "pip"
                        | "Yarn"
                        | "pnpm"
                        | "go-build"
                        | "org.swift.swiftpm"
                        | "com.apple.dt.Xcode"
                )
            {
                return false;
            }
            !claimed.iter().any(|claim| path == claim || path.starts_with(claim))
        })
        .collect();

    let mut items = Vec::new();
    for chunk in paths.chunks(6) {
        let sized = measure(chunk);
        for (path, bytes) in sized {
            let name = file_name(&path);
            ctx.emit(3, "Cachés", &name);
            if bytes < 4096 || !can_offer(ctx.home, &path) {
                continue;
            }
            ctx.found += bytes;
            let id = ctx.next_id("caches");
            items.push(make_item(
                id,
                name,
                display_path(ctx.home, &path),
                vec![path.clone()],
                bytes,
                Risk::Safe,
                bytes >= MIN_PRESELECT,
                "Caché regenerable. La app la volverá a crear si la necesita.",
                CleanKind::MoveToTrash,
                modified_secs(&path),
            ));
        }
    }
    items
}

fn scan_leftovers(ctx: &mut ScanCtx, browser_roots: &HashSet<PathBuf>) -> Vec<CleanItem> {
    let mut items = Vec::new();
    let support = ctx.home.join("Library/Application Support");
    ctx.emit(4, "Apps desinstaladas", "Application Support");
    for path in list_dir(&support) {
        consider_support(&path, ctx, browser_roots, &mut items, 0);
    }

    scan_id_dir(
        ctx,
        "Library/Saved Application State",
        Risk::Safe,
        true,
        "Estado de ventanas de una app que ya no está. No son documentos.",
        &mut items,
    );
    scan_id_dir(
        ctx,
        "Library/Containers",
        Risk::Review,
        false,
        "Contenedor de una app que no está instalada. Puede incluir documentos. Los enlaces a tu Escritorio, Descargas o Llavero no se mueven. macOS puede conservar un registro interno de unos pocos KB.",
        &mut items,
    );
    scan_id_dir(
        ctx,
        "Library/Group Containers",
        Risk::Review,
        false,
        "Datos compartidos de una app que no está instalada.",
        &mut items,
    );
    scan_id_dir(
        ctx,
        "Library/HTTPStorages",
        Risk::Review,
        false,
        "Almacenamiento web de una app que no está instalada.",
        &mut items,
    );
    scan_id_dir(
        ctx,
        "Library/WebKit",
        Risk::Review,
        false,
        "Datos web de una app que no está instalada.",
        &mut items,
    );
    scan_id_dir(
        ctx,
        "Library/Application Scripts",
        Risk::Safe,
        false,
        "Scripts de una app de sandbox que ya no está.",
        &mut items,
    );

    let mut prefs = Vec::new();
    prefs.extend(orphan_plists(&ctx.home.join("Library/Preferences"), ctx.inv));
    prefs.extend(orphan_plists(
        &ctx.home.join("Library/Preferences/ByHost"),
        ctx.inv,
    ));
    prefs.retain(|path| can_offer(ctx.home, path));
    if !prefs.is_empty() {
        let bytes: u64 = prefs.iter().map(|path| path_size(path)).sum();
        if bytes > 0 {
            ctx.found += bytes;
            let id = ctx.next_id("leftovers");
            items.push(make_item(
                id,
                format!("Preferencias huérfanas ({})", prefs.len()),
                "~/Library/Preferences".into(),
                prefs,
                bytes,
                Risk::Review,
                false,
                "Pueden ser de una app desinstalada o de una herramienta de terminal. No vienen marcadas.",
                CleanKind::MoveToTrash,
                None,
            ));
        }
    }

    for path in list_dir(&ctx.home.join("Library/LaunchAgents")) {
        let name = file_name(&path);
        if !name.ends_with(".plist") || apple_name(&name) {
            continue;
        }
        let Some(missing) = missing_launch_program(&path) else {
            continue;
        };
        if !can_offer(ctx.home, &path) {
            continue;
        }
        let bytes = path_size(&path).max(1);
        ctx.found += bytes;
        let id = ctx.next_id("leftovers");
        items.push(make_item(
            id,
            name,
            display_path(ctx.home, &path),
            vec![path],
            bytes,
            Risk::Review,
            false,
            &format!("Agente de inicio cuyo programa ya no existe: {missing}"),
            CleanKind::MoveToTrash,
            None,
        ));
    }

    items
}

fn consider_support(
    path: &Path,
    ctx: &mut ScanCtx,
    browser_roots: &HashSet<PathBuf>,
    items: &mut Vec<CleanItem>,
    depth: u32,
) {
    let name = file_name(path);
    if name.starts_with('.') || protected_support_name(&name) || apple_name(&name) {
        return;
    }
    if ctx.inv.owns_loose(&name) || ctx.inv.owns_identifier(&name) {
        return;
    }

    if is_real_dir(path) && depth == 0 {
        let children = list_dir(path);
        let vendor = ctx.inv.is_vendor_prefix(&name)
            || children.iter().any(|child| {
                let child_name = file_name(child);
                ctx.inv.owns_exact(&child_name)
                    || ctx.inv.owns_identifier(&child_name)
                    || protected_support_name(&child_name)
            });
        if vendor {
            for child in children {
                consider_support(&child, ctx, browser_roots, items, 1);
            }
            return;
        }
    }

    let mut reason = "No hay una app instalada que coincida con este nombre. Ábrelo antes: a veces quedan documentos.".to_string();
    if browser_roots.iter().any(|root| path == root || path.starts_with(root) || root.starts_with(path))
    {
        reason = "Parece el perfil de un navegador que ya no está instalado. Puede incluir marcadores y sesiones.".into();
    } else if path.join("Documents").is_dir() || path.join("Data/Documents").is_dir() {
        reason = "No coincide con una app instalada e incluye una carpeta Documentos. Revísala antes.".into();
    }
    propose(ctx, items, path, &name, Risk::Review, false, &reason);
}

fn scan_id_dir(
    ctx: &mut ScanCtx,
    relative: &str,
    risk: Risk,
    default_selected: bool,
    reason: &str,
    items: &mut Vec<CleanItem>,
) {
    let root = ctx.home.join(relative);
    ctx.emit(4, "Apps desinstaladas", relative);
    for path in list_dir(&root) {
        let name = file_name(&path);
        if name.starts_with('.') || apple_name(&name) || protected_support_name(&name) {
            continue;
        }
        let ident = name.trim_end_matches(".savedState");
        if ctx.inv.owns_identifier(ident) || ctx.inv.owns_exact(&name) {
            continue;
        }
        let mut reason = reason.to_string();
        if relative.ends_with("Containers") && container_has_documents(&path) {
            reason = "Contiene una carpeta Documentos. Ábrela antes de moverla.".into();
        }
        let selected = default_selected && risk == Risk::Safe && !container_has_documents(&path);
        propose(ctx, items, &path, &name, risk, selected, &reason);
    }
}

fn container_has_documents(path: &Path) -> bool {
    let docs = path.join("Data/Documents");
    docs.is_dir()
        && fs::read_dir(&docs)
            .ok()
            .and_then(|mut entries| entries.next())
            .is_some()
}

fn orphan_plists(dir: &Path, inv: &Inventory) -> Vec<PathBuf> {
    list_dir(dir)
        .into_iter()
        .filter(|path| {
            let name = file_name(path);
            if !name.ends_with(".plist") || name.ends_with(".plist.lock") {
                return false;
            }
            let stem = name.trim_end_matches(".plist");
            if apple_name(stem) || protected_support_name(stem) {
                return false;
            }
            !inv.owns_identifier(stem)
        })
        .collect()
}

fn missing_launch_program(path: &Path) -> Option<String> {
    let value = plist::Value::from_file(path).ok()?;
    let dict = value.as_dictionary()?;
    let program = if let Some(text) = dict.get("Program").and_then(|value| value.as_string()) {
        text.to_string()
    } else if let Some(args) = dict.get("ProgramArguments").and_then(|value| value.as_array()) {
        args.first()
            .and_then(|value| value.as_string())
            .unwrap_or("")
            .to_string()
    } else {
        return None;
    };
    if !program.starts_with('/') {
        return None;
    }
    if Path::new(&program).exists() {
        return None;
    }
    Some(program)
}

fn scan_logs(ctx: &mut ScanCtx) -> Vec<CleanItem> {
    let root = ctx.home.join("Library/Logs");
    let mut items = Vec::new();
    let mut loose = Vec::new();
    for path in list_dir(&root) {
        let name = file_name(&path);
        if name.starts_with('.') {
            continue;
        }
        ctx.emit(5, "Registros", &name);
        if path.is_file() {
            loose.push(path);
            continue;
        }
        let diagnostic = name.eq_ignore_ascii_case("DiagnosticReports");
        propose(
            ctx,
            &mut items,
            &path,
            &name,
            if diagnostic { Risk::Review } else { Risk::Safe },
            !diagnostic,
            if diagnostic {
                "Informes de fallos. Útiles si estás diagnosticando un problema."
            } else {
                "Registro regenerable."
            },
        );
    }
    loose.retain(|path| can_offer(ctx.home, path));
    if !loose.is_empty() {
        let bytes: u64 = loose.iter().map(|path| path_size(path)).sum();
        if bytes >= 4096 {
            ctx.found += bytes;
            let id = ctx.next_id("logs");
            items.push(make_item(
                id,
                format!("Archivos de registro sueltos ({})", loose.len()),
                "~/Library/Logs".into(),
                loose,
                bytes,
                Risk::Safe,
                true,
                "Logs sueltos en la carpeta de registros.",
                CleanKind::MoveToTrash,
                None,
            ));
        }
    }
    items
}

fn scan_dev(ctx: &mut ScanCtx) -> Vec<CleanItem> {
    let targets = [
        (
            "Xcode DerivedData",
            ctx.home.join("Library/Developer/Xcode/DerivedData"),
            "Resultados de compilación. Xcode los regenera. No viene marcado por si estás compilando.",
        ),
        (
            "Dispositivos iOS de Xcode",
            ctx.home.join("Library/Developer/Xcode/iOS DeviceSupport"),
            "Símbolos de iPhone o iPad. Se vuelven a descargar al conectar el dispositivo.",
        ),
        (
            "Caché de simuladores",
            ctx.home.join("Library/Developer/CoreSimulator/Caches"),
            "Caché del simulador. Se regenera.",
        ),
        (
            "Caché de Xcode",
            ctx.home.join("Library/Caches/com.apple.dt.Xcode"),
            "Caché de Xcode. Se regenera.",
        ),
        (
            "Homebrew",
            ctx.home.join("Library/Caches/Homebrew"),
            "Descargas de Homebrew. Se vuelven a bajar si haces brew install.",
        ),
        (
            "npm",
            ctx.home.join(".npm/_cacache"),
            "Caché de npm. No borra tus proyectos.",
        ),
        (
            "pip",
            ctx.home.join("Library/Caches/pip"),
            "Caché de pip. Los paquetes se vuelven a descargar.",
        ),
        (
            "pip",
            ctx.home.join(".cache/pip"),
            "Caché de pip. Los paquetes se vuelven a descargar.",
        ),
        (
            "Yarn",
            ctx.home.join("Library/Caches/Yarn"),
            "Caché de Yarn.",
        ),
        (
            "Gradle",
            ctx.home.join(".gradle/caches"),
            "Caché de Gradle. La próxima compilación Android tardará más.",
        ),
        (
            "Cargo",
            ctx.home.join(".cargo/registry/cache"),
            "Caché de crates. No toca credenciales ni el código de tus proyectos.",
        ),
        (
            "Go",
            ctx.home.join("Library/Caches/go-build"),
            "Caché de compilación de Go.",
        ),
        (
            "Swift Package Manager",
            ctx.home.join("Library/Caches/org.swift.swiftpm"),
            "Caché de paquetes Swift.",
        ),
    ];

    let mut items = Vec::new();
    for (name, path, reason) in targets {
        ctx.emit(6, "Desarrollo", name);
        if !path.exists() || !can_offer(ctx.home, &path) {
            continue;
        }
        let bytes = path_size(&path);
        if bytes < 4096 {
            continue;
        }
        ctx.found += bytes;
        let risk = if name.contains("iOS") {
            Risk::Review
        } else {
            Risk::Safe
        };
        let id = ctx.next_id("dev");
        items.push(make_item(
            id,
            name.into(),
            display_path(ctx.home, &path),
            vec![path.clone()],
            bytes,
            risk,
            false,
            reason,
            CleanKind::MoveToTrash,
            modified_secs(&path),
        ));
    }
    items
}

fn scan_installers(ctx: &mut ScanCtx) -> Vec<CleanItem> {
    let downloads = ctx.home.join("Downloads");
    let mut items = Vec::new();
    for path in list_dir(&downloads) {
        let name = file_name(&path);
        let ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext != "dmg" && ext != "pkg" {
            continue;
        }
        if !path.is_file() || !older_than_days(&path, 30) || !can_offer(ctx.home, &path) {
            continue;
        }
        let bytes = path_size(&path);
        if bytes == 0 {
            continue;
        }
        ctx.found += bytes;
        let id = ctx.next_id("installers");
        items.push(make_item(
            id,
            name,
            display_path(ctx.home, &path),
            vec![path.clone()],
            bytes,
            Risk::Review,
            false,
            "Instalador en Descargas con más de 30 días. No es un documento, pero revísalo.",
            CleanKind::MoveToTrash,
            modified_secs(&path),
        ));
    }
    items
}

fn scan_trash(ctx: &mut ScanCtx) -> Vec<CleanItem> {
    let trash = ctx.home.join(".Trash");
    ctx.emit(7, "Papelera", "Calculando tamaño");
    if !trash.is_dir() {
        return Vec::new();
    }
    let bytes = path_size(&trash);
    if bytes < 4096 {
        return Vec::new();
    }
    ctx.found += bytes;
    vec![make_item(
        ctx.next_id("trash"),
        "Vaciar Papelera".into(),
        "~/.Trash".into(),
        Vec::new(),
        bytes,
        Risk::Permanent,
        false,
        "Esto borra de forma permanente lo que ya está en la Papelera. Finder puede pedir confirmación.",
        CleanKind::EmptyTrash,
        None,
    )]
}

fn propose(
    ctx: &mut ScanCtx,
    items: &mut Vec<CleanItem>,
    path: &Path,
    name: &str,
    risk: Risk,
    default_selected: bool,
    reason: &str,
) {
    ctx.emit(4, "Apps desinstaladas", name);
    if !can_offer(ctx.home, path) {
        return;
    }
    let bytes = path_size(path);
    if bytes < 4096 {
        return;
    }
    ctx.found += bytes;
    let selected = default_selected && bytes >= 4096;
    let id = ctx.next_id("leftovers");
    items.push(make_item(
        id,
        name.to_string(),
        display_path(ctx.home, path),
        vec![path.to_path_buf()],
        bytes,
        risk,
        selected,
        reason,
        CleanKind::MoveToTrash,
        modified_secs(path),
    ));
}

fn push_group(
    ctx: &mut ScanCtx,
    items: &mut Vec<CleanItem>,
    category: &str,
    name: &str,
    paths: &[PathBuf],
    risk: Risk,
    preselect: bool,
    reason: &str,
) {
    let existing: Vec<_> = paths
        .iter()
        .filter(|path| path.exists() && can_offer(ctx.home, path))
        .cloned()
        .collect();
    if existing.is_empty() {
        return;
    }
    ctx.emit(2, "Navegadores", name);
    let sized = measure(&existing);
    let bytes: u64 = sized.iter().map(|(_, bytes)| *bytes).sum();
    if bytes < 4096 {
        return;
    }
    ctx.found += bytes;
    let delete_paths: Vec<PathBuf> = sized.into_iter().map(|(path, _)| path).collect();
    let id = ctx.next_id(category);
    items.push(make_item(
        id,
        name.to_string(),
        delete_paths
            .first()
            .map(|path| display_path(ctx.home, path))
            .unwrap_or_default(),
        delete_paths,
        bytes,
        risk,
        preselect && risk == Risk::Safe && bytes >= MIN_PRESELECT,
        reason,
        CleanKind::MoveToTrash,
        None,
    ));
}

fn can_offer(home: &Path, path: &Path) -> bool {
    safety::is_deletable(home, path) && !safety::is_apple_private(home, path)
}

fn make_item(
    id: String,
    name: String,
    display_path: String,
    delete_paths: Vec<PathBuf>,
    bytes: u64,
    risk: Risk,
    default_selected: bool,
    reason: &str,
    kind: CleanKind,
    modified_secs: Option<i64>,
) -> CleanItem {
    CleanItem {
        id,
        name,
        display_path,
        delete_paths: delete_paths
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect(),
        bytes,
        risk,
        reason: reason.to_string(),
        default_selected,
        kind,
        modified_secs,
    }
}

fn measure(paths: &[PathBuf]) -> Vec<(PathBuf, u64)> {
    paths
        .par_iter()
        .map(|path| (path.clone(), path_size(path)))
        .collect()
}

fn list_dir(path: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(path) else {
        return Vec::new();
    };
    let mut paths: Vec<_> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| !file_name(path).starts_with('.'))
        .collect();
    paths.sort();
    paths
}

fn is_real_dir(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|meta| meta.is_dir())
        .unwrap_or(false)
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default()
}

fn display_path(home: &Path, path: &Path) -> String {
    let home = home.to_string_lossy();
    let path = path.to_string_lossy();
    match path.strip_prefix(home.as_ref()) {
        Some(rest) => format!("~{rest}"),
        None => path.into_owned(),
    }
}
