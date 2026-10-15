//! Inventario de apps instaladas, para no marcar como "resto" algo que sigue en uso.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Inventory {
    exact_names: HashSet<String>,
    bundle_ids: Vec<String>,
    bins: HashSet<String>,
    pub app_count: usize,
}

impl Inventory {
    pub fn owns_exact(&self, name: &str) -> bool {
        let key = normalize(name);
        !key.is_empty() && (self.exact_names.contains(&key) || self.bins.contains(&key))
    }

    /// Coincidencia más amplia para no marcar “Docker Desktop” si Docker está instalado.
    pub fn owns_loose(&self, name: &str) -> bool {
        if self.owns_exact(name) {
            return true;
        }
        let key = normalize(name);
        if key.len() < 6 {
            return false;
        }
        let related = |other: &String| other.len() >= 6 && (key.contains(other) || other.contains(key.as_str()));
        self.exact_names.iter().any(&related) || self.bins.iter().any(&related)
    }

    /// La carpeta parece de un fabricante: alguna app instalada empieza por ese nombre.
    pub fn is_vendor_prefix(&self, name: &str) -> bool {
        let key = normalize(name);
        if key.len() < 3 {
            return false;
        }
        self.exact_names
            .iter()
            .any(|app| app.len() > key.len() && app.starts_with(&key))
    }

    pub fn owns_identifier(&self, raw: &str) -> bool {
        let mut id = raw.trim().to_lowercase();
        if let Some(rest) = id.strip_suffix(".plist") {
            id = rest.to_string();
        }
        if let Some(rest) = id.strip_suffix(".savedstate") {
            id = rest.to_string();
        }
        if let Some(rest) = id.strip_prefix("group.") {
            id = rest.to_string();
        }
        if id.starts_with("com.apple.") {
            return true;
        }
        if !id.contains('.') {
            return self.owns_exact(&id);
        }
        self.bundle_ids.iter().any(|bundle| {
            id == *bundle
                || id.starts_with(&format!("{bundle}."))
                || bundle.starts_with(&format!("{id}."))
        })
    }
}

pub fn inventory() -> Inventory {
    let mut apps = Vec::new();
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/Users"));
    for dir in [
        PathBuf::from("/Applications"),
        PathBuf::from("/System/Applications"),
        PathBuf::from("/System/Library/CoreServices"),
        home.join("Applications"),
    ] {
        collect_dir(&dir, true, &mut apps);
    }

    let mut exact_names = HashSet::new();
    let mut bundle_ids = Vec::new();
    for app in &apps {
        for name in [
            app.name.as_str(),
            app.display.as_str(),
            app.executable.as_str(),
            app.folder_name.as_str(),
        ] {
            let key = normalize(name);
            if key.len() >= 2 {
                exact_names.insert(key);
            }
        }
        if !app.bundle_id.is_empty() {
            let bundle = app.bundle_id.to_lowercase();
            if let Some(last) = bundle.rsplit('.').next() {
                if last.len() >= 4 && !GENERIC_LAST.contains(&last) {
                    exact_names.insert(normalize(last));
                }
            }
            bundle_ids.push(bundle);
        }
    }

    Inventory {
        exact_names,
        bundle_ids,
        bins: collect_bins(),
        app_count: apps.len(),
    }
}

const GENERIC_LAST: &[&str] = &[
    "helper", "agent", "service", "client", "app", "mac", "desktop", "osx", "main",
];

#[derive(Debug)]
struct AppInfo {
    name: String,
    display: String,
    bundle_id: String,
    executable: String,
    folder_name: String,
}

fn collect_dir(dir: &Path, recurse_once: bool, out: &mut Vec<AppInfo>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let fname = entry.file_name().to_string_lossy().to_string();
        if fname.starts_with('.') {
            continue;
        }
        if fname.ends_with(".app") {
            out.push(read_app(&path, &fname));
            continue;
        }
        let localized = fname.ends_with(".localized");
        let plain_dir = path.extension().is_none();
        if recurse_once && path.is_dir() && (plain_dir || localized) {
            collect_dir(&path, false, out);
        }
    }
}

fn read_app(path: &Path, file_name: &str) -> AppInfo {
    let folder_name = file_name.trim_end_matches(".app").to_string();
    let mut name = folder_name.clone();
    let mut display = folder_name.clone();
    let mut bundle_id = String::new();
    let mut executable = folder_name.clone();

    if let Ok(value) = plist::Value::from_file(path.join("Contents/Info.plist")) {
        if let Some(dict) = value.as_dictionary() {
            if let Some(text) = dict_string(dict, "CFBundleName") {
                name = text;
            }
            if let Some(text) = dict_string(dict, "CFBundleDisplayName") {
                display = text;
            }
            if let Some(text) = dict_string(dict, "CFBundleIdentifier") {
                bundle_id = text;
            }
            if let Some(text) = dict_string(dict, "CFBundleExecutable") {
                executable = text;
            }
        }
    }

    AppInfo {
        name,
        display,
        bundle_id,
        executable,
        folder_name,
    }
}

fn dict_string(dict: &plist::Dictionary, key: &str) -> Option<String> {
    dict.get(key)
        .and_then(|value| value.as_string())
        .map(|text| text.to_string())
}

fn collect_bins() -> HashSet<String> {
    let mut bins = HashSet::new();
    let mut dirs = vec![
        PathBuf::from("/opt/homebrew/bin"),
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("/opt/homebrew/Caskroom"),
        PathBuf::from("/usr/local/Caskroom"),
    ];
    if let Some(home) = dirs::home_dir() {
        dirs.push(home.join("bin"));
        dirs.push(home.join(".local/bin"));
    }
    for dir in dirs {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let key = normalize(&name);
            if key.len() >= 2 {
                bins.insert(key);
            }
        }
    }
    bins
}

pub fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Inventory {
        Inventory {
            exact_names: HashSet::from([
                "code".into(),
                "visualstudiocode".into(),
                "microsoftword".into(),
                "googlechrome".into(),
                "zoomus".into(),
            ]),
            bundle_ids: vec!["com.microsoft.word".into(), "com.google.chrome".into()],
            bins: HashSet::from(["codex".into()]),
            app_count: 3,
        }
    }

    #[test]
    fn matches_apps_and_cli_tools() {
        let inv = sample();
        assert!(inv.owns_exact("Code"));
        assert!(inv.owns_exact("zoom.us"));
        assert!(inv.owns_exact("codex"));
        assert!(inv.is_vendor_prefix("Microsoft"));
        assert!(inv.is_vendor_prefix("Google"));
        assert!(!inv.is_vendor_prefix("go"));
        assert!(inv.owns_identifier("com.google.Chrome.helper"));
        assert!(inv.owns_identifier("com.apple.mail"));
        assert!(!inv.owns_identifier("com.abandoned.editor"));
    }
}
