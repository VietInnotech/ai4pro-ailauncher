use crate::errors::{AppError, AppResult};
use crate::models::AppSettingsRecord;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct DeveloperModeController {
    enabled: Arc<RwLock<bool>>,
    persistent: bool,
}

impl DeveloperModeController {
    pub fn new(settings: &AppSettingsRecord) -> Self {
        let controller = Self {
            enabled: Arc::new(RwLock::new(false)),
            persistent: settings.developer_mode_persisted,
        };

        if settings.developer_mode_persisted {
            controller.enable_session();
        }

        controller
    }

    pub fn is_enabled(&self) -> bool {
        *self.enabled.read().unwrap()
    }

    pub fn enable_session(&self) {
        *self.enabled.write().unwrap() = true;
    }

    pub fn disable_session(&self) {
        *self.enabled.write().unwrap() = false;
    }

    pub fn require_enabled(&self) -> AppResult<()> {
        if self.is_enabled() {
            Ok(())
        } else {
            Err(AppError::developer_mode_required())
        }
    }

    pub fn persistent(&self) -> bool {
        self.persistent
    }
}
