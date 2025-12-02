# User Profiles and Multi-User Support
## Technical Specification

## Overview

Robert supports **multiple user profiles** on shared computers, with isolated workspaces, browser profiles, commands, and encrypted storage. This document provides the complete technical specification for implementing the profiles system.

## Terminology

| Term | Definition |
|------|------------|
| **User Profile** | An individual person's account in Robert with isolated data and preferences |
| **Browser Profile** | A Chromium user data directory with cookies, history, and state |
| **Ephemeral Profile** | Temporary browser profile with no persistent state (deleted after session) |
| **Named Profile** | Persistent browser profile with saved state (e.g., "shopping", "work") |
| **Default Profile** | User's preferred persistent browser profile for general use |
| **Command** | Reusable automation workflow defined in Markdown |
| **Session** | Lifetime of a browser profile from launch to close |
| **Generative UI** | Dynamically generated form interface based on command parameters |

## Architecture

### High-Level Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Robert Application                       │
│                                                              │
│  ┌────────────────┐              ┌─────────────────┐        │
│  │  User Manager  │◄────────────►│  Auth Service   │        │
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
│  └────────────────┘  └──────────┬──────────┘               │
│                                  │                           │
│                                  │ launches                  │
│                                  ▼                           │
│                    ┌─────────────────────────┐              │
│                    │  Browser Launcher       │              │
│                    │  - Ephemeral            │              │
│                    │  - Default              │              │
│                    │  - Named                │              │
│                    └───────────┬─────────────┘              │
└────────────────────────────────┼──────────────────────────
│
                                  │ Chrome DevTools Protocol
                                  ▼
                    ┌──────────────────────────┐
                    │   Chromium Browser       │
                    │   (per browser profile)  │
                    └──────────────────────────┘
```

### File System Structure

```
~/.robert/
├── app-config.json                          # Global app settings
├── users/
│   ├── alice/                               # User directory (encrypted)
│   │   ├── user.json                        # User metadata and config
│   │   ├── user-profile.md                  # AI context document
│   │   ├── browser-profiles/
│   │   │   ├── default/                     # Default persistent profile
│   │   │   │   ├── Default/                 # Chromium profile data
│   │   │   │   ├── Cache/
│   │   │   │   ├── Cookies
│   │   │   │   └── ...
│   │   │   ├── writing/                     # Named profile: "writing"
│   │   │   │   └── ... (Chromium data)
│   │   │   └── product-search/              # Named profile: "product-search"
│   │   │       └── ... (Chromium data)
│   │   └── commands/
│   │       ├── clothing-search.md           # Command: clothing search
│   │       ├── research-topic.md            # Command: research topic
│   │       └── check-prices.md              # Command: price comparison
│   └── bob/                                 # Another user (encrypted separately)
│       ├── user.json
│       ├── user-profile.md
│       ├── browser-profiles/
│       │   └── default/
│       └── commands/
│           └── daily-news.md
└── .tmp/                                    # Temporary ephemeral profiles
    └── ephemeral-{uuid}/                    # Deleted after session
```

## Data Models

### user.json

Configuration and metadata for a user profile.

```typescript
interface UserConfig {
  // Identity
  username: string;                          // Unique username
  created_at: string;                        // ISO 8601 timestamp
  last_login: string;                        // ISO 8601 timestamp

  // Browser Profiles
  browser_profiles: {
    [profileName: string]: string;           // profile name -> path
  };
  default_browser_profile?: string;          // Optional default profile name

  // Preferences
  preferences: {
    theme: "light" | "dark" | "system";
    default_timeout_ms: number;
    inference_mode: "local" | "cloud";
    language: string;                        // ISO 639-1 code
  };

