//! Authentication service for user login/logout
//!
//! This module handles user authentication and session management:
//! - User login with password validation
//! - User logout with session cleanup
//! - Password verification against stored credentials
//! - Active session tracking

use crate::profiles::{
    crypto::{derive_key, EncryptionKey},
    manager::UserManager,
    storage::{load_salt, load_user_config, user_exists},
    types::UserConfig,
};
use std::sync::{Arc, Mutex};
use thiserror::Error;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Error, Debug)]
pub enum AuthError {
    /// User not found
    #[error("User not found: {0}")]
    UserNotFound(String),

    /// Invalid password
    #[error("Invalid password")]
    InvalidPassword,

    /// No active session
    #[error("No active session")]
    #[allow(dead_code)]
    NoActiveSession,

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(#[from] crate::profiles::storage::StorageError),

    /// Crypto error
    #[error("Crypto error: {0}")]
    CryptoError(#[from] crate::profiles::crypto::CryptoError),

    /// Manager error
    #[error("Manager error: {0}")]
    ManagerError(#[from] crate::profiles::manager::ManagerError),
}

pub type Result<T> = std::result::Result<T, AuthError>;

// ============================================================================
// Session Data
// ============================================================================

/// Active user session data
///
/// This structure holds the current user's session information including
/// their username, configuration, and encryption key.
#[derive(Clone)]
pub struct UserSession {
    /// Username of the active user
    pub username: String,

    /// User configuration
    pub config: UserConfig,

    /// Encryption key for accessing user's encrypted files
    /// Wrapped in Arc<Mutex<>> for thread-safe access
    pub encryption_key: Arc<Mutex<EncryptionKey>>,
}

impl UserSession {
    /// Create a new user session
    pub fn new(username: String, config: UserConfig, encryption_key: EncryptionKey) -> Self {
        Self {
            username,
            config,
            encryption_key: Arc::new(Mutex::new(encryption_key)),
        }
    }

    /// Get a clone of the encryption key
    pub fn get_encryption_key(&self) -> EncryptionKey {
        self.encryption_key.lock().unwrap().clone()
    }
}

// ============================================================================
// Authentication Service
// ============================================================================

/// Authentication service for user login/logout operations
pub struct AuthService;

impl AuthService {
    /// Login a user with username and password
    ///
    /// # Parameters
    /// - `username`: The username to login
    /// - `password`: The user's password
    ///
    /// # Returns
    /// - `UserSession`: Active session data including encryption key
    ///
    /// # Errors
    /// - Returns `UserNotFound` if user doesn't exist
    /// - Returns `InvalidPassword` if password is incorrect
    /// - Returns `StorageError` if file operations fail
    /// - Returns `CryptoError` if decryption fails
    ///
    /// # Example
    /// ```no_run
    /// use crate::profiles::auth::AuthService;
    ///
    /// let session = AuthService::login("alice", "password123")?;
    /// println!("Logged in as: {}", session.username);
    /// ```
    pub fn login(username: &str, password: &str) -> Result<UserSession> {
        log::info!("🔐 Login attempt for user: {}", username);

        // Check if user exists
        if !user_exists(username)? {
            log::warn!("❌ Login failed: User '{}' not found", username);
            return Err(AuthError::UserNotFound(username.to_string()));
        }

        // Load the salt
        let salt = load_salt(username)?;

        // Derive encryption key from password and salt
        let (encryption_key, _) = derive_key(password, Some(&salt))?;

        // Try to load user config with the derived key
        // If this succeeds, the password was correct
        match load_user_config(username, &encryption_key) {
            Ok(config) => {
                log::info!("✅ Login successful for user: {}", username);

                // Update last login timestamp
                let mut updated_config = config.clone();
                if let Err(e) = UserManager::update_last_login(username, &mut updated_config, &encryption_key) {
                    log::warn!("⚠️  Failed to update last login timestamp: {}", e);
                    // Don't fail login for this, use original config
                } else {
                    // Use updated config with new timestamp
                    return Ok(UserSession::new(
                        username.to_string(),
                        updated_config,
                        encryption_key,
                    ));
                }

                Ok(UserSession::new(
                    username.to_string(),
                    config,
                    encryption_key,
                ))
            }
            Err(_) => {
                log::warn!("❌ Login failed: Invalid password for user '{}'", username);
                Err(AuthError::InvalidPassword)
            }
        }
    }

    /// Create a new user and return an active session
    ///
    /// # Parameters
    /// - `username`: Unique username for the new user
    /// - `password`: Password for the new user
    ///
    /// # Returns
    /// - `UserSession`: Active session for the newly created user
    ///
    /// # Errors
    /// - Returns `ManagerError` if user creation fails
    ///
    /// # Example
    /// ```no_run
    /// use crate::profiles::auth::AuthService;
    ///
    /// let session = AuthService::create_and_login("alice", "secure_password")?;
    /// println!("Created and logged in as: {}", session.username);
    /// ```
    pub fn create_and_login(username: &str, password: &str) -> Result<UserSession> {
        log::info!("👤 Creating new user: {}", username);

        // Create the user (this validates username and password)
        let (encryption_key, config) = UserManager::create_user(username, password)?;

        log::info!("✅ User '{}' created successfully", username);

        Ok(UserSession::new(
            username.to_string(),
            config,
            encryption_key,
        ))
    }

    /// Verify if a password is correct for a user (without logging in)
    ///
    /// # Parameters
    /// - `username`: The username to verify
    /// - `password`: The password to check
    ///
    /// # Returns
    /// - `true` if password is correct
    /// - `false` if password is incorrect
    ///
    /// # Errors
    /// - Returns error if user doesn't exist or storage fails
    #[allow(dead_code)]
    pub fn verify_password(username: &str, password: &str) -> Result<bool> {
        if !user_exists(username)? {
            return Err(AuthError::UserNotFound(username.to_string()));
        }

        let salt = load_salt(username)?;
        let (encryption_key, _) = derive_key(password, Some(&salt))?;

        // Try to load config - if it succeeds, password is correct
        match load_user_config(username, &encryption_key) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::storage::{create_user_directory, save_salt, save_user_config};
    use tempfile::TempDir;

    fn setup_test_user(username: &str, password: &str) -> (TempDir, EncryptionKey) {
        let temp_dir = TempDir::new().unwrap();
        // Use a unique temp directory per test
        let home = temp_dir.path().to_path_buf();
        std::env::set_var("HOME", &home);

        let (key, salt) = derive_key(password, None).unwrap();

        // Create .robert directory first
        std::fs::create_dir_all(home.join(".robert/users")).unwrap();

        // Create user directory structure
        create_user_directory(username).unwrap();
        save_salt(username, &salt).unwrap();

        // Create a test config
        let config = UserConfig {
            username: username.to_string(),
            created_at: chrono::Utc::now(),
            last_login: chrono::Utc::now(),
            browser_profiles: std::collections::HashMap::new(),
            default_browser_profile: None,
            preferences: crate::profiles::types::UserPreferences {
                theme: crate::profiles::types::Theme::Dark,
                default_timeout_ms: 5000,
                inference_mode: crate::profiles::types::InferenceMode::Local,
                language: "en".to_string(),
            },
            stats: crate::profiles::types::UserStats {
                total_commands_run: 0,
                total_sessions: 0,
                commands_created: 0,
            },
        };

        save_user_config(username, &config, &key).unwrap();

        (temp_dir, key)
    }

    #[test]
    fn test_login_success() {
        let (_temp, _key) = setup_test_user("test_user_1", "password123");

        let session = AuthService::login("test_user_1", "password123").unwrap();
        assert_eq!(session.username, "test_user_1");
        assert_eq!(session.config.username, "test_user_1");
    }

    #[test]
    fn test_login_wrong_password() {
        let (_temp, _key) = setup_test_user("test_user_2", "password123");

        let result = AuthService::login("test_user_2", "wrongpassword");
        assert!(matches!(result, Err(AuthError::InvalidPassword)));
    }

    #[test]
    fn test_login_user_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let home = temp_dir.path().to_path_buf();
        std::env::set_var("HOME", &home);
        std::fs::create_dir_all(home.join(".robert/users")).unwrap();

        let result = AuthService::login("nonexistent", "password");
        assert!(matches!(
            result,
            Err(AuthError::UserNotFound(username)) if username == "nonexistent"
        ));
    }

    #[test]
    fn test_verify_password_correct() {
        let (_temp, _key) = setup_test_user("test_user_3", "password123");

        let is_correct = AuthService::verify_password("test_user_3", "password123").unwrap();
        assert!(is_correct);
    }

    #[test]
    fn test_verify_password_incorrect() {
        let (_temp, _key) = setup_test_user("test_user_4", "password123");

        let is_correct = AuthService::verify_password("test_user_4", "wrongpassword").unwrap();
        assert!(!is_correct);
    }
}
