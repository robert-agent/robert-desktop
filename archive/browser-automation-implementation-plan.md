# Browser Automation Tool - Implementation Plan

## Project Structure

```
inferno/
├── crates/
│   └── browser-automation/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs              # CLI entry point
│       │   ├── lib.rs               # Library exports
│       │   ├── browser/
│       │   │   ├── mod.rs           # Browser module
│       │   │   ├── chrome.rs        # Chrome driver implementation
│       │   │   └── lifecycle.rs     # Launch/cleanup logic
│       │   ├── script/
│       │   │   ├── mod.rs           # Script module
│       │   │   ├── parser.rs        # YAML/DSL parser
│       │   │   ├── executor.rs      # Script execution engine
│       │   │   └── actions.rs       # Action definitions
│       │   ├── capture/
│       │   │   ├── mod.rs           # Capture module
│       │   │   ├── screenshot.rs    # Screenshot capture
│       │   │   ├── text.rs          # Text extraction
│       │   │   └── metadata.rs      # Metadata collection
│       │   ├── storage/
│       │   │   ├── mod.rs           # Storage module
│       │   │   ├── filesystem.rs    # File operations
│       │   │   └── manifest.rs      # Manifest generation
│       │   ├── error.rs             # Error types
│       │   └── config.rs            # Configuration
│       ├── examples/
│       │   ├── basic_navigation.yaml
│       │   ├── form_interaction.yaml
│       │   └── data_extraction.yaml
│       ├── tests/
│       │   ├── integration/
│       │   └── unit/
│       └── README.md
└── docs/
    ├── browser-automation-prd.md
    └── browser-automation-implementation-plan.md
```

## Phase 1: Project Setup and Browser Control (Week 1)

### Milestone 1.1: Project Initialization

