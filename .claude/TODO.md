# TODO

## In Progress
- [ ] PR review and CI checks for Phase 2: Browser Profile Management
  - PR #24: https://github.com/robert-agent/robert-desktop/pull/24
  - Waiting for CI checks to complete

## Planned

### Phase 3 - Simple Command System

**Goal:** Allow users to save and run simple automation commands

#### Backend Tasks
- [ ] Simplify command structure (JSON-based, skip markdown parser)
  - `CommandConfig` struct with: name, description, script, parameters
  - Store as encrypted JSON files (not markdown)
- [ ] Implement basic command manager
  - `save_command(name, config)`
  - `load_command(name)`
  - `list_commands()` → Vec<CommandInfo>
  - `delete_command(name)`
- [ ] Implement basic command executor
  - Load command config
  - Substitute parameter values into script
  - Execute in browser session
  - Return results
- [ ] Add Tauri commands
  - `save_command(name, config)`
  - `list_commands()`
  - `get_command(name)`
  - `execute_command(name, params)`
  - `delete_command(name)`

#### Frontend Tasks
- [ ] Create simple command list UI
  - Display saved commands
  - Edit/Delete buttons
- [ ] Create basic command editor
  - Name, description inputs
  - Simple text area for script
  - Parameter definitions (name, type, required)
- [ ] Create command executor UI
  - Select command from dropdown
  - Simple form inputs for parameters (text only for MVP)
  - Execute button
  - Display results

### Robert Server (Remote Execution)

- [ ] Phase 5: Production Readiness - Add TLS support
- [ ] Phase 5: Production Readiness - Add metrics endpoint
- [ ] Phase 5: Production Readiness - Performance benchmarks
- [ ] Phase 5: Production Readiness - Error handling polish

## Completed

### Phase 2 - Browser Profiles ✅
- [x] Review existing profiles code in codebase
- [x] Implement Browser Profile Types module (profile.rs:1-469)
  - BrowserProfile enum (Ephemeral vs Named)
  - Profile path resolution with temp directory management
  - Automatic cleanup on session close
  - Comprehensive unit tests
- [x] Implement ChromeDriver launcher with ephemeral profiles (launcher.rs:1-393)
  - Launch Chrome with temporary user-data-dir
  - Track active browser session
  - BrowserConfig for headless/sandbox options
  - Auto-detection for CI environments
  - Unit tests for launcher
- [x] Create session manager (session.rs:1-537)
  - Single active session limit (Phase 2 constraint)
  - `launch_session()` and `close_session()` methods
  - Session state tracking with UUID generation
  - Thread-safe implementation using Arc<RwLock<HashMap>>
  - Comprehensive unit tests
- [x] Implement ephemeral cleanup
  - Cleanup temp directory on session close
  - Orphaned profile cleanup on app start
  - Unit tests for cleanup logic
- [x] Add Tauri commands (commands/browser.rs:1-299)
  - `launch_browser_session()` → SessionId with headless option
  - `close_browser_session(session_id)` → Result
  - `get_browser_status()` → session info
  - `close_all_browser_sessions()` → cleanup all
  - Error handling and validation
- [x] Write integration tests (browser_session_integration.rs:1-273)
  - Test browser launch and cleanup
  - Verify ephemeral profile deletion
  - Test session lifecycle end-to-end
  - Max sessions limit enforcement
  - 78 total tests passing
- [x] Add "Launch Browser" button to main UI
- [x] Show active browser session status (BrowserSessionManager.svelte:1-493)
- [x] Add "Close Browser" button
- [x] Handle session lifecycle in state with real-time polling
- [x] Disable doctests for tauri app crate (Cargo.toml, lib.rs:1-6)
- [x] All code quality checks passing (lint, format, type checks)
- [x] Create pull request #24

## Triage

**Deferred to Post-MVP:**
- [ ] Named browser profiles (user can manually log in to sites each time for now)
- [ ] Default profile selection
- [ ] Multiple simultaneous sessions
- [ ] Profile manager UI
- [ ] Custom user-data-dir in ChromeDriver (requires enhancement)

## Won't Fix

### Browser Profiles:
- ❌ Named profiles - Users can log into sites manually each session
- ❌ Profile persistence - Ephemeral only keeps things simple
- ❌ Profile selector UI - Only one mode for now
- ❌ Multiple simultaneous sessions - One at a time

### Command System:
- ❌ Markdown parsing - JSON is simpler and sufficient
- ❌ YAML frontmatter - Not needed without markdown
- ❌ Versioning system - Can add when users request it
- ❌ Changelog tracking - Not critical for MVP
- ❌ AI-assisted creation - Manual creation works fine initially

### Generative UI (Entire Phase 4):
- ❌ 8 component types (dropdown, slider, color picker, etc.) - Text inputs sufficient
- ❌ Layout system (vertical, two-column, grid) - Simple vertical list works
- ❌ Form validator - Basic HTML5 validation enough
- ❌ Chat integration for real-time updates - Not needed yet
