use crate::developer_mode::DevTestServer;
use crate::profiles::auth::UserSession;
use crate::profiles::browser::SessionManager;
use robert_webdriver::ChromeDriver;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state that holds the browser driver, developer mode resources, and user session
pub struct AppState {
    pub driver: Arc<Mutex<Option<ChromeDriver>>>,
    pub dev_server: Arc<Mutex<Option<DevTestServer>>>,
    /// Unique session ID for organizing screenshots and other session data
    pub session_id: Arc<Mutex<String>>,
    /// Active user session (username, config, and encryption key)
    /// None if no user is logged in
    pub user_session: Arc<Mutex<Option<UserSession>>>,
    /// Browser session manager for Phase 2 (ephemeral profiles)
    pub session_manager: SessionManager,
}

impl AppState {
    pub fn new() -> Self {
        // Generate a unique session ID using timestamp
        let session_id = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();

        Self {
            driver: Arc::new(Mutex::new(None)),
            dev_server: Arc::new(Mutex::new(None)),
            session_id: Arc::new(Mutex::new(session_id)),
            user_session: Arc::new(Mutex::new(None)),
            session_manager: SessionManager::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
