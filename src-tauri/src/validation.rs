use crate::errors::{AppError, AppResult};
use crate::models::{
    BinaryMode, EngineKind, EngineProfileRecord, ValidationIssueDto, ValidationResultDto,
};
use crate::sherpa_registry::{parse_models_config, resolved_models_config_path, SherpaModelEntry};
use std::path::{Path, PathBuf};

pub fn validate_profile(
    profile: &EngineProfileRecord,
    app_root: &Path,
) -> AppResult<ValidationResultDto> {
    let mut issues = Vec::new();

    if profile.host != "127.0.0.1" && profile.host != "localhost" {
        issues.push(issue(
            "error",
            "INVALID_HOST",
            "Máy chủ phải chỉ cho phép truy cập cục bộ.",
        ));
    }

    if profile.port == 0 {
        issues.push(issue(
            "error",
            "INVALID_PORT",
            "Cổng phải nằm trong khoảng từ 1 đến 65535.",
        ));
    }

    if matches!(profile.binary_mode, BinaryMode::Custom) {
        let inferred_runtime_binary = infer_runtime_binary(app_root, profile);
        match &profile.binary_path {
            Some(path) if resolve_path(app_root, path).exists() => {}
            Some(_) => issues.push(issue(
                "error",
                "MISSING_BINARY",
                "Đường dẫn tệp nhị phân tùy chỉnh hiện không khả dụng.",
            )),
            None if inferred_runtime_binary
                .as_ref()
                .is_some_and(|path| path.exists()) => {}
            None => issues.push(issue(
                "error",
                "MISSING_BINARY",
                "Cần có đường dẫn tệp nhị phân tùy chỉnh hoặc thiếu môi trường chạy được đóng gói.",
            )),
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
            return Err(AppError::new(
                "INVALID_PORT",
                "Cổng phải nằm trong khoảng từ 1 đến 65535.",
            ));
        }
    }

    if let Some(host) = &input.host {
        if host != "127.0.0.1" && host != "localhost" {
            return Err(AppError::new(
                "INVALID_HOST",
                "Máy chủ phải chỉ cho phép truy cập cục bộ.",
            ));
        }
    }

    Ok(())
}

fn validate_llama_profile(
    profile: &EngineProfileRecord,
    app_root: &Path,
    issues: &mut Vec<ValidationIssueDto>,
) {
    match &profile.model_path {
        Some(path) => {
            let resolved = resolve_path(app_root, path);
            if !resolved.exists() {
                issues.push(issue(
                    "error",
                    "MISSING_MODEL",
                    "Tệp mô hình GGUF hiện không khả dụng.",
                ));
            } else if resolved.extension().and_then(|ext| ext.to_str()) != Some("gguf") {
                issues.push(issue(
                    "error",
                    "INVALID_MODEL_PATH",
                    "Tệp mô hình Llama phải dùng phần mở rộng .gguf.",
                ));
            }
        }
        None => issues.push(issue(
            "error",
            "MISSING_MODEL",
            "Cần có đường dẫn mô hình GGUF.",
        )),
    }
}

fn validate_sherpa_profile(
    profile: &EngineProfileRecord,
    app_root: &Path,
    issues: &mut Vec<ValidationIssueDto>,
) {
    validate_profile_model_dir(profile, app_root, issues);
    validate_models_config(profile, app_root, issues);

    match profile
        .runtime
        .get("entrypoint")
        .and_then(|value| value.as_str())
    {
        Some("python_module") | Some("console_script") => {}
        Some(_) => issues.push(issue(
            "error",
            "INVALID_CONFIG",
            "Entrypoint runtime của Sherpa phải là python_module hoặc console_script.",
        )),
        None => issues.push(issue(
            "warning",
            "CONFIG_PARSE_ERROR",
            "Thiếu entrypoint runtime của Sherpa; sẽ dùng giá trị mặc định.",
        )),
    }

    if profile
        .runtime
        .get("argsTemplate")
        .and_then(|value| value.as_array())
        .is_none()
    {
        issues.push(issue(
            "warning",
            "CONFIG_PARSE_ERROR",
            "Thiếu mẫu đối số của Sherpa; sẽ dùng giá trị mặc định.",
        ));
    }

    if profile
        .runtime
        .get("packagedRuntimeDir")
        .and_then(|value| value.as_str())
        .is_none()
    {
        issues.push(issue(
            "warning",
            "CONFIG_PARSE_ERROR",
            "Thiếu thư mục runtime đóng gói của Sherpa.",
        ));
    }
}