  // Statistics
  stats?: {
    total_commands_run: number;
    total_sessions: number;
    commands_created: number;
  };
}
```

**Example:**
```json
{
  "username": "alice",
  "created_at": "2025-10-17T10:00:00Z",
  "last_login": "2025-10-17T14:30:00Z",
  "browser_profiles": {
    "default": "~/.robert/users/alice/browser-profiles/default",
    "writing": "~/.robert/users/alice/browser-profiles/writing",
    "product-search": "~/.robert/users/alice/browser-profiles/product-search"
  },
  "default_browser_profile": "default",
  "preferences": {
    "theme": "dark",
    "default_timeout_ms": 5000,
    "inference_mode": "local",
    "language": "en"
  },
  "stats": {
    "total_commands_run": 42,
    "total_sessions": 15,
    "commands_created": 8
  }
}
```

### user-profile.md

Human-readable document describing user preferences, goals, and communication style. This file is included as context in all AI prompts.

**Structure:**
```markdown
# User Profile: {username}

## Preferences
- List of user preferences relevant to automation
- Examples: detail level, privacy concerns, technical comfort

## Goals
- What the user wants to accomplish with Robert
- Use cases and workflows they care about

## Language Style
- How the user prefers to communicate
- Tone, formality, emoji usage, etc.

## Additional Context
(Optional sections the user or AI can add over time)
```

**Example:**
```markdown
# User Profile: Alice

## Preferences
- Prefers detailed explanations over concise ones
- Values privacy and data minimization
- Tech-savvy, comfortable with technical terminology
- Dislikes Amazon due to ethical concerns

## Goals
- Automate repetitive research tasks for work
- Streamline online shopping workflows to save time
- Track competitor product launches in fashion industry
- Document web flows for design portfolio

## Language Style
- Professional and direct
- No emojis or casual language
- Prefers bullet points over paragraphs
- Appreciates technical accuracy

## Shopping Preferences
- Prioritize sustainable and ethical brands
- Free returns are important
- Willing to pay premium for quality
```

### command-{name}.md

Command definition file specifying automation workflow, parameters, and UI.

**What is a Command?**

A command is a **markdown template** that describes an automation task for an AI agent. The markdown provides:
- **Instructions** for the agent on what to do
- **Parameters** that users can customize
- **Rules and constraints** for execution
- **Success criteria** to verify completion

The markdown may optionally include a sample CDP JSON script, but this is **not required**. The agent can generate CDP commands dynamically based on the markdown description.

**YAML Frontmatter:**
```yaml
---
command_name: string              # Unique command identifier
description: string               # Human-readable description
browser_profile?: string          # Which profile to use (optional)
created_at: string                # ISO 8601 timestamp
updated_at: string                # ISO 8601 timestamp
version: string                   # Semantic version
---
```

**Markdown Body Sections:**

1. **Parameters**: Define user inputs
2. **Rules**: Constraints and preferences
3. **Checklist**: Success criteria
4. **Generative UI**: UI layout specification (JSON, optional)
5. **CDP Script Template**: Example CDP automation (optional - agent can generate dynamically)

**Example:**
```markdown
---
command_name: clothing-search
description: Search for clothing items across multiple retailers
browser_profile: product-search
created_at: 2025-10-17T10:00:00Z
updated_at: 2025-10-17T15:30:00Z
version: 1.2.0
changelog:
  - 1.0.0: Initial creation
  - 1.1.0: Added color preference parameter
  - 1.2.0: Fixed timeout issue with slow-loading product pages
---

# Clothing Search Command

## Parameters
- **outfit_type** (text, required): "What outfit are you looking for?"
- **size** (dropdown, required): ["XS", "S", "M", "L", "XL", "XXL"]
- **budget_max** (number, optional): Maximum price in USD
- **color_preference** (text, optional): Preferred color or pattern

## Rules
- Never shop on amazon.com
- Prioritize retailers with free returns
- Filter out fast fashion brands (H&M, Zara, Shein)
- Only show items with at least 4-star ratings

## Checklist
- [ ] Search at least 3 different retailers
- [ ] Capture product images and prices
- [ ] Extract shipping information
- [ ] Check return policy for each retailer
- [ ] Save results to markdown file
- [ ] Present top 5 options ranked by value

