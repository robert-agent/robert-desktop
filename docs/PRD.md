# Robert - Automation for Everyone
## Product Requirements Document

## Executive Summary

**Robert** is an open-source, local-first browser automation tool that brings the power of automation to everyone—not just programmers. While tools like Zapier and IFTTT require API knowledge, Claude agents are complex to create, and GPT's interface provides no visibility, Robert lets users **watch automation happen in real-time**, learn by observation, and maintain full control with the ability to abort operations at any moment.

Built with eye-candy visual feedback and delivered as a native desktop application, Robert runs entirely on your device with optional cloud inference. Unlike centralized alternatives (Herd, Monitoro) that require programming knowledge and lock you into their platforms, Robert is free, open, and designed for **visual learners who want to see their automation work**.

## Product Vision

**Automation for everyone, powered by local execution and visual feedback.**

We believe automation should be:
- **Visual**: Watch automation happen, don't just hope it works
- **Learnable**: See what's happening, understand by observation
- **Controllable**: Abort operations, pause, inspect state
- **Local-first**: Run on your device, own your data and privacy
- **Open**: Free, auditable, community-driven
- **Beautiful**: Eye-candy UI that makes automation delightful

### The Problem

**Current automation tools have critical flaws:**

1. **API-based tools (Zapier, IFTTT)** require understanding APIs, webhooks, and integration points
2. **AI agents (Claude, GPT)** are black boxes—you don't see what they're doing until it's done (or wrong)
3. **Programming tools (Playwright, Selenium)** require coding skills and test-runner mindsets
4. **Centralized platforms (Herd, Monitoro)** lock you into their ecosystem and require programming knowledge

**What users actually need:**
- Visual confirmation that automation is working
- Ability to learn by watching
- Control to stop operations mid-flight
- Privacy through local execution
- Freedom from vendor lock-in

### Our Solution

**Browser automation as the universal interface:**
- The browser is the universal app—everything happens there
- Visual feedback shows exactly what's happening
- Users can watch, learn, and build confidence
- Local execution keeps data private and secure
- Open source ensures freedom and transparency

### Competitive Landscape

**Tools like Herd and Monitoro** are moving in the right direction with visual feedback and eye-candy interfaces. However, they:
- ❌ Require programming/test-runner knowledge
- ❌ Run on centralized infrastructure
- ❌ Lock users into proprietary platforms
- ❌ Charge for what should be free

**Robert's differentiators:**
- ✅ **Voice-driven creation** - Talk through your automation, we write it for you
- ✅ **Markdown scripts** - Readable format like Claude agents, not scary YAML
- ✅ **Local-first execution** - Runs on your device, your control
- ✅ **Open source** - Free, auditable, community-owned
- ✅ **Optional cloud** - Send inference to cloud when you choose
- ✅ **Visual feedback** - Watch automation happen in real-time
- ✅ **Eye-candy UI** - Beautiful, delightful, confidence-inspiring

### Vision Statement

> "Make automation as easy as watching a recording, as powerful as programming, and as trustworthy as doing it yourself—all while running on your own device."

## Target Users

### Primary Users - "Automation Curious"
**Non-programmers who need automation but lack technical skills:**
- **Small business owners**: Automate repetitive web tasks (data entry, monitoring)
- **Content creators**: Schedule social media, monitor analytics, capture content
- **Researchers**: Collect data from websites without coding
- **Journalists**: Track news sites, archive content, monitor changes
- **Students**: Automate research workflows, collect academic data
- **Marketers**: Monitor competitors, track campaigns, capture screenshots

**Key characteristics:**
- Visual learners who need to see automation work
- Want control and transparency (not black-box AI)
- Value privacy and local execution
- Can talk through what they want, no need to learn syntax
- Need immediate feedback and error visibility

### Secondary Users - "Power Users"
**Technical users who want better tools:**
- **QA Engineers**: Visual testing without test-runner complexity
- **Product Managers**: Document user flows without engineering help
- **Designers**: Capture UI states across workflows
- **Data Analysts**: Extract web data without scraping frameworks
- **DevOps Engineers**: CI/CD integration (future headless mode)

**Key characteristics:**
- Comfortable with technical tools but want better UX
- Appreciate visual feedback even though they can code
- Value open source and local execution
- Need reliable, auditable automation

## Goals

