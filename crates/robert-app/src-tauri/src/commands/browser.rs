/// Tauri commands for browser session management (Phase 2)
///
/// This module provides Tauri commands that expose browser session functionality
/// to the frontend, including:
/// - Launching browser sessions with ephemeral profiles
/// - Closing browser sessions and cleaning up resources
/// - Querying active session status and information
///
/// All commands follow Tauri's error handling conventions by returning
/// Result<T, String> where errors are serialized as strings for the frontend.
use crate::profiles::browser::{BrowserConfig, SessionError, SessionId, SessionInfo};
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to launch a new browser session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchBrowserRequest {
    /// Whether to launch in headless mode (optional, default: false)
    #[serde(default)]
    pub headless: bool,

    /// Whether to disable sandbox (optional, default: false)
    #[serde(default)]
    pub no_sandbox: bool,
}

/// Response from launching a browser session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchBrowserResponse {
    /// Unique session ID for the launched browser
    pub session_id: String,

    /// Session information
    pub session_info: SessionInfo,
}

/// Response from closing a browser session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseBrowserResponse {
    /// Whether the session was successfully closed
    pub success: bool,

    /// Human-readable message
    pub message: String,
}

/// Response with browser status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserStatusResponse {
    /// Whether there are any active sessions
    pub has_active_sessions: bool,

    /// Number of active sessions
    pub active_session_count: usize,

    /// List of all active sessions
    pub sessions: Vec<SessionInfo>,
}

// ============================================================================
// Error Conversion
// ============================================================================

/// Convert SessionError to a string for Tauri error responses
impl From<SessionError> for String {
    fn from(err: SessionError) -> Self {
        err.to_string()
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

/// Launch a new browser session with an ephemeral profile
///
/// This command:
/// 1. Creates a new ephemeral profile
/// 2. Launches Chrome with the profile
/// 3. Returns the session ID and info
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const response = await invoke<LaunchBrowserResponse>('launch_browser_session', {
///   request: {
///     headless: false,
///     no_sandbox: false,
///   }
/// });
///
/// console.log('Session ID:', response.session_id);
/// ```
///
/// # Parameters
/// - `request`: Launch configuration (headless, no_sandbox)
/// - `state`: App state containing the session manager
///
/// # Returns
/// - `Ok(LaunchBrowserResponse)`: Session ID and info
///
/// # Errors
/// - Returns error string if launch fails or max sessions reached
#[tauri::command]
pub async fn launch_browser_session(
    request: LaunchBrowserRequest,
    state: State<'_, AppState>,
) -> Result<LaunchBrowserResponse, String> {
    log::info!(
        "Launching browser (headless: {}, no_sandbox: {})",
        request.headless,
        request.no_sandbox
    );

    // Build browser config from request
    let config = BrowserConfig::new()
        .headless(request.headless)
        .no_sandbox(request.no_sandbox);

    // Launch session
    let session_id = state.session_manager.launch_session(config).await?;

    // Get session info
    let session_info = state.session_manager.get_session_info(&session_id).await?;

    log::info!("Browser session launched successfully: {}", session_id);

    Ok(LaunchBrowserResponse {
        session_id: session_id.to_string(),
        session_info,
    })
}

/// Close a browser session and clean up resources
///
/// This command:
/// 1. Closes the browser instance
/// 2. Cleans up the ephemeral profile directory
/// 3. Removes the session from the active sessions map
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const response = await invoke<CloseBrowserResponse>('close_browser_session', {
///   sessionId: 'session-uuid-here'
/// });
///
/// console.log('Closed:', response.success);
/// ```
///
/// # Parameters
/// - `session_id`: The session ID to close
/// - `state`: App state containing the session manager
///
/// # Returns
/// - `Ok(CloseBrowserResponse)`: Success status and message
///
/// # Errors
/// - Returns error string if session not found or cleanup fails
#[tauri::command]
pub async fn close_browser_session(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<CloseBrowserResponse, String> {
    log::info!("Closing browser session: {}", session_id);

    // Parse session ID
    let id = SessionId::from_string(&session_id);

    // Close session
    state.session_manager.close_session(&id).await?;

    log::info!("Browser session closed successfully: {}", session_id);

    Ok(CloseBrowserResponse {
        success: true,
        message: format!("Session {} closed successfully", session_id),
    })
}

/// Get the current browser status and active sessions
///
/// This command returns information about all active browser sessions,
/// which can be displayed in the UI.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const status = await invoke<BrowserStatusResponse>('get_browser_status');
///
/// console.log('Active sessions:', status.active_session_count);
/// status.sessions.forEach(session => {
///   console.log('Session:', session.id, session.profile_name);
/// });
/// ```
///
/// # Parameters
/// - `state`: App state containing the session manager
///
/// # Returns
/// - `Ok(BrowserStatusResponse)`: Status information and session list
///
/// # Errors
/// - This command should never fail in normal operation
#[tauri::command]
pub async fn get_browser_status(
    state: State<'_, AppState>,
) -> Result<BrowserStatusResponse, String> {
    let has_active_sessions = state.session_manager.has_active_sessions().await;
    let active_session_count = state.session_manager.session_count().await;
    let sessions = state.session_manager.list_sessions().await;

    Ok(BrowserStatusResponse {
        has_active_sessions,
        active_session_count,
        sessions,
    })
}

/// Close all active browser sessions
///
/// This is a convenience command for cleanup, typically called when
/// the app is shutting down or the user wants to close everything.
///
/// # Frontend Usage
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
///
/// const count = await invoke<number>('close_all_browser_sessions');
/// console.log('Closed', count, 'sessions');
/// ```
///
/// # Parameters
/// - `state`: App state containing the session manager
///
/// # Returns
/// - `Ok(usize)`: Number of sessions closed
///
/// # Errors
/// - Returns error string if cleanup fails for any session
#[tauri::command]
pub async fn close_all_browser_sessions(state: State<'_, AppState>) -> Result<usize, String> {
    log::info!("Closing all browser sessions");

    let count = state.session_manager.close_all_sessions().await?;

    log::info!("Closed {} browser sessions", count);

    Ok(count)
}

// ============================================================================
// Helper for SessionId
// ============================================================================

impl SessionId {
    /// Create a SessionId from a string (for Tauri command deserialization)
    pub fn from_string(s: &str) -> Self {
        Self(s.to_string())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_browser_request_default() {
        let request = serde_json::from_str::<LaunchBrowserRequest>("{}").unwrap();
        assert!(!request.headless);
        assert!(!request.no_sandbox);
    }

    #[test]
    fn test_launch_browser_request_custom() {
        let json = r#"{"headless": true, "no_sandbox": true}"#;
        let request = serde_json::from_str::<LaunchBrowserRequest>(json).unwrap();
        assert!(request.headless);
        assert!(request.no_sandbox);
    }

    #[test]
    fn test_session_id_from_string() {
        let id = SessionId::from_string("test-id");
        assert_eq!(id.as_str(), "test-id");
    }
}
