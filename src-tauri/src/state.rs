use crate::app_paths::AppPaths;
use crate::app_settings::AppSettingsStore;
use crate::bundled_resources;
use crate::db::Database;
use crate::developer::developer_mode::DeveloperModeController;
use crate::engine_manager::EngineManager;
use crate::errors::AppResult;
use crate::local_ai::LocalAiService;
use crate::process_supervisor::ProcessSupervisor;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub paths: AppPaths,
    pub db: Database,
    pub settings: AppSettingsStore,
    pub developer_mode: DeveloperModeController,
    pub engine_manager: Arc<EngineManager>,
    pub local_ai: LocalAiService,
}

impl AppContext {
    pub fn bootstrap(resource_dir: Option<&Path>) -> AppResult<Self> {
        let paths = AppPaths::discover()?;
        if let Some(resource_dir) = resource_dir {
            bundled_resources::sync_bundled_resources(&paths, resource_dir)?;
        }

        let db = Database::initialize(paths.database_path.clone())?;
        let settings = AppSettingsStore::new(db.clone());
        let mut loaded_settings = settings.load()?;

        if loaded_settings.app_data_root.is_empty() {
            loaded_settings.app_data_root = paths.app_root.to_string_lossy().to_string();
            settings.save(&loaded_settings)?;
        }

        let developer_mode = DeveloperModeController::new(&loaded_settings);
        let supervisor = ProcessSupervisor::new();
        let engine_manager = Arc::new(EngineManager::new(db.clone(), paths.clone(), supervisor));
        engine_manager.bootstrap_defaults()?;
        if loaded_settings.auto_start_local_ai {
            let _ = engine_manager.start_all();
        }
        let local_ai = LocalAiService::new(engine_manager.clone());

        Ok(Self {
            paths,
            db,
            settings,
            developer_mode,
            engine_manager,
            local_ai,
        })
    }
}