### Primary Goals (v1.0) - "Watch It Work"
1. ✅ **Voice-driven agent creator** - Talk through your automation, AI writes it
2. ✅ **Markdown scripts** - Human-readable format (like Claude agents), not code
3. ✅ **Real-time visual feedback** - Watch browser automation happen with eye-candy UI
4. ✅ **Local-first execution** - Runs on your device, complete privacy
5. ✅ **Full control** - Pause, abort, inspect state at any moment
6. ✅ **Beautiful UI** - Eye-candy that makes automation delightful
7. ✅ **Open source** - Free, auditable, community-owned

### Secondary Goals (v1.0)
1. ✅ **Observable browser** - Visible Chrome window showing automation
2. ✅ **Step-by-step progress** - See exactly where automation is
3. ✅ **Screenshot & text capture** - Collect data visually
4. ✅ **Organized output** - Visual file browser for results
5. ✅ **User-friendly errors** - Learn from failures with clear guidance

### Future Goals (Roadmap)
1. 🔄 **Voice-driven script creation** - Talk naturally, AI writes Markdown (local-first, optional cloud)
2. 🔄 **Visual script builder** - Drag-and-drop for non-technical users
3. 🔄 **Record & replay** - Watch once, automate forever
4. 🔄 **Community script library** - Share and discover automations
5. 🔄 **Workflow learning system** - AI agents learn and improve navigation paths automatically
6. 🔄 **Linux headless mode** - CI/CD and server deployments
7. 🔄 **Multi-browser support** - Firefox, Edge, Safari
8. 🔄 **Optional cloud inference** - Send LLM requests to cloud when desired

## Product Scope

### In Scope (v1.0)

#### Platform & Deployment
- ✅ **macOS desktop application** (macOS 11+)
- ✅ **Chrome browser** automation
- ✅ **Local execution** with visible browser
- ✅ **Native .app bundle** with DMG installer

#### User Interface
- ✅ **Tauri-based desktop app** with Svelte frontend
- ✅ **Script editor** with Markdown syntax highlighting
- ✅ **Real-time execution dashboard**
- ✅ **Live step progress** and status indicators
- ✅ **Visual output browser** (screenshots, text files)
- ✅ **Settings panel** for configuration
- ✅ **Native macOS integration** (menus, notifications)

#### Automation Features
- ✅ **Claude-generated CDP scripts** - AI writes Chrome DevTools Protocol commands as text
- ✅ **Runtime script interpretation** - Scripts stored externally, not compiled into binary
- ✅ **Direct CDP execution** - Full Chrome automation capabilities via CDP protocol
- ✅ **Basic navigation** (goto, back, forward, refresh)
- ✅ **Element interactions** (click, type, scroll)
- ✅ **Wait conditions** (element, timeout, page load)
- ✅ **Content capture** (screenshots: viewport/full-page/element)
- ✅ **Text extraction** (page, element-specific)
- ✅ **Metadata collection** (URL, title, timestamp)
- ✅ **Error handling** (continue on error, stop on error)
- ✅ **Advanced CDP commands** - Any CDP command supported by Chrome

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
- **FR2.1**: Create new Markdown scripts via voice or editor
- **FR2.2**: Load scripts from file picker
- **FR2.3**: Save scripts to local filesystem
- **FR2.4**: Validate script syntax with visual feedback
- **FR2.5**: Provide script templates and examples
- **FR2.6**: Syntax highlighting for Markdown
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
- **NFR1.6**: Memory footprint < 100MB (app only), app + browser combined < 1.5GB
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
- **NFR5.1**: Local-first inference (100% on-device by default)
- **NFR5.2**: No telemetry or data collection (opt-in only)
- **NFR5.3**: Optional cloud inference with automatic data obfuscation
- **NFR5.4**: Multi-layer protection (text + image obfuscation)
- **NFR5.5**: Secure storage of sensitive data (macOS Keychain integration)
- **NFR5.6**: Sandboxed browser contexts
- **NFR5.7**: Audit logging for cloud data transmission
- **NFR5.8**: No credential harvesting or malicious use

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
- Export script as Markdown file
- Include dependencies/requirements
- Add comments/documentation
- Share via file, email, or repo
- Import and run on another machine

**UI Flow:**
```
Create Script → Add Documentation → Export → Share File → Teammate Imports → Runs Successfully
```

## Script Format Specification

**Note:** See [SCRIPT_FORMAT.md](SCRIPT_FORMAT.md) for full specification.

### Claude-Generated CDP Scripts

Robert uses **AI-generated Chrome DevTools Protocol (CDP) commands** for browser automation. Claude analyzes your natural language requests and generates CDP command sequences as text strings.

