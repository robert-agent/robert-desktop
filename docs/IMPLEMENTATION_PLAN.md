# Browser Automation Tool - Revised Implementation Plan
## Desktop App with Tauri for macOS

## Overview

A user-friendly browser automation tool with a **Tauri desktop app** for macOS, featuring:
- **Voice-driven automation creation** - Talk through your automation, AI writes Markdown scripts
- **Real-time visual feedback** - Watch automation happen in visible browser
- **Local-first AI** - All inference runs on-device by default
- **Privacy protection** - Auto-obfuscates sensitive data if using cloud AI (optional)

## Architecture

### Two-Build Strategy

#### Build 1: macOS Desktop App (Priority)
- **Frontend**: Tauri with React/Vue/Svelte
- **Backend**: Rust (browser automation engine)
- **Target**: macOS with Chrome
- **Mode**: Visible browser window with desktop UI

#### Build 2: Linux Headless (Roadmap)
- **Deployment**: Linux server/container
- **Mode**: Headless Chrome
- **Target**: Automated workflows, CI/CD integration

## Why Tauri Over React Native?

| Criteria | Tauri | React Native |
|----------|-------|--------------|
| **Rust Integration** | Native - Rust backend built-in | Requires bridges/FFI |
| **Binary Size** | ~600KB - 3MB | 50MB+ |
| **Performance** | Native OS renderer (WebKit on macOS) | JavaScript bridge overhead |
| **Security** | Rust memory safety + sandboxing | JavaScript vulnerabilities |
| **Desktop Focus** | Primary use case | Mobile-first, desktop secondary |
| **macOS Native** | Uses system WebView | Bundle entire runtime |
| **Development** | Web frontend + Rust commands | React + Native modules |
| **Distribution** | .app bundle, DMG | More complex setup |

**Decision: Use Tauri** for better performance, smaller binaries, native Rust integration, and desktop-first design.

## Technology Stack

### Core Technologies
- **Desktop Framework**: Tauri 2.0
- **Frontend**: Svelte + TypeScript
- **Backend/Engine**: Rust 1.70+
- **Browser Automation**: chromiumoxide (Chrome DevTools Protocol)
- **Chrome Management**: System Chrome (auto-download Chrome)
- **Async Runtime**: tokio
- **IPC**: Tauri commands (built-in)
- **State Management**: Svelte stores + Tauri state
- **Styling**: Tailwind CSS

### Rust Dependencies
```toml
[dependencies]
# Tauri core
tauri = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Browser automation
chromiumoxide = { version = "0.1", features = ["fetcher"] }
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"

# Utilities
anyhow = "1.0"
thiserror = "1.0"
pulldown-cmark = "0.9"  # Markdown parsing
tracing = "0.1"
tracing-subscriber = "0.3"
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
image = "0.24"
base64 = "0.21"

# CLI (for debugging/advanced users)
clap = { version = "4.0", features = ["derive"], optional = true }
```

## Project Structure

```
inferno/
├── crates/
│   └── browser-automation/
│       ├── Cargo.toml
│       ├── tauri.conf.json          # Tauri configuration
│       ├── src-tauri/               # Rust backend
│       │   ├── main.rs              # Tauri entry point
│       │   ├── lib.rs               # Library exports
│       │   ├── commands.rs          # Tauri command handlers
│       │   ├── state.rs             # Application state
│       │   ├── events.rs            # Event emitters
│       │   ├── browser/
│       │   │   ├── mod.rs
│       │   │   ├── driver.rs        # Browser trait
│       │   │   ├── chrome.rs        # Chrome implementation
│       │   │   ├── firefox.rs       # Firefox (future)
│       │   │   └── lifecycle.rs
│       │   ├── script/
│       │   │   ├── mod.rs
│       │   │   ├── parser.rs        # Markdown script parser
│       │   │   ├── executor.rs      # Script executor with events
│       │   │   └── actions.rs
│       │   ├── capture/
│       │   │   ├── mod.rs
│       │   │   ├── screenshot.rs
│       │   │   ├── text.rs
│       │   │   └── metadata.rs
│       │   ├── storage/
│       │   │   ├── mod.rs
│       │   │   ├── filesystem.rs
│       │   │   └── manifest.rs
│       │   └── error.rs
│       ├── src/                     # Frontend (Svelte)
│       │   ├── App.svelte
│       │   ├── main.ts
│       │   ├── components/
│       │   │   ├── ScriptEditor.svelte
│       │   │   ├── ExecutionStatus.svelte
│       │   │   ├── StepList.svelte
│       │   │   ├── OutputViewer.svelte
│       │   │   ├── BrowserSelector.svelte
│       │   │   └── SettingsPanel.svelte
│       │   ├── lib/
│       │   │   ├── tauri.ts          # Tauri command wrappers
│       │   │   ├── events.ts         # Event listeners
│       │   │   └── stores.ts         # Svelte stores
│       │   ├── types/
│       │   │   └── automation.ts
│       │   └── styles/
│       │       └── app.css
│       ├── public/
│       │   └── icons/
│       ├── examples/
│       │   ├── basic_navigation.md
│       │   └── form_interaction.md
│       ├── tests/
│       │   ├── integration/
│       │   └── unit/
│       └── README.md
└── docs/
    ├── browser-automation-prd.md
    ├── browser-automation-implementation-plan.md
    └── browser-automation-revised-plan.md
```

