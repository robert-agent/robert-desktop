//! Developer mode Tauri commands

use crate::developer_mode::{DevTestServer, SystemPaths};
use crate::events::*;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, State};

/// Get system paths for developer mode
#[tauri::command]
pub async fn get_system_paths(app: AppHandle) -> Result<SystemPaths, String> {
    SystemPaths::get(&app).map_err(|e| format!("Failed to get system paths: {}", e))
}

/// Test server status
#[derive(Debug, Serialize, Deserialize)]
pub struct TestServerStatus {
    pub running: bool,
    pub url: Option<String>,
    pub port: Option<u16>,
}

/// Start the developer mode test server
#[tauri::command]
pub async fn start_dev_test_server(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<TestServerStatus, String> {
    let mut server_lock = state.dev_server.lock().await;

    // If already running, return current status
    if let Some(server) = server_lock.as_ref() {
        emit_info(&app, "Dev test server already running").ok();
        return Ok(TestServerStatus {
            running: true,
            url: Some(server.url()),
            port: Some(server.port()),
        });
    }

    emit_info(&app, "Starting developer test server...").ok();

    // Start new server
    match DevTestServer::start().await {
        Ok(server) => {
            // Wait for server to be ready
            if let Err(e) = server.wait_ready().await {
                emit_error(
                    &app,
                    "Test server started but not responding",
                    Some(e.to_string()),
                )
                .ok();
                return Err(format!("Test server not ready: {}", e));
            }

            let url = server.url();
            let port = server.port();

            emit_success(&app, format!("Dev test server started at {}", url)).ok();

            let status = TestServerStatus {
                running: true,
                url: Some(url),
                port: Some(port),
            };

            *server_lock = Some(server);
            Ok(status)
        }
        Err(e) => {
            let error_msg = format!("Failed to start test server: {}", e);
            emit_error(&app, error_msg.clone(), Some(e.to_string())).ok();
            Err(error_msg)
        }
    }
}

/// Stop the developer mode test server
#[tauri::command]
pub async fn stop_dev_test_server(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<TestServerStatus, String> {
    let mut server_lock = state.dev_server.lock().await;

    if let Some(server) = server_lock.take() {
        emit_info(&app, "Stopping developer test server...").ok();
        drop(server); // Explicitly drop to trigger shutdown
        emit_success(&app, "Dev test server stopped").ok();
    }

    Ok(TestServerStatus {
        running: false,
        url: None,
        port: None,
    })
}

/// Get the current status of the developer mode test server
#[tauri::command]
pub async fn get_dev_test_server_status(
    state: State<'_, AppState>,
) -> Result<TestServerStatus, String> {
    let server_lock = state.dev_server.lock().await;

    if let Some(server) = server_lock.as_ref() {
        Ok(TestServerStatus {
            running: true,
            url: Some(server.url()),
            port: Some(server.port()),
        })
    } else {
        Ok(TestServerStatus {
            running: false,
            url: None,
            port: None,
        })
    }
}

// ===== SCREENSHOT MANAGEMENT COMMANDS =====

/// Information about a screenshot file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenshotInfo {
    pub path: String,
    pub filename: String,
    pub timestamp: i64,
    pub size_bytes: u64,
    pub size_kb: u64,
}

/// Capture a screenshot manually (for developer mode testing)
#[tauri::command]
pub async fn dev_capture_screenshot(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ScreenshotInfo, String> {
    log::info!("📸 [Dev Mode] Manual screenshot capture requested");
    emit_info(&app, "Capturing screenshot...").ok();

    let driver_lock = state.driver.lock().await;

    if driver_lock.is_none() {
        let msg = "Browser not launched. Please launch browser first.";
        log::error!("❌ [Dev Mode] {}", msg);
        emit_error(&app, msg, None).ok();
        return Err(msg.to_string());
    }

    let driver = driver_lock.as_ref().unwrap();

    // Get session ID
    let session_id = state.session_id.lock().await.clone();

    // Create screenshots directory organized by session
    let base_dir = std::env::temp_dir().join("robert-screenshots");
    let session_dir = base_dir.join(&session_id);
    log::debug!("Screenshot session directory: {:?}", session_dir);

    tokio::fs::create_dir_all(&session_dir)
        .await
        .map_err(|e| {
            log::error!("❌ Failed to create screenshot directory: {}", e);
            format!("Failed to create screenshot directory: {}", e)
        })?;

    // Generate filename with detailed timestamp (for proper ordering)
    let now = chrono::Utc::now();
    let timestamp = now.timestamp();
    let filename = format!(
        "screenshot_{}.png",
        now.format("%Y%m%d_%H%M%S_%3f")
    );
    let screenshot_path = session_dir.join(&filename);
    log::debug!("Target path: {:?}", screenshot_path);

    // Capture screenshot
    log::info!("📸 Capturing screenshot to {:?}", screenshot_path);
    driver
        .screenshot_to_file(&screenshot_path)
        .await
        .map_err(|e| {
            log::error!("❌ Screenshot capture failed: {}", e);
            emit_error(&app, "Screenshot capture failed", Some(e.to_string())).ok();
            format!("Screenshot capture failed: {}", e)
        })?;

    // Verify and get file info
    let metadata = tokio::fs::metadata(&screenshot_path)
        .await
        .map_err(|e| {
            log::error!("❌ Screenshot file not found after capture: {}", e);
            format!("Failed to verify screenshot file: {}", e)
        })?;

    let size_bytes = metadata.len();
    let size_kb = size_bytes / 1024;

    log::info!("✓ Screenshot captured: {} ({} KB)", filename, size_kb);
    emit_success(&app, format!("Screenshot captured: {} ({} KB)", filename, size_kb)).ok();

    Ok(ScreenshotInfo {
        path: screenshot_path.to_string_lossy().to_string(),
        filename,
        timestamp,
        size_bytes,
        size_kb,
    })
}

/// List all screenshots in the current session
#[tauri::command]
pub async fn dev_list_screenshots(
    _app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<ScreenshotInfo>, String> {
    log::debug!("📋 [Dev Mode] Listing screenshots");

    // Get session ID
    let session_id = state.session_id.lock().await.clone();

    let base_dir = std::env::temp_dir().join("robert-screenshots");
    let session_dir = base_dir.join(&session_id);

    if !session_dir.exists() {
        log::debug!("Screenshot directory doesn't exist yet for session {}", session_id);
        return Ok(vec![]);
    }

    let mut screenshots = Vec::new();
    let mut entries = tokio::fs::read_dir(&session_dir)
        .await
        .map_err(|e| format!("Failed to read screenshot directory: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("Failed to read entry: {}", e))? {
        let path = entry.path();

        // Only include PNG files with our naming pattern
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with("screenshot_") && filename.ends_with(".png") {
                if let Ok(metadata) = tokio::fs::metadata(&path).await {
                    // Use file modification time as timestamp
                    let timestamp = metadata
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);

                    let size_bytes = metadata.len();
                    let size_kb = size_bytes / 1024;

                    screenshots.push(ScreenshotInfo {
                        path: path.to_string_lossy().to_string(),
                        filename: filename.to_string(),
                        timestamp,
                        size_bytes,
                        size_kb,
                    });
                }
            }
        }
    }

    // Sort by timestamp (newest first)
    screenshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    log::debug!("✓ Found {} screenshots in session {}", screenshots.len(), session_id);
    Ok(screenshots)
}

/// Delete all screenshots in the current session
#[tauri::command]
pub async fn dev_delete_all_screenshots(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    log::info!("🗑️  [Dev Mode] Deleting all screenshots");
    emit_info(&app, "Deleting screenshots...").ok();

    // Get session ID
    let session_id = state.session_id.lock().await.clone();

    let base_dir = std::env::temp_dir().join("robert-screenshots");
    let session_dir = base_dir.join(&session_id);

    if !session_dir.exists() {
        log::debug!("Screenshot directory doesn't exist for session {}", session_id);
        return Ok(0);
    }

    let mut count = 0;
    let mut entries = tokio::fs::read_dir(&session_dir)
        .await
        .map_err(|e| format!("Failed to read screenshot directory: {}", e))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| format!("Failed to read entry: {}", e))? {
        let path = entry.path();

        // Only delete PNG files with our naming pattern
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with("screenshot_") && filename.ends_with(".png") {
                match tokio::fs::remove_file(&path).await {
                    Ok(_) => {
                        log::debug!("Deleted: {}", filename);
                        count += 1;
                    }
                    Err(e) => {
                        log::warn!("Failed to delete {}: {}", filename, e);
                    }
                }
            }
        }
    }

    log::info!("✓ Deleted {} screenshot(s) from session {}", count, session_id);
    emit_success(&app, format!("Deleted {} screenshot(s)", count)).ok();

    Ok(count)
}