**Key Architecture Decisions:**
- **Scripts are NOT compiled** - CDP commands stored as text/JSON, interpreted at runtime
- **Claude generates the scripts** - User describes task in natural language, Claude outputs CDP commands
- **Direct CDP execution** - Runtime interpreter sends commands directly to Chrome via chromiumoxide
- **No code in binary** - All browsing logic external to compiled application

### Example Claude-Generated Script

**User Request:** "Navigate to example.com, take a screenshot, and click the login button"

**Claude Generates:**
```json
{
  "name": "example-login-automation",
  "description": "Navigate and click login",
  "cdp_commands": [
    {
      "method": "Page.navigate",
      "params": {
        "url": "https://example.com"
      }
    },
    {
      "method": "Page.captureScreenshot",
      "params": {
        "format": "png",
        "captureBeyondViewport": true
      },
      "save_as": "homepage.png"
    },
    {
      "method": "Runtime.evaluate",
      "params": {
        "expression": "document.querySelector('button#login').click()"
      }
    }
  ]
}
```

### CDP Command Examples

Common CDP commands Claude can generate:

| CDP Method | Purpose | Example Params |
|------------|---------|----------------|
| `Page.navigate` | Navigate to URL | `{"url": "https://example.com"}` |
| `Page.captureScreenshot` | Take screenshot | `{"format": "png", "captureBeyondViewport": true}` |
| `Runtime.evaluate` | Execute JavaScript | `{"expression": "document.title"}` |
| `Input.dispatchMouseEvent` | Click elements | `{"type": "mousePressed", "x": 100, "y": 200}` |
| `Input.dispatchKeyEvent` | Type text | `{"type": "char", "text": "hello"}` |
| `Network.getCookies` | Get cookies | `{}` |
| `DOM.getDocument` | Get DOM tree | `{}` |
| `Emulation.setGeolocationOverride` | Set location | `{"latitude": 37.7749, "longitude": -122.4194}` |

See [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/) for full reference.

## Workflow Learning System (Future Feature)

### Overview

The **Workflow Learning System** enables AI agents to learn, document, and iteratively improve website navigation workflows without broad exploration. Instead of exploring a website from scratch each time, agents reference standardized workflow files that contain proven navigation paths with empirical confidence scores.

**Problem Solved:**
- Traditional AI agents waste time exploring websites repeatedly
- No shared knowledge between agent instances
- Brittle selectors break when sites update
- No learning from experience or failures

**Solution:**
Two complementary file formats that agents create and improve autonomously:

### Format 1: Workflow Graph (`.workflow.md`)

**Purpose**: High-level navigation roadmap showing how to navigate from A→B

**Structure**:
- **Nodes**: Pages, buttons, forms, modals with selectors and URL patterns
- **Edges**: Actions (click, type, navigate) with wait conditions and success indicators
- **Confidence scores**: Based on empirical success rates from actual executions
- **Error recovery**: Documented strategies for common failures
- **Alternative paths**: Multiple proven ways to achieve the same goal
- **Mermaid diagrams**: Visual workflow representation

**Example**:
```markdown
---
domain: github.com
workflow_name: create_repository
version: 1.0.0
success_rate: 0.98
tested_sessions: 45
---

## Edge: github_home → new_repo_button
**Action**: click
**Selector**: `[data-test-selector="global-create-menu-button"]`
**Confidence**: 0.98
**Success Indicators**: Dropdown menu becomes visible
**Alternative Selectors**: ["button[aria-label='Create new...']"]
```

### Format 2: Step Frame (`.frames.json`)

**Purpose**: Detailed execution traces capturing every moment of a workflow session

**Each frame contains**:
- **Screenshot**: Visual state saved as PNG file
- **DOM snapshot**: Complete page structure as HTML
- **Interactive elements**: All clickable/typeable elements with bounding boxes
- **Action metadata**: What action was taken, target selector, input data
- **Natural language transcript**: Human-readable description, reasoning, expected outcome
- **State changes**: URL changes, network requests, DOM mutations
- **Verification**: Success/failure indicators
- **Learning data**: Selector stability scores, alternative selectors discovered

**Example**:
```json
{
  "frames": [
    {
      "frame_id": 0,
      "screenshot": {"path": "./screenshots/frame_0000.png"},
      "action": {
        "type": "click",
        "target": {"selector": "[data-test-selector='create-button']"},
        "intent": "Open the create menu"
      },
      "transcript": {
        "action_description": "Clicking the '+' button to open create menu",
        "reasoning": "Standard entry point for creating items on GitHub",
        "expected_outcome": "Dropdown menu should appear"
      },
      "learning": {
        "selector_stability": 0.98,
        "action_reliability": 0.96
      }
    }
  ]
}
```