fn validate_profile_model_dir(
    profile: &EngineProfileRecord,
    app_root: &Path,
    issues: &mut Vec<ValidationIssueDto>,
) {
    let model_dir = profile.model_dir.as_ref().or(profile.model_path.as_ref());

    match model_dir {
        Some(path) => {
            let resolved = resolve_path(app_root, path);
            if !resolved.exists() {
                issues.push(issue(
                    "error",
                    "MISSING_MODEL",
                    "Thư mục mô hình Sherpa hiện không khả dụng.",
                ));
                return;
            }
            if !resolved.is_dir() {
                issues.push(issue(
                    "error",
                    "INVALID_MODEL_DIR",
                    "Đường dẫn mô hình Sherpa phải là một thư mục.",
                ));
                return;
            }

            if !has_match(&resolved, "encoder") {
                issues.push(issue(
                    "error",
                    "MISSING_MODEL",
                    "Thư mục mô hình Sherpa thiếu encoder*.onnx.",
                ));
            }
            if !has_match(&resolved, "decoder") {
                issues.push(issue(
                    "error",
                    "MISSING_MODEL",
                    "Thư mục mô hình Sherpa thiếu decoder*.onnx.",
                ));
            }
            if !has_match(&resolved, "joiner") {
                issues.push(issue(
                    "error",
                    "MISSING_MODEL",
                    "Thư mục mô hình Sherpa thiếu joiner*.onnx.",
                ));
            }

            let has_tokens = resolved.join("tokens.txt").exists();
            let has_config = resolved.join("config.json").exists();
            if !has_tokens && !has_config {
                issues.push(issue(
                    "error",
                    "INVALID_TOKENS_PATH",
                    "Thư mục mô hình Sherpa phải chứa tokens.txt hoặc config.json.",
                ));
            }

            if let Some(tokens_path) = &profile.tokens_path {
                let resolved_tokens = resolve_path(app_root, tokens_path);
                if !resolved_tokens.exists() {
                    issues.push(issue("warning", "INVALID_TOKENS_PATH", "Đường dẫn tokens đã cấu hình hiện không khả dụng; sẽ dùng cách phân giải ưu tiên thư mục mô hình."));
                }
            }
        }
        None => issues.push(issue(
            "error",
            "INVALID_MODEL_DIR",
            "Cần có thư mục mô hình Sherpa.",
        )),
    }
}

fn validate_models_config(
    profile: &EngineProfileRecord,
    app_root: &Path,
    issues: &mut Vec<ValidationIssueDto>,
) {
    let models_config_path = resolved_models_config_path(app_root, profile);
    if !models_config_path.exists() {
        issues.push(issue(
            "error",
            "MISSING_MODELS_CONFIG",
            "Thiếu tệp cấu hình models.local.json của Sherpa.",
        ));
        return;
    }

    if !models_config_path.is_file() {
        issues.push(issue(
            "error",
            "INVALID_MODELS_CONFIG",
            "Đường dẫn cấu hình models.local.json của Sherpa phải là một tệp JSON.",
        ));
        return;
    }

    let config = match parse_models_config(&models_config_path) {
        Ok(config) => config,
        Err(error) => {
            issues.push(issue(
                "error",
                "INVALID_MODELS_CONFIG",
                &format!(
                    "Không thể đọc hoặc phân tích models.local.json của Sherpa: {}",
                    error.message
                ),
            ));
            return;
        }
    };

    if config.models.is_empty() {
        issues.push(issue(
            "error",
            "EMPTY_MODELS_CONFIG",
            "Tệp cấu hình models.local.json của Sherpa phải chứa ít nhất một mục trong models.",
        ));
        return;
    }

    for (index, model) in config.models.iter().enumerate() {
        validate_registry_model(index, model, app_root, issues);
    }
}

fn validate_registry_model(
    index: usize,
    model: &SherpaModelEntry,
    app_root: &Path,
    issues: &mut Vec<ValidationIssueDto>,
) {
    let label = model
        .id
        .as_deref()
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("model[{}]", index));

    let language = model.language.trim();
    if language != "vi" && language != "en" {
        issues.push(issue(
            "error",
            "INVALID_MODELS_CONFIG",
            &format!("Mục {label} dùng language không được hỗ trợ: {language}."),
        ));
        return;
    }

    let resolved_dir = resolve_path(app_root, &model.model_dir);
    if !resolved_dir.exists() || !resolved_dir.is_dir() {
        issues.push(issue(
            "error",
            "MISSING_REGISTRY_MODEL_DIR",
            &format!("Mục {label} có model_dir hiện không khả dụng."),
        ));
        return;
    }

    let missing_files = missing_registry_model_files(&resolved_dir);
    if !missing_files.is_empty() {
        issues.push(issue(
            "error",
            "INVALID_REGISTRY_MODEL_FILES",
            &format!("Mục {label} thiếu: {}.", missing_files.join(", ")),
        ));
    }

    if let Some(postprocess_mode) = model.postprocess_mode.as_deref() {
        if !postprocess_mode_compatible(language, postprocess_mode) {
            issues.push(issue(
                "error",
                "INCOMPATIBLE_POSTPROCESS_MODE",
                &format!(
                    "Mục {label} dùng postprocess_mode không tương thích với language={language}: {postprocess_mode}."
                ),
            ));
        }
    }
}

