/// Browser profile types and utilities for Phase 2
///
/// This module defines browser profile types (ephemeral and named) and provides
/// utilities for creating, managing, and cleaning up browser profiles.
///
/// For Phase 2, we focus on:
/// - Ephemeral profiles (temporary, deleted after session)
/// - Basic profile path resolution
/// - Profile metadata for UI display
///
/// Named persistent profiles are deferred to later phases.
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during browser profile operations
#[derive(Error, Debug)]
pub enum ProfileError {
    /// Failed to create temporary directory for ephemeral profile
    #[error("Failed to create ephemeral profile directory: {0}")]
    EphemeralCreationFailed(String),

    /// Failed to clean up ephemeral profile
    #[error("Failed to cleanup ephemeral profile at {path}: {reason}")]
    CleanupFailed { path: String, reason: String },

    /// Invalid profile name format
    #[error("Invalid profile name: {0}")]
    InvalidName(String),

    /// Profile does not exist
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    /// IO error during profile operations
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ProfileError>;

// ============================================================================
// Browser Profile Types
// ============================================================================

/// Browser profile type (ephemeral or persistent)
///
/// This enum distinguishes between temporary browser sessions that are deleted
/// after use and persistent profiles that maintain state across sessions.
///
/// # Phase 2 Implementation
/// For Phase 2, we implement only Ephemeral profiles. Named profiles are
/// defined but their full implementation is deferred to later phases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BrowserProfile {
    /// Temporary browser profile deleted after session ends
    ///
    /// Used for privacy-sensitive tasks or one-off operations.
    /// The profile directory is created in ~/.robert/.tmp/ephemeral-{uuid}/
    /// and automatically cleaned up when the session is closed.
    Ephemeral {
        /// Unique identifier for this ephemeral session
        id: String,
        /// Path to temporary directory
        temp_path: PathBuf,
    },

    /// Persistent browser profile with saved state (Phase 3+)
    ///
    /// Used for workflows requiring logged-in accounts, cookies, etc.
    /// Profile data persists across sessions in ~/.robert/users/{username}/browser-profiles/{name}/
    #[allow(dead_code)]
    Named {
        /// Profile name (e.g., "shopping", "work")
        name: String,
        /// Path to Chromium user-data-dir
        path: PathBuf,
    },
}

impl BrowserProfile {
    /// Create a new ephemeral profile with a unique temporary directory
    ///
    /// The directory is created at ~/.robert/.tmp/ephemeral-{uuid}/
    /// This directory will need to be manually cleaned up when the session ends.
    ///
    /// # Returns
    /// - `Ok(BrowserProfile::Ephemeral)` with the created profile
    ///
    /// # Errors
    /// - `ProfileError::EphemeralCreationFailed` if directory creation fails
    ///
    /// # Example
    /// ```no_run
    /// use robert_app_lib::profiles::browser::profile::BrowserProfile;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let profile = BrowserProfile::create_ephemeral()?;
    /// println!("Created ephemeral profile at: {}", profile.path().display());
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_ephemeral() -> Result<Self> {
        // Generate unique ID for this ephemeral session
        let id = Uuid::new_v4().to_string();

        // Get base directory: ~/.robert/.tmp/
        let base_dir = dirs::home_dir()
            .ok_or_else(|| {
                ProfileError::EphemeralCreationFailed("Could not determine home directory".into())
            })?
            .join(".robert")
            .join(".tmp");

        // Create base temp directory if it doesn't exist
        std::fs::create_dir_all(&base_dir).map_err(|e| {
            ProfileError::EphemeralCreationFailed(format!(
                "Failed to create temp base directory: {}",
                e
            ))
        })?;

        // Create unique ephemeral profile directory
        let temp_path = base_dir.join(format!("ephemeral-{}", id));
        std::fs::create_dir_all(&temp_path).map_err(|e| {
            ProfileError::EphemeralCreationFailed(format!(
                "Failed to create ephemeral directory {}: {}",
                temp_path.display(),
                e
            ))
        })?;

        log::info!(
            "Created ephemeral profile with id {} at {}",
            id,
            temp_path.display()
        );

        Ok(BrowserProfile::Ephemeral { id, temp_path })
    }

