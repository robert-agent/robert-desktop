# End-to-End Test Suite

This directory contains end-to-end tests for the robert-webdriver crate using headless Chrome.

## Test Files

- `e2e.rs` - Main end-to-end test suite that runs with headless Chrome in CI
- `integration_test.rs` - Manual integration tests (marked with `#[ignore]`)

## Running Tests Locally

### Prerequisites

1. Install Chrome/Chromium:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install google-chrome-stable

   # macOS
   brew install --cask google-chrome
   ```

2. Install ChromeDriver:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install chromium-chromedriver

   # macOS
   brew install chromedriver

   # Or download from https://chromedriver.chromium.org/downloads
   ```

### Running Tests

Run all tests (including e2e):
```bash
cargo test --workspace
```

Run only e2e tests:
```bash
cargo test --test e2e
```

Run with output visible:
```bash
cargo test --test e2e -- --nocapture
```

Run a specific test:
```bash
cargo test --test e2e test_headless_chrome_navigation
```

## CI/CD

Tests run automatically in GitHub Actions on every push and pull request. The workflow:
1. Installs Chrome and ChromeDriver
2. Runs tests in headless mode
3. Uploads artifacts if tests fail

See `.github/workflows/ci.yml` and `.github/workflows/e2e-tests.yml` for details.

## Test Structure

Each test:
1. Starts a ChromeDriver process
2. Connects to it using `ChromeDriver::connect_with_options(port, headless: true)`
3. Performs browser automation
4. Cleans up automatically via Drop trait

Tests that need a local server use the `start_test_server()` helper which:
- Creates a warp HTTP server with test pages
- Binds to an ephemeral port
- Serves test HTML and API endpoints
- Cleans up automatically

## Troubleshooting

### ChromeDriver fails to start
- Ensure chromedriver is in your PATH: `which chromedriver`
- Check version compatibility: `chromedriver --version` and `google-chrome --version`
- Port 9515 might be in use, check with: `lsof -i :9515`

### Tests timeout
- Increase timeout in the test code
- Check network connectivity for external URLs (example.com)
- Chrome might be crashing - check system resources

### Tests fail in CI but pass locally
- CI runs in headless mode - ensure tests work headless locally:
  ```bash
  cargo test --test e2e
  ```
- Check CI logs for ChromeDriver version mismatches
