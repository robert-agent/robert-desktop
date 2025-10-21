// Debug event types matching Rust backend
export type DebugEventType =
  | { type: 'ChromeDownloading'; data: { message: string } }
  | { type: 'ChromeDownloadProgress'; data: { downloaded: number; total: number; percent: number } }
  | { type: 'ChromeDownloaded'; data: { path: string; version: string } }
  | { type: 'ChromeLaunching'; data: { message: string } }
  | { type: 'ChromeLaunched'; data: { message: string } }
  | { type: 'PageNavigating'; data: { url: string } }
  | { type: 'PageLoading'; data: { message: string } }
  | { type: 'PageLoaded'; data: { url: string; title: string } }
  | { type: 'PageParsing'; data: { message: string } }
  | { type: 'Info'; data: { message: string } }
  | { type: 'Success'; data: { message: string } }
  | { type: 'Error'; data: { message: string; details?: string } }
  | { type: 'ClaudeChecking'; data: { message: string } }
  | { type: 'ClaudeReady'; data: { version: string; path: string; authenticated: boolean } }
  | { type: 'ClaudeNotReady'; data: { issue: string; suggestion: string } }
  | { type: 'ClaudeProcessing'; data: { message: string } }
  | { type: 'ClaudeScreenshot'; data: { path: string } }
  | { type: 'ClaudeHtmlExtracted'; data: { size_kb: number } }
  | { type: 'ClaudeApiCall'; data: { model: string; prompt_preview: string } }
  | { type: 'ClaudeResponse'; data: { preview: string; full_length: number } };

export interface DebugLogEntry {
  id: string;
  timestamp: Date;
  event: DebugEventType;
  level: 'info' | 'success' | 'error' | 'warning';
}

export interface NavigationResult {
  success: boolean;
  url: string;
  title: string;
  message: string;
}

// Developer mode types
export interface SystemPaths {
  installation_dir: string;
  config_dir: string;
  data_dir: string;
  cache_dir: string;
  temp_dir: string;
  current_dir: string;
  chrome_path: string | null;
}

export interface TestServerStatus {
  running: boolean;
  url: string | null;
  port: number | null;
}

export interface ScreenshotInfo {
  path: string;
  filename: string;
  timestamp: number;
  size_bytes: number;
  size_kb: number;
}

// CDP Execution types
export type CommandStatus = 'success' | 'failed' | 'skipped';

// JSON value type matching serde_json::Value from Rust
export type JsonValue =
  | null
  | boolean
  | number
  | string
  | JsonValue[]
  | { [key: string]: JsonValue };

export interface CommandResult {
  step: number;
  method: string;
  status: CommandStatus;
  duration: {
    secs: number;
    nanos: number;
  };
  response?: JsonValue;
  error?: string;
  saved_file?: string;
}

export interface ExecutionReport {
  script_name: string;
  total_commands: number;
  successful: number;
  failed: number;
  skipped: number;
  total_duration: {
    secs: number;
    nanos: number;
  };
  results: CommandResult[];
}

// Agent workflow types
export type WorkflowType = 'cdp_automation' | 'config_update';

export interface ChatMessageRequest {
  message: string;
  workflow_type: WorkflowType;
  agent_name?: string;
  include_screenshot?: boolean;
  include_html?: boolean;
}

export interface ClarificationQuestion {
  question: string;
  options: string[];
  context?: string;
}

export interface WorkflowResult {
  success: boolean;
  message: string;
  error?: string;
  cdp_script?: string;
  execution_report?: ExecutionReport;
  clarification?: ClarificationQuestion[];
  understanding?: string;
}

// User Profile Management Types
// These types match the Rust backend ProfileResult<T> and UserConfig types

/**
 * Generic result type for profile operations
 * Matches Rust's ProfileResult<T> serialization
 */
export interface ProfileResult<T> {
  success: boolean;
  data?: T;
  error?: string;
}

/**
 * User configuration and metadata
 * Stored encrypted in ~/.robert/users/{username}/user.json.enc
 */
export interface UserConfig {
  username: string;
  created_at: string; // ISO 8601 timestamp
  last_login: string; // ISO 8601 timestamp
  browser_profiles: Record<string, string>; // profile name -> path
  default_browser_profile?: string;
  preferences: UserPreferences;
  stats: UserStats;
}

/**
 * User preferences for UI and behavior
 */
export interface UserPreferences {
  theme: 'light' | 'dark' | 'system';
  default_timeout_ms: number;
  inference_mode: 'local' | 'cloud';
  language: string; // ISO 639-1 code (e.g., "en")
}

/**
 * User usage statistics
 */
export interface UserStats {
  total_commands_run: number;
  total_sessions: number;
  commands_created: number;
}

/**
 * Password validation result with strength indicator
 */
export interface PasswordValidation {
  valid: boolean;
  strength: 'weak' | 'medium' | 'strong';
  errors: string[];
  suggestions: string[];
}

// ============================================================================
// Command System Types (Phase 3)
// ============================================================================

/**
 * Simple parameter type for command inputs
 */
export type SimpleParameterType = 'text' | 'number' | 'boolean';

/**
 * Simple parameter definition for commands
 */
export interface SimpleParameter {
  name: string; // Parameter name (used in script as {{name}})
  param_type: SimpleParameterType;
  label: string; // User-facing label
  required: boolean;
  default_value?: string;
}

/**
 * Command configuration
 * Stored as encrypted JSON in ~/.robert/users/{username}/commands/
 */
export interface CommandConfig {
  name: string; // Unique command identifier (kebab-case)
  description: string; // Human-readable description
  script: string; // CDP script template with {{parameter}} placeholders
  parameters: SimpleParameter[];
  created_at: string; // ISO 8601 timestamp
  updated_at: string; // ISO 8601 timestamp
}

/**
 * Command summary information for list views
 */
export interface CommandInfo {
  name: string;
  description: string;
  parameter_count: number;
  created_at: string; // ISO 8601 timestamp
  updated_at: string; // ISO 8601 timestamp
}