## Generative UI
```json
{
  "layout": "vertical",
  "components": [
    {
      "type": "text_input",
      "name": "outfit_type",
      "label": "Outfit Type",
      "placeholder": "e.g., winter jacket, cocktail dress, running shoes",
      "required": true
    },
    {
      "type": "dropdown",
      "name": "size",
      "label": "Size",
      "options": ["XS", "S", "M", "L", "XL", "XXL"],
      "required": true
    },
    {
      "type": "slider",
      "name": "budget_max",
      "label": "Maximum Budget",
      "min": 0,
      "max": 500,
      "step": 10,
      "default": 150,
      "unit": "$"
    },
    {
      "type": "color_picker",
      "name": "color_preference",
      "label": "Preferred Color (optional)"
    }
  ]
}
```

## CDP Script Template (Optional)

This section is **optional**. The agent can generate CDP commands dynamically based on the markdown instructions above. However, you can include a sample CDP script as a starting point or reference.

```json
{
  "name": "clothing-search",
  "cdp_commands": [
    {
      "method": "Page.navigate",
      "params": {"url": "https://retailer1.com"}
    },
    {
      "method": "Runtime.evaluate",
      "params": {
        "expression": "document.querySelector('input[name=search]').value = '{{outfit_type}}'"
      }
    }
  ]
}
```

**Note**: The agent can work with or without this section. If omitted, the agent will generate CDP commands based on the Parameters, Rules, and Checklist sections.

### Generative UI Specification

The `generative_ui` section defines dynamic form components rendered in the UI.

**Supported Component Types:**

| Type | Description | Properties |
|------|-------------|------------|
| `text_input` | Multi-line text input | `name`, `label`, `placeholder`, `required` |
| `short_text` | Single-line text input | `name`, `label`, `placeholder`, `required`, `maxLength` |
| `dropdown` | Select from options | `name`, `label`, `options`, `required` |
| `radio` | Single choice (2-4 options) | `name`, `label`, `options`, `required` |
| `checkbox` | Boolean toggle | `name`, `label`, `default` |
| `slider` | Numeric range | `name`, `label`, `min`, `max`, `step`, `default`, `unit` |
| `color_picker` | Color selection | `name`, `label` |
| `date_picker` | Date selection | `name`, `label`, `min`, `max`, `required` |

**Layout Options:**
- `vertical`: Single column
- `two_column`: Side-by-side panels
- `grid`: Responsive grid layout

**Example Component:**
```json
{
  "type": "slider",
  "name": "budget_max",
  "label": "Maximum Budget",
  "min": 0,
  "max": 1000,
  "step": 25,
  "default": 200,
  "unit": "$"
}
```

## User Authentication and Encryption

### Password-Based Encryption

User directories are encrypted using a password-derived key.

**Algorithm:** Argon2id (recommended) or PBKDF2-SHA256 (fallback)

**Encryption Flow:**
```
User Password
    │
    ├─► Argon2id(password, salt, iterations)
    │   └─► 256-bit Encryption Key
    │
    └─► Encrypt user directory with AES-256-GCM
         - Encrypt file contents
         - Encrypt file names (optional)
```

**Key Derivation Parameters (Argon2id):**
- Memory: 64 MB
- Iterations: 3
- Parallelism: 4 threads
- Output: 32 bytes (256 bits)
- Salt: 16 bytes random (stored in `~/.robert/users/{username}/.salt`)

**Fallback (PBKDF2-SHA256):**
- Iterations: 100,000
- Salt: 16 bytes random
- Output: 32 bytes (256 bits)

**Encrypted Files:**
- `user.json` → `user.json.enc`
- `user-profile.md` → `user-profile.md.enc`
- `commands/*.md` → `commands/*.md.enc`
- Browser profiles remain unencrypted (Chromium handles own encryption)

### Authentication Flow

#### First Launch (No Users)
```
1. App detects empty ~/.robert/users/
2. Display "Create Your Profile" screen
3. User enters username + password
4. Generate salt, derive key
5. Create directory: ~/.robert/users/{username}/
6. Create encrypted user.json and user-profile.md
7. Store salt in .salt file
8. Auto-login to new profile
```

