use serde::{Deserialize, Serialize};

/// Qué tan seguro es mover este elemento.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Risk {
    /// Se puede regenerar (cachés, logs).
    Safe,
    /// Puede haber documentos o datos personales. No se marca solo.
    Review,
    /// Vaciar la Papelera no se puede deshacer.
    Permanent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanKind {
    MoveToTrash,
    EmptyTrash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanItem {
    pub id: String,
    pub name: String,
    pub display_path: String,
    pub delete_paths: Vec<String>,
    pub bytes: u64,
    pub risk: Risk,
    pub reason: String,
    pub default_selected: bool,
    pub kind: CleanKind,
    pub modified_secs: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub description: String,
    pub items: Vec<CleanItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub categories: Vec<Category>,
    pub notes: Vec<String>,
    pub elapsed_ms: u64,
    pub apps_found: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub stage: String,
    pub detail: String,
    pub stage_index: u32,
    pub stage_count: u32,
    pub found_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanFailure {
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanReport {
    pub moved: usize,
    pub bytes: u64,
    pub failures: Vec<CleanFailure>,
    pub emptied_trash: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanProgress {
    pub name: String,
    pub done: u32,
    pub total: u32,
}
