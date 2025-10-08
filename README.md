# Robert - Browser Automation Tool

A user-friendly browser automation tool with a native macOS desktop application built with Tauri and Rust.

## Project Status

✅ **Phase 0 Complete** - CLI Prototype Implemented

## Overview

Robert is a desktop application that enables users to automate browser interactions through YAML scripts while observing the automation in real-time. The tool provides:

- **Visual Desktop App** - Native macOS application with real-time status display
- **Simple YAML Scripts** - Easy-to-write automation scripts
- **Chrome Automation** - Visible browser automation via WebDriver
- **Content Capture** - Screenshots and text extraction
- **Output Management** - Organized file storage with visual browser

## Technology Stack

- **Desktop Framework**: Tauri 2.0
- **Frontend**: Svelte + TypeScript + Tailwind CSS
- **Backend**: Rust 1.70+
- **Browser Automation**: thirtyfour (WebDriver)
- **Target Platform**: macOS (v1.0), Linux headless (roadmap)

## Project Structure

```
robert/
├── PRD.md                    # Product Requirements Document
├── IMPLEMENTATION_PLAN.md   # 7+ week development roadmap
├── PHASE_0_TASKS.md         # CLI prototype task breakdown
├── README.md                # This file
├── CHANGELOG.md             # Version history
├── Cargo.toml               # Workspace configuration
├── archive/                 # Deprecated documents
└── crates/                  # Rust workspace
    ├── robert-webdriver/    # Browser automation library
    │   ├── src/
    │   │   ├── browser/     # Chrome driver implementation
    │   │   ├── error.rs     # Error types
    │   │   └── lib.rs
    │   └── tests/           # Integration tests with warp server
    ├── robert-cli/          # CLI tool (implemented)
    │   └── src/main.rs
    └── robert-app/          # Tauri desktop app (placeholder)
```

## Documentation

- **[PRD](PRD.md)** - Complete product requirements document
- **[Implementation Plan](IMPLEMENTATION_PLAN.md)** - 7+ week development roadmap with Phase 0 CLI prototype
- **[Phase 0 Tasks](PHASE_0_TASKS.md)** - Detailed task breakdown for CLI prototype (2-3 days)

## Roadmap

### Phase 0 (Days 1-3) - CLI Prototype ✅ COMPLETE
- ✅ Cargo workspace with 3 crates
- ✅ Basic browser automation library
- ✅ CLI tool: navigate and fetch content
- ✅ Validated thirtyfour integration
- ✅ Integration tests with warp test server

### Version 1.0 (Weeks 1-7) - macOS Desktop App
- 🔄 Tauri desktop application
- 🔄 Chrome automation via thirtyfour
- 🔄 Real-time execution UI
- 🔄 Screenshot and text capture
- 🔄 YAML script format
- 🔄 Output management

### Version 1.5 (Months 3-4) - Linux Headless
- 🔄 Linux CLI binary
- 🔄 Headless Chrome support
- 🔄 Docker container
- 🔄 REST API

### Version 2.0+ - Multi-Browser & Beyond
- 🔄 Firefox, Edge, Safari support
- 🔄 Windows desktop app
- 🔄 Visual script builder
- 🔄 Cloud execution

## Example Script

```yaml
name: "Example Automation"
version: "1.0.0"
output_dir: "./output/example"

browser:
  type: "chrome"
  window_size: [1280, 1024]

steps:
  - action: navigate
    url: "https://example.com"
    wait_for: "dom_content_loaded"

  - action: screenshot
    type: "full_page"
    filename: "homepage.png"

  - action: extract_text
    selector: "h1"
    output: "title.txt"
```

## Getting Started

### Phase 0: CLI Prototype (Current)

**Prerequisites:**
- Rust 1.70+
- Chrome browser
- chromedriver (`brew install chromedriver`)

**Quick Start:**
```bash
# 1. Start chromedriver in a separate terminal
chromedriver --port=9515

# 2. Run CLI prototype
cargo run --bin robert -- https://example.com

# Output HTML (default)
cargo run --bin robert -- https://example.com --format html

# Output visible text only
cargo run --bin robert -- https://example.com --format text

# Extract text from specific element
cargo run --bin robert -- https://example.com --selector "h1"

# Use different chromedriver port
cargo run --bin robert -- https://example.com --port 9516
```

**Running Tests:**
```bash
# Run unit tests
cargo test --workspace --lib

# Run integration tests (requires chromedriver running)
cargo test -p robert-webdriver --test integration_test -- --ignored

# Run all workspace checks
cargo check --workspace
cargo clippy --workspace
cargo fmt --all -- --check
```

### Phase 1+: Desktop App (Future)

**Additional Prerequisites:**
- Node.js 18+
- macOS 11+ (for v1.0)

Implementation begins after Phase 0 validation.

## Development Status

**Current Phase**: Phase 0 Complete ✅ → Ready for Phase 1

### Completed:
1. ✅ Planning documents finalized
2. ✅ **Phase 0 implementation complete** (see [PHASE_0_TASKS.md](PHASE_0_TASKS.md))
3. ✅ Browser automation approach validated
4. ✅ Workspace with 3 crates created
5. ✅ robert-webdriver library implemented
6. ✅ robert-cli working with Chrome via chromedriver
7. ✅ Integration tests with warp test server

### Next Steps:
1. 🔄 Begin Phase 1: Tauri desktop app
2. 🔄 Implement YAML script parser
3. 🔄 Build execution status UI
4. 🔄 Add screenshot capture functionality

### Phase 0 Summary:
- **Workspace**: Cargo multi-crate workspace with shared dependencies
- **robert-webdriver**: Core browser automation library using thirtyfour
- **robert-cli**: CLI tool that connects to Chrome and fetches content
- **robert-app**: Placeholder for future Tauri application
- **Tests**: Integration tests with local warp HTTP server

## Contributing

This project is not yet accepting contributions. Please check back after v1.0 release.

## License

TBD

## Contact

Project maintained by the Robert team.

---

**Note**: This is a planning repository. All implementation details are subject to change based on technical feasibility and user feedback.
