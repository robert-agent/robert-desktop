# Browser Automation Tool - Product Requirements Document (Revised)
## Tauri Desktop App for macOS

## Executive Summary

A user-friendly browser automation tool delivered as a **native macOS desktop application** built with Tauri. Users can create, edit, and run automation scripts while observing browser interactions in real-time through an intuitive visual interface. The tool captures screenshots, extracts text, and provides live execution status with comprehensive output management.

## Product Vision

Democratize browser automation by providing a visual, user-friendly desktop application that makes scripted web interactions accessible to both technical and non-technical users. Enable observable automation with real-time feedback, making testing, data collection, and workflow automation simple and reliable.

## Target Users

### Primary Users (v1.0 - macOS Desktop)
- **QA Engineers**: Visual testing and regression detection
- **Product Managers**: Creating user flow documentation
- **Data Analysts**: Collecting structured web data
- **Researchers**: Archiving web content with metadata
- **Designers**: Capturing UI states and screenshots
- **Marketing Teams**: Monitoring competitor websites

### Secondary Users (Future - Linux Headless)
- **DevOps Engineers**: CI/CD integration
- **Backend Developers**: Scheduled automation tasks
- **System Administrators**: Server-based monitoring

## Goals

### Primary Goals (v1.0)
1. ✅ Provide a **native macOS desktop app** with visual UI
2. ✅ Enable **observable browser automation** (visible Chrome window)
3. ✅ Show **real-time execution status** with step-by-step progress
4. ✅ Support **script-driven automation** via YAML
5. ✅ Capture **screenshots and text** during automation
6. ✅ Provide **organized output** with visual file browser
7. ✅ Deliver **user-friendly error handling** and guidance

### Future Goals (Roadmap)
1. 🔄 **Linux headless mode** for server deployments
2. 🔄 **Multi-browser support** (Firefox, Edge, Safari)
3. 🔄 **Cloud execution** and distributed processing
4. 🔄 **Visual script builder** (drag-and-drop)
5. 🔄 **AI-assisted script generation**
6. 🔄 **Team collaboration** features

## Product Scope

### In Scope (v1.0)

#### Platform & Deployment
- ✅ **macOS desktop application** (macOS 11+)
- ✅ **Chrome browser** automation
- ✅ **Local execution** with visible browser
- ✅ **Native .app bundle** with DMG installer

#### User Interface
- ✅ **Tauri-based desktop app** with React frontend
- ✅ **Script editor** with YAML syntax highlighting
- ✅ **Real-time execution dashboard**
- ✅ **Live step progress** and status indicators
- ✅ **Visual output browser** (screenshots, text files)
- ✅ **Settings panel** for configuration
- ✅ **Native macOS integration** (menus, notifications)

#### Automation Features
- ✅ **YAML script format**
- ✅ **Basic navigation** (goto, back, forward, refresh)
- ✅ **Element interactions** (click, type, scroll)
- ✅ **Wait conditions** (element, timeout, page load)
- ✅ **Content capture** (screenshots: viewport/full-page/element)
- ✅ **Text extraction** (page, element-specific)
- ✅ **Metadata collection** (URL, title, timestamp)
- ✅ **Error handling** (continue on error, stop on error)

#### Output & Storage
- ✅ **Structured file output** (screenshots/, text/, html/)
- ✅ **Manifest.json** with execution summary
- ✅ **Preview captured content** in app
- ✅ **Open in Finder** integration

### Out of Scope (v1.0)

#### Deferred to Future Versions
- ❌ **Windows/Linux desktop apps** (v2.0+)
- ❌ **Headless mode** (Linux build, v1.5)
- ❌ **Firefox/Edge/Safari** automation (v2.0+)
- ❌ **Multi-tab/window** concurrent automation (v1.5)
- ❌ **Visual script builder** (v2.5)
- ❌ **Cloud execution** (v3.0)
- ❌ **API access** (REST/GraphQL) (v1.5 with Linux)
- ❌ **Team features** (sharing, versioning) (v3.0)

#### Explicitly Excluded
- ❌ CAPTCHA solving
- ❌ Proxy rotation
- ❌ Anti-detection mechanisms
- ❌ Browser fingerprinting evasion
- ❌ Cryptocurrency/blockchain interactions
- ❌ Social media automation (spam/abuse prevention)

## Requirements

### Functional Requirements

