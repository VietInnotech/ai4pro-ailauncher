use crate::errors::AppResult;
use crate::models::{DiagnosticsBundleDto, DiagnosticsLogSnippetDto};
use crate::state::AppContext;

pub fn build_bundle(context: &AppContext) -> AppResult<DiagnosticsBundleDto> {
    let settings = context.settings.load()?;
    let paths = &context.paths;
    let runtime_state = context.engine_manager.runtime_state()?;
    let recent_logs = collect_recent_logs(paths.logs_dir.as_path())?;

    Ok(DiagnosticsBundleDto {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        app_data_root: paths.app_root.to_string_lossy().to_string(),
        sqlite_path: paths.database_path.to_string_lossy().to_string(),
        logs_root: paths.logs_dir.to_string_lossy().to_string(),
        machine_configured: settings.machine_configured,
        app_settings: settings,
        app_paths: crate::models::AppPathsDto {
            app_root: paths.app_root.to_string_lossy().to_string(),
            config_dir: paths.config_dir.to_string_lossy().to_string(),
            data_dir: paths.data_dir.to_string_lossy().to_string(),
            logs_dir: paths.logs_dir.to_string_lossy().to_string(),
            models_dir: paths.models_dir.to_string_lossy().to_string(),
            binaries_dir: paths.binaries_dir.to_string_lossy().to_string(),
            database_path: paths.database_path.to_string_lossy().to_string(),
        },
        engine_profiles: context.engine_manager.list_engine_profiles()?,
        model_packages: context.engine_manager.list_model_packages()?,
        validation: context
            .db
            .list_engine_profiles()?
            .into_iter()
            .map(|profile| context.engine_manager.validate_engine_profile(&profile.id))
            .collect::<AppResult<Vec<_>>>()?,
        recent_crashes: context
            .db
            .list_engine_profiles()?
            .into_iter()
            .filter_map(|profile| {
                profile
                    .last_error
                    .as_ref()
                    .map(|last_error| crate::models::DiagnosticsCrashDto {
                        engine_id: profile.id.clone(),
                        last_error: Some(last_error.clone()),
                        last_exit_code: profile.last_exit_code,
                        updated_at: profile.updated_at.clone(),
                    })
            })
            .collect(),
        runtime_state,
        recent_logs,
    })
}

fn collect_recent_logs(log_root: &std::path::Path) -> AppResult<Vec<DiagnosticsLogSnippetDto>> {
    let mut snippets = Vec::new();

    for path in [
        log_root.join("launcher.log"),
        log_root
            .join("engines")
            .join("language_engine")
            .join("stdout.log"),
        log_root
            .join("engines")
            .join("language_engine")
            .join("stderr.log"),
        log_root
            .join("engines")
            .join("speech_engine")
            .join("stdout.log"),
        log_root
            .join("engines")
            .join("speech_engine")
            .join("stderr.log"),
    ] {
        let content = crate::logs::tail_file(&path, 80)?;
        if content.is_empty() {
            continue;
        }
        snippets.push(DiagnosticsLogSnippetDto {
            name: path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("log")
                .to_string(),
            path: path.to_string_lossy().to_string(),
            content,
        });
    }

    Ok(snippets)
}
