use std::path::Path;

use serde::Deserialize;

use crate::models::errors::{FastDiskError, FastDiskResult, FrontendError};

#[derive(Debug, Deserialize)]
pub struct RevealInExplorerInput {
    pub path: String,
}

#[tauri::command]
pub async fn reveal_in_explorer(input: RevealInExplorerInput) -> Result<(), FrontendError> {
    reveal_in_explorer_inner(&input.path).map_err(|error| error.to_frontend_error())?;
    reveal_path(&input.path).map_err(|error| error.to_frontend_error())
}

pub fn reveal_in_explorer_inner(path: &str) -> FastDiskResult<()> {
    if path.trim().is_empty() || !Path::new(path).exists() {
        return Err(FastDiskError::MissingPath);
    }
    Ok(())
}

fn reveal_path(path: &str) -> FastDiskResult<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(format!("/select,{path}"))
            .spawn()?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = path;
    }

    Ok(())
}
