mod adapters;
mod app_paths;
mod app_settings;
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

fn main() {
    let context = match AppContext::bootstrap() {
        Ok(context) => context,
        Err(error) => {
            eprintln!("failed to bootstrap app context: {error}");
            std::process::exit(1);
        }
    };

    let shutdown_context = context.clone();

    tauri::Builder::default()
        .manage(context)
        .invoke_handler(tauri::generate_handler![
            commands::get_simple_local_ai_status,
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
                if let Ok(settings) = shutdown_context.settings.load() {
                    if settings.stop_engines_on_exit {
                        let _ = shutdown_context.engine_manager.shutdown_all();
                    }
                }
                let _ = window;
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run tauri application");
}
