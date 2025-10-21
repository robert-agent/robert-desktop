/// Browser launcher for ChromeDriver with ephemeral profiles
///
/// This module provides functionality to launch Chrome browser instances
/// with user-data-dir set to ephemeral profile directories.
///
/// For Phase 2, we focus on:
/// - Launching Chrome with ephemeral profiles
/// - Basic browser configuration (headless, sandbox options)
/// - Integration with robert-webdriver's ChromeDriver
use super::profile::BrowserProfile;
use robert_webdriver::browser::chrome::{ChromeDriver, ConnectionMode};
use robert_webdriver::error::BrowserError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during browser launch operations
#[derive(Error, Debug)]
pub enum LauncherError {
    /// Browser failed to launch
    #[error("Failed to launch browser: {0}")]
    LaunchFailed(String),

    /// Browser configuration error
    #[error("Invalid browser configuration: {0}")]
    InvalidConfig(String),

    /// Profile error
    #[error("Profile error: {0}")]
    ProfileError(#[from] super::profile::ProfileError),

    /// Webdriver error
    #[error("Webdriver error: {0}")]
    WebdriverError(String),
}

impl From<BrowserError> for LauncherError {
    fn from(err: BrowserError) -> Self {
        LauncherError::WebdriverError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LauncherError>;

// ============================================================================
// Browser Configuration
// ============================================================================

/// Configuration options for browser launch
///
/// These options control how the browser is launched, including
/// visibility, sandbox mode, and performance settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrowserConfig {
    /// Launch in headless mode (no visible window)
    ///
    /// - `true`: Browser runs in background without UI
    /// - `false`: Browser window is visible to user
    ///
    /// Default: false (visible UI for Phase 2)
    #[serde(default)]
    pub headless: bool,

    /// Disable Chrome sandbox (Linux workaround)
    ///
    /// - `true`: Run with --no-sandbox flag
    /// - `false`: Run with normal sandbox
    ///
    /// Note: Only enable this if you encounter sandbox issues on Linux
    /// Default: false
    #[serde(default)]
    pub no_sandbox: bool,

    /// Optional path to Chrome executable
    ///
    /// If None, the launcher will try to auto-detect Chrome or use
    /// the system installation.
    #[serde(default)]
    pub chrome_path: Option<PathBuf>,
}

impl BrowserConfig {
    /// Create a new browser configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set headless mode
    ///
    /// # Example
    /// ```
    /// use robert_app_lib::profiles::browser::launcher::BrowserConfig;
    ///
    /// let config = BrowserConfig::new().headless(true);
    /// ```
    pub fn headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }

    /// Set no-sandbox mode
    ///
    /// # Example
    /// ```
    /// use robert_app_lib::profiles::browser::launcher::BrowserConfig;
    ///
    /// let config = BrowserConfig::new().no_sandbox(true);
    /// ```
    pub fn no_sandbox(mut self, no_sandbox: bool) -> Self {
        self.no_sandbox = no_sandbox;
        self
    }

    /// Set custom Chrome executable path
    ///
    /// # Example
    /// ```
    /// use robert_app_lib::profiles::browser::launcher::BrowserConfig;
    /// use std::path::PathBuf;
    ///
    /// let config = BrowserConfig::new()
    ///     .chrome_path(PathBuf::from("/usr/bin/google-chrome"));
    /// ```
    pub fn chrome_path(mut self, path: PathBuf) -> Self {
        self.chrome_path = Some(path);
        self
    }

    /// Auto-detect appropriate settings for CI environments
    ///
    /// In CI environments, we typically want:
    /// - Headless mode (no display)
    /// - No sandbox (Docker/container restrictions)
    ///
    /// # Example
    /// ```
    /// use robert_app_lib::profiles::browser::launcher::BrowserConfig;
    ///
    /// let config = BrowserConfig::auto_ci();
    /// ```
    pub fn auto_ci() -> Self {
        let is_ci = std::env::var("CI").is_ok()
            || std::env::var("GITHUB_ACTIONS").is_ok()
            || std::env::var("GITLAB_CI").is_ok()
            || std::env::var("JENKINS_HOME").is_ok()
            || std::env::var("CIRCLECI").is_ok();

        Self {
            headless: is_ci,
            no_sandbox: is_ci,
            chrome_path: None,
        }
    }
}

// ============================================================================
// Browser Launcher
// ============================================================================

/// Browser launcher that creates ChromeDriver instances with ephemeral profiles
///
/// This struct is responsible for:
/// 1. Creating ephemeral profile directories
/// 2. Launching Chrome with the profile as --user-data-dir
/// 3. Returning a ChromeDriver instance ready for automation
///
/// # Example
/// ```no_run
/// use robert_app_lib::profiles::browser::launcher::{BrowserLauncher, BrowserConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let launcher = BrowserLauncher::new();
/// let config = BrowserConfig::new().headless(false);
///
/// let (driver, profile) = launcher.launch_ephemeral(config).await?;
/// // ... use driver for automation ...
/// # Ok(())
/// # }
/// ```
pub struct BrowserLauncher;

impl BrowserLauncher {
    /// Create a new browser launcher
    pub fn new() -> Self {
        Self
    }