#### FR1: Desktop Application
- **FR1.1**: Launch native macOS application (.app bundle)
- **FR1.2**: Display main window with tabbed interface
- **FR1.3**: Support dark/light mode (system preference)
- **FR1.4**: Provide menu bar with standard macOS menus
- **FR1.5**: Handle window minimize/maximize/close
- **FR1.6**: Persist window size and position

#### FR2: Script Management
- **FR2.1**: Create new YAML scripts in editor
- **FR2.2**: Load scripts from file picker
- **FR2.3**: Save scripts to local filesystem
- **FR2.4**: Validate script syntax with visual feedback
- **FR2.5**: Provide script templates and examples
- **FR2.6**: Syntax highlighting for YAML
- **FR2.7**: Auto-completion for action types

#### FR3: Browser Control
- **FR3.1**: Launch Chrome browser in controlled mode
- **FR3.2**: Auto-detect Chrome installation path
- **FR3.3**: Allow manual Chrome path configuration
- **FR3.4**: Navigate to specified URLs
- **FR3.5**: Execute JavaScript in page context
- **FR3.6**: Wait for page load and element conditions
- **FR3.7**: Handle browser lifecycle (launch, close, cleanup)
- **FR3.8**: Manage chromedriver automatically

#### FR4: Execution Control
- **FR4.1**: Start script execution from UI button
- **FR4.2**: Pause execution mid-script
- **FR4.3**: Stop/cancel running scripts
- **FR4.4**: Resume paused execution
- **FR4.5**: Retry failed steps
- **FR4.6**: Configure error handling behavior

#### FR5: Real-time Status Display
- **FR5.1**: Show execution progress bar
- **FR5.2**: Display current step and total steps
- **FR5.3**: List all steps with status icons (pending/running/success/failed)
- **FR5.4**: Show live logs with timestamps
- **FR5.5**: Display step duration for completed steps
- **FR5.6**: Update UI in real-time during execution
- **FR5.7**: Show current browser URL and page title

#### FR6: User Interactions
- **FR6.1**: Click elements (by selector)
- **FR6.2**: Type text into input fields
- **FR6.3**: Scroll pages (vertical, horizontal, to element)
- **FR6.4**: Select dropdown options
- **FR6.5**: Check/uncheck checkboxes
- **FR6.6**: Select radio buttons
- **FR6.7**: Submit forms
- **FR6.8**: Handle alerts/confirms/prompts

#### FR7: Content Capture
- **FR7.1**: Capture viewport screenshots
- **FR7.2**: Capture full-page screenshots
- **FR7.3**: Capture element-specific screenshots
- **FR7.4**: Extract text content from page or elements
- **FR7.5**: Capture page HTML
- **FR7.6**: Collect page metadata (title, URL, timestamp)
- **FR7.7**: Support PNG and JPEG formats for screenshots

#### FR8: Output Management
- **FR8.1**: Save captured content to organized directories
- **FR8.2**: Generate manifest.json with execution summary
- **FR8.3**: Display output files in visual browser
- **FR8.4**: Preview screenshots within app
- **FR8.5**: View text content within app
- **FR8.6**: Open output directory in Finder
- **FR8.7**: Clear/delete output from previous runs
- **FR8.8**: Export manifest as JSON

#### FR9: Configuration & Settings
- **FR9.1**: Configure Chrome binary path
- **FR9.2**: Set default timeout values
- **FR9.3**: Choose output directory location
- **FR9.4**: Select theme (light/dark/system)
- **FR9.5**: Configure keyboard shortcuts
- **FR9.6**: Adjust logging verbosity
- **FR9.7**: Persist settings across sessions

#### FR10: Help & Documentation
- **FR10.1**: In-app help documentation
- **FR10.2**: Example scripts library
- **FR10.3**: Action reference guide
- **FR10.4**: Tooltips for UI elements
- **FR10.5**: Error message explanations
- **FR10.6**: Onboarding tutorial (first launch)

### Non-Functional Requirements

#### NFR1: Performance
- **NFR1.1**: App launch < 3 seconds
- **NFR1.2**: Script load/parse < 500ms
- **NFR1.3**: Browser launch < 5 seconds
- **NFR1.4**: Screenshot capture < 2 seconds
- **NFR1.5**: UI responsiveness < 100ms for interactions
- **NFR1.6**: Memory footprint < 500MB (app + browser combined < 1.5GB)
- **NFR1.7**: Smooth 60fps UI animations

