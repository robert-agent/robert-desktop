# User Profiles and Multi-User Support
## Implementation Plan

---

## 1. Executive Summary

### Feature Overview

Robert will support **multiple user profiles** on shared computers, with isolated workspaces, browser profiles, commands, and encrypted storage. This feature transforms Robert from a single-user automation tool into a family-friendly application where each person has their own private, personalized automation environment.

Key capabilities:
- Password-protected user accounts with encrypted storage
- Ephemeral and named browser profiles per user
- User-specific commands that evolve through AI refinement
- Personalized AI agent context via user profile documents
- Multi-user support with single-user lock (only one active at a time)

### Strategic Value

**User Value:**
- **Privacy**: Password-encrypted directories prevent cross-user data access
- **Personalization**: AI learns each user's preferences, goals, and style
- **Organization**: Separate browser profiles for different workflows (work, shopping, research)
- **Family-friendly**: Multiple people can use one computer without mixing data
- **Command reusability**: Create once, refine over time, run with parameters

**Technical Value:**
- **Extensibility**: Foundation for future team features and command sharing
- **Data model**: Well-structured user, browser profile, and command storage
- **Security**: Industry-standard encryption (Argon2id, AES-256-GCM)
- **Scalability**: Architecture supports 10+ browser profiles and 100+ commands per user

### Implementation Timeline Estimate

**Total Duration**: 11 weeks (2.75 months)

- Phase 1: User Management (v1.5) - 2 weeks
- Phase 2: Browser Profile Management (v1.6) - 2 weeks
- Phase 3: Command System (v1.7) - 3 weeks
- Phase 4: Generative UI (v1.8) - 2 weeks
- Phase 5: Command Refinement (v1.9) - 2 weeks

**Team Size**: 1-2 engineers (full-stack: Rust backend + Svelte frontend)

**Dependencies**: Existing CDP execution engine, chat interface, AI inference system

---

## 2. Technical Architecture

### Component Breakdown

```
┌─────────────────────────────────────────────────────────────┐
│                     Robert Application                       │
│                                                              │
│  ┌────────────────┐              ┌─────────────────┐        │
│  │  User Manager  │◄────────────►│  Auth Service   │        │
│  │                │              │  (Argon2id)     │        │
│  └────────┬───────┘              └─────────────────┘        │
│           │                                                  │
│           │ manages                                          │
│           ▼                                                  │
│  ┌────────────────────────────────────────────┐             │
│  │         Active User Context                │             │
│  │  - user.json                               │             │
│  │  - user-profile.md                         │             │
│  │  - Commands                                │             │
│  │  - Browser Profiles                        │             │
│  └────────┬──────────────────┬────────────────┘             │
│           │                  │                               │
│           ▼                  ▼                               │
│  ┌────────────────┐  ┌─────────────────────┐               │
│  │ Command System │  │  Profile Manager    │               │
│  │  - Parse MD    │  │  - Ephemeral        │               │
│  │  - Generate UI │  │  - Named            │               │
│  │  - Versioning  │  │  - Default          │               │
│  └────────────────┘  └──────────┬──────────┘               │
│                                  │                           │
│                                  │ launches                  │
│                                  ▼                           │
│                    ┌─────────────────────────┐              │
│                    │  Browser Launcher       │              │
│                    │  - Chromiumoxide        │              │
│                    │  - CDP Protocol         │              │
│                    └───────────┬─────────────┘              │
└────────────────────────────────┼────────────────────────────┘
                                 │
                                 │ Chrome DevTools Protocol
                                 ▼
                    ┌──────────────────────────┐
                    │   Chromium Browser       │
                    │   (per browser profile)  │
                    └──────────────────────────┘
```

### Data Models and Schemas

