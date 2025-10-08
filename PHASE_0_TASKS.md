# Phase 0: CLI Prototype - Implementation Tasks

## Overview
Build a minimal CLI tool to validate browser automation approach before developing the full desktop app.

**Duration**: 2-3 days
**Goal**: CLI tool that navigates to a URL and fetches content using visible Chrome

---

## Milestone 0.1: Workspace Setup

### Task 1.1: Initialize Cargo Workspace
**Estimated Time**: 30 minutes

**Steps:**
1. Create workspace root `Cargo.toml`:
   ```bash
   cd /home/jeef/robert
   cat > Cargo.toml << 'EOF'
   [workspace]
   resolver = "2"
   members = [
       "crates/robert-webdriver",
       "crates/robert-cli",
       "crates/robert-app",
   ]

   [workspace.dependencies]
   # Shared dependencies across all crates
   tokio = { version = "1.0", features = ["full"] }
   anyhow = "1.0"
   thiserror = "1.0"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"

   # Browser automation
   thirtyfour = "0.32"

   # CLI
   clap = { version = "4.0", features = ["derive"] }
   EOF
   ```

2. Create crates directory:
   ```bash
   mkdir -p crates
   ```

**Acceptance Criteria:**
- [ ] `Cargo.toml` created at workspace root
- [ ] `crates/` directory exists
- [ ] Workspace resolver set to "2"

---

### Task 1.2: Create robert-webdriver Crate
**Estimated Time**: 15 minutes

**Steps:**
1. Create library crate:
   ```bash
   cd crates
   cargo new --lib robert-webdriver
   cd robert-webdriver
   ```

2. Update `Cargo.toml`:
   ```toml
   [package]
   name = "robert-webdriver"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   thirtyfour = { workspace = true }
   tokio = { workspace = true }
   anyhow = { workspace = true }
   thiserror = { workspace = true }
   ```

3. Create module structure:
   ```bash
   mkdir -p src/browser
   touch src/browser/mod.rs
   touch src/browser/chrome.rs
   touch src/error.rs
   ```

4. Update `src/lib.rs`:
   ```rust
   pub mod browser;
   pub mod error;

   // Re-export commonly used items
   pub use browser::chrome::ChromeDriver;
   pub use error::BrowserError;
   ```

**Acceptance Criteria:**
- [ ] `robert-webdriver` crate created
- [ ] Dependencies configured
- [ ] Module structure created
- [ ] `cargo build -p robert-webdriver` succeeds

---

### Task 1.3: Create robert-cli Crate
**Estimated Time**: 15 minutes

**Steps:**
1. Create binary crate:
   ```bash
   cd /home/jeef/robert/crates
   cargo new robert-cli
   cd robert-cli
   ```

2. Update `Cargo.toml`:
   ```toml
   [package]
   name = "robert-cli"
   version = "0.1.0"
   edition = "2021"

   [[bin]]
   name = "robert"
   path = "src/main.rs"

   [dependencies]
   robert-webdriver = { path = "../robert-webdriver" }
   tokio = { workspace = true }
   clap = { workspace = true }
   anyhow = { workspace = true }
   ```

3. Add placeholder main.rs:
   ```rust
   fn main() {
       println!("Robert CLI - Coming soon!");
   }
   ```

**Acceptance Criteria:**
- [ ] `robert-cli` crate created
- [ ] Depends on `robert-webdriver`
- [ ] Binary name configured as "robert"
- [ ] `cargo build -p robert-cli` succeeds
- [ ] `cargo run --bin robert` prints placeholder message

---

### Task 1.4: Create robert-app Placeholder
**Estimated Time**: 10 minutes

**Steps:**
1. Create placeholder crate:
   ```bash
   cd /home/jeef/robert/crates
   cargo new --lib robert-app
   cd robert-app
   ```

2. Update `Cargo.toml`:
   ```toml
   [package]
   name = "robert-app"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   # Will be populated in Phase 1
   ```

3. Add placeholder `src/lib.rs`:
   ```rust
   // Tauri desktop application
   // To be implemented in Phase 1
   ```

**Acceptance Criteria:**
- [ ] `robert-app` crate created
- [ ] Placeholder only (no implementation)
- [ ] Workspace builds successfully: `cargo build --workspace`

