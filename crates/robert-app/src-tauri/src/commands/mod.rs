mod agent;
mod developer_mode;

pub use agent::*;
pub use developer_mode::*;

use crate::claude::{ClaudeClient, ClaudeConfig, ClaudeHealthCheck, ClaudeInput, ClaudeResponse};
use crate::events::*;
use crate::state::AppState;
use robert_webdriver::{CdpValidator, ChromeDriver, ChatUI, ValidationResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationResult {
    pub success: bool,
    pub url: String,
    pub title: String,
    pub message: String,
}

/// Launch the browser (sandboxed mode with auto-download)
#[tauri::command]
pub async fn launch_browser(app: AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    emit_info(&app, "Initializing browser...").ok();

    // Check if Chrome needs to be downloaded
    emit_chrome_downloading(&app, "Checking for Chrome...").ok();

    let mut driver_lock = state.driver.lock().await;

    // If already launched, return success
    if driver_lock.is_some() {
        emit_info(&app, "Browser already running").ok();
        return Ok("Browser already running".to_string());
    }

    emit_chrome_launching(&app, "Launching browser...").ok();

    // Launch with auto-detection (headless in CI, visible otherwise)
    match ChromeDriver::launch_auto().await {
        Ok(driver) => {
            emit_chrome_launched(&app, "Browser launched successfully!").ok();
            emit_success(&app, "Chrome is ready for automation").ok();

            *driver_lock = Some(driver);
            drop(driver_lock);

            // Start chat message polling task
            let state_clone = state.inner().driver.clone();
            let shutdown_clone = state.inner().chat_poll_shutdown.clone();
            start_chat_polling(app.clone(), state_clone, shutdown_clone).await;

            Ok("Browser launched successfully".to_string())
        }
        Err(e) => {
            let error_msg = format!("Failed to launch browser: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    }
}

/// Start a background task to poll for chat messages
async fn start_chat_polling(
    app: AppHandle,
    driver: Arc<tokio::sync::Mutex<Option<ChromeDriver>>>,
    shutdown_holder: Arc<tokio::sync::Mutex<Option<tokio::sync::oneshot::Sender<()>>>>,
) {
    log::info!("Starting chat message polling task");

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();

    // Store shutdown channel
    *shutdown_holder.lock().await = Some(shutdown_tx);

    // Spawn polling task
    tokio::spawn(async move {
        let chat_ui = ChatUI::new();
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(500));

        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    log::info!("Chat polling task shutting down");
                    break;
                }
                _ = interval.tick() => {
                    // Check for unprocessed messages
                    let driver_lock = driver.lock().await;
                    if let Some(drv) = driver_lock.as_ref() {
                        if let Ok(page) = drv.current_page().await {
                            match chat_ui.get_unprocessed_messages(&page).await {
                                Ok(messages) if !messages.is_empty() => {
                                    log::info!("📬 Found {} unprocessed chat message(s)", messages.len());

                                    for msg in &messages {
                                        log::info!("💬 User message: {}", msg.text);
                                        emit_info(&app, format!("Chat message: {}", msg.text)).ok();

                                        // Process message through CDP generation workflow
                                        match process_chat_message_internal(
                                            &app,
                                            &driver,
                                            msg.text.clone(),
                                        ).await {
                                            Ok(result) => {
                                                let response = if result.success {
                                                    result.message
                                                } else {
                                                    format!("Error: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()))
                                                };
                                                if let Err(e) = chat_ui.send_agent_message(&page, &response).await {
                                                    log::error!("Failed to send agent response: {}", e);
                                                }
                                            }
                                            Err(e) => {
                                                let error_msg = format!("Workflow failed: {}", e);
                                                log::error!("{}", error_msg);
                                                if let Err(e) = chat_ui.send_agent_message(&page, &error_msg).await {
                                                    log::error!("Failed to send error message: {}", e);
                                                }
                                            }
                                        }
                                    }

                                    // Clear processed messages
                                    if let Err(e) = chat_ui.clear_unprocessed_messages(&page).await {
                                        log::error!("Failed to clear unprocessed messages: {}", e);
                                    }
                                }
                                Ok(_) => {
                                    // No messages, continue polling
                                }
                                Err(e) => {
                                    log::warn!("Error checking for chat messages: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

/// Internal helper to process chat messages (used by polling task)
async fn process_chat_message_internal(
    app: &AppHandle,
    driver: &Arc<tokio::sync::Mutex<Option<ChromeDriver>>>,
    message: String,
) -> Result<crate::agent::WorkflowResult, String> {
    use crate::agent::{AgentConfig, WorkflowExecutor, WorkflowType};

    log::info!("╔═══════════════════════════════════════════════════════════╗");
    log::info!("║  💬 PROCESSING CHAT MESSAGE                               ║");
    log::info!("╚═══════════════════════════════════════════════════════════╝");
    log::info!("📝 Message: {}", message);

    emit_info(app, "Processing chat message...").ok();

    // Load agent configuration
    let agent_name = "cdp-generator";
    let agent_config = {
        let config_path = AgentConfig::config_path(agent_name)
            .map_err(|e| format!("Failed to get config path: {}", e))?;

        if config_path.exists() {
            AgentConfig::load(&config_path)
                .await
                .map_err(|e| format!("Failed to load agent config: {}", e))?
        } else {
            let config = AgentConfig::default_cdp_agent();
            config
                .save(&config_path)
                .await
                .map_err(|e| format!("Failed to save agent config: {}", e))?;
            emit_info(app, format!("Created default config for '{}'", agent_name)).ok();
            config
        }
    };

    log::info!("✓ Agent config loaded: {}", agent_config.name);

    // Get screenshot and HTML
    log::info!("📸 Attempting to capture screenshot...");
    let driver_lock = driver.lock().await;
    let driver_ref = driver_lock.as_ref();

    let screenshot_path = if let Some(drv) = driver_ref {
        let temp_dir = std::env::temp_dir().join("robert-chat");
        log::debug!("Screenshot temp directory: {:?}", temp_dir);

        match tokio::fs::create_dir_all(&temp_dir).await {
            Ok(_) => {
                log::debug!("✓ Temp directory created/verified");
                let timestamp = chrono::Utc::now().timestamp();
                let screenshot_path = temp_dir.join(format!("chat-screenshot-{}.png", timestamp));
                log::debug!("Target screenshot path: {:?}", screenshot_path);

                match drv.screenshot_to_file(&screenshot_path).await {
                    Ok(_) => {
                        // Verify file was actually written
                        match tokio::fs::metadata(&screenshot_path).await {
                            Ok(metadata) => {
                                let size_kb = metadata.len() / 1024;
                                log::info!("✓ Screenshot captured successfully: {:?} ({} KB)", screenshot_path, size_kb);
                                emit_info(app, format!("Captured screenshot ({} KB)", size_kb)).ok();
                                Some(screenshot_path)
                            }
                            Err(e) => {
                                log::error!("❌ Screenshot file not found after save: {}", e);
                                emit_error(app, "Screenshot save verification failed", Some(e.to_string())).ok();
                                None
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("❌ Screenshot capture failed: {}", e);
                        emit_error(app, "Failed to capture screenshot", Some(e.to_string())).ok();
                        None
                    }
                }
            }
            Err(e) => {
                log::error!("❌ Failed to create temp directory {:?}: {}", temp_dir, e);
                emit_error(app, "Failed to create screenshot directory", Some(e.to_string())).ok();
                None
            }
        }
    } else {
        log::warn!("⚠️  No driver available for screenshot (browser not launched)");
        emit_error(app, "Cannot capture screenshot", Some("Browser not launched".to_string())).ok();
        None
    };

    log::info!("📄 Attempting to extract HTML...");
    let html_content = if let Some(drv) = driver_ref {
        match drv.get_page_source().await {
            Ok(html) => {
                log::info!("✓ HTML extracted ({} KB)", html.len() / 1024);
                emit_info(app, format!("Extracted {} KB of HTML", html.len() / 1024)).ok();
                Some(html)
            }
            Err(e) => {
                log::warn!("⚠️  HTML extraction failed: {}", e);
                None
            }
        }
    } else {
        log::warn!("⚠️  No driver available for HTML extraction");
        None
    };

    log::info!("🚀 Initiating workflow executor...");
    let executor = WorkflowExecutor::new();
    emit_claude_processing(app, "Executing CDP generation workflow...").ok();

    log::info!("🔄 Executing workflow...");
    // Execute workflow
    let result = executor
        .execute(
            WorkflowType::CdpAutomation,
            message.clone(),
            &agent_config,
            screenshot_path,
            html_content,
            driver_ref,
        )
        .await
        .map_err(|e| {
            log::error!("❌ Workflow execution error: {}", e);
            format!("Workflow execution failed: {}", e)
        })?;

    log::info!("✅ Workflow execution completed");

    if result.success {
        emit_success(app, result.message.clone()).ok();
    } else {
        emit_error(app, result.message.clone(), result.error.clone()).ok();
    }

    Ok(result)
}

/// Navigate to a URL
#[tauri::command]
pub async fn navigate_to_url(
    app: AppHandle,
    state: State<'_, AppState>,
    url: String,
) -> Result<NavigationResult, String> {
    // Ensure browser is launched
    let driver_lock = state.driver.lock().await;

    if driver_lock.is_none() {
        let msg = "Browser not launched. Please launch browser first.";
        emit_error(&app, msg, None).ok();
        return Err(msg.to_string());
    }

    let driver = driver_lock.as_ref().unwrap();

    // Emit navigation started
    emit_page_navigating(&app, &url).ok();
    emit_info(&app, format!("Navigating to: {}", url)).ok();

    // Navigate to URL
    match driver.navigate(&url).await {
        Ok(_) => {
            emit_info(&app, "Page loading...").ok();

            // Get page title
            let title = driver
                .title()
                .await
                .unwrap_or_else(|_| "Unknown".to_string());

            emit_page_loaded(&app, &url, &title).ok();

            // Inject chat UI after page loads
            let chat_ui = ChatUI::new();
            if let Ok(page) = driver.current_page().await {
                if let Err(e) = chat_ui.inject(&page).await {
                    log::warn!("Failed to inject chat UI: {}", e);
                } else {
                    log::info!("Chat UI injected successfully");
                    emit_info(&app, "Chat UI injected").ok();
                }
            }

            emit_success(&app, format!("Successfully loaded: {}", title)).ok();

            Ok(NavigationResult {
                success: true,
                url: url.clone(),
                title,
                message: "Navigation successful".to_string(),
            })
        }
        Err(e) => {
            let error_msg = format!("Navigation failed: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    }
}

/// Get current page content
#[tauri::command]
pub async fn get_page_content(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let driver_lock = state.driver.lock().await;

    if driver_lock.is_none() {
        return Err("Browser not launched".to_string());
    }

    let driver = driver_lock.as_ref().unwrap();

    emit_info(&app, "Extracting page content...").ok();

    match driver.get_page_text().await {
        Ok(text) => {
            emit_success(&app, "Content extracted successfully").ok();
            Ok(text)
        }
        Err(e) => {
            let error_msg = format!("Failed to get content: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    }
}

/// Close the browser
#[tauri::command]
pub async fn close_browser(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let mut driver_lock = state.driver.lock().await;

    if let Some(driver) = driver_lock.take() {
        emit_info(&app, "Closing browser...").ok();
        drop(driver);
        emit_success(&app, "Browser closed").ok();
        Ok(())
    } else {
        Err("Browser not running".to_string())
    }
}

/// Take a screenshot of the current page
#[tauri::command]
pub async fn take_screenshot(
    app: AppHandle,
    state: State<'_, AppState>,
    output_path: String,
) -> Result<String, String> {
    let driver_lock = state.driver.lock().await;

    if driver_lock.is_none() {
        return Err("Browser not launched".to_string());
    }

    let driver = driver_lock.as_ref().unwrap();

    emit_info(&app, "Taking screenshot...").ok();

    let path = PathBuf::from(&output_path);
    match driver.screenshot_to_file(&path).await {
        Ok(_) => {
            emit_success(&app, format!("Screenshot saved to: {}", output_path)).ok();
            Ok(output_path)
        }
        Err(e) => {
            let error_msg = format!("Failed to take screenshot: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    }
}

/// Call Claude CLI with screenshot and HTML
#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeRequest {
    pub prompt: String,
    pub screenshot_path: Option<String>,
    pub include_html: bool,
    pub model: Option<String>,
}

#[tauri::command]
pub async fn ask_claude(
    app: AppHandle,
    state: State<'_, AppState>,
    request: ClaudeRequest,
) -> Result<ClaudeResponse, String> {
    let driver_lock = state.driver.lock().await;

    if driver_lock.is_none() {
        return Err("Browser not launched".to_string());
    }

    let driver = driver_lock.as_ref().unwrap();

    emit_claude_processing(&app, "Preparing data for Claude...").ok();

    // Get HTML if requested
    let html = if request.include_html {
        emit_info(&app, "Extracting page HTML...").ok();
        match driver.get_page_source().await {
            Ok(html) => {
                let size_kb = html.len() / 1024;
                emit_claude_html_extracted(&app, size_kb).ok();
                Some(html)
            }
            Err(e) => {
                emit_error(&app, "Failed to get HTML".to_string(), Some(e.to_string())).ok();
                None
            }
        }
    } else {
        None
    };

    // Build Claude input
    let images = if let Some(path) = request.screenshot_path.clone() {
        emit_claude_screenshot(&app, &path).ok();
        vec![PathBuf::from(path)]
    } else {
        vec![]
    };

    let input = ClaudeInput {
        prompt: request.prompt.clone(),
        images,
        html,
    };

    // Configure Claude client
    let mut config = ClaudeConfig::default();
    let model_name = request
        .model
        .clone()
        .unwrap_or_else(|| "default".to_string());
    config.model = request.model;
    config.skip_permissions = true; // For automation purposes

    let client = ClaudeClient::with_config(config);

    // Show prompt preview (first 100 chars)
    let prompt_preview = if request.prompt.len() > 100 {
        format!("{}...", &request.prompt[..100])
    } else {
        request.prompt.clone()
    };
    emit_claude_api_call(&app, model_name, prompt_preview).ok();

    // Execute Claude
    match client.execute(input).await {
        Ok(response) => {
            let preview = if response.text.len() > 200 {
                format!("{}...", &response.text[..200])
            } else {
                response.text.clone()
            };
            emit_claude_response(&app, preview, response.text.len()).ok();
            emit_success(&app, "Claude response received").ok();
            Ok(response)
        }
        Err(e) => {
            let error_msg = format!("Claude CLI failed: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    }
}

/// Call Claude CLI with screenshot and HTML (one-shot helper)
#[tauri::command]
pub async fn ask_claude_about_page(
    app: AppHandle,
    state: State<'_, AppState>,
    prompt: String,
    model: Option<String>,
) -> Result<ClaudeResponse, String> {
    let driver_lock = state.driver.lock().await;

    if driver_lock.is_none() {
        return Err("Browser not launched".to_string());
    }

    let driver = driver_lock.as_ref().unwrap();

    emit_claude_processing(&app, "Taking screenshot for Claude...").ok();

    // Create temp directory for screenshot
    let temp_dir = std::env::temp_dir().join("robert-screenshots");
    tokio::fs::create_dir_all(&temp_dir)
        .await
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;

    let timestamp = chrono::Utc::now().timestamp();
    let screenshot_path = temp_dir.join(format!("screenshot-{}.png", timestamp));

    // Take screenshot
    driver
        .screenshot_to_file(&screenshot_path)
        .await
        .map_err(|e| format!("Failed to take screenshot: {}", e))?;

    emit_claude_screenshot(&app, screenshot_path.to_string_lossy()).ok();

    emit_info(&app, "Getting page HTML...").ok();

    // Get HTML
    let html = driver
        .get_page_source()
        .await
        .map_err(|e| format!("Failed to get HTML: {}", e))?;

    let html_size_kb = html.len() / 1024;
    emit_claude_html_extracted(&app, html_size_kb).ok();

    // Build Claude input
    let input = ClaudeInput {
        prompt: prompt.clone(),
        images: vec![screenshot_path.clone()],
        html: Some(html),
    };

    // Configure Claude client
    let mut config = ClaudeConfig::default();
    let model_name = model.clone().unwrap_or_else(|| "default".to_string());
    config.model = model;
    config.skip_permissions = true;

    let client = ClaudeClient::with_config(config);

    // Show prompt preview
    let prompt_preview = if prompt.len() > 100 {
        format!("{}...", &prompt[..100])
    } else {
        prompt.clone()
    };
    emit_claude_api_call(&app, model_name, prompt_preview).ok();

    // Execute Claude
    let result = match client.execute(input).await {
        Ok(response) => {
            let preview = if response.text.len() > 200 {
                format!("{}...", &response.text[..200])
            } else {
                response.text.clone()
            };
            emit_claude_response(&app, preview, response.text.len()).ok();
            emit_success(&app, "Claude response received").ok();
            Ok(response)
        }
        Err(e) => {
            let error_msg = format!("Claude CLI failed: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    };

    // Clean up screenshot
    let _ = tokio::fs::remove_file(&screenshot_path).await;

    result
}

/// Check Claude CLI installation and configuration
#[tauri::command]
pub async fn check_claude_health(app: AppHandle) -> Result<ClaudeHealthCheck, String> {
    emit_claude_checking(&app, "Checking Claude CLI installation...").ok();

    let health = ClaudeHealthCheck::check().await;

    // Emit appropriate event based on health status
    match health.status {
        crate::claude::HealthStatus::Healthy => {
            emit_claude_ready(
                &app,
                health.version.as_deref().unwrap_or("unknown"),
                health.path.as_deref().unwrap_or("unknown"),
                health.authenticated,
            )
            .ok();
        }
        crate::claude::HealthStatus::Warning | crate::claude::HealthStatus::Error => {
            if let Some(issue) = health.issues.first() {
                let suggestion = health
                    .suggestions
                    .first()
                    .map(|s| s.as_str())
                    .unwrap_or("See documentation for setup instructions");
                emit_claude_not_ready(&app, issue, suggestion).ok();
            }
        }
    }

    Ok(health)
}

/// System diagnostics - check all dependencies
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemDiagnostics {
    pub chrome_status: String,
    pub chrome_installed: bool,
    pub claude_health: ClaudeHealthCheck,
    pub browser_running: bool,
    pub current_url: Option<String>,
}

#[tauri::command]
pub async fn run_diagnostics(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<SystemDiagnostics, String> {
    log::info!("Running system diagnostics...");
    emit_info(&app, "Running system diagnostics...").ok();

    // Check Claude
    let claude_health = ClaudeHealthCheck::check().await;

    // Check if Chrome/Chromium is installed (try to detect)
    let chrome_installed = check_chrome_installed().await;

    // Check browser status
    let driver_lock = state.driver.lock().await;
    let browser_running = driver_lock.is_some();

    let current_url = if let Some(driver) = driver_lock.as_ref() {
        driver.current_url().await.ok()
    } else {
        None
    };

    let chrome_status = if browser_running {
        "Running".to_string()
    } else if chrome_installed {
        "Installed (not running)".to_string()
    } else {
        "Not installed".to_string()
    };

    drop(driver_lock);

    log::info!(
        "Diagnostics complete - Chrome: {}, Claude: {:?}",
        chrome_status,
        claude_health.status
    );

    let diagnostics = SystemDiagnostics {
        chrome_status,
        chrome_installed,
        claude_health,
        browser_running,
        current_url,
    };

    emit_success(&app, "Diagnostics complete").ok();

    Ok(diagnostics)
}

/// Check if Chrome/Chromium is installed on the system
async fn check_chrome_installed() -> bool {
    use std::process::Command;

    // Try common Chrome/Chromium locations and commands
    let commands = vec![
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
    ];

    for cmd in commands {
        if let Ok(output) = Command::new(cmd).arg("--version").output() {
            if output.status.success() {
                log::debug!("Found Chrome at: {}", cmd);
                return true;
            }
        }
    }

    // Also check if chromiumoxide can auto-download
    log::debug!("Chrome not found in system, but auto-download is available");
    false
}

/// Validate a CDP script from JSON string
#[tauri::command]
pub async fn validate_cdp_script(app: AppHandle, json: String) -> Result<ValidationResult, String> {
    emit_info(&app, "Validating CDP script...").ok();

    let validator = CdpValidator::new();
    let result = validator.validate_json(&json);

    if result.is_valid {
        emit_success(&app, "CDP script validation passed").ok();
    } else {
        let error_count = result.errors.len();
        let warning_count = result.warnings.len();
        emit_error(
            &app,
            format!(
                "CDP script validation failed: {} error(s), {} warning(s)",
                error_count, warning_count
            ),
            Some(format!("Found {} validation errors", error_count)),
        )
        .ok();
    }

    Ok(result)
}

/// Validate a CDP script file
#[tauri::command]
pub async fn validate_cdp_script_file(
    app: AppHandle,
    file_path: String,
) -> Result<ValidationResult, String> {
    emit_info(&app, format!("Validating CDP script file: {}", file_path)).ok();

    // Read the file
    let json = tokio::fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Validate
    let validator = CdpValidator::new();
    let result = validator.validate_json(&json);

    if result.is_valid {
        emit_success(&app, format!("CDP script file {} is valid", file_path)).ok();
    } else {
        let error_count = result.errors.len();
        emit_error(
            &app,
            format!("CDP script file {} has {} error(s)", file_path, error_count),
            None,
        )
        .ok();
    }

    Ok(result)
}
