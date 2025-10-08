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
├── docs/
│   ├── browser-automation-prd.md                  # Original PRD
│   ├── browser-automation-implementation-plan.md  # Original plan
│   ├── browser-automation-prd-revised.md          # Updated PRD
│   ├── browser-automation-revised-plan.md         # Updated implementation plan
│   └── consistency-review.md                      # Document consistency review
└── README.md
```

## Documentation

- **[PRD (Revised)](browser-automation-prd-revised.md)** - Complete product requirements
- **[Implementation Plan (Revised)](browser-automation-revised-plan.md)** - 7-week development roadmap
- **[Consistency Review](consistency-review.md)** - Documentation audit results

## Roadmap

### Version 1.0 (Weeks 1-7) - macOS Desktop App
- ✅ Tauri desktop application
- ✅ Chrome automation via thirtyfour
- ✅ Real-time execution UI
- ✅ Screenshot and text capture
- ✅ YAML script format
- ✅ Output management

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

### Prerequisites

- Rust 1.70+
- Node.js 18+
- macOS 11+ (for v1.0)
- Chrome browser

### Installation

Coming soon - project not yet implemented.

## Development

This project is currently in the planning phase. Implementation will begin after:

1. Document consistency review completion
2. Final PRD approval
3. Team formation and resource allocation

## Contributing

This project is not yet accepting contributions. Please check back after v1.0 release.

## License

TBD

## Contact

Project maintained by the Robert team.

---

**Note**: This is a planning repository. All implementation details are subject to change based on technical feasibility and user feedback.
