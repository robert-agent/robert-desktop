// Allow dead code for Phase 1 - types will be used in later phases
#![allow(dead_code)]

/// Data structures for user profiles system
///
/// This module defines all the core types used throughout the profiles system,
/// including user configuration, browser profiles, commands, and UI components.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// ============================================================================
// User Configuration and Metadata
// ============================================================================

/// Complete user configuration stored in user.json
///
/// This structure contains all user-specific settings, browser profiles,
/// preferences, and usage statistics. It is serialized to JSON and encrypted
/// with the user's password-derived key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    /// Unique username (filesystem-safe, alphanumeric + underscore/dash)
    pub username: String,

    /// ISO 8601 timestamp of account creation
    pub created_at: DateTime<Utc>,

    /// ISO 8601 timestamp of last successful login
    pub last_login: DateTime<Utc>,

    /// Map of browser profile names to their filesystem paths
    /// Key: profile name (e.g., "shopping", "work")
    /// Value: absolute path to Chromium user-data-dir
    pub browser_profiles: HashMap<String, String>,

    /// Optional default browser profile name to use when none specified
    /// If None, ephemeral profiles are used by default
    pub default_browser_profile: Option<String>,

    /// User preferences and settings
    pub preferences: UserPreferences,

    /// Usage statistics for analytics and insights
    #[serde(default)]
    pub stats: UserStats,
}

/// User preferences and application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// UI theme preference
    pub theme: Theme,

    /// Default timeout for page operations in milliseconds
    /// Typical value: 5000 (5 seconds)
    pub default_timeout_ms: u64,

    /// AI inference mode (local vs cloud)
    pub inference_mode: InferenceMode,

    /// UI language as ISO 639-1 code (e.g., "en", "es", "fr")
    pub language: String,
}

/// UI theme options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// Follow system preference
    System,
}

/// AI inference execution mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InferenceMode {
    /// Run AI inference on local device
    Local,
    /// Send inference requests to cloud APIs
    Cloud,
}

/// User usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStats {
    /// Total number of commands executed
    pub total_commands_run: u64,

    /// Total number of browser sessions launched
    pub total_sessions: u64,

    /// Number of commands created by this user
    pub commands_created: u64,
}

// ============================================================================
// Browser Profile Types
// ============================================================================

/// Browser profile type (ephemeral or persistent)
///
/// This enum distinguishes between temporary browser sessions that are deleted
/// after use and persistent profiles that maintain state across sessions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserProfile {
    /// Temporary browser profile deleted after session ends
    /// Used for privacy-sensitive tasks or one-off operations
    Ephemeral {
        /// Path to temporary directory
        temp_path: PathBuf,
    },

    /// Persistent browser profile with saved state
    /// Used for workflows requiring logged-in accounts, cookies, etc.
    Named {
        /// Profile name (e.g., "shopping", "work")
        name: String,
        /// Path to Chromium user-data-dir
        path: PathBuf,
    },
}

impl BrowserProfile {
    /// Get the filesystem path to the browser profile directory
    pub fn path(&self) -> &PathBuf {
        match self {
            BrowserProfile::Ephemeral { temp_path } => temp_path,
            BrowserProfile::Named { path, .. } => path,
        }
    }

    /// Check if this is an ephemeral profile
    pub fn is_ephemeral(&self) -> bool {
        matches!(self, BrowserProfile::Ephemeral { .. })
    }

    /// Get a human-readable display name for this profile
    pub fn display_name(&self) -> String {
        match self {
            BrowserProfile::Ephemeral { .. } => "Ephemeral (Clean)".to_string(),
            BrowserProfile::Named { name, .. } => name.clone(),
        }
    }
}

/// Browser profile metadata for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfileInfo {
    /// Profile name
    pub name: String,

    /// Filesystem path
    pub path: String,

    /// Whether this is the user's default profile
    pub is_default: bool,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last usage timestamp (None if never used)
    pub last_used: Option<DateTime<Utc>>,
}

// ============================================================================
// Command System Types
// ============================================================================

/// Simple command configuration for Phase 3 (JSON-based, not markdown)
///
/// This is a simplified version for MVP. Full markdown-based commands
/// with frontmatter and generative UI will come in Phase 4.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    /// Unique command identifier (kebab-case)
    /// Example: "clothing-search", "check-prices"
    pub name: String,

    /// Human-readable description shown in UI
    pub description: String,

    /// CDP script template with {{parameter}} placeholders
    /// Example: "Page.navigate {\"url\": \"{{url}}\"}"
    pub script: String,

    /// Command parameters
    pub parameters: Vec<SimpleParameter>,

    /// ISO 8601 timestamp of command creation
    pub created_at: DateTime<Utc>,

    /// ISO 8601 timestamp of last update
    pub updated_at: DateTime<Utc>,
}

/// Simple parameter definition for Phase 3 (text only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleParameter {
    /// Parameter name (used in script as {{name}})
    pub name: String,

    /// Parameter type (text, number, boolean for MVP)
    pub param_type: SimpleParameterType,

    /// User-facing label
    pub label: String,

    /// Whether this parameter is required
    pub required: bool,

    /// Optional default value
    pub default_value: Option<String>,
}

/// Simple parameter types for Phase 3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SimpleParameterType {
    /// Text input
    Text,
    /// Numeric input
    Number,
    /// Boolean checkbox
    Boolean,
}