fn missing_registry_model_files(dir: &Path) -> Vec<String> {
    let mut missing = Vec::new();
    if !has_match(dir, "encoder") {
        missing.push("encoder*.onnx".to_string());
    }
    if !has_match(dir, "decoder") {
        missing.push("decoder*.onnx".to_string());
    }
    if !has_match(dir, "joiner") {
        missing.push("joiner*.onnx".to_string());
    }
    if !dir.join("tokens.txt").exists() && !dir.join("config.json").exists() {
        missing.push("tokens.txt hoặc config.json".to_string());
    }
    missing
}

fn postprocess_mode_compatible(language: &str, mode: &str) -> bool {
    match language {
        "vi" => matches!(mode, "clean" | "clean_lower" | "capu"),
        "en" => matches!(mode, "none" | "clean" | "clean_lower"),
        _ => false,
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
    let file_name = if cfg!(target_os = "windows") {
        "python.exe"
    } else {
        "python3"
    };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{BinaryMode, EngineProfileRecord, EngineStatus};
    use crate::sherpa_registry::default_runtime_json;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!("ai4pro-validation-{name}-{unique}"))
    }

    fn sherpa_profile(root: &Path) -> EngineProfileRecord {
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
            model_dir: Some(
                root.join("models/stt/gipformer-65M-rnnt")
                    .to_string_lossy()
                    .to_string(),
            ),
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

    fn seed_sherpa_model(dir: &Path) {
        fs::create_dir_all(dir).expect("model dir should exist");
        for file in ["encoder.onnx", "decoder.onnx", "joiner.onnx", "tokens.txt"] {
            fs::write(dir.join(file), b"test").expect("test file should write");
        }
    }

    #[test]
    fn sherpa_validation_reports_missing_models_config() {
        let root = temp_dir("missing-config");
        let model_dir = root.join("models/stt/gipformer-65M-rnnt");
        seed_sherpa_model(&model_dir);
        let profile = sherpa_profile(&root);

        let result = validate_profile(&profile, &root).expect("validation should succeed");
        assert!(result
            .issues
            .iter()
            .any(|issue| issue.code == "MISSING_MODELS_CONFIG"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn sherpa_validation_accepts_valid_registry() {
        let root = temp_dir("valid-registry");
        let vi_model_dir = root.join("models/stt/gipformer-65M-rnnt");
        let en_model_dir =
            root.join("models/stt/sherpa-onnx-zipformer-en-libriheavy-20230830-medium-punct-case");
        let config_dir = root.join("config/sherpa");
        seed_sherpa_model(&vi_model_dir);
        seed_sherpa_model(&en_model_dir);
        fs::create_dir_all(&config_dir).expect("config dir should exist");
        fs::write(
            config_dir.join("models.local.json"),
            format!(
                "{{\n  \"models\": [\n    {{\n      \"language\": \"vi\",\n      \"model_dir\": \"{}\",\n      \"postprocess_mode\": \"capu\",\n      \"vad_min_silence\": 0.5\n    }},\n    {{\n      \"language\": \"en\",\n      \"model_dir\": \"{}\",\n      \"postprocess_mode\": \"none\",\n      \"vad_min_silence\": 0.5\n    }}\n  ]\n}}\n",
                vi_model_dir.to_string_lossy(),
                en_model_dir.to_string_lossy()
            ),
        )
        .expect("config should write");
        let profile = sherpa_profile(&root);

        let result = validate_profile(&profile, &root).expect("validation should succeed");
        assert!(!result
            .issues
            .iter()
            .any(|issue| issue.code == "MISSING_MODELS_CONFIG"));
        assert!(!result
            .issues
            .iter()
            .any(|issue| issue.code == "INVALID_REGISTRY_MODEL_FILES"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn sherpa_validation_rejects_incompatible_postprocess_mode() {
        let root = temp_dir("invalid-postprocess");
        let model_dir = root.join("models/stt/gipformer-65M-rnnt");
        let config_dir = root.join("config/sherpa");
        seed_sherpa_model(&model_dir);
        fs::create_dir_all(&config_dir).expect("config dir should exist");
        fs::write(
            config_dir.join("models.local.json"),
            format!(
                "{{\n  \"models\": [\n    {{\n      \"language\": \"en\",\n      \"model_dir\": \"{}\",\n      \"postprocess_mode\": \"capu\"\n    }}\n  ]\n}}\n",
                model_dir.to_string_lossy()
            ),
        )
        .expect("config should write");
        let profile = sherpa_profile(&root);

        let result = validate_profile(&profile, &root).expect("validation should succeed");
        assert!(result
            .issues
            .iter()
            .any(|issue| issue.code == "INCOMPATIBLE_POSTPROCESS_MODE"));

        let _ = fs::remove_dir_all(root);
    }
}
