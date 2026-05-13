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
mod sherpa_registry;
mod state;
mod validation;

use state::AppContext;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Manager, RunEvent};

const MAIN_WINDOW_LABEL: &str = "main";
const TRAY_OPEN_ID: &str = "open_local_ai";
const TRAY_QUIT_ID: &str = "quit_local_ai";

fn main() {
    let full_exit_requested = Arc::new(AtomicBool::new(false));
    let setup_full_exit_requested = Arc::clone(&full_exit_requested);
    let close_full_exit_requested = Arc::clone(&full_exit_requested);
    let run_full_exit_requested = Arc::clone(&full_exit_requested);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let packaged_resource_dir = app
                .path()
                .resource_dir()
                .map_err(bundled_resources::resource_dir_error)?;
            let resource_dir = runtime_resource_dir(packaged_resource_dir);
            let context = AppContext::bootstrap(resource_dir.as_deref())?;
            app.manage(context);
            configure_tray(app, Arc::clone(&setup_full_exit_requested))?;
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
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if close_full_exit_requested.load(Ordering::SeqCst) {
                    return;
                }

                api.prevent_close();
                let _ = window.hide();
            }
        })
        .build(tauri::generate_context!())
        .expect("failed to build tauri application")
        .run(move |app_handle, event| {
            if let RunEvent::ExitRequested { .. } = event {
                shutdown_engines_for_full_exit(app_handle, &run_full_exit_requested);
            }
        });
}

fn configure_tray(app: &mut App, full_exit_requested: Arc<AtomicBool>) -> tauri::Result<()> {
    let open_item = MenuItem::with_id(app, TRAY_OPEN_ID, "Open Local AI", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, TRAY_QUIT_ID, "Quit Local AI", true, None::<&str>)?;
    let tray_menu = Menu::with_items(app, &[&open_item, &separator, &quit_item])?;

    let tray_icon = tauri::image::Image::from_bytes(include_bytes!("../icons/icon.png"))?;
    let quit_requested_from_menu = Arc::clone(&full_exit_requested);

    TrayIconBuilder::with_id("local-ai")
        .menu(&tray_menu)
        .icon(tray_icon)
        .tooltip("Local AI")
        .show_menu_on_left_click(false)
        .on_menu_event(move |app_handle, event| match event.id().as_ref() {
            TRAY_OPEN_ID => restore_main_window(app_handle),
            TRAY_QUIT_ID => request_full_exit(app_handle, &quit_requested_from_menu),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button,
                button_state,
                ..
            } = event
            {
                if button == MouseButton::Left && button_state == MouseButtonState::Up {
                    restore_main_window(tray.app_handle());
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn restore_main_window(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn request_full_exit(app_handle: &AppHandle, full_exit_requested: &AtomicBool) {
    shutdown_engines_for_full_exit(app_handle, full_exit_requested);
    app_handle.exit(0);
}

fn shutdown_engines_for_full_exit(app_handle: &AppHandle, full_exit_requested: &AtomicBool) {
    if full_exit_requested
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }

    let context = app_handle.state::<AppContext>();
    let _ = context.engine_manager.shutdown_all();
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
