pub mod session;

#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use walkdir::WalkDir;

use crate::models::errors::{FastDiskError, FastDiskResult};

#[derive(Debug, Clone)]
pub struct ScannedEntry {
    pub parent_path: Option<PathBuf>,
    pub path: PathBuf,
    pub name: String,
    pub size: i64,
    pub is_directory: bool,
    pub extension: Option<String>,
    pub depth: i64,
    pub modified_at: Option<String>,
    pub created_at: Option<String>,
    pub is_symlink: bool,
}

#[derive(Debug, Clone)]
pub struct ScanIssueRecord {
    pub path: PathBuf,
    pub error_kind: String,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Default, Clone)]
pub struct ScanOutput {
    pub entries: Vec<ScannedEntry>,
    pub issues: Vec<ScanIssueRecord>,
    pub total_files: i64,
    pub total_folders: i64,
    pub total_size: i64,
}

pub fn scan_path(root_path: impl AsRef<Path>, cancel: Arc<AtomicBool>) -> FastDiskResult<ScanOutput> {
    let root_path = root_path.as_ref();
    if !root_path.exists() {
        return Err(FastDiskError::InvalidPath);
    }

    let mut output = ScanOutput::default();
    let walker = WalkDir::new(root_path)
        .follow_links(false)
        .same_file_system(false)
        .into_iter();

    for item in walker {
        if cancel.load(Ordering::Relaxed) {
            break;
        }

        match item {
            Ok(dir_entry) => {
                let path = dir_entry.path().to_path_buf();
                match std::fs::symlink_metadata(&path) {
                    Ok(metadata) => {
                        let is_directory = metadata.is_dir();
                        let is_symlink = metadata.file_type().is_symlink();
                        let size = if is_directory { 0 } else { metadata.len() as i64 };
                        if is_directory {
                            output.total_folders += 1;
                        } else {
                            output.total_files += 1;
                            output.total_size += size;
                        }
                        output.entries.push(ScannedEntry {
                            parent_path: path.parent().map(Path::to_path_buf).filter(|parent| parent != root_path.parent().unwrap_or(root_path)),
                            name: dir_entry
                                .file_name()
                                .to_string_lossy()
                                .to_string(),
                            extension: if is_directory {
                                None
                            } else {
                                path.extension().map(|value| value.to_string_lossy().to_ascii_lowercase())
                            },
                            depth: dir_entry.depth() as i64,
                            modified_at: metadata.modified().ok().map(system_time_to_iso),
                            created_at: metadata.created().ok().map(system_time_to_iso),
                            is_symlink,
                            is_directory,
                            path,
                            size,
                        });
                    }
                    Err(error) => output.issues.push(issue_for_path(path, "metadata", error.to_string())),
                }
            }
            Err(error) => {
                let path = error.path().map(Path::to_path_buf).unwrap_or_else(|| root_path.to_path_buf());
                output.issues.push(issue_for_path(path, "walkdir", error.to_string()));
            }
        }
    }

    Ok(output)
}

fn issue_for_path(path: PathBuf, error_kind: &str, error_message: String) -> ScanIssueRecord {
    ScanIssueRecord {
        path,
        error_kind: error_kind.to_string(),
        error_message: Some(error_message),
        created_at: Utc::now().to_rfc3339(),
    }
}

fn system_time_to_iso(value: SystemTime) -> String {
    DateTime::<Utc>::from(value).to_rfc3339()
}