### How It Works

**1. Recording Phase**
- Agent executes a workflow while capturing detailed frames
- Each frame includes screenshot, DOM state, action, and transcript
- Session saved as `.frames.json` with all evidence

**2. Learning Phase**
- Agent analyzes multiple session frames
- Calculates selector stability and confidence scores
- Extracts nodes, edges, and alternative paths
- Creates/updates `.workflow.md` with learned knowledge

**3. Execution Phase**
- Agent loads `.workflow.md` before executing
- Follows highest-confidence path
- Records new session while executing
- Updates confidence scores based on results
- Continuous improvement loop

### Key Features

**🎯 Confidence-Based Navigation**
- Every selector has empirical success rate (0.0-1.0)
- Agent automatically chooses most reliable selectors
- Falls back to alternatives when primary fails
- Example: `data-test-selector` (0.98) preferred over class-based (0.87)

**🔄 Self-Improving System**
```
Session 1:   60% success → discovering the workflow
Session 10:  85% success → learning reliable selectors
Session 50:  94% success → optimized with error recovery
Session 100: 97% success → mature, production-ready
```

**🛡️ Built-In Error Recovery**
- Documents common failure scenarios
- Proven recovery strategies included
- Automatic fallback to alternative paths
- Example: Rate limit → Wait 60s → Retry

**🤝 Multi-Agent Knowledge Sharing**
- Multiple agents contribute to same workflow
- Merge alternative selectors and strategies
- Weighted confidence score averaging
- Version control for workflow evolution

### File Organization

```
workflows/
├── github.com/
│   ├── create_repository/
│   │   ├── github.com_create_repository_v1.workflow.md    # Graph
│   │   ├── session_abc123/
│   │   │   ├── *.frames.json                              # Frames
│   │   │   ├── screenshots/frame_*.png
│   │   │   └── dom/frame_*.html
│   │   └── session_xyz789/
│   └── create_issue/
└── gmail.com/
    └── compose_email/
```

### Agent Integration

Agents use this system to:
1. **Execute workflows** using proven paths from `.workflow.md`
2. **Record sessions** capturing frames for learning
3. **Analyze patterns** to improve selector choices
4. **Update graphs** with new confidence scores
5. **Share knowledge** by committing updated workflows
6. **Recover from errors** using documented strategies

### Benefits

**For Agents:**
- ✅ No exploration needed - follow proven paths
- ✅ Higher reliability with empirical confidence
- ✅ Automatic error recovery
- ✅ Learn from every execution
- ✅ Share knowledge across instances

**For Users:**
- ✅ Faster automation setup
- ✅ More reliable workflows
- ✅ Transparent agent behavior (view frames/graphs)
- ✅ Workflows improve over time
- ✅ Community can share workflow knowledge

### Documentation

Complete specifications available in:
- [agent-formats/specs/WORKFLOW_GRAPH_SCHEMA.md](../agent-formats/specs/WORKFLOW_GRAPH_SCHEMA.md) - Graph format spec
- [agent-formats/specs/STEP_FRAME_SCHEMA.md](../agent-formats/specs/STEP_FRAME_SCHEMA.md) - Frame format spec
- [agent-formats/specs/AGENT_WORKFLOW_STANDARDS.md](../agent-formats/specs/AGENT_WORKFLOW_STANDARDS.md) - Integration guide
- [agent-formats/README.md](../agent-formats/README.md) - Overview and examples

**Note**: This feature is planned for a future release. The format specifications are complete and ready for implementation when AI agent capabilities are integrated into Robert.

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

## Safety & Privacy Features

### Local-First Inference

**Default: 100% Local Execution**
- All AI inference runs on-device (voice-to-script, planning, suggestions)
- Zero data sent to cloud by default
- Complete privacy and offline capability
- Uses Mac's Neural Engine when available
- Fast, efficient quantized models

**Benefits:**
- Complete privacy - your automation workflows never leave your device
- No API keys required
- No usage tracking or data retention
- Works offline
- Zero ongoing costs

### Optional Cloud Inference

**Advanced Feature (Opt-In Only):**
- Users can choose to use cloud AI (GPT-4, Claude, etc.) for more powerful models
- Clearly opt-in with informed consent
- Never required - local inference always available

