use crate::adapters::resolve_relative_or_absolute;
use crate::app_paths::AppPaths;
use crate::errors::{AppError, AppResult};
use crate::models::{EngineKind, EngineProfileRecord};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub const SHERPA_UPSTREAM_REPO: &str = "https://github.com/VietInnotech/sherpa-onnx-vit";
pub const SHERPA_UPSTREAM_COMMIT: &str = "2ce802dc045dbb306d38085423de5327d45f1d26";
pub const DEFAULT_MODELS_CONFIG_RELATIVE_PATH: &str = "config/sherpa/models.local.json";
pub const DEFAULT_VI_MODEL_ID: &str = "stt-vi";
pub const DEFAULT_EN_MODEL_ID: &str = "stt-en";
pub const DEFAULT_VI_MODEL_RELATIVE_PATH: &str = "models/stt/gipformer-65M-rnnt";
pub const DEFAULT_EN_MODEL_RELATIVE_PATH: &str =
    "models/stt/sherpa-onnx-zipformer-en-libriheavy-20230830-medium-punct-case";
pub const DEFAULT_LANGUAGE: &str = "vi";
pub const DEFAULT_VI_POSTPROCESS_MODE: &str = "capu";
pub const DEFAULT_EN_POSTPROCESS_MODE: &str = "none";
pub const DEFAULT_VAD_MIN_SILENCE: f64 = 0.5;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SherpaModelsConfig {
    #[serde(default)]
    pub models: Vec<SherpaModelEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SherpaModelEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub language: String,
    pub model_dir: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postprocess_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vad_min_silence: Option<f64>,
}

pub fn default_args_template() -> Vec<Value> {
    vec![
        Value::String("-m".to_string()),
        Value::String("sherpa_onnx_vit".to_string()),
        Value::String("--host".to_string()),
        Value::String("{host}".to_string()),
        Value::String("--port".to_string()),
        Value::String("{port}".to_string()),
        Value::String("--provider".to_string()),
        Value::String("{provider}".to_string()),
        Value::String("--models-config".to_string()),
        Value::String("{modelsConfigPath}".to_string()),
        Value::String("--num-threads".to_string()),
        Value::String("{numThreads}".to_string()),
    ]
}

pub fn default_runtime_json() -> Value {
    json!({
        "upstreamRepo": SHERPA_UPSTREAM_REPO,
        "upstreamCommit": SHERPA_UPSTREAM_COMMIT,
        "entrypoint": "python_module",
        "moduleName": "sherpa_onnx_vit",
        "packagedRuntimeDir": "runtime/sherpa-onnx-vit",
        "serverType": "http",
        "provider": "cpu",
        "language": DEFAULT_LANGUAGE,
        "postprocessMode": DEFAULT_VI_POSTPROCESS_MODE,
        "vadMinSilence": DEFAULT_VAD_MIN_SILENCE,
        "numThreads": 2,
        "modelsConfigPath": DEFAULT_MODELS_CONFIG_RELATIVE_PATH,
        "argsTemplate": default_args_template(),
        "alias": "default-speech"
    })
}

pub fn models_config_path_value(profile: &EngineProfileRecord) -> String {
    profile
        .runtime
        .get("modelsConfigPath")
        .and_then(|value| value.as_str())
        .unwrap_or(DEFAULT_MODELS_CONFIG_RELATIVE_PATH)
        .to_string()
}

pub fn resolved_models_config_path(app_root: &Path, profile: &EngineProfileRecord) -> PathBuf {
    resolve_relative_or_absolute(app_root, &models_config_path_value(profile))
}

pub fn is_managed_models_config(profile: &EngineProfileRecord) -> bool {
    profile
        .runtime
        .get("modelsConfigPath")
        .and_then(|value| value.as_str())
        .map(|value| value == DEFAULT_MODELS_CONFIG_RELATIVE_PATH)
        .unwrap_or(true)
}

pub fn looks_like_legacy_args_template(runtime: &Value) -> bool {
    runtime
        .get("argsTemplate")
        .and_then(|value| value.as_array())
        .map(|template| {
            let values = template
                .iter()
                .filter_map(|value| value.as_str())
                .collect::<Vec<_>>();
            values.contains(&"--stt-model-family")
                || values.contains(&"--model-dir")
                || values.contains(&"--alias")
        })
        .unwrap_or(false)
}