    /// Get the filesystem path to the browser profile directory
    ///
    /// This path is used as the Chrome --user-data-dir argument.
    ///
    /// # Returns
    /// Reference to the PathBuf containing the profile directory path
    pub fn path(&self) -> &Path {
        match self {
            BrowserProfile::Ephemeral { temp_path, .. } => temp_path,
            BrowserProfile::Named { path, .. } => path,
        }
    }

    /// Check if this is an ephemeral profile
    ///
    /// # Returns
    /// - `true` if this is an ephemeral profile
    /// - `false` if this is a named profile
    pub fn is_ephemeral(&self) -> bool {
        matches!(self, BrowserProfile::Ephemeral { .. })
    }

    /// Get a human-readable display name for this profile
    ///
    /// # Returns
    /// String representation suitable for UI display
    ///
    /// # Examples
    /// - Ephemeral profile: "Ephemeral (Clean Session)"
    /// - Named profile: the profile name (e.g., "shopping")
    pub fn display_name(&self) -> String {
        match self {
            BrowserProfile::Ephemeral { id, .. } => {
                format!("Ephemeral (Clean Session) [{}]", &id[..8])
            }
            BrowserProfile::Named { name, .. } => name.clone(),
        }
    }

    /// Get the profile ID if this is an ephemeral profile
    ///
    /// # Returns
    /// - `Some(id)` for ephemeral profiles
    /// - `None` for named profiles
    pub fn id(&self) -> Option<&str> {
        match self {
            BrowserProfile::Ephemeral { id, .. } => Some(id),
            BrowserProfile::Named { .. } => None,
        }
    }

    /// Clean up the profile directory if this is an ephemeral profile
    ///
    /// For ephemeral profiles, this deletes the temporary directory and all its contents.
    /// For named profiles, this is a no-op as those profiles should persist.
    ///
    /// # Returns
    /// - `Ok(())` if cleanup succeeded or profile is named
    ///
    /// # Errors
    /// - `ProfileError::CleanupFailed` if directory removal fails
    ///
    /// # Example
    /// ```no_run
    /// use robert_app_lib::profiles::browser::profile::BrowserProfile;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let profile = BrowserProfile::create_ephemeral()?;
    /// // ... use profile ...
    /// profile.cleanup()?; // Clean up when done
    /// # Ok(())
    /// # }
    /// ```
    pub fn cleanup(&self) -> Result<()> {
        match self {
            BrowserProfile::Ephemeral { id, temp_path } => {
                if temp_path.exists() {
                    log::info!(
                        "Cleaning up ephemeral profile {} at {}",
                        id,
                        temp_path.display()
                    );

                    std::fs::remove_dir_all(temp_path).map_err(|e| {
                        ProfileError::CleanupFailed {
                            path: temp_path.display().to_string(),
                            reason: e.to_string(),
                        }
                    })?;

                    log::info!("Successfully cleaned up ephemeral profile {}", id);
                } else {
                    log::warn!(
                        "Ephemeral profile {} directory not found at {} (may have been cleaned up already)",
                        id,
                        temp_path.display()
                    );
                }
                Ok(())
            }
            BrowserProfile::Named { .. } => {
                // Named profiles persist, no cleanup needed
                Ok(())
            }
        }
    }
}

// ============================================================================
// Profile Metadata for UI Display
// ============================================================================

/// Browser profile metadata for UI display
///
/// This structure contains information about a profile that can be
/// serialized and sent to the frontend for display purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfileInfo {
    /// Profile type ("ephemeral" or "named")
    pub profile_type: String,

    /// Profile identifier (UUID for ephemeral, name for named)
    pub id: String,

    /// Display name for UI
    pub display_name: String,

    /// Filesystem path
    pub path: String,
}