## Desktop App UI Design

### Main Window Layout

```
┌─────────────────────────────────────────────────────────────┐
│  Browser Automation                           [−] [□] [×]   │
├─────────────────────────────────────────────────────────────┤
│  Script  |  Execution  |  Output  |  Settings                │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Script Tab:                                                  │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ name: "Login Flow"                                   │    │
│  │ version: "1.0"                                       │    │
│  │ output_dir: "./output"                               │    │
│  │                                                       │    │
│  │ steps:                                               │    │
│  │   - action: navigate                                 │    │
│  │     url: "https://example.com/login"                │    │
│  │   - action: screenshot                               │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
│  [📁 Load Script]  [💾 Save]  [✓ Validate]  [▶ Run]        │
│                                                               │
├─────────────────────────────────────────────────────────────┤
│  Execution Status:  ● Running  Step 2/5  [=======   ] 40%   │
└─────────────────────────────────────────────────────────────┘
```

### Execution Tab (Real-time Status)

```
┌─────────────────────────────────────────────────────────────┐
│  Execution Progress                                          │
├─────────────────────────────────────────────────────────────┤
│  Script: login_flow.md                                       │
│  Started: 10:30:45                                           │
│  Status: Running                                             │
│                                                               │
│  Steps:                                                       │
│  ✓ 1. Navigate to login page       [1.2s]                   │
│  ✓ 2. Take screenshot              [0.4s]                   │
│  ▶ 3. Click login button           [running...]             │
│  ⋯ 4. Type username                                          │
│  ⋯ 5. Submit form                                            │
│                                                               │
│  Recent Logs:                                                │
│  [10:30:46] Navigating to https://example.com/login         │
│  [10:30:47] Page loaded successfully                        │
│  [10:30:47] Capturing screenshot...                         │
│  [10:30:48] Screenshot saved: 001_homepage.png              │
│  [10:30:48] Clicking element: #login-button                 │
│                                                               │
│  [⏸ Pause]  [⏹ Stop]                                        │
└─────────────────────────────────────────────────────────────┘
```

### Output Tab

```
┌─────────────────────────────────────────────────────────────┐
│  Output Files                                                │
├─────────────────────────────────────────────────────────────┤
│  📁 ./output/run_abc123/                                     │
│                                                               │
│  Screenshots (3):                                            │
│  ┌────────┐ ┌────────┐ ┌────────┐                          │
│  │ [img1] │ │ [img2] │ │ [img3] │                          │
│  │ 001... │ │ 002... │ │ 003... │                          │
│  └────────┘ └────────┘ └────────┘                          │
│                                                               │
│  Text Files (2):                                             │
│  📄 001_homepage.txt (2.3 KB)                                │
│  📄 002_results.txt (1.8 KB)                                 │
│                                                               │
│  📋 manifest.json (5.1 KB)                                   │
│                                                               │
│  [📂 Open Folder]  [👁 Preview]  [🗑 Clear]                 │
└─────────────────────────────────────────────────────────────┘
```

## Tauri IPC Architecture

### Command Pattern (Frontend → Backend)

