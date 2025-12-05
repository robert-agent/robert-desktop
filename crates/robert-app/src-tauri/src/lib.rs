// Disable doctests for this crate
#![cfg_attr(doctest, allow(dead_code, unused_imports, unused_variables))]

// Modules
mod agent;
mod claude;
mod commands;
pub mod developer_mode;
mod events;
mod logging;
pub mod profiles;
mod setup;
mod state;

use robert_core::context::{Context, ContextManager};
use setup::RobertState;
use state::AppState;
use tauri::State;

#[tauri::command]
async fn get_contexts(state: State<'_, RobertState>) -> Result<Vec<Context>, String> {
    let personal = state
        .context_manager
        .get_context("personal")
        .map_err(|e| e.to_string())?;
    let work = state
        .context_manager
        .get_context("work")
        .map_err(|e| e.to_string())?;
    Ok(vec![personal, work])
}

#[tauri::command]
async fn create_context(state: State<'_, RobertState>, name: String) -> Result<(), String> {
    let id = name.to_lowercase().replace(" ", "-");
    let ctx = Context {
        id: id.clone(),
        name,
        description: "".to_string(),
        rules: vec![],
        included_paths: vec![],
    };
    state
        .context_manager
        .create_context(ctx)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn chat(state: State<'_, RobertState>, message: String) -> Result<String, String> {
    // Use SearchManager for reasoning
    let results = state
        .search_manager
        .search(&message, 3)
        .await
        .map_err(|e| e.to_string())?;

    if results.is_empty() {
        Ok("I couldn't find any relevant information in your context.".to_string())
    } else {
        // For Alpha, just return the top result's content preview
        let top_doc = &results[0];
        let preview = top_doc
            .properties
            .get("content_preview")
            .and_then(|v| v.as_str())
            .unwrap_or("No preview");
        Ok(format!(
            "Found relevant context: {}\n\n{}",
            top_doc.properties["title"], preview
        ))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize custom logger that writes to both console and encrypted log file
    logging::logger::init_logger();

    log::info!("🚀 Robert App starting...");

    // Check if Claude CLI is accessible
    check_claude_cli_availability();

    // Initialize Backend (Async)
    let robert_state = tauri::async_runtime::block_on(setup::init_backend())
        .expect("Failed to initialize backend");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .manage(AppState::new())
        .manage(robert_state)
        .invoke_handler(tauri::generate_handler![
            // Legacy commands (kept for reference/compatibility if needed)
            // commands::launch_browser,
            // commands::navigate_to_url,
            // commands::get_page_content,
            // commands::close_browser,
            // commands::take_screenshot,
            // commands::ask_claude,
            // commands::ask_claude_about_page,
            // commands::check_claude_health,
            // commands::run_diagnostics,
            // commands::validate_cdp_script,
            // commands::validate_cdp_script_file,
            // commands::execute_cdp_script,
            commands::get_system_paths,
            commands::start_dev_test_server,
            commands::stop_dev_test_server,
            commands::get_dev_test_server_status,
            commands::dev_capture_screenshot,
            commands::dev_list_screenshots,
            commands::dev_delete_all_screenshots,
            commands::dev_delete_screenshot,
            commands::process_chat_message,
            commands::init_agent_configs,
            commands::list_agent_configs,
            commands::get_agent_config,
            commands::update_agent_config,
            commands::submit_action_feedback,
            commands::create_user,
            commands::login_user,
            commands::logout_user,
            commands::get_current_user,
            commands::list_users,
            commands::get_user_profile,
            commands::update_user_profile,
            commands::has_users,
            // Browser commands removed
            // commands::browser::launch_browser_session,
            // commands::browser::close_browser_session,
            // commands::browser::get_browser_status,
            // commands::browser::close_all_browser_sessions,
            commands::save_command,
            commands::get_command,
            commands::list_commands,
            commands::delete_command,
            commands::build_command_prompt,
            commands::get_static_cdp,
            commands::log_frontend_message,
            commands::get_logs,
            commands::clear_logs,
            commands::get_log_size,
            // New ContextOS Commands
            get_contexts,
            create_context,
            chat,
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