#### Subsequent Launches (Users Exist)
```
1. App lists users from ~/.robert/users/
2. Display profile selector dropdown
3. User selects profile
4. Display password input
5. Load salt from .salt file
6. Derive key from password + salt
7. Attempt to decrypt user.json
   - Success: Login, load user context
   - Failure: Show "Incorrect password" error
```

#### Profile Switching
```
1. User clicks "Switch Profile" button
2. Save current user's state
3. Clear session context
4. Display profile selector
5. Repeat authentication flow
```

### Security Considerations

**Threats Mitigated:**
- ✅ Unauthorized access to user files (password required)
- ✅ Offline password attacks (Argon2id makes brute force expensive)
- ✅ Cross-user data leakage (encrypted directories)

**Threats NOT Mitigated:**
- ❌ File system permissions (not enforced)
- ❌ Memory dumps (decrypted data in RAM)
- ❌ Malware on host system
- ❌ Physical access with debugger

**Best Practices:**
- Use strong passwords (enforce minimum 12 characters)
- Show password strength indicator during creation
- No password recovery (lost password = lost data)
- Warn users to back up important commands

## Browser Profile Management

### Ephemeral Profiles (Default Behavior)

**Purpose:** Clean, temporary browser with no persistent state.

**Lifecycle:**
1. User starts session without specifying profile
2. App creates temp directory: `~/.robert/.tmp/ephemeral-{uuid}/`
3. Launch Chrome with `--user-data-dir=~/.robert/.tmp/ephemeral-{uuid}/`
4. Run commands, automation, etc.
5. User closes browser or ends session
6. App deletes `ephemeral-{uuid}/` directory
7. No trace remains

**Use Cases:**
- Testing automations
- One-off tasks
- Privacy-conscious workflows
- Avoiding cookie tracking

### Named Profiles (Persistent)

**Purpose:** Reusable browser sessions with saved state.

**Creation:**
```
1. User navigates to Settings > Browser Profiles
2. Clicks "New Browser Profile"
3. Enters profile name: "shopping"
4. App creates: ~/.robert/users/{user}/browser-profiles/shopping/
5. Launch Chrome with --user-data-dir pointing to shopping/
6. User can log into accounts, save bookmarks, etc.
7. State persists across sessions
```

**Profile Metadata:**
Stored in `user.json`:
```json
{
  "browser_profiles": {
    "shopping": "~/.robert/users/alice/browser-profiles/shopping",
    "work": "~/.robert/users/alice/browser-profiles/work"
  }
}
```

### Default Profile

**Purpose:** User's preferred persistent profile for general use.

**Setting Default:**
```
1. User navigates to Settings > Browser Profiles
2. Clicks "Set as Default" on a named profile
3. App updates user.json:
   {
     "default_browser_profile": "shopping"
   }
4. Future sessions use "shopping" profile unless overridden
```

**Behavior:**
- If user starts session without specifying profile:
  - Check if `default_browser_profile` is set
  - If yes: Use default
  - If no: Use ephemeral
- Commands can override by specifying `browser_profile` in frontmatter

### Profile Selection Priority

When launching a browser, the app follows this priority:

1. **Command specifies profile**: Use that profile
2. **User manually selects profile**: Use selected profile
3. **User has default profile set**: Use default
4. **No default set**: Use ephemeral

```rust
fn resolve_browser_profile(
    command: &Command,
    user_selection: Option<&str>,
    user_config: &UserConfig,
) -> BrowserProfileType {
    if let Some(profile) = command.browser_profile {
        // Command explicitly specifies profile
        return BrowserProfileType::Named(profile.to_string());
    }

    if let Some(profile) = user_selection {
        // User manually selected profile for this session
        return BrowserProfileType::Named(profile.to_string());
    }

    if let Some(default) = &user_config.default_browser_profile {
        // User has configured a default profile
        return BrowserProfileType::Named(default.clone());
    }

    // Fallback: ephemeral
    BrowserProfileType::Ephemeral
}
```

### Browser Profile Validation

Before launching a browser with a named profile, validate it exists:

