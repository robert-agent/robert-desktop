# TODO

## Current Focus: Frontend Updates for Markdown Commands

**Status**: Backend refactoring COMPLETE. All tests passing (82/82). All lints passing.

**What was done:**
- Implemented markdown parsing with YAML frontmatter
- Created new CommandManager for .md files
- Built AI prompt generation system
- Updated Tauri commands for markdown format
- Fixed all lint issues (Rust and TypeScript/Svelte)
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

**Backend Status**: COMPLETE - All tests passing (82/82), all lints clean

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

### Phase 3 - Markdown Command System ✅

**Status**: Completed 2025-10-21

#### Backend Tasks (Completed)
- [x] Implemented markdown parsing with YAML frontmatter
- [x] Created CommandManager for .md files
- [x] Built AI prompt generation system
- [x] Updated Tauri commands for markdown format
- [x] 100% test coverage (82/82 tests passing)
- [x] All lints passing (Rust and TypeScript)

#### Frontend Tasks (Partially Complete)
- [x] Basic command list UI (needs markdown metadata update)
- [x] Basic command editor (needs markdown template editor)
- [x] Basic command executor UI (needs AI integration)
- [x] Command manager container (working)
- [x] Integration into main UI (working)

**Note**: Frontend needs updates for markdown format, but basic structure is in place.

---

## Triage
_No tasks requiring triage_

## Won't Fix (Post-MVP)

**Browser Profiles (Deferred to v1.6+):**
- ❌ Named profiles - Users can log into sites manually each session
- ❌ Profile persistence - Ephemeral only keeps things simple
- ❌ Profile selector UI - Only one mode for now
- ❌ Multiple simultaneous sessions - One at a time

**Command System (Now Complete):**
- ✅ Markdown parsing - COMPLETE
- ✅ YAML frontmatter - COMPLETE
- ✅ AI prompt generation - COMPLETE
- ⚠️ Versioning system - Add in Phase 4 (changelog tracking)
- ⚠️ AI-assisted creation - Add in Phase 4 (generate from description)

**Generative UI (Deferred to Phase 4):**
- ⚠️ 8 component types (dropdown, slider, color picker, etc.) - Add in Phase 4
- ⚠️ Layout system (vertical, two-column, grid) - Add in Phase 4
- ⚠️ Form validator - Add in Phase 4
- ⚠️ Chat integration for real-time updates - Add in Phase 4

**Note**: Items marked with ⚠️ are deferred but planned. Items marked with ❌ are indefinitely postponed. Items marked with ✅ are complete.
