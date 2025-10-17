// Modules
mod agent;
mod claude;
mod commands;
pub mod developer_mode;
mod events;
mod profiles;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger with debug level for robert crates
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format_timestamp_millis()
        .init();

    log::info!("🚀 Robert App starting...");

    // Check if Claude CLI is accessible
    check_claude_cli_availability();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
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
            commands::execute_cdp_script,
            // Developer mode commands
            commands::get_system_paths,
            commands::start_dev_test_server,
            commands::stop_dev_test_server,
            commands::get_dev_test_server_status,
            // Developer mode screenshot commands
            commands::dev_capture_screenshot,
            commands::dev_list_screenshots,
            commands::dev_delete_all_screenshots,
            commands::dev_delete_screenshot,
            // Agent workflow commands
            commands::process_chat_message,
            commands::init_agent_configs,
            commands::list_agent_configs,
            commands::get_agent_config,
            commands::update_agent_config,
            commands::submit_action_feedback,
            // Profile management commands
            commands::create_user,
            commands::login_user,
            commands::logout_user,
            commands::get_current_user,
            commands::list_users,
            commands::get_user_profile,
            commands::update_user_profile,
            commands::has_users,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Check if Claude CLI is accessible for process spawning
fn check_claude_cli_availability() {
    use std::process::Command;

    log::info!("🔍 Checking Claude CLI availability...");

    match Command::new("claude").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                log::info!("✓ Claude CLI found: {}", version.trim());
            } else {
                log::warn!(
                    "⚠️  Claude CLI command failed with status: {}",
                    output.status
                );
                log::warn!("   Make sure Claude is installed and in your PATH");
                log::warn!("   Visit: https://www.anthropic.com/claude-cli");
            }
        }
        Err(e) => {
            log::warn!("⚠️  Claude CLI not found or not accessible: {}", e);
            log::warn!("   Agent workflows will not be available without Claude CLI");
            log::warn!("   Install from: https://www.anthropic.com/claude-cli");
        }
    }
}
