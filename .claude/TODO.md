# TODO

## Current Status: Markdown Command System Complete! 🎉

**All implementation finished**: Backend + Frontend fully refactored from JSON to Markdown templates

**Test Results:**
- ✅ 131 tests passing (106 backend + 25 integration)
- ✅ 0 lint errors (Rust, TypeScript, Svelte)
- ✅ 0 build warnings
- ✅ Full type safety with TypeScript

**What was built:**
- Markdown parsing with YAML frontmatter
- AI prompt generation for dynamic CDP
- Static CDP fallback support
- Complete UI rewrite (Editor, List, Executor)
- 8 parameter input types supported
- Live markdown preview
- Version badges, browser profile badges

**Next:** Manual testing and cleanup

---

## In Progress

### Manual Testing (User Testing Required)
- [ ] Test command creation flow (new → edit → save)
- [ ] Test all 8 parameter input types:
  - [ ] Text Input (textarea)
  - [ ] Short Text (input)
  - [ ] Dropdown (select)
  - [ ] Radio buttons
  - [ ] Checkbox
  - [ ] Slider
  - [ ] Color picker
  - [ ] Date picker
- [ ] Test AI prompt generation workflow
- [ ] Test static CDP fallback
- [ ] Test markdown preview accuracy
- [ ] Test command list display (version, badges)
- [ ] Test command execution with parameters
- [ ] Test edit existing command
- [ ] Test delete command

---

## Planned

### High Priority: Cleanup

**Deprecation Cleanup**
- [ ] Remove `crates/robert-app/src-tauri/src/profiles/command.rs` (deprecated JSON system)
- [ ] Remove JSON command tests from deprecated file
- [ ] Clean up any JSON test fixtures
- [ ] Update documentation references

**Documentation**
- [ ] Create command authoring guide
  - How to write markdown templates
  - Best practices for parameters and rules
  - Example templates (navigation, form filling, data extraction)
- [ ] Add inline examples in UI
  - Template generator with sample commands
  - Tooltips explaining each section

---

### Next Phase: Browser Automation (Phase 2)

**Backend Tasks**
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

**Frontend Tasks**
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

### Future: Robert Server (Remote Execution)

- [ ] Phase 5: Production Readiness - Add TLS support
- [ ] Phase 5: Production Readiness - Add metrics endpoint
- [ ] Phase 5: Production Readiness - Performance benchmarks
- [ ] Phase 5: Production Readiness - Error handling polish

---

## Completed

### Phase 3: Markdown Command System ✅ (Completed 2025-10-21)

**Backend Implementation**
- [x] Created markdown parser (`markdown.rs` - 450 lines)
  - YAML frontmatter parsing
  - Section extraction (Parameters, Rules, Checklist, CDP Script)
  - Template generation for roundtrip
  - 9 comprehensive unit tests
- [x] Created command manager (`command_md.rs` - 750 lines)
  - CommandManager for .md file storage
  - AI prompt builder
  - Static CDP fallback
  - Parameter validation
  - 7 comprehensive unit tests
- [x] Updated Tauri commands
  - `save_command()`, `get_command()`, `list_commands()`, `delete_command()`
  - NEW: `build_command_prompt()` for AI integration
  - NEW: `get_static_cdp()` for static fallback
- [x] Added dependencies: `pulldown-cmark` for markdown parsing
- [x] All tests passing (131 total: 106 backend + 25 integration)
- [x] All lints clean (cargo fmt, cargo xlint, cargo machete)

**Frontend Implementation**
- [x] Created TypeScript types (`lib/types.ts`)
  - Command, CommandFrontmatter, CommandParameter
  - 8 ParameterType variants
  - CommandInfo, GenerativeUI
- [x] Created Tauri API wrapper (`lib/tauri.ts`)
  - All command operations wrapped
  - AI prompt and static CDP functions
- [x] Rewrote CommandEditor (600+ lines)
  - Frontmatter editor (name, description, version, profile)
  - Parameters section with add/edit/delete for 8 types
  - Rules editor (bullet list)
  - Checklist editor (success criteria)
  - Optional CDP script section (collapsible JSON editor)
  - Live markdown preview (toggle-able)
- [x] Updated CommandList
  - Version badges (v1.0.0)
  - Browser profile badges
  - Parameter count display
  - Improved card layout
- [x] Rewrote CommandExecutor (500+ lines)
  - Dynamic parameter forms for all 8 input types
  - Execution mode selector (AI vs Static)
  - AI workflow: Generate prompt → Copy → Paste CDP → Execute
  - Static workflow: Show CDP → Execute
  - Detailed execution reports
- [x] Fixed all ESLint and TypeScript errors (0 errors, 0 warnings)
- [x] Fixed all Svelte check errors (0 errors, 0 warnings)

**Documentation**
- [x] Created `REFACTORING_SUMMARY.md` with:
  - Complete architecture overview
  - Migration guide
  - API reference
  - Performance notes
  - Security considerations

**Files Changed:**
- New: `markdown.rs`, `command_md.rs`, `REFACTORING_SUMMARY.md`
- Modified: `Cargo.toml`, `types.ts`, `tauri.ts`, all command components
- Deprecated: `command.rs` (old JSON system)

---

## Triage
_No tasks requiring triage_

---

## Won't Fix (Post-MVP)

**Browser Profiles (Deferred to v1.6+):**
- ❌ Named profiles - Users can log into sites manually each session
- ❌ Profile persistence - Ephemeral only keeps things simple
- ❌ Profile selector UI - Only one mode for now
- ❌ Multiple simultaneous sessions - One at a time

**Command System (Complete):**
- ✅ Markdown parsing - COMPLETE
- ✅ YAML frontmatter - COMPLETE
- ✅ AI prompt generation - COMPLETE
- ✅ 8 parameter types - COMPLETE
- ⚠️ Versioning system - Deferred to Phase 4 (changelog tracking)
- ⚠️ AI-assisted command creation - Deferred to Phase 4 (generate from description)

**Generative UI (Deferred to Phase 4):**
- ⚠️ Custom layouts (vertical, two-column, grid) - Deferred
- ⚠️ Advanced validators - Deferred
- ⚠️ Chat integration for real-time updates - Deferred

**Legend:**
- ✅ Complete
- ⚠️ Deferred but planned
- ❌ Indefinitely postponed

---

**Last Updated:** 2025-10-21
**Next Milestone:** Manual testing → Phase 2 (Browser automation)
