// Debug event types matching Rust backend
export type DebugEventType =
  | { type: "ChromeDownloading"; data: { message: string } }
  | { type: "ChromeDownloadProgress"; data: { downloaded: number; total: number; percent: number } }
  | { type: "ChromeDownloaded"; data: { path: string; version: string } }
  | { type: "ChromeLaunching"; data: { message: string } }
  | { type: "ChromeLaunched"; data: { message: string } }
  | { type: "PageNavigating"; data: { url: string } }
  | { type: "PageLoading"; data: { message: string } }
  | { type: "PageLoaded"; data: { url: string; title: string } }
  | { type: "PageParsing"; data: { message: string } }
  | { type: "Info"; data: { message: string } }
  | { type: "Success"; data: { message: string } }
  | { type: "Error"; data: { message: string; details?: string } };

export interface DebugLogEntry {
  id: string;
  timestamp: Date;
  event: DebugEventType;
  level: "info" | "success" | "error" | "warning";
}

export interface NavigationResult {
  success: boolean;
  url: string;
  title: string;
  message: string;
}
