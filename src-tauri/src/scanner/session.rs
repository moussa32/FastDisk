use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::models::errors::{FastDiskError, FastDiskResult};

#[derive(Debug, Clone)]
pub struct ActiveScan {
    pub scan_session_id: i64,
    pub cancel: Arc<AtomicBool>,
    pub started_at: Instant,
}

#[derive(Debug, Default)]
pub struct ScanManager {
    active: Mutex<Option<ActiveScan>>,
}

impl ScanManager {
    pub fn begin_scan(&self, scan_session_id: i64) -> FastDiskResult<Arc<AtomicBool>> {
        let mut active = self
            .active
            .lock()
            .map_err(|_| FastDiskError::Other("Scan manager lock poisoned.".into()))?;
        if active.is_some() {
            return Err(FastDiskError::ScanAlreadyRunning);
        }
        let cancel = Arc::new(AtomicBool::new(false));
        *active = Some(ActiveScan {
            scan_session_id,
            cancel: Arc::clone(&cancel),
            started_at: Instant::now(),
        });
        Ok(cancel)
    }

    pub fn cancel_scan(&self, scan_session_id: i64) -> FastDiskResult<bool> {
        let active = self
            .active
            .lock()
            .map_err(|_| FastDiskError::Other("Scan manager lock poisoned.".into()))?;
        if let Some(active) = active.as_ref().filter(|scan| scan.scan_session_id == scan_session_id) {
            active.cancel.store(true, Ordering::Relaxed);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn finish_scan(&self, scan_session_id: i64) -> FastDiskResult<Option<ActiveScan>> {
        let mut active = self
            .active
            .lock()
            .map_err(|_| FastDiskError::Other("Scan manager lock poisoned.".into()))?;
        if active.as_ref().is_some_and(|scan| scan.scan_session_id == scan_session_id) {
            return Ok(active.take());
        }
        Ok(None)
    }

    pub fn is_active(&self) -> FastDiskResult<bool> {
        Ok(self
            .active
            .lock()
            .map_err(|_| FastDiskError::Other("Scan manager lock poisoned.".into()))?
            .is_some())
    }
}
