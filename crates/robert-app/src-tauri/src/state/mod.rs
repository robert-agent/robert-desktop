use crate::developer_mode::DevTestServer;
use robert_webdriver::ChromeDriver;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state that holds the browser driver and developer mode resources
pub struct AppState {
    pub driver: Arc<Mutex<Option<ChromeDriver>>>,
    pub dev_server: Arc<Mutex<Option<DevTestServer>>>,
    pub chat_poll_shutdown: Arc<Mutex<Option<tokio::sync::oneshot::Sender<()>>>>,
    /// Unique session ID for organizing screenshots and other session data
    pub session_id: Arc<Mutex<String>>,
}

impl AppState {
    pub fn new() -> Self {
        // Generate a unique session ID using timestamp
        let session_id = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();

        Self {
            driver: Arc::new(Mutex::new(None)),
            dev_server: Arc::new(Mutex::new(None)),
            chat_poll_shutdown: Arc::new(Mutex::new(None)),
            session_id: Arc::new(Mutex::new(session_id)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
