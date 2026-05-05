use crate::adapters::{adapter_for, resolve_binary_path};
use crate::app_paths::AppPaths;
use crate::db::Database;
use crate::errors::{AppError, AppResult};
use crate::health;
use crate::models::{
    DeveloperEngineProfileDto, DeveloperModelPackageDto, EngineKind, EngineProfileRecord,
    EngineRuntimeStateRecord, EngineStatus, ModelPackageRecord, ProcessSnapshotDto,
    SimpleLocalAiStatus, SimpleLocalAiStatusDto, UpdateEngineProfileInput, ValidationIssueDto,
    ValidationResultDto,
};
use crate::ports;
use crate::process_supervisor::ProcessSupervisor;
use crate::validation::{validate_profile, validate_update_input};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct EngineManager {
    db: Database,
    paths: AppPaths,
    supervisor: ProcessSupervisor,
}

impl EngineManager {
    pub fn new(db: Database, paths: AppPaths, supervisor: ProcessSupervisor) -> Self {
        Self { db, paths, supervisor }
    }

    pub fn bootstrap_defaults(&self) -> AppResult<()> {
        self.reconcile_runtime_state()?;
        Ok(())
    }

    pub fn shutdown_all(&self) -> AppResult<()> {
        self.stop_all()
    }

    pub fn list_engine_profiles(&self) -> AppResult<Vec<DeveloperEngineProfileDto>> {
        self.refresh_runtime_state()?;
        self.db
            .list_engine_profiles()?
            .into_iter()
            .map(|profile| self.to_developer_dto(profile))
            .collect()
    }

    pub fn get_engine_profile(&self, id: &str) -> AppResult<DeveloperEngineProfileDto> {
        self.refresh_runtime_state()?;
        let profile = self
            .db
            .load_engine_profile(id)?
            .ok_or_else(|| AppError::new("ENGINE_NOT_FOUND", "Engine profile not found."))?;
        self.to_developer_dto(profile)
    }

    pub fn update_engine_profile(
        &self,
        id: &str,
        input: UpdateEngineProfileInput,
    ) -> AppResult<DeveloperEngineProfileDto> {
        validate_update_input(&input)?;
        let mut profile = self
            .db
            .load_engine_profile(id)?
            .ok_or_else(|| AppError::new("ENGINE_NOT_FOUND", "Engine profile not found."))?;

        if let Some(name) = input.name {
            profile.name = name;
        }
        if let Some(enabled) = input.enabled {
            profile.enabled = enabled;
        }
        if let Some(binary_mode) = input.binary_mode {
            profile.binary_mode = binary_mode;
        }
        if let Some(binary_path) = input.binary_path {
            profile.binary_path = binary_path;
        }
        if let Some(model_package_id) = input.model_package_id {
            profile.model_package_id = model_package_id;
        }
        if let Some(model_path) = input.model_path {
            profile.model_path = model_path;
        }
        if let Some(model_dir) = input.model_dir {
            profile.model_dir = model_dir;
        }
        if let Some(tokens_path) = input.tokens_path {
            profile.tokens_path = tokens_path;
        }
        if let Some(host) = input.host {
            profile.host = host;
        }
        if let Some(port) = input.port {
            profile.port = port;
        }
        if let Some(health_url) = input.health_url {
            profile.health_url = health_url;
        }
        if let Some(runtime) = input.runtime {
            profile.runtime = runtime;
        }
        if let Some(extra_args) = input.extra_args {
            profile.extra_args = extra_args;
        }
        if let Some(auto_start) = input.auto_start {
            profile.auto_start = auto_start;
        }

        profile.updated_at = crate::models::now_timestamp();
        self.db.upsert_engine_profile(&profile)?;
        self.db.upsert_runtime_state(&self.runtime_state_for_profile(&profile))?;
        self.to_developer_dto(profile)
    }

    pub fn list_model_packages(&self) -> AppResult<Vec<DeveloperModelPackageDto>> {
        self.db
            .list_model_packages()?
            .into_iter()
            .map(|package| self.to_model_package_dto(package))
            .collect()
    }

