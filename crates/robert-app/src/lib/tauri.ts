import { invoke } from '@tauri-apps/api/core';
import type {
  NavigationResult,
  SystemPaths,
  TestServerStatus,
  ScreenshotInfo,
  ExecutionReport,
  ChatMessageRequest,
  WorkflowResult,
} from './types';

export async function launchBrowser(screenWidth?: number, screenHeight?: number): Promise<string> {
  return await invoke<string>('launch_browser', {
    screenWidth: screenWidth ? screenWidth : null,
    screenHeight: screenHeight ? screenHeight : null,
  });
}

export async function navigateToUrl(url: string): Promise<NavigationResult> {
  return await invoke<NavigationResult>('navigate_to_url', { url });
}

export async function getPageContent(): Promise<string> {
  return await invoke<string>('get_page_content');
}

export async function closeBrowser(): Promise<void> {
  return await invoke<void>('close_browser');
}

// Developer mode commands
export async function getSystemPaths(): Promise<SystemPaths> {
  return await invoke<SystemPaths>('get_system_paths');
}

export async function startDevTestServer(): Promise<TestServerStatus> {
  return await invoke<TestServerStatus>('start_dev_test_server');
}

export async function stopDevTestServer(): Promise<TestServerStatus> {
  return await invoke<TestServerStatus>('stop_dev_test_server');
}

export async function getDevTestServerStatus(): Promise<TestServerStatus> {
  return await invoke<TestServerStatus>('get_dev_test_server_status');
}

// Screenshot management commands
export async function devCaptureScreenshot(): Promise<ScreenshotInfo> {
  return await invoke<ScreenshotInfo>('dev_capture_screenshot');
}

export async function devListScreenshots(): Promise<ScreenshotInfo[]> {
  return await invoke<ScreenshotInfo[]>('dev_list_screenshots');
}

export async function devDeleteAllScreenshots(): Promise<number> {
  return await invoke<number>('dev_delete_all_screenshots');
}

export async function devDeleteScreenshot(path: string): Promise<void> {
  return await invoke<void>('dev_delete_screenshot', { path });
}

// CDP script execution
export async function executeCdpScript(scriptJson: string): Promise<ExecutionReport> {
  return await invoke<ExecutionReport>('execute_cdp_script', { scriptJson });
}

// Agent workflow commands
export async function processChatMessage(request: ChatMessageRequest): Promise<WorkflowResult> {
  return await invoke<WorkflowResult>('process_chat_message', { request });
}