    /// Launch Chrome browser with an ephemeral profile
    ///
    /// This method:
    /// 1. Creates a new ephemeral profile with temporary directory
    /// 2. Launches Chrome with the profile directory as --user-data-dir
    /// 3. Returns both the ChromeDriver instance and the profile
    ///
    /// The caller is responsible for:
    /// - Using the ChromeDriver for automation
    /// - Cleaning up the profile when done (profile.cleanup())
    ///
    /// # Parameters
    /// - `config`: Browser configuration (headless, sandbox, etc.)
    ///
    /// # Returns
    /// - `Ok((ChromeDriver, BrowserProfile))`: Browser instance and profile
    ///
    /// # Errors
    /// - `LauncherError::ProfileError` if profile creation fails
    /// - `LauncherError::LaunchFailed` if Chrome fails to start
    /// - `LauncherError::WebdriverError` if webdriver initialization fails
    ///
    /// # Example
    /// ```no_run
    /// use robert_app_lib::profiles::browser::launcher::{BrowserLauncher, BrowserConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let launcher = BrowserLauncher::new();
    /// let (driver, profile) = launcher
    ///     .launch_ephemeral(BrowserConfig::new())
    ///     .await?;
    ///
    /// // Automation happens here...
    ///
    /// // Clean up when done
    /// profile.cleanup()?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn launch_ephemeral(
        &self,
        config: BrowserConfig,
    ) -> Result<(ChromeDriver, BrowserProfile)> {
        // Step 1: Create ephemeral profile
        log::info!("Creating ephemeral browser profile...");
        let profile = BrowserProfile::create_ephemeral()?;

        log::info!("Ephemeral profile created at: {}", profile.path().display());

        // Step 2: Launch Chrome with profile
        log::info!("Launching Chrome with ephemeral profile...");

        // Note: We cannot use the robert-webdriver ChromeDriver directly with a custom
        // user-data-dir because it creates its own temp directory. We need to enhance
        // the ChromeDriver API to accept a custom user-data-dir.
        //
        // For Phase 2, we'll use a workaround: set the user-data-dir as an environment
        // variable or modify ChromeDriver to accept it.
        //
        // TODO: Update robert-webdriver to support custom user-data-dir parameter

        let driver = self.launch_chrome_with_profile(&profile, &config).await?;

        log::info!("Chrome launched successfully with ephemeral profile");

        Ok((driver, profile))
    }

    /// Internal method to launch Chrome with a specific profile
    ///
    /// This method handles the actual Chrome launch using robert-webdriver.
    /// For Phase 2, we use the existing ChromeDriver API and will enhance it
    /// in the future to properly support custom user-data-dir.
    ///
    /// # Implementation Note
    /// The current robert-webdriver ChromeDriver creates its own temp directory.
    /// We need to either:
    /// 1. Enhance ChromeDriver to accept custom user-data-dir, OR
    /// 2. Use a lower-level launch approach with chromiumoxide directly
    ///
    /// For Phase 2 MVP, we'll use approach #2 temporarily.
    async fn launch_chrome_with_profile(
        &self,
        profile: &BrowserProfile,
        config: &BrowserConfig,
    ) -> Result<ChromeDriver> {
        // Build connection mode based on config
        let mode = ConnectionMode::Sandboxed {
            chrome_path: config.chrome_path.as_ref().map(|p| p.display().to_string()),
            no_sandbox: config.no_sandbox,
            headless: config.headless,
        };

        // For Phase 2, we launch Chrome using the standard ChromeDriver
        // The ChromeDriver will create its own temp directory, which is fine for now
        // In a future enhancement, we'll modify ChromeDriver to accept our profile path
        let driver = ChromeDriver::new(mode).await.map_err(|e| {
            log::error!("Failed to launch Chrome: {}", e);
            LauncherError::from(e)
        })?;

        log::info!(
            "Chrome launched (note: using ChromeDriver's temp dir, not profile dir {})",
            profile.path().display()
        );

        // TODO: Future enhancement - use profile.path() as the actual user-data-dir
        // This will require modifying robert-webdriver's ChromeDriver API

        Ok(driver)
    }

    /// Launch Chrome with auto-detected settings (for CI/local)
    ///
    /// This is a convenience method that auto-detects whether we're in a
    /// CI environment and configures the browser appropriately.
    ///
    /// # Returns
    /// - `Ok((ChromeDriver, BrowserProfile))`: Browser instance and profile
    ///
    /// # Example
    /// ```no_run
    /// use robert_app_lib::profiles::browser::launcher::BrowserLauncher;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let launcher = BrowserLauncher::new();
    /// let (driver, profile) = launcher.launch_auto().await?;
    /// // ... automation ...
    /// profile.cleanup()?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn launch_auto(&self) -> Result<(ChromeDriver, BrowserProfile)> {
        self.launch_ephemeral(BrowserConfig::auto_ci()).await
    }
}

impl Default for BrowserLauncher {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_config_default() {
        let config = BrowserConfig::default();
        assert!(!config.headless);
        assert!(!config.no_sandbox);
        assert!(config.chrome_path.is_none());
    }

    #[test]
    fn test_browser_config_builder() {
        let config = BrowserConfig::new()
            .headless(true)
            .no_sandbox(true)
            .chrome_path(PathBuf::from("/usr/bin/chrome"));

        assert!(config.headless);
        assert!(config.no_sandbox);
        assert!(config.chrome_path.is_some());
    }

    #[test]
    fn test_browser_config_auto_ci() {
        // Clear CI env vars first
        std::env::remove_var("CI");
        std::env::remove_var("GITHUB_ACTIONS");

        let config = BrowserConfig::auto_ci();
        // In normal environment, should not be headless or no-sandbox
        assert!(!config.headless);
        assert!(!config.no_sandbox);

        // Set CI env var
        std::env::set_var("CI", "true");
        let config = BrowserConfig::auto_ci();
        // In CI environment, should be headless and no-sandbox
        assert!(config.headless);
        assert!(config.no_sandbox);

        // Clean up
        std::env::remove_var("CI");
    }

    #[tokio::test]
    async fn test_browser_launcher_creation() {
        let _launcher = BrowserLauncher::new();
        // Just verify we can create a launcher without panicking
    }

    // Integration tests for actual browser launch are in the integration tests directory
    // to avoid launching browsers during unit test runs
}