```typescript
// Frontend: src/lib/tauri.ts
import { invoke } from '@tauri-apps/api/core';
import type { Script } from '../types/automation';

export async function loadScript(path: string): Promise<Script> {
  return await invoke<Script>('load_script', { path });
}

export async function runScript(script: Script): Promise<string> {
  return await invoke<string>('run_script', { script });
}

export async function stopExecution(runId: string): Promise<void> {
  return await invoke('stop_execution', { runId });
}
```

```rust
// Backend: src-tauri/commands.rs
use tauri::State;
use crate::state::AppState;

#[tauri::command]
async fn load_script(path: String) -> Result<Script, String> {
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read script: {}", e))?;

    let script = parse_script(&content)
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(script)
}

#[tauri::command]
async fn run_script(
    script: Script,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let run_id = uuid::Uuid::new_v4().to_string();

    // Spawn async task for script execution
    tokio::spawn(async move {
        let mut executor = Executor::new(&app_handle);
        executor.execute(script, &run_id).await;
    });

    Ok(run_id)
}

#[tauri::command]
fn stop_execution(run_id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.cancel_execution(&run_id)
        .map_err(|e| format!("Failed to stop: {}", e))
}
```

### Event Pattern (Backend → Frontend)

```rust
// Backend: src-tauri/events.rs
use tauri::Emitter;

pub fn emit_step_started(app: &tauri::AppHandle, step: &Step) {
    let _ = app.emit("step:started", StepEvent {
        step_number: step.number,
        action: step.action.clone(),
        timestamp: Utc::now(),
    });
}

pub fn emit_step_completed(app: &tauri::AppHandle, step: &Step, duration: u64) {
    let _ = app.emit("step:completed", StepCompletedEvent {
        step_number: step.number,
        duration_ms: duration,
        success: true,
    });
}

pub fn emit_log(app: &tauri::AppHandle, message: &str) {
    let _ = app.emit("log", LogEvent {
        timestamp: Utc::now(),
        level: "info",
        message: message.to_string(),
    });
}
```

```typescript
// Frontend: src/lib/stores.ts
import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import type { StepEvent, ExecutionState } from '../types/automation';

export const executionState = writable<ExecutionState>({
  isRunning: false,
  currentStep: 0,
  totalSteps: 0,
  logs: []
});

// Setup event listeners
export async function setupEventListeners() {
  await listen<StepEvent>('step:started', (event) => {
    executionState.update(state => ({
      ...state,
      currentStep: event.payload.step_number
    }));
  });

  await listen<StepCompletedEvent>('step:completed', (event) => {
    console.log('Step completed:', event.payload);
  });

  await listen<LogEvent>('log', (event) => {
    executionState.update(state => ({
      ...state,
      logs: [...state.logs, event.payload]
    }));
  });
}
```

```svelte
<!-- Frontend: src/components/ExecutionStatus.svelte -->
<script lang="ts">
  import { executionState } from '../lib/stores';
  import { onMount } from 'svelte';
  import { setupEventListeners } from '../lib/stores';

  onMount(() => {
    setupEventListeners();
  });

  $: progress = $executionState.totalSteps > 0
    ? ($executionState.currentStep / $executionState.totalSteps) * 100
    : 0;
</script>

<div class="execution-panel">
  <h2>Execution Status</h2>

  {#if $executionState.isRunning}
    <div class="status-indicator running">● Running</div>
    <div class="progress-bar">
      <div class="progress-fill" style="width: {progress}%"></div>
    </div>
    <p>Step {$executionState.currentStep} / {$executionState.totalSteps}</p>
  {:else}
    <div class="status-indicator idle">○ Idle</div>
  {/if}

  <div class="logs">
    {#each $executionState.logs as log}
      <div class="log-entry">[{log.timestamp}] {log.message}</div>
    {/each}
  </div>
</div>
```

## Implementation Phases (Revised)

### Phase 0: CLI Prototype (Week 0 - Proof of Concept)

**Goal**: Validate browser automation approach with a minimal CLI tool before building the full desktop app.

#### Milestone 0.1: Workspace Setup

**Tasks:**
1. Initialize Cargo workspace with multi-crate structure
2. Create three crates:
   - `robert-webdriver` - Core browser automation library
   - `robert-cli` - Command-line interface binary
   - `robert-app` - Tauri desktop application (placeholder)
3. Configure workspace dependencies
4. Setup shared types and traits