---

### Task 1.5: Verify Workspace Setup
**Estimated Time**: 10 minutes

**Steps:**
1. From workspace root, run:
   ```bash
   cargo build --workspace
   cargo test --workspace
   cargo check --workspace
   ```

2. Verify structure:
   ```bash
   tree crates -L 2
   # Should show:
   # crates/
   # ├── robert-app/
   # │   ├── Cargo.toml
   # │   └── src/
   # ├── robert-cli/
   # │   ├── Cargo.toml
   # │   └── src/
   # └── robert-webdriver/
   #     ├── Cargo.toml
   #     └── src/
   ```

**Acceptance Criteria:**
- [ ] All crates compile without errors
- [ ] No dependency conflicts
- [ ] Workspace structure verified

---

## Milestone 0.2: Implement Browser Automation

### Task 2.1: Implement Error Types
**Estimated Time**: 30 minutes

**Steps:**
1. Edit `crates/robert-webdriver/src/error.rs`:
   ```rust
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
   ```

**Acceptance Criteria:**
- [ ] Error types defined
- [ ] Implements `std::error::Error`
- [ ] Converts from `thirtyfour::error::WebDriverError`
- [ ] Compiles without warnings

---

### Task 2.2: Implement ChromeDriver::connect()
**Estimated Time**: 45 minutes

**Steps:**
1. Edit `crates/robert-webdriver/src/browser/chrome.rs`:
   ```rust
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

       /// Close the browser connection
       pub async fn close(self) -> Result<()> {
           self.driver.quit().await?;
           Ok(())
       }
   }
   ```

2. Update `crates/robert-webdriver/src/browser/mod.rs`:
   ```rust
   pub mod chrome;
   pub use chrome::ChromeDriver;
   ```

**Acceptance Criteria:**
- [ ] `ChromeDriver::connect()` implemented
- [ ] Takes port number as parameter
- [ ] Returns `Result<Self>`
- [ ] Error handling for connection failures
- [ ] Compiles and passes basic tests

**Testing:**
```bash
# Terminal 1: Start chromedriver
chromedriver --port=9515

# Terminal 2: Test connection
cargo test -p robert-webdriver connect_test
```

---

### Task 2.3: Implement Navigation Methods
**Estimated Time**: 30 minutes

**Steps:**
1. Add to `crates/robert-webdriver/src/browser/chrome.rs`:
   ```rust
   impl ChromeDriver {
       // ... existing methods ...

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
   }
   ```

**Acceptance Criteria:**
- [ ] `navigate()` method works
- [ ] `current_url()` returns current page URL
- [ ] `title()` returns page title
- [ ] Error handling for navigation failures

---

### Task 2.4: Implement Content Extraction
**Estimated Time**: 45 minutes

**Steps:**
1. Add to `crates/robert-webdriver/src/browser/chrome.rs`:
   ```rust
   impl ChromeDriver {
       // ... existing methods ...

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
               .map_err(|e| BrowserError::ElementNotFound(selector.to_string()))?;

           let text = element.text().await?;
           Ok(text)
       }
   }
   ```

**Acceptance Criteria:**
- [ ] `get_page_source()` returns HTML
- [ ] `get_page_text()` returns visible text
- [ ] `get_element_text()` finds element by CSS selector
- [ ] Proper error handling for missing elements

---

### Task 2.5: Add Integration Tests
**Estimated Time**: 30 minutes

**Steps:**
1. Create `crates/robert-webdriver/tests/integration_test.rs`:
   ```rust
   use robert_webdriver::ChromeDriver;

   #[tokio::test]
   #[ignore] // Run manually with chromedriver running
   async fn test_basic_navigation() {
       let driver = ChromeDriver::connect(9515)
           .await
           .expect("Failed to connect");

       driver.navigate("https://example.com")
           .await
           .expect("Failed to navigate");

       let title = driver.title()
           .await
           .expect("Failed to get title");

       assert_eq!(title, "Example Domain");

       let text = driver.get_page_text()
           .await
           .expect("Failed to get text");

       assert!(text.contains("Example Domain"));
   }
   ```

2. Run tests:
   ```bash
   # Start chromedriver first
   chromedriver --port=9515

   # Run tests
   cargo test -p robert-webdriver --test integration_test -- --ignored
   ```

