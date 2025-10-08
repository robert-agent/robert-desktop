pub mod browser;
pub mod error;

// Re-export commonly used items
pub use browser::chrome::{ChromeDriver, ConnectionMode};
pub use error::BrowserError;