**Project Structure:**
```
robert/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── robert-webdriver/         # Library crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── browser/
│   │       │   ├── mod.rs
│   │       │   └── chrome.rs
│   │       ├── navigation.rs
│   │       └── error.rs
│   ├── robert-cli/               # CLI binary crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── robert-app/               # Tauri app (future)
│       └── Cargo.toml
└── docs/
```

**Deliverables:**
- Cargo workspace compiles
- All three crates created
- Workspace dependency sharing configured

#### Milestone 0.2: Basic CLI - Navigate and Fetch

**Tasks:**
1. Implement basic browser automation in `robert-webdriver`:
   ```rust
   // Core functionality
   - ChromeDriver::launch_sandboxed() - Auto-download and launch Chrome
   - ChromeDriver::connect_debug_port() - Connect to existing Chrome
   - navigate(url) - Go to URL
   - get_page_content() - Fetch HTML content
   - get_text() - Extract visible text
   ```
2. Implement CLI in `robert-cli`:
   - Parse command-line arguments (URL, mode flags)
   - Launch sandboxed Chrome OR connect to debug port
   - Navigate to URL
   - Print page content to stdout
3. Two modes: sandboxed (default) and advanced (debug port)
4. Add basic error handling

**CLI Usage:**
```bash
# Sandboxed mode (default) - auto-downloads Chrome
cargo run --bin robert-cli -- https://example.com

# Advanced mode - connect to user's Chrome on debug port
cargo run --bin robert-cli -- https://example.com --debug-port 9222

# Output:
# Connecting to Chrome...
# Navigating to https://example.com...
# Page loaded: Example Domain
#
# <full page HTML content>
```

**Code Example:**
```rust
// crates/robert-webdriver/src/browser/chrome.rs
use chromiumoxide::{Browser, BrowserConfig};
use chromiumoxide::fetcher::{System Chrome, System ChromeOptions};

pub struct ChromeDriver {
    browser: Browser,
}

pub enum ConnectionMode {
    Sandboxed,
    DebugPort(u16),
}

impl ChromeDriver {
    /// Launch sandboxed Chrome (auto-downloads if needed)
    pub async fn launch_sandboxed() -> Result<Self, BrowserError> {
        // Auto-download Chrome on first run
        let cache_dir = dirs::home_dir()
            .ok_or(BrowserError::NoCacheDir)?
            .join(".robert/chrome");

        let fetcher = System Chrome::new(
            System ChromeOptions::builder()
                .with_path(cache_dir)
                .build()
        );

        let info = fetcher.fetch().await
            .map_err(|e| BrowserError::FetchFailed(e.to_string()))?;

        let browser = Browser::launch(
            BrowserConfig::builder()
                .chrome_executable(info.executable_path)
                .build()
        ).await?;

        Ok(Self { browser })
    }

    /// Connect to existing Chrome on debug port
    pub async fn connect_debug_port(port: u16) -> Result<Self, BrowserError> {
        let url = format!("http://localhost:{}", port);
        let browser = Browser::connect(&url).await?;
        Ok(Self { browser })
    }

    pub async fn navigate(&self, url: &str) -> Result<(), BrowserError> {
        let page = self.browser.new_page(url).await?;
        Ok(())
    }

    pub async fn get_page_source(&self) -> Result<String, BrowserError> {
        let pages = self.browser.get_pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;
        let html = page.get_content().await?;
        Ok(html)
    }

    pub async fn get_page_text(&self) -> Result<String, BrowserError> {
        let pages = self.browser.get_pages().await?;
        let page = pages.first().ok_or(BrowserError::NoPage)?;
        let text = page.get_inner_text("body").await?;
        Ok(text)
    }
}
```

```rust
// crates/robert-cli/src/main.rs
use robert_webdriver::browser::ChromeDriver;
use clap::Parser;

#[derive(Parser)]
#[command(name = "robert")]
#[command(about = "Browser automation CLI prototype")]
struct Cli {
    /// URL to navigate to
    url: String,

    /// Connect to existing Chrome debug port (advanced mode)
    #[arg(long)]
    debug_port: Option<u16>,

    /// Output format: html or text
    #[arg(short = 'f', long, default_value = "html")]
    format: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Connect to Chrome (sandboxed or debug port)
    let driver = if let Some(port) = cli.debug_port {
        println!("🔌 Connecting to Chrome debug port {}...", port);
        ChromeDriver::connect_debug_port(port).await?
    } else {
        println!("🚀 Launching sandboxed Chrome (auto-downloading if needed)...");
        ChromeDriver::launch_sandboxed().await?
    };

    println!("🌐 Navigating to {}...", cli.url);
    driver.navigate(&cli.url).await?;

    println!("✅ Page loaded!\n");

    let content = match cli.format.as_str() {
        "text" => driver.get_page_text().await?,
        _ => driver.get_page_source().await?,
    };

    println!("{}", content);

    Ok(())
}
```