#### UserConfig (user.json)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    /// Unique username (filesystem-safe)
    pub username: String,

    /// ISO 8601 timestamp of account creation
    pub created_at: DateTime<Utc>,

    /// ISO 8601 timestamp of last login
    pub last_login: DateTime<Utc>,

    /// Map of browser profile names to filesystem paths
    pub browser_profiles: HashMap<String, String>,

    /// Optional default browser profile name
    pub default_browser_profile: Option<String>,

    /// User preferences
    pub preferences: UserPreferences,

    /// Usage statistics
    #[serde(default)]
    pub stats: UserStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// UI theme preference
    pub theme: Theme,

    /// Default timeout for page operations (milliseconds)
    pub default_timeout_ms: u64,

    /// Inference mode (local vs cloud)
    pub inference_mode: InferenceMode,

    /// UI language (ISO 639-1 code)
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InferenceMode {
    Local,
    Cloud,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStats {
    /// Total commands executed
    pub total_commands_run: u64,

    /// Total browser sessions launched
    pub total_sessions: u64,

    /// Number of commands created by user
    pub commands_created: u64,
}
```

#### Command (command-{name}.md frontmatter)

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandFrontmatter {
    /// Unique command identifier (kebab-case)
    pub command_name: String,

    /// Human-readable description
    pub description: String,

    /// Browser profile to use (optional, defaults to ephemeral)
    pub browser_profile: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Semantic version
    pub version: String,

    /// Version history
    #[serde(default)]
    pub changelog: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParameter {
    /// Parameter name (snake_case)
    pub name: String,

    /// Parameter type
    pub param_type: ParameterType,

    /// User-facing label
    pub label: String,

    /// Optional placeholder text
    pub placeholder: Option<String>,

    /// Required field?
    pub required: bool,

    /// Default value (if any)
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    TextInput,
    ShortText,
    Dropdown { options: Vec<String> },
    Radio { options: Vec<String> },
    Checkbox,
    Slider { min: f64, max: f64, step: f64, unit: Option<String> },
    ColorPicker,
    DatePicker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerativeUI {
    /// Layout style
    pub layout: LayoutType,

    /// UI components
    pub components: Vec<UIComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutType {
    Vertical,
    TwoColumn,
    Grid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIComponent {
    /// Component type and properties
    #[serde(flatten)]
    pub component_type: ComponentType,

    /// Parameter name this component binds to
    pub name: String,

    /// User-facing label
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComponentType {
    TextInput { placeholder: Option<String>, required: bool },
    ShortText { placeholder: Option<String>, required: bool, max_length: Option<usize> },
    Dropdown { options: Vec<String>, required: bool },
    Radio { options: Vec<String>, required: bool },
    Checkbox { default: bool },
    Slider { min: f64, max: f64, step: f64, default: f64, unit: Option<String> },
    ColorPicker,
    DatePicker { min: Option<String>, max: Option<String>, required: bool },
}
```

### File System Structure

```
~/.robert/
├── app-config.json                          # Global app settings
├── users/
│   ├── alice/                               # User directory (encrypted)
│   │   ├── .salt                            # Argon2id salt (16 bytes)
│   │   ├── user.json.enc                    # Encrypted user config
│   │   ├── user-profile.md.enc              # Encrypted AI context
│   │   ├── browser-profiles/
│   │   │   ├── default/                     # Default persistent profile
│   │   │   │   ├── Default/                 # Chromium profile data
│   │   │   │   │   ├── Cookies
│   │   │   │   │   ├── History
│   │   │   │   │   ├── Preferences
│   │   │   │   │   └── ...
│   │   │   │   ├── Cache/
│   │   │   │   └── ...
│   │   │   ├── writing/                     # Named profile: "writing"
│   │   │   │   └── ... (Chromium data)
│   │   │   └── product-search/              # Named profile: "product-search"
│   │   │       └── ... (Chromium data)
│   │   └── commands/
│   │       ├── clothing-search.md.enc       # Encrypted command
│   │       ├── research-topic.md.enc
│   │       └── check-prices.md.enc
│   └── bob/                                 # Another user (encrypted separately)
│       ├── .salt
│       ├── user.json.enc
│       ├── user-profile.md.enc
│       ├── browser-profiles/
│       │   └── default/
│       └── commands/
│           └── daily-news.md.enc
└── .tmp/                                    # Temporary ephemeral profiles
    └── ephemeral-{uuid}/                    # Deleted after session
        └── ... (Chromium data)
```

**Storage Locations:**
- **macOS**: `~/.robert/` or `~/Library/Application Support/robert/`
- **Encryption**: User-specific files encrypted with password-derived key
- **Browser profiles**: NOT encrypted (Chromium handles its own encryption)
- **Ephemeral profiles**: Temporary, deleted after session ends

### Security/Encryption Approach

#### Password Derivation (Argon2id)

```rust
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, Algorithm, Version, Params,
};
use rand::rngs::OsRng;

/// Argon2id parameters for key derivation
const ARGON2_MEM_SIZE_KB: u32 = 65536; // 64 MB
const ARGON2_ITERATIONS: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;
const KEY_LENGTH: usize = 32; // 256 bits

/// Generate salt and derive encryption key from password
pub fn derive_key(password: &str, salt: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>)> {
    let salt = match salt {
        Some(s) => SaltString::encode_b64(s).unwrap(),
        None => SaltString::generate(&mut OsRng),
    };

    let params = Params::new(
        ARGON2_MEM_SIZE_KB,
        ARGON2_ITERATIONS,
        ARGON2_PARALLELISM,
        Some(KEY_LENGTH),
    )?;

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .hash
        .unwrap();

    Ok((password_hash.as_bytes().to_vec(), salt.as_bytes().to_vec()))
}
```

#### File Encryption (AES-256-GCM)

```rust
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;

/// Encrypt file content with AES-256-GCM
pub fn encrypt_file(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)?;

    // Generate random nonce (96 bits)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt plaintext
    let ciphertext = cipher.encrypt(nonce, plaintext)?;

    // Return: nonce || ciphertext || tag
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt file content with AES-256-GCM
pub fn decrypt_file(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    if encrypted.len() < 12 {
        return Err("Invalid ciphertext: too short".into());
    }

    let cipher = Aes256Gcm::new_from_slice(key)?;

    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&encrypted[..12]);
    let ciphertext = &encrypted[12..];

    // Decrypt
    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    Ok(plaintext)
}
```

#### Security Guarantees and Limitations

**Threats Mitigated:**
- Unauthorized access to user files (password required)
- Offline password attacks (Argon2id makes brute force expensive)
- Cross-user data leakage (encrypted directories)

**Threats NOT Mitigated:**
- File system permissions (not enforced)
- Memory dumps (decrypted data in RAM)
- Malware on host system
- Physical access with debugger
- Lost password (no recovery mechanism)

**Best Practices:**
- Enforce minimum 12-character passwords
- Show password strength indicator during creation
- No password recovery (lost password = lost data)
- Warn users to back up important commands
- Use macOS Keychain for password storage (future enhancement)

---

## 3. Implementation Phases

### Phase 1: User Management (v1.5)

**Goal:** Basic multi-user support with encryption

**Duration:** 2 weeks

#### Deliverables

1. User creation and authentication system
2. Password-based encryption (Argon2id + AES-256-GCM)
3. Profile selector UI
4. User directory structure
5. Auto-create default user on first launch
6. user.json and user-profile.md files
7. Profile switching

#### Rust Backend Tasks

**Module Structure:**
```
src-tauri/src/
├── user/
│   ├── mod.rs              # Module exports
│   ├── auth.rs             # Authentication logic
│   ├── crypto.rs           # Encryption/decryption
│   ├── manager.rs          # User CRUD operations
│   └── storage.rs          # Filesystem operations
└── commands/
    └── user.rs             # Tauri commands for frontend
```

**Tasks:**

1. **Crypto Module** (2 days)
   - Implement Argon2id key derivation
   - Implement AES-256-GCM encryption/decryption
   - Salt generation and storage
   - Key management in memory
   - Unit tests for crypto functions (>90% coverage)

2. **User Manager** (2 days)
   - Create user directory structure
   - Save/load user.json (encrypted)
   - Generate default user-profile.md
   - List available users
   - Delete user (with confirmation)
   - Unit tests for user operations

3. **Authentication Service** (1 day)
   - Validate password on login
   - Decrypt user directory
   - Load user config into app state
   - Logout (clear sensitive data from memory)
   - Unit tests for auth flows

4. **Tauri Commands** (1 day)
   - `create_user(username, password)` → Result<(), String>
   - `login_user(username, password)` → Result<UserConfig, String>
   - `logout_user()` → Result<(), String>
   - `list_users()` → Result<Vec<String>, String>
   - `get_current_user()` → Result<UserConfig, String>
   - `update_user_profile(content)` → Result<(), String>
   - Integration tests for commands

5. **App State Management** (1 day)
   - Global AppState with active_user: Option<UserConfig>
   - Encryption key storage in AppState (cleared on logout)
   - Session lock (only one user active at a time)

#### Frontend Tasks

**Component Structure:**
```
src/lib/
├── components/
│   ├── auth/
│   │   ├── LoginScreen.svelte
│   │   ├── CreateUserForm.svelte
│   │   └── ProfileSelector.svelte
│   └── user/
│       ├── UserProfileEditor.svelte
│       └── UserSettings.svelte
└── stores/
    └── user.ts              # User state management
```

**Tasks:**

1. **Login Screen UI** (1 day)
   - Profile selector dropdown (if users exist)
   - Password input field (with show/hide toggle)
   - "Add Profile" button
   - Error display (incorrect password)
   - Loading state during login

2. **Create User Form** (1 day)
   - Username input (validation: alphanumeric + underscore)
   - Password input (with confirmation)
   - Password strength indicator
   - Submit button
   - Error handling (username exists, weak password)

3. **User State Store** (0.5 days)
   - Svelte store for current user
   - Auto-load on app start
   - Persist across navigation
   - Clear on logout

4. **Profile Switching** (0.5 days)
   - "Switch Profile" button in settings
   - Confirmation modal ("Close active sessions?")
   - Logout current user and show profile selector

5. **User Profile Editor** (1 day)
   - Markdown editor for user-profile.md
   - Save button
   - Preview mode
   - Syntax highlighting

#### Testing Requirements

**Unit Tests:**
- Crypto: Key derivation, encryption, decryption (>90% coverage)
- User manager: CRUD operations, directory creation
- Auth: Password validation, login/logout flows

**Integration Tests:**
- End-to-end user creation flow
- Login with correct/incorrect password
- Profile switching
- First launch (auto-create default user)

**Manual Testing:**
- UI responsiveness
- Password strength indicator accuracy
- Error message clarity
- Profile selector usability

#### Time Estimate

- Backend: 7 days (1 engineer)
- Frontend: 4 days (1 engineer)
- Testing: 3 days (parallelizable)
- **Total: 2 weeks**

---

### Phase 2: Browser Profile Management (v1.6)

**Goal:** Support ephemeral and named browser profiles

**Duration:** 2 weeks

#### Deliverables

1. Ephemeral profile creation and cleanup
2. Named profile creation UI
3. Default profile configuration
4. Browser profile validation
5. Profile selection priority logic
6. Multiple simultaneous sessions

#### Rust Backend Tasks

**Module Structure:**
```
src-tauri/src/
├── browser/
│   ├── mod.rs              # Module exports
│   ├── profile.rs          # Browser profile types
│   ├── launcher.rs         # Browser launch logic
│   ├── session.rs          # Session management
│   └── cleanup.rs          # Ephemeral cleanup
└── commands/
    └── browser.rs          # Tauri commands for browser
```

**Tasks:**

1. **Browser Profile Types** (1 day)
   ```rust
   #[derive(Debug, Clone)]
   pub enum BrowserProfile {
       Ephemeral { temp_path: PathBuf },
       Named { name: String, path: PathBuf },
   }

   impl BrowserProfile {
       pub fn path(&self) -> &Path;
       pub fn is_ephemeral(&self) -> bool;
       pub fn display_name(&self) -> String;
   }
   ```
   - Unit tests for profile types

2. **Ephemeral Profile Lifecycle** (2 days)
   - Create temp directory: `~/.robert/.tmp/ephemeral-{uuid}/`
   - Launch Chrome with temp user-data-dir
   - Track active ephemeral sessions
   - Delete temp directory on session close
   - Cleanup orphaned ephemeral profiles on app start
   - Unit tests for cleanup logic

3. **Named Profile Management** (2 days)
   - Create named profile directory
   - Update user.json with profile mapping
   - Validate profile exists before launch
   - List available profiles for user
   - Delete named profile (with data)
   - Unit tests for profile CRUD

4. **Profile Selection Priority** (1 day)
   ```rust
   fn resolve_browser_profile(
       command: Option<&Command>,
       user_selection: Option<&str>,
       user_config: &UserConfig,
   ) -> BrowserProfile {
       // 1. Command specifies profile
       // 2. User manually selects profile
       // 3. User has default profile set
       // 4. Fallback: ephemeral
   }
   ```
   - Unit tests for priority logic

5. **Session Manager** (2 days)
   ```rust
   pub struct SessionManager {
       active_sessions: HashMap<SessionId, BrowserSession>,
   }

   impl SessionManager {
       pub fn launch_session(&mut self, profile: BrowserProfile) -> Result<SessionId>;
       pub fn close_session(&mut self, id: &SessionId) -> Result<()>;
       pub fn close_all_sessions(&mut self) -> Result<()>;
       pub fn list_active_sessions(&self) -> Vec<SessionInfo>;
   }
   ```
   - Track multiple simultaneous sessions
   - Cleanup on session close
   - Integration tests

6. **Tauri Commands** (1 day)
   - `create_browser_profile(name)` → Result<(), String>
   - `list_browser_profiles()` → Result<Vec<BrowserProfileInfo>, String>
   - `set_default_browser_profile(name)` → Result<(), String>
   - `delete_browser_profile(name)` → Result<(), String>
   - `launch_browser_session(profile_name?)` → Result<SessionId, String>
   - `close_browser_session(session_id)` → Result<(), String>
   - Integration tests

#### Frontend Tasks

**Component Structure:**
```
src/lib/components/
├── browser/
│   ├── BrowserProfileManager.svelte
│   ├── CreateProfileModal.svelte
│   ├── ProfileSelector.svelte
│   └── ActiveSessions.svelte
└── settings/
    └── BrowserSettings.svelte
```

**Tasks:**

1. **Browser Profile Manager UI** (2 days)
   - List named profiles (grid/list view)
   - "New Browser Profile" button
   - Default profile indicator
   - "Set as Default" button
   - "Delete" button (with confirmation)
   - Profile usage stats (commands using it)

2. **Create Profile Modal** (0.5 days)
   - Name input (validation: alphanumeric + dash)
   - Submit button
   - Error handling (name exists)

3. **Profile Selector Dropdown** (1 day)
   - Dropdown in chat interface
   - Options: Ephemeral (default), Default, Named profiles
   - Selection persists for session
   - Clear selection button

4. **Active Sessions Panel** (1 day)
   - List active browser sessions
   - Show profile name and start time
   - "Close Session" button
   - "Close All" button

5. **Browser Settings** (0.5 days)
   - Default profile selector
   - Cleanup orphaned ephemeral profiles button
   - Browser profile storage location display

#### Testing Requirements

**Unit Tests:**
- Ephemeral profile creation and cleanup
- Named profile CRUD operations
- Profile selection priority logic
- Session manager lifecycle

**Integration Tests:**
- Launch browser with ephemeral profile
- Launch browser with named profile
- Multiple simultaneous sessions
- Cleanup on session close

**Manual Testing:**
- UI flows for creating profiles
- Default profile behavior
- Session management panel
- Error handling (missing profile)

#### Time Estimate

- Backend: 9 days
- Frontend: 5 days
- Testing: 2 days (parallelizable)
- **Total: 2 weeks**

---

### Phase 3: Command System (v1.7)

**Goal:** User-created commands with parameters

**Duration:** 3 weeks

#### Deliverables

1. Command markdown parser
2. Command creation workflow (AI-assisted)
3. Command dropdown UI
4. Parameter definition schema
5. Command storage and versioning
6. Command execution flow

#### Rust Backend Tasks

**Module Structure:**
```
src-tauri/src/
├── command/
│   ├── mod.rs              # Module exports
│   ├── parser.rs           # Markdown parser
│   ├── schema.rs           # Command data structures
│   ├── manager.rs          # Command CRUD
│   ├── executor.rs         # Command execution
│   └── versioning.rs       # Version management
└── commands/
    └── command.rs          # Tauri commands
```

**Tasks:**

1. **Command Schema** (1 day)
   - Define CommandFrontmatter, CommandParameter, GenerativeUI structs
   - Serde serialization/deserialization
   - Validation logic (required fields, valid types)
   - Unit tests

2. **Markdown Parser** (3 days)
   - Parse YAML frontmatter (serde_yaml)
   - Extract markdown sections: Parameters, Rules, Checklist, Generative UI
   - Parse JSON blocks (generative UI)
   - Handle malformed markdown gracefully
   - Unit tests with example commands

3. **Command Manager** (2 days)
   - Save command to `~/.robert/users/{user}/commands/{name}.md.enc`
   - Load command by name
   - List all commands for user
   - Delete command
   - Validate command schema on save
   - Unit tests

4. **Command Versioning** (2 days)
   - Semantic version increment logic (patch/minor/major)
   - Changelog management
   - Version comparison
   - Rollback to previous version (future)
   - Unit tests

5. **Command Execution Logic** (3 days)
   - Resolve browser profile for command
   - Validate profile exists
   - Substitute parameter values into CDP script
   - Launch browser session
   - Execute CDP commands
   - Capture results
   - Integration with existing CDP executor

6. **Tauri Commands** (2 days)
   - `create_command(content)` → Result<(), String>
   - `list_commands()` → Result<Vec<CommandInfo>, String>
   - `get_command(name)` → Result<Command, String>
   - `update_command(name, content)` → Result<(), String>
   - `delete_command(name)` → Result<(), String>
   - `execute_command(name, params)` → Result<ExecutionResult, String>
   - Integration tests

#### Frontend Tasks

**Component Structure:**
```
src/lib/components/
├── command/
│   ├── CommandList.svelte
│   ├── CommandCreator.svelte
│   ├── CommandEditor.svelte
│   ├── CommandDropdown.svelte
│   └── CommandPreview.svelte
└── execution/
    └── CommandExecutor.svelte
```

**Tasks:**

1. **Command Dropdown** (1 day)
   - Dropdown in chat interface
   - List all user commands
   - Search/filter by name
   - Show command description on hover
   - Recent commands at top

2. **Command Creator** (3 days)
   - Chat-driven workflow
   - User describes task in natural language
   - AI generates command markdown
   - Preview generated command
   - "Request Changes" button (refine)
   - "Save Command" button

3. **Command Editor** (2 days)
   - Markdown editor with syntax highlighting
   - YAML frontmatter validation
   - JSON validation for generative UI
   - Save button
   - Version display (read-only)

4. **Command Preview** (1 day)
   - Render command as formatted UI
   - Show parameters, rules, checklist
   - Syntax highlighting for code blocks

5. **Command List** (1 day)
   - Grid/list view of commands
   - Show name, description, version, last used
   - "Edit" and "Delete" buttons
   - "Run Command" button

6. **Command Executor** (2 days)
   - Parameter input form (generated from command)
   - Browser profile selector (if not specified)
   - "Run" button
   - Execution progress display
   - Results panel

#### Testing Requirements

**Unit Tests:**
- Markdown parser with various input formats
- Command schema validation
- Version increment logic
- Parameter substitution

**Integration Tests:**
- End-to-end command creation flow
- Command execution with parameters
- Browser profile resolution
- Error handling (missing profile, invalid params)

**Manual Testing:**
- Chat-driven command creation
- Markdown editor usability
- Command dropdown functionality
- Execution flow with real commands

#### Time Estimate

- Backend: 13 days
- Frontend: 10 days
- Testing: 4 days (parallelizable)
- **Total: 3 weeks** (with overlap)

---

### Phase 4: Generative UI (v1.8)

**Goal:** Dynamic form generation from commands

**Duration:** 2 weeks

#### Deliverables

1. Generative UI JSON schema
2. UI component renderer (text, dropdown, slider, etc.)
3. Two-column and grid layouts
4. Form validation
5. Integration with command execution
6. Real-time parameter updates via chat

#### Rust Backend Tasks

**Module Structure:**
```
src-tauri/src/
├── generative_ui/
│   ├── mod.rs              # Module exports
│   ├── schema.rs           # UI component schema
│   ├── validator.rs        # Validate UI JSON
│   └── renderer.rs         # Backend rendering logic (if any)
└── commands/
    └── generative_ui.rs    # Tauri commands
```

**Tasks:**

1. **UI Component Schema** (1 day)
   - Define ComponentType enum (all types)
   - Validation logic for each component type
   - Unit tests

2. **JSON Schema Validation** (1 day)
   - Validate generative UI JSON structure
   - Check required fields for each component
   - Validate layout types
   - Error messages for invalid JSON
   - Unit tests

3. **Tauri Commands** (0.5 days)
   - `validate_generative_ui(json)` → Result<(), String>
   - Integration tests

#### Frontend Tasks

**Component Structure:**
```
src/lib/components/
└── generative_ui/
    ├── UIRenderer.svelte
    ├── components/
    │   ├── TextInput.svelte
    │   ├── ShortText.svelte
    │   ├── Dropdown.svelte
    │   ├── Radio.svelte
    │   ├── Checkbox.svelte
    │   ├── Slider.svelte
    │   ├── ColorPicker.svelte
    │   └── DatePicker.svelte
    ├── layouts/
    │   ├── VerticalLayout.svelte
    │   ├── TwoColumnLayout.svelte
    │   └── GridLayout.svelte
    └── FormValidator.svelte
```

**Tasks:**

1. **UI Component Library** (5 days)
   - Implement all 8 component types
   - Each component:
     - Props: name, label, options, validation rules
     - State: value, error, touched
     - Events: onChange, onBlur
     - Styling: Tailwind CSS
     - Accessibility: ARIA labels, keyboard navigation
   - Unit tests for each component

2. **Layout Renderer** (2 days)
   - Vertical layout (single column)
   - Two-column layout (side-by-side)
   - Grid layout (responsive)
   - Dynamic rendering based on JSON
   - Unit tests

3. **Form Validator** (1 day)
   - Required field validation
   - Type validation (number, date, etc.)
   - Custom validation rules
   - Error display
   - "Submit" button state (disabled if invalid)

4. **UIRenderer Component** (2 days)
   - Parse generative UI JSON
   - Instantiate components dynamically
   - Bind component values to parameter names
   - Collect form data on submit
   - Integration with command executor

5. **Chat Integration** (2 days)
   - Update parameter values via chat
   - Example: User says "change budget to $200"
   - AI updates form field in real-time
   - Bi-directional sync (chat ↔ form)

#### Testing Requirements

**Unit Tests:**
- Each UI component (props, state, events)
- Layout rendering logic
- Form validation rules
- JSON parser

**Integration Tests:**
- Render generative UI from command JSON
- Form submission with valid/invalid data
- Chat-driven parameter updates

**Manual Testing:**
- UI responsiveness and styling
- Accessibility (keyboard navigation, screen readers)
- Layout rendering on different screen sizes
- Real-time chat updates

#### Time Estimate

- Backend: 2.5 days
- Frontend: 12 days
- Testing: 3 days (parallelizable)
- **Total: 2 weeks**

---

### Phase 5: Command Refinement (v1.9)

**Goal:** AI-assisted command improvement

**Duration:** 2 weeks

#### Deliverables

1. Thumbs up/down feedback buttons
2. Failure context capture
3. AI refinement suggestions
4. Version increment logic
5. Changelog tracking
6. Rollback to previous versions (future)

#### Rust Backend Tasks

**Module Structure:**
```
src-tauri/src/
├── command/
│   ├── feedback.rs         # Feedback handling
│   ├── refinement.rs       # AI-assisted refinement
│   └── version_control.rs  # Version management
└── commands/
    └── refinement.rs       # Tauri commands
```

**Tasks:**

1. **Feedback Capture** (1 day)
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct CommandFeedback {
       pub command_name: String,
       pub thumbs_up: bool,
       pub user_comment: Option<String>,
       pub error_message: Option<String>,
       pub execution_context: ExecutionContext,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct ExecutionContext {
       pub parameters: HashMap<String, serde_json::Value>,
       pub browser_profile: String,
       pub execution_time_ms: u64,
       pub steps_completed: usize,
       pub steps_failed: usize,
   }
   ```
   - Save feedback to file
   - Unit tests

2. **Refinement Workflow** (3 days)
   - Load current command markdown
   - Build refinement prompt:
     - Current command
     - Failure context
     - User feedback
     - User profile context
   - Send to AI (Claude via existing inference engine)
   - Parse AI response (updated command markdown)
   - Validate updated command
   - Increment version
   - Add changelog entry
   - Integration with AI agent system

3. **Version Control** (2 days)
   - Save command history (v1.0.0, v1.1.0, etc.)
   - List available versions
   - Diff between versions (future)
   - Rollback to previous version (future)
   - Unit tests

4. **Tauri Commands** (1 day)
   - `submit_command_feedback(name, feedback)` → Result<(), String>
   - `refine_command(name)` → Result<String, String>
   - `get_command_versions(name)` → Result<Vec<VersionInfo>, String>
   - `rollback_command(name, version)` → Result<(), String>
   - Integration tests

#### Frontend Tasks

**Component Structure:**
```
src/lib/components/
└── command/
    ├── FeedbackButtons.svelte
    ├── RefinementModal.svelte
    ├── VersionHistory.svelte
    └── ChangelogDisplay.svelte
```

**Tasks:**

1. **Feedback Buttons** (1 day)
   - Thumbs up/down buttons after execution
   - Optional comment field
   - Submit feedback
   - Show confirmation

2. **Refinement Modal** (2 days)
   - Display failure context
   - AI-generated suggestions (streaming)
   - "Approve" and "Reject" buttons
   - "Request Changes" (iterate)
   - Show diff (current vs. proposed)

3. **Version History** (1 day)
   - List all versions
   - Show changelog for each
   - "Rollback" button (future)
   - Diff viewer (future)

4. **Changelog Display** (0.5 days)
   - Show version history in command preview
   - Formatted changelog

5. **Refinement Integration** (1 day)
   - Trigger refinement on thumbs down
   - Show progress indicator
   - Display refined command
   - Auto-save on approval

#### Testing Requirements

**Unit Tests:**
- Feedback capture and storage
- Version increment logic
- Changelog management

**Integration Tests:**
- End-to-end refinement workflow
- AI prompt generation and parsing
- Version history tracking

**Manual Testing:**
- Feedback button UX
- Refinement modal flow
- AI suggestion quality
- Version history display

#### Time Estimate

- Backend: 7 days
- Frontend: 5.5 days
- Testing: 3 days (parallelizable)
- **Total: 2 weeks**

---

## 4. Data Structures & APIs

### Rust Structs/Enums

See Section 2 (Technical Architecture) for detailed type definitions:
- `UserConfig`
- `UserPreferences`
- `UserStats`
- `CommandFrontmatter`
- `CommandParameter`
- `ParameterType`
- `GenerativeUI`
- `ComponentType`
- `BrowserProfile`
- `SessionManager`
- `CommandFeedback`

### Tauri Command Signatures

```rust
// User Management
#[tauri::command]
async fn create_user(username: String, password: String) -> Result<(), String>;

#[tauri::command]
async fn login_user(username: String, password: String) -> Result<UserConfig, String>;

#[tauri::command]
async fn logout_user(state: State<'_, AppState>) -> Result<(), String>;

#[tauri::command]
async fn list_users() -> Result<Vec<String>, String>;

#[tauri::command]
async fn get_current_user(state: State<'_, AppState>) -> Result<UserConfig, String>;

#[tauri::command]
async fn update_user_profile(content: String, state: State<'_, AppState>) -> Result<(), String>;

// Browser Profile Management
#[tauri::command]
async fn create_browser_profile(name: String, state: State<'_, AppState>) -> Result<(), String>;

#[tauri::command]
async fn list_browser_profiles(state: State<'_, AppState>) -> Result<Vec<BrowserProfileInfo>, String>;

#[tauri::command]
async fn set_default_browser_profile(name: String, state: State<'_, AppState>) -> Result<(), String>;

#[tauri::command]
async fn delete_browser_profile(name: String, state: State<'_, AppState>) -> Result<(), String>;

#[tauri::command]
async fn launch_browser_session(
    profile_name: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String>; // Returns SessionId

#[tauri::command]
async fn close_browser_session(session_id: String, state: State<'_, AppState>) -> Result<(), String>;

// Command System
#[tauri::command]
async fn create_command(content: String, state: State<'_, AppState>) -> Result<(), String>;

#[tauri::command]
async fn list_commands(state: State<'_, AppState>) -> Result<Vec<CommandInfo>, String>;

#[tauri::command]
async fn get_command(name: String, state: State<'_, AppState>) -> Result<Command, String>;

#[tauri::command]
async fn update_command(
    name: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<(), String>;

#[tauri::command]
async fn delete_command(name: String, state: State<'_, AppState>) -> Result<(), String>;

#[tauri::command]
async fn execute_command(
    name: String,
    parameters: HashMap<String, serde_json::Value>,
    state: State<'_, AppState>,
) -> Result<ExecutionResult, String>;

// Command Refinement
#[tauri::command]
async fn submit_command_feedback(
    name: String,
    feedback: CommandFeedback,
    state: State<'_, AppState>,
) -> Result<(), String>;

#[tauri::command]
async fn refine_command(name: String, state: State<'_, AppState>) -> Result<String, String>;

#[tauri::command]
async fn get_command_versions(
    name: String,
    state: State<'_, AppState>,
) -> Result<Vec<VersionInfo>, String>;

#[tauri::command]
async fn rollback_command(
    name: String,
    version: String,
    state: State<'_, AppState>,
) -> Result<(), String>;

// Generative UI
#[tauri::command]
async fn validate_generative_ui(json: String) -> Result<(), String>;
```

### TypeScript Interfaces

```typescript
// src/lib/types/user.ts

export interface UserConfig {
  username: string;
  created_at: string;
  last_login: string;
  browser_profiles: Record<string, string>;
  default_browser_profile?: string;
  preferences: UserPreferences;
  stats: UserStats;
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'system';
  default_timeout_ms: number;
  inference_mode: 'local' | 'cloud';
  language: string;
}

export interface UserStats {
  total_commands_run: number;
  total_sessions: number;
  commands_created: number;
}

// src/lib/types/browser.ts

export interface BrowserProfileInfo {
  name: string;
  path: string;
  is_default: boolean;
  created_at: string;
  last_used?: string;
}

// src/lib/types/command.ts

export interface CommandInfo {
  command_name: string;
  description: string;
  browser_profile?: string;
  created_at: string;
  updated_at: string;
  version: string;
}

export interface Command {
  frontmatter: CommandFrontmatter;
  parameters: CommandParameter[];
  rules: string[];
  checklist: string[];
  generative_ui?: GenerativeUI;
  cdp_script_template?: string;
}

export interface CommandFrontmatter {
  command_name: string;
  description: string;
  browser_profile?: string;
  created_at: string;
  updated_at: string;
  version: string;
  changelog: string[];
}

export interface CommandParameter {
  name: string;
  param_type: ParameterType;
  label: string;
  placeholder?: string;
  required: boolean;
  default?: any;
}

export type ParameterType =
  | 'text_input'
  | 'short_text'
  | { dropdown: { options: string[] } }
  | { radio: { options: string[] } }
  | 'checkbox'
  | { slider: { min: number; max: number; step: number; unit?: string } }
  | 'color_picker'
  | 'date_picker';

export interface GenerativeUI {
  layout: 'vertical' | 'two_column' | 'grid';
  components: UIComponent[];
}

export interface UIComponent {
  type: string;
  name: string;
  label: string;
  [key: string]: any; // Component-specific props
}

// src/lib/types/execution.ts

export interface ExecutionResult {
  status: 'success' | 'failure' | 'partial';
  duration_ms: number;
  steps_completed: number;
  steps_failed: number;
  outputs: string[];
  errors: string[];
}

export interface CommandFeedback {
  command_name: string;
  thumbs_up: boolean;
  user_comment?: string;
  error_message?: string;
  execution_context: ExecutionContext;
}

export interface ExecutionContext {
  parameters: Record<string, any>;
  browser_profile: string;
  execution_time_ms: number;
  steps_completed: number;
  steps_failed: number;
}

export interface VersionInfo {
  version: string;
  updated_at: string;
  changelog_entry: string;
}
```

### File Format Specifications

#### user.json

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["username", "created_at", "last_login", "browser_profiles", "preferences"],
  "properties": {
    "username": {
      "type": "string",
      "pattern": "^[a-zA-Z0-9_-]+$"
    },
    "created_at": {
      "type": "string",
      "format": "date-time"
    },
    "last_login": {
      "type": "string",
      "format": "date-time"
    },
    "browser_profiles": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      }
    },
    "default_browser_profile": {
      "type": "string"
    },
    "preferences": {
      "type": "object",
      "required": ["theme", "default_timeout_ms", "inference_mode", "language"],
      "properties": {
        "theme": {
          "enum": ["light", "dark", "system"]
        },
        "default_timeout_ms": {
          "type": "integer",
          "minimum": 1000,
          "maximum": 60000
        },
        "inference_mode": {
          "enum": ["local", "cloud"]
        },
        "language": {
          "type": "string",
          "pattern": "^[a-z]{2}$"
        }
      }
    },
    "stats": {
      "type": "object",
      "properties": {
        "total_commands_run": { "type": "integer", "minimum": 0 },
        "total_sessions": { "type": "integer", "minimum": 0 },
        "commands_created": { "type": "integer", "minimum": 0 }
      }
    }
  }
}
```

#### command-{name}.md

**YAML Frontmatter:**
```yaml
---
command_name: string              # Unique command identifier
description: string               # Human-readable description
browser_profile: string           # Optional: which profile to use
created_at: string                # ISO 8601 timestamp
updated_at: string                # ISO 8601 timestamp
version: string                   # Semantic version (1.2.0)
changelog:                        # Version history
  - "1.0.0: Initial creation"
  - "1.1.0: Added color parameter"
---
```

**Markdown Sections:**
- `## Parameters`: Parameter definitions (one per line)
- `## Rules`: Constraints and preferences (bullet list)
- `## Checklist`: Success criteria (checkbox list)
- `## Generative UI`: JSON code block
- `## CDP Script Template`: Optional AI-generated script

---

## 5. Security Implementation

### Password Hashing (Argon2id)

**Algorithm**: Argon2id (winner of Password Hashing Competition)

**Parameters:**
- **Memory**: 64 MB (65536 KB)
- **Iterations**: 3
- **Parallelism**: 4 threads
- **Output**: 32 bytes (256 bits)
- **Salt**: 16 bytes random (stored in `.salt` file)

**Why Argon2id?**
- Resistant to GPU/ASIC attacks (memory-hard)
- Balanced approach (time-memory trade-off)
- Recommended by OWASP and security experts
- Fallback to PBKDF2-SHA256 if Argon2id unavailable

**Implementation:**
```rust
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, Algorithm, Version, Params,
};

pub fn hash_password(password: &str) -> Result<(String, Vec<u8>)> {
    let salt = SaltString::generate(&mut OsRng);

    let params = Params::new(
        65536, // 64 MB
        3,     // 3 iterations
        4,     // 4 threads
        Some(32), // 32 byte output
    )?;

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    let hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok((hash.to_string(), salt.as_bytes().to_vec()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
```

### Encryption/Decryption Flow

**Algorithm**: AES-256-GCM (Authenticated Encryption with Associated Data)

**Flow:**

```
┌─────────────────┐
│  User Password  │
└────────┬────────┘
         │
         ▼
┌─────────────────────────┐
│  Argon2id Derivation    │
│  (64MB, 3 iter, 4 par)  │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  256-bit Encryption Key │
└────────┬────────────────┘
         │
         ├─────────────────────┐
         │                     │
         ▼                     ▼
┌──────────────────┐   ┌──────────────────┐
│  Encrypt Files   │   │  Decrypt Files   │
│  - user.json     │   │  - user.json.enc │
│  - user-profile  │   │  - commands/     │
│  - commands/     │   │                  │
└──────────────────┘   └──────────────────┘
```

**File Format:**
```
[12 bytes: Nonce] || [N bytes: Ciphertext] || [16 bytes: Auth Tag]
```

**Implementation:**
```rust
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

pub fn encrypt_file(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt (includes auth tag)
    let ciphertext = cipher.encrypt(nonce, plaintext)?;

    // Return: nonce || ciphertext || tag
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

pub fn decrypt_file(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)?;

    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&encrypted[..12]);
    let ciphertext = &encrypted[12..];

    // Decrypt (verifies auth tag)
    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    Ok(plaintext)
}
```

### Key Derivation Parameters

**Argon2id Configuration:**
```rust
const ARGON2_MEM_SIZE_KB: u32 = 65536;  // 64 MB
const ARGON2_ITERATIONS: u32 = 3;       // 3 passes
const ARGON2_PARALLELISM: u32 = 4;      // 4 threads
const KEY_LENGTH: usize = 32;           // 256 bits
```

**Benchmarks (on typical laptop):**
- Key derivation time: ~200-500ms
- Acceptable for login flow
- Expensive for brute-force attacks

**PBKDF2 Fallback:**
```rust
const PBKDF2_ITERATIONS: u32 = 100_000;
const PBKDF2_SALT_LEN: usize = 16;
const PBKDF2_KEY_LEN: usize = 32;
```

### Secure Storage Considerations

**What's Encrypted:**
- `user.json` → `user.json.enc`
- `user-profile.md` → `user-profile.md.enc`
- `commands/*.md` → `commands/*.md.enc`

**What's NOT Encrypted:**
- Browser profile data (Chromium handles own encryption)
- Ephemeral profiles (deleted after use anyway)
- App-wide config (`app-config.json`)

**Sensitive Data in Memory:**
- Encryption key stored in AppState (cleared on logout)
- Decrypted files loaded temporarily
- No sensitive data logged
- Memory zeroed on drop (use `zeroize` crate)

**Future Enhancements:**
- macOS Keychain integration for password storage
- Hardware-backed keys (Secure Enclave)
- Biometric authentication (Touch ID)

---

## 6. Testing Strategy

### Unit Test Requirements

**User Management:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_create_user_success() { /* ... */ }

    #[test]
    fn test_create_user_duplicate_username() { /* ... */ }

    #[test]
    fn test_login_correct_password() { /* ... */ }

    #[test]
    fn test_login_incorrect_password() { /* ... */ }

    #[test]
    fn test_encrypt_decrypt_roundtrip() { /* ... */ }

    #[test]
    fn test_argon2id_key_derivation() { /* ... */ }
}
```

**Browser Profiles:**
```rust
#[test]
fn test_create_ephemeral_profile() { /* ... */ }

#[test]
fn test_cleanup_ephemeral_on_close() { /* ... */ }

#[test]
fn test_create_named_profile() { /* ... */ }

#[test]
fn test_resolve_profile_priority() { /* ... */ }

#[test]
fn test_multiple_simultaneous_sessions() { /* ... */ }
```

**Command System:**
```rust
#[test]
fn test_parse_command_markdown() { /* ... */ }

#[test]
fn test_validate_command_schema() { /* ... */ }

#[test]
fn test_version_increment() { /* ... */ }

#[test]
fn test_parameter_substitution() { /* ... */ }
```

**Target Coverage**: >90% for core modules (crypto, user, command)

### Integration Test Scenarios

**Scenario 1: First Launch**
```rust
#[tokio::test]
async fn test_first_launch_creates_default_user() {
    // 1. App detects no users
    // 2. Auto-creates default user
    // 3. Prompts for password
    // 4. Encrypts user directory
    // 5. Logs in automatically
}
```

**Scenario 2: Multi-User Login**
```rust
#[tokio::test]
async fn test_multi_user_login_flow() {
    // 1. Create two users (alice, bob)
    // 2. Login as alice
    // 3. Create command as alice
    // 4. Logout alice
    // 5. Login as bob
    // 6. Verify alice's command not visible
}
```

**Scenario 3: Command Execution**
```rust
#[tokio::test]
async fn test_command_execution_with_browser_profile() {
    // 1. Create user
    // 2. Create named browser profile
    // 3. Create command specifying profile
    // 4. Execute command with parameters
    // 5. Verify browser launches with correct profile
    // 6. Verify command completes successfully
}
```

**Scenario 4: Command Refinement**
```rust
#[tokio::test]
async fn test_command_refinement_workflow() {
    // 1. Create and execute command
    // 2. Submit thumbs down feedback
    // 3. Trigger refinement
    // 4. Verify AI generates improved command
    // 5. Approve refinement
    // 6. Verify version incremented
    // 7. Verify changelog updated
}
```

### Security Test Cases

**Test 1: Password Strength Validation**
```rust
#[test]
fn test_reject_weak_passwords() {
    assert!(validate_password("123").is_err());       // Too short
    assert!(validate_password("password").is_err());  // Common word
    assert!(validate_password("12345678").is_err());  // No complexity
    assert!(validate_password("P@ssw0rd123!").is_ok()); // Strong
}
```

**Test 2: Encryption Security**
```rust
#[test]
fn test_decrypt_with_wrong_password_fails() {
    let (key1, _) = derive_key("correct_password", None).unwrap();
    let encrypted = encrypt_file(b"sensitive data", &key1).unwrap();

    let (key2, _) = derive_key("wrong_password", None).unwrap();
    assert!(decrypt_file(&encrypted, &key2).is_err());
}
```

**Test 3: Profile Isolation**
```rust
#[tokio::test]
async fn test_users_cannot_access_each_others_commands() {
    // 1. Create alice and bob
    // 2. Alice creates command
    // 3. Login as bob
    // 4. Attempt to list commands
    // 5. Verify alice's command not returned
}
```

**Test 4: Session Lock**
```rust
#[tokio::test]
async fn test_only_one_user_active_at_time() {
    // 1. Login as alice
    // 2. Attempt to login as bob (should fail)
    // 3. Logout alice
    // 4. Login as bob (should succeed)
}
```

### User Acceptance Criteria

**UAC-1: User Creation**
- User can create account with username + password
- Password strength indicator shows strength
- Weak passwords rejected
- Duplicate usernames rejected
- User directory created and encrypted

**UAC-2: Login/Logout**
- User can log in with correct password
- Incorrect password shows clear error
- User profile loaded after login
- Logout clears sensitive data from memory

**UAC-3: Browser Profiles**
- User can create named browser profiles
- Ephemeral profile used by default (no setup required)
- User can set default profile
- Browser launches with correct profile
- Multiple profiles can run simultaneously
- Ephemeral profiles cleaned up after close

**UAC-4: Command Creation**
- User can create command via chat interface
- AI generates command markdown
- User can preview before saving
- User can request changes (refine)
- Command saved to user directory (encrypted)

**UAC-5: Command Execution**
- User can select command from dropdown
- Generative UI form renders correctly
- User can fill parameters
- Command executes with correct browser profile
- Results displayed after completion
- User can provide feedback (thumbs up/down)

**UAC-6: Command Refinement**
- Thumbs down triggers refinement flow
- AI suggests improvements
- User can approve/reject suggestions
- Command version incremented on approval
- Changelog updated with changes

---

## 7. Migration & Backwards Compatibility

### How Existing Users Migrate to Profiles

**Scenario**: User upgrades from v1.4 (no profiles) to v1.5 (with profiles)

**Migration Strategy:**

1. **Detect First Launch with Profiles**
   ```rust
   fn needs_migration() -> bool {
       !Path::new("~/.robert/users/").exists()
   }
   ```

2. **Auto-Migrate Existing Data**
   ```rust
   async fn migrate_to_profiles() -> Result<()> {
       // 1. Create default user directory
       fs::create_dir_all("~/.robert/users/default/")?;

       // 2. Move existing data to default user
       if Path::new("~/.robert/commands/").exists() {
           fs::rename(
               "~/.robert/commands/",
               "~/.robert/users/default/commands/",
           )?;
       }

       // 3. Move browser data if exists
       if Path::new("~/.robert/browser-data/").exists() {
           fs::rename(
               "~/.robert/browser-data/",
               "~/.robert/users/default/browser-profiles/default/",
           )?;
       }

       // 4. Prompt for password (first time)
       // UI displays "Set Password" screen

       // 5. Encrypt user directory
       // (Wait for user to set password)

       Ok(())
   }
   ```

3. **Prompt User for Password**
   - Show migration screen: "Welcome to Multi-User Support!"
   - Explain: "Set a password to secure your data"
   - Password input field
   - "Continue" button
   - Encrypt existing data with password

4. **Create user.json**
   ```rust
   fn create_default_user_config() -> UserConfig {
       UserConfig {
           username: "default".to_string(),
           created_at: Utc::now(),
           last_login: Utc::now(),
           browser_profiles: HashMap::from([
               ("default".to_string(), "~/.robert/users/default/browser-profiles/default".to_string()),
           ]),
           default_browser_profile: Some("default".to_string()),
           preferences: UserPreferences {
               theme: Theme::System,
               default_timeout_ms: 5000,
               inference_mode: InferenceMode::Local,
               language: "en".to_string(),
           },
           stats: UserStats::default(),
       }
   }
   ```

5. **Auto-Login**
   - After migration, auto-login as "default" user
   - User continues using app seamlessly

### Data Migration Scripts

**Script 1: Migrate Commands**
```rust
async fn migrate_commands(from: &Path, to: &Path) -> Result<()> {
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let filename = entry.file_name();

        // Copy command file
        let src = entry.path();
        let dst = to.join(&filename);
        fs::copy(&src, &dst)?;

        // Encrypt command file
        let content = fs::read(&dst)?;
        let key = get_encryption_key()?;
        let encrypted = encrypt_file(&content, &key)?;
        fs::write(dst.with_extension("md.enc"), encrypted)?;
        fs::remove_file(&dst)?;
    }

    Ok(())
}
```

**Script 2: Migrate Browser Data**
```rust
async fn migrate_browser_data(from: &Path, to: &Path) -> Result<()> {
    // Move entire Chromium profile directory
    fs::rename(from, to)?;

    // Update user.json with new path
    // (handled by migration flow)

    Ok(())
}
```

**Script 3: Update App Config**
```rust
async fn update_app_config() -> Result<()> {
    let config_path = Path::new("~/.robert/app-config.json");

    if config_path.exists() {
        let mut config: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(config_path)?
        )?;

        // Add migration flag
        config["migrated_to_profiles"] = serde_json::json!(true);
        config["migration_date"] = serde_json::json!(Utc::now().to_rfc3339());

        fs::write(config_path, serde_json::to_string_pretty(&config)?)?;
    }

    Ok(())
}
```

### Backwards Compatibility Considerations

**File Format Changes:**
- Commands: No breaking changes (add frontmatter fields, preserve markdown)
- Browser profiles: No changes (Chromium handles)
- App config: Additive (new fields, old fields preserved)

**API Compatibility:**
- Old Tauri commands remain functional
- New commands added (not breaking)
- Frontend components updated (graceful degradation)

**Rollback Strategy:**
- Keep backup of old data structure during migration
- Allow rollback to v1.4 within 7 days
- After 7 days, backup deleted (user prompted)

**Version Detection:**
```rust
fn detect_app_version() -> Version {
    let config = load_app_config();
    config.version.parse().unwrap_or(Version::new(1, 4, 0))
}

fn can_rollback() -> bool {
    let migration_date = get_migration_date()?;
    let days_since_migration = (Utc::now() - migration_date).num_days();
    days_since_migration < 7
}
```

---

## 8. Open Questions & Risks

### Technical Challenges

**Challenge 1: Argon2id Performance**
- **Issue**: Key derivation (200-500ms) may feel slow on old hardware
- **Mitigation**: Show progress indicator, allow tuning parameters in advanced settings
- **Decision Needed**: Should we reduce iterations for faster machines?

**Challenge 2: Browser Profile Size**
- **Issue**: Chromium profiles can grow to 100s of MB
- **Mitigation**: Warn user when profile exceeds threshold, offer cleanup tools
- **Decision Needed**: Should we auto-clean cache periodically?

**Challenge 3: Concurrent Session Management**
- **Issue**: Multiple browser instances consume significant memory
- **Mitigation**: Warn user when >5 sessions active, offer bulk close
- **Decision Needed**: Hard limit on concurrent sessions?

**Challenge 4: Command Markdown Parsing**
- **Issue**: Malformed markdown can break parsing
- **Mitigation**: Strict validation on save, fallback to plain text display
- **Decision Needed**: Should we auto-fix common markdown errors?

### UX Decisions Needed

**Decision 1: Default User Auto-Creation**
- **Question**: Should first launch require manual user creation or auto-create "default"?
- **Recommendation**: Auto-create "default" for single-user scenario (reduces friction)

**Decision 2: Password Recovery**
- **Question**: Should we offer password recovery (security questions, email)?
- **Recommendation**: NO recovery (security best practice), warn users clearly

**Decision 3: Command Sharing**
- **Question**: Should v1.0 support command sharing between users?
- **Recommendation**: NO (defer to v2.0 as "command templates")

**Decision 4: Browser Profile Import**
- **Question**: Should users import existing Chrome profiles?
- **Recommendation**: NO (complexity + security concerns), users can log in manually

**Decision 5: Generative UI Customization**
- **Question**: Should users edit generative UI JSON directly?
- **Recommendation**: Advanced mode only (most users use AI-generated UI)

### Performance Considerations

**Concern 1: Encryption Overhead**
- **Impact**: Encrypt/decrypt on every command load
- **Measurement**: Benchmark with 100+ commands
- **Target**: <100ms for file operations
- **Mitigation**: Cache decrypted commands in memory (cleared on logout)

**Concern 2: Browser Launch Time**
- **Impact**: Launching Chrome takes 2-5 seconds
- **Measurement**: Profile launch time with large profiles
- **Target**: <5 seconds for any profile
- **Mitigation**: Pre-warm browser instances (future optimization)

**Concern 3: AI Inference Latency**
- **Impact**: Command refinement requires AI call (1-3 seconds)
- **Measurement**: Local vs. cloud inference time
- **Target**: <3 seconds for local, <5 seconds for cloud
- **Mitigation**: Show progress indicator, allow cancellation

**Concern 4: File System I/O**
- **Impact**: Reading/writing encrypted files on every operation
- **Measurement**: Monitor I/O with 100+ commands and 10+ profiles
- **Target**: <500ms for directory listing
- **Mitigation**: Implement file system cache (invalidate on change)

### Security Concerns

**Concern 1: Password in Memory**
- **Issue**: Encryption key stored in AppState during session
- **Mitigation**: Zero memory on logout, use `zeroize` crate
- **Risk Level**: Medium (requires process memory dump)

**Concern 2: Browser Profile Unencrypted**
- **Issue**: Chromium profiles not encrypted by Robert
- **Mitigation**: Rely on Chromium's own encryption, warn users in docs
- **Risk Level**: Low (Chromium handles security)

**Concern 3: File System Permissions**
- **Issue**: No filesystem-level protection (only encryption)
- **Mitigation**: Recommend macOS FileVault, warn users in docs
- **Risk Level**: Medium (physical access + debugger)

**Concern 4: Side-Channel Attacks**
- **Issue**: Timing attacks on password verification
- **Mitigation**: Argon2id is timing-attack resistant
- **Risk Level**: Low (academic concern)

**Concern 5: Backup and Export**
- **Issue**: Users may export encrypted files without encryption key
- **Mitigation**: Warn during export, provide "Export with Decryption" option
- **Risk Level**: Medium (user error)

---

## 9. Dependencies

### New Rust Crates

```toml
[dependencies]
# Existing dependencies
tauri = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
anyhow = "1.0"

# NEW: Cryptography
argon2 = "0.5"
aes-gcm = "0.10"
rand = "0.8"
zeroize = "1.6"

# NEW: Markdown parsing
pulldown-cmark = "0.9"
serde_yaml = "0.9"

# NEW: UUID generation
uuid = { version = "1.0", features = ["v4", "serde"] }

# NEW: Path manipulation
directories = "5.0"
```

**Crate Descriptions:**

- **argon2**: Password hashing with Argon2id (PHC winner)
- **aes-gcm**: Authenticated encryption (AES-256-GCM)
- **rand**: Cryptographically secure random number generation
- **zeroize**: Securely zero memory on drop
- **pulldown-cmark**: Markdown parser (fast, CommonMark compliant)
- **serde_yaml**: YAML frontmatter parsing
- **uuid**: Generate unique identifiers for ephemeral profiles
- **directories**: Cross-platform path handling

### Frontend Libraries

```json
{
  "dependencies": {
    "svelte": "^4.0.0",
    "@sveltejs/kit": "^1.0.0",
    "typescript": "^5.0.0",
    "tailwindcss": "^3.0.0",
    "@tauri-apps/api": "^2.0.0",

    // NEW: UI components
    "svelte-markdown": "^0.4.0",
    "codemirror": "^6.0.0",
    "@codemirror/lang-markdown": "^6.0.0",
    "@codemirror/lang-yaml": "^6.0.0",

    // NEW: Form handling
    "svelte-forms-lib": "^2.0.0",
    "yup": "^1.0.0",

    // NEW: Date/time
    "date-fns": "^2.0.0",

    // NEW: Icons
    "@tabler/icons-svelte": "^2.0.0"
  }
}
```

**Library Descriptions:**

- **svelte-markdown**: Render markdown in Svelte
- **codemirror**: Code editor with syntax highlighting
- **svelte-forms-lib**: Form management for generative UI
- **yup**: Schema validation for forms
- **date-fns**: Date formatting and manipulation
- **@tabler/icons-svelte**: Icon library

### External Tools

**Development:**
- Rust 1.70+ (stable toolchain)
- Node.js 18+ and npm/yarn
- Xcode Command Line Tools (macOS builds)
- Apple Developer ID (code signing)

**Testing:**
- `cargo test` for Rust unit/integration tests
- `vitest` for frontend tests
- `playwright` for E2E tests (future)

**CI/CD:**
- GitHub Actions for automated builds
- Notarization via Apple's API (altool/notarytool)

---

## 10. Success Metrics

### How We'll Measure Successful Implementation

**Metric 1: Feature Completeness**
- All 5 phases delivered on schedule
- 100% of requirements implemented
- No critical bugs in production

**Metric 2: Test Coverage**
- >90% unit test coverage for core modules
- 100% integration test coverage for critical flows
- All security tests passing

**Metric 3: User Adoption**
- 70%+ of existing users migrate to v1.5
- No support tickets related to migration issues
- <1% rollback rate after migration

**Metric 4: User Satisfaction**
- 4.5+ star rating for profile features
- <5% of users report confusion with UI
- Positive feedback on command system (surveys)

### Performance Benchmarks

**Login Performance:**
- Password validation: <500ms (Argon2id)
- User directory decryption: <200ms
- Total login time: <1 second

**Browser Launch Performance:**
- Ephemeral profile creation: <500ms
- Named profile launch: <5 seconds
- Session manager operations: <100ms

**Command Performance:**
- Command list load: <500ms (100 commands)
- Markdown parsing: <100ms per command
- Generative UI rendering: <200ms
- Command execution: (depends on CDP operations)

**File System Performance:**
- Encrypt/decrypt single file: <50ms
- List directory (encrypted): <200ms
- Save command: <100ms

**Memory Footprint:**
- Base app: <100MB
- With 10 commands loaded: <150MB
- With 3 active sessions: <500MB (excluding browser processes)

### User Experience Goals

**UX Goal 1: Zero-Friction Single-User Experience**
- First launch auto-creates default user
- Password setup takes <30 seconds
- No configuration required to start automating
- **Target**: 95% of single users never create additional profiles

**UX Goal 2: Intuitive Multi-User Experience**
- Profile selector immediately understandable
- Switching profiles takes <3 clicks
- No data loss during switching
- **Target**: 80% of multi-user households successfully set up profiles

**UX Goal 3: Command Creation Delight**
- AI-generated commands accurate 80%+ of the time
- Refinement workflow reduces errors by 50%
- Users create 5+ commands within first month
- **Target**: 4+ star rating for command creation experience

**UX Goal 4: Generative UI Excellence**
- Forms render instantly (<200ms)
- All 8 component types work flawlessly
- Layouts adapt to screen size
- **Target**: 90% of users prefer generative UI over chat-only

**UX Goal 5: Security Without Friction**
- Password setup clear and fast
- Encryption invisible to users
- No lost passwords (clear warnings)
- **Target**: <1% of users lose data due to forgotten passwords

---

## Conclusion

This implementation plan provides a comprehensive roadmap for building the User Profiles and Multi-User Support feature in Robert. The phased approach ensures steady progress while maintaining code quality and user experience.

**Key Success Factors:**
1. Strong encryption foundation (Argon2id + AES-256-GCM)
2. Intuitive UX for both single-user and multi-user scenarios
3. Robust command system with AI-assisted refinement
4. Comprehensive testing at every phase
5. Clear migration path for existing users

**Next Steps:**
1. Review and approve this plan
2. Set up development environment
3. Begin Phase 1: User Management
4. Iterate based on feedback and testing

**Contact:**
- Technical questions: [engineering team]
- UX feedback: [product team]
- Security review: [security team]

---

**Document Version:** 1.0
**Last Updated:** 2025-10-17
**Status:** Ready for Implementation
**Target Release:** v1.5-v1.9 (11 weeks)