**Acceptance Criteria:**
- [ ] Integration test passes
- [ ] Can connect to Chrome
- [ ] Can navigate to example.com
- [ ] Can extract title and text

---

## Milestone 0.3: Build CLI Interface

### Task 3.1: Implement CLI Argument Parsing
**Estimated Time**: 30 minutes

**Steps:**
1. Edit `crates/robert-cli/src/main.rs`:
   ```rust
   use clap::Parser;
   use robert_webdriver::ChromeDriver;

   #[derive(Parser)]
   #[command(name = "robert")]
   #[command(version = "0.1.0")]
   #[command(about = "Browser automation CLI prototype", long_about = None)]
   struct Cli {
       /// URL to navigate to
       url: String,

       /// Chromedriver port
       #[arg(short, long, default_value = "9515")]
       port: u16,

       /// Output format: html or text
       #[arg(short = 'f', long, default_value = "html")]
       format: String,

       /// CSS selector for specific element (optional)
       #[arg(short = 's', long)]
       selector: Option<String>,
   }

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       let cli = Cli::parse();

       println!("Robert CLI v0.1.0");
       println!("================\n");

       // TODO: Implement logic

       Ok(())
   }
   ```

**Acceptance Criteria:**
- [ ] Parses URL as positional argument
- [ ] Accepts `--port` flag
- [ ] Accepts `--format` flag (html/text)
- [ ] Accepts `--selector` for specific elements
- [ ] `cargo run --bin robert -- --help` shows help

---

### Task 3.2: Implement CLI Logic
**Estimated Time**: 45 minutes

**Steps:**
1. Complete `crates/robert-cli/src/main.rs`:
   ```rust
   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       let cli = Cli::parse();

       println!("Robert CLI v0.1.0");
       println!("================\n");

       // Connect to Chrome
       println!("🔌 Connecting to Chrome on port {}...", cli.port);
       let driver = ChromeDriver::connect(cli.port).await?;

       // Navigate
       println!("🌐 Navigating to {}...", cli.url);
       driver.navigate(&cli.url).await?;

       // Get page info
       let title = driver.title().await?;
       println!("✅ Page loaded: {}\n", title);

       // Extract content
       let content = if let Some(selector) = cli.selector {
           println!("📍 Extracting content from: {}\n", selector);
           driver.get_element_text(&selector).await?
       } else {
           match cli.format.as_str() {
               "text" => driver.get_page_text().await?,
               _ => driver.get_page_source().await?,
           }
       };

       println!("{}", content);

       Ok(())
   }
   ```

**Acceptance Criteria:**
- [ ] Connects to chromedriver
- [ ] Navigates to URL
- [ ] Prints page title
- [ ] Extracts content based on format
- [ ] Handles errors gracefully

---

### Task 3.3: Add CLI Error Handling
**Estimated Time**: 20 minutes

**Steps:**
1. Improve error messages in `main.rs`:
   ```rust
   #[tokio::main]
   async fn main() {
       if let Err(e) = run().await {
           eprintln!("❌ Error: {}", e);
           std::process::exit(1);
       }
   }

   async fn run() -> Result<(), Box<dyn std::error::Error>> {
       let cli = Cli::parse();

       // ... existing logic with better error context ...

       let driver = ChromeDriver::connect(cli.port)
           .await
           .map_err(|e| format!("Failed to connect to Chrome. Is chromedriver running on port {}?\n  Error: {}", cli.port, e))?;

       driver.navigate(&cli.url)
           .await
           .map_err(|e| format!("Failed to navigate to {}:\n  Error: {}", cli.url, e))?;

       // ... rest of logic ...

       Ok(())
   }
   ```

**Acceptance Criteria:**
- [ ] Clear error messages
- [ ] Helpful hints (e.g., "Is chromedriver running?")
- [ ] Non-zero exit code on error

---

### Task 3.4: Test CLI End-to-End
**Estimated Time**: 30 minutes

**Steps:**
1. Start chromedriver:
   ```bash
   chromedriver --port=9515
   ```

