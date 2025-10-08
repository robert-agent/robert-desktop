# robert-cli

Command-line interface for browser automation - Phase 0 prototype.

## Overview

A simple CLI tool that demonstrates browser automation capabilities using the `robert-webdriver` library. This tool connects to Chrome via chromedriver and can navigate to URLs and extract content.

## Installation

```bash
# Build from workspace root
cargo build --bin robert

# Or run directly
cargo run --bin robert -- <URL>
```

## Prerequisites

- Rust 1.70+
- Chrome browser installed
- chromedriver installed and running

### Installing chromedriver

**macOS:**
```bash
brew install chromedriver
```

**Linux:**
```bash
# Download from https://chromedriver.chromium.org/
# Or use package manager (e.g., apt, yum)
```

## Usage

### Start chromedriver

First, start chromedriver in a separate terminal:

```bash
chromedriver --port=9515
```

### Basic Usage

```bash
# Navigate to a URL and get HTML
cargo run --bin robert -- https://example.com

# Get visible text only
cargo run --bin robert -- https://example.com --format text

# Extract text from specific element
cargo run --bin robert -- https://example.com --selector "h1"

# Use different port
cargo run --bin robert -- https://example.com --port 9516
```

## Command-Line Options

```
Usage: robert [OPTIONS] <URL>

Arguments:
  <URL>  URL to navigate to

Options:
  -p, --port <PORT>          Chromedriver port [default: 9515]
  -f, --format <FORMAT>      Output format: html or text [default: html]
  -s, --selector <SELECTOR>  CSS selector for specific element (optional)
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples

### Get page HTML
```bash
cargo run --bin robert -- https://example.com
```

Output:
```
Robert CLI v0.1.0
================

🔌 Connecting to Chrome on port 9515...
🌐 Navigating to https://example.com...
✅ Page loaded: Example Domain

<!doctype html>
<html>
...
</html>
```

### Get visible text
```bash
cargo run --bin robert -- https://example.com --format text
```

Output:
```
Robert CLI v0.1.0
================

🔌 Connecting to Chrome on port 9515...
🌐 Navigating to https://example.com...
✅ Page loaded: Example Domain

Example Domain
This domain is for use in illustrative examples in documents...
```

### Extract specific element
```bash
cargo run --bin robert -- https://example.com --selector "h1"
```

Output:
```
Robert CLI v0.1.0
================

🔌 Connecting to Chrome on port 9515...
🌐 Navigating to https://example.com...
✅ Page loaded: Example Domain
📍 Extracting content from: h1

Example Domain
```

## Error Handling

The CLI provides helpful error messages:

### chromedriver not running
```
❌ Error: Failed to connect to Chrome. Is chromedriver running on port 9515?
  Error: Connection refused
```

### Invalid URL
```
❌ Error: Failed to navigate to invalid-url:
  Error: Invalid URL
```

### Element not found
```
❌ Error: Element not found: .non-existent-selector
```

## Architecture

```
robert-cli (main.rs)
    │
    ├─ Argument parsing (clap)
    ├─ Error handling
    └─ Uses robert-webdriver
        │
        ├─ ChromeDriver::connect()
        ├─ navigate()
        ├─ get_page_source()
        ├─ get_page_text()
        └─ get_element_text()
```

## Future Enhancements

This CLI tool is a Phase 0 prototype. Future versions will include:

- YAML script support
- Multiple page automation
- Screenshot capture
- Form interaction
- Wait conditions
- Output to files

The primary focus is shifting to the Tauri desktop application (`robert-app`) in Phase 1.

## Contributing

This is a prototype tool. See the main project README for contribution guidelines.

## License

TBD
