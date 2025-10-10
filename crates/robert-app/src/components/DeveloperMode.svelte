<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import {
    getSystemPaths,
    startDevTestServer,
    stopDevTestServer,
    getDevTestServerStatus,
    launchBrowser,
    navigateToUrl,
  } from '../lib/tauri';
  import type { SystemPaths, TestServerStatus } from '../lib/types';

  let systemPaths: SystemPaths | null = null;
  let serverStatus: TestServerStatus = { running: false, url: null, port: null };
  let loading = false;
  let error: string | null = null;
  let statusCheckInterval: number | null = null;

  onMount(async () => {
    await loadSystemPaths();
    await checkServerStatus();

    // Poll server status every 2 seconds
    statusCheckInterval = window.setInterval(async () => {
      await checkServerStatus();
    }, 2000);
  });

  onDestroy(() => {
    if (statusCheckInterval) {
      clearInterval(statusCheckInterval);
    }
  });

  async function loadSystemPaths() {
    try {
      loading = true;
      error = null;
      systemPaths = await getSystemPaths();
    } catch (e) {
      error = `Failed to load system paths: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function checkServerStatus() {
    try {
      serverStatus = await getDevTestServerStatus();
    } catch (e) {
      console.error('Failed to check server status:', e);
    }
  }

  async function handleStartServer() {
    try {
      loading = true;
      error = null;
      serverStatus = await startDevTestServer();
    } catch (e) {
      error = `Failed to start server: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function handleStopServer() {
    try {
      loading = true;
      error = null;
      serverStatus = await stopDevTestServer();
    } catch (e) {
      error = `Failed to stop server: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function handleOpenInBrowser() {
    if (!serverStatus.url) return;

    try {
      loading = true;
      error = null;

      // Ensure browser is launched
      await launchBrowser();

      // Navigate to test server
      await navigateToUrl(serverStatus.url);
    } catch (e) {
      error = `Failed to open in browser: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text);
  }
</script>

<div class="developer-mode">
  <h2>🛠️ Developer Mode</h2>

  {#if error}
    <div class="error-banner">
      {error}
    </div>
  {/if}

  <!-- System Paths Section -->
  <section class="section">
    <h3>📁 System Paths</h3>
    {#if loading && !systemPaths}
      <div class="loading">Loading system paths...</div>
    {:else if systemPaths}
      <div class="paths-grid">
        <div class="path-item">
          <span class="path-label">Installation Directory:</span>
          <div class="path-value">
            <code>{systemPaths.installation_dir}</code>
            <button
              class="copy-btn"
              on:click={() => copyToClipboard(systemPaths!.installation_dir)}
            >
              📋
            </button>
          </div>
        </div>

        <div class="path-item">
          <span class="path-label">Config Directory:</span>
          <div class="path-value">
            <code>{systemPaths.config_dir}</code>
            <button class="copy-btn" on:click={() => copyToClipboard(systemPaths!.config_dir)}>
              📋
            </button>
          </div>
        </div>

        <div class="path-item">
          <span class="path-label">Data Directory:</span>
          <div class="path-value">
            <code>{systemPaths.data_dir}</code>
            <button class="copy-btn" on:click={() => copyToClipboard(systemPaths!.data_dir)}>
              📋
            </button>
          </div>
        </div>

        <div class="path-item">
          <span class="path-label">Cache Directory:</span>
          <div class="path-value">
            <code>{systemPaths.cache_dir}</code>
            <button class="copy-btn" on:click={() => copyToClipboard(systemPaths!.cache_dir)}>
              📋
            </button>
          </div>
        </div>

        <div class="path-item">
          <span class="path-label">Temp Directory:</span>
          <div class="path-value">
            <code>{systemPaths.temp_dir}</code>
            <button class="copy-btn" on:click={() => copyToClipboard(systemPaths!.temp_dir)}>
              📋
            </button>
          </div>
        </div>

        <div class="path-item">
          <span class="path-label">Current Directory:</span>
          <div class="path-value">
            <code>{systemPaths.current_dir}</code>
            <button class="copy-btn" on:click={() => copyToClipboard(systemPaths!.current_dir)}>
              📋
            </button>
          </div>
        </div>

        {#if systemPaths.chrome_path}
          <div class="path-item">
            <span class="path-label">Chrome Path:</span>
            <div class="path-value">
              <code>{systemPaths!.chrome_path}</code>
              <button class="copy-btn" on:click={() => copyToClipboard(systemPaths!.chrome_path!)}>
                📋
              </button>
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </section>

  <!-- Test Server Section -->
  <section class="section">
    <h3>🌐 Mock Test Server</h3>
    <p class="description">
      Start a local HTTP server for manual e2e testing. Use this to test CDP commands against a
      controlled webpage environment.
    </p>

    <div class="server-status">
      <div class="status-indicator">
        <span class="status-dot {serverStatus.running ? 'running' : 'stopped'}"></span>
        <span class="status-text">
          {serverStatus.running ? 'Running' : 'Stopped'}
        </span>
      </div>

      {#if serverStatus.running && serverStatus.url}
        <div class="server-info">
          <div class="info-item">
            <strong>URL:</strong>
            <a href={serverStatus.url} target="_blank">{serverStatus.url}</a>
            <button class="copy-btn" on:click={() => copyToClipboard(serverStatus.url!)}>
              📋
            </button>
          </div>
          <div class="info-item">
            <strong>Port:</strong>
            {serverStatus.port}
          </div>
        </div>
      {/if}
    </div>

    <div class="server-controls">
      {#if !serverStatus.running}
        <button class="btn btn-primary" on:click={handleStartServer} disabled={loading}>
          {loading ? 'Starting...' : 'Start Server'}
        </button>
      {:else}
        <button class="btn btn-primary" on:click={handleOpenInBrowser} disabled={loading}>
          {loading ? 'Loading...' : 'Open in Browser'}
        </button>
        <button class="btn btn-secondary" on:click={handleStopServer} disabled={loading}>
          {loading ? 'Stopping...' : 'Stop Server'}
        </button>
      {/if}
    </div>

    {#if serverStatus.running}
      <div class="server-guide">
        <h4>💡 Quick Start Guide</h4>
        <ol>
          <li>Click "Open in Browser" to navigate to the test page</li>
          <li>Use the chat interface to test CDP commands</li>
          <li>The test page includes interactive elements for testing automation</li>
          <li>Check the DebugView to see CDP commands being executed</li>
        </ol>
      </div>
    {/if}
  </section>
</div>

<style>
  .developer-mode {
    padding: 1.5rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  h2 {
    margin: 0 0 1.5rem 0;
    font-size: 1.75rem;
    color: #333;
  }

  .error-banner {
    background: #fee;
    border: 1px solid #fcc;
    color: #c33;
    padding: 1rem;
    margin-bottom: 1rem;
    border-radius: 4px;
  }

  .section {
    background: white;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  h3 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    color: #667eea;
  }

  .description {
    color: #666;
    margin-bottom: 1rem;
    line-height: 1.5;
  }

  .loading {
    color: #999;
    font-style: italic;
  }

  .paths-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .path-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .path-label {
    font-weight: 600;
    color: #555;
    font-size: 0.875rem;
  }

  .path-value {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: #f8f8f8;
    padding: 0.5rem;
    border-radius: 4px;
    border: 1px solid #e0e0e0;
  }

  .path-value code {
    flex: 1;
    font-size: 0.875rem;
    word-break: break-all;
    color: #333;
  }

  .copy-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 1rem;
    padding: 0.25rem 0.5rem;
    opacity: 0.6;
    transition: opacity 0.2s;
  }

  .copy-btn:hover {
    opacity: 1;
  }

  .server-status {
    margin-bottom: 1rem;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .status-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    display: inline-block;
  }

  .status-dot.running {
    background: #4caf50;
    box-shadow: 0 0 8px rgba(76, 175, 80, 0.6);
    animation: pulse 2s infinite;
  }

  .status-dot.stopped {
    background: #999;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .status-text {
    font-weight: 600;
    font-size: 1rem;
  }

  .server-info {
    background: #f0f8ff;
    padding: 1rem;
    border-radius: 4px;
    border: 1px solid #d0e8ff;
  }

  .info-item {
    margin-bottom: 0.5rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .info-item:last-child {
    margin-bottom: 0;
  }

  .info-item a {
    color: #667eea;
    text-decoration: none;
  }

  .info-item a:hover {
    text-decoration: underline;
  }

  .server-controls {
    display: flex;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #667eea;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #5568d3;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #d0d0d0;
  }

  .server-guide {
    background: #fffbea;
    padding: 1rem;
    border-radius: 4px;
    border: 1px solid #ffe8a1;
  }

  .server-guide h4 {
    margin: 0 0 0.75rem 0;
    color: #976a00;
    font-size: 1rem;
  }

  .server-guide ol {
    margin: 0;
    padding-left: 1.5rem;
    color: #666;
  }

  .server-guide li {
    margin-bottom: 0.5rem;
    line-height: 1.5;
  }

  .server-guide li:last-child {
    margin-bottom: 0;
  }
</style>
