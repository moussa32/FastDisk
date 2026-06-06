pub mod errors;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanStatus {
    Idle,
    Scanning,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryType {
    All,
    File,
    Folder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanSession {
    pub id: i64,
    pub root_path: String,
    pub status: ScanStatus,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub total_files: i64,
    pub total_folders: i64,
    pub total_size: i64,
    pub skipped_items: i64,
    pub elapsed_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub id: i64,
    pub scan_session_id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub is_directory: bool,
    pub extension: Option<String>,
    pub depth: i64,
    pub modified_at: Option<String>,
    pub created_at: Option<String>,
    pub is_symlink: bool,
    pub child_count: i64,
    pub descendant_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanIssue {
    pub id: i64,
    pub scan_session_id: i64,
    pub path: String,
    pub error_kind: String,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionSummary {
    pub extension: String,
    pub total_size: i64,
    pub file_count: i64,
    pub percentage_of_scan: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreemapNode {
    pub id: Option<i64>,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub is_directory: bool,
    pub percentage_of_parent: f64,
    pub children: Option<Vec<TreemapNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterSet {
    pub query: Option<String>,
    pub entry_type: EntryType,
    pub extension: Option<String>,
    pub extension_group: Option<String>,
    pub min_size: Option<i64>,
    pub max_size: Option<i64>,
    pub modified_from: Option<String>,
    pub modified_to: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl Default for FilterSet {
    fn default() -> Self {
        Self {
            query: None,
            entry_type: EntryType::All,
            extension: None,
            extension_group: None,
            min_size: None,
            max_size: None,
            modified_from: None,
            modified_to: None,
            limit: 100,
            offset: 0,
        }
    }
}
