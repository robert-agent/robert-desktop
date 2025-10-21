/// Browser session management for tracking active browser instances
///
/// This module manages the lifecycle of browser sessions, including:
/// - Creating and tracking active sessions
/// - Mapping session IDs to browser instances and profiles
/// - Closing sessions and cleaning up resources
///
/// For Phase 2, we implement a simple single-session manager.
/// Multiple simultaneous sessions are deferred to later phases.
use super::launcher::{BrowserConfig, BrowserLauncher, LauncherError};
use super::profile::BrowserProfile;
use robert_webdriver::browser::chrome::ChromeDriver;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during session management
#[derive(Error, Debug)]
pub enum SessionError {
    /// Session not found
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    /// Session already exists
    #[error("Session already exists: {0}")]
    SessionAlreadyExists(String),

    /// Failed to launch session
    #[error("Failed to launch session: {0}")]
    LaunchFailed(String),

    /// Failed to close session
    #[error("Failed to close session: {0}")]
    CloseFailed(String),

    /// Launcher error
    #[error("Launcher error: {0}")]
    LauncherError(#[from] LauncherError),

    /// Profile error
    #[error("Profile error: {0}")]
    ProfileError(#[from] super::profile::ProfileError),

    /// Maximum sessions limit reached
    #[error("Maximum active sessions limit reached: {0}")]
    MaxSessionsReached(usize),
}

pub type Result<T> = std::result::Result<T, SessionError>;

// ============================================================================
// Session Types
// ============================================================================

/// Unique identifier for a browser session
///
/// Each active browser instance is assigned a unique SessionId.
/// This ID is used to reference and manage the session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

impl SessionId {
    /// Create a new unique session ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Get the string representation of the session ID
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Browser session containing the driver and profile information
///
/// This struct encapsulates all the state for a single browser session,
/// including the ChromeDriver instance, profile, and metadata.
pub struct BrowserSession {
    /// Unique session identifier
    pub id: SessionId,

    /// ChromeDriver instance for browser automation
    pub driver: ChromeDriver,

    /// Browser profile used for this session
    pub profile: BrowserProfile,

    /// Session creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Browser configuration used
    pub config: BrowserConfig,
}

impl BrowserSession {
    /// Create a new browser session
    fn new(
        id: SessionId,
        driver: ChromeDriver,
        profile: BrowserProfile,
        config: BrowserConfig,
    ) -> Self {
        Self {
            id,
            driver,
            profile,
            created_at: chrono::Utc::now(),
            config,
        }
    }

    /// Get session information for UI display
    pub fn info(&self) -> SessionInfo {
        SessionInfo {
            id: self.id.clone(),
            profile_name: self.profile.display_name(),
            profile_type: if self.profile.is_ephemeral() {
                "ephemeral".to_string()
            } else {
                "named".to_string()
            },
            created_at: self.created_at,
            headless: self.config.headless,
        }
    }
}

/// Session information for UI display
///
/// This struct contains metadata about a session that can be
/// serialized and sent to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    pub id: SessionId,

    /// Profile display name
    pub profile_name: String,

    /// Profile type ("ephemeral" or "named")
    pub profile_type: String,

    /// Session creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Whether browser is running in headless mode
    pub headless: bool,
}

// ============================================================================
// Session Manager
// ============================================================================

/// Session manager for browser instances
///
/// This manager tracks all active browser sessions and provides
/// methods to create, retrieve, and close sessions.
///
/// For Phase 2, we support a single active session. Multiple simultaneous
/// sessions will be added in later phases.
///
/// # Thread Safety
/// The SessionManager uses Arc<RwLock<...>> to safely share state across
/// Tauri command handlers and async tasks.
///
/// # Example
/// ```no_run
/// use robert_app_lib::profiles::browser::session::SessionManager;
/// use robert_app_lib::profiles::browser::launcher::BrowserConfig;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let manager = SessionManager::new();
///
/// // Launch a session
/// let session_id = manager
///     .launch_session(BrowserConfig::new())
///     .await?;
///
/// // Get session info
/// let info = manager.get_session_info(&session_id).await?;
///
/// // Close session when done
/// manager.close_session(&session_id).await?;
/// # Ok(())
/// # }
/// ```
pub struct SessionManager {
    /// Map of active sessions (SessionId -> BrowserSession)
    sessions: Arc<RwLock<HashMap<SessionId, BrowserSession>>>,

    /// Browser launcher
    launcher: BrowserLauncher,

    /// Maximum number of simultaneous sessions (Phase 2: 1, later: unlimited)
    max_sessions: usize,
}

impl SessionManager {
    /// Create a new session manager
    ///
    /// For Phase 2, we limit to 1 active session. This will be configurable
    /// in later phases.
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            launcher: BrowserLauncher::new(),
            max_sessions: 1, // Phase 2: single session only
        }
    }

