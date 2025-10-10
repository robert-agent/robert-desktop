// Modules
mod claude;
mod commands;
pub mod developer_mode;
mod events;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::launch_browser,
            commands::navigate_to_url,
            commands::get_page_content,
            commands::close_browser,
            commands::take_screenshot,
            commands::ask_claude,
            commands::ask_claude_about_page,
            commands::check_claude_health,
            commands::run_diagnostics,
            commands::validate_cdp_script,
            commands::validate_cdp_script_file,
            // Developer mode commands
            commands::get_system_paths,
            commands::start_dev_test_server,
            commands::stop_dev_test_server,
            commands::get_dev_test_server_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