```rust
fn validate_browser_profile(profile_name: &str, user_config: &UserConfig) -> Result<PathBuf> {
    let profile_path = user_config.browser_profiles
        .get(profile_name)
        .ok_or(ProfileError::NotFound(profile_name.to_string()))?;

    let path = PathBuf::from(profile_path);

    if !path.exists() {
        return Err(ProfileError::DirectoryMissing(path));
    }

    Ok(path)
}
```

**Error Handling:**
- If profile doesn't exist: Alert user, suggest "Refine Command"
- If profile path is invalid: Log error, fallback to ephemeral
- If Chromium can't launch with profile: Show diagnostic message

## Command System

### Command Creation Workflow

**Step 1: User Describes Task**
```
User (in chat): "I want to search for winter jackets under $200
                 on multiple websites, but not Amazon"
```

**Step 2: AI Generates Command**
AI agent creates initial command markdown:
```markdown
---
command_name: winter-jacket-search
description: Search for winter jackets under $200 across retailers
browser_profile: product-search
created_at: 2025-10-17T10:00:00Z
version: 1.0.0
---

# Winter Jacket Search

## Parameters
- max_price (number, required): Maximum price in USD (default: 200)
- size (dropdown, required): ["S", "M", "L", "XL"]

## Rules
- Exclude Amazon
- Only show jackets with 4+ star ratings

## Checklist
- [ ] Search at least 3 retailers
- [ ] Capture product images
- [ ] Save results

## Generative UI
(AI generates UI JSON)
```

**Step 3: User Reviews and Approves**
```
App displays generated command with preview
User can:
  - Accept (save as-is)
  - Request changes ("add color preference parameter")
  - Edit directly (advanced mode)
```

**Step 4: Save Command**
```
App saves to: ~/.robert/users/alice/commands/winter-jacket-search.md
Updates user.json stats: commands_created++
```

### Command Execution Workflow

**Step 1: User Invokes Command**
```
User clicks command dropdown
Selects "winter-jacket-search"
```

**Step 2: Render Generative UI**
```
App parses command markdown
Extracts generative_ui JSON
Renders form components in UI
```

**Step 3: User Fills Parameters**
```
User enters:
  - max_price: 150
  - size: "M"
```

**Step 4: Validate Browser Profile**
```
Command specifies: browser_profile: "product-search"
App checks if profile exists
  - Exists: Continue
  - Missing: Show error, suggest refine
```

**Step 5: Generate CDP Script**
```
AI agent receives:
  - Command template
  - User parameters (max_price=150, size="M")
  - User profile context (user-profile.md)
  - Current page state (if applicable)

AI generates CDP commands:
[
  {"method": "Page.navigate", "params": {"url": "https://retailer1.com"}},
  {"method": "Runtime.evaluate", "params": {"expression": "search('winter jacket')"}},
  ...
]
```

**Step 6: Execute Automation**
```
App launches browser with "product-search" profile
Executes CDP commands sequentially
Shows progress in UI
Captures results (screenshots, text)
```

**Step 7: Present Results**
```
App displays:
  - Captured product images
  - Prices and links
  - Shipping information

User provides feedback:
  - 👍 (success, no changes)
  - 👎 (failure, trigger refinement)
```

### Command Refinement

**Trigger Conditions:**
1. User clicks 👎 after command execution
2. Command execution fails (error thrown)
3. User says "that didn't work" or provides correction

**Refinement Workflow:**
```
1. Capture failure context:
   - Error messages
   - User feedback
   - Screenshots of unexpected results

2. Send to AI agent:
   - Current command markdown
   - Failure context
   - User profile

3. AI suggests improvements:
   "The command failed because the product-search profile doesn't exist.
    Suggestion: Change browser_profile to 'default' or create the profile."

4. User reviews suggestions:
   - Approve: Update command, increment version
   - Reject: Keep command as-is
   - Modify: Iteratively refine

5. Save updated command:
   command-name.md (version 1.1.0)
```

**Versioning:**
```yaml
---
version: 1.1.0
changelog:
  - 1.0.0: Initial creation
  - 1.1.0: Changed browser profile from 'product-search' to 'default'
---
```

## Session Management

### Session Definition

A **session** is the lifetime of a browser instance from launch to close.