#### NFR2: Reliability
- **NFR2.1**: 99% uptime for local execution
- **NFR2.2**: Graceful handling of network timeouts
- **NFR2.3**: Recovery from page load failures
- **NFR2.4**: Proper cleanup of browser processes
- **NFR2.5**: No zombie processes after app quit
- **NFR2.6**: Crash resistance with automatic recovery
- **NFR2.7**: Data persistence (no loss on crash)

#### NFR3: Usability
- **NFR3.1**: Intuitive UI (no training required for basic use)
- **NFR3.2**: Clear, actionable error messages
- **NFR3.3**: Visual consistency with macOS design guidelines
- **NFR3.4**: Keyboard shortcuts for common actions
- **NFR3.5**: Drag-and-drop support for script files
- **NFR3.6**: Search/filter in logs and output files
- **NFR3.7**: Undo/redo in script editor

#### NFR4: Compatibility
- **NFR4.1**: macOS 11 Big Sur and later
- **NFR4.2**: Chrome 100+ (latest 3 major versions)
- **NFR4.3**: Both Intel and Apple Silicon Macs
- **NFR4.4**: Screen resolutions from 1280x800 to 5K

#### NFR5: Security & Privacy
- **NFR5.1**: No telemetry or data collection (opt-in only)
- **NFR5.2**: Local-only execution (no cloud required)
- **NFR5.3**: Secure storage of sensitive data (keychain integration)
- **NFR5.4**: Sandboxed browser contexts
- **NFR5.5**: No credential harvesting or malicious use

#### NFR6: Maintainability
- **NFR6.1**: Modular architecture
- **NFR6.2**: Comprehensive logging
- **NFR6.3**: Unit test coverage > 70%
- **NFR6.4**: Integration tests for core flows
- **NFR6.5**: Automated CI/CD pipeline
- **NFR6.6**: Documentation for developers

#### NFR7: Distribution
- **NFR7.1**: Code-signed macOS application
- **NFR7.2**: Notarized for Gatekeeper
- **NFR7.3**: DMG installer with drag-to-Applications
- **NFR7.4**: App bundle size < 50MB (excluding dependencies)
- **NFR7.5**: Automatic update mechanism
- **NFR7.6**: Release notes for each version

## User Stories

### US1: First Launch Experience
**As a** new user
**I want to** quickly understand how to create my first automation
**So that** I can be productive immediately

**Acceptance Criteria:**
- App launches with welcome screen
- Onboarding tutorial explains key concepts
- Example scripts are pre-loaded
- One-click to run example
- Chrome is auto-detected or user is prompted

**UI Flow:**
```
Launch App → Welcome Screen → Example Selection → Run Example → View Results
```

### US2: Script Creation and Editing
**As a** QA engineer
**I want to** create and edit automation scripts visually
**So that** I don't need to remember syntax

**Acceptance Criteria:**
- Script editor with syntax highlighting
- Auto-completion for actions
- Real-time validation with error indicators
- Save/load from file system
- Templates for common patterns

**UI Flow:**
```
New Script → Choose Template → Edit in Editor → Validate → Save
```

### US3: Running Automation with Visual Feedback
**As a** product manager
**I want to** see automation progress in real-time
**So that** I can verify it's working correctly

**Acceptance Criteria:**
- Click "Run" button to start
- Browser window opens visibly
- Execution tab shows live progress
- Each step highlighted as it runs
- Logs stream in real-time
- Screenshots appear as captured

**UI Flow:**
```
Load Script → Click Run → View Execution Tab → Monitor Progress → Review Output
```

### US4: Capturing User Flows
**As a** designer
**I want to** capture screenshots of multi-step user flows
**So that** I can document the current state

**Acceptance Criteria:**
- Script navigates through flow
- Screenshot after each major step
- Full-page captures for documentation
- Organized output with numbered files
- Preview screenshots in app
- Export to folder for sharing

**UI Flow:**
```
Create Flow Script → Add Screenshot Actions → Run → View Screenshots → Open in Finder
```

### US5: Form Interaction Testing
**As a** QA engineer
**I want to** automate form filling and submission
**So that** I can test validation rules

**Acceptance Criteria:**
- Script fills all form fields (text, select, checkbox, radio)
- Submits form
- Captures result page
- Extracts success/error messages
- Handles validation errors gracefully

**UI Flow:**
```
Define Form Steps → Configure Field Values → Add Validation → Run Test → Check Results
```

### US6: Data Extraction
**As a** data analyst
**I want to** extract text from specific page sections
**So that** I can analyze content