    /// Launch a new browser session
    ///
    /// This method:
    /// 1. Checks if we've reached the max sessions limit
    /// 2. Launches Chrome with an ephemeral profile
    /// 3. Creates a new session and stores it
    /// 4. Returns the session ID
    ///
    /// # Parameters
    /// - `config`: Browser configuration (headless, sandbox, etc.)
    ///
    /// # Returns
    /// - `Ok(SessionId)`: The ID of the newly created session
    ///
    /// # Errors
    /// - `SessionError::MaxSessionsReached` if limit is reached
    /// - `SessionError::LaunchFailed` if browser fails to start
    ///
    /// # Example
    /// ```no_run
    /// use robert_app_lib::profiles::browser::session::SessionManager;
    /// use robert_app_lib::profiles::browser::launcher::BrowserConfig;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SessionManager::new();
    /// let session_id = manager.launch_session(BrowserConfig::new()).await?;
    /// println!("Launched session: {}", session_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn launch_session(&self, config: BrowserConfig) -> Result<SessionId> {
        // Check session limit
        {
            let sessions = self.sessions.read().await;
            if sessions.len() >= self.max_sessions {
                return Err(SessionError::MaxSessionsReached(self.max_sessions));
            }
        }

        log::info!("Launching new browser session...");

        // Generate new session ID
        let session_id = SessionId::new();

        // Launch browser with ephemeral profile
        let (driver, profile) = self
            .launcher
            .launch_ephemeral(config.clone())
            .await
            .map_err(|e| {
                log::error!("Failed to launch browser: {}", e);
                SessionError::LaunchFailed(e.to_string())
            })?;

        // Create session
        let session = BrowserSession::new(session_id.clone(), driver, profile, config);

        log::info!(
            "Browser session {} launched successfully with profile: {}",
            session_id,
            session.profile.display_name()
        );