**When Enabled: Automatic Safety**

### Multi-Layer Data Protection

**1. Text Obfuscation**
- Automatic detection and redaction of sensitive data:
  - Passwords and API keys
  - Credit card numbers and CVV codes
  - Social Security Numbers (SSN)
  - Personal identifiable information (PII)
  - Private keys and credentials
- Pattern matching with context awareness
- Tokenization (sensitive data stored locally, tokens sent to cloud)
- Original data never leaves your device

**Example:**
```
Original: "Login with password SuperSecret123"
Sent to cloud: "Login with password [PASSWORD_REDACTED_8CHARS]"
Cloud response: "Fill input#password with [PASSWORD_REDACTED_8CHARS]"
Local execution: Robert replaces token with actual password
```

**2. Image Obfuscation**
- Screenshots and page captures are scanned before cloud transmission
- Computer vision detects sensitive visual content:
  - Password fields (visible or masked)
  - Credit card forms
  - Personal information in documents
  - Private messages or emails
- Automatic blurring or pixelation of sensitive regions
- OCR + pattern matching for text in images

**Example:**
```
Screenshot contains login form:
- Email field: "john@example.com" → Blurred
- Password field: "••••••••" → Blurred region
- Only safe layout information sent to cloud
```

**3. User Controls**
- Sensitivity levels (Maximum/Balanced/Minimal)
- Manual review before sending to cloud
- Per-automation opt-in/out
- Audit log of what was sent where

**4. Encryption**
- Sensitive data encrypted in macOS Keychain
- TLS 1.3 for cloud API calls
- Certificate pinning
- Hardware-backed keys (Secure Enclave)

### Privacy Guarantees

**What We Never Do:**
- ❌ Send data to cloud by default
- ❌ Store sensitive data long-term
- ❌ Log passwords or API keys
- ❌ Share data with third parties
- ❌ Train AI models on your data
- ❌ Track usage without consent

**What We Always Do:**
- ✅ Local inference by default
- ✅ Obfuscate before any cloud transmission
- ✅ Encrypt sensitive data
- ✅ Give users full control
- ✅ Maintain audit logs (local only)
- ✅ Open source for auditability

### Compliance
- GDPR compliant (data minimization, user consent)
- CCPA compliant (consumer rights, opt-out)
- SOC 2 principles (security, privacy, audit)

**See [SAFETY_PRIVACY.md](SAFETY_PRIVACY.md) for complete technical details.**

---

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

### Privacy Metrics
- 95%+ users comfortable with privacy approach
- <1% opt-in to cloud inference initially (grows with trust)
- Zero data breaches or privacy incidents

### Technical Metrics
- 99% uptime (local execution reliability)
- <100MB memory footprint (app only), <1.5GB with browser
- <50ms UI latency
- <1 second local inference time

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

## Go-to-Market Strategy: The Tesla Approach

### Philosophy: Start Premium, Signal Quality

Like Tesla started with the Roadster for affluent early adopters, Robert will launch **exclusively for macOS**—targeting discerning, moneyed users who value quality and design. When they adopt and advocate, it signals status and utility to the broader market.

### Why macOS First?

**Strategic Advantages:**
1. **Discerning audience** - Mac users expect polished, native experiences and spot poor UX immediately
2. **Affluent market** - Higher disposable income for productivity tools, willing to pay for quality
3. **Status signaling** - Mac adoption creates aspirational appeal for Windows/Linux users (FOMO effect)
4. **Quality feedback** - Mac users provide thoughtful, detailed feedback that refines the product
5. **Network effects** - Mac users are vocal on Twitter, Product Hunt, Hacker News—they influence their networks

**Technical Advantages:**
- macOS provides excellent native frameworks (Tauri works beautifully)
- Consistent platform reduces support burden and testing complexity
- High-quality screenshots and demos look professional
- Single platform focus = better UX and polish

### The Playbook

**Phase 0: Foundation (Q4 2025)**
- ✅ CLI prototype proves technology works
- 🔄 Beautiful macOS native app in development
- 🔄 Focus on perfect UX and eye-candy UI

**Phase 1: Premium Launch (Q1 2026) - "The Roadster"**
- **Target:** macOS power users, designers, product managers, entrepreneurs
- **Distribution:** Product Hunt, Hacker News, Twitter, Mac communities
- **Messaging:** "Automation for the discerning Mac user"
- **Goal:** 1,000 passionate early adopters who love and advocate