**Tasks:**
1. Create new crate `browser-automation` in workspace
2. Add dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   chromiumoxide = "0.5"
   tokio = { version = "1.0", features = ["full"] }
   futures = "0.3"
   anyhow = "1.0"
   thiserror = "1.0"
   clap = { version = "4.0", features = ["derive"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   serde_yaml = "0.9"
   tracing = "0.1"
   tracing-subscriber = "0.3"
   uuid = { version = "1.0", features = ["v4"] }
   chrono = { version = "0.4", features = ["serde"] }
   image = "0.24"
   base64 = "0.21"
   ```
3. Set up module structure
4. Configure logging with `tracing`

**Deliverables:**
- Compiling crate with basic structure
- CLI skeleton with `--help` output

### Milestone 1.2: Basic Browser Launch

**Tasks:**
1. Implement `browser/chrome.rs`:
   - `ChromeDriver` struct
   - `launch()` method with Chrome DevTools Protocol
   - Configure Chrome flags (window mode, disable automation warnings)
   - Handle Chrome binary detection on Windows
2. Implement `browser/lifecycle.rs`:
   - Process management
   - Cleanup on exit
   - Signal handling
3. Add error handling for browser launch failures

**Code Example:**
```rust
use chromiumoxide::Browser;
use chromiumoxide::BrowserConfig;

pub struct ChromeDriver {
    browser: Browser,
}

impl ChromeDriver {
    pub async fn launch() -> Result<Self, BrowserError> {
        let (browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .window_size(1280, 1024)
                .build()?
        ).await?;

        // Spawn handler
        tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                // Handle events
            }
        });

        Ok(Self { browser })
    }

    pub async fn close(self) -> Result<(), BrowserError> {
        self.browser.close().await?;
        Ok(())
    }
}
```

**Deliverables:**
- Working Chrome launch
- Clean browser shutdown
- Basic error handling

### Milestone 1.3: Navigation and Page Control

**Tasks:**
1. Implement navigation methods:
   - `navigate(url: &str)`
   - `wait_for_load()`
   - `wait_for_selector(selector: &str)`
2. Implement page interaction:
   - `get_page()` - get current page handle
   - `evaluate(js: &str)` - execute JavaScript
3. Add timeout configuration
4. Implement wait strategies

**Deliverables:**
- Can navigate to URLs
- Can wait for page load
- Can execute JS in page context

**Testing:**
- Navigate to test URLs (example.com, httpbin.org)
- Verify page load completion
- Execute simple JS and verify results

## Phase 2: Content Capture (Week 2)

### Milestone 2.1: Screenshot Capture

**Tasks:**
1. Implement `capture/screenshot.rs`:
   ```rust
   pub enum ScreenshotType {
       Viewport,
       FullPage,
       Element { selector: String },
   }

   pub async fn capture_screenshot(
       page: &Page,
       screenshot_type: ScreenshotType,
   ) -> Result<Vec<u8>, CaptureError>
   ```
2. Handle different screenshot types:
   - Viewport: Direct CDP capture
   - Full page: Scroll and stitch
   - Element: Calculate bounds and capture region
3. Image encoding (PNG, JPEG)
4. Quality/compression options

**Deliverables:**
- All three screenshot types working
- Configurable format and quality

### Milestone 2.2: Text Extraction

**Tasks:**
1. Implement `capture/text.rs`:
   ```rust
   pub async fn extract_text(
       page: &Page,
       selector: Option<&str>,
   ) -> Result<String, CaptureError>
   ```
2. Support extraction modes:
   - Full page text
   - Element-specific text
   - Multiple elements (return Vec)
3. Handle text normalization
4. Preserve basic structure (paragraphs)

**Deliverables:**
- Text extraction from page
- Selector-based extraction
- Clean text output

### Milestone 2.3: Metadata Collection

**Tasks:**
1. Implement `capture/metadata.rs`:
   ```rust
   pub struct PageMetadata {
       pub url: String,
       pub title: String,
       pub timestamp: DateTime<Utc>,
       pub viewport: Viewport,
       pub user_agent: String,
   }
   ```
2. Collect page metadata
3. Implement serialization
4. Add custom metadata fields

**Deliverables:**
- Complete metadata capture
- JSON serialization

## Phase 3: User Interactions (Week 3)

### Milestone 3.1: Element Interactions

**Tasks:**
1. Implement `script/actions.rs` with action types:
   ```rust
   #[derive(Debug, Clone, Deserialize)]
   #[serde(tag = "action", rename_all = "snake_case")]
   pub enum Action {
       Navigate { url: String, wait_for: WaitCondition },
       Click { selector: String },
       Type { selector: String, text: String },
       Scroll { direction: ScrollDirection, amount: i32 },
       Wait { condition: WaitCondition, timeout: u64 },
       Screenshot { screenshot_type: ScreenshotType, filename: String },
       ExtractText { selector: Option<String>, output: String },
       Execute { javascript: String },
   }
   ```
2. Implement each action handler
3. Add retry logic for flaky interactions
4. Handle element not found gracefully

**Deliverables:**
- All interaction actions working
- Reliable element targeting
- Error recovery

### Milestone 3.2: Advanced Interactions

**Tasks:**
1. Form handling:
   - Select dropdowns
   - Radio buttons
   - Checkboxes
   - File uploads (if needed)
2. Keyboard events (Tab, Enter, Escape)
3. Mouse events (hover, double-click)
4. Dialog handling (alert, confirm, prompt)

**Deliverables:**
- Complex form interactions
- Keyboard/mouse support
- Dialog handling

## Phase 4: Script System (Week 4)

### Milestone 4.1: Script Parser

**Tasks:**
1. Implement `script/parser.rs`:
   ```rust
   #[derive(Debug, Deserialize)]
   pub struct Script {
       pub name: String,
       pub version: String,
       pub output_dir: PathBuf,
       pub steps: Vec<Action>,
   }

   pub fn parse_script(path: &Path) -> Result<Script, ParseError>
   ```
2. YAML parsing with serde_yaml
3. Validation:
   - Required fields present
   - Selectors valid format
   - Paths valid
   - URLs parseable
4. Clear error messages for malformed scripts

**Deliverables:**
- YAML script parsing
- Validation logic
- Helpful error messages

### Milestone 4.2: Script Executor

**Tasks:**
1. Implement `script/executor.rs`:
   ```rust
   pub struct Executor {
       driver: ChromeDriver,
       config: ExecutorConfig,
   }

   impl Executor {
       pub async fn execute(&mut self, script: Script) -> Result<ExecutionResult, ExecutorError>
   }
   ```
2. Sequential step execution
3. Progress reporting (tracing logs)
4. Error handling strategies:
   - Continue on error
   - Stop on error
   - Retry failed steps
5. Collect execution metadata

**Deliverables:**
- Complete script execution
- Configurable error handling
- Execution reporting

### Milestone 4.3: Execution Context

**Tasks:**
1. Add variable support:
   - Extract data and store in variables
   - Reference variables in later steps
   - Environment variable substitution
2. Conditional execution:
   - If element exists
   - If text matches
   - If variable equals
3. Loop support (optional for v1)

**Example:**
```yaml
steps:
  - action: navigate
    url: "${BASE_URL}/login"

  - action: extract_text
    selector: "h1"
    variable: "page_title"

  - action: conditional
    condition:
      variable: "page_title"
      equals: "Login Page"
    then:
      - action: type
        selector: "#username"
        text: "user"
