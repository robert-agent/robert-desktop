import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export interface UpdateCheckResult {
  available: boolean;
  currentVersion: string;
  version?: string;
  date?: string;
  body?: string; // Release notes
}

export interface UpdateProgress {
  downloaded: number;
  total: number;
  percentage: number;
}

export type UpdaterCallback = (_progress: UpdateProgress) => void;

/**
 * Check for available updates silently (no user prompts)
 * @returns Update information if available, null otherwise
 */
export async function checkForUpdates(): Promise<UpdateCheckResult | null> {
  try {
    console.log('[Updater] Checking for updates...');

    const update = await check();

    if (!update) {
      console.error('[Updater] Failed to check for updates');
      return null;
    }

    if (!update.available) {
      console.log('[Updater] No update available, running latest version');
      return {
        available: false,
        currentVersion: update.currentVersion,
      };
    }

    console.log(`[Updater] Update available: ${update.version}`);
    console.log(`[Updater] Current version: ${update.currentVersion}`);

    return {
      available: true,
      currentVersion: update.currentVersion,
      version: update.version,
      date: update.date,
      body: update.body,
    };
  } catch (error) {
    console.error('[Updater] Error checking for updates:', error);
    return null;
  }
}

/**
 * Download and install an available update
 * @param onProgress Optional callback for download progress
 * @returns True if update was successful, false otherwise
 */
export async function downloadAndInstallUpdate(onProgress?: UpdaterCallback): Promise<boolean> {
  try {
    console.log('[Updater] Starting update download and installation...');

    const update = await check();

    if (!update || !update.available) {
      console.error('[Updater] No update available to install');
      return false;
    }

    let totalBytes = 0;
    let downloadedBytes = 0;

    await update.downloadAndInstall((event) => {
      switch (event.event) {
        case 'Started':
          totalBytes = event.data.contentLength || 0;
          console.log(`[Updater] Download started: ${totalBytes} bytes`);
          if (onProgress) {
            onProgress({
              downloaded: 0,
              total: totalBytes,
              percentage: 0,
            });
          }
          break;

        case 'Progress': {
          downloadedBytes += event.data.chunkLength;
          const percentage = totalBytes > 0 ? (downloadedBytes / totalBytes) * 100 : 0;
          console.log(
            `[Updater] Progress: ${downloadedBytes}/${totalBytes} bytes (${percentage.toFixed(1)}%)`
          );
          if (onProgress) {
            onProgress({
              downloaded: downloadedBytes,
              total: totalBytes,
              percentage,
            });
          }
          break;
        }

        case 'Finished':
          console.log('[Updater] Download complete, installing...');
          if (onProgress) {
            onProgress({
              downloaded: totalBytes,
              total: totalBytes,
              percentage: 100,
            });
          }
          break;
      }
    });

    console.log('[Updater] Update installed successfully, relaunching app...');

    // Small delay to ensure progress UI updates
    await new Promise((resolve) => setTimeout(resolve, 500));

    // Relaunch the application
    await relaunch();

    return true;
  } catch (error) {
    console.error('[Updater] Error during update:', error);
    return false;
  }
}

/**
 * Format bytes to human-readable string
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
}