**Phase 2: Refinement (Q2-Q3 2026) - "Listen & Polish"**
- Gather feedback from Mac community
- Refine UI based on real usage patterns
- Build reputation for quality and responsiveness
- **Goal:** 5,000+ MAU, 4.5+ star rating, featured in Mac blogs

**Phase 3: Expansion (Q4 2026) - "The Model S"**
- Windows desktop app (if demand warrants)
- Linux headless mode for developers
- Enterprise features for teams
- **Goal:** 20,000+ users across platforms

**Phase 4: Mass Market (2027) - "The Model 3"**
- Web version (if feasible)
- Mobile companion apps
- Community marketplace
- **Goal:** 100,000+ users, established brand

### Pricing Strategy (Premium Positioning)

**Free Tier (Open Source):**
- Full core functionality
- Local execution
- Community scripts
- No time limits

**Pro Tier ($9.99/month or $99/year):**
- Cloud sync for scripts
- Priority support
- Advanced features (scheduled runs, team sharing)
- Early access to new features

**Rationale:**
- Mac users are willing to pay for quality
- Premium price signals premium quality and sustainable development
- But open source core ensures freedom, trust, and community ownership

### Success Metrics

**Phase 1 (First 3 months):**
- 1,000 active users
- 100+ GitHub stars
- 4.5+ star average rating
- Featured on Product Hunt top 5
- Mentioned by 3+ Mac influencers

**Phase 2 (Months 4-9):**
- 5,000 active users
- 500+ GitHub stars
- Featured in Mac productivity articles
- 100+ community scripts
- 50+ Pro subscribers

### Why This Works

1. **Quality Bar** - Mac users will immediately spot poor UX. Meeting their standards means the product is truly great.
2. **Feedback Quality** - Mac users provide thoughtful feedback that refines the product before wider release.
3. **Social Proof** - When Mac users advocate, it creates FOMO for Windows/Linux users, building demand.
4. **Premium Association** - Starting on Mac creates premium brand perception, even though it's free/open source.
5. **Sustainable Growth** - Better to have 1,000 passionate Mac users than 10,000 lukewarm users. Passion drives word-of-mouth.

## Product Roadmap

### Version 1.0 (Q1 2026) - **macOS Desktop App**
- ✅ Native Tauri desktop application
- ✅ Chrome automation via CDP
- ✅ Beautiful, eye-candy UI designed for Mac
- ✅ Real-time execution with visual feedback
- ✅ Screenshot and text capture
- ✅ Voice-driven Markdown script creation
- ✅ Output management with visual browser

### Version 1.5 (Q2-Q3 2026) - **Polish & Refine**
- 🔄 Community script library
- 🔄 Template system for common automations
- 🔄 Enhanced visual feedback and animations
- 🔄 Performance optimizations
- 🔄 Mac-specific integrations (Shortcuts, Automator)

### Version 2.0 (Q4 2026) - **Cross-Platform Expansion**
- 🔄 Windows desktop app (if demand warrants)
- 🔄 Linux headless mode for developers
- 🔄 Enterprise features (team sharing, SSO)
- 🔄 Docker container
- 🔄 REST API

### Version 2.5 (Q1 2027) - **Visual Builder**
- 🔄 Drag-and-drop script builder
- 🔄 Record browser interactions (watch once, automate forever)
- 🔄 Visual selector picker
- 🔄 Flow diagram view

### Version 3.0 (Q2 2027+) - **AI & Collaboration**
- ✅ Voice-driven script creation (in v1.0)
- 🔄 Advanced AI features (error recovery, optimization suggestions)
- 🔄 **Workflow Learning System** - AI agents learn and improve navigation paths
  - Session recording with frame capture (screenshots + DOM + transcripts)
  - Workflow graph generation from sessions
  - Confidence-based selector management
  - Multi-agent knowledge sharing
  - Automatic error recovery strategies
- 🔄 Team workspaces
- 🔄 Community script marketplace
- 🔄 Scheduled runs and automation
- 🔄 Multi-browser support (Firefox, Safari, Edge)

## Competitive Analysis

### Automation Platforms

