use crate::errors::AppResult;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct AppPaths {
    pub app_root: PathBuf,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub models_dir: PathBuf,
    pub binaries_dir: PathBuf,
    pub database_path: PathBuf,
}

impl AppPaths {
    pub fn discover() -> AppResult<Self> {
        let app_root = default_app_root();
        let config_dir = app_root.join("config");
        let data_dir = app_root.join("data");
        let logs_dir = app_root.join("logs");
        let models_dir = app_root.join("models");
        let binaries_dir = app_root.join("binaries");
        let database_path = data_dir.join("local_ai.sqlite");

        for path in [&app_root, &config_dir, &data_dir, &logs_dir, &models_dir, &binaries_dir] {
            fs::create_dir_all(path)?;
        }

        Ok(Self {
            app_root,
            config_dir,
            data_dir,
            logs_dir,
            models_dir,
            binaries_dir,
            database_path,
        })
    }

    pub fn bundled_binary_path(&self, binary_name: &str) -> PathBuf {
        self.binaries_dir.join(binary_name)
    }

    pub fn models_root(&self) -> &Path {
        &self.models_dir
    }
}

fn default_app_root() -> PathBuf {
    if let Some(path) = env::var_os("LOCAL_AI_APP_DATA_ROOT") {
        return PathBuf::from(path);
    }

    if let Some(path) = env::var_os("LOCALAI_HOME") {
        return PathBuf::from(path);
    }

    if cfg!(target_os = "windows") {
        return env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(r"C:\\ProgramData"))
            .join("AI4Pro")
            .join("AILauncher");
    }

    if cfg!(target_os = "macos") {
        return env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Library/Application Support/AI4Pro/AILauncher");
    }

    env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".local/share")
        })
        .join("AI4Pro")
        .join("AILauncher")
}
