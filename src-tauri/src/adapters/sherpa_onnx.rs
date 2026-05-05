use super::EngineAdapter;
use crate::app_paths::AppPaths;
use crate::errors::{AppError, AppResult};
use crate::models::{EngineKind, EngineProfileRecord};
use crate::process_supervisor::LaunchSpec;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct SherpaOnnxAdapter;

impl EngineAdapter for SherpaOnnxAdapter {
    fn kind(&self) -> EngineKind {
        EngineKind::SherpaOnnx
    }

    fn binary_name(&self) -> &'static str {
        if cfg!(target_os = "windows") {
            "python.exe"
        } else {
            "python3"
        }
    }

    fn launch_spec(&self, paths: &AppPaths, profile: &EngineProfileRecord) -> AppResult<LaunchSpec> {
        let runtime = &profile.runtime;
        let model_dir = profile
            .model_dir
            .clone()
            .or(profile.model_path.clone())
            .ok_or_else(|| AppError::new("INVALID_MODEL_DIR", "A sherpa model directory is required."))?;
        let provider = runtime
            .get("provider")
            .and_then(|value| value.as_str())
            .unwrap_or("cpu");
        let family = runtime
            .get("sttModelFamily")
            .and_then(|value| value.as_str())
            .unwrap_or("offline_int8");
        let postprocess = runtime
            .get("postprocessMode")
            .and_then(|value| value.as_str())
            .unwrap_or("clean");
        let alias = runtime
            .get("alias")
            .and_then(|value| value.as_str())
            .unwrap_or(&profile.id);
        let encoder = profile.model_path.clone().unwrap_or_default();
        let decoder = runtime
            .get("modelDecoder")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        let joiner = runtime
            .get("modelJoiner")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        let tokens = profile.tokens_path.clone().unwrap_or_default();
        let bpe = runtime
            .get("modelBpeVocab")
            .and_then(|value| value.as_str())
            .unwrap_or("");

        let binary_path = runtime_python_binary(paths, profile, self.binary_name());
        let module_name = runtime
            .get("moduleName")
            .and_then(|value| value.as_str())
            .unwrap_or("sherpa_onnx_vit");
        let template = runtime
            .get("argsTemplate")
            .and_then(|value| value.as_array())
            .cloned()
            .unwrap_or_else(|| {
                vec![
                    serde_json::Value::String("-m".to_string()),
                    serde_json::Value::String("sherpa_onnx_vit".to_string()),
                    serde_json::Value::String("--host".to_string()),
                    serde_json::Value::String("{host}".to_string()),
                    serde_json::Value::String("--port".to_string()),
                    serde_json::Value::String("{port}".to_string()),
                    serde_json::Value::String("--provider".to_string()),
                    serde_json::Value::String("{provider}".to_string()),
                    serde_json::Value::String("--stt-model-family".to_string()),
                    serde_json::Value::String("{sttModelFamily}".to_string()),
                    serde_json::Value::String("--model-dir".to_string()),
                    serde_json::Value::String("{modelDir}".to_string()),
                    serde_json::Value::String("--postprocess-mode".to_string()),
                    serde_json::Value::String("{postprocessMode}".to_string()),
                    serde_json::Value::String("--alias".to_string()),
                    serde_json::Value::String("{alias}".to_string()),
                ]
            });

        let mut args = template
            .into_iter()
            .filter_map(|value| value.as_str().map(str::to_string))
            .map(|value| {
                value
                    .replace("{moduleName}", module_name)
                    .replace("{host}", &profile.host)
                    .replace("{port}", &profile.port.to_string())
                    .replace("{provider}", provider)
                    .replace("{sttModelFamily}", family)
                    .replace("{modelDir}", &model_dir)
                    .replace("{postprocessMode}", postprocess)
                    .replace("{alias}", alias)
                    .replace("{modelEncoder}", &encoder)
                    .replace("{modelDecoder}", decoder)
                    .replace("{modelJoiner}", joiner)
                    .replace("{modelTokens}", &tokens)
                    .replace("{modelBpeVocab}", bpe)
            })
            .collect::<Vec<_>>();
        args.extend(profile.extra_args.clone());

        let mut env = vec![];
        if let Some(runtime_dir) = runtime.get("packagedRuntimeDir").and_then(|value| value.as_str()) {
            let resolved = paths.app_root.join(runtime_dir);
            env.push((
                if cfg!(target_os = "windows") { "PYTHONHOME".into() } else { "PYTHONPATH".into() },
                resolved.to_string_lossy().to_string(),
            ));
        }

        Ok(LaunchSpec {
            id: profile.id.clone(),
            binary_path,
            args,
            env,
            log_root: paths.logs_dir.clone(),
        })
    }
}

fn runtime_python_binary(paths: &AppPaths, profile: &EngineProfileRecord, default_binary: &str) -> PathBuf {
    if let Some(path) = &profile.binary_path {
        PathBuf::from(path)
    } else {
        let runtime_dir = profile
            .runtime
            .get("packagedRuntimeDir")
            .and_then(|value| value.as_str())
            .unwrap_or("runtime/sherpa-onnx-vit");
        let filename = if cfg!(target_os = "windows") {
            "python.exe"
        } else {
            default_binary
        };
        paths.app_root.join(runtime_dir).join(filename)
    }
}
