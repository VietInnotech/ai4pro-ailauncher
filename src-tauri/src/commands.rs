use crate::errors::{AppError, AppResult};
use crate::models::{AppSettingsRecord, DeveloperEngineProfileDto, DeveloperModelPackageDto, DiagnosticsBundleDto, SimpleLocalAiStatusDto, UpdateAppSettingsInput, UpdateEngineProfileInput, ValidationResultDto};
use crate::state::AppContext;
use tauri::State;

fn simple_error(error: AppError) -> String {
    let safe = error.to_safe();
    serde_json::to_string(&safe).unwrap_or_else(|_| safe.message)
}

fn dev_error(error: AppError) -> String {
    serde_json::to_string(&error).unwrap_or_else(|_| error.message)
}

fn require_dev(context: &AppContext) -> AppResult<()> {
    context.developer_mode.require_enabled()
}

#[tauri::command]
pub fn get_simple_local_ai_status(state: State<'_, AppContext>) -> Result<SimpleLocalAiStatusDto, String> {
    state.local_ai.status().map_err(simple_error)
}

#[tauri::command]
pub fn check_simple_model_status(state: State<'_, AppContext>, id: String) -> Result<SimpleLocalAiStatusDto, String> {
    state.local_ai.check_model(&id).map_err(simple_error)
}

#[tauri::command]
pub fn start_local_ai(state: State<'_, AppContext>) -> Result<SimpleLocalAiStatusDto, String> {
    state.local_ai.start().map_err(simple_error)
}

#[tauri::command]
pub fn stop_local_ai(state: State<'_, AppContext>) -> Result<SimpleLocalAiStatusDto, String> {
    state.local_ai.stop().map_err(simple_error)
}

#[tauri::command]
pub fn restart_local_ai(state: State<'_, AppContext>) -> Result<SimpleLocalAiStatusDto, String> {
    state.local_ai.restart().map_err(simple_error)
}

#[tauri::command]
pub fn enable_developer_mode_for_session(state: State<'_, AppContext>) -> Result<(), String> {
    state.developer_mode.enable_session();
    Ok(())
}

#[tauri::command]
pub fn disable_developer_mode_for_session(state: State<'_, AppContext>) -> Result<(), String> {
    state.developer_mode.disable_session();
    Ok(())
}

#[tauri::command]
pub fn get_app_settings(state: State<'_, AppContext>) -> Result<AppSettingsRecord, String> {
    require_dev(&state).map_err(dev_error)?;
    state.settings.load().map_err(dev_error)
}

#[tauri::command]
pub fn update_app_settings(state: State<'_, AppContext>, input: UpdateAppSettingsInput) -> Result<AppSettingsRecord, String> {
    require_dev(&state).map_err(dev_error)?;
    let mut settings = state.settings.load().map_err(dev_error)?;
    if let Some(value) = input.app_data_root { settings.app_data_root = value; }
    if let Some(value) = input.developer_mode_persisted { settings.developer_mode_persisted = value; }
    if let Some(value) = input.stop_engines_on_exit { settings.stop_engines_on_exit = value; }
    if let Some(value) = input.auto_start_local_ai { settings.auto_start_local_ai = value; }
    if let Some(value) = input.simple_mode_only { settings.simple_mode_only = value; }
    if let Some(value) = input.machine_configured { settings.machine_configured = value; }
    if let Some(value) = input.setup_version { settings.setup_version = value; }
    state.settings.save(&settings).map_err(dev_error)?;
    Ok(settings)
}

#[tauri::command]
pub fn dev_list_engine_profiles(state: State<'_, AppContext>) -> Result<Vec<DeveloperEngineProfileDto>, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.list_engine_profiles().map_err(dev_error)
}

#[tauri::command]
pub fn dev_list_model_packages(state: State<'_, AppContext>) -> Result<Vec<DeveloperModelPackageDto>, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.list_model_packages().map_err(dev_error)
}

#[tauri::command]
pub fn dev_get_engine_profile(state: State<'_, AppContext>, id: String) -> Result<DeveloperEngineProfileDto, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.get_engine_profile(&id).map_err(dev_error)
}

#[tauri::command]
pub fn dev_update_engine_profile(
    state: State<'_, AppContext>,
    id: String,
    input: UpdateEngineProfileInput,
) -> Result<DeveloperEngineProfileDto, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.update_engine_profile(&id, input).map_err(dev_error)
}

#[tauri::command]
pub fn dev_validate_engine_profile(state: State<'_, AppContext>, id: String) -> Result<ValidationResultDto, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.validate_engine_profile(&id).map_err(dev_error)
}

#[tauri::command]
pub fn dev_start_engine_profile(state: State<'_, AppContext>, id: String) -> Result<DeveloperEngineProfileDto, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.start_profile(&id).map_err(dev_error)?;
    state.engine_manager.get_engine_profile(&id).map_err(dev_error)
}

#[tauri::command]
pub fn dev_stop_engine_profile(state: State<'_, AppContext>, id: String) -> Result<DeveloperEngineProfileDto, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.stop_profile(&id).map_err(dev_error)?;
    state.engine_manager.get_engine_profile(&id).map_err(dev_error)
}

#[tauri::command]
pub fn dev_restart_engine_profile(state: State<'_, AppContext>, id: String) -> Result<DeveloperEngineProfileDto, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.restart_profile(&id).map_err(dev_error)?;
    state.engine_manager.get_engine_profile(&id).map_err(dev_error)
}

#[tauri::command]
pub fn dev_validate_model_package(state: State<'_, AppContext>, id: String) -> Result<ValidationResultDto, String> {
    require_dev(&state).map_err(dev_error)?;
    state.engine_manager.validate_model_package(&id).map_err(dev_error)
}

#[tauri::command]
pub fn dev_read_engine_log(
    state: State<'_, AppContext>,
    id: String,
    log_type: String,
    tail_lines: Option<u32>,
) -> Result<String, String> {
    require_dev(&state).map_err(dev_error)?;
    let lines = tail_lines.unwrap_or(200) as usize;
    let path = match log_type.as_str() {
        "stderr" => state.paths.logs_dir.join("engines").join(id).join("stderr.log"),
        _ => state.paths.logs_dir.join("engines").join(id).join("stdout.log"),
    };
    crate::logs::tail_file(&path, lines).map_err(dev_error)
}

#[tauri::command]
pub fn dev_get_diagnostics_bundle(state: State<'_, AppContext>) -> Result<DiagnosticsBundleDto, String> {
    require_dev(&state).map_err(dev_error)?;
    crate::developer::diagnostics::build_bundle(state.inner()).map_err(dev_error)
}

#[tauri::command]
pub fn dev_open_logs_folder(state: State<'_, AppContext>, _id: String) -> Result<(), String> {
    require_dev(&state).map_err(dev_error)?;
    let path = if _id.is_empty() {
        state.paths.logs_dir.clone()
    } else {
        state.paths.logs_dir.join("engines").join(_id)
    };
    crate::logs::open_in_file_manager(&path).map_err(dev_error)
}
