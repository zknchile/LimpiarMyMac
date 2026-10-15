use std::path::Path;
use std::time::UNIX_EPOCH;

use walkdir::WalkDir;

/// Tamaño de un archivo o carpeta. No sigue enlaces simbólicos.
pub fn path_size(path: &Path) -> u64 {
    let meta = match std::fs::symlink_metadata(path) {
        Ok(meta) => meta,
        Err(_) => return 0,
    };
    if meta.file_type().is_symlink() {
        return 0;
    }
    if meta.is_file() {
        return meta.len();
    }
    if !meta.is_dir() {
        return 0;
    }

    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| !entry.path_is_symlink() && entry.file_type().is_file())
        .filter_map(|entry| entry.metadata().ok())
        .map(|meta| meta.len())
        .sum()
}

pub fn modified_secs(path: &Path) -> Option<i64> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    let secs = modified.duration_since(UNIX_EPOCH).ok()?.as_secs();
    Some(secs as i64)
}

pub fn older_than_days(path: &Path, days: u64) -> bool {
    let modified = match std::fs::metadata(path).and_then(|meta| meta.modified()) {
        Ok(time) => time,
        Err(_) => return false,
    };
    match modified.elapsed() {
        Ok(elapsed) => elapsed.as_secs() > days.saturating_mul(24 * 60 * 60),
        Err(_) => false,
    }
}
