# TODO

## In Progress
- [ ] Phase 1: User Management (v1.5) - Complete authentication and frontend integration

## Planned

### Phase 1: User Management (v1.5) - 2 weeks - BACKEND COMPLETE
Backend Foundation (✅ COMPLETE):
- [x] Add crypto dependencies (argon2, aes-gcm, rand, zeroize, uuid, directories)
- [x] Implement crypto module (Argon2id key derivation, AES-256-GCM encryption)
- [x] Implement storage module (filesystem operations, encrypted file I/O)
- [x] Implement user manager (CRUD operations, directory structure)
- [x] Write unit tests for crypto (14 tests, all passing)
- [x] Write unit tests for storage (8 tests, all passing)
- [x] Write unit tests for manager (4 tests, all passing)
- [x] Fix failing test in crypto module (salt length assertion)

Remaining Phase 1 Tasks:
- [ ] Implement authentication service (auth.rs - login, logout, session management)
- [ ] Add Tauri commands for user operations (create_user, login_user, logout_user, list_users, switch_user)
- [ ] Implement app state management with active user and encryption key
- [ ] Create login screen UI (profile selector dropdown, password input)
- [ ] Create user creation form UI (username, password with strength indicator)
- [ ] Create user state store in Svelte (reactive state management)
- [ ] Implement profile switching UI
- [ ] Create user profile editor UI (markdown editor for user-profile.md)
- [ ] Write integration tests for auth flows (create → login → logout → switch)

### Phase 2: Browser Profile Management (v1.6) - 2 weeks
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

### Phase 3: Command System (v1.7) - 3 weeks
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

### Phase 4: Generative UI (v1.8) - 2 weeks
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

### Phase 5: Command Refinement (v1.9) - 2 weeks
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

## Completed

### Documentation & Planning
- [x] Read all profiles documentation (PROFILES.md, PROFILES_QUESTIONS.md, USER_PROFILES_PRIVACY.md, PRD.md)
- [x] Create comprehensive implementation plan (.claude/profiles-implementation-plan.md)
- [x] Review existing project structure

### Phase 1: User Management - Backend Implementation (✅ COMPLETE)
- [x] Add all required dependencies to Cargo.toml (argon2, aes-gcm, rand, zeroize, uuid, directories, pulldown-cmark, serde_yaml)
- [x] Create profiles module structure (mod.rs, types.rs, crypto.rs, storage.rs, manager.rs)
- [x] Implement types module with all data structures (UserConfig, BrowserProfile, Command types, GenerativeUI, ExecutionContext)
- [x] Implement crypto module with Argon2id password hashing and AES-256-GCM encryption
- [x] Implement storage module with filesystem operations and encrypted file I/O
- [x] Implement user manager module with CRUD operations and validation
- [x] Write and pass all unit tests for crypto module (14 tests, 100% passing)
- [x] Write and pass all unit tests for storage module (8 tests, 100% passing)
- [x] Write and pass all unit tests for manager module (4 tests, 100% passing)
- [x] Fix failing test in crypto module (salt base64 length assertion)
- [x] Verify compilation succeeds with all new code (26 total tests passing)

## Triage
- [ ] Determine if PBKDF2 fallback is needed or Argon2id only
- [ ] Decide on password strength requirements (min length, complexity)
- [ ] Clarify performance benchmarks for key derivation
- [ ] Determine cache strategy for decrypted commands
- [ ] Decide on hard limit for concurrent browser sessions
- [ ] Clarify markdown error handling strategy (auto-fix vs strict)

## Won't Fix
- [ ] Password recovery mechanism (explicitly not supported for security)
- [ ] Command sharing between users in v1 (deferred to v2.0)
- [ ] Import existing Chrome profiles (complexity + security concerns)
- [ ] Multi-user simultaneous login (single-user lock enforced)
