# TODO

## In Progress
- [ ] Phase 1: User Management (v1.5) - Set up crypto module and basic user infrastructure

## Planned

### Phase 1: User Management (v1.5) - 2 weeks
- [ ] Add crypto dependencies (argon2, aes-gcm, rand, zeroize, uuid, directories)
- [ ] Implement crypto module (Argon2id key derivation, AES-256-GCM encryption)
- [ ] Implement user manager (CRUD operations, directory structure)
- [ ] Implement authentication service (login, logout, password validation)
- [ ] Add Tauri commands for user operations
- [ ] Implement app state management with active user
- [ ] Create login screen UI (profile selector, password input)
- [ ] Create user creation form UI (username, password, strength indicator)
- [ ] Create user state store (Svelte)
- [ ] Implement profile switching UI
- [ ] Create user profile editor UI (markdown editor)
- [ ] Write unit tests for crypto (>90% coverage target)
- [ ] Write unit tests for user management
- [ ] Write integration tests for auth flows
- [ ] Migration script for existing users

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
- [x] Read implementation plan document
- [x] Read technical specification (PROFILES.md)
- [x] Read design decisions (PROFILES_QUESTIONS.md)
- [x] Read privacy requirements (USER_PROFILES_PRIVACY.md)
- [x] Read PRD profiles section
- [x] Review existing project structure

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