| Feature | **Robert (v1.0)** | **Zapier/IFTTT** | **Claude Agents** | **GPT Interface** | **Herd/Monitoro** |
|---------|-------------------|------------------|-------------------|-------------------|-------------------|
| **Target User** | Everyone | API-literate users | Programmers | General users | Programmers |
| **Visual Feedback** | ✅ Real-time browser | ❌ Log-based | ❌ None | ❌ None | ✅ Real-time |
| **Learning Curve** | ⭐ Voice-driven | ⭐⭐⭐⭐ API knowledge | ⭐⭐⭐⭐⭐ Programming | ⭐ Natural language | ⭐⭐⭐⭐ Test runners |
| **Control** | ✅ Pause/abort/inspect | ❌ Fire & forget | ❌ Black box | ❌ Black box | ✅ Pause/abort |
| **Local Execution** | ✅ Your device | ❌ Cloud only | ❌ Cloud only | ❌ Cloud only | ❌ Cloud only |
| **Privacy** | ✅ Complete | ❌ Data sent to cloud | ❌ Data sent to cloud | ❌ Data sent to cloud | ❌ Data sent to cloud |
| **Open Source** | ✅ Free & auditable | ❌ Proprietary | ❌ Proprietary | ❌ Proprietary | ❌ Proprietary |
| **Programming Required** | ❌ Talk only | ⚠️ API knowledge | ✅ Yes | ❌ No | ✅ Yes |
| **Eye-candy UI** | ✅ Beautiful native | ❌ Functional | ❌ Chat interface | ❌ Chat interface | ✅ Modern web |
| **Cost** | 🆓 Free | 💰 Subscription | 💰 Subscription | 💰 Subscription | 💰 Subscription |

### Developer Tools

| Feature | **Robert (v1.0)** | **Selenium IDE** | **Playwright** | **Puppeteer** |
|---------|-------------------|------------------|----------------|---------------|
| **UI Quality** | ⭐⭐⭐⭐⭐ Native app | ⭐⭐⭐ Extension | ⭐⭐ CLI | ⭐⭐⭐ DevTools |
| **Target User** | Non-programmers | QA Engineers | Developers | Developers |
| **Real-time Status** | ✅ Visual dashboard | ⚠️ Basic logs | ❌ Console only | ❌ Console only |
| **Script Format** | Markdown (readable) | Selenium format | JavaScript/TS | JavaScript |
| **Ease of Use** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐ |
| **Programming** | ❌ Not required | ⚠️ Some scripting | ✅ Required | ✅ Required |
| **Output Management** | ✅ Visual browser | ❌ Manual | ❌ Manual | ❌ Manual |
| **Local Execution** | ✅ Always | ✅ Yes | ✅ Yes | ✅ Yes |

**Unique Selling Points:**
1. 🌍 **Automation for everyone** - No programming or API knowledge required
2. 👁️ **Watch it work** - Real-time visual feedback builds confidence
3. 🎮 **Full control** - Pause, abort, inspect at any moment
4. 🏠 **Local-first** - Your device, your data, your privacy
5. 🆓 **Open & free** - No vendor lock-in, community-owned
6. 🎨 **Eye-candy UI** - Beautiful interface makes automation delightful
7. 🎤 **Voice-driven** - Talk through automation, AI writes Markdown

## Technical Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                   Tauri Desktop App                     │
│                                                          │
│  ┌────────────────────┐      ┌────────────────────┐    │
│  │  Svelte Frontend   │ IPC  │   Rust Backend     │    │
│  │                    │◄────►│                    │    │
│  │  - Script Editor   │      │  - chromiumoxide   │    │
│  │  - Execution UI    │      │  - System Chrome  │    │
│  │  - Output Browser  │      │  - Script Executor │    │
│  │  - Settings Panel  │      │  - Capture Engine  │    │
│  │                    │      │  - Workflow Learn* │    │
│  └────────────────────┘      └──────────┬─────────┘    │
│                                          │              │
└──────────────────────────────────────────┼──────────────┘
                                           │
                                * Future: Workflow Learning System
                                  - Session recording (frames)
                                  - Workflow graph generation
                                  - Confidence-based execution
                                  - Multi-agent knowledge sharing
                              ┌────────────┴──────────────┐
                              │                           │
                        Sandboxed Mode           Advanced Mode
                              │                           │
                    ┌─────────▼──────────┐    ┌──────────▼─────────┐
                    │ System Chrome     │    │ Connect to         │
                    │ Auto-download      │    │ localhost:9222     │
                    │ Chrome to          │    │                    │
                    │ ~/.robert/chrome/  │    │ (User's Chrome     │
                    └─────────┬──────────┘    │  with debug port)  │
                              │               └──────────┬─────────┘
                              │                          │
                              │ Chrome DevTools Protocol │
                              │                          │
                    ┌─────────▼──────────────────────────▼─────────┐
                    │            Chrome Browser                     │
                    │         (Visible or Headless)                 │
                    └───────────────────────────────────────────────┘