    pub fn validate_engine_profile(&self, id: &str) -> AppResult<ValidationResultDto> {
        let profile = self
            .db
            .load_engine_profile(id)?
            .ok_or_else(|| AppError::new("ENGINE_NOT_FOUND", "Engine profile not found."))?;
        let mut validation = validate_profile(&profile, &self.paths.app_root)?;
        if !ports::port_is_available(profile.port) && profile.status == EngineStatus::Stopped {
            validation.issues.push(ValidationIssueDto {
                severity: "error".to_string(),
                code: "PORT_IN_USE".to_string(),
                message: format!("Port {} is already in use on 127.0.0.1.", profile.port),
            });
            validation.valid = false;
        }
        validation.generated_args = self.generated_args(&profile);
        Ok(validation)
    }

    pub fn validate_model_package(&self, id: &str) -> AppResult<ValidationResultDto> {
        let package = self
            .db
            .list_model_packages()?
            .into_iter()
            .find(|package| package.id == id)
            .ok_or_else(|| AppError::new("MODEL_PACKAGE_NOT_FOUND", "Model package not found."))?;

        let resolved = self.resolve_model_package_path(&package);
        let mut issues = Vec::new();

        if !resolved.exists() {
            issues.push(ValidationIssueDto {
                severity: "error".to_string(),
                code: "MISSING_MODEL".to_string(),
                message: format!("Required model path is unavailable: {}", resolved.to_string_lossy()),
            });
        }

        for required in &package.required_files {
            match required.as_str() {
                "model.gguf" => {
                    if !resolved.exists() {
                        issues.push(missing_model_issue(required));
                    }
                }
                "tokens.txt|config.json" => {
                    if !resolved.join("tokens.txt").exists() && !resolved.join("config.json").exists() {
                        issues.push(missing_model_issue(required));
                    }
                }
                pattern if pattern.ends_with("*.onnx") => {
                    let prefix = pattern.trim_end_matches("*.onnx");
                    let found = resolved
                        .read_dir()
                        .ok()
                        .into_iter()
                        .flat_map(|entries| entries.filter_map(Result::ok))
                        .any(|entry| {
                            entry.path().file_name().and_then(|name| name.to_str()).map(|name| {
                                name.starts_with(prefix) && name.ends_with(".onnx")
                            }).unwrap_or(false)
                        });
                    if !found {
                        issues.push(missing_model_issue(required));
                    }
                }
                required_file => {
                    if !resolved.join(required_file).exists() {
                        issues.push(missing_model_issue(required_file));
                    }
                }
            }
        }

        Ok(ValidationResultDto {
            engine_id: package.id,
            valid: issues.is_empty(),
            issues,
            generated_args: vec![],
        })
    }