**Dependencies (robert-webdriver):**
```toml
[dependencies]
chromiumoxide = { version = "0.1", features = ["fetcher"] }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
thiserror = "1.0"
dirs = "5.0"  # For cache directory
```

**Dependencies (robert-cli):**
```toml
[dependencies]
robert-webdriver = { path = "../robert-webdriver" }
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

**Deliverables:**
- CLI tool with two modes: sandboxed and advanced
- Auto-downloads Chrome on first run (sandboxed mode)
- Can connect to user's Chrome via debug port (advanced mode)
- Can navigate to any URL
- Prints page content (HTML or text)
- Works with visible Chrome window
- Basic error messages

**Testing:**
```bash
# 1. Test sandboxed mode (no setup required!)
cargo run --bin robert-cli -- https://example.com
cargo run --bin robert-cli -- https://example.com --format text

# 2. Test advanced mode (requires Chrome with debug port)
# First, start Chrome with debug flag:
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222

# Then connect:
cargo run --bin robert-cli -- https://example.com --debug-port 9222

# 3. Should see page content printed in both modes
```

**Success Criteria:**
- [ ] Workspace structure created
- [ ] Three crates compile successfully
- [ ] Sandboxed mode auto-downloads Chrome on first run
- [ ] Sandboxed mode launches Chrome successfully
- [ ] Advanced mode connects to Chrome debug port
- [ ] CLI navigates to provided URL
- [ ] Page content displayed in terminal
- [ ] Works with visible Chrome (not headless)
- [ ] Basic error handling (connection failed, navigation timeout, download failed)

**Duration**: 2-3 days

**Why This Phase?**
- ✅ Validates chromiumoxide integration before GUI work
- ✅ Tests both connection modes (sandboxed and advanced)
- ✅ Proves zero-dependency approach works
- ✅ Creates reusable library crate for Tauri app
- ✅ Quick feedback loop for debugging
- ✅ Foundation for all future browser automation

---

### Phase 1: Tauri Setup & Basic Browser Control (Week 1)

#### Milestone 1.1: Tauri Project Setup
**Tasks:**
1. Initialize Tauri project: `npm create tauri-app`
2. Configure `tauri.conf.json`:
   - App name, identifier
   - Window size (1280x800)
   - macOS permissions (file access, network)
   - Build settings
3. Setup Svelte frontend with Vite
4. Install dependencies (Rust + npm)
5. Test basic Tauri app launch

**Deliverables:**
- Tauri app launches on macOS
- Basic "Hello World" window
- Hot-reload working

#### Milestone 1.2: Rust Backend - Browser Control
**Tasks:**
1. Add chromiumoxide to Cargo.toml with fetcher feature
2. Implement `browser/chrome.rs` with chromiumoxide
3. Implement both modes: sandboxed and advanced
4. Implement browser launch/close
5. Add Tauri commands: `launch_browser_sandboxed`, `launch_browser_advanced`, `close_browser`
6. Handle Chrome auto-download and caching

**Code Example:**
```rust
// src-tauri/browser/chrome.rs
use chromiumoxide::{Browser, BrowserConfig};
use chromiumoxide::fetcher::{System Chrome, System ChromeOptions};

pub struct ChromeDriver {
    browser: Browser,
}

pub enum BrowserMode {
    Sandboxed,
    Advanced { debug_port: u16 },
}

