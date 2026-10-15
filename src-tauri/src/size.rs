use std::path::Path;
use std::time::UNIX_EPOCH;

use walkdir::WalkDir;

use crate::safety;

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
        if safety::container_metadata(path) || finder_junk(path) {
            return 0;
        }
        return meta.len();
    }
    if !meta.is_dir() {
        return 0;
    }

    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            !entry.path_is_symlink()
                && entry.file_type().is_file()
                && !safety::container_metadata(entry.path())
                && !finder_junk(entry.path())
        })
        .filter_map(|entry| entry.metadata().ok())
        .map(|meta| meta.len())
        .sum()
}

fn finder_junk(path: &Path) -> bool {
    path.file_name().and_then(|name| name.to_str()) == Some(".DS_Store")
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

#[cfg(test)]
mod tests {
    use super::*;

    // TEST-size-001
    #[test]
    fn path_size_ignores_container_metadata() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("cristal-size-{stamp}"));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("nota.txt"), vec![1; 100]).unwrap();
        std::fs::write(
            root.join(".com.apple.containermanagerd.metadata.plist"),
            vec![0; 30_000],
        )
        .unwrap();
        std::fs::write(root.join(".DS_Store"), vec![0; 6_148]).unwrap();

        assert_eq!(path_size(&root), 100);

        let _ = std::fs::remove_dir_all(&root);
    }
}
