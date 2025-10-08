# robert-webdriver

Core browser automation library for the Robert project using thirtyfour WebDriver.

## Overview

This library provides a high-level interface for browser automation, currently supporting Chrome via chromedriver. It serves as the foundation for both the CLI tool (`robert-cli`) and the future desktop application (`robert-app`).

## Features

- **Browser Connection**: Connect to Chrome via chromedriver
- **Navigation**: Navigate to URLs and track page state
- **Content Extraction**: Get page source, visible text, and element text
- **Error Handling**: Comprehensive error types with context

## Usage

### Basic Example

```rust
use robert_webdriver::ChromeDriver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to chromedriver running on port 9515
    let driver = ChromeDriver::connect(9515).await?;

    // Navigate to a URL
    driver.navigate("https://example.com").await?;

    // Get page title
    let title = driver.title().await?;
    println!("Page title: {}", title);

    // Get page text
    let text = driver.get_page_text().await?;
    println!("Page text: {}", text);

    // Extract specific element text
    let h1 = driver.get_element_text("h1").await?;
    println!("H1 text: {}", h1);

    Ok(())
}
```

## API Reference

### ChromeDriver

#### `ChromeDriver::connect(port: u16) -> Result<Self>`
Connect to an existing Chrome instance via chromedriver.

#### `navigate(&self, url: &str) -> Result<()>`
Navigate to a URL.

#### `current_url(&self) -> Result<String>`
Get the current page URL.

#### `title(&self) -> Result<String>`
Get the current page title.

#### `get_page_source(&self) -> Result<String>`
Get the full HTML source of the page.

#### `get_page_text(&self) -> Result<String>`
Get all visible text on the page.

#### `get_element_text(&self, selector: &str) -> Result<String>`
Get text from a specific element using CSS selector.

#### `close(self) -> Result<()>`
Close the browser connection.

## Error Types

```rust
pub enum BrowserError {
    ConnectionFailed(String),
    NavigationFailed(String),
    ElementNotFound(String),
    WebDriverError(WebDriverError),
    Other(String),
}
```

## Testing

### Unit Tests
```bash
cargo test -p robert-webdriver
```

### Integration Tests
Integration tests require chromedriver to be running:

```bash
# Terminal 1: Start chromedriver
chromedriver --port=9515

# Terminal 2: Run integration tests
cargo test -p robert-webdriver --test integration_test -- --ignored
```

Integration tests use a local warp HTTP server to provide test pages, eliminating dependency on external websites.

## Dependencies

- **thirtyfour**: WebDriver client for Rust
- **tokio**: Async runtime
- **anyhow**: Error handling
- **thiserror**: Custom error types

### Dev Dependencies

- **warp**: HTTP server for integration tests
- **serde_json**: JSON serialization for test data

## Future Enhancements

- Support for Firefox (geckodriver)
- Support for Edge (msedgedriver)
- Support for Safari (safaridriver)
- Screenshot capture
- Cookie management
- JavaScript execution
- Element interaction (click, type, etc.)

## License

TBD