**Key Properties:**
- One session = one Chromium process
- Session persists across multiple command executions
- Session ends when browser closes (manual or automatic)
- Named profile sessions save state; ephemeral sessions do not

**Example Timeline:**
```
10:00 AM - User launches "shopping" profile
10:01 AM - Runs "clothing-search" command (winter jacket)
10:05 AM - Runs "compare-prices" command
10:10 AM - Runs "clothing-search" again (shoes)
10:15 AM - Closes browser
10:15 AM - Session ends, state saved to shopping/ profile
```

### Multiple Simultaneous Sessions

**Capability:** User can have multiple browser profiles open at once.

**Example:**
```
User has two browser windows:
  1. "writing" profile - researching articles
  2. "shopping" profile - comparing prices

Both run simultaneously in separate Chromium processes
Commands can target either profile
```

**Implementation:**
```rust
struct SessionManager {
    active_sessions: HashMap<String, BrowserSession>,
}

impl SessionManager {
    fn launch_session(&mut self, profile: BrowserProfile) -> Result<SessionId> {
        let session_id = Uuid::new_v4().to_string();
        let browser = launch_chromium(profile)?;

        self.active_sessions.insert(session_id.clone(), BrowserSession {
            id: session_id.clone(),
            profile,
            browser,
            started_at: Utc::now(),
        });

        Ok(session_id)
    }

    fn close_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.remove(session_id) {
            session.browser.close()?;

            // Clean up ephemeral profile
            if session.profile.is_ephemeral() {
                fs::remove_dir_all(session.profile.path())?;
            }
        }

        Ok(())
    }
}
```

### Single-User Lock

**Constraint:** Only one user can have active sessions at a time.

**Rationale:**
- Prevents confusion about which user's commands are running
- Simplifies UI (no need to show multi-user session management)
- Matches typical use case (one person at keyboard)

**Implementation:**
```rust
struct AppState {
    active_user: Option<String>,
    session_manager: SessionManager,
}

impl AppState {
    fn login_user(&mut self, username: String) -> Result<()> {
        if self.active_user.is_some() {
            return Err(AppError::UserAlreadyActive);
        }

        self.active_user = Some(username);
        Ok(())
    }

    fn logout_user(&mut self) -> Result<()> {
        // Close all sessions
        self.session_manager.close_all_sessions()?;

        // Clear active user
        self.active_user = None;

        Ok(())
    }
}
```

**UI Behavior:**
- If user tries to switch profiles while sessions active:
  - Show warning: "You have active browser sessions. Close them before switching profiles."
  - Offer "Close All and Switch" button

## Implementation Roadmap

### Phase 1: User Management (v1.5)
**Goal:** Basic multi-user support with encryption

- [ ] User creation flow (username + password)
- [ ] Password-based encryption (Argon2id)
- [ ] Profile selector UI
- [ ] User directory structure
- [ ] Auto-create default user on first launch
- [ ] user.json and user-profile.md files
- [ ] Profile switching

**Estimated Time:** 2 weeks

### Phase 2: Browser Profile Management (v1.6)
**Goal:** Support ephemeral and named browser profiles

- [ ] Ephemeral profile creation and cleanup
- [ ] Named profile creation UI
- [ ] Default profile configuration
- [ ] Browser profile validation
- [ ] Profile selection priority logic
- [ ] Multiple simultaneous sessions

**Estimated Time:** 2 weeks

### Phase 3: Command System (v1.7)
**Goal:** User-created commands with parameters

- [ ] Command markdown parser
- [ ] Command creation workflow (AI-assisted)
- [ ] Command dropdown UI
- [ ] Parameter definition schema
- [ ] Command storage and versioning
- [ ] Command execution flow

**Estimated Time:** 3 weeks

### Phase 4: Generative UI (v1.8)
**Goal:** Dynamic form generation from commands

- [ ] Generative UI JSON schema
- [ ] UI component renderer (text, dropdown, slider, etc.)
- [ ] Two-column and grid layouts
- [ ] Form validation
- [ ] Integration with command execution
- [ ] Real-time parameter updates via chat

