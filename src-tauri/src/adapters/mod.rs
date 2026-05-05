pub mod llama_cpp;
pub mod sherpa_onnx;

use crate::app_paths::AppPaths;
use crate::errors::AppResult;
use crate::models::{BinaryMode, EngineKind, EngineProfileRecord};
use crate::process_supervisor::LaunchSpec;

pub trait EngineAdapter {
    fn kind(&self) -> EngineKind;
    fn binary_name(&self) -> &'static str;
    fn launch_spec(&self, paths: &AppPaths, profile: &EngineProfileRecord) -> AppResult<LaunchSpec>;
}

pub fn adapter_for(kind: &EngineKind) -> Box<dyn EngineAdapter + Send + Sync> {
    match kind {
        EngineKind::LlamaCpp => Box::new(llama_cpp::LlamaCppAdapter),
        EngineKind::SherpaOnnx => Box::new(sherpa_onnx::SherpaOnnxAdapter),
    }
}

pub fn resolve_binary_path(paths: &AppPaths, profile: &EngineProfileRecord, adapter: &dyn EngineAdapter) -> std::path::PathBuf {
    match profile.binary_mode {
        BinaryMode::Custom => profile
            .binary_path
            .as_ref()
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| paths.bundled_binary_path(adapter.binary_name())),
        BinaryMode::Bundled => paths.bundled_binary_path(adapter.binary_name()),
    }
}
