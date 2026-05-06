mod adapters;
mod app_paths;
mod app_settings;
mod bundled_resources;
mod commands;
mod db;
mod developer;
mod engine_manager;
mod errors;
mod health;
mod local_ai;
mod logs;
mod migrations;
mod models;
mod ports;
mod process_registry;
mod process_supervisor;
mod state;
mod validation;

use state::AppContext;
use std::path::PathBuf;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let packaged_resource_dir = app
                .path()
                .resource_dir()
                .map_err(bundled_resources::resource_dir_error)?;
            let resource_dir = runtime_resource_dir(packaged_resource_dir);
            let context = AppContext::bootstrap(resource_dir.as_deref())?;
            app.manage(context);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_simple_local_ai_status,
            commands::check_simple_model_status,
            commands::start_local_ai,
            commands::stop_local_ai,
            commands::restart_local_ai,
            commands::get_app_settings,
            commands::update_app_settings,
            commands::enable_developer_mode_for_session,
            commands::disable_developer_mode_for_session,
            commands::dev_list_engine_profiles,
            commands::dev_get_engine_profile,
            commands::dev_update_engine_profile,
            commands::dev_validate_engine_profile,
            commands::dev_start_engine_profile,
            commands::dev_stop_engine_profile,
            commands::dev_restart_engine_profile,
            commands::dev_list_model_packages,
            commands::dev_validate_model_package,
            commands::dev_read_engine_log,
            commands::dev_get_diagnostics_bundle,
            commands::dev_open_logs_folder,
        ])
        .on_window_event(move |window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let context = window.state::<AppContext>();
                if let Ok(settings) = context.settings.load() {
                    if settings.stop_engines_on_exit {
                        let _ = context.engine_manager.shutdown_all();
                    }
                }
                let _ = window;
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run tauri application");
}

fn has_runtime_resources(path: &PathBuf) -> bool {
    path.join("binaries").exists() || path.join("runtime").exists()
}

fn runtime_resource_dir(packaged_resource_dir: PathBuf) -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    if let Some(dev_resource_dir) = bundled_resources::resource_dir_for_dev() {
        return Some(dev_resource_dir);
    }

    has_runtime_resources(&packaged_resource_dir).then_some(packaged_resource_dir)
}
