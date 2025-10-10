import { invoke } from '@tauri-apps/api/core';
import type { NavigationResult, SystemPaths, TestServerStatus } from './types';

export async function launchBrowser(): Promise<string> {
  return await invoke<string>('launch_browser');
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
