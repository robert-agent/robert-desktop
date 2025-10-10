//! Developer mode Tauri commands

use crate::developer_mode::{DevTestServer, SystemPaths};
use crate::events::*;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
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