    pub fn start_all(&self) -> AppResult<()> {
        let mut last_error = None;

        for profile in self.db.list_engine_profiles()? {
            if !profile.enabled {
                continue;
            }

            if let Err(error) = self.start_profile(&profile.id) {
                self.record_runtime_failure(&profile, &error)?;
                last_error = Some(error);
            }
        }

        if let Some(error) = last_error {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn stop_all(&self) -> AppResult<()> {
        for profile in self.db.list_engine_profiles()? {
            let _ = self.stop_profile(&profile.id)?;
        }
        Ok(())
    }

    pub fn restart_all(&self) -> AppResult<()> {
        self.stop_all()?;
        self.start_all()
    }

    pub fn restart_profile(&self, id: &str) -> AppResult<ProcessSnapshotDto> {
        let _ = self.stop_profile(id)?;
        self.start_profile(id)
    }

    pub fn start_profile(&self, id: &str) -> AppResult<ProcessSnapshotDto> {
        let profile = self
            .db
            .load_engine_profile(id)?
            .ok_or_else(|| AppError::new("ENGINE_NOT_FOUND", "Engine profile not found."))?;

        let validation = self.validate_engine_profile(&profile.id)?;
        if !validation.valid {
            return Err(AppError::new("INVALID_CONFIG", "Engine profile validation failed."));
        }
        if !ports::port_is_available(profile.port) {
            return Err(AppError::new("PORT_IN_USE", "Port is already in use."));
        }

        let mut updated = profile.clone();
        updated.status = EngineStatus::Starting;
        updated.updated_at = crate::models::now_timestamp();
        self.db.upsert_engine_profile(&updated)?;
        self.db.upsert_runtime_state(&EngineRuntimeStateRecord {
            engine_id: updated.id.clone(),
            status: EngineStatus::Starting,
            pid: None,
            health_url: updated.health_url.clone(),
            started_at: Some(crate::models::now_timestamp()),
            stopped_at: None,
            last_error: None,
            last_exit_code: None,
            updated_at: crate::models::now_timestamp(),
        })?;

        let adapter = adapter_for(&profile.kind);
        let spec = adapter.launch_spec(&self.paths, &profile)?;
        let snapshot = self.supervisor.spawn(spec)?;

        let healthy = self.wait_for_profile_health(&profile)?;
        updated.status = if healthy { EngineStatus::Running } else { EngineStatus::Unhealthy };
        updated.pid = snapshot.pid;
        updated.last_error = if healthy {
            None
        } else {
            Some("Health check did not pass within the startup window.".to_string())
        };
        updated.last_exit_code = None;
        updated.updated_at = crate::models::now_timestamp();
        self.db.upsert_engine_profile(&updated)?;
        self.db.upsert_runtime_state(&EngineRuntimeStateRecord {
            engine_id: updated.id.clone(),
            status: updated.status.clone(),
            pid: updated.pid,
            health_url: updated.health_url.clone(),
            started_at: Some(crate::models::now_timestamp()),
            stopped_at: None,
            last_error: updated.last_error.clone(),
            last_exit_code: None,
            updated_at: crate::models::now_timestamp(),
        })?;

        Ok(ProcessSnapshotDto {
            status: updated.status,
            ..snapshot
        })
    }

    pub fn stop_profile(&self, id: &str) -> AppResult<Option<ProcessSnapshotDto>> {
        let snapshot = self.supervisor.stop(id)?;
        if let Some(mut profile) = self.db.load_engine_profile(id)? {
            profile.status = EngineStatus::Stopped;
            profile.pid = None;
            profile.updated_at = crate::models::now_timestamp();
            self.db.upsert_engine_profile(&profile)?;
            self.db.upsert_runtime_state(&EngineRuntimeStateRecord {
                engine_id: profile.id.clone(),
                status: EngineStatus::Stopped,
                pid: None,
                health_url: profile.health_url.clone(),
                started_at: None,
                stopped_at: Some(crate::models::now_timestamp()),
                last_error: None,
                last_exit_code: snapshot.as_ref().and_then(|value| value.last_exit_code),
                updated_at: crate::models::now_timestamp(),
            })?;
        }
        Ok(snapshot)
    }

    pub fn simple_status(&self) -> AppResult<SimpleLocalAiStatusDto> {
        self.refresh_runtime_state()?;

        let profiles = self.db.list_engine_profiles()?;
        let enabled_profiles: Vec<_> = profiles.iter().filter(|profile| profile.enabled).collect();

        if enabled_profiles.is_empty() {
            return Ok(SimpleLocalAiStatusDto {
                status: SimpleLocalAiStatus::NeedsAttention,
                title: "Local AI is not ready".to_string(),
                message: "Local AI is not configured. Please contact support.".to_string(),
                can_start: false,
                can_stop: false,
                can_restart: true,
            });
        }

        let running = enabled_profiles
            .iter()
            .filter(|profile| profile.status == EngineStatus::Running)
            .count();
        let starting = enabled_profiles
            .iter()
            .any(|profile| profile.status == EngineStatus::Starting);
        let stopping = enabled_profiles
            .iter()
            .any(|profile| profile.status == EngineStatus::Stopping);
        let needs_attention = enabled_profiles.iter().any(|profile| {
            matches!(
                profile.status,
                EngineStatus::Unhealthy
                    | EngineStatus::Crashed
                    | EngineStatus::MissingBinary
                    | EngineStatus::MissingModel
                    | EngineStatus::InvalidConfig
                    | EngineStatus::PortConflict
            )
        });

        let dto = if running == enabled_profiles.len() {
            SimpleLocalAiStatusDto {
                status: SimpleLocalAiStatus::Ready,
                title: "Local AI is running".to_string(),
                message: "The local AI service is available.".to_string(),
                can_start: false,
                can_stop: true,
                can_restart: true,
            }
        } else if stopping {
            SimpleLocalAiStatusDto {
                status: SimpleLocalAiStatus::Stopping,
                title: "Local AI is stopping".to_string(),
                message: "Please wait...".to_string(),
                can_start: false,
                can_stop: false,
                can_restart: false,
            }
        } else if starting || running > 0 {
            SimpleLocalAiStatusDto {
                status: SimpleLocalAiStatus::Starting,
                title: "Local AI is starting".to_string(),
                message: "Please wait...".to_string(),
                can_start: false,
                can_stop: true,
                can_restart: false,
            }
        } else if needs_attention {
            SimpleLocalAiStatusDto {
                status: SimpleLocalAiStatus::NeedsAttention,
                title: "Local AI needs attention".to_string(),
                message: "Local AI could not start. Please contact support.".to_string(),
                can_start: false,
                can_stop: false,
                can_restart: true,
            }
        } else {
            SimpleLocalAiStatusDto {
                status: SimpleLocalAiStatus::NotRunning,
                title: "Local AI is ready".to_string(),
                message: "The local AI service is available when you need it.".to_string(),
                can_start: true,
                can_stop: false,
                can_restart: false,
            }
        };

        Ok(dto)
    }

    pub fn process_snapshots(&self) -> Vec<ProcessSnapshotDto> {
        self.supervisor.snapshots()
    }

    pub fn recent_log_paths(&self) -> Vec<String> {
        vec![
            self.paths.logs_dir.join("launcher.log").to_string_lossy().to_string(),
            self.paths.logs_dir.join("engines").to_string_lossy().to_string(),
        ]
    }

    pub fn runtime_state(&self) -> AppResult<Vec<EngineRuntimeStateRecord>> {
        self.db.list_runtime_state()
    }

    pub fn resolve_profile_binary(&self, profile: &EngineProfileRecord) -> PathBuf {
        let adapter = adapter_for(&profile.kind);
        adapter
            .launch_spec(&self.paths, profile)
            .map(|spec| spec.binary_path)
            .unwrap_or_else(|_| resolve_binary_path(&self.paths, profile, adapter.as_ref()))
    }

    fn to_developer_dto(&self, profile: EngineProfileRecord) -> AppResult<DeveloperEngineProfileDto> {
        let generated_args = self.generated_args(&profile);
        let resolved_binary_path = Some(self.resolve_profile_binary(&profile).to_string_lossy().to_string());

        Ok(DeveloperEngineProfileDto {
            id: profile.id,
            kind: profile.kind,
            name: profile.name,
            enabled: profile.enabled,
            binary_mode: profile.binary_mode,
            binary_name: profile.binary_name,
            binary_path: profile.binary_path,
            resolved_binary_path,
            model_package_id: profile.model_package_id,
            resolved_model_path: profile
                .model_path
                .as_ref()
                .map(|path| self.resolve_relative_or_absolute(path).to_string_lossy().to_string()),
            resolved_model_dir: profile
                .model_dir
                .as_ref()
                .map(|path| self.resolve_relative_or_absolute(path).to_string_lossy().to_string()),
            resolved_tokens_path: profile
                .tokens_path
                .as_ref()
                .map(|path| self.resolve_relative_or_absolute(path).to_string_lossy().to_string()),
            host: profile.host,
            port: profile.port,
            health_url: profile.health_url,
            runtime: profile.runtime,
            extra_args: profile.extra_args,
            generated_args,
            status: status_name(&profile.status).to_string(),
            pid: profile.pid,
            last_error: profile.last_error,
            last_exit_code: profile.last_exit_code,
            auto_start: profile.auto_start,
        })
    }

    fn to_model_package_dto(&self, package: ModelPackageRecord) -> AppResult<DeveloperModelPackageDto> {
        let resolved_path = self.resolve_model_package_path(&package).to_string_lossy().to_string();

        Ok(DeveloperModelPackageDto {
            id: package.id,
            kind: package.kind,
            display_name: package.display_name,
            internal_name: package.internal_name,
            relative_path: package.relative_path.clone(),
            resolved_path: Some(resolved_path),
            installed: package.installed,
            verified: package.verified,
            last_verified_at: package.last_verified_at,
            required_files: package.required_files,
            manifest: package.manifest_json,
        })
    }

    fn generated_args(&self, profile: &EngineProfileRecord) -> Vec<String> {
        match profile.kind {
            EngineKind::LlamaCpp => {
                let mut args = vec![
                    "-m".to_string(),
                    profile
                        .model_path
                        .as_ref()
                        .map(|path| self.resolve_relative_or_absolute(path).to_string_lossy().to_string())
                        .unwrap_or_default(),
                    "--host".to_string(),
                    profile.host.clone(),
                    "--port".to_string(),
                    profile.port.to_string(),
                ];

                if let Some(ctx_size) = profile.runtime.get("ctxSize").and_then(|value| value.as_i64()) {
                    args.extend(["-c".to_string(), ctx_size.to_string()]);
                }
                if let Some(gpu_layers) = profile.runtime.get("gpuLayers").and_then(|value| value.as_i64()) {
                    args.extend(["-ngl".to_string(), gpu_layers.to_string()]);
                }
                if let Some(threads) = profile.runtime.get("threads").and_then(|value| value.as_i64()) {
                    args.extend(["-t".to_string(), threads.to_string()]);
                }
                if let Some(parallel) = profile.runtime.get("parallel").and_then(|value| value.as_i64()) {
                    args.extend(["-np".to_string(), parallel.to_string()]);
                }
                if profile.runtime.get("metrics").and_then(|value| value.as_bool()).unwrap_or(false) {
                    args.push("--metrics".to_string());
                }
                if let Some(api_key) = profile.runtime.get("apiKey").and_then(|value| value.as_str()) {
                    if !api_key.is_empty() {
                        args.extend(["--api-key".to_string(), api_key.to_string()]);
                    }
                }

                args.extend(profile.extra_args.clone());
                args
            }
            EngineKind::SherpaOnnx => {
                let model_dir = profile
                    .model_dir
                    .as_ref()
                    .or(profile.model_path.as_ref())
                    .map(|path| self.resolve_relative_or_absolute(path).to_string_lossy().to_string())
                    .unwrap_or_default();
                let alias = profile
                    .runtime
                    .get("alias")
                    .and_then(|value| value.as_str())
                    .unwrap_or(&profile.id)
                    .to_string();
                let provider = profile.runtime.get("provider").and_then(|value| value.as_str()).unwrap_or("cpu");
                let family = profile.runtime.get("sttModelFamily").and_then(|value| value.as_str()).unwrap_or("offline_int8");
                let postprocess = profile.runtime.get("postprocessMode").and_then(|value| value.as_str()).unwrap_or("clean");
                let template = profile
                    .runtime
                    .get("argsTemplate")
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();

                let mut args: Vec<String> = template
                    .into_iter()
                    .filter_map(|value| value.as_str().map(str::to_string))
                    .map(|value| {
                        value
                            .replace("{host}", &profile.host)
                            .replace("{port}", &profile.port.to_string())
                            .replace("{provider}", provider)
                            .replace("{sttModelFamily}", family)
                            .replace("{modelDir}", &model_dir)
                            .replace("{postprocessMode}", postprocess)
                            .replace("{alias}", &alias)
                    })
                    .collect();

                args.extend(profile.extra_args.clone());
                args
            }
        }
    }

    fn resolve_model_package_path(&self, package: &ModelPackageRecord) -> PathBuf {
        self.resolve_relative_or_absolute(&package.relative_path)
    }

    fn resolve_relative_or_absolute(&self, value: &str) -> PathBuf {
        resolve_relative_or_absolute(&self.paths.app_root, value)
    }

    fn record_runtime_failure(&self, profile: &EngineProfileRecord, error: &AppError) -> AppResult<()> {
        let mut failed = profile.clone();
        failed.status = match error.code.as_str() {
            "MISSING_BINARY" => EngineStatus::MissingBinary,
            "MISSING_MODEL" | "INVALID_MODEL_DIR" | "INVALID_MODEL_PATH" | "INVALID_TOKENS_PATH" => {
                EngineStatus::MissingModel
            }
            "PORT_IN_USE" => EngineStatus::PortConflict,
            _ => EngineStatus::InvalidConfig,
        };
        failed.last_error = Some(error.message.clone());
        failed.pid = None;
        failed.updated_at = crate::models::now_timestamp();
        self.db.upsert_engine_profile(&failed)?;
        self.db.upsert_runtime_state(&EngineRuntimeStateRecord {
            engine_id: failed.id.clone(),
            status: failed.status.clone(),
            pid: None,
            health_url: failed.health_url.clone(),
            started_at: None,
            stopped_at: Some(crate::models::now_timestamp()),
            last_error: failed.last_error.clone(),
            last_exit_code: failed.last_exit_code,
            updated_at: crate::models::now_timestamp(),
        })
    }

    fn wait_for_profile_health(&self, profile: &EngineProfileRecord) -> AppResult<bool> {
        match profile.kind {
            EngineKind::LlamaCpp => health::wait_for_http_any(&profile.host, profile.port, &["/health", "/v1/models", "/props", "/"], 10_000),
            EngineKind::SherpaOnnx => health::wait_for_http_any(&profile.host, profile.port, &["/health", "/v1/models"], 12_000),
        }
    }

    fn refresh_runtime_state(&self) -> AppResult<()> {
        self.supervisor.refresh();
        for snapshot in self.supervisor.snapshots() {
            if let Some(mut profile) = self.db.load_engine_profile(&snapshot.id)? {
                profile.status = snapshot.status.clone();
                profile.pid = snapshot.pid;
                profile.last_error = snapshot.last_error.clone();
                profile.last_exit_code = snapshot.last_exit_code;
                profile.updated_at = crate::models::now_timestamp();
                if profile.status == EngineStatus::Running {
                    let healthy = self.wait_for_profile_health(&profile)?;
                    if !healthy {
                        profile.status = EngineStatus::Unhealthy;
                        profile.last_error = Some("Engine process is alive but HTTP health did not respond.".to_string());
                    }
                }
                self.db.upsert_engine_profile(&profile)?;
                self.db.upsert_runtime_state(&self.runtime_state_for_profile(&profile))?;
            }
        }
        Ok(())
    }

    fn reconcile_runtime_state(&self) -> AppResult<()> {
        for mut state in self.db.list_runtime_state()? {
            state.status = EngineStatus::Stopped;
            state.pid = None;
            state.last_error = None;
            state.stopped_at = Some(crate::models::now_timestamp());
            state.updated_at = crate::models::now_timestamp();
            self.db.upsert_runtime_state(&state)?;
        }

        for mut profile in self.db.list_engine_profiles()? {
            if !matches!(profile.status, EngineStatus::MissingBinary | EngineStatus::MissingModel | EngineStatus::InvalidConfig | EngineStatus::PortConflict) {
                profile.status = EngineStatus::Stopped;
            }
            profile.pid = None;
            profile.updated_at = crate::models::now_timestamp();
            self.db.upsert_engine_profile(&profile)?;
            self.db.upsert_runtime_state(&self.runtime_state_for_profile(&profile))?;
        }

        Ok(())
    }

    fn runtime_state_for_profile(&self, profile: &EngineProfileRecord) -> EngineRuntimeStateRecord {
        EngineRuntimeStateRecord {
            engine_id: profile.id.clone(),
            status: profile.status.clone(),
            pid: profile.pid,
            health_url: profile.health_url.clone(),
            started_at: if matches!(profile.status, EngineStatus::Running | EngineStatus::Starting) {
                Some(crate::models::now_timestamp())
            } else {
                None
            },
            stopped_at: if matches!(profile.status, EngineStatus::Stopped | EngineStatus::Unhealthy | EngineStatus::Crashed | EngineStatus::MissingBinary | EngineStatus::MissingModel | EngineStatus::InvalidConfig | EngineStatus::PortConflict) {
                Some(crate::models::now_timestamp())
            } else {
                None
            },
            last_error: profile.last_error.clone(),
            last_exit_code: profile.last_exit_code,
            updated_at: crate::models::now_timestamp(),
        }
    }
}

fn resolve_relative_or_absolute(app_root: &Path, value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        app_root.join(path)
    }
}

fn status_name(status: &EngineStatus) -> &'static str {
    match status {
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

fn missing_model_issue(required: &str) -> ValidationIssueDto {
    ValidationIssueDto {
        severity: "error".to_string(),
        code: "MISSING_MODEL".to_string(),
        message: format!("Required model file is unavailable: {required}"),
    }
}
