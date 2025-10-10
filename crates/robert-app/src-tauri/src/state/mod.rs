use crate::developer_mode::DevTestServer;
use robert_webdriver::ChromeDriver;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state that holds the browser driver and developer mode resources
pub struct AppState {
    pub driver: Arc<Mutex<Option<ChromeDriver>>>,
    pub dev_server: Arc<Mutex<Option<DevTestServer>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            driver: Arc::new(Mutex::new(None)),
            dev_server: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
