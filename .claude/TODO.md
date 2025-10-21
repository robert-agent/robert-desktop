# TODO

## Current Focus: Frontend Updates for Markdown Commands

**Status**: Backend refactoring COMPLETE. All tests passing (82/82).

**What was done:**
- Implemented markdown parsing with YAML frontmatter
- Created new CommandManager for .md files
- Built AI prompt generation system
- Updated Tauri commands for markdown format
- 100% test coverage on new code

**What's needed:**
- Update frontend command editor for markdown templates
- Implement AI integration in frontend
- Update command list and executor UI

See `REFACTORING_SUMMARY.md` for complete details.

---

## In Progress
_No tasks currently in progress_

## Planned

### Next: Frontend Updates for Markdown Commands

**Backend Status**: COMPLETE - All tests passing (82/82)

#### Frontend Tasks (URGENT)

**1. Update Command Editor UI**
- [ ] Create markdown template editor component
  - Syntax highlighting for markdown
  - YAML frontmatter editor section
  - Preview pane for rendered markdown
  - Template generator helper
- [ ] Add section editors
  - Parameters section editor (structured form)
  - Rules section editor (bullet list)
  - Checklist section editor (checkbox list)
  - CDP Script section (optional, collapsible)

**2. Update Command List UI**
- [ ] Show markdown-based command metadata
  - Display frontmatter (version, description, etc.)
  - Show parameter count
  - Show if CDP script is static or dynamic
- [ ] Add command preview modal
  - Render markdown
  - Show all sections

**3. Implement AI Integration**
- [ ] Add AI service communication
  - Call `build_command_prompt()` to get prompt
  - Send prompt to AI service (Claude, OpenAI, etc.)
  - Parse CDP JSON from AI response
  - Error handling for AI failures
- [ ] Add fallback to static CDP
  - Call `get_static_cdp()` if AI unavailable
  - Show which mode is being used

**4. Update Command Executor UI**
- [ ] Generate parameter forms from Command.parameters
- [ ] Show execution mode indicator
  - "Dynamic CDP Generation" or "Static CDP Script"
  - Display agent prompt (optional)
  - Show generated CDP before execution (optional)

#### Backend Cleanup

**5. Deprecation Cleanup**
- [ ] Remove old JSON-based command.rs file (marked deprecated)
- [ ] Remove JSON command tests
- [ ] Delete any JSON command fixtures

#### Documentation

**6. User Documentation**
- [ ] Create command authoring guide
  - How to write markdown templates
  - Best practices
  - Example templates
- [ ] Update API documentation for frontend
  - New command structure
  - Tauri command signatures
  - Migration guide

---

### Phase 2 - Browser profiles

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

### Phase 3 - Simple Command System ✅ (Being Refactored)

**Note**: Phase 3 was completed but implemented with a misunderstanding. Commands were stored as JSON with `{{param}}` substitution. This is being refactored to use markdown templates as specified in the PRD.

#### Backend Tasks (Completed - Will be refactored)
- [x] ~~JSON-based command structure~~ (Being replaced with markdown templates)
  - `CommandConfig` struct with: name, description, script, parameters
  - Store as encrypted JSON files
- [x] Basic command manager (Will be adapted for markdown)
  - `save_command(name, config)`
  - `load_command(name)`
  - `list_commands()` → Vec<CommandInfo>
  - `delete_command(name)`
- [x] Basic command executor (Will be refactored for AI integration)
  - Load command config
  - Substitute parameter values into script
  - Execute in browser session
  - Return results
- [x] Tauri commands (Signatures will change for markdown)
  - `save_command(name, config)`
  - `list_commands()`
  - `get_command(name)`
  - `execute_command(name, params)`
  - `delete_command(name)`

#### Frontend Tasks (Completed - Will be refactored)
- [x] Command list UI (Will show markdown-based commands)
- [x] Command editor (Will become markdown template editor)
- [x] Command executor UI (Will add execution mode indicator)
- [x] Command manager container (Structure will remain similar)
- [x] Integration into main UI (No changes needed)

---

## Triage
_No tasks requiring triage_

## Won't Fix (Post-MVP)

**Browser Profiles (Deferred to v1.6+):**
- ❌ Named profiles - Users can log into sites manually each session
- ❌ Profile persistence - Ephemeral only keeps things simple
- ❌ Profile selector UI - Only one mode for now
- ❌ Multiple simultaneous sessions - One at a time

**Command System (Now Required - Misunderstood Initially):**
- ✅ Markdown parsing - **REQUIRED** - Commands are markdown templates
- ✅ YAML frontmatter - **REQUIRED** - Metadata for commands
- ⚠️ Versioning system - Add in Phase 4 (changelog tracking)
- ⚠️ AI-assisted creation - Add in Phase 4 (generate from description)

**Generative UI (Deferred to Phase 4):**
- ⚠️ 8 component types (dropdown, slider, color picker, etc.) - Add in Phase 4
- ⚠️ Layout system (vertical, two-column, grid) - Add in Phase 4
- ⚠️ Form validator - Add in Phase 4
- ⚠️ Chat integration for real-time updates - Add in Phase 4

**Note**: Items marked with ⚠️ are deferred but planned. Items marked with ❌ are indefinitely postponed. Items marked with ✅ were incorrectly marked as "Won't Fix" and are now required.
