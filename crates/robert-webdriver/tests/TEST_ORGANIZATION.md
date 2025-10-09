# Robert WebDriver Test Suite Organization

## Test Categories

### 🔧 Meta/Infrastructure Tests (`meta_infrastructure_test.rs`)
**Purpose**: Verify testing infrastructure itself
**Chrome Required**: ❌ No
**Tests**: 4

- `meta_test_server_starts` - Test server starts on random port
- `meta_test_server_serves_html` - Test server serves correct HTML
- `meta_test_multiple_servers_different_ports` - Isolation with different ports
- `meta_test_server_wait_ready` - Server readiness detection

```bash
cargo test --package robert-webdriver --test meta_infrastructure_test
```

---

### ✅ Validation Tests (`validation_test.rs`)
**Purpose**: CDP JSON schema validation
**Chrome Required**: ❌ No
**Tests**: 20

Tests the CDP script validation system without needing Chrome.

```bash
cargo test --package robert-webdriver validation
```

---

### 🌐 E2E Tests (`e2e.rs`)
**Purpose**: End-to-end Chrome automation
**Chrome Required**: ✅ Yes (auto-downloads)
**Tests**: 3

All use CDP commands for reliable navigation with local test server.

- `test_navigate_and_get_title` - Navigate, extract title and text
- `test_get_element_text` - Extract h1 element text
- `test_get_page_source` - Get full page HTML source

```bash
cargo test --package robert-webdriver --test e2e -- --test-threads=1
```

---

### 🎯 Headless Integration Tests (`headless_integration.rs`)
**Purpose**: Headless CI/CD workflows
**Chrome Required**: ✅ Yes (auto-downloads)
**Tests**: 5

- `test_basic_navigation_headless` - Navigation + title via CDP
- `test_cdp_script_execution_headless` - Complete CDP script execution
- `test_screenshot_capture_headless` - Capture screenshot
- `test_data_extraction_headless` - Extract structured data
- `test_multiple_commands_headless` - Multiple CDP commands

```bash
cargo test --package robert-webdriver --test headless_integration -- --test-threads=1
```

---

### 🚫 Ignored Tests (Manual Only)

#### CDP Generator Tests (`cdp_generator_test.rs`)
**Status**: `#[ignore]` - Requires AI integration
**Tests**: 5

#### CDP Script Execution Tests (`cdp_script_execution_test.rs`)
**Status**: `#[ignore]` - Depends on external files
**Tests**: 4

#### CDP Execution Tests (`cdp_execution_test.rs`)
**Status**: `#[ignore]` - Low-level manual tests
**Tests**: 2

```bash
# Run all ignored tests
cargo test --package robert-webdriver -- --ignored
```

---

## Test Summary

| Category | Count | Chrome | Always Run |
|----------|-------|--------|------------|
| Meta/Infrastructure | 4 | ❌ | ✅ |
| Validation | 20 | ❌ | ✅ |
| Library Unit | 15 | ❌ | ✅ |
| E2E | 3 | ✅ | ✅ |
| Headless Integration | 5 | ✅ | ✅ |
| **Total Active** | **47** | - | ✅ |
| Ignored (Manual) | 11 | ✅ | ❌ |

---

## Key Features

### Local Test Server
All Chrome tests use `test_server` module:
- Random ports for isolation
- Serves example.com-like HTML
- `wait_ready()` for reliability
- Zero network dependencies
- Offline capable

### CDP Commands
E2E tests use CDP commands directly (not high-level `navigate()`) because:
- More reliable with concurrent Chrome instances
- Better control over navigation
- Consistent across environments

### No Duplicates
- ✅ Removed duplicate test_server tests (were running 3x)
- ✅ Created separate `meta_infrastructure_test.rs` for infrastructure tests
- ✅ Each test has clear, distinct purpose

---

## Quick Commands

```bash
# All tests (recommended)
cargo test --package robert-webdriver -- --test-threads=1

# Fast tests only (no Chrome, ~instant)
cargo test --package robert-webdriver validation
cargo test --package robert-webdriver --test meta_infrastructure_test

# Chrome tests only
cargo test --package robert-webdriver --test e2e -- --test-threads=1
cargo test --package robert-webdriver --test headless_integration -- --test-threads=1

# Linting
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

---

## Test Isolation

**Why `--test-threads=1`?**
Chrome instances share profile directories. Running tests sequentially prevents:
```
ERROR:process_singleton_posix.cc(340)] Failed to create SingletonLock
```

**Each test gets**:
- Own test server on random port
- Clean Chrome instance
- Isolated execution environment
