/// User profiles and multi-user support system
///
/// This module implements the complete user profiles feature including:
/// - User authentication and encryption
/// - Browser profile management (ephemeral and named)
/// - Command system with versioning
/// - Generative UI for command parameters
///
/// The system ensures data isolation between users through password-based
/// encryption using Argon2id for key derivation and AES-256-GCM for file encryption.
pub mod auth;
pub mod browser;
pub mod command;
pub mod command_md;
pub mod crypto;
pub mod manager;
pub mod markdown;
pub mod storage;
pub mod types;

// Re-export commonly used types
// Note: Temporarily allow dead_code for types that will be used in Phase 2-5
#[allow(unused_imports)]
pub use types::{
    CommandConfig, SimpleParameter, SimpleParameterType, UserConfig, UserPreferences, UserStats,
};

// Re-export browser types from the browser module (Phase 2)
#[allow(unused_imports)]
pub use browser::{
    BrowserConfig, BrowserLauncher, BrowserProfile, BrowserProfileInfo, BrowserSession,
    LauncherError, ProfileError, SessionError, SessionId, SessionInfo, SessionManager,
};