2. Test various scenarios:
   ```bash
   # Basic HTML output
   cargo run --bin robert -- https://example.com

   # Text output
   cargo run --bin robert -- https://example.com --format text

   # Specific element
   cargo run --bin robert -- https://example.com --selector "h1"

   # Different port
   cargo run --bin robert -- https://example.com --port 9516

   # Invalid URL (should show error)
   cargo run --bin robert -- invalid-url

   # No chromedriver running (should show helpful error)
   # (stop chromedriver first)
   cargo run --bin robert -- https://example.com
   ```

**Acceptance Criteria:**
- [ ] All test scenarios work as expected
- [ ] Visible Chrome window opens
- [ ] Content is displayed correctly
- [ ] Error messages are helpful

---

## Milestone 0.4: Documentation and Cleanup

### Task 4.1: Update README
**Estimated Time**: 20 minutes

**Steps:**
1. Add Phase 0 usage to main README.md
2. Document how to run the CLI prototype
3. Add troubleshooting section

**Acceptance Criteria:**
- [ ] README updated with CLI usage
- [ ] Prerequisites listed
- [ ] Examples provided

---

### Task 4.2: Add Workspace README
**Estimated Time**: 15 minutes

**Steps:**
1. Create `crates/robert-webdriver/README.md`:
   - API documentation
   - Examples
   - Integration instructions

2. Create `crates/robert-cli/README.md`:
   - CLI usage
   - Examples
   - Options reference

**Acceptance Criteria:**
- [ ] Each crate has README
- [ ] Documentation is clear
- [ ] Examples are runnable

---

### Task 4.3: Final Testing and Validation
**Estimated Time**: 30 minutes

**Steps:**
1. Run all tests:
   ```bash
   cargo test --workspace
   cargo test --workspace -- --ignored  # with chromedriver running
   ```

2. Run clippy:
   ```bash
   cargo clippy --workspace -- -D warnings
   ```

3. Format code:
   ```bash
   cargo fmt --all
   ```

4. Verify all success criteria from Phase 0

**Acceptance Criteria:**
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted
- [ ] All Phase 0 deliverables met

---

## Success Checklist

### Workspace Setup
- [ ] Cargo workspace created with 3 crates
- [ ] `robert-webdriver` library crate compiles
- [ ] `robert-cli` binary crate compiles
- [ ] `robert-app` placeholder exists
- [ ] Workspace builds successfully

### Browser Automation (robert-webdriver)
- [ ] `ChromeDriver::connect()` works
- [ ] Can navigate to URLs
- [ ] Can extract HTML source
- [ ] Can extract visible text
- [ ] Can find and extract element text
- [ ] Error handling implemented
- [ ] Integration tests pass

### CLI Tool (robert-cli)
- [ ] Accepts URL argument
- [ ] Connects to chromedriver
- [ ] Navigates to provided URL
- [ ] Displays page content
- [ ] Supports `--format` flag (html/text)
- [ ] Supports `--selector` flag
- [ ] Shows helpful error messages
- [ ] Works with visible Chrome window

### Documentation
- [ ] Main README updated
- [ ] Crate READMEs created
- [ ] Examples documented
- [ ] Usage instructions clear

---

## Timeline

- **Day 1 Morning**: Tasks 1.1-1.5 (Workspace Setup)
- **Day 1 Afternoon**: Tasks 2.1-2.2 (Error handling + Connection)
- **Day 2 Morning**: Tasks 2.3-2.5 (Navigation + Content + Tests)
- **Day 2 Afternoon**: Tasks 3.1-3.2 (CLI interface)
- **Day 3 Morning**: Tasks 3.3-3.4 (Error handling + Testing)
- **Day 3 Afternoon**: Tasks 4.1-4.3 (Documentation + Cleanup)

---

## Next Steps After Phase 0

Once Phase 0 is complete:
1. Review and validate the approach
2. Begin Phase 1: Tauri desktop app setup
3. Reuse `robert-webdriver` in `robert-app`
4. Keep `robert-cli` for debugging and testing

---

## Troubleshooting

### chromedriver not found
```bash
# macOS
brew install chromedriver

# Or download from https://chromedriver.chromium.org/
```

### Connection refused
- Ensure chromedriver is running: `chromedriver --port=9515`
- Check Chrome is installed
- Verify port is not in use

### Element not found
- Check selector syntax (CSS selector required)
- Wait for page to load fully
- Verify element exists on page
