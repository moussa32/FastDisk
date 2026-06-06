#[cfg(test)]
mod tests;

use std::time::{Duration, Instant};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanProgressEvent {
    pub scan_session_id: i64,
    pub current_path: String,
    pub files_scanned: i64,
    pub folders_scanned: i64,
    pub bytes_scanned: i64,
    pub skipped_items: i64,
    pub elapsed_ms: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanCompletedEvent {
    pub scan_session_id: i64,
    pub total_files: i64,
    pub total_folders: i64,
    pub total_size: i64,
    pub skipped_items: i64,
    pub elapsed_ms: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanFailedEvent {
    pub scan_session_id: i64,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanCancelledEvent {
    pub scan_session_id: i64,
}

#[derive(Debug)]
pub struct ProgressThrottle {
    interval: Duration,
    last_emit: Option<Instant>,
}

impl ProgressThrottle {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_emit: None,
        }
    }

    pub fn should_emit(&mut self, now: Instant) -> bool {
        match self.last_emit {
            None => {
                self.last_emit = Some(now);
                true
            }
            Some(last_emit) if now.duration_since(last_emit) >= self.interval => {
                self.last_emit = Some(now);
                true
            }
            Some(_) => false,
        }
    }
}
