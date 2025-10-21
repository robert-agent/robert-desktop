/// Browser profile management module (Phase 2)
///
/// This module implements ephemeral browser profile support with the following components:
///
/// - `profile`: Browser profile types (ephemeral and named)
/// - `launcher`: ChromeDriver launcher with profile support
/// - `session`: Session manager for tracking active browser instances
///
/// # Phase 2 Implementation
///
/// For Phase 2, we focus on:
/// - Ephemeral profiles (temporary, auto-deleted)
/// - Single active session at a time
/// - Basic browser launch and cleanup
///
/// Named profiles and multiple simultaneous sessions are deferred to later phases.
///
/// # Example Usage
///
/// ```no_run
/// use robert_app_lib::profiles::browser::{
///     launcher::BrowserConfig,
///     session::SessionManager,
/// };
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create session manager
/// let manager = SessionManager::new();
///
/// // Launch browser with ephemeral profile
/// let session_id = manager
///     .launch_session(BrowserConfig::new())
///     .await?;
///
/// // ... perform automation ...
///
/// // Close session and cleanup
/// manager.close_session(&session_id).await?;
/// # Ok(())
/// # }
/// ```
pub mod launcher;
pub mod profile;
pub mod session;

// Re-export commonly used types
pub use launcher::{BrowserConfig, BrowserLauncher, LauncherError};
pub use profile::{BrowserProfile, BrowserProfileInfo, ProfileError};
pub use session::{BrowserSession, SessionError, SessionId, SessionInfo, SessionManager};
