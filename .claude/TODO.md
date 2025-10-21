# TODO

## Project Overview
This project combines two main components:
1. **Robert Server** - Remote execution server for Claude CLI
2. **Robert Desktop** - Tauri-based desktop application with user profiles and browser automation

---

## Robert Server (Remote Execution)

### In Progress
_No tasks currently in progress_

### Planned
- [ ] Phase 5: Production Readiness - Add TLS support
- [ ] Phase 5: Production Readiness - Add metrics endpoint
- [ ] Phase 5: Production Readiness - Performance benchmarks
- [ ] Phase 5: Production Readiness - Error handling polish

### Completed
- [x] Phase 1: Foundation (data models, config, health endpoint, all tests passing)
- [x] Phase 2: Auth & Middleware (token auth, rate limiting, all tests passing)
- [x] Phase 3: Claude CLI Integration (process spawning, streaming, timeout/cleanup, tested with real CLI)
- [x] Phase 4: Execute Endpoint (SSE streaming, session management, integration tests)
- [x] Phase 5: Documentation and README
- [x] Pull request created and merged (PR #16 merged on 2025-10-17)

---

## Robert Desktop (Tauri Application)

### In Progress
_No tasks currently in progress_

### Planned

#### Phase 2: Browser Profile Management (v1.6)
- [ ] Define BrowserProfile enum (Ephemeral, Named)
- [ ] Implement ephemeral profile lifecycle (create, cleanup)
- [ ] Implement named profile management (CRUD)
- [ ] Implement profile selection priority logic
- [ ] Implement session manager (multiple simultaneous sessions)
- [ ] Add Tauri commands for browser profiles
- [ ] Create browser profile manager UI
- [ ] Create profile selector dropdown in chat interface
- [ ] Create active sessions panel UI
- [ ] Write unit tests for profile management
- [ ] Write integration tests for session lifecycle

#### Phase 3: Command System (v1.7)
- [ ] Define command schema (CommandFrontmatter, CommandParameter, etc.)
- [ ] Implement markdown parser (YAML frontmatter, sections)
- [ ] Implement command manager (save, load, list, delete)
- [ ] Implement command versioning (semver, changelog)
- [ ] Implement command execution logic (profile resolution, parameter substitution)
- [ ] Add Tauri commands for command operations
- [ ] Create command dropdown UI
- [ ] Create command creator workflow (AI-assisted)
- [ ] Create command editor UI (markdown with validation)
- [ ] Create command preview UI
- [ ] Create command list UI
- [ ] Create command executor UI (parameter form)
- [ ] Write unit tests for parser and schema
- [ ] Write integration tests for command workflows

#### Phase 4: Generative UI (v1.8)
- [ ] Define UI component schema (all 8 types)
- [ ] Implement JSON schema validation
- [ ] Add Tauri commands for generative UI validation
- [ ] Implement all 8 UI components (text, dropdown, slider, etc.)
- [ ] Implement layout renderers (vertical, two-column, grid)
- [ ] Implement form validator
- [ ] Create UIRenderer component
- [ ] Integrate with chat for real-time parameter updates
- [ ] Write unit tests for components and layouts
- [ ] Write integration tests for form rendering

#### Phase 5: Command Refinement (v1.9)
- [ ] Implement feedback capture (CommandFeedback struct)
- [ ] Implement refinement workflow (AI integration)
- [ ] Implement version control (history, rollback)
- [ ] Add Tauri commands for refinement
- [ ] Create feedback buttons UI (thumbs up/down)
- [ ] Create refinement modal UI
- [ ] Create version history UI
- [ ] Create changelog display UI
- [ ] Write unit tests for feedback and versioning
- [ ] Write integration tests for refinement workflow

### Completed

#### Test Coverage & Documentation - ✅ COMPLETE
- [x] Verified all critical functions are covered by unit tests
- [x] Added 2 new unit tests for `create_and_login` (success + duplicate user detection)
- [x] Made `profiles` module public to enable doctest verification
- [x] Fixed all 5 doctests to compile with `no_run` (verifies examples, doesn't execute)
- [x] Fixed file structure diagram in storage.rs to use `text` annotation
- [x] All 61 unit tests passing (crypto: 14, storage: 8, manager: 4, auth: 7)
- [x] All 5 doctests compiling and verifying
- [x] Total test coverage: 61 unit tests + 5 integration tests + 3 developer mode tests + 5 doctests = 74 tests passing

#### Storage Layer Refactor - ✅ COMPLETE
- [x] Update storage.rs path functions to accept optional base_dir parameter
- [x] Update auth.rs AuthService methods to accept base_dir parameter
- [x] Update manager.rs UserManager methods to accept base_dir parameter
- [x] Update all tests to pass temp_dir instead of modifying HOME env var
- [x] Remove serial_test dependency from Cargo.toml
- [x] Verify unit tests pass with --test-threads=8 (59 tests passing in 12.44s)

#### Phase 1: User Management (v1.5) - ✅ 100% COMPLETE
- [x] Documentation & planning (read all specs, create implementation plan)
- [x] Backend: Core modules (profiles structure, types, all dependencies added)
- [x] Backend: Security & crypto (Argon2id, AES-256-GCM, key derivation, zeroize)
- [x] Backend: Storage & file operations (encrypted I/O, user directories, validation)
- [x] Backend: User management (CRUD, password validation, username validation)
- [x] Backend: Authentication (login, logout, session management, password verification)
- [x] Backend: App integration (AppState extension, 7 Tauri commands, all registered)
- [x] Backend: Testing & quality (31 tests passing, cargo xlint passing)
- [x] Frontend: TypeScript types, user state store, all UI components
- [x] Frontend: Login screen, user creation form, profile switcher, profile editor
- [x] Frontend: App.svelte integration (auth flow, first launch detection)
- [x] Frontend: All code quality checks passing (bun lint, bun check)

---

## Triage & Decisions Needed

### Robert Server
- [ ] Docker Compose setup for local testing (optional for now)
- [ ] VS Code launch configuration (optional for now)
- [ ] Enterprise features (Phase 4 - future scope)

### Robert Desktop
- [ ] Determine if PBKDF2 fallback is needed or Argon2id only
- [ ] Decide on password strength requirements (min length, complexity)
- [ ] Clarify performance benchmarks for key derivation
- [ ] Determine cache strategy for decrypted commands
- [ ] Decide on hard limit for concurrent browser sessions
- [ ] Clarify markdown error handling strategy (auto-fix vs strict)

---

## Won't Fix / Out of Scope

### Robert Server
- [ ] OAuth2/mTLS advanced authentication (out of scope for MVP)
- [ ] Multi-tenancy support (future feature)
- [ ] HA deployment patterns (deployment concern, not implementation)

### Robert Desktop
- [ ] Password recovery mechanism (explicitly not supported for security)
- [ ] Command sharing between users in v1 (deferred to v2.0)
- [ ] Import existing Chrome profiles (complexity + security concerns)
- [ ] Multi-user simultaneous login (single-user lock enforced)
