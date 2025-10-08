# Browser Automation Tool - Revised Implementation Plan
## Desktop App with Tauri for macOS

## Overview

A user-friendly browser automation tool with a **Tauri desktop app** for macOS, featuring real-time automation status viewing and visual feedback.

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
- **Browser Automation**: thirtyfour (WebDriver)
- **Browser Driver**: chromedriver
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
thirtyfour = "0.32"
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"

# Utilities
anyhow = "1.0"
thiserror = "1.0"
serde_yaml = "0.9"
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
│       │   │   ├── parser.rs        # YAML parser
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
│       │   ├── basic_navigation.yaml
│       │   └── form_interaction.yaml
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
│  Script: login_flow.yaml                                     │
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

### Phase 1: Tauri Setup & Basic Browser Control (Week 1)

#### Milestone 1.1: Tauri Project Setup
**Tasks:**
1. Initialize Tauri project: `npm create tauri-app`
2. Configure `tauri.conf.json`:
   - App name, identifier
   - Window size (1280x800)
   - macOS permissions (file access, network)
   - Build settings
3. Setup React frontend with Vite
4. Install dependencies (Rust + npm)
5. Test basic Tauri app launch

**Deliverables:**
- Tauri app launches on macOS
- Basic "Hello World" window
- Hot-reload working

#### Milestone 1.2: Rust Backend - Browser Control
**Tasks:**
1. Add thirtyfour to Cargo.toml
2. Implement `browser/chrome.rs` with thirtyfour
3. Setup chromedriver auto-download or bundling
4. Implement browser launch/close
5. Add Tauri commands: `launch_browser`, `close_browser`
6. Handle macOS Chrome path detection

**Code Example:**
```rust
// src-tauri/browser/chrome.rs
use thirtyfour::prelude::*;

pub struct ChromeDriver {
    driver: WebDriver,
}

impl ChromeDriver {
    pub async fn new() -> Result<Self, BrowserError> {
        let mut caps = DesiredCapabilities::chrome();
        caps.set_window_size(1280, 1024)?;

        // macOS Chrome paths
        let chrome_path = Self::find_chrome_macos()?;
        caps.add_chrome_arg(&format!("--binary={}", chrome_path))?;

        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        Ok(Self { driver })
    }

    fn find_chrome_macos() -> Result<String, BrowserError> {
        let paths = vec![
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
        ];

        for path in paths {
            if std::path::Path::new(path).exists() {
                return Ok(path.to_string());
            }
        }

        Err(BrowserError::ChromeNotFound)
    }
}
```

**Deliverables:**
- Chrome launches from Tauri app
- Frontend button to launch/close browser
- Error handling for Chrome not found

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
1. Implement YAML script parser
2. Add Tauri commands: `load_script`, `validate_script`
3. Frontend: Script editor with syntax highlighting (monaco-editor)
4. File picker for loading scripts
5. Script validation UI

**Deliverables:**
- Load and parse YAML scripts
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
- [x] Tauri app launches on macOS
- [x] Chrome automation working
- [x] Script loading and parsing
- [x] Real-time execution status display
- [x] Screenshot and text capture
- [x] Output management and viewing
- [x] All interaction actions working

### UX
- [x] Intuitive UI (user testing)
- [x] Real-time feedback
- [x] Clear error messages
- [x] Native macOS feel
- [x] Responsive performance

### Technical
- [x] <3 second app launch
- [x] <2 second screenshot capture
- [x] <500MB memory footprint
- [x] >70% test coverage
- [x] No memory leaks

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
- `thirtyfour` - Browser automation
- `tokio` - Async runtime
- `serde`, `serde_json`, `serde_yaml` - Serialization
- `uuid`, `chrono` - Utilities
- `tracing` - Logging
- `image`, `base64` - Image handling
- `anyhow`, `thiserror` - Error handling

### Frontend (npm)
- `@tauri-apps/api` - Tauri bindings
- `svelte` - UI framework
- `tailwindcss` - Styling
- `@codemirror/lang-yaml` - Script editor
- `lucide-svelte` - Icons
- Native Svelte stores - State management

### External
- `chromedriver` - WebDriver for Chrome
- Chrome browser (user installation)

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Chrome not found | Path detection + manual configuration |
| chromedriver version mismatch | Auto-download compatible version |
| Tauri learning curve | Extensive examples in docs |
| IPC complexity | Abstraction layer for commands |
| macOS signing issues | Clear documentation, CI setup |
| Performance with large scripts | Streaming events, pagination |

## Next Steps

1. **Initialize Tauri project**
2. **Setup basic browser automation** with thirtyfour
3. **Implement core IPC** commands
4. **Build execution status UI**
5. **Iterate with user testing**

This revised plan delivers a much more user-friendly experience with visual feedback, making browser automation accessible to non-technical users while maintaining the power of scripted automation.
