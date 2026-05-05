use crate::errors::{AppError, AppResult};
use crate::models::{BinaryMode, EngineKind, EngineProfileRecord, ValidationIssueDto, ValidationResultDto};
use std::path::{Path, PathBuf};

pub fn validate_profile(profile: &EngineProfileRecord, app_root: &Path) -> AppResult<ValidationResultDto> {
    let mut issues = Vec::new();

    if profile.host != "127.0.0.1" && profile.host != "localhost" {
        issues.push(issue("error", "INVALID_HOST", "Máy chủ phải chỉ cho phép truy cập cục bộ."));
    }

    if profile.port == 0 {
        issues.push(issue("error", "INVALID_PORT", "Cổng phải nằm trong khoảng từ 1 đến 65535."));
    }

    if matches!(profile.binary_mode, BinaryMode::Custom) {
        let inferred_runtime_binary = infer_runtime_binary(app_root, profile);
        match &profile.binary_path {
            Some(path) if resolve_path(app_root, path).exists() => {}
            Some(_) => issues.push(issue("error", "MISSING_BINARY", "Đường dẫn tệp nhị phân tùy chỉnh hiện không khả dụng.")),
            None if inferred_runtime_binary.as_ref().is_some_and(|path| path.exists()) => {}
            None => issues.push(issue("error", "MISSING_BINARY", "Cần có đường dẫn tệp nhị phân tùy chỉnh hoặc thiếu môi trường chạy được đóng gói.")),
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
            return Err(AppError::new("INVALID_PORT", "Cổng phải nằm trong khoảng từ 1 đến 65535."));
        }
    }

    if let Some(host) = &input.host {
        if host != "127.0.0.1" && host != "localhost" {
            return Err(AppError::new("INVALID_HOST", "Máy chủ phải chỉ cho phép truy cập cục bộ."));
        }
    }

    Ok(())
}

fn validate_llama_profile(profile: &EngineProfileRecord, app_root: &Path, issues: &mut Vec<ValidationIssueDto>) {
    match &profile.model_path {
        Some(path) => {
            let resolved = resolve_path(app_root, path);
            if !resolved.exists() {
                issues.push(issue("error", "MISSING_MODEL", "Tệp mô hình GGUF hiện không khả dụng."));
            } else if resolved.extension().and_then(|ext| ext.to_str()) != Some("gguf") {
                issues.push(issue("error", "INVALID_MODEL_PATH", "Tệp mô hình Llama phải dùng phần mở rộng .gguf."));
            }
        }
        None => issues.push(issue("error", "MISSING_MODEL", "Cần có đường dẫn mô hình GGUF.")),
    }
}

fn validate_sherpa_profile(profile: &EngineProfileRecord, app_root: &Path, issues: &mut Vec<ValidationIssueDto>) {
    let model_dir = profile.model_dir.as_ref().or(profile.model_path.as_ref());

    match model_dir {
        Some(path) => {
            let resolved = resolve_path(app_root, path);
            if !resolved.exists() {
                issues.push(issue("error", "MISSING_MODEL", "Thư mục mô hình Sherpa hiện không khả dụng."));
                return;
            }
            if !resolved.is_dir() {
                issues.push(issue("error", "INVALID_MODEL_DIR", "Đường dẫn mô hình Sherpa phải là một thư mục."));
                return;
            }

            if !has_match(&resolved, "encoder") {
                issues.push(issue("error", "MISSING_MODEL", "Thư mục mô hình Sherpa thiếu encoder*.onnx."));
            }
            if !has_match(&resolved, "decoder") {
                issues.push(issue("error", "MISSING_MODEL", "Thư mục mô hình Sherpa thiếu decoder*.onnx."));
            }
            if !has_match(&resolved, "joiner") {
                issues.push(issue("error", "MISSING_MODEL", "Thư mục mô hình Sherpa thiếu joiner*.onnx."));
            }

            let has_tokens = resolved.join("tokens.txt").exists();
            let has_config = resolved.join("config.json").exists();
            if !has_tokens && !has_config {
                issues.push(issue("error", "INVALID_TOKENS_PATH", "Thư mục mô hình Sherpa phải chứa tokens.txt hoặc config.json."));
            }

            if let Some(tokens_path) = &profile.tokens_path {
                let resolved_tokens = resolve_path(app_root, tokens_path);
                if !resolved_tokens.exists() {
                    issues.push(issue("warning", "INVALID_TOKENS_PATH", "Đường dẫn tokens đã cấu hình hiện không khả dụng; sẽ dùng cách phân giải ưu tiên thư mục mô hình."));
                }
            }
        }
        None => issues.push(issue("error", "INVALID_MODEL_DIR", "Cần có thư mục mô hình Sherpa.")),
    }

    match profile.runtime.get("entrypoint").and_then(|value| value.as_str()) {
        Some("python_module") | Some("console_script") => {}
        Some(_) => issues.push(issue("error", "INVALID_CONFIG", "Entrypoint runtime của Sherpa phải là python_module hoặc console_script.")),
        None => issues.push(issue("warning", "CONFIG_PARSE_ERROR", "Thiếu entrypoint runtime của Sherpa; sẽ dùng giá trị mặc định.")),
    }

    if profile.runtime.get("argsTemplate").and_then(|value| value.as_array()).is_none() {
        issues.push(issue("warning", "CONFIG_PARSE_ERROR", "Thiếu mẫu đối số của Sherpa; sẽ dùng giá trị mặc định."));
    }

    if profile.runtime.get("packagedRuntimeDir").and_then(|value| value.as_str()).is_none() {
        issues.push(issue("warning", "CONFIG_PARSE_ERROR", "Thiếu thư mục runtime đóng gói của Sherpa."));
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
