use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

/// Debug event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum DebugEvent {
    /// Chrome is being downloaded
    ChromeDownloading { message: String },
    /// Chrome download progress
    ChromeDownloadProgress {
        downloaded: u64,
        total: u64,
        percent: u8,
    },
    /// Chrome download completed
    ChromeDownloaded { path: String, version: String },
    /// Chrome is launching
    ChromeLaunching { message: String },
    /// Chrome successfully launched
    ChromeLaunched { message: String },
    /// Page navigation started
    PageNavigating { url: String },
    /// Page is loading
    PageLoading { message: String },
    /// Page loaded successfully
    PageLoaded { url: String, title: String },
    /// Content is being parsed
    PageParsing { message: String },
    /// General info message
    Info { message: String },
    /// Success message
    Success { message: String },
    /// Error occurred
    Error {
        message: String,
        details: Option<String>,
    },
}

impl DebugEvent {
    /// Get the event name for Tauri emission
    pub fn event_name(&self) -> &'static str {
        "debug-event"
    }

    /// Emit this event to the frontend
    pub fn emit(&self, app: &AppHandle) -> Result<(), String> {
        app.emit(self.event_name(), self)
            .map_err(|e| format!("Failed to emit event: {}", e))
    }
}

/// Helper functions to emit specific events
pub fn emit_info(app: &AppHandle, message: impl Into<String>) -> Result<(), String> {
    DebugEvent::Info {
        message: message.into(),
    }
    .emit(app)
}

pub fn emit_success(app: &AppHandle, message: impl Into<String>) -> Result<(), String> {
    DebugEvent::Success {
        message: message.into(),
    }
    .emit(app)
}

pub fn emit_error(
    app: &AppHandle,
    message: impl Into<String>,
    details: Option<String>,
) -> Result<(), String> {
    DebugEvent::Error {
        message: message.into(),
        details,
    }
    .emit(app)
}

pub fn emit_chrome_downloading(app: &AppHandle, message: impl Into<String>) -> Result<(), String> {
    DebugEvent::ChromeDownloading {
        message: message.into(),
    }
    .emit(app)
}

pub fn emit_chrome_downloaded(
    app: &AppHandle,
    path: impl Into<String>,
    version: impl Into<String>,
) -> Result<(), String> {
    DebugEvent::ChromeDownloaded {
        path: path.into(),
        version: version.into(),
    }
    .emit(app)
}

pub fn emit_chrome_launching(app: &AppHandle, message: impl Into<String>) -> Result<(), String> {
    DebugEvent::ChromeLaunching {
        message: message.into(),
    }
    .emit(app)
}

pub fn emit_chrome_launched(app: &AppHandle, message: impl Into<String>) -> Result<(), String> {
    DebugEvent::ChromeLaunched {
        message: message.into(),
    }
    .emit(app)
}

pub fn emit_page_navigating(app: &AppHandle, url: impl Into<String>) -> Result<(), String> {
    DebugEvent::PageNavigating { url: url.into() }.emit(app)
}

pub fn emit_page_loaded(
    app: &AppHandle,
    url: impl Into<String>,
    title: impl Into<String>,
) -> Result<(), String> {
    DebugEvent::PageLoaded {
        url: url.into(),
        title: title.into(),
    }
    .emit(app)
}