impl ChromeDriver {
    pub async fn new(mode: BrowserMode) -> Result<Self, BrowserError> {
        let browser = match mode {
            BrowserMode::Sandboxed => {
                // Auto-download Chrome for Testing
                let cache_dir = dirs::home_dir()
                    .ok_or(BrowserError::NoCacheDir)?
                    .join(".robert/chrome");

                let fetcher = System Chrome::new(
                    System ChromeOptions::builder()
                        .with_path(cache_dir)
                        .build()
                );

                let info = fetcher.fetch().await?;

                Browser::launch(
                    BrowserConfig::builder()
                        .chrome_executable(info.executable_path)
                        .window_size(1280, 1024)
                        .build()
                ).await?
            }
            BrowserMode::Advanced { debug_port } => {
                let url = format!("http://localhost:{}", debug_port);
                Browser::connect(&url).await?
            }
        };

        Ok(Self { browser })
    }
}
```

**Deliverables:**
- Chrome launches from Tauri app (sandboxed mode)
- Can connect to user's Chrome (advanced mode)
- Frontend buttons for both modes
- Chrome auto-download on first launch
- Error handling for connection failures

#### Milestone 1.3: Navigation & Basic Commands
**Tasks:**
1. Implement navigation: `navigate`, `wait_for_load`
2. Add Tauri commands: `navigate_to_url`
3. Frontend: URL input and navigate button
4. Display current URL in UI
5. Handle navigation errors

**Deliverables:**
- Can navigate to URLs from desktop app
- URL displayed in UI
- Loading states shown

### Phase 2: Script System & Execution Status (Week 2)

#### Milestone 2.1: Script Parser
**Tasks:**
1. Implement Markdown script parser (with YAML frontmatter)
2. Add Tauri commands: `load_script`, `validate_script`
3. Frontend: Script editor with syntax highlighting (CodeMirror)
4. File picker for loading scripts
5. Script validation UI

**Deliverables:**
- Load and parse Markdown scripts
- Visual validation feedback
- Example scripts included

#### Milestone 2.2: Script Executor with Events
**Tasks:**
1. Implement script executor
2. Add event emission:
   - `execution:started`
   - `step:started`
   - `step:completed`
   - `step:failed`
   - `execution:completed`
   - `log`
3. Frontend: Real-time execution status display
4. Progress bar and step list
5. Log viewer

**Code Example:**
```rust
// src-tauri/script/executor.rs
use tauri::Emitter;

pub struct Executor {
    driver: ChromeDriver,
    app: tauri::AppHandle,
}

impl Executor {
    pub async fn execute(&mut self, script: Script, run_id: &str) -> Result<()> {
        self.emit_execution_started(run_id);

        for (i, step) in script.steps.iter().enumerate() {
            self.emit_step_started(i + 1, step);

            match self.execute_step(step).await {
                Ok(duration) => {
                    self.emit_step_completed(i + 1, duration);
                }
                Err(e) => {
                    self.emit_step_failed(i + 1, &e);
                    return Err(e);
                }
            }
        }

        self.emit_execution_completed(run_id);
        Ok(())
    }

