// spider_chrome re-exports chromiumoxide API
use crate::error::{BrowserError, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide_fetcher::{BrowserFetcher, BrowserFetcherOptions};
use futures::StreamExt;
use std::path::{Path, PathBuf};

pub struct ChromeDriver {
    browser: Browser,
    temp_dir: Option<PathBuf>,
}

/// Connection mode for Chrome browser
pub enum ConnectionMode {
    /// Sandboxed mode - launches Chrome using system installation
    Sandboxed {
        chrome_path: Option<String>,
        no_sandbox: bool,
        headless: bool,
    },
    /// Advanced mode - connects to existing Chrome on debug port
    DebugPort(u16),
}

impl ChromeDriver {
    /// Launch Chrome in sandboxed mode (uses system Chrome)
    pub async fn launch_sandboxed() -> Result<Self> {
        Self::new(ConnectionMode::Sandboxed {
            chrome_path: None,
            no_sandbox: false,
            headless: false,
        })
        .await
    }

    /// Launch Chrome in sandboxed mode with custom path
    pub async fn launch_with_path(
        chrome_path: String,
        no_sandbox: bool,
        headless: bool,
    ) -> Result<Self> {
        Self::new(ConnectionMode::Sandboxed {
            chrome_path: Some(chrome_path),
            no_sandbox,
            headless,
        })
        .await
    }

    /// Launch Chrome with no-sandbox flag (Linux workaround for AppArmor restrictions)
    pub async fn launch_no_sandbox() -> Result<Self> {
        Self::new(ConnectionMode::Sandboxed {
            chrome_path: None,
            no_sandbox: true,
            headless: false,
        })
        .await
    }

    /// Launch Chrome with auto-detection for CI environments
    pub async fn launch_auto() -> Result<Self> {
        let is_ci = std::env::var("CI").is_ok()
            || std::env::var("GITHUB_ACTIONS").is_ok()
            || std::env::var("GITLAB_CI").is_ok()
            || std::env::var("JENKINS_HOME").is_ok()
            || std::env::var("CIRCLECI").is_ok();

        Self::new(ConnectionMode::Sandboxed {
            chrome_path: None,
            no_sandbox: is_ci, // CI environments typically need --no-sandbox
            headless: is_ci,   // CI environments should run headless
        })
        .await
    }

    /// Connect to existing Chrome on debug port (advanced mode)
    pub async fn connect_debug_port(port: u16) -> Result<Self> {
        Self::new(ConnectionMode::DebugPort(port)).await
    }

    /// Create new ChromeDriver with specified connection mode
    pub async fn new(mode: ConnectionMode) -> Result<Self> {
        let (browser, temp_dir) = match mode {
            ConnectionMode::Sandboxed {
                chrome_path,
                no_sandbox,
                headless,
            } => {
                // Create a unique temporary directory for this browser instance
                // This ensures parallel tests don't share profile data
                // Using timestamp in nanoseconds ensures uniqueness across threads
                let unique_id = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                let temp_dir = std::env::temp_dir().join(format!("chromiumoxide-{}", unique_id));
                std::fs::create_dir_all(&temp_dir).map_err(|e| {
                    BrowserError::LaunchFailed(format!("Failed to create temp directory: {}", e))
                })?;

                // Launch Chrome with visible UI or headless
                let mut config = if headless {
                    BrowserConfig::builder()
                } else {
                    BrowserConfig::builder().with_head()
                };

                // Set unique user data directory for test isolation
                config = config.user_data_dir(&temp_dir);

                // Add no-sandbox flag if requested (Linux AppArmor workaround)
                if no_sandbox {
                    config = config.arg("--no-sandbox");
                }

                // Use custom Chrome path if provided, otherwise try auto-download
                if let Some(path) = chrome_path {
                    config = config.chrome_executable(path);
                } else {
                    // Try to auto-download Chrome if not found
                    match Self::ensure_chrome_installed().await {
                        Ok(path) => {
                            config = config.chrome_executable(path);
                        }
                        Err(e) => {
                            // If auto-download fails, let chromiumoxide try to find system Chrome
                            eprintln!(
                                "Note: Auto-download failed ({}), trying system Chrome...",
                                e
                            );
                        }
                    }
                }

                let (browser, mut handler) = Browser::launch(config.build().map_err(|e| {
                    BrowserError::LaunchFailed(format!(
                        "{}. \n\n\
                                 Chrome not found. You can:\n\
                                 - Install Chrome: https://www.google.com/chrome/\n\
                                 - Ubuntu/Debian: sudo apt install chromium-browser\n\
                                 - Fedora: sudo dnf install chromium\n\
                                 - macOS: brew install --cask google-chrome\n\
                                 - Or specify path: --chrome-path /path/to/chrome\n\
                                 - Linux sandbox issue? Try: --no-sandbox",
                        e
                    ))
                })?)
                .await
                .map_err(|e| {
                    BrowserError::LaunchFailed(format!(
                        "{}. \n\n\
                         Chrome not found. You can:\n\
                         - Install Chrome: https://www.google.com/chrome/\n\
                         - Ubuntu/Debian: sudo apt install chromium-browser\n\
                         - Fedora: sudo dnf install chromium\n\
                         - macOS: brew install --cask google-chrome\n\
                         - Or specify path: --chrome-path /path/to/chrome\n\
                         - Linux sandbox issue? Try: --no-sandbox",
                        e
                    ))
                })?;

                // Spawn handler task
                tokio::spawn(async move {
                    while (handler.next().await).is_some() {
                        // Handle browser events
                    }
                });

                (browser, Some(temp_dir))
            }
            ConnectionMode::DebugPort(port) => {
                let url = format!("http://localhost:{}", port);
                let (browser, mut handler) = Browser::connect(&url).await.map_err(|e| {
                    BrowserError::ConnectionFailed(format!(
                        "Failed to connect to Chrome on port {}. \
                             Make sure Chrome is running with --remote-debugging-port={}: {}",
                        port, port, e
                    ))
                })?;

                // Spawn handler task
                tokio::spawn(async move {
                    while (handler.next().await).is_some() {
                        // Handle browser events
                    }
                });

                (browser, None)
            }
        };

        Ok(Self { browser, temp_dir })
    }

    /// Navigate to a URL
    pub async fn navigate(&self, url: &str) -> Result<()> {
        // Get the existing page instead of creating a new one
        let pages = self.browser.pages().await?;
        let page = if let Some(page) = pages.first() {
            // Use existing page
            page.clone()
        } else {
            // No page exists, create a new one
            self.browser
                .new_page("about:blank")
                .await
                .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?
        };

        // Navigate to the URL using the goto command
        // goto() returns a Page, which we can then use to wait for navigation
        let navigated_page = page.goto(url)
            .await
            .map_err(|e| BrowserError::NavigationFailed(format!("Failed to navigate to {}: {}", url, e)))?;

        // Wait for the navigation to complete
        navigated_page.wait_for_navigation()
            .await
            .map_err(|e| BrowserError::NavigationFailed(format!("Navigation timeout for {}: {}", url, e)))?;

        Ok(())
    }

    /// Get current URL
    pub async fn current_url(&self) -> Result<String> {
        let pages = self.browser.pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;

        let url = page
            .url()
            .await
            .map_err(|e| BrowserError::Other(e.to_string()))?
            .ok_or(BrowserError::NoPage)?;

        Ok(url)
    }

    /// Get page title
    pub async fn title(&self) -> Result<String> {
        let pages = self.browser.pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;

        let title = page
            .get_title()
            .await
            .map_err(|e| BrowserError::Other(e.to_string()))?
            .ok_or(BrowserError::NoPage)?;

        Ok(title)
    }

    /// Get page HTML source
    pub async fn get_page_source(&self) -> Result<String> {
        let pages = self.browser.pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;

        let html = page
            .content()
            .await
            .map_err(|e| BrowserError::Other(e.to_string()))?;

        Ok(html)
    }

    /// Get visible page text
    pub async fn get_page_text(&self) -> Result<String> {
        let pages = self.browser.pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;

        let text = page
            .find_element("body")
            .await
            .map_err(|_e| BrowserError::ElementNotFound("body".to_string()))?
            .inner_text()
            .await
            .map_err(|_e| BrowserError::ElementNotFound("body".to_string()))?
            .ok_or(BrowserError::ElementNotFound("body".to_string()))?;

        Ok(text)
    }

    /// Get text from specific element
    pub async fn get_element_text(&self, selector: &str) -> Result<String> {
        let pages = self.browser.pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;

        let text = page
            .find_element(selector)
            .await
            .map_err(|_e| BrowserError::ElementNotFound(selector.to_string()))?
            .inner_text()
            .await
            .map_err(|_e| BrowserError::ElementNotFound(selector.to_string()))?
            .ok_or(BrowserError::ElementNotFound(selector.to_string()))?;

        Ok(text)
    }

    /// Take a screenshot of the current page
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        let pages = self.browser.pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;

        let screenshot = page
            .screenshot(chromiumoxide::page::ScreenshotParams::default())
            .await
            .map_err(|e| BrowserError::Other(format!("Failed to take screenshot: {}", e)))?;

        Ok(screenshot)
    }

    /// Take a screenshot and save to file
    pub async fn screenshot_to_file(&self, path: &Path) -> Result<()> {
        let screenshot_data = self.screenshot().await?;

        tokio::fs::write(path, screenshot_data)
            .await
            .map_err(|e| BrowserError::Other(format!("Failed to write screenshot: {}", e)))?;

        Ok(())
    }

    /// Execute arbitrary JavaScript in the page context
    pub async fn execute_script(&self, script: &str) -> Result<serde_json::Value> {
        let pages = self.browser.pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;

        let result = page
            .evaluate(script)
            .await
            .map_err(|e| BrowserError::Other(format!("Script execution failed: {}", e)))?;

        Ok(result.into_value().unwrap_or(serde_json::Value::Null))
    }

    /// Execute JavaScript and return a specific type
    pub async fn execute_script_typed<T: serde::de::DeserializeOwned>(
        &self,
        script: &str,
    ) -> Result<T> {
        let pages = self.browser.pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;

        let result = page
            .evaluate(script)
            .await
            .map_err(|e| BrowserError::Other(format!("Script execution failed: {}", e)))?;

        result
            .into_value()
            .map_err(|e| BrowserError::Other(format!("Failed to deserialize result: {}", e)))
    }

    /// Send a raw CDP (Chrome DevTools Protocol) command using JSON
    ///
    /// This is a convenience wrapper for sending arbitrary CDP commands.
    /// The method should be in the format "Domain.method" (e.g., "Page.captureScreenshot", "Network.getCookies")
    ///
    /// For typed/safe CDP usage, use `driver.current_page()` to get the Page and use chromiumoxide's typed CDP methods.
    ///
    /// # Note on JavaScript Execution
    /// For executing JavaScript, use `execute_script()` instead - it's simpler and more reliable.
    ///
    /// # Common CDP Commands
    /// - `Page.captureScreenshot` - Take screenshots with custom options
    /// - `Emulation.setDeviceMetricsOverride` - Mobile device emulation
    /// - `Network.getCookies` - Get all cookies
    /// - `Performance.getMetrics` - Get performance metrics
    /// - `DOM.getDocument` - Get DOM tree
    /// - `Input.dispatchMouseEvent` - Simulate mouse events
    /// - `Input.dispatchKeyEvent` - Simulate keyboard events
    ///
    /// # Example - Set Geolocation
    /// ```ignore
    /// use serde_json::json;
    /// use robert_webdriver::ChromeDriver;
    ///
    /// # async fn example(driver: &ChromeDriver) -> anyhow::Result<()> {
    /// let params = json!({
    ///     "latitude": 37.7749,
    ///     "longitude": -122.4194,
    ///     "accuracy": 100
    /// });
    /// driver.send_cdp_command("Emulation.setGeolocationOverride", params).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Example - Get Cookies
    /// ```ignore
    /// use serde_json::json;
    /// use robert_webdriver::ChromeDriver;
    ///
    /// # async fn example(driver: &ChromeDriver) -> anyhow::Result<()> {
    /// let result = driver.send_cdp_command("Network.getCookies", json!({})).await?;
    /// println!("Cookies: {}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_cdp_command(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // For now, we'll implement common use cases via JavaScript
        // This is a limitation of chromiumoxide's typed API
        // TODO: Implement proper CDP command execution when chromiumoxide supports it

        // Special handling for common commands
        match method {
            "Runtime.evaluate" => {
                // Use our built-in execute_script for this
                if let Some(expression) = params.get("expression").and_then(|v| v.as_str()) {
                    let result = self.execute_script(expression).await?;
                    Ok(serde_json::json!({
                        "result": {
                            "type": "object",
                            "value": result
                        }
                    }))
                } else {
                    Err(BrowserError::Other(
                        "Runtime.evaluate requires 'expression' parameter".to_string(),
                    ))
                }
            }
            _ => {
                // For other CDP commands, user should use current_page() and chromiumoxide types
                Err(BrowserError::Other(format!(
                    "CDP command '{}' not directly supported. Use driver.current_page() and chromiumoxide::cdp types for typed CDP access. \
                    For JavaScript execution, use driver.execute_script(). \
                    See documentation for examples.",
                    method
                )))
            }
        }
    }

    /// Get access to the underlying Browser for advanced CDP usage
    pub fn browser(&self) -> &Browser {
        &self.browser
    }

    /// Get access to the current page for advanced operations
    /// Creates a new page if none exists
    pub async fn current_page(&self) -> Result<chromiumoxide::page::Page> {
        let pages = self.browser.pages().await?;
        if let Some(page) = pages.first().cloned() {
            Ok(page)
        } else {
            // No pages exist, create one
            self.browser
                .new_page("about:blank")
                .await
                .map_err(|e| BrowserError::Other(format!("Failed to create page: {}", e)))
        }
    }

    /// Close the browser connection
    pub async fn close(self) -> Result<()> {
        self.browser
            .close()
            .await
            .map_err(|e| BrowserError::Other(e.to_string()))?;
        Ok(())
    }

    /// Ensure Chrome is installed, downloading if necessary
    async fn ensure_chrome_installed() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| BrowserError::Other("Cannot determine cache directory".to_string()))?
            .join("robert")
            .join("chrome");

        // Create cache directory if it doesn't exist
        tokio::fs::create_dir_all(&cache_dir)
            .await
            .map_err(|e| BrowserError::Other(format!("Failed to create cache dir: {}", e)))?;

        // Check if Chrome already downloaded
        let revision_info_path = cache_dir.join(".downloaded");
        if revision_info_path.exists() {
            // Chrome already downloaded, find the executable
            if let Some(executable) = Self::find_chrome_in_cache(&cache_dir).await {
                return Ok(executable);
            }
        }

        // Download Chrome
        eprintln!("📥 Downloading Chrome for Testing (first time only, ~150MB)...");
        let fetcher = BrowserFetcher::new(
            BrowserFetcherOptions::builder()
                .with_path(&cache_dir)
                .build()
                .map_err(|e| BrowserError::Other(format!("Fetcher config failed: {}", e)))?,
        );

        let info = fetcher
            .fetch()
            .await
            .map_err(|e| BrowserError::Other(format!("Chrome download failed: {}", e)))?;

        // Mark as downloaded
        tokio::fs::write(&revision_info_path, "downloaded")
            .await
            .map_err(|e| BrowserError::Other(format!("Failed to write marker: {}", e)))?;

        eprintln!("✅ Chrome downloaded successfully!");

        Ok(info.executable_path)
    }

    /// Find Chrome executable in cache directory
    async fn find_chrome_in_cache(cache_dir: &Path) -> Option<PathBuf> {
        // Look for Chrome executable in various possible locations
        let possible_paths = vec![
            cache_dir.join("chrome"),
            cache_dir.join("chrome.exe"),
            cache_dir.join("Google Chrome.app/Contents/MacOS/Google Chrome"),
            cache_dir.join("chrome-linux/chrome"),
            cache_dir.join("chrome-mac/Chromium.app/Contents/MacOS/Chromium"),
            cache_dir.join("chrome-win/chrome.exe"),
        ];

        for path in possible_paths {
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    /// Execute a CDP script from a JSON file
    ///
    /// This method loads a CDP script and executes it via the CDP executor.
    /// Scripts are JSON files containing Chrome DevTools Protocol commands.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let report = driver.execute_cdp_script(Path::new("script.json")).await?;
    /// println!("Executed {} commands, {} successful",
    ///     report.total_commands, report.successful);
    /// ```
    pub async fn execute_cdp_script(
        &self,
        script_path: &std::path::Path,
    ) -> Result<crate::cdp::ExecutionReport> {
        // Load script from file
        let script = crate::cdp::CdpScript::from_file(script_path)
            .await
            .map_err(|e| BrowserError::Other(format!("Failed to load script: {}", e)))?;

        // Get current page
        let page = self.current_page().await?;

        // Create executor and run script
        let executor = crate::cdp::CdpExecutor::new(page);
        executor
            .execute_script(&script)
            .await
            .map_err(|e| BrowserError::Other(format!("Script execution failed: {}", e)))
    }

    /// Execute a CDP script from an in-memory CdpScript struct
    ///
    /// Useful when scripts are generated programmatically (e.g., by Claude)
    /// rather than loaded from files.
    pub async fn execute_cdp_script_direct(
        &self,
        script: &crate::cdp::CdpScript,
    ) -> Result<crate::cdp::ExecutionReport> {
        let page = self.current_page().await?;
        let executor = crate::cdp::CdpExecutor::new(page);
        executor
            .execute_script(script)
            .await
            .map_err(|e| BrowserError::Other(format!("Script execution failed: {}", e)))
    }

}

impl Drop for ChromeDriver {
    fn drop(&mut self) {
        // Clean up temporary directory if it exists
        if let Some(temp_dir) = &self.temp_dir {
            if temp_dir.exists() {
                let _ = std::fs::remove_dir_all(temp_dir);
            }
        }
    }
}
