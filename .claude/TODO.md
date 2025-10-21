# TODO

## Current Focus: Command System Refactoring

**Why?** Phase 3 was implemented with a misunderstanding of the specification. Commands were stored as JSON files with simple `{{param}}` substitution, but the correct approach is:

- **Commands are markdown template files** that describe tasks for an AI agent
- The markdown includes instructions, parameters, rules, and success criteria
- **CDP JSON is optional** - the agent can generate it dynamically from the markdown description
- This enables natural language automation where users describe *what* to do, not *how*

**Impact**: We need to refactor from JSON-based commands to markdown-template-based commands. Since this hasn't been deployed to production, we can do a clean rewrite without backward compatibility concerns.

---

## In Progress
_No tasks currently in progress_

## Planned

### Next: Refactor Command System - Markdown Templates

**Context**: The current implementation (Phase 3) uses JSON files with simple `{{param}}` substitution. The correct approach is to use **markdown template files** that describe tasks for an AI agent. The markdown provides instructions, parameters, rules, and success criteria. CDP JSON is optional - the agent can generate it dynamically.

#### Backend Refactoring Tasks

**1. Update Command Data Models**
- [ ] Refactor `CommandConfig` to parse markdown with YAML frontmatter
  - Add dependencies: `pulldown-cmark` for markdown parsing
  - Already have `serde_yaml` for frontmatter
  - Parse markdown sections: Parameters, Rules, Checklist, Generative UI, CDP Script
  - Make CDP Script section optional
- [ ] Create markdown section parser
  - Extract `## Parameters`, `## Rules`, `## Checklist` sections
  - Parse optional `## CDP Script Template` section
  - Parse optional `## Generative UI` JSON block
- [ ] Update `CommandManager` to work with `.md` files
  - Change file extension from `.json` to `.md`
  - Update encryption to work with markdown content
  - Remove JSON-based `CommandConfig` struct (clean break)

**2. Implement AI Agent Integration**
- [ ] Create agent prompt builder for command execution
  - Include markdown template as context
  - Include user parameters
  - Include user-profile.md context
  - Request CDP command generation
- [ ] Implement CDP generation from markdown
  - Send markdown + parameters to AI agent
  - Parse CDP JSON response
  - Validate CDP commands before execution
- [ ] Add fallback to static CDP script
  - If markdown includes CDP Script section, use it
  - Support parameter substitution in static CDP
  - Prefer dynamic generation over static when available

**3. Update Command Storage**
- [ ] Implement markdown template storage
  - Save as `command-{name}.md` files
  - Encrypt markdown content
  - Preserve formatting and sections
- [ ] Delete JSON-based command files
  - Remove old `.json` command files
  - Clean up test fixtures
  - Update file references in code

**4. Update Command Executor**
- [ ] Refactor `CommandExecutor` for markdown templates
  - Load markdown template
  - Build AI prompt with parameters
  - Generate or retrieve CDP commands
  - Execute CDP commands
  - Return execution results
- [ ] Add validation for markdown structure
  - Validate YAML frontmatter
  - Check required sections present
  - Validate parameter definitions
- [ ] Handle optional CDP script section
  - Check if CDP section exists
  - Use static CDP if present, dynamic if absent
  - Log which approach was used

#### Frontend Refactoring Tasks

**5. Update Command Editor UI**
- [ ] Create markdown template editor
  - Syntax highlighting for markdown
  - YAML frontmatter editor section
  - Preview pane for rendered markdown
  - Template generator (help users create structure)
- [ ] Add section editors
  - Parameters section editor (structured form)
  - Rules section editor (bullet list)
  - Checklist section editor (checkbox list)
  - CDP Script section (optional, collapsible)
- [ ] Add AI assistance
  - "Generate command from description" button
  - AI suggests parameters, rules, checklist
  - Option to include or omit CDP script

**6. Update Command List UI**
- [ ] Show markdown-based command info
  - Display frontmatter metadata
  - Show parameter count
  - Show if CDP script is included or dynamic
- [ ] Add command preview
  - Render markdown in modal
  - Show all sections
  - Highlight what agent will see

**7. Update Command Executor UI**
- [ ] Keep existing parameter input forms
  - Generate form from markdown parameters
  - Support all parameter types
- [ ] Add execution mode indicator
  - Show "Dynamic CDP Generation" or "Static CDP Script"
  - Display agent prompt being used
  - Show generated CDP before execution (optional)

#### Testing Tasks

**8. Test Suite Updates**
- [ ] Unit tests for markdown parser
  - Test YAML frontmatter parsing
  - Test section extraction (Parameters, Rules, etc.)
  - Test optional CDP script handling
- [ ] Unit tests for AI prompt generation
  - Test prompt builder with all sections
  - Test parameter substitution in prompt
  - Validate prompt format
- [ ] Integration tests for command execution
  - Test dynamic CDP generation
  - Test static CDP execution
  - Test fallback behavior
- [ ] Remove old JSON command tests
  - Delete JSON-based test cases
  - Update test fixtures to use markdown
  - Verify all tests pass with new format

#### Documentation Tasks

**9. Update Documentation**
- [ ] Update command examples in docs
  - Show markdown template examples
  - Demonstrate optional CDP script
  - Show dynamic vs static approaches
- [ ] Create command authoring guide
  - How to write markdown templates
  - Best practices for parameters and rules
  - When to include CDP script vs dynamic
- [ ] Update API documentation
  - Document new command structure
  - Update Tauri command signatures
  - Remove references to JSON format

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
