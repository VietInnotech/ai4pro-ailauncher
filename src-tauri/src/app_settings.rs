use crate::db::Database;
use crate::errors::AppResult;
use crate::models::AppSettingsRecord;

#[derive(Debug, Clone)]
pub struct AppSettingsStore {
    db: Database,
}

impl AppSettingsStore {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn load(&self) -> AppResult<AppSettingsRecord> {
        self.db.load_settings()
    }

    pub fn save(&self, settings: &AppSettingsRecord) -> AppResult<()> {
        self.db.save_settings(settings)
    }
}
