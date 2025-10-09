// spider_chrome re-exports chromiumoxide API
use crate::error::{BrowserError, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide_fetcher::{BrowserFetcher, BrowserFetcherOptions};
use futures::StreamExt;
use std::path::{Path, PathBuf};

pub struct ChromeDriver {
    browser: Browser,
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
        let browser = match mode {
            ConnectionMode::Sandboxed {
                chrome_path,
                no_sandbox,
                headless,
            } => {
                // Launch Chrome with visible UI or headless
                let mut config = if headless {
                    BrowserConfig::builder()
                } else {
                    BrowserConfig::builder().with_head()
                };

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

                browser
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

                browser
            }
        };

        Ok(Self { browser })
    }

    /// Navigate to a URL
    pub async fn navigate(&self, url: &str) -> Result<()> {
        let page = self
            .browser
            .new_page(url)
            .await
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        // Wait for page to load
        page.wait_for_navigation()
            .await
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

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
}