**Acceptance Criteria:**
- Script specifies CSS selectors
- Text extracted and saved to files
- Structured format (JSON/CSV option)
- Preview extracted data in app
- Metadata includes timestamps

**UI Flow:**
```
Navigate to Page → Define Selectors → Extract Text → View in App → Export Data
```

### US7: Error Handling and Recovery
**As a** user
**I want to** handle errors gracefully during automation
**So that** partial results are still captured

**Acceptance Criteria:**
- Failed steps highlighted in red
- Clear error messages with context
- Option to retry failed step
- Option to continue or stop
- Partial outputs still saved
- Summary report shows success/failure counts

**UI Flow:**
```
Run Script → Step Fails → View Error → Choose Action (Retry/Continue/Stop)
```

### US8: Output Management
**As a** researcher
**I want to** browse and manage captured outputs
**So that** I can find and use the data

**Acceptance Criteria:**
- Output tab shows all captured files
- Thumbnails for screenshots
- Text file previews
- Search/filter by filename or date
- Open individual files or entire folder
- Delete old runs

**UI Flow:**
```
Complete Execution → Switch to Output Tab → Browse Files → Preview → Open in Finder
```

### US9: Configuration and Customization
**As a** power user
**I want to** customize app behavior
**So that** it fits my workflow

**Acceptance Criteria:**
- Settings panel accessible from menu
- Configure Chrome path
- Adjust default timeouts
- Choose output directory
- Set theme preference
- Define keyboard shortcuts
- Settings persist across launches

**UI Flow:**
```
Open Settings → Adjust Preferences → Save → Settings Applied
```

### US10: Script Sharing
**As a** team lead
**I want to** share scripts with my team
**So that** everyone can run the same automation

**Acceptance Criteria:**
- Export script as YAML file
- Include dependencies/requirements
- Add comments/documentation
- Share via file, email, or repo
- Import and run on another machine

**UI Flow:**
```
Create Script → Add Documentation → Export → Share File → Teammate Imports → Runs Successfully
```

## Script Format Specification

### YAML Structure

```yaml
# Script metadata
name: "Example Automation"
version: "1.0.0"
description: "Demonstrates basic navigation and capture"
author: "Your Name"
output_dir: "./output/example"

# Browser configuration
browser:
  type: "chrome"
  window_size: [1280, 1024]
  headless: false  # Always false for v1.0 desktop

# Global settings
settings:
  default_timeout: 30000  # milliseconds
  screenshot_format: "png"
  on_error: "stop"  # stop | continue | retry

# Automation steps
steps:
  - action: navigate
    url: "https://example.com"
    wait_for: "dom_content_loaded"

  - action: wait
    condition: "element"
    selector: "h1"
    timeout: 5000

  - action: screenshot
    type: "full_page"
    filename: "homepage.png"

  - action: click
    selector: "button#cta"
    wait_after: 1000

  - action: type
    selector: "input[name='email']"
    text: "user@example.com"
    clear_first: true

  - action: extract_text
    selector: ".results"
    output: "results.txt"

  - action: scroll
    direction: "down"
    amount: 500

  - action: execute
    javascript: "return document.title;"
    variable: "page_title"

  - action: conditional
    condition:
      type: "element_exists"
      selector: ".error"
    then:
      - action: screenshot
        type: "element"
        selector: ".error"
        filename: "error.png"
    else:
      - action: extract_text
        selector: ".success"
        output: "success.txt"
```

### Action Reference

| Action | Description | Required Parameters | Optional Parameters |
|--------|-------------|---------------------|---------------------|
| `navigate` | Navigate to URL | `url` | `wait_for`, `timeout` |
| `click` | Click element | `selector` | `wait_after`, `timeout` |
| `type` | Type text | `selector`, `text` | `clear_first`, `delay` |
| `scroll` | Scroll page | `direction`, `amount` | `selector` (scroll to) |
| `wait` | Wait for condition | `condition`, `selector` | `timeout` |
| `screenshot` | Capture screenshot | `type`, `filename` | `selector` (for element), `format` |
| `extract_text` | Extract text | `selector`, `output` | `all` (get all matches) |
| `execute` | Run JavaScript | `javascript` | `variable` (store result) |
| `select` | Select dropdown | `selector`, `value` | `by` (value, text, index) |
| `check` | Check checkbox | `selector` | `state` (true/false) |
| `submit` | Submit form | `selector` | - |
| `conditional` | Conditional execution | `condition`, `then` | `else` |

## Output Format Specification

### Directory Structure

