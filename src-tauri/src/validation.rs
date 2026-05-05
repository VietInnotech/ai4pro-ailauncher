use crate::errors::{AppError, AppResult};
use crate::models::{BinaryMode, EngineKind, EngineProfileRecord, ValidationIssueDto, ValidationResultDto};
use std::path::{Path, PathBuf};

pub fn validate_profile(profile: &EngineProfileRecord, app_root: &Path) -> AppResult<ValidationResultDto> {
    let mut issues = Vec::new();

    if profile.host != "127.0.0.1" && profile.host != "localhost" {
        issues.push(issue("error", "INVALID_HOST", "Host must remain local-only."));
    }

    if profile.port == 0 {
        issues.push(issue("error", "INVALID_PORT", "Port must be between 1 and 65535."));
    }

    if matches!(profile.binary_mode, BinaryMode::Custom) {
        let inferred_runtime_binary = infer_runtime_binary(app_root, profile);
        match &profile.binary_path {
            Some(path) if resolve_path(app_root, path).exists() => {}
            Some(_) => issues.push(issue("error", "MISSING_BINARY", "Custom binary path is unavailable.")),
            None if inferred_runtime_binary.as_ref().is_some_and(|path| path.exists()) => {}
            None => issues.push(issue("error", "MISSING_BINARY", "Custom binary path is required or the packaged runtime is missing.")),
        }
    }

    match profile.kind {
        EngineKind::LlamaCpp => validate_llama_profile(profile, app_root, &mut issues),
        EngineKind::SherpaOnnx => validate_sherpa_profile(profile, app_root, &mut issues),
    }

    Ok(ValidationResultDto {
        engine_id: profile.id.clone(),
        valid: issues.iter().all(|issue| issue.severity != "error"),
        issues,
        generated_args: vec![],
    })
}

pub fn validate_update_input(input: &crate::models::UpdateEngineProfileInput) -> AppResult<()> {
    if let Some(port) = input.port {
        if port == 0 {
            return Err(AppError::new("INVALID_PORT", "Port must be between 1 and 65535."));
        }
    }

    if let Some(host) = &input.host {
        if host != "127.0.0.1" && host != "localhost" {
            return Err(AppError::new("INVALID_HOST", "Host must remain local-only."));
        }
    }

    Ok(())
}

fn validate_llama_profile(profile: &EngineProfileRecord, app_root: &Path, issues: &mut Vec<ValidationIssueDto>) {
    match &profile.model_path {
        Some(path) => {
            let resolved = resolve_path(app_root, path);
            if !resolved.exists() {
                issues.push(issue("error", "MISSING_MODEL", "GGUF model file is unavailable."));
            } else if resolved.extension().and_then(|ext| ext.to_str()) != Some("gguf") {
                issues.push(issue("error", "INVALID_MODEL_PATH", "Llama model file must use the .gguf extension."));
            }
        }
        None => issues.push(issue("error", "MISSING_MODEL", "GGUF model path is required.")),
    }
}

fn validate_sherpa_profile(profile: &EngineProfileRecord, app_root: &Path, issues: &mut Vec<ValidationIssueDto>) {
    let model_dir = profile.model_dir.as_ref().or(profile.model_path.as_ref());

    match model_dir {
        Some(path) => {
            let resolved = resolve_path(app_root, path);
            if !resolved.exists() {
                issues.push(issue("error", "MISSING_MODEL", "Sherpa model directory is unavailable."));
                return;
            }
            if !resolved.is_dir() {
                issues.push(issue("error", "INVALID_MODEL_DIR", "Sherpa model path must be a directory."));
                return;
            }

            if !has_match(&resolved, "encoder") {
                issues.push(issue("error", "MISSING_MODEL", "Sherpa model directory is missing encoder*.onnx."));
            }
            if !has_match(&resolved, "decoder") {
                issues.push(issue("error", "MISSING_MODEL", "Sherpa model directory is missing decoder*.onnx."));
            }
            if !has_match(&resolved, "joiner") {
                issues.push(issue("error", "MISSING_MODEL", "Sherpa model directory is missing joiner*.onnx."));
            }

            let has_tokens = resolved.join("tokens.txt").exists();
            let has_config = resolved.join("config.json").exists();
            if !has_tokens && !has_config {
                issues.push(issue("error", "INVALID_TOKENS_PATH", "Sherpa model directory must contain tokens.txt or config.json."));
            }

            if let Some(tokens_path) = &profile.tokens_path {
                let resolved_tokens = resolve_path(app_root, tokens_path);
                if !resolved_tokens.exists() {
                    issues.push(issue("warning", "INVALID_TOKENS_PATH", "Configured tokens path is unavailable; relying on model-dir-first resolution."));
                }
            }
        }
        None => issues.push(issue("error", "INVALID_MODEL_DIR", "Sherpa model directory is required.")),
    }

    match profile.runtime.get("entrypoint").and_then(|value| value.as_str()) {
        Some("python_module") | Some("console_script") => {}
        Some(_) => issues.push(issue("error", "INVALID_CONFIG", "Sherpa runtime entrypoint must be python_module or console_script.")),
        None => issues.push(issue("warning", "CONFIG_PARSE_ERROR", "Sherpa runtime entrypoint is missing; defaults will be used.")),
    }

    if profile.runtime.get("argsTemplate").and_then(|value| value.as_array()).is_none() {
        issues.push(issue("warning", "CONFIG_PARSE_ERROR", "Sherpa args template is missing; defaults will be used."));
    }

    if profile.runtime.get("packagedRuntimeDir").and_then(|value| value.as_str()).is_none() {
        issues.push(issue("warning", "CONFIG_PARSE_ERROR", "Sherpa packaged runtime directory is missing."));
    }
}

fn issue(severity: &str, code: &str, message: &str) -> ValidationIssueDto {
    ValidationIssueDto {
        severity: severity.to_string(),
        code: code.to_string(),
        message: message.to_string(),
    }
}

fn resolve_path(app_root: &Path, value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        app_root.join(path)
    }
}

fn infer_runtime_binary(app_root: &Path, profile: &EngineProfileRecord) -> Option<PathBuf> {
    if !matches!(profile.kind, EngineKind::SherpaOnnx) {
        return None;
    }

    let runtime_dir = profile
        .runtime
        .get("packagedRuntimeDir")
        .and_then(|value| value.as_str())?;
    let file_name = if cfg!(target_os = "windows") { "python.exe" } else { "python3" };
    Some(resolve_path(app_root, runtime_dir).join(file_name))
}

fn has_match(dir: &Path, prefix: &str) -> bool {
    dir.read_dir()
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .any(|entry| {
            let path = entry.path();
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with(prefix) && name.ends_with(".onnx"))
                .unwrap_or(false)
        })
}
