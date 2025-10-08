use thirtyfour::prelude::*;
use crate::error::{BrowserError, Result};

pub struct ChromeDriver {
    driver: WebDriver,
}

impl ChromeDriver {
    /// Connect to an existing Chrome instance via chromedriver
    pub async fn connect(port: u16) -> Result<Self> {
        let caps = DesiredCapabilities::chrome();

        let url = format!("http://localhost:{}", port);
        let driver = WebDriver::new(&url, caps)
            .await
            .map_err(|e| BrowserError::ConnectionFailed(e.to_string()))?;

        Ok(Self { driver })
    }

    /// Navigate to a URL
    pub async fn navigate(&self, url: &str) -> Result<()> {
        self.driver.goto(url)
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
        let body = self.driver
            .find(By::Tag("body"))
            .await
            .map_err(|e| BrowserError::ElementNotFound(e.to_string()))?;

        let text = body.text().await?;
        Ok(text)
    }

    /// Get text from specific element
    pub async fn get_element_text(&self, selector: &str) -> Result<String> {
        let element = self.driver
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