pub fn build_managed_models_config(
    app_root: &Path,
    profile: &EngineProfileRecord,
) -> AppResult<SherpaModelsConfig> {
    let Some(model_dir_value) = profile
        .model_dir
        .as_deref()
        .or(profile.model_path.as_deref())
    else {
        return Err(AppError::new(
            "INVALID_MODEL_DIR",
            "Cần có thư mục mô hình Sherpa để tạo cấu hình models.local.json.",
        ));
    };

    let language = profile
        .runtime
        .get("language")
        .and_then(|value| value.as_str())
        .unwrap_or(DEFAULT_LANGUAGE)
        .to_string();
    let postprocess_mode = profile
        .runtime
        .get("postprocessMode")
        .and_then(|value| value.as_str())
        .unwrap_or(DEFAULT_VI_POSTPROCESS_MODE)
        .to_string();
    let vad_min_silence = profile
        .runtime
        .get("vadMinSilence")
        .and_then(|value| value.as_f64())
        .unwrap_or(DEFAULT_VAD_MIN_SILENCE);
    let english_postprocess_mode = profile
        .runtime
        .get("englishPostprocessMode")
        .and_then(|value| value.as_str())
        .unwrap_or(DEFAULT_EN_POSTPROCESS_MODE)
        .to_string();
    let english_vad_min_silence = profile
        .runtime
        .get("englishVadMinSilence")
        .and_then(|value| value.as_f64())
        .unwrap_or(DEFAULT_VAD_MIN_SILENCE);
    let english_model_dir = profile
        .runtime
        .get("englishModelDir")
        .and_then(|value| value.as_str())
        .unwrap_or(DEFAULT_EN_MODEL_RELATIVE_PATH);
    let vi_model_dir = resolve_relative_or_absolute(app_root, model_dir_value)
        .to_string_lossy()
        .to_string();
    let en_model_dir = resolve_relative_or_absolute(app_root, english_model_dir)
        .to_string_lossy()
        .to_string();

    Ok(SherpaModelsConfig {
        models: vec![
            SherpaModelEntry {
                id: Some(DEFAULT_VI_MODEL_ID.to_string()),
                language,
                model_dir: vi_model_dir,
                postprocess_mode: Some(postprocess_mode),
                vad_min_silence: Some(vad_min_silence),
            },
            SherpaModelEntry {
                id: Some(DEFAULT_EN_MODEL_ID.to_string()),
                language: "en".to_string(),
                model_dir: en_model_dir,
                postprocess_mode: Some(english_postprocess_mode),
                vad_min_silence: Some(english_vad_min_silence),
            },
        ],
    })
}

pub fn sync_managed_models_config(
    paths: &AppPaths,
    profile: &EngineProfileRecord,
) -> AppResult<Option<PathBuf>> {
    if profile.kind != EngineKind::SherpaOnnx || !is_managed_models_config(profile) {
        return Ok(None);
    }

    let config = build_managed_models_config(&paths.app_root, profile)?;
    let path = resolved_models_config_path(&paths.app_root, profile);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = format!("{}\n", serde_json::to_string_pretty(&config)?);
    let current = fs::read_to_string(&path).ok();
    if current.as_deref() != Some(content.as_str()) {
        fs::write(&path, content)?;
    }

    Ok(Some(path))
}

pub fn parse_models_config(path: &Path) -> AppResult<SherpaModelsConfig> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{BinaryMode, EngineProfileRecord, EngineStatus};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn profile() -> EngineProfileRecord {
        EngineProfileRecord {
            id: "speech_engine".to_string(),
            kind: EngineKind::SherpaOnnx,
            name: "Speech Engine".to_string(),
            enabled: true,
            binary_mode: BinaryMode::Bundled,
            binary_name: "python".to_string(),
            binary_path: None,
            model_package_id: Some("default_speech".to_string()),
            model_path: None,
            model_dir: Some(DEFAULT_VI_MODEL_RELATIVE_PATH.to_string()),
            tokens_path: None,
            host: "127.0.0.1".to_string(),
            port: 6006,
            health_url: Some("http://127.0.0.1:6006/health".to_string()),
            runtime: default_runtime_json(),
            extra_args: vec![],
            auto_start: false,
            status: EngineStatus::Stopped,
            pid: None,
            last_error: None,
            last_exit_code: None,
            created_at: "0".to_string(),
            updated_at: "0".to_string(),
        }
    }

    fn temp_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!("ai4pro-{name}-{unique}"))
    }

    #[test]
    fn managed_registry_uses_absolute_model_dir_and_defaults() {
        let app_root = PathBuf::from("/tmp/ai4pro-test");
        let config =
            build_managed_models_config(&app_root, &profile()).expect("config should build");

        assert_eq!(config.models.len(), 2);
        assert_eq!(config.models[0].id.as_deref(), Some(DEFAULT_VI_MODEL_ID));
        assert_eq!(config.models[0].language, "vi");
        assert_eq!(config.models[0].postprocess_mode.as_deref(), Some("capu"));
        assert_eq!(config.models[0].vad_min_silence, Some(0.5));
        assert_eq!(
            config.models[0].model_dir,
            "/tmp/ai4pro-test/models/stt/gipformer-65M-rnnt"
        );
        assert_eq!(config.models[1].id.as_deref(), Some(DEFAULT_EN_MODEL_ID));
        assert_eq!(config.models[1].language, "en");
        assert_eq!(config.models[1].postprocess_mode.as_deref(), Some("none"));
        assert_eq!(
            config.models[1].model_dir,
            "/tmp/ai4pro-test/models/stt/sherpa-onnx-zipformer-en-libriheavy-20230830-medium-punct-case"
        );
    }

    #[test]
    fn sync_managed_registry_writes_models_config_file() {
        let root = temp_dir("registry-sync");
        let paths = AppPaths {
            app_root: root.clone(),
            config_dir: root.join("config"),
            data_dir: root.join("data"),
            logs_dir: root.join("logs"),
            models_dir: root.join("models"),
            binaries_dir: root.join("binaries"),
            database_path: root.join("data/local_ai.sqlite"),
        };
        std::fs::create_dir_all(&paths.models_dir).expect("temp dirs should create");

        let written = sync_managed_models_config(&paths, &profile())
            .expect("sync should succeed")
            .expect("managed config path should be returned");
        let parsed = parse_models_config(&written).expect("written config should parse");

        assert_eq!(written, root.join(DEFAULT_MODELS_CONFIG_RELATIVE_PATH));
        assert_eq!(parsed.models.len(), 2);

        let _ = std::fs::remove_dir_all(root);
    }
}