/// Command frontmatter metadata (parsed from YAML)
///
/// This structure is extracted from the YAML frontmatter section at the
/// beginning of each command markdown file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandFrontmatter {
    /// Unique command identifier (kebab-case)
    /// Example: "clothing-search", "check-prices"
    pub command_name: String,

    /// Human-readable description shown in UI
    pub description: String,

    /// Optional browser profile to use for this command
    /// If None, uses profile selection priority logic
    pub browser_profile: Option<String>,

    /// ISO 8601 timestamp of command creation
    pub created_at: DateTime<Utc>,

    /// ISO 8601 timestamp of last update
    pub updated_at: DateTime<Utc>,

    /// Semantic version string (e.g., "1.2.0")
    pub version: String,

    /// Version history with changelog entries
    /// Example: ["1.0.0: Initial creation", "1.1.0: Added timeout parameter"]
    #[serde(default)]
    pub changelog: Vec<String>,
}

/// Complete command definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Frontmatter metadata
    pub frontmatter: CommandFrontmatter,

    /// Command parameters (inputs from user)
    pub parameters: Vec<CommandParameter>,

    /// Rules and constraints for command execution
    pub rules: Vec<String>,

    /// Success criteria checklist
    pub checklist: Vec<String>,

    /// Optional generative UI specification
    pub generative_ui: Option<GenerativeUI>,

    /// Optional CDP script template (AI-generated)
    pub cdp_script_template: Option<String>,
}

/// Command parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParameter {
    /// Parameter name (snake_case)
    pub name: String,

    /// Parameter type and validation rules
    pub param_type: ParameterType,

    /// User-facing label
    pub label: String,

    /// Optional placeholder text for input fields
    pub placeholder: Option<String>,

    /// Whether this parameter is required
    pub required: bool,

    /// Optional default value
    pub default: Option<serde_json::Value>,
}

/// Parameter types for command inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ParameterType {
    /// Multi-line text input
    TextInput,

    /// Single-line short text input
    ShortText { max_length: Option<usize> },

    /// Dropdown selection
    Dropdown { options: Vec<String> },

    /// Radio button selection (2-4 options)
    Radio { options: Vec<String> },

    /// Boolean checkbox
    Checkbox,

    /// Numeric slider
    Slider {
        min: f64,
        max: f64,
        step: f64,
        unit: Option<String>,
    },

    /// Color picker
    ColorPicker,

    /// Date picker
    DatePicker,
}

/// Command summary for list views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    /// Command identifier
    pub command_name: String,

    /// Description
    pub description: String,

    /// Optional browser profile
    pub browser_profile: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Current version
    pub version: String,
}

// ============================================================================
// Generative UI Types
// ============================================================================

/// Generative UI specification for command parameters
///
/// This defines how the parameter input form should be rendered in the UI,
/// including layout style and component definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerativeUI {
    /// Layout style for components
    pub layout: LayoutType,

    /// UI components to render
    pub components: Vec<UIComponent>,
}

/// Layout types for generative UI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayoutType {
    /// Single column vertical layout
    Vertical,

    /// Two-column side-by-side layout
    TwoColumn,

    /// Responsive grid layout
    Grid,
}

/// Individual UI component definition
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

/// Component types with their specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComponentType {
    /// Multi-line text input
    TextInput {
        placeholder: Option<String>,
        required: bool,
    },

    /// Single-line short text
    ShortText {
        placeholder: Option<String>,
        required: bool,
        max_length: Option<usize>,
    },

    /// Dropdown selection
    Dropdown {
        options: Vec<String>,
        required: bool,
    },

    /// Radio button selection
    Radio {
        options: Vec<String>,
        required: bool,
    },

    /// Boolean checkbox
    Checkbox { default: bool },

    /// Numeric slider
    Slider {
        min: f64,
        max: f64,
        step: f64,
        default: f64,
        unit: Option<String>,
    },

    /// Color picker
    ColorPicker,

    /// Date picker
    DatePicker {
        min: Option<String>,
        max: Option<String>,
        required: bool,
    },
}

// ============================================================================
// Command Execution and Feedback Types
// ============================================================================

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Execution status
    pub status: ExecutionStatus,

    /// Duration in milliseconds
    pub duration_ms: u64,

    /// Number of steps completed successfully
    pub steps_completed: usize,

    /// Number of steps that failed
    pub steps_failed: usize,

    /// Output messages and results
    pub outputs: Vec<String>,

    /// Error messages (if any)
    pub errors: Vec<String>,
}

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    /// All steps completed successfully
    Success,
    /// Command failed with errors
    Failure,
    /// Some steps completed, some failed
    Partial,
}

/// User feedback on command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandFeedback {
    /// Command that was executed
    pub command_name: String,

    /// Thumbs up (true) or thumbs down (false)
    pub thumbs_up: bool,

    /// Optional user comment
    pub user_comment: Option<String>,

    /// Error message if execution failed
    pub error_message: Option<String>,

    /// Execution context for refinement
    pub execution_context: ExecutionContext,
}

/// Context from command execution for refinement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// Parameters used for execution
    pub parameters: HashMap<String, serde_json::Value>,

    /// Browser profile used
    pub browser_profile: String,

    /// Execution duration in milliseconds
    pub execution_time_ms: u64,

    /// Steps completed
    pub steps_completed: usize,

    /// Steps failed
    pub steps_failed: usize,
}

/// Version information for command history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Semantic version string
    pub version: String,

    /// Update timestamp
    pub updated_at: DateTime<Utc>,

    /// Changelog entry for this version
    pub changelog_entry: String,
}

// ============================================================================
// Default Implementations
// ============================================================================

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            default_timeout_ms: 5000,
            inference_mode: InferenceMode::Local,
            language: "en".to_string(),
        }
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            username: "default".to_string(),
            created_at: Utc::now(),
            last_login: Utc::now(),
            browser_profiles: HashMap::new(),
            default_browser_profile: None,
            preferences: UserPreferences::default(),
            stats: UserStats::default(),
        }
    }
}