impl From<&BrowserProfile> for BrowserProfileInfo {
    fn from(profile: &BrowserProfile) -> Self {
        match profile {
            BrowserProfile::Ephemeral { id, temp_path } => BrowserProfileInfo {
                profile_type: "ephemeral".to_string(),
                id: id.clone(),
                display_name: format!("Ephemeral (Clean Session) [{}]", &id[..8]),
                path: temp_path.display().to_string(),
            },
            BrowserProfile::Named { name, path } => BrowserProfileInfo {
                profile_type: "named".to_string(),
                id: name.clone(),
                display_name: name.clone(),
                path: path.display().to_string(),
            },
        }
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Clean up orphaned ephemeral profiles on application start
///
/// This function scans ~/.robert/.tmp/ for ephemeral profile directories
/// and removes any that are not associated with active sessions.
///
/// This is useful for cleaning up profiles that may have been left behind
/// if the application crashed or was force-quit.
///
/// # Returns
/// - `Ok(count)` with the number of profiles cleaned up
///
/// # Errors
/// - Returns error if directory scanning or removal fails
///
/// # Example
/// ```no_run
/// use robert_app_lib::profiles::browser::profile::cleanup_orphaned_profiles;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let count = cleanup_orphaned_profiles()?;
/// println!("Cleaned up {} orphaned ephemeral profiles", count);
/// # Ok(())
/// # }
/// ```
pub fn cleanup_orphaned_profiles() -> Result<usize> {
    let base_dir = dirs::home_dir()
        .ok_or_else(|| {
            ProfileError::EphemeralCreationFailed("Could not determine home directory".into())
        })?
        .join(".robert")
        .join(".tmp");

    // If temp directory doesn't exist, nothing to clean up
    if !base_dir.exists() {
        return Ok(0);
    }

    let mut count = 0;

    // Iterate through all ephemeral-* directories
    for entry in std::fs::read_dir(&base_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only process directories with ephemeral- prefix
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("ephemeral-") {
                    log::info!("Cleaning up orphaned profile at {}", path.display());

                    if let Err(e) = std::fs::remove_dir_all(&path) {
                        log::warn!(
                            "Failed to remove orphaned profile at {}: {}",
                            path.display(),
                            e
                        );
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }

    if count > 0 {
        log::info!("Cleaned up {} orphaned ephemeral profiles", count);
    }

    Ok(count)
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ephemeral_profile() {
        let profile = BrowserProfile::create_ephemeral().expect("Failed to create profile");

        // Should be ephemeral type
        assert!(profile.is_ephemeral());

        // Path should exist
        assert!(profile.path().exists());

        // Should have a valid ID
        assert!(profile.id().is_some());

        // Display name should contain "Ephemeral"
        assert!(profile.display_name().contains("Ephemeral"));

        // Cleanup should succeed
        profile.cleanup().expect("Failed to cleanup profile");

        // Path should no longer exist after cleanup
        assert!(!profile.path().exists());
    }

    #[test]
    fn test_profile_path() {
        let profile = BrowserProfile::create_ephemeral().unwrap();
        let path = profile.path();

        // Path should contain .robert/.tmp/ephemeral-
        assert!(path.to_string_lossy().contains(".robert"));
        assert!(path.to_string_lossy().contains(".tmp"));
        assert!(path.to_string_lossy().contains("ephemeral-"));

        profile.cleanup().unwrap();
    }

    #[test]
    fn test_profile_display_name() {
        let profile = BrowserProfile::create_ephemeral().unwrap();
        let display_name = profile.display_name();

        // Should contain "Ephemeral"
        assert!(display_name.contains("Ephemeral"));
        // Should contain "Clean Session"
        assert!(display_name.contains("Clean Session"));

        profile.cleanup().unwrap();
    }

    #[test]
    fn test_profile_info_from_profile() {
        let profile = BrowserProfile::create_ephemeral().unwrap();
        let info = BrowserProfileInfo::from(&profile);

        assert_eq!(info.profile_type, "ephemeral");
        assert!(info.display_name.contains("Ephemeral"));
        assert!(info.path.contains("ephemeral-"));

        profile.cleanup().unwrap();
    }

    #[test]
    fn test_cleanup_idempotent() {
        let profile = BrowserProfile::create_ephemeral().unwrap();

        // First cleanup should succeed
        profile.cleanup().expect("First cleanup failed");

        // Second cleanup should also succeed (idempotent)
        profile.cleanup().expect("Second cleanup failed");
    }

    #[test]
    fn test_cleanup_orphaned_profiles() {
        // Create a few ephemeral profiles
        let profile1 = BrowserProfile::create_ephemeral().unwrap();
        let profile2 = BrowserProfile::create_ephemeral().unwrap();

        // Cleanup orphaned profiles (should find our test profiles)
        let count = cleanup_orphaned_profiles().expect("Failed to cleanup orphaned profiles");

        // Should have cleaned up at least our 2 profiles
        assert!(count >= 2, "Expected at least 2 profiles cleaned up");

        // Profiles should no longer exist
        assert!(!profile1.path().exists());
        assert!(!profile2.path().exists());
    }
}
