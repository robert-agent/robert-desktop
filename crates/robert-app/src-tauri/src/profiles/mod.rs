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
pub mod crypto;
pub mod manager;
pub mod storage;
pub mod types;

// Re-export commonly used types
pub use types::{
    BrowserProfile, BrowserProfileInfo, UserConfig, UserPreferences, UserStats,
};
