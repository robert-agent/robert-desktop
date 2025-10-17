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
