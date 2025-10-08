# End-to-End Test Suite

This directory contains end-to-end tests for the robert-webdriver crate using Chrome DevTools Protocol (CDP).

## Test Files

- `e2e.rs` - Main end-to-end test suite with three tests:
  - `test_navigate_and_get_title` - Basic navigation and title extraction
  - `test_get_element_text` - CSS selector and element text extraction
  - `test_get_page_source` - HTML source retrieval

## Running Tests Locally

### Prerequisites

**No installation required!** Tests automatically download Chrome for Testing on first run (~150MB, cached at `~/.cache/robert/chrome`).

Optionally, you can install Chrome/Chromium:
```bash
# Ubuntu/Debian
sudo apt-get install google-chrome-stable

# macOS
brew install --cask google-chrome
```

### Running Tests

Run all tests in the webdriver crate:
```bash
cargo test --package robert-webdriver
```

Run only e2e tests:
```bash
cargo test --package robert-webdriver --test e2e
```

Run with output visible:
```bash
cargo test --package robert-webdriver --test e2e -- --nocapture
```

Run a specific test:
```bash
cargo test --package robert-webdriver test_navigate_and_get_title
```

Run in CI mode (headless, no 5-second delay):
```bash
CI=true cargo test --package robert-webdriver
```

## Test Behavior

### Local Testing (Visible Window)
- Chrome window opens and is **visible** during test execution
- Tests keep the window open for **5 seconds** before closing
- Allows you to see what the automation is doing
- Uses auto-downloaded Chrome or system Chrome

### CI Testing (Headless)
- Automatically detects CI environment variables:
  - `CI`, `GITHUB_ACTIONS`, `GITLAB_CI`, `JENKINS_HOME`, `CIRCLECI`
- Runs Chrome in **headless mode** (no visible window)
- Skips the 5-second delay for faster execution
- Automatically uses `--no-sandbox` flag for Linux CI environments

## CI/CD

Tests run automatically in GitHub Actions on every push and pull request. The workflow:
1. Auto-downloads Chrome for Testing (~150MB, first run only)
2. Runs tests in headless mode with `--no-sandbox`
3. Uploads artifacts if tests fail

See `.github/workflows/e2e-tests.yml` for details.

## Test Structure

Each test:
1. Detects CI environment automatically
2. Launches Chrome via CDP (headless in CI, visible locally)
3. Performs browser automation (navigate, extract text, get HTML)
4. Keeps window open for 5 seconds (local only)
5. Closes browser and cleans up

## Troubleshooting

### Tests fail with "No usable sandbox" on Linux
- Use CI mode which auto-enables `--no-sandbox`:
  ```bash
  CI=true cargo test --package robert-webdriver
  ```
- Or manually set the environment variable in your test environment

### Chrome auto-download fails
- Check internet connection
- Verify cache directory is writable: `ls -la ~/.cache/robert/`
- Manually install Chrome and tests will use system Chrome

### Tests timeout
- Increase timeout in the test code
- Check network connectivity for external URLs (example.com)
- Chrome might be crashing - check system resources

### Tests fail in CI but pass locally
- CI runs in headless mode - test locally with:
  ```bash
  CI=true cargo test --package robert-webdriver
  ```
- Check that your tests don't depend on visible UI elements

### Window doesn't stay open for 5 seconds
- Make sure you're not running in CI mode
- Check that `CI` environment variable is not set: `echo $CI`