```

**Deliverables:**
- Variable system
- Basic conditionals
- Environment variable support

## Phase 5: Storage and Output (Week 5)

### Milestone 5.1: Filesystem Storage

**Tasks:**
1. Implement `storage/filesystem.rs`:
   ```rust
   pub struct FileStorage {
       base_dir: PathBuf,
   }

   impl FileStorage {
       pub fn save_screenshot(&self, name: &str, data: &[u8]) -> Result<PathBuf>
       pub fn save_text(&self, name: &str, content: &str) -> Result<PathBuf>
       pub fn save_html(&self, name: &str, content: &str) -> Result<PathBuf>
   }
   ```
2. Directory structure creation
3. Filename sanitization
4. Collision handling (increment, overwrite, error)
5. Path validation for Windows

**Deliverables:**
- Organized output directory
- Safe file operations
- Windows path compatibility

### Milestone 5.2: Manifest Generation

**Tasks:**
1. Implement `storage/manifest.rs`:
   ```rust
   pub struct Manifest {
       pub run_id: Uuid,
       pub timestamp: DateTime<Utc>,
       pub script: String,
       pub browser: String,
       pub status: ExecutionStatus,
       pub duration_ms: u64,
       pub steps: Vec<StepResult>,
       pub errors: Vec<ErrorRecord>,
       pub warnings: Vec<String>,
   }
   ```
2. Collect step results during execution
3. Generate final manifest
4. Pretty-print JSON output
5. Include file references

**Deliverables:**
- Complete manifest.json
- Execution summary
- Error/warning tracking

### Milestone 5.3: Output Formats

**Tasks:**
1. Support multiple screenshot formats (PNG, JPEG, WebP)
2. Support text formats (TXT, MD, JSON)
3. Optional HTML capture
4. Configuration for output preferences

**Deliverables:**
- Configurable output formats
- Format conversion utilities

## Phase 6: CLI and Testing (Week 6)

### Milestone 6.1: Command-Line Interface

**Tasks:**
1. Implement `main.rs` with clap:
   ```rust
   #[derive(Parser)]
   #[command(name = "browser-automation")]
   #[command(about = "Automate Chrome browser with scripts")]
   struct Cli {
       #[command(subcommand)]
       command: Commands,
   }

   #[derive(Subcommand)]
   enum Commands {
       Run {
           #[arg(short, long)]
           script: PathBuf,

           #[arg(short, long)]
           output: Option<PathBuf>,
       },
       Validate {
           #[arg(short, long)]
           script: PathBuf,
       },
   }
   ```
2. Implement subcommands:
   - `run` - execute script
   - `validate` - check script syntax
3. Add global options:
   - `--verbose` - increase logging
   - `--chrome-path` - custom Chrome binary
   - `--timeout` - default timeout
4. Progress indicators
5. Pretty error output

**Deliverables:**
- Full-featured CLI
- User-friendly output
- Help documentation

### Milestone 6.2: Testing

**Tasks:**
1. Unit tests:
   - Script parsing
   - Action validation
   - Metadata generation
   - File operations
2. Integration tests:
   - Full script execution (using local test server)
   - Screenshot capture validation
   - Text extraction accuracy
   - Error handling paths
3. Test utilities:
   - Mock HTTP server
   - Test page generator
   - Fixture scripts

**Testing Strategy:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_navigate_and_screenshot() {
        let driver = ChromeDriver::launch().await.unwrap();
        let page = driver.new_page().await.unwrap();

        page.navigate("http://example.com").await.unwrap();
        page.wait_for_load().await.unwrap();

        let screenshot = capture_screenshot(&page, ScreenshotType::Viewport)
            .await
            .unwrap();

        assert!(!screenshot.is_empty());
    }
}
```

**Deliverables:**
- >70% code coverage
- Integration test suite
- CI-ready tests

### Milestone 6.3: Documentation

**Tasks:**
1. README.md:
   - Installation instructions
   - Quick start guide
   - Feature overview
   - Example usage
2. Script reference documentation:
   - All actions documented
   - Examples for each action
   - Best practices
3. API documentation (rustdoc)
4. Example scripts:
   - basic_navigation.yaml
   - form_interaction.yaml
   - data_extraction.yaml
   - error_handling.yaml
5. Troubleshooting guide

**Deliverables:**
- Complete documentation
- Working examples
- Troubleshooting guide

## Phase 7: Polish and Release (Week 7)

### Milestone 7.1: Windows Testing

**Tasks:**
1. Test on Windows 10 and 11
2. Verify Chrome detection
3. Test path handling
4. Verify file permissions
5. Test with Windows Defender
6. Package for Windows (optional: installer/portable exe)