    fn emit_step_started(&self, step_num: usize, step: &Step) {
        let _ = self.app.emit("step:started", StepStartedEvent {
            step_number: step_num,
            action: step.action.to_string(),
        });
    }
}
```

**Deliverables:**
- Real-time execution status in UI
- Step-by-step progress
- Live logs
- Pause/stop functionality

### Phase 3: Content Capture & Display (Week 3)

#### Milestone 3.1: Screenshot Capture
**Tasks:**
1. Implement screenshot capture (viewport, full-page, element)
2. Base64 encode for Tauri transfer
3. Add Tauri command: `capture_screenshot`
4. Frontend: Display captured screenshots
5. Thumbnail gallery

**Deliverables:**
- Screenshots captured during automation
- Preview in UI
- Saved to disk

#### Milestone 3.2: Text Extraction
**Tasks:**
1. Implement text extraction
2. Add Tauri command: `extract_text`
3. Frontend: Text viewer component
4. Search/filter extracted text

**Deliverables:**
- Text extraction working
- Viewable in UI
- Saved to files

#### Milestone 3.3: Output Management
**Tasks:**
1. Implement filesystem storage
2. Generate manifest.json
3. Frontend: Output tab with file browser
4. Preview screenshots and text
5. "Open in Finder" button

**Deliverables:**
- Organized output directory
- File browser in UI
- Native file manager integration

### Phase 4: User Interactions (Week 4)

#### Milestone 4.1: Basic Interactions
**Tasks:**
1. Implement click, type, scroll actions
2. Add wait conditions
3. Test with real websites
4. Error recovery

**Deliverables:**
- All basic actions working
- Reliable interaction handling

#### Milestone 4.2: Advanced Interactions
**Tasks:**
1. Form handling (dropdowns, checkboxes, radio)
2. Keyboard/mouse events
3. Dialog handling
4. Multi-element selection

**Deliverables:**
- Complex interactions supported
- Form automation working

### Phase 5: Polish & macOS Integration (Week 5)

#### Milestone 5.1: Settings & Configuration
**Tasks:**
1. Settings panel in UI
2. Chrome path configuration
3. Default timeout settings
4. Output directory picker
5. Theme selection (light/dark)

**Deliverables:**
- Comprehensive settings UI
- Persist user preferences

#### Milestone 5.2: macOS Native Features
**Tasks:**
1. App icon and branding
2. macOS menu bar integration
3. Native notifications
4. Keyboard shortcuts
5. Window management

**Deliverables:**
- Native macOS look and feel
- System integration

#### Milestone 5.3: Error Handling & UX
**Tasks:**
1. User-friendly error messages
2. Retry dialogs
3. Help tooltips
4. Onboarding flow

**Deliverables:**
- Polished UX
- Clear error guidance

### Phase 6: Testing & Documentation (Week 6)

#### Milestone 6.1: Testing
**Tasks:**
1. Unit tests (Rust)
2. Integration tests
3. Frontend tests (Vitest)
4. macOS testing (different OS versions)
5. Chrome version compatibility matrix

**Deliverables:**
- >70% test coverage
- CI/CD pipeline

#### Milestone 6.2: Documentation
**Tasks:**
1. User guide
2. Script reference
3. API documentation
4. Video tutorials
5. Example scripts

**Deliverables:**
- Complete documentation
- Example library

### Phase 7: Packaging & Release (Week 7)

#### Milestone 7.1: macOS Distribution
**Tasks:**
1. Code signing setup
2. Build .app bundle
3. Create DMG installer
4. Notarization for macOS Gatekeeper
5. Auto-updater setup

**Deliverables:**
- Signed macOS app
- DMG installer
- Update mechanism

#### Milestone 7.2: Release
**Tasks:**
1. Version 1.0.0 release
2. GitHub releases
3. Website/landing page
4. Release notes
5. User feedback channels

**Deliverables:**
- v1.0.0 released
- Distribution channels active

## Roadmap: Linux Headless Build

### Phase 8: Headless Mode (Post v1.0)

**Goals:**
- Headless Chrome support
- Linux binary build
- Docker containerization
- API-driven execution (REST/gRPC)
- No GUI dependencies

**Use Cases:**
- CI/CD integration
- Server automation
- Scheduled tasks
- High-volume processing

**Architecture Changes:**
- Conditional compilation (GUI vs headless)
- HTTP API server (actix-web/axum)
- Database for job queue (SQLite/PostgreSQL)
- Job scheduler

## Success Criteria

### Functional
- [ ] Tauri app launches on macOS
- [ ] Chrome automation working
- [ ] Script loading and parsing
- [ ] Real-time execution status display
- [ ] Screenshot and text capture
- [ ] Output management and viewing
- [ ] All interaction actions working

### UX
- [ ] Intuitive UI (user testing)
- [ ] Real-time feedback
- [ ] Clear error messages
- [ ] Native macOS feel
- [ ] Responsive performance

### Technical
- [ ] <3 second app launch
- [ ] <2 second screenshot capture
- [ ] <100MB app memory footprint
- [ ] >70% test coverage
- [ ] No memory leaks

## Key Differences from Original Plan

| Aspect | Original | Revised |
|--------|----------|---------|
| **Interface** | CLI only | Tauri desktop app |
| **Platform** | Windows first | macOS first |
| **User Experience** | Terminal output | Visual GUI with real-time status |
| **Distribution** | Binary executable | macOS .app bundle + DMG |
| **Future** | Cross-platform CLI | Desktop app + Linux headless |
| **Communication** | Logs to console | Tauri IPC events |

## Dependencies

### Rust Crates
- `tauri` - Desktop framework
- `chromiumoxide` - Browser automation via CDP (with `fetcher` feature)
- `tokio` - Async runtime
- `serde`, `serde_json` - Serialization
- `pulldown-cmark` - Markdown parsing
- `uuid`, `chrono` - Utilities
- `tracing` - Logging
- `image`, `base64` - Image handling
- `anyhow`, `thiserror` - Error handling
- `dirs` - Cross-platform directory paths

### Frontend (npm)
- `@tauri-apps/api` - Tauri bindings
- `svelte` - UI framework
- `tailwindcss` - Styling
- `@codemirror/lang-markdown` - Script editor
- `lucide-svelte` - Icons
- Native Svelte stores - State management

### External
- **None** (sandboxed mode) - Chrome auto-downloaded via System Chrome
- Chrome browser (optional, for advanced mode only)

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Chrome not found (sandboxed) | Auto-download via System Chrome on first run |
| Chrome not found (advanced) | Clear error message, guide to start Chrome with debug port |
| Chrome download fails | Retry logic, fallback to system Chrome detection |
| Debug port connection fails | Check if Chrome running with debug flag, clear instructions |
| Tauri learning curve | Extensive examples in docs |
| IPC complexity | Abstraction layer for commands |
| macOS signing issues | Clear documentation, CI setup |
| Performance with large scripts | Streaming events, pagination |

## CDP Script Architecture

### How Claude-Generated Scripts Work

**Flow:**
```
User describes task → Claude generates CDP JSON → Stored as text file → Runtime loads & executes
```

**Key Components:**

1. **Script Generator (Claude Integration)**
   - User provides natural language description
   - Claude CLI analyzes request
   - Outputs CDP command sequence as JSON
   - Saved to file (not compiled)

2. **Script Format (JSON)**
   ```json
   {
     "name": "automation-name",
     "description": "What it does",
     "cdp_commands": [
       {
         "method": "Page.navigate",
         "params": {"url": "https://example.com"}
       },
       {
         "method": "Runtime.evaluate",
         "params": {"expression": "document.title"}
       }
     ]
   }
   ```

3. **Runtime Interpreter**
   - Loads script from file at runtime
   - Parses JSON
   - Sends each CDP command to Chrome via chromiumoxide
   - Handles responses and errors

4. **Execution Engine**
   ```rust
   pub struct CdpScriptExecutor {
       driver: ChromeDriver,
   }

   impl CdpScriptExecutor {
       pub async fn execute_script(&self, script_path: &Path) -> Result<()> {
           // 1. Load JSON from file
           let script = self.load_script(script_path)?;

           // 2. Execute each CDP command
           for cmd in script.cdp_commands {
               self.execute_cdp_command(&cmd.method, cmd.params).await?;
           }

           Ok(())
       }

       async fn execute_cdp_command(
           &self,
           method: &str,
           params: serde_json::Value
       ) -> Result<serde_json::Value> {
           // Send raw CDP command to Chrome
           let page = self.driver.current_page().await?;
           page.execute_cdp(method, params).await
       }
   }
   ```

### Implementation Approach

**Phase 1: CDP Interpreter**
- Implement JSON script loader
- Add generic CDP command executor
- Handle responses and errors
- Support all CDP domains (Page, Runtime, Input, Network, etc.)

**Phase 2: Claude Integration**
- Connect Claude CLI for script generation
- Prompt engineering for CDP output
- Validate generated scripts
- Save to script directory

**Phase 3: UI Integration**
- "Describe automation" text field
- Claude generates script in background
- Show generated CDP commands
- Allow edit before execution
- Execute button runs interpreter

### Benefits of This Architecture

1. **No Compilation** - Scripts are text files, change without rebuilding
2. **Full CDP Access** - Any Chrome feature available
3. **AI-Generated** - Non-technical users can automate
4. **Auditable** - Scripts are readable JSON
5. **Shareable** - Export/import script files
6. **Flexible** - Can be hand-edited by power users

### Security Considerations

- **Sandboxed execution** - CDP commands run in isolated browser
- **No arbitrary code** - Only CDP protocol commands
- **User approval** - Scripts displayed before execution
- **Audit log** - All commands logged

## Next Steps

1. **Complete Phase 0 CLI** with chromiumoxide (both modes)
2. **Initialize Tauri project**
3. **Implement CDP script interpreter** - Generic CDP command executor
4. **Integrate Claude CLI** - For script generation
5. **Build script execution UI** - Load, display, execute CDP scripts
6. **Test with real automation tasks**

This revised plan delivers a much more user-friendly experience with visual feedback and zero external dependencies, making browser automation accessible to all users while maintaining the power of scripted automation and advanced features for power users.
