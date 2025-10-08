use crate::events::*;
use crate::state::AppState;
use robert_webdriver::ChromeDriver;
use serde::{Deserialize, Serialize};
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
            Ok("Browser launched successfully".to_string())
        }
        Err(e) => {
            let error_msg = format!("Failed to launch browser: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    }
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
