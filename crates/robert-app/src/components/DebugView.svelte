<script lang="ts">
  import { debugLogs, clearDebugLogs } from '$lib/stores';
  import { afterUpdate } from 'svelte';
  import type { DebugLogEntry } from '$lib/types';

  let logContainer: HTMLDivElement;
  let autoScroll = true;

  afterUpdate(() => {
    if (autoScroll && logContainer) {
      logContainer.scrollTop = logContainer.scrollHeight;
    }
  });

  function formatTime(date: Date): string {
    return date.toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      fractionalSecondDigits: 3,
    });
  }

  function getEventMessage(entry: DebugLogEntry): string {
    const event = entry.event;

    switch (event.type) {
      case 'ChromeDownloading':
      case 'ChromeLaunching':
      case 'ChromeLaunched':
      case 'PageLoading':
      case 'PageParsing':
      case 'ClaudeChecking':
      case 'ClaudeProcessing':
      case 'Info':
      case 'Success':
        return event.data.message;

      case 'ChromeDownloadProgress':
        return `Downloading Chrome: ${event.data.percent}% (${event.data.downloaded}/${event.data.total} bytes)`;

      case 'ChromeDownloaded':
        return `Chrome downloaded: ${event.data.version} at ${event.data.path}`;

      case 'PageNavigating':
        return `Navigating to: ${event.data.url}`;

      case 'PageLoaded':
        return `Page loaded: ${event.data.title} (${event.data.url})`;

      case 'ClaudeReady':
        return `✓ Claude CLI ready: ${event.data.version} at ${event.data.path} (${event.data.authenticated ? 'authenticated' : 'not authenticated'})`;

      case 'ClaudeNotReady':
        return `✗ Claude CLI issue: ${event.data.issue}\n  → ${event.data.suggestion}`;

      case 'ClaudeScreenshot':
        return `📸 Screenshot captured: ${event.data.path}`;

      case 'ClaudeHtmlExtracted':
        return `📄 HTML extracted: ${event.data.size_kb} KB`;

      case 'ClaudeApiCall':
        return `🤖 Calling Claude (${event.data.model}): "${event.data.prompt_preview}"`;

      case 'ClaudeResponse':
        return `💬 Claude responded (${event.data.full_length} chars): "${event.data.preview}"`;

      case 'Error':
        return `Error: ${event.data.message}${event.data.details ? `\n${event.data.details}` : ''}`;

      default:
        // Exhaustive check - should never reach here
        return JSON.stringify((event as any).data);
    }
  }

  function getIcon(level: string): string {
    switch (level) {
      case 'success':
        return '✓';
      case 'error':
        return '✗';
      case 'warning':
        return '⚠';
      default:
        return '•';
    }
  }

  function handleClear() {
    clearDebugLogs();
  }

  function toggleAutoScroll() {
    autoScroll = !autoScroll;
  }
</script>

<div class="debug-view">
  <div class="debug-header">
    <h3>Debug Log</h3>
    <div class="controls">
      <label>
        <input type="checkbox" bind:checked={autoScroll} on:change={toggleAutoScroll} />
        Auto-scroll
      </label>
      <button on:click={handleClear} class="clear-btn">Clear</button>
    </div>
  </div>

  <div class="log-container" bind:this={logContainer}>
    {#if $debugLogs.length === 0}
      <div class="empty-state">
        No debug events yet. Launch browser and navigate to see activity.
      </div>
    {:else}
      {#each $debugLogs as log (log.id)}
        <div class="log-entry {log.level}">
          <span class="timestamp">{formatTime(log.timestamp)}</span>
          <span class="icon">{getIcon(log.level)}</span>
          <span class="message">{getEventMessage(log)}</span>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .debug-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #1e1e1e;
    color: #d4d4d4;
  }

  .debug-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: #252525;
    border-bottom: 1px solid #3a3a3a;
  }

  .debug-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #d4d4d4;
  }

  .controls {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .controls label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
    color: #a0a0a0;
    cursor: pointer;
  }

  .clear-btn {
    padding: 0.4rem 0.8rem;
    font-size: 0.85rem;
    color: #d4d4d4;
    background: #3a3a3a;
    border: 1px solid #555;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .clear-btn:hover {
    background: #4a4a4a;
  }

  .log-container {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 0.85rem;
  }

  .empty-state {
    text-align: center;
    padding: 3rem 1rem;
    color: #666;
    font-style: italic;
  }

  .log-entry {
    display: flex;
    gap: 0.75rem;
    padding: 0.4rem 0.5rem;
    margin-bottom: 0.25rem;
    border-left: 3px solid transparent;
    transition: background 0.1s;
  }

  .log-entry:hover {
    background: #2a2a2a;
  }

  .log-entry.info {
    border-left-color: #4a90e2;
  }

  .log-entry.success {
    border-left-color: #4caf50;
  }

  .log-entry.error {
    border-left-color: #f44336;
    background: #2a1a1a;
  }

  .log-entry.warning {
    border-left-color: #ff9800;
  }

  .timestamp {
    color: #858585;
    min-width: 6rem;
    font-size: 0.8rem;
  }

  .icon {
    min-width: 1rem;
    text-align: center;
  }

  .log-entry.info .icon {
    color: #4a90e2;
  }

  .log-entry.success .icon {
    color: #4caf50;
  }

  .log-entry.error .icon {
    color: #f44336;
  }

  .log-entry.warning .icon {
    color: #ff9800;
  }

  .message {
    flex: 1;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .log-entry.error .message {
    color: #ff8080;
  }
</style>
