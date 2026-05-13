use crate::errors::AppResult;
use crate::sherpa_registry::{
    default_args_template, looks_like_legacy_args_template, DEFAULT_EN_MODEL_RELATIVE_PATH,
    DEFAULT_EN_POSTPROCESS_MODE, DEFAULT_LANGUAGE, DEFAULT_MODELS_CONFIG_RELATIVE_PATH,
    DEFAULT_VAD_MIN_SILENCE, DEFAULT_VI_MODEL_RELATIVE_PATH, DEFAULT_VI_POSTPROCESS_MODE,
    SHERPA_UPSTREAM_COMMIT,
};
use rusqlite::Connection;

pub const CURRENT_SCHEMA_VERSION: i64 = 4;

pub fn migrate(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;

        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS model_packages (
            id TEXT PRIMARY KEY,
            kind TEXT NOT NULL,
            display_name TEXT NOT NULL,
            internal_name TEXT NOT NULL,
            relative_path TEXT NOT NULL,
            required_files_json TEXT NOT NULL,
            manifest_json TEXT NOT NULL,
            installed INTEGER NOT NULL,
            verified INTEGER NOT NULL,
            last_verified_at TEXT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS engine_profiles (
            id TEXT PRIMARY KEY,
            kind TEXT NOT NULL,
            name TEXT NOT NULL,
            enabled INTEGER NOT NULL,
            binary_mode TEXT NOT NULL,
            binary_name TEXT NOT NULL,
            binary_path TEXT NULL,
            model_package_id TEXT NULL,
            model_path TEXT NULL,
            model_dir TEXT NULL,
            tokens_path TEXT NULL,
            host TEXT NOT NULL,
            port INTEGER NOT NULL,
            health_url TEXT NULL,
            runtime_json TEXT NOT NULL,
            extra_args_json TEXT NOT NULL,
            auto_start INTEGER NOT NULL,
            status TEXT NOT NULL,
            pid INTEGER NULL,
            last_error TEXT NULL,
            last_exit_code INTEGER NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS engine_runtime_state (
            engine_id TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            pid INTEGER NULL,
            health_url TEXT NULL,
            started_at TEXT NULL,
            stopped_at TEXT NULL,
            last_error TEXT NULL,
            last_exit_code INTEGER NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS log_files (
            id TEXT PRIMARY KEY,
            engine_id TEXT NULL,
            kind TEXT NOT NULL,
            path TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )?;

    let current_version: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if current_version < 4 {
        migrate_sherpa_registry_contract(conn)?;
    }

    if current_version < CURRENT_SCHEMA_VERSION {
        conn.execute("DELETE FROM schema_version", [])?;
        conn.execute(
            "INSERT INTO schema_version (version) VALUES (?)",
            [CURRENT_SCHEMA_VERSION],
        )?;
    }

    Ok(())
}

fn migrate_sherpa_registry_contract(conn: &Connection) -> AppResult<()> {
    let mut stmt = conn.prepare(
        "SELECT id, model_dir, runtime_json, updated_at FROM engine_profiles WHERE kind = 'sherpa_onnx'",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    })?;

    let default_template = serde_json::Value::Array(default_args_template());
    for row in rows {
        let (id, model_dir, runtime_json, updated_at) = row?;
        let mut runtime = serde_json::from_str::<serde_json::Value>(&runtime_json)
            .unwrap_or_else(|_| serde_json::json!({}));

        let is_old_commit = runtime
            .get("upstreamCommit")
            .and_then(|value| value.as_str())
            == Some("6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3");
        let is_legacy_template = looks_like_legacy_args_template(&runtime);
        let is_legacy_model_dir = model_dir.as_deref() == Some("models/sherpa/default");
        let should_update_contract = is_old_commit || is_legacy_template;
        let next_model_dir = if is_legacy_model_dir || should_update_contract {
            Some(DEFAULT_VI_MODEL_RELATIVE_PATH.to_string())
        } else {
            model_dir
        };

        if should_update_contract {
            runtime["upstreamCommit"] =
                serde_json::Value::String(SHERPA_UPSTREAM_COMMIT.to_string());
            runtime["argsTemplate"] = default_template.clone();
        }

        if runtime
            .get("modelsConfigPath")
            .and_then(|value| value.as_str())
            .is_none()
        {
            runtime["modelsConfigPath"] =
                serde_json::Value::String(DEFAULT_MODELS_CONFIG_RELATIVE_PATH.to_string());
        }
        if runtime
            .get("language")
            .and_then(|value| value.as_str())
            .is_none()
        {
            runtime["language"] = serde_json::Value::String(DEFAULT_LANGUAGE.to_string());
        }
        if runtime
            .get("vadMinSilence")
            .and_then(|value| value.as_f64())
            .is_none()
        {
            runtime["vadMinSilence"] = serde_json::Value::from(DEFAULT_VAD_MIN_SILENCE);
        }

        if runtime
            .get("postprocessMode")
            .and_then(|value| value.as_str())
            .is_none()
            || should_update_contract
            || runtime
                .get("postprocessMode")
                .and_then(|value| value.as_str())
                == Some("clean")
        {
            runtime["postprocessMode"] =
                serde_json::Value::String(DEFAULT_VI_POSTPROCESS_MODE.to_string());
        }

        conn.execute(
            "UPDATE engine_profiles SET model_dir = ?, runtime_json = ?, updated_at = ? WHERE id = ?",
            rusqlite::params![
                next_model_dir,
                runtime.to_string(),
                updated_at,
                id
            ],
        )?;
    }

    let now = crate::models::now_timestamp();
    let mut stmt = conn.prepare(
        "SELECT manifest_json, relative_path FROM model_packages WHERE id IN ('default_speech', 'default_speech_en')",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut seen_default_speech = false;
    let mut seen_default_speech_en = false;

    for row in rows {
        let (manifest_json, relative_path) = row?;
        let mut manifest = serde_json::from_str::<serde_json::Value>(&manifest_json)
            .unwrap_or_else(|_| serde_json::json!({}));
        let package_id = manifest
            .get("id")
            .and_then(|value| value.as_str())
            .unwrap_or("");

        if package_id == "default_speech" {
            seen_default_speech = true;
            if relative_path == "models/sherpa/default" {
                manifest["relativePath"] =
                    serde_json::Value::String(DEFAULT_VI_MODEL_RELATIVE_PATH.to_string());
            }
            manifest["displayName"] =
                serde_json::Value::String("Vietnamese Speech Model".to_string());
            manifest["internalName"] =
                serde_json::Value::String("default_sherpa_vi_model".to_string());
            manifest["upstreamCommit"] =
                serde_json::Value::String(SHERPA_UPSTREAM_COMMIT.to_string());
            manifest["language"] = serde_json::Value::String("vi".to_string());
            manifest["postprocessMode"] =
                serde_json::Value::String(DEFAULT_VI_POSTPROCESS_MODE.to_string());
            conn.execute(
                "UPDATE model_packages SET display_name = ?, internal_name = ?, relative_path = ?, manifest_json = ?, updated_at = ? WHERE id = 'default_speech'",
                rusqlite::params![
                    "Vietnamese Speech Model",
                    "default_sherpa_vi_model",
                    DEFAULT_VI_MODEL_RELATIVE_PATH,
                    manifest.to_string(),
                    now,
                ],
            )?;
        } else if package_id == "default_speech_en" {
            seen_default_speech_en = true;
            manifest["displayName"] = serde_json::Value::String("English Speech Model".to_string());
            manifest["internalName"] =
                serde_json::Value::String("default_sherpa_en_model".to_string());
            manifest["relativePath"] =
                serde_json::Value::String(DEFAULT_EN_MODEL_RELATIVE_PATH.to_string());
            manifest["upstreamCommit"] =
                serde_json::Value::String(SHERPA_UPSTREAM_COMMIT.to_string());
            manifest["language"] = serde_json::Value::String("en".to_string());
            manifest["postprocessMode"] =
                serde_json::Value::String(DEFAULT_EN_POSTPROCESS_MODE.to_string());
            conn.execute(
                "UPDATE model_packages SET display_name = ?, internal_name = ?, relative_path = ?, manifest_json = ?, updated_at = ? WHERE id = 'default_speech_en'",
                rusqlite::params![
                    "English Speech Model",
                    "default_sherpa_en_model",
                    DEFAULT_EN_MODEL_RELATIVE_PATH,
                    manifest.to_string(),
                    now,
                ],
            )?;
        }
    }

    if !seen_default_speech {
        conn.execute(
            "INSERT INTO model_packages (id, kind, display_name, internal_name, relative_path, required_files_json, manifest_json, installed, verified, last_verified_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, 0, 0, NULL, ?, ?)",
            rusqlite::params![
                "default_speech",
                "sherpa_onnx",
                "Vietnamese Speech Model",
                "default_sherpa_vi_model",
                DEFAULT_VI_MODEL_RELATIVE_PATH,
                serde_json::to_string(&vec![
                    "encoder*.onnx",
                    "decoder*.onnx",
                    "joiner*.onnx",
                    "tokens.txt|config.json"
                ])?,
                serde_json::json!({
                    "id": "default_speech",
                    "kind": "sherpa_onnx",
                    "displayName": "Vietnamese Speech Model",
                    "internalName": "default_sherpa_vi_model",
                    "relativePath": DEFAULT_VI_MODEL_RELATIVE_PATH,
                    "requiredFiles": ["encoder*.onnx", "decoder*.onnx", "joiner*.onnx", "tokens.txt|config.json"],
                    "required": true,
                    "sha256": null,
                    "upstreamRepo": "https://github.com/VietInnotech/sherpa-onnx-vit",
                    "upstreamCommit": SHERPA_UPSTREAM_COMMIT,
                    "modelFamily": "offline_int8",
                    "language": "vi",
                    "postprocessMode": DEFAULT_VI_POSTPROCESS_MODE
                })
                .to_string(),
                now,
                now,
            ],
        )?;
    }

    if !seen_default_speech_en {
        conn.execute(
            "INSERT INTO model_packages (id, kind, display_name, internal_name, relative_path, required_files_json, manifest_json, installed, verified, last_verified_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, 0, 0, NULL, ?, ?)",
            rusqlite::params![
                "default_speech_en",
                "sherpa_onnx",
                "English Speech Model",
                "default_sherpa_en_model",
                DEFAULT_EN_MODEL_RELATIVE_PATH,
                serde_json::to_string(&vec![
                    "encoder*.onnx",
                    "decoder*.onnx",
                    "joiner*.onnx",
                    "tokens.txt|config.json"
                ])?,
                serde_json::json!({
                    "id": "default_speech_en",
                    "kind": "sherpa_onnx",
                    "displayName": "English Speech Model",
                    "internalName": "default_sherpa_en_model",
                    "relativePath": DEFAULT_EN_MODEL_RELATIVE_PATH,
                    "requiredFiles": ["encoder*.onnx", "decoder*.onnx", "joiner*.onnx", "tokens.txt|config.json"],
                    "required": true,
                    "sha256": null,
                    "upstreamRepo": "https://github.com/VietInnotech/sherpa-onnx-vit",
                    "upstreamCommit": SHERPA_UPSTREAM_COMMIT,
                    "modelFamily": "offline_int8",
                    "language": "en",
                    "postprocessMode": DEFAULT_EN_POSTPROCESS_MODE
                })
                .to_string(),
                now,
                now,
            ],
        )?;
    }

    Ok(())
}