/// Delete a specific screenshot
#[tauri::command]
pub async fn dev_delete_screenshot(
    app: AppHandle,
    state: State<'_, AppState>,
    path: String,
) -> Result<(), String> {
    log::info!("🗑️  [Dev Mode] Deleting screenshot: {}", path);

    let screenshot_path = PathBuf::from(&path);

    // Get session ID
    let session_id = state.session_id.lock().await.clone();

    // Verify it's in our screenshot directory for safety
    let base_dir = std::env::temp_dir().join("robert-screenshots");
    let session_dir = base_dir.join(&session_id);

    if !screenshot_path.starts_with(&session_dir) {
        let msg = "Invalid screenshot path (not in current session directory)";
        log::error!("❌ {}", msg);
        return Err(msg.to_string());
    }

    // Verify filename pattern for safety
    if let Some(filename) = screenshot_path.file_name().and_then(|n| n.to_str()) {
        if !filename.starts_with("screenshot_") || !filename.ends_with(".png") {
            let msg = "Invalid screenshot filename pattern";
            log::error!("❌ {}", msg);
            return Err(msg.to_string());
        }
    }

    tokio::fs::remove_file(&screenshot_path)
        .await
        .map_err(|e| {
            log::error!("❌ Failed to delete screenshot: {}", e);
            format!("Failed to delete screenshot: {}", e)
        })?;

    log::info!("✓ Screenshot deleted from session {}", session_id);
    emit_success(&app, "Screenshot deleted").ok();

    Ok(())
}
