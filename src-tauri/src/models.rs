use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EngineKind {
    LlamaCpp,
    SherpaOnnx,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BinaryMode {
    Bundled,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EngineStatus {
    Stopped,
    Starting,
    Running,
    Unhealthy,
    Stopping,
    Crashed,
    MissingBinary,
    MissingModel,
    InvalidConfig,
    PortConflict,
}

pub use EngineStatus as ProcessStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SimpleLocalAiStatus {
    NotRunning,
    Starting,
    Ready,
    Stopping,
    NeedsAttention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPathsDto {
    pub app_root: String,
    pub config_dir: String,
    pub data_dir: String,
    pub logs_dir: String,
    pub models_dir: String,
    pub binaries_dir: String,
    pub database_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsRecord {
    pub app_data_root: String,
    pub developer_mode_persisted: bool,
    pub stop_engines_on_exit: bool,
    pub auto_start_local_ai: bool,
    pub simple_mode_only: bool,
    pub machine_configured: bool,
    pub setup_version: String,
}

impl Default for AppSettingsRecord {
    fn default() -> Self {
        Self {
            app_data_root: String::new(),
            developer_mode_persisted: false,
            stop_engines_on_exit: true,
            auto_start_local_ai: false,
            simple_mode_only: false,
            machine_configured: false,
            setup_version: "0.1.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppSettingsInput {
    pub app_data_root: Option<String>,
    pub developer_mode_persisted: Option<bool>,
    pub stop_engines_on_exit: Option<bool>,
    pub auto_start_local_ai: Option<bool>,
    pub simple_mode_only: Option<bool>,
    pub machine_configured: Option<bool>,
    pub setup_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPackageRecord {
    pub id: String,
    pub kind: EngineKind,
    pub display_name: String,
    pub internal_name: String,
    pub relative_path: String,
    pub required_files: Vec<String>,
    pub manifest_json: serde_json::Value,
    pub installed: bool,
    pub verified: bool,
    pub last_verified_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineProfileRecord {
    pub id: String,
    pub kind: EngineKind,
    pub name: String,
    pub enabled: bool,
    pub binary_mode: BinaryMode,
    pub binary_name: String,
    pub binary_path: Option<String>,
    pub model_package_id: Option<String>,
    pub model_path: Option<String>,
    pub model_dir: Option<String>,
    pub tokens_path: Option<String>,
    pub host: String,
    pub port: u16,
    pub health_url: Option<String>,
    pub runtime: serde_json::Value,
    pub extra_args: Vec<String>,
    pub auto_start: bool,
    pub status: EngineStatus,
    pub pid: Option<u32>,
    pub last_error: Option<String>,
    pub last_exit_code: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineRuntimeStateRecord {
    pub engine_id: String,
    pub status: EngineStatus,
    pub pid: Option<u32>,
    pub health_url: Option<String>,
    pub started_at: Option<String>,
    pub stopped_at: Option<String>,
    pub last_error: Option<String>,
    pub last_exit_code: Option<i32>,
    pub updated_at: String,
}

impl EngineRuntimeStateRecord {
    pub fn stopped(engine_id: impl Into<String>) -> Self {
        Self {
            engine_id: engine_id.into(),
            status: EngineStatus::Stopped,
            pid: None,
            health_url: None,
            started_at: None,
            stopped_at: Some(now_timestamp()),
            last_error: None,
            last_exit_code: None,
            updated_at: now_timestamp(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogFileRecord {
    pub id: String,
    pub engine_id: Option<String>,
    pub kind: String,
    pub path: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleLocalAiStatusDto {
    pub status: SimpleLocalAiStatus,
    pub title: String,
    pub message: String,
    pub can_start: bool,
    pub can_stop: bool,
    pub can_restart: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperEngineProfileDto {
    pub id: String,
    pub kind: EngineKind,
    pub name: String,
    pub enabled: bool,
    pub binary_mode: BinaryMode,
    pub binary_name: String,
    pub binary_path: Option<String>,
    pub resolved_binary_path: Option<String>,
    pub model_package_id: Option<String>,
    pub resolved_model_path: Option<String>,
    pub resolved_model_dir: Option<String>,
    pub resolved_tokens_path: Option<String>,
    pub host: String,
    pub port: u16,
    pub health_url: Option<String>,
    pub runtime: serde_json::Value,
    pub extra_args: Vec<String>,
    pub generated_args: Vec<String>,
    pub status: String,
    pub pid: Option<u32>,
    pub last_error: Option<String>,
    pub last_exit_code: Option<i32>,
    pub auto_start: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEngineProfileInput {
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub binary_mode: Option<BinaryMode>,
    pub binary_path: Option<Option<String>>,
    pub model_package_id: Option<Option<String>>,
    pub model_path: Option<Option<String>>,
    pub model_dir: Option<Option<String>>,
    pub tokens_path: Option<Option<String>>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub health_url: Option<Option<String>>,
    pub runtime: Option<serde_json::Value>,
    pub extra_args: Option<Vec<String>>,
    pub auto_start: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssueDto {
    pub severity: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResultDto {
    pub engine_id: String,
    pub valid: bool,
    pub issues: Vec<ValidationIssueDto>,
    pub generated_args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperModelPackageDto {
    pub id: String,
    pub kind: EngineKind,
    pub display_name: String,
    pub internal_name: String,
    pub relative_path: String,
    pub resolved_path: Option<String>,
    pub installed: bool,
    pub verified: bool,
    pub last_verified_at: Option<String>,
    pub required_files: Vec<String>,
    pub manifest: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsCrashDto {
    pub engine_id: String,
    pub last_error: Option<String>,
    pub last_exit_code: Option<i32>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsLogSnippetDto {
    pub name: String,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsBundleDto {
    pub app_version: String,
    pub os: String,
    pub arch: String,
    pub app_data_root: String,
    pub sqlite_path: String,
    pub logs_root: String,
    pub machine_configured: bool,
    pub app_settings: AppSettingsRecord,
    pub app_paths: AppPathsDto,
    pub engine_profiles: Vec<DeveloperEngineProfileDto>,
    pub model_packages: Vec<DeveloperModelPackageDto>,
    pub validation: Vec<ValidationResultDto>,
    pub recent_crashes: Vec<DiagnosticsCrashDto>,
    pub runtime_state: Vec<EngineRuntimeStateRecord>,
    pub recent_logs: Vec<DiagnosticsLogSnippetDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessSnapshotDto {
    pub id: String,
    pub status: EngineStatus,
    pub pid: Option<u32>,
    pub last_error: Option<String>,
    pub last_exit_code: Option<i32>,
}

pub fn now_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
