use crate::engine_manager::EngineManager;
use crate::errors::AppResult;
use crate::models::SimpleLocalAiStatusDto;
use std::sync::Arc;

#[derive(Clone)]
pub struct LocalAiService {
    engine_manager: Arc<EngineManager>,
}

impl LocalAiService {
    pub fn new(engine_manager: Arc<EngineManager>) -> Self {
        Self { engine_manager }
    }

    pub fn status(&self) -> AppResult<SimpleLocalAiStatusDto> {
        self.engine_manager.simple_status()
    }

    pub fn check_model(&self, id: &str) -> AppResult<SimpleLocalAiStatusDto> {
        self.engine_manager.check_simple_model(id)
    }

    pub fn start(&self) -> AppResult<SimpleLocalAiStatusDto> {
        self.engine_manager.start_all()?;
        self.status()
    }

    pub fn stop(&self) -> AppResult<SimpleLocalAiStatusDto> {
        self.engine_manager.stop_all()?;
        self.status()
    }

    pub fn restart(&self) -> AppResult<SimpleLocalAiStatusDto> {
        let _ = self.engine_manager.stop_all();
        self.engine_manager.start_all()?;
        self.status()
    }
}