**Estimated Time:** 2 weeks

### Phase 5: Command Refinement (v1.9)
**Goal:** AI-assisted command improvement

- [ ] Thumbs up/down feedback buttons
- [ ] Failure context capture
- [ ] AI refinement suggestions
- [ ] Version increment logic
- [ ] Changelog tracking
- [ ] Rollback to previous versions

**Estimated Time:** 2 weeks

**Total Estimated Time:** 11 weeks

## Testing Requirements

### Unit Tests

**User Management:**
- [ ] Create user with password
- [ ] Encrypt/decrypt user directory
- [ ] Validate password
- [ ] Load user config
- [ ] Save user config

**Browser Profiles:**
- [ ] Create ephemeral profile
- [ ] Create named profile
- [ ] Delete ephemeral after session
- [ ] Resolve profile priority
- [ ] Validate profile exists

**Commands:**
- [ ] Parse command markdown
- [ ] Validate command schema
- [ ] Extract parameters
- [ ] Generate UI from parameters
- [ ] Version increment

### Integration Tests

- [ ] End-to-end user creation flow
- [ ] Login and profile switching
- [ ] Create and execute command
- [ ] Multiple simultaneous browser sessions
- [ ] Command refinement workflow

### Security Tests

- [ ] Password strength validation
- [ ] Encryption key derivation (Argon2id)
- [ ] Decrypt with wrong password (should fail)
- [ ] Profile isolation (users can't see each other's data in UI)

## API Reference

### Rust (Tauri Commands)

```rust
#[tauri::command]
async fn create_user(username: String, password: String) -> Result<(), String>;

#[tauri::command]
async fn login_user(username: String, password: String) -> Result<UserConfig, String>;

#[tauri::command]
async fn logout_user() -> Result<(), String>;

#[tauri::command]
async fn list_users() -> Result<Vec<String>, String>;

#[tauri::command]
async fn create_browser_profile(name: String) -> Result<(), String>;

#[tauri::command]
async fn list_browser_profiles() -> Result<Vec<BrowserProfileInfo>, String>;

#[tauri::command]
async fn set_default_browser_profile(name: String) -> Result<(), String>;

#[tauri::command]
async fn create_command(content: String) -> Result<(), String>;

#[tauri::command]
async fn list_commands() -> Result<Vec<CommandInfo>, String>;

#[tauri::command]
async fn execute_command(
    command_name: String,
    parameters: HashMap<String, Value>,
) -> Result<ExecutionResult, String>;

#[tauri::command]
async fn refine_command(
    command_name: String,
    feedback: String,
) -> Result<String, String>;
```

### TypeScript (Frontend)

```typescript
// User Management
await invoke('create_user', { username: 'alice', password: 'secret' });
const user = await invoke<UserConfig>('login_user', { username: 'alice', password: 'secret' });
await invoke('logout_user');
const users = await invoke<string[]>('list_users');

// Browser Profiles
await invoke('create_browser_profile', { name: 'shopping' });
const profiles = await invoke<BrowserProfileInfo[]>('list_browser_profiles');
await invoke('set_default_browser_profile', { name: 'shopping' });

// Commands
await invoke('create_command', { content: markdownContent });
const commands = await invoke<CommandInfo[]>('list_commands');
const result = await invoke<ExecutionResult>('execute_command', {
  commandName: 'clothing-search',
  parameters: { max_price: 150, size: 'M' },
});
await invoke('refine_command', {
  commandName: 'clothing-search',
  feedback: 'Timeout was too short',
});
```

## Appendix

### Glossary

See "Terminology" section at the beginning of this document.

### References

- [PRD.md](./PRD.md) - Product Requirements Document
- [SCRIPT_FORMAT.md](./SCRIPT_FORMAT.md) - CDP Script Format
- [CHAT_UI.md](./CHAT_UI.md) - Chat Interface Specification

### Change Log

- **2025-10-17**: Initial specification v1.0

---

**Document Version:** 1.0
**Last Updated:** 2025-10-17
**Status:** Approved for Implementation
**Target Release:** v1.5 - v1.9 (over 11 weeks)
