use robert_webdriver::BrowserDriver;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state that holds the browser driver
pub struct AppState {
    pub driver: Arc<Mutex<Option<BrowserDriver>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            driver: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
