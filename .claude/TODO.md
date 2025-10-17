# TODO

## Project: Robert Server (Remote Execution)

### In Progress
- [ ] Create pull request for server branch

### Planned
- [ ] Phase 5: Production Readiness - Add TLS support
- [ ] Phase 5: Production Readiness - Add metrics endpoint
- [ ] Phase 5: Production Readiness - Performance benchmarks
- [ ] Phase 5: Production Readiness - Error handling polish

### Completed
- [x] Read specification document
- [x] Set up TODO tracking
- [x] Create robert-server project structure and Cargo.toml configuration
- [x] Phase 1: Foundation - Define core data models (RobertRequest, ClaudeEvent, etc.)
- [x] Phase 1: Foundation - Write tests for request parsing/validation
- [x] Phase 1: Foundation - Implement configuration loading
- [x] Phase 1: Foundation - Write tests for config validation
- [x] Phase 1: Foundation - Implement basic health endpoint
- [x] Phase 1: Foundation - Write integration test for health endpoint
- [x] Phase 2: Auth & Middleware - Write tests for token authentication
- [x] Phase 2: Auth & Middleware - Implement auth middleware
- [x] Phase 2: Auth & Middleware - Write tests for rate limiting
- [x] Phase 2: Auth & Middleware - Implement rate limiting middleware
- [x] Phase 2: Auth & Middleware - Integration tests for auth flow
- [x] Phase 3: Claude CLI Integration - Write tests for executor interface
- [x] Phase 3: Claude CLI Integration - Implement claude-cli process spawning (real only, no mocks)
- [x] Phase 3: Claude CLI Integration - Implement stdout/stderr streaming
- [x] Phase 3: Claude CLI Integration - Add timeout and cleanup logic
- [x] Phase 3: Claude CLI Integration - Test with real claude-cli commands
- [x] Phase 4: Execute Endpoint - Write tests for execute endpoint
- [x] Phase 4: Execute Endpoint - Implement execute endpoint with SSE streaming
- [x] Phase 4: Execute Endpoint - Write tests for session management
- [x] Phase 4: Execute Endpoint - Implement session tracking and cancellation
- [x] Phase 4: Execute Endpoint - Integration tests for full execute flow
- [x] Phase 5: Production Readiness - Documentation and README

### Triage
- [ ] Docker Compose setup for local testing (optional for now)
- [ ] VS Code launch configuration (optional for now)
- [ ] Enterprise features (Phase 4 - future scope)

### Won't Fix
- [ ] OAuth2/mTLS advanced authentication (out of scope for MVP)
- [ ] Multi-tenancy support (future feature)
- [ ] HA deployment patterns (deployment concern, not implementation)

---

## In Progress
- [ ] Refactor storage layer to use dependency injection for base directory (removes HOME env var dependency)
  - [ ] Update storage.rs path functions to accept optional base_dir parameter
  - [ ] Update auth.rs AuthService methods to accept base_dir parameter
  - [ ] Update manager.rs UserManager methods to accept base_dir parameter
  - [ ] Update all tests to pass temp_dir instead of modifying HOME env var
  - [ ] Remove serial_test dependency from Cargo.toml
  - [ ] Verify tests pass with --test-threads=8

## Planned

- [ ] Phase 2: Browser Profile Management (v1.6) - Not started

### Phase 1: User Management (v1.5) - ✅ 100% COMPLETE (Backend + Frontend)
**Backend (✅ COMPLETE - All 31 tests passing, cargo xlint passing):**
- [x] Add crypto dependencies (argon2, aes-gcm, rand, zeroize, uuid, directories, tempfile)
- [x] Implement crypto module (Argon2id key derivation, AES-256-GCM encryption)
- [x] Implement storage module (filesystem operations, encrypted file I/O)
- [x] Implement user manager (CRUD operations, directory structure)
- [x] Implement authentication service (login, logout, session management, password verification)
- [x] Extend AppState with user_session field (Arc<Mutex<Option<UserSession>>>)
- [x] Create Tauri commands module (create_user, login_user, logout_user, list_users, get_current_user, update_user_profile, has_users)
- [x] Register all profile commands in lib.rs invoke_handler
- [x] Write unit tests for crypto (14 tests)
- [x] Write unit tests for storage (8 tests)
- [x] Write unit tests for manager (4 tests)
- [x] Write unit tests for auth (5 tests)
- [x] Fix all cargo xlint warnings and errors

