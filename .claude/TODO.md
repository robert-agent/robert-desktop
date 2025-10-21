# TODO

## In Progress
_No tasks currently in progress_

## Planned

### Next: Phase 2 - Browser profiles

#### Backend Tasks
- [ ] Implement ChromeDriver launcher with ephemeral profiles
  - Launch Chrome with temporary user-data-dir
  - Track active browser session
  - Cleanup temp directory on session close
- [ ] Create basic session manager
  - Single active session (keep it simple)
  - `launch_session()` and `close_session()` methods
- [ ] Add Tauri commands
  - `launch_browser()` → SessionId
  - `close_browser(session_id)` → Result
  - `get_browser_status()` → session info
- [ ] Write integration tests
  - Test browser launch and cleanup
  - Verify ephemeral profile deletion

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

### Robert Server (Remote Execution)

- [ ] Phase 5: Production Readiness - Add TLS support
- [ ] Phase 5: Production Readiness - Add metrics endpoint
- [ ] Phase 5: Production Readiness - Performance benchmarks
- [ ] Phase 5: Production Readiness - Error handling polish

---

## Completed

### Phase 3 - Simple Command System ✅

#### Backend Tasks
- [x] Simplify command structure (JSON-based, skip markdown parser)
  - `CommandConfig` struct with: name, description, script, parameters
  - Store as encrypted JSON files (not markdown)
- [x] Implement basic command manager
  - `save_command(name, config)`
  - `load_command(name)`
  - `list_commands()` → Vec<CommandInfo>
  - `delete_command(name)`
- [x] Implement basic command executor
  - Load command config
  - Substitute parameter values into script ({{param}} syntax)
  - Execute in browser session
  - Return results
- [x] Add Tauri commands
  - `save_command(name, config)`
  - `list_commands()`
  - `get_command(name)`
  - `execute_command(name, params)`
  - `delete_command(name)`

#### Frontend Tasks
- [x] Create simple command list UI (CommandList.svelte)
  - Display saved commands
  - Edit/Delete buttons
- [x] Create basic command editor (CommandEditor.svelte)
  - Name, description inputs
  - Simple text area for script
  - Parameter definitions (name, type, required, default)
- [x] Create command executor UI (CommandExecutor.svelte)
  - Select command from dropdown
  - Simple form inputs for parameters (text, number, boolean)
  - Execute button
  - Display results
- [x] Create command manager container (CommandManager.svelte)
  - View routing between list, create, edit, and execute views
- [x] Integrate CommandManager into main UI (App.svelte)
  - Added "Commands" menu item
  - Added commands view to main content area

---

## Triage
_No tasks requiring triage_

## Won't Fix

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
