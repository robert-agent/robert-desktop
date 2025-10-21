# In Progress
- [ ] Phase 2: Browser Profile Management Implementation
  - Implementing ChromeDriver launcher with ephemeral profiles
  - Creating basic session manager
  - Adding Tauri commands for browser lifecycle
  - Writing comprehensive tests

# TODO

### Phase 2 - Browser Profiles (CURRENT)

#### Backend Tasks
- [ ] Review existing profiles code in codebase
- [ ] Implement Browser Profile Types module (profile.rs)
  - BrowserProfile enum (Ephemeral vs Named)
  - Profile path resolution
  - Unit tests
- [ ] Implement ChromeDriver launcher with ephemeral profiles (launcher.rs)
  - Launch Chrome with temporary user-data-dir
  - Track active browser session
  - Cleanup temp directory on session close
  - Unit tests for launcher
- [ ] Create basic session manager (session.rs)
  - Single active session (keep it simple for Phase 2)
  - `launch_session()` and `close_session()` methods
  - Session state tracking
  - Unit tests for session lifecycle
- [ ] Implement ephemeral cleanup (cleanup.rs)
  - Cleanup temp directory on session close
  - Orphaned profile cleanup on app start
  - Unit tests for cleanup logic
- [ ] Add Tauri commands (commands/browser.rs)
  - `launch_browser()` → SessionId
  - `close_browser(session_id)` → Result
  - `get_browser_status()` → session info
  - Error handling and validation
- [ ] Write integration tests
  - Test browser launch and cleanup
  - Verify ephemeral profile deletion
  - Test session lifecycle end-to-end

#### Frontend Tasks
- [ ] Add "Launch Browser" button to main UI
- [ ] Show active browser session status
- [ ] Add "Close Browser" button
- [ ] Handle session lifecycle in state

**Deferred to Post-MVP:**
- Named browser profiles (user can manually log in to sites each time for now)
- Default profile selection
- Multiple simultaneous sessions
- Profile manager UI

---

### Next: Phase 3 - Simple Command System

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

---

## SOMEDAY:

**Browser Profiles:**
- ❌ Named profiles - Users can log into sites manually each session
- ❌ Profile persistence - Ephemeral only keeps things simple
- ❌ Profile selector UI - Only one mode for now
- ❌ Multiple simultaneous sessions - One at a time

**Command System:**
- ❌ Markdown parsing - JSON is simpler and sufficient
- ❌ YAML frontmatter - Not needed without markdown
- ❌ Versioning system - Can add when users request it
- ❌ Changelog tracking - Not critical for MVP
- ❌ AI-assisted creation - Manual creation works fine initially

**Generative UI (Entire Phase 4):**
- ❌ 8 component types (dropdown, slider, color picker, etc.) - Text inputs sufficient
- ❌ Layout system (vertical, two-column, grid) - Simple vertical list works
- ❌ Form validator - Basic HTML5 validation enough
- ❌ Chat integration for real-time updates - Not needed yet