**Frontend (✅ COMPLETE - All components implemented, all lints passing):**
- [x] Create TypeScript types (ProfileResult, UserConfig, PasswordValidation)
- [x] Create user state store in Svelte (reactive state management)
- [x] Create login screen UI (profile selector dropdown, password input)
- [x] Create user creation form UI (username, password with strength indicator)
- [x] Create profile switcher UI (user info, logout, switch profile)
- [x] Create user profile editor UI (markdown editor for user-profile.md)
- [x] Integrate all components into App.svelte (auth flow, first launch detection)
- [x] Add get_user_profile Tauri command for loading profile markdown
- [x] All code quality checks passing (cargo xlint, bun lint, bun check)

#### Phase 2: Browser Profile Management (v1.6) - 2 weeks
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

#### Phase 3: Command System (v1.7) - 3 weeks
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

#### Phase 4: Generative UI (v1.8) - 2 weeks
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

#### Phase 5: Command Refinement (v1.9) - 2 weeks
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

#### Documentation & Planning
- [x] Read all profiles documentation (PROFILES.md, PROFILES_QUESTIONS.md, USER_PROFILES_PRIVACY.md, PRD.md)
- [x] Create comprehensive implementation plan (.claude/profiles-implementation-plan.md ~25,000 words)
- [x] Review existing project structure

#### Phase 1: User Management - Backend Implementation (✅ 100% COMPLETE)

**Core Modules:**
- [x] Create profiles module structure (mod.rs, types.rs, crypto.rs, storage.rs, manager.rs, auth.rs)
- [x] Add all dependencies to Cargo.toml (argon2, aes-gcm, rand, zeroize, uuid, directories, pulldown-cmark, serde_yaml, tempfile)
- [x] Implement types module with all data structures (UserConfig, BrowserProfile, Command types, GenerativeUI, ExecutionContext)

**Security & Crypto:**
- [x] Implement crypto module with Argon2id password hashing (64MB, 3 iterations, 4 threads)
- [x] Implement AES-256-GCM authenticated encryption
- [x] Implement secure key derivation (~200-500ms)
- [x] Implement EncryptionKey with zeroize on drop
- [x] Implement constant-time password comparison

**Storage & File Operations:**
- [x] Implement storage module with filesystem operations
- [x] Implement encrypted file I/O (save/load user config, profile, commands)
- [x] Implement user directory structure (~/.robert/users/{username}/)
- [x] Implement salt management
- [x] Implement validation functions (username, profile name, command name)

**User Management:**
- [x] Implement user manager module with CRUD operations
- [x] Implement user creation with password validation (min 12 chars)
- [x] Implement username validation (alphanumeric, underscore, dash, max 32 chars)
- [x] Implement last login tracking
- [x] Implement default user profile generation

**Authentication:**
- [x] Implement authentication service (login, logout, session management)
- [x] Implement UserSession with thread-safe encryption key storage (Arc<Mutex<>>)
- [x] Implement create_and_login for new users
- [x] Implement password verification
- [x] Implement last login timestamp updates

**App Integration:**
- [x] Extend AppState with user_session field (Arc<Mutex<Option<UserSession>>>)
- [x] Create Tauri commands module (profiles.rs)
- [x] Implement create_user command
- [x] Implement login_user command
- [x] Implement logout_user command
- [x] Implement get_current_user command
- [x] Implement list_users command
- [x] Implement update_user_profile command
- [x] Implement has_users command (for first launch detection)
- [x] Register all 7 profile commands in lib.rs invoke_handler
- [x] Export auth module in profiles/mod.rs

**Testing & Quality:**
- [x] Write and pass all crypto module tests (14 tests)
- [x] Write and pass all storage module tests (8 tests)
- [x] Write and pass all manager module tests (4 tests)
- [x] Write and pass all auth module tests (5 tests)
- [x] Fix test parallelism issues (unique usernames, serial execution)
- [x] Fix all cargo xlint warnings and errors
- [x] Verify all 31 tests passing
- [x] Verify cargo xlint passing with no errors

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
