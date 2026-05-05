use crate::errors::AppResult;
use crate::migrations::migrate;
use crate::models::{
    AppSettingsRecord, BinaryMode, EngineKind, EngineProfileRecord, EngineRuntimeStateRecord,
    EngineStatus, ModelPackageRecord,
};
use rusqlite::{params, Connection};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn initialize(path: PathBuf) -> AppResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let database = Self { path };
        let conn = database.open()?;
        migrate(&conn)?;
        database.ensure_defaults()?;
        Ok(database)
    }

    fn open(&self) -> AppResult<Connection> {
        Ok(Connection::open(&self.path)?)
    }

    fn ensure_defaults(&self) -> AppResult<()> {
        if self.load_model_packages()?.is_empty() {
            for package in default_model_packages() {
                self.upsert_model_package(&package)?;
            }
        }

        if self.list_engine_profiles()?.is_empty() {
            for profile in default_engine_profiles() {
                self.upsert_engine_profile(&profile)?;
            }
        }

        if self.list_runtime_state()?.is_empty() {
            for profile in self.list_engine_profiles()? {
                self.upsert_runtime_state(&EngineRuntimeStateRecord::stopped(profile.id))?;
            }
        }

        Ok(())
    }

    pub fn load_settings(&self) -> AppResult<AppSettingsRecord> {
        let conn = self.open()?;
        let settings = AppSettingsRecord {
            app_data_root: get_setting(&conn, "app_data_root")?.unwrap_or_default(),
            developer_mode_persisted: get_setting(&conn, "developer_mode_persisted")?
                .as_deref()
                == Some("true"),
            stop_engines_on_exit: get_setting(&conn, "stop_engines_on_exit")?
                .as_deref()
                .map(|value| value == "true")
                .unwrap_or(true),
            auto_start_local_ai: get_setting(&conn, "auto_start_local_ai")?
                .as_deref()
                .map(|value| value == "true")
                .unwrap_or(false),
            simple_mode_only: get_setting(&conn, "simple_mode_only")?
                .as_deref()
                .map(|value| value == "true")
                .unwrap_or(false),
            machine_configured: get_setting(&conn, "machine_configured")?
                .as_deref()
                .map(|value| value == "true")
                .unwrap_or(false),
            setup_version: get_setting(&conn, "setup_version")?.unwrap_or_else(|| "0.1.0".to_string()),
        };
        Ok(settings)
    }

    pub fn save_settings(&self, settings: &AppSettingsRecord) -> AppResult<()> {
        let conn = self.open()?;
        set_setting(&conn, "app_data_root", &settings.app_data_root)?;
        set_setting(&conn, "developer_mode_persisted", bool_str(settings.developer_mode_persisted))?;
        set_setting(&conn, "stop_engines_on_exit", bool_str(settings.stop_engines_on_exit))?;
        set_setting(&conn, "auto_start_local_ai", bool_str(settings.auto_start_local_ai))?;
        set_setting(&conn, "simple_mode_only", bool_str(settings.simple_mode_only))?;
        set_setting(&conn, "machine_configured", bool_str(settings.machine_configured))?;
        set_setting(&conn, "setup_version", &settings.setup_version)?;
        Ok(())
    }

    pub fn list_model_packages(&self) -> AppResult<Vec<ModelPackageRecord>> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT id, kind, display_name, internal_name, relative_path, required_files_json,
                   manifest_json, installed, verified, last_verified_at, created_at, updated_at
            FROM model_packages
            ORDER BY id
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            let kind: String = row.get(1)?;
            Ok(ModelPackageRecord {
                id: row.get(0)?,
                kind: parse_kind(&kind),
                display_name: row.get(2)?,
                internal_name: row.get(3)?,
                relative_path: row.get(4)?,
                required_files: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                manifest_json: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_else(|_| serde_json::json!({})),
                installed: row.get::<_, i64>(7)? != 0,
                verified: row.get::<_, i64>(8)? != 0,
                last_verified_at: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?;

        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn load_model_packages(&self) -> AppResult<Vec<ModelPackageRecord>> {
        self.list_model_packages()
    }

    pub fn upsert_model_package(&self, package: &ModelPackageRecord) -> AppResult<()> {
        let conn = self.open()?;
        conn.execute(
            r#"
            INSERT INTO model_packages (
              id, kind, display_name, internal_name, relative_path, required_files_json,
              manifest_json, installed, verified, last_verified_at, created_at, updated_at
            ) VALUES (
              ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            ON CONFLICT(id) DO UPDATE SET
              kind = excluded.kind,
              display_name = excluded.display_name,
              internal_name = excluded.internal_name,
              relative_path = excluded.relative_path,
              required_files_json = excluded.required_files_json,
              manifest_json = excluded.manifest_json,
              installed = excluded.installed,
              verified = excluded.verified,
              last_verified_at = excluded.last_verified_at,
              created_at = excluded.created_at,
              updated_at = excluded.updated_at
            "#,
            params![
                package.id,
                format_kind(&package.kind),
                package.display_name,
                package.internal_name,
                package.relative_path,
                serde_json::to_string(&package.required_files)?,
                package.manifest_json.to_string(),
                package.installed as i64,
                package.verified as i64,
                package.last_verified_at,
                package.created_at,
                package.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_engine_profiles(&self) -> AppResult<Vec<EngineProfileRecord>> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT id, kind, name, enabled, binary_mode, binary_name, binary_path,
                   model_package_id, model_path, model_dir, tokens_path, host, port,
                   health_url, runtime_json, extra_args_json, auto_start, status, pid,
                   last_error, last_exit_code, created_at, updated_at
            FROM engine_profiles
            ORDER BY id
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            let kind: String = row.get(1)?;
            let binary_mode: String = row.get(4)?;
            let runtime_json: String = row.get(14)?;
            let extra_args_json: String = row.get(15)?;
            let status: String = row.get(17)?;
            Ok(EngineProfileRecord {
                id: row.get(0)?,
                kind: parse_kind(&kind),
                name: row.get(2)?,
                enabled: row.get::<_, i64>(3)? != 0,
                binary_mode: parse_binary_mode(&binary_mode),
                binary_name: row.get(5)?,
                binary_path: row.get(6)?,
                model_package_id: row.get(7)?,
                model_path: row.get(8)?,
                model_dir: row.get(9)?,
                tokens_path: row.get(10)?,
                host: row.get(11)?,
                port: row.get::<_, u16>(12)?,
                health_url: row.get(13)?,
                runtime: serde_json::from_str(&runtime_json).unwrap_or_else(|_| serde_json::json!({})),
                extra_args: serde_json::from_str(&extra_args_json).unwrap_or_default(),
                auto_start: row.get::<_, i64>(16)? != 0,
                status: parse_status(&status),
                pid: row.get(18)?,
                last_error: row.get(19)?,
                last_exit_code: row.get(20)?,
                created_at: row.get(21)?,
                updated_at: row.get(22)?,
            })
        })?;

        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn load_engine_profile(&self, id: &str) -> AppResult<Option<EngineProfileRecord>> {
        Ok(self.list_engine_profiles()?.into_iter().find(|profile| profile.id == id))
    }

    pub fn upsert_engine_profile(&self, profile: &EngineProfileRecord) -> AppResult<()> {
        let conn = self.open()?;
        conn.execute(
            r#"
            INSERT INTO engine_profiles (
              id, kind, name, enabled, binary_mode, binary_name, binary_path,
              model_package_id, model_path, model_dir, tokens_path, host, port,
              health_url, runtime_json, extra_args_json, auto_start, status, pid,
              last_error, last_exit_code, created_at, updated_at
            ) VALUES (
              ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            ON CONFLICT(id) DO UPDATE SET
              kind = excluded.kind,
              name = excluded.name,
              enabled = excluded.enabled,
              binary_mode = excluded.binary_mode,
              binary_name = excluded.binary_name,
              binary_path = excluded.binary_path,
              model_package_id = excluded.model_package_id,
              model_path = excluded.model_path,
              model_dir = excluded.model_dir,
              tokens_path = excluded.tokens_path,
              host = excluded.host,
              port = excluded.port,
              health_url = excluded.health_url,
              runtime_json = excluded.runtime_json,
              extra_args_json = excluded.extra_args_json,
              auto_start = excluded.auto_start,
              status = excluded.status,
              pid = excluded.pid,
              last_error = excluded.last_error,
              last_exit_code = excluded.last_exit_code,
              created_at = excluded.created_at,
              updated_at = excluded.updated_at
            "#,
            params![
                profile.id,
                format_kind(&profile.kind),
                profile.name,
                profile.enabled as i64,
                format_binary_mode(&profile.binary_mode),
                profile.binary_name,
                profile.binary_path,
                profile.model_package_id,
                profile.model_path,
                profile.model_dir,
                profile.tokens_path,
                profile.host,
                profile.port,
                profile.health_url,
                profile.runtime.to_string(),
                serde_json::to_string(&profile.extra_args)?,
                profile.auto_start as i64,
                format_status(&profile.status),
                profile.pid,
                profile.last_error,
                profile.last_exit_code,
                profile.created_at,
                profile.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_runtime_state(&self) -> AppResult<Vec<EngineRuntimeStateRecord>> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT engine_id, status, pid, health_url, started_at, stopped_at, last_error, last_exit_code, updated_at
            FROM engine_runtime_state
            ORDER BY engine_id
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            let status: String = row.get(1)?;
            Ok(EngineRuntimeStateRecord {
                engine_id: row.get(0)?,
                status: parse_status(&status),
                pid: row.get(2)?,
                health_url: row.get(3)?,
                started_at: row.get(4)?,
                stopped_at: row.get(5)?,
                last_error: row.get(6)?,
                last_exit_code: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn load_runtime_state(&self, id: &str) -> AppResult<Option<EngineRuntimeStateRecord>> {
        Ok(self.list_runtime_state()?.into_iter().find(|state| state.engine_id == id))
    }

    pub fn upsert_runtime_state(&self, state: &EngineRuntimeStateRecord) -> AppResult<()> {
        let conn = self.open()?;
        conn.execute(
            r#"
            INSERT INTO engine_runtime_state (
              engine_id, status, pid, health_url, started_at, stopped_at, last_error, last_exit_code, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(engine_id) DO UPDATE SET
              status = excluded.status,
              pid = excluded.pid,
              health_url = excluded.health_url,
              started_at = excluded.started_at,
              stopped_at = excluded.stopped_at,
              last_error = excluded.last_error,
              last_exit_code = excluded.last_exit_code,
              updated_at = excluded.updated_at
            "#,
            params![
                state.engine_id,
                format_status(&state.status),
                state.pid,
                state.health_url,
                state.started_at,
                state.stopped_at,
                state.last_error,
                state.last_exit_code,
                state.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

fn get_setting(conn: &Connection, key: &str) -> AppResult<Option<String>> {
    Ok(conn
        .query_row("SELECT value FROM app_settings WHERE key = ?", [key], |row| row.get::<_, String>(0))
        .ok())
}

fn set_setting(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
    conn.execute(
        "INSERT INTO app_settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

fn bool_str(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

fn default_model_packages() -> Vec<ModelPackageRecord> {
    let ts = crate::models::now_timestamp();
    vec![
        ModelPackageRecord {
            id: "default_llm".to_string(),
            kind: EngineKind::LlamaCpp,
            display_name: "Language Model".to_string(),
            internal_name: "default_llama_model".to_string(),
            relative_path: "models/llama/default/model.gguf".to_string(),
            required_files: vec!["model.gguf".to_string()],
            manifest_json: serde_json::json!({
                "id": "default_llm",
                "kind": "llama_cpp",
                "displayName": "Language Model",
                "internalName": "default_llama_model",
                "relativePath": "models/llama/default/model.gguf",
                "required": true,
                "sha256": null
            }),
            installed: false,
            verified: false,
            last_verified_at: None,
            created_at: ts.clone(),
            updated_at: ts.clone(),
        },
        ModelPackageRecord {
            id: "default_speech".to_string(),
            kind: EngineKind::SherpaOnnx,
            display_name: "Speech Model".to_string(),
            internal_name: "default_sherpa_model".to_string(),
            relative_path: "models/sherpa/default".to_string(),
            required_files: vec![
                "encoder*.onnx".to_string(),
                "decoder*.onnx".to_string(),
                "joiner*.onnx".to_string(),
                "tokens.txt|config.json".to_string(),
            ],
            manifest_json: serde_json::json!({
                "id": "default_speech",
                "kind": "sherpa_onnx",
                "displayName": "Speech Model",
                "internalName": "default_sherpa_model",
                "relativePath": "models/sherpa/default",
                "requiredFiles": ["encoder*.onnx", "decoder*.onnx", "joiner*.onnx", "tokens.txt|config.json"],
                "required": true,
                "sha256": null,
                "upstreamRepo": "https://github.com/VietInnotech/sherpa-onnx-vit",
                "upstreamCommit": "6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3",
                "modelFamily": "offline_int8"
            }),
            installed: false,
            verified: false,
            last_verified_at: None,
            created_at: ts.clone(),
            updated_at: ts,
        },
    ]
}

fn default_engine_profiles() -> Vec<EngineProfileRecord> {
    let ts = crate::models::now_timestamp();
    vec![
        EngineProfileRecord {
            id: "language_engine".to_string(),
            kind: EngineKind::LlamaCpp,
            name: "Language Engine".to_string(),
            enabled: true,
            binary_mode: BinaryMode::Bundled,
            binary_name: "llama-server".to_string(),
            binary_path: None,
            model_package_id: Some("default_llm".to_string()),
            model_path: Some("models/llama/default/model.gguf".to_string()),
            model_dir: Some("models/llama/default".to_string()),
            tokens_path: None,
            host: "127.0.0.1".to_string(),
            port: 8080,
            health_url: Some("http://127.0.0.1:8080/health".to_string()),
            runtime: serde_json::json!({
                "ctxSize": 4096,
                "gpuLayers": 99,
                "threads": 8,
                "parallel": 1,
                "metrics": false,
                "apiKey": null
            }),
            extra_args: vec![],
            auto_start: false,
            status: EngineStatus::Stopped,
            pid: None,
            last_error: None,
            last_exit_code: None,
            created_at: ts.clone(),
            updated_at: ts.clone(),
        },
        EngineProfileRecord {
            id: "speech_engine".to_string(),
            kind: EngineKind::SherpaOnnx,
            name: "Speech Engine".to_string(),
            enabled: true,
            binary_mode: BinaryMode::Custom,
            binary_name: "python".to_string(),
            binary_path: None,
            model_package_id: Some("default_speech".to_string()),
            model_path: Some("models/sherpa/default".to_string()),
            model_dir: Some("models/sherpa/default".to_string()),
            tokens_path: Some("models/sherpa/default/tokens.txt".to_string()),
            host: "127.0.0.1".to_string(),
            port: 6006,
            health_url: Some("http://127.0.0.1:6006/health".to_string()),
            runtime: serde_json::json!({
                "upstreamRepo": "https://github.com/VietInnotech/sherpa-onnx-vit",
                "upstreamCommit": "6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3",
                "entrypoint": "python_module",
                "moduleName": "sherpa_onnx_vit",
                "packagedRuntimeDir": "runtime/sherpa-onnx-vit",
                "serverType": "http",
                "provider": "cpu",
                "postprocessMode": "clean",
                "sttModelFamily": "offline_int8",
                "numThreads": 2,
                "argsTemplate": [
                    "-m",
                    "sherpa_onnx_vit",
                    "--host",
                    "{host}",
                    "--port",
                    "{port}",
                    "--provider",
                    "{provider}",
                    "--stt-model-family",
                    "{sttModelFamily}",
                    "--model-dir",
                    "{modelDir}",
                    "--postprocess-mode",
                    "{postprocessMode}",
                    "--alias",
                    "{alias}"
                ],
                "alias": "default-speech"
            }),
            extra_args: vec![],
            auto_start: false,
            status: EngineStatus::Stopped,
            pid: None,
            last_error: None,
            last_exit_code: None,
            created_at: ts.clone(),
            updated_at: ts,
        },
    ]
}

fn parse_kind(value: &str) -> EngineKind {
    match value {
        "sherpa_onnx" => EngineKind::SherpaOnnx,
        _ => EngineKind::LlamaCpp,
    }
}

fn format_kind(value: &EngineKind) -> &'static str {
    match value {
        EngineKind::LlamaCpp => "llama_cpp",
        EngineKind::SherpaOnnx => "sherpa_onnx",
    }
}

fn parse_binary_mode(value: &str) -> BinaryMode {
    match value {
        "custom" => BinaryMode::Custom,
        _ => BinaryMode::Bundled,
    }
}

fn format_binary_mode(value: &BinaryMode) -> &'static str {
    match value {
        BinaryMode::Bundled => "bundled",
        BinaryMode::Custom => "custom",
    }
}

fn parse_status(value: &str) -> EngineStatus {
    match value {
        "starting" => EngineStatus::Starting,
        "running" => EngineStatus::Running,
        "stopping" => EngineStatus::Stopping,
        "unhealthy" => EngineStatus::Unhealthy,
        "crashed" => EngineStatus::Crashed,
        "missing_binary" => EngineStatus::MissingBinary,
        "missing_model" => EngineStatus::MissingModel,
        "invalid_config" => EngineStatus::InvalidConfig,
        "port_conflict" => EngineStatus::PortConflict,
        _ => EngineStatus::Stopped,
    }
}

fn format_status(value: &EngineStatus) -> &'static str {
    match value {
        EngineStatus::Stopped => "stopped",
        EngineStatus::Starting => "starting",
        EngineStatus::Running => "running",
        EngineStatus::Unhealthy => "unhealthy",
        EngineStatus::Stopping => "stopping",
        EngineStatus::Crashed => "crashed",
        EngineStatus::MissingBinary => "missing_binary",
        EngineStatus::MissingModel => "missing_model",
        EngineStatus::InvalidConfig => "invalid_config",
        EngineStatus::PortConflict => "port_conflict",
    }
}
