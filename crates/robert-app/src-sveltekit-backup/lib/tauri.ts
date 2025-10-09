import { invoke } from '@tauri-apps/api/core';
import type { NavigationResult } from './types';

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
