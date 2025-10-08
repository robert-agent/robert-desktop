pub mod browser;
pub mod error;

// Re-export commonly used items
pub use browser::chrome::ChromeDriver;
pub use error::BrowserError;
