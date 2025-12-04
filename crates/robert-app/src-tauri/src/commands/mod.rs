mod agent;

mod developer_mode;
mod logging;
mod profiles;

pub use agent::*;
// Note: browser module is pub mod so we can selectively export commands to avoid conflicts
pub use developer_mode::*;
pub use logging::*;
pub use profiles::*;

use crate::claude::{ClaudeClient, ClaudeConfig, ClaudeHealthCheck, ClaudeInput, ClaudeResponse};
use crate::events::*;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigationResult {
    pub success: bool,
    pub url: String,
    pub title: String,
    pub message: String,
}

// Browser automation commands removed as robert-webdriver is deprecated.
// Future implementation will use a different approach if needed.