        // Store session
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id.clone(), session);
        }

        Ok(session_id)
    }

    /// Close a browser session and clean up resources
    ///
    /// This method:
    /// 1. Removes the session from the active sessions map
    /// 2. Cleans up the ephemeral profile directory
    /// 3. Drops the ChromeDriver instance (which closes the browser)
    ///
    /// # Parameters
    /// - `id`: The session ID to close
    ///
    /// # Returns
    /// - `Ok(())` if session was closed successfully
    ///
    /// # Errors
    /// - `SessionError::SessionNotFound` if session doesn't exist
    /// - `SessionError::CloseFailed` if cleanup fails
    ///
    /// # Example
    /// ```no_run
    /// use robert_app_lib::profiles::browser::session::SessionManager;
    /// use robert_app_lib::profiles::browser::launcher::BrowserConfig;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SessionManager::new();
    /// let session_id = manager.launch_session(BrowserConfig::new()).await?;
    ///
    /// // ... use session ...
    ///
    /// manager.close_session(&session_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn close_session(&self, id: &SessionId) -> Result<()> {
        log::info!("Closing browser session: {}", id);

        // Remove session from map
        let session = {
            let mut sessions = self.sessions.write().await;
            sessions
                .remove(id)
                .ok_or_else(|| SessionError::SessionNotFound(id.to_string()))?
        };

        // Clean up ephemeral profile
        if let Err(e) = session.profile.cleanup() {
            log::warn!("Failed to cleanup profile for session {}: {}", id, e);
            return Err(SessionError::CloseFailed(format!(
                "Profile cleanup failed: {}",
                e
            )));
        }

        // ChromeDriver will be dropped automatically, closing the browser
        drop(session.driver);

        log::info!("Browser session {} closed successfully", id);

        Ok(())
    }

    /// Close all active sessions
    ///
    /// This is useful for cleanup when the application is shutting down.
    ///
    /// # Returns
    /// - `Ok(count)` with the number of sessions closed
    ///
    /// # Errors
    /// - Returns the first error encountered during cleanup
    ///
    /// # Example
    /// ```no_run
    /// use robert_app_lib::profiles::browser::session::SessionManager;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SessionManager::new();
    /// // ... launch sessions ...
    ///
    /// let count = manager.close_all_sessions().await?;
    /// println!("Closed {} sessions", count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn close_all_sessions(&self) -> Result<usize> {
        log::info!("Closing all browser sessions...");

        let session_ids: Vec<SessionId> = {
            let sessions = self.sessions.read().await;
            sessions.keys().cloned().collect()
        };

        let count = session_ids.len();

        for id in session_ids {
            self.close_session(&id).await?;
        }

        log::info!("Closed {} browser sessions", count);

        Ok(count)
    }

    /// Get information about a specific session
    ///
    /// # Parameters
    /// - `id`: The session ID to query
    ///
    /// # Returns
    /// - `Ok(SessionInfo)` with session metadata
    ///
    /// # Errors
    /// - `SessionError::SessionNotFound` if session doesn't exist
    pub async fn get_session_info(&self, id: &SessionId) -> Result<SessionInfo> {
        let sessions = self.sessions.read().await;
        let session = sessions
            .get(id)
            .ok_or_else(|| SessionError::SessionNotFound(id.to_string()))?;

        Ok(session.info())
    }

    /// List all active sessions
    ///
    /// # Returns
    /// Vector of SessionInfo for all active sessions
    ///
    /// # Example
    /// ```no_run
    /// use robert_app_lib::profiles::browser::session::SessionManager;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SessionManager::new();
    /// let sessions = manager.list_sessions().await;
    /// println!("Active sessions: {}", sessions.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_sessions(&self) -> Vec<SessionInfo> {
        let sessions = self.sessions.read().await;
        sessions.values().map(|s| s.info()).collect()
    }

    /// Check if there are any active sessions
    ///
    /// # Returns
    /// - `true` if there are active sessions
    /// - `false` if no sessions are active
    pub async fn has_active_sessions(&self) -> bool {
        let sessions = self.sessions.read().await;
        !sessions.is_empty()
    }

    /// Get the number of active sessions
    ///
    /// # Returns
    /// Count of active sessions
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }

    /// Get a reference to the ChromeDriver for a session
    ///
    /// This is used internally by Tauri commands to access the driver
    /// for automation operations.
    ///
    /// # Safety
    /// The caller must ensure they do not hold onto this reference across
    /// await points, as it holds a read lock on the sessions map.
    pub async fn get_driver(&self, id: &SessionId) -> Result<Arc<RwLock<ChromeDriver>>> {
        // Note: This is a simplified version for Phase 2
        // In a production implementation, we would return an Arc<RwLock<ChromeDriver>>
        // to allow concurrent access while the session is active

        // For now, we just verify the session exists
        let sessions = self.sessions.read().await;
        sessions
            .get(id)
            .ok_or_else(|| SessionError::SessionNotFound(id.to_string()))?;

        // TODO: Return actual driver reference in Phase 3 when we need automation
        Err(SessionError::SessionNotFound(
            "Driver access not yet implemented".to_string(),
        ))
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_creation() {
        let id1 = SessionId::new();
        let id2 = SessionId::new();

        // IDs should be unique
        assert_ne!(id1, id2);

        // ID should not be empty
        assert!(!id1.as_str().is_empty());
    }

    #[test]
    fn test_session_id_display() {
        let id = SessionId::new();
        let display = format!("{}", id);
        assert_eq!(display, id.as_str());
    }

    #[tokio::test]
    async fn test_session_manager_creation() {
        let manager = SessionManager::new();
        assert_eq!(manager.session_count().await, 0);
        assert!(!manager.has_active_sessions().await);
    }

    #[tokio::test]
    async fn test_list_sessions_empty() {
        let manager = SessionManager::new();
        let sessions = manager.list_sessions().await;
        assert_eq!(sessions.len(), 0);
    }

    // Integration tests that actually launch browsers are in the integration tests directory
}