```

## Browser Automation Approach

### Chrome DevTools Protocol (CDP) via chromiumoxide

The application uses **Chrome DevTools Protocol (CDP)** directly through the `chromiumoxide` Rust library, eliminating the need for ChromeDriver. This provides:

- **Zero external dependencies** for end users
- **Automatic Chrome download** for sandboxed execution
- **Two operation modes** for different use cases

### Operation Modes

#### 1. Sandboxed Mode (Default)
- **Target**: General users, testing, isolated automation
- **Chrome Source**: Auto-downloaded via System Chrome to `~/.robert/chrome/`
- **Profile**: Fresh, isolated session (no user data)
- **Setup**: Zero - app handles everything
- **Use Case**: Testing, automation scripts, CI/CD

**Features:**
```rust
// First run: Auto-downloads Chrome binary
let fetcher = System Chrome::new(
    System ChromeOptions::builder()
        .with_path("~/.robert/chrome")
        .build()
);
let info = fetcher.fetch().await?;

// Launch isolated Chrome instance
let browser = Browser::launch(
    BrowserConfig::builder()
        .chrome_executable(info.executable_path)
        .headless(false)  // Desktop app shows browser
        .build()
).await?;
```

#### 2. Advanced Mode (Power Users)
- **Target**: Power users needing existing sessions/cookies
- **Chrome Source**: User's installed Chrome with active profile
- **Profile**: User's real profile (logged-in accounts, history, cookies)
- **Setup**: User starts Chrome with `--remote-debugging-port=9222`
- **Use Case**: Authenticated workflows, personal automation

**User Workflow:**
```bash
# User restarts their Chrome with debug flag
chrome --remote-debugging-port=9222 \
       --user-data-dir="$HOME/Library/Application Support/Google/Chrome/Default"
```

**App Connects:**
```rust
// Connect to existing Chrome on debug port
let browser = Browser::connect("http://localhost:9222").await?;
// Now automating user's real browser with their session
```

**CLI Flags:**
```bash
# Sandboxed mode (default)
robert google.com

# Advanced mode - connect to debug port
robert google.com --debug-port 9222

# Or let app launch Chrome with user profile
robert google.com --use-profile
```

### Why CDP Instead of WebDriver?

| Aspect | CDP (chromiumoxide) | WebDriver (chromedriver) |
|--------|---------------------|-------------------------|
| **External Binary** | None | Requires chromedriver |
| **User Setup** | Zero | Must install/manage chromedriver |
| **Version Compatibility** | Automatic | Version matching required |
| **Chrome Download** | Built-in fetcher | Manual installation |
| **Performance** | Direct protocol | Extra abstraction layer |
| **Real Browser Control** | Native support | Limited |

## Dependencies

### External (End Users)
- **None** - App downloads Chrome automatically in sandboxed mode
- **Chrome browser** (optional) - For advanced mode with user profile

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
- **Browser Automation**: chromiumoxide (Chrome DevTools Protocol)
- **Chrome Management**: System Chrome (auto-download)
- **Async Runtime**: tokio
- **Script Format**: Markdown with YAML frontmatter (pulldown-cmark + serde_yaml)
- **Build System**: Cargo + Vite

### Reference Links
- Tauri: https://v2.tauri.app/
- chromiumoxide: https://github.com/spider-rs/chromiumoxide
- Chrome DevTools Protocol: https://chromedevtools.github.io/devtools-protocol/
- Chrome for Testing: https://developer.chrome.com/blog/chrome-for-testing

### Glossary
- **Tauri**: Desktop app framework using Rust backend and web frontend
- **CDP (Chrome DevTools Protocol)**: Low-level protocol for controlling Chrome browsers
- **chromiumoxide**: Rust library for browser automation via CDP
- **System Chrome**: Component that auto-downloads Chrome binaries
- **Sandboxed Mode**: Isolated Chrome instance with auto-downloaded binary
- **Advanced Mode**: Connect to user's existing Chrome via debug port
- **IPC**: Inter-Process Communication (between Tauri frontend/backend)
- **DMG**: macOS disk image for app distribution
- **Notarization**: Apple's security verification for macOS apps

---

**Document Version**: 2.0
**Last Updated**: 2025-10-08
**Status**: Approved for Development
**Target Release**: v1.0 - Q1 2026