```
output/
├── run_2025-10-08_10-30-45_abc123/
│   ├── manifest.json
│   ├── screenshots/
│   │   ├── 001_homepage.png
│   │   ├── 002_results.png
│   │   └── 003_error.png
│   ├── text/
│   │   ├── 001_homepage.txt
│   │   └── 002_results.txt
│   ├── html/
│   │   └── 001_homepage.html
│   └── logs/
│       └── execution.log
└── run_2025-10-08_11-15-22_def456/
    └── ...
```

### Manifest Format

```json
{
  "run_id": "abc123-def456-789",
  "script_name": "Example Automation",
  "script_version": "1.0.0",
  "timestamp_start": "2025-10-08T10:30:45Z",
  "timestamp_end": "2025-10-08T10:31:15Z",
  "duration_ms": 30000,
  "status": "completed",
  "browser": {
    "type": "chrome",
    "version": "120.0.6099.109",
    "user_agent": "Mozilla/5.0..."
  },
  "steps": [
    {
      "step_number": 1,
      "action": "navigate",
      "parameters": {
        "url": "https://example.com"
      },
      "status": "success",
      "timestamp": "2025-10-08T10:30:46Z",
      "duration_ms": 1234,
      "outputs": []
    },
    {
      "step_number": 2,
      "action": "screenshot",
      "parameters": {
        "type": "full_page",
        "filename": "homepage.png"
      },
      "status": "success",
      "timestamp": "2025-10-08T10:30:48Z",
      "duration_ms": 456,
      "outputs": [
        {
          "type": "file",
          "path": "screenshots/001_homepage.png",
          "size_bytes": 234567
        }
      ]
    }
  ],
  "summary": {
    "total_steps": 5,
    "successful_steps": 4,
    "failed_steps": 1,
    "skipped_steps": 0
  },
  "errors": [
    {
      "step_number": 4,
      "timestamp": "2025-10-08T10:31:10Z",
      "error_type": "ElementNotFound",
      "message": "Could not find element: .missing-selector",
      "recoverable": true,
      "action_taken": "continued"
    }
  ],
  "warnings": [
    "Timeout value lower than recommended for network requests"
  ],
  "system_info": {
    "os": "macOS 14.0",
    "app_version": "1.0.0"
  }
}
```

## Success Metrics

### Adoption Metrics (3 months post-launch)
- 500+ downloads
- 100+ active users (weekly)
- 50+ scripts created by community
- 4.0+ average rating

### Performance Metrics
- 95% of operations complete within expected time
- <1% crash rate
- 90% of executions succeed without manual intervention

### User Satisfaction
- 80%+ positive feedback
- <24 hour average support response time
- 70%+ retention (monthly active users)

### Technical Metrics
- 99% uptime (local execution reliability)
- <100MB memory footprint (app only)
- <50ms UI latency

## Risks and Mitigations

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **Chrome not found on system** | High | Medium | Auto-detection with fallback to manual path config |
| **chromedriver version mismatch** | High | Medium | Auto-download compatible driver on first launch |
| **Tauri learning curve for developers** | Medium | Medium | Extensive examples, documentation, active community |
| **macOS code signing complexity** | High | High | Early setup of certificates, CI automation |
| **Notarization delays** | Medium | Medium | Automated notarization in CI/CD pipeline |
| **Chrome updates breaking automation** | High | Medium | Support latest 3 Chrome versions, thirtyfour updates |
| **Memory leaks in long-running scripts** | Medium | Low | Thorough testing, proper cleanup, memory profiling |
| **User expectations for unsupported features** | Low | High | Clear documentation of scope, roadmap visibility |
| **Competition from established tools** | Medium | High | Focus on UX, macOS-native feel, ease of use |

## Roadmap

### Version 1.0 (Weeks 1-7) - **macOS Desktop App**
- ✅ Tauri desktop application
- ✅ Chrome automation via thirtyfour
- ✅ Real-time execution UI
- ✅ Screenshot and text capture
- ✅ YAML script format
- ✅ Output management

### Version 1.5 (Month 3-4) - **Linux Headless**
- 🔄 Linux CLI binary
- 🔄 Headless Chrome support
- 🔄 Docker container
- 🔄 REST API for remote control
- 🔄 Job queue and scheduler

### Version 2.0 (Month 6-8) - **Multi-Browser**
- 🔄 Firefox support (geckodriver)
- 🔄 Edge support (msedgedriver)
- 🔄 Safari support (macOS only)
- 🔄 Windows desktop app
- 🔄 Multi-tab/window automation

