use super::{resolve_binary_path, resolve_relative_or_absolute, EngineAdapter};
use crate::app_paths::AppPaths;
use crate::errors::{AppError, AppResult};
use crate::models::{EngineKind, EngineProfileRecord};
use crate::process_supervisor::LaunchSpec;

#[derive(Debug, Clone, Copy)]
pub struct LlamaCppAdapter;

impl EngineAdapter for LlamaCppAdapter {
    fn kind(&self) -> EngineKind {
        EngineKind::LlamaCpp
    }

    fn binary_name(&self) -> &'static str {
        current_binary_name("llama-server")
    }

    fn launch_spec(
        &self,
        paths: &AppPaths,
        profile: &EngineProfileRecord,
    ) -> AppResult<LaunchSpec> {
        let binary_path = resolve_binary_path(paths, profile, self);
        let model_path = profile
            .model_path
            .as_deref()
            .map(|path| {
                resolve_relative_or_absolute(&paths.app_root, path)
                    .to_string_lossy()
                    .to_string()
            })
            .ok_or_else(|| AppError::new("MISSING_MODEL", "Cần có đường dẫn mô hình."))?;

        Ok(LaunchSpec {
            id: profile.id.clone(),
            binary_path,
            args: vec![
                "--host".into(),
                profile.host.clone(),
                "--port".into(),
                profile.port.to_string(),
                "--model".into(),
                model_path,
            ],
            env: vec![],
            log_root: paths.logs_dir.clone(),
        })
    }
}

fn current_binary_name(base: &str) -> &'static str {
    match (
        cfg!(target_os = "windows"),
        cfg!(target_arch = "aarch64"),
        cfg!(target_os = "macos"),
    ) {
        (true, _, _) => Box::leak(format!("{base}.exe").into_boxed_str()),
        (false, true, true) => "llama-server-aarch64-apple-darwin",
        (false, false, true) => "llama-server-x86_64-apple-darwin",
        _ => "llama-server-x86_64-pc-windows-msvc.exe",
    }
}
