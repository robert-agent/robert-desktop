# Robert - AI Automation for Browser Tasks

**AI-powered browser automation you can see and control.**

Robert is a local-first desktop application that uses AI to automate browser-based tasks. Unlike traditional automation tools, Robert provides real-time visual feedback, letting you watch and control every step of the automation process.

## Key Features

- **Real-time Visual Feedback** - Watch browser automation happen with color-coded debug logs
- **Screenshot & DOM Capture** - Capture page screenshots and extract DOM content at any point
- **Browser Control** - Navigate, click, type, and interact with any web page
- **Local-first Architecture** - Built with Rust and Tauri for native performance
- **Event System** - Track every automation step with detailed event logging
- **Open Source** - Free, auditable, MIT/Apache-2.0 dual-licensed

## Current Status

**Active Development** - Desktop app implemented with core automation features

### Implemented
- ✅ Tauri desktop application with Svelte/TypeScript frontend
- ✅ Chrome DevTools Protocol (CDP) integration for browser control
- ✅ Screenshot capture functionality
- ✅ DOM content extraction
- ✅ Real-time debug logs with color coding
- ✅ Event system tracking automation steps
- ✅ Auto-download Chrome for Testing
- ✅ CI/CD with GitHub Actions for releases and updates

### In Development
- 🔄 AI-powered automation scripting
- 🔄 Advanced browser interactions (click, type, scroll)
- 🔄 Form filling and data extraction
- 🔄 Multi-step automation workflows

## Technology Stack

- **Desktop Framework**: Tauri 2.0
- **Frontend**: Svelte 5 + TypeScript + Vite
- **Backend**: Rust 1.70+
- **Browser Automation**: spider_chrome (Chrome DevTools Protocol)
- **Chrome Management**: Auto-download via spider_chromiumoxide_fetcher
- **Target Platform**: macOS (primary), Linux (supported), Windows (planned)

## Project Structure

```
robert/
├── README.md                # This file
├── BUILD.md                 # Build instructions
├── Cargo.toml               # Workspace configuration
└── crates/                  # Rust workspace
    ├── robert-webdriver/    # Browser automation library
    │   ├── src/
    │   │   ├── browser/     # Chrome CDP implementation
    │   │   ├── error.rs     # Error types
    │   │   └── lib.rs
    │   └── tests/           # E2E tests
    ├── robert-cli/          # CLI tool
    │   └── src/main.rs
    └── robert-app/          # Tauri desktop application
        ├── src/             # Svelte frontend
        ├── src-tauri/       # Rust backend
        └── package.json     # Frontend dependencies
```

## Quick Start

### Prerequisites
- **Rust** 1.70 or later
- **Bun** (for frontend development)
- **System libraries** (macOS: Xcode, Linux: GTK/WebKit)

### Running the Desktop App

```bash
# Clone the repository
git clone https://github.com/yourusername/robert.git
cd robert

# Install frontend dependencies
cd crates/robert-app
bun install

# Run in development mode
bun run dev
```

### Running the CLI

```bash
# Navigate to robert directory
cd robert

# Run the CLI (Chrome auto-downloads on first run)
cargo run --bin robert -- example.com

# Extract text only
cargo run --bin robert -- example.com --format text

# Run headless
cargo run --bin robert -- example.com --headless
```

See [BUILD.md](BUILD.md) for detailed build instructions.

## How It Works

1. **Launch the app** - Start Robert desktop application
2. **Enter a URL** - Type the website you want to automate
3. **Watch it work** - See real-time logs as browser automation happens
4. **Capture content** - Take screenshots and extract DOM content
5. **Control everything** - Monitor, inspect, and control the automation process

### Example Use Cases

- **Web scraping** - Extract data from websites
- **Testing** - Automated UI testing and validation
- **Monitoring** - Check website changes and availability
- **Data collection** - Gather information from multiple sources
- **Screenshot capture** - Document web pages and applications

## Development & Testing

### Running Tests

```bash
# Run all tests
cargo test --package robert-webdriver

# Run in CI mode (headless)
CI=true cargo test --package robert-webdriver

# Run with output visible
cargo test --package robert-webdriver -- --nocapture

# Run all workspace checks
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

### Building for Production

```bash
# Build the desktop app
cd crates/robert-app
bun run build

# Build debug version
bun run build:debug
```

## Contributing

Robert is in active development. Contributions are welcome!

Areas where you can help:
- Browser automation features
- UI/UX improvements
- Documentation
- Testing and bug reports
- Performance optimizations

## Architecture

Robert consists of three main components:

1. **robert-webdriver** - Core Rust library for browser automation using Chrome DevTools Protocol
2. **robert-cli** - Command-line interface for quick automation tasks
3. **robert-app** - Tauri desktop application with Svelte frontend for visual automation

The architecture prioritizes:
- **Performance** - Native Rust backend with minimal overhead
- **Reliability** - Robust error handling and recovery
- **Transparency** - Real-time logging and event tracking
- **Portability** - Cross-platform support via Tauri

---

**AI-powered browser automation you can see and control.**