### Version 2.5 (Month 9-10) - **Visual Builder**
- 🔄 Drag-and-drop script builder
- 🔄 Record browser interactions
- 🔄 Visual selector picker
- 🔄 Flow diagram view

### Version 3.0 (Month 12+) - **Cloud & Collaboration**
- 🔄 Cloud execution platform
- 🔄 Team workspaces
- 🔄 Script sharing and marketplace
- 🔄 Scheduled runs
- 🔄 Advanced reporting and analytics

## Competitive Analysis

| Feature | Our Tool (v1.0) | Selenium IDE | Playwright Inspector | Puppeteer Recorder |
|---------|-----------------|--------------|----------------------|--------------------|
| **Platform** | macOS native app | Browser extension | Node.js CLI | Chrome DevTools |
| **UI Quality** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **Real-time Status** | ✅ Visual dashboard | ❌ Basic logs | ❌ Console only | ❌ Console only |
| **Script Format** | YAML (simple) | Selenium format | JavaScript/TS | JavaScript |
| **Multi-Browser** | Chrome only (v1) | ✅ All browsers | ✅ Chrome/FF/WebKit | Chrome only |
| **Ease of Use** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Output Management** | ✅ Visual browser | ❌ Manual | ❌ Manual | ❌ Manual |
| **macOS Native** | ✅ Tauri app | ❌ | ❌ | ❌ |

**Unique Selling Points:**
1. 🎨 **Beautiful native macOS UI** - Feels like a Mac app
2. 👁️ **Real-time visual feedback** - See automation as it happens
3. 📝 **Simple YAML scripts** - Non-programmers can create automation
4. 📦 **Output management built-in** - Browse, preview, export easily
5. 🚀 **Fast and lightweight** - Tauri keeps it small and fast

## Technical Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                   Tauri Desktop App                     │
│                                                          │
│  ┌────────────────────┐      ┌────────────────────┐    │
│  │   React Frontend   │ IPC  │   Rust Backend     │    │
│  │                    │◄────►│                    │    │
│  │  - Script Editor   │      │  - Browser Driver  │    │
│  │  - Execution UI    │      │  - Script Executor │    │
│  │  - Output Browser  │      │  - Capture Engine  │    │
│  │  - Settings Panel  │      │  - Storage Manager │    │
│  └────────────────────┘      └──────────┬─────────┘    │
│                                          │              │
└──────────────────────────────────────────┼──────────────┘
                                           │
                                           │ WebDriver Protocol
                                           │
                                    ┌──────▼────────┐
                                    │  chromedriver │
                                    └──────┬────────┘
                                           │ Chrome DevTools Protocol
                                           │
                                    ┌──────▼────────┐
                                    │ Chrome Browser│
                                    │   (Visible)   │
                                    └───────────────┘
```

## Dependencies

### External
- **Chrome browser** - User installation (auto-detected)
- **chromedriver** - Auto-downloaded by app

### Development Tools
- **Rust** 1.70+
- **Node.js** 18+ (for frontend build)
- **Xcode Command Line Tools** (for macOS builds)
- **Apple Developer ID** (for code signing)

## Appendix

### Technology Stack Summary
- **Desktop Framework**: Tauri 2.0
- **Frontend**: Svelte + TypeScript + Tailwind CSS
- **Backend**: Rust 1.70+
- **Browser Automation**: thirtyfour (WebDriver)
- **Async Runtime**: tokio
- **Script Format**: YAML (serde_yaml)
- **Build System**: Cargo + Vite

### Reference Links
- Tauri: https://v2.tauri.app/
- thirtyfour: https://github.com/Vrtgs/thirtyfour
- Chrome DevTools Protocol: https://chromedevtools.github.io/devtools-protocol/
- WebDriver W3C Spec: https://www.w3.org/TR/webdriver/

### Glossary
- **Tauri**: Desktop app framework using Rust backend and web frontend
- **WebDriver**: W3C standard for browser automation
- **thirtyfour**: Rust WebDriver client library
- **chromedriver**: WebDriver implementation for Chrome
- **IPC**: Inter-Process Communication (between Tauri frontend/backend)
- **DMG**: macOS disk image for app distribution
- **Notarization**: Apple's security verification for macOS apps

---

**Document Version**: 2.0
**Last Updated**: 2025-10-08
**Status**: Approved for Development
**Target Release**: v1.0 - Q1 2026
