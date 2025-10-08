use crate::error::{BrowserError, Result};
use thirtyfour::prelude::*;

pub struct ChromeDriver {
    driver: WebDriver,
}

impl ChromeDriver {
    /// Connect to an existing Chrome instance via chromedriver
    pub async fn connect(port: u16) -> Result<Self> {
        Self::connect_with_options(port, false).await
    }

    /// Connect to Chrome with custom options
    pub async fn connect_with_options(port: u16, headless: bool) -> Result<Self> {
        let mut caps = DesiredCapabilities::chrome();

        if headless {
            caps.add_arg("--headless")?;
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            caps.add_arg("--disable-gpu")?;
        }

        let url = format!("http://localhost:{}", port);
        let driver = WebDriver::new(&url, caps)
            .await
            .map_err(|e| BrowserError::ConnectionFailed(e.to_string()))?;

        Ok(Self { driver })
    }

    /// Connect to Chrome via custom URL (e.g., Selenium Grid)
    pub async fn connect_url(url: &str, headless: bool) -> Result<Self> {
        let mut caps = DesiredCapabilities::chrome();

        if headless {
            caps.add_arg("--headless")?;
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            caps.add_arg("--disable-gpu")?;
        }

        let driver = WebDriver::new(url, caps)
            .await
            .map_err(|e| BrowserError::ConnectionFailed(e.to_string()))?;

        Ok(Self { driver })
    }

    /// Navigate to a URL
    pub async fn navigate(&self, url: &str) -> Result<()> {
        self.driver
            .goto(url)
            .await
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;
        Ok(())
    }

    /// Get current URL
    pub async fn current_url(&self) -> Result<String> {
        let url = self.driver.current_url().await?;
        Ok(url.to_string())
    }

    /// Get page title
    pub async fn title(&self) -> Result<String> {
        let title = self.driver.title().await?;
        Ok(title)
    }

    /// Get page HTML source
    pub async fn get_page_source(&self) -> Result<String> {
        let source = self.driver.source().await?;
        Ok(source)
    }

    /// Get visible page text
    pub async fn get_page_text(&self) -> Result<String> {
        let body = self
            .driver
            .find(By::Tag("body"))
            .await
            .map_err(|e| BrowserError::ElementNotFound(e.to_string()))?;

        let text = body.text().await?;
        Ok(text)
    }

    /// Get text from specific element
    pub async fn get_element_text(&self, selector: &str) -> Result<String> {
        let element = self
            .driver
            .find(By::Css(selector))
            .await
            .map_err(|_e| BrowserError::ElementNotFound(selector.to_string()))?;

        let text = element.text().await?;
        Ok(text)
    }

    /// Close the browser connection
    pub async fn close(self) -> Result<()> {
        self.driver.quit().await?;
        Ok(())
    }
}
