# Robert CLI - Browser Automation Tool

A command-line interface for browser automation using Chrome DevTools Protocol (CDP).

## Installation

Requires Chrome or Chromium to be installed on your system.

### Install Chrome/Chromium

**Ubuntu/Debian:**
```bash
sudo apt install chromium-browser
```

**Fedora:**
```bash
sudo dnf install chromium
```

**macOS:**
```bash
brew install --cask google-chrome
```

**Or download from:** https://www.google.com/chrome/

## Usage

### Basic Usage (Auto-detect Chrome)

```bash
# Navigate to a URL and print HTML
cargo run --bin robert -- https://example.com

# Print visible text only
cargo run --bin robert -- https://example.com --format text

# Extract specific element
cargo run --bin robert -- https://example.com --selector "h1"
```

### Custom Chrome Path

If Chrome isn't auto-detected, specify the path:

```bash
cargo run --bin robert -- https://example.com --chrome-path /usr/bin/chromium
```

### Advanced Mode (Connect to Running Chrome)

Connect to an existing Chrome instance with your profile:

```bash
# First, start Chrome with remote debugging
google-chrome --remote-debugging-port=9222

# Then connect to it
cargo run --bin robert -- https://example.com --debug-port 9222
```

## Options

```
robert <URL> [OPTIONS]

Arguments:
  <URL>  URL to navigate to

Options:
  --debug-port <PORT>       Connect to existing Chrome debug port (advanced mode)
  --chrome-path <PATH>      Path to Chrome/Chromium executable
  -f, --format <FORMAT>     Output format: html or text [default: html]
  -s, --selector <SELECTOR> CSS selector for specific element
  -h, --help                Print help
  -V, --version             Print version
```

## Modes

### 1. Sandboxed Mode (Default)
- Uses system Chrome installation
- Isolated session (no cookies/history)
- Auto-detects Chrome location
- Best for: Testing, automation, scripts

### 2. Advanced Mode (--debug-port)
- Connects to your running Chrome
- Uses your real profile (logged-in accounts, cookies, etc.)
- Requires Chrome started with `--remote-debugging-port`
- Best for: Authenticated workflows, personal automation

### 3. Custom Path (--chrome-path)
- Specify exact Chrome/Chromium binary
- Useful when multiple versions installed
- Useful when Chrome not in standard location

## Examples

```bash
# Basic navigation
robert https://github.com

# Get page title area
robert https://github.com --selector "title"

# Use specific Chrome
robert https://github.com --chrome-path /usr/bin/chromium

# Connect to running Chrome (macOS)
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222 &
robert https://github.com --debug-port 9222

# Get visible text only
robert https://github.com --format text
```

## Troubleshooting

**Error: "Chrome/Chromium not found"**

Solution:
1. Install Chrome/Chromium (see Installation above)
2. Or specify path: `--chrome-path /path/to/chrome`
3. Or use advanced mode: `--debug-port 9222`

**Error: "Failed to connect to Chrome on port 9222"**

Solution:
1. Make sure Chrome is running with: `chrome --remote-debugging-port=9222`
2. Check no other process is using port 9222
3. Try a different port number

## Technology

- **Library:** spider_chrome (maintained chromiumoxide fork)
- **Protocol:** Chrome DevTools Protocol (CDP)
- **Runtime:** Tokio async
- **Language:** Rust

## License

MIT OR Apache-2.0
