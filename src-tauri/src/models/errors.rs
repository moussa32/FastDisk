use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FastDiskError {
    #[error("The selected path is missing or invalid.")]
    InvalidPath,
    #[error("A scan is already running.")]
    ScanAlreadyRunning,
    #[error("The requested item no longer exists.")]
    MissingPath,
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendError {
    pub kind: String,
    pub message: String,
}

impl FastDiskError {
    pub fn frontend_kind(&self) -> &'static str {
        match self {
            Self::InvalidPath => "invalid_path",
            Self::ScanAlreadyRunning => "scan_already_running",
            Self::MissingPath => "missing_path",
            Self::Database(_) => "database",
            Self::Io(_) => "io",
            Self::Other(_) => "unknown",
        }
    }

    pub fn to_frontend_error(&self) -> FrontendError {
        FrontendError {
            kind: self.frontend_kind().to_string(),
            message: self.to_string(),
        }
    }
}

pub type FastDiskResult<T> = Result<T, FastDiskError>;
