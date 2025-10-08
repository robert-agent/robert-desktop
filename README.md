# Robert - Browser Automation Tool

A user-friendly browser automation tool with a native macOS desktop application built with Tauri and Rust.

## Project Status

🚧 **Planning Phase** - Not yet implemented

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
├── archive/                 # Deprecated documents
└── crates/                  # Rust workspace (to be created)
    ├── robert-webdriver/    # Browser automation library
    ├── robert-cli/          # CLI tool
    └── robert-app/          # Tauri desktop app
```

## Documentation

- **[PRD](PRD.md)** - Complete product requirements document
- **[Implementation Plan](IMPLEMENTATION_PLAN.md)** - 7+ week development roadmap with Phase 0 CLI prototype
- **[Phase 0 Tasks](PHASE_0_TASKS.md)** - Detailed task breakdown for CLI prototype (2-3 days)

## Roadmap

### Phase 0 (Days 1-3) - CLI Prototype
- 🚧 Cargo workspace with 3 crates
- 🚧 Basic browser automation library
- 🚧 CLI tool: navigate and fetch content
- 🚧 Validate thirtyfour integration

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
# 1. Start chromedriver
chromedriver --port=9515

# 2. Run CLI prototype (after Phase 0 implementation)
cargo run --bin robert -- https://example.com

# Output HTML
cargo run --bin robert -- https://example.com --format html

# Output text
cargo run --bin robert -- https://example.com --format text
```

### Phase 1+: Desktop App (Future)

**Additional Prerequisites:**
- Node.js 18+
- macOS 11+ (for v1.0)

Implementation begins after Phase 0 validation.

## Development Status

**Current Phase**: Phase 0 - CLI Prototype (Planning Complete)

### Next Steps:
1. ✅ Planning documents finalized
2. 🚧 **Start Phase 0 implementation** (see [PHASE_0_TASKS.md](PHASE_0_TASKS.md))
3. 🔄 Validate browser automation approach
4. 🔄 Begin Phase 1: Tauri desktop app

### Phase 0 Timeline:
- Day 1: Workspace setup + basic connection
- Day 2: Navigation + content extraction
- Day 3: CLI interface + testing

## Contributing

This project is not yet accepting contributions. Please check back after v1.0 release.

## License

TBD

## Contact

Project maintained by the Robert team.

---

**Note**: This is a planning repository. All implementation details are subject to change based on technical feasibility and user feedback.
