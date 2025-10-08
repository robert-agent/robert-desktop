use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("Failed to connect to chromedriver: {0}")]
    ConnectionFailed(String),

    #[error("Navigation failed: {0}")]
    NavigationFailed(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("WebDriver error: {0}")]
    WebDriverError(#[from] thirtyfour::error::WebDriverError),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, BrowserError>;