**Deliverables:**
- Windows-compatible release
- Platform-specific documentation

### Milestone 7.2: Performance Optimization

**Tasks:**
1. Profile memory usage
2. Optimize screenshot capture
3. Reduce startup time
4. Minimize async overhead
5. Add resource limits

**Deliverables:**
- Performance benchmarks
- Optimized implementation

### Milestone 7.3: Release Preparation

**Tasks:**
1. Version tagging (0.1.0)
2. Release notes
3. Binary builds
4. Example repository
5. User feedback channels

**Deliverables:**
- v0.1.0 release
- Distribution artifacts

## Dependencies and Risks

### Critical Dependencies
- **chromiumoxide** or **headless_chrome**: Core browser automation
  - Risk: API instability, CDP changes
  - Mitigation: Version pinning, abstraction layer
- **Chrome browser**: Target browser
  - Risk: Version incompatibilities
  - Mitigation: Support multiple versions, clear requirements

### Technical Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Chrome binary detection fails | High | Allow manual path specification |
| CDP connection issues | High | Retry logic, clear error messages |
| Screenshot stitching bugs | Medium | Thorough testing, fallback to viewport |
| Script parsing errors | Medium | Comprehensive validation, examples |
| Memory leaks | High | Profiling, RAII patterns, testing |
| Windows-specific bugs | Medium | Windows testing environment, CI |

## Testing Strategy

### Unit Tests
- Each module tested independently
- Mock external dependencies
- Fast execution (<1s total)

### Integration Tests
- Full workflow tests
- Local test server for HTTP
- Snapshot testing for outputs
- Longer execution acceptable

### Manual Testing
- Windows 10/11 testing
- Chrome version matrix
- Real-world websites
- Performance testing

### Continuous Integration
- GitHub Actions workflow
- Windows runner
- Chrome installation
- Test execution
- Code coverage reporting

## Success Criteria

### Functional Completeness
- [ ] Can launch Chrome on Windows
- [ ] Can navigate and interact with pages
- [ ] Can capture screenshots (all types)
- [ ] Can extract text
- [ ] Can parse and execute YAML scripts
- [ ] Can save outputs in organized structure
- [ ] Generates manifest.json

### Quality Metrics
- [ ] >70% test coverage
- [ ] Zero critical bugs
- [ ] <5s browser launch time
- [ ] <2s screenshot capture
- [ ] Documentation complete
- [ ] Example scripts working

### User Acceptance
- [ ] Successfully runs example scripts
- [ ] Clear error messages
- [ ] Intuitive script syntax
- [ ] Organized output
- [ ] Easy installation

## Future Enhancements (Post-v1)

### Phase 8: Headless Mode
- Chrome headless support
- Performance improvements
- Background execution

### Phase 9: Multi-Tab Support
- Concurrent automation
- Tab management
- Resource pooling

### Phase 10: Advanced Features
- Network capture (HAR files)
- Cookie/session management
- Proxy support
- PDF export
- Video recording
- Browser extension support

### Phase 11: Cross-Browser
- Firefox support (via geckodriver)
- Edge support
- Safari support (macOS)

### Phase 12: Cloud Execution
- Remote browser support
- Docker containerization
- Distributed execution

## Appendix: Key Code Patterns

### Error Handling Pattern
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("Failed to launch browser: {0}")]
    LaunchFailed(String),

    #[error("Navigation timeout: {0}")]
    NavigationTimeout(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),
}
```

### Async Pattern
```rust
use tokio::time::{sleep, Duration};

pub async fn wait_for_element(
    page: &Page,
    selector: &str,
    timeout: Duration,
) -> Result<Element, BrowserError> {
    let start = Instant::now();

    loop {
        if let Ok(element) = page.find_element(selector).await {
            return Ok(element);
        }

        if start.elapsed() > timeout {
            return Err(BrowserError::ElementNotFound(selector.to_string()));
        }

        sleep(Duration::from_millis(100)).await;
    }
}
```

### Resource Cleanup Pattern
```rust
impl Drop for ChromeDriver {
    fn drop(&mut self) {
        // Ensure browser process cleanup
        if let Some(process) = self.process.take() {
            let _ = process.kill();
        }
    }
}
```

## References

- [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/)
- [chromiumoxide docs](https://docs.rs/chromiumoxide/)
- [Selenium WebDriver Spec](https://www.w3.org/TR/webdriver/)
- [Playwright Architecture](https://playwright.dev/docs/library) - for inspiration
- [Puppeteer API](https://pptr.dev/) - reference API design
