//! Tamaño de lo marcado. No decide si una ruta se puede borrar.

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionSummary {
    pub count: usize,
    pub bytes: u64,
    pub total: u64,
}

/// Texto de un tamaño, con el mismo redondeo que la vista anterior en JavaScript.
pub fn format_bytes(bytes: u64) -> String {
    let value = bytes as f64;
    if value < 1024.0 {
        return format!("{} B", value.round() as u64);
    }
    let units = ["KB", "MB", "GB", "TB"];
    let mut size = value / 1024.0;
    let mut index = 0;
    while size >= 1024.0 && index < units.len() - 1 {
        size /= 1024.0;
        index += 1;
    }
    let digits = if index >= 2 {
        if size >= 100.0 { 1 } else { 2 }
    } else if size >= 10.0 {
        1
    } else {
        2
    };
    format!("{:.*} {}", digits, size, units[index])
}

pub fn selection_of<'a>(
    items: impl IntoIterator<Item = (&'a str, u64)>,
    selected: &HashSet<String>,
) -> SelectionSummary {
    let items: Vec<_> = items.into_iter().collect();
    let picked: Vec<_> = items
        .iter()
        .filter(|(id, _)| selected.contains(*id))
        .collect();
    SelectionSummary {
        count: picked.len(),
        bytes: picked.iter().map(|(_, bytes)| *bytes).sum(),
        total: items.iter().map(|(_, bytes)| *bytes).sum(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;

    // TEST-selection-001
    #[test]
    fn empty_selection_does_not_inherit_total() {
        let items = [("caches-1", 2 * GB), ("caches-2", 400 * MB)];
        let result = selection_of(items, &HashSet::new());
        assert_eq!(result.count, 0);
        assert_eq!(result.bytes, 0);
        assert_eq!(result.total, 2 * GB + 400 * MB);
        assert_eq!(format_bytes(result.bytes), "0 B");
    }

    // TEST-selection-002
    #[test]
    fn toggling_changes_selected_size() {
        let items = [("caches-1", 2 * GB), ("caches-2", 80 * MB)];
        let only_large = selection_of(items, &HashSet::from(["caches-1".into()]));
        let both = selection_of(
            items,
            &HashSet::from(["caches-1".into(), "caches-2".into()]),
        );
        assert_eq!(only_large.bytes, 2 * GB);
        assert_eq!(both.bytes, 2 * GB + 80 * MB);
        assert_ne!(format_bytes(only_large.bytes), format_bytes(both.bytes));
    }

    // TEST-selection-003
    #[test]
    fn eighty_mb_on_a_gb_total_changes_the_text() {
        let before = format_bytes(15 * GB);
        let after = format_bytes(15 * GB - 80 * MB);
        assert_ne!(before, after);
        assert!(before.ends_with("GB"), "{before}");
        assert!(after.ends_with("GB"), "{after}");
    }
}
