// spider_chrome re-exports chromiumoxide API
use crate::error::{BrowserError, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;

pub struct ChromeDriver {
    browser: Browser,
}

/// Connection mode for Chrome browser
pub enum ConnectionMode {
    /// Sandboxed mode - launches Chrome using system installation
    Sandboxed,
    /// Advanced mode - connects to existing Chrome on debug port
    DebugPort(u16),
}

impl ChromeDriver {
    /// Launch Chrome in sandboxed mode (uses system Chrome)
    pub async fn launch_sandboxed() -> Result<Self> {
        Self::new(ConnectionMode::Sandboxed).await
    }

    /// Connect to existing Chrome on debug port (advanced mode)
    pub async fn connect_debug_port(port: u16) -> Result<Self> {
        Self::new(ConnectionMode::DebugPort(port)).await
    }

    /// Create new ChromeDriver with specified connection mode
    pub async fn new(mode: ConnectionMode) -> Result<Self> {
        let browser = match mode {
            ConnectionMode::Sandboxed => {
                // Launch Chrome with visible UI
                let (browser, mut handler) = Browser::launch(
                    BrowserConfig::builder()
                        .with_head() // Show browser window
                        .build()
                        .map_err(|e| BrowserError::LaunchFailed(e.to_string()))?,
                )
                .await
                .map_err(|e| BrowserError::LaunchFailed(e.to_string()))?;

                // Spawn handler task
                tokio::spawn(async move {
                    while let Some(_) = handler.next().await {
                        // Handle browser events
                    }
                });

                browser
            }
            ConnectionMode::DebugPort(port) => {
                let url = format!("http://localhost:{}", port);
                let (browser, mut handler) = Browser::connect(&url)
                    .await
                    .map_err(|e| {
                        BrowserError::ConnectionFailed(format!(
                            "Failed to connect to Chrome on port {}. \
                             Make sure Chrome is running with --remote-debugging-port={}: {}",
                            port, port, e
                        ))
                    })?;

                // Spawn handler task
                tokio::spawn(async move {
                    while let Some(_) = handler.next().await {
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

    /// Close the browser connection
    pub async fn close(self) -> Result<()> {
        self.browser
            .close()
            .await
            .map_err(|e| BrowserError::Other(e.to_string()))?;
        Ok(())
    }
}
