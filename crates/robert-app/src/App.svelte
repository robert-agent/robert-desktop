<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { setupEventListeners, cleanupEventListeners } from './lib/events';
  import UrlInput from './components/UrlInput.svelte';
  import DebugView from './components/DebugView.svelte';
  import DeveloperMode from './components/DeveloperMode.svelte';

  let showDeveloperMode = false;

  onMount(async () => {
    // Setup event listeners for debug events
    await setupEventListeners();
  });

  onDestroy(() => {
    // Cleanup event listeners
    cleanupEventListeners();
  });

  function toggleDeveloperMode() {
    showDeveloperMode = !showDeveloperMode;
  }
</script>

<div class="app-container">
  <header>
    <div class="header-content">
      <div class="title-section">
        <h1>🤖 Robert</h1>
        <p>Browser Automation for Everyone</p>
      </div>
      <button class="dev-mode-toggle" on:click={toggleDeveloperMode}>
        {showDeveloperMode ? '✖' : '🛠️'} Developer Mode
      </button>
    </div>
  </header>

  <main>
    {#if showDeveloperMode}
      <div class="developer-section">
        <DeveloperMode />
      </div>
    {:else}
      <div class="url-section">
        <UrlInput />
      </div>

      <div class="debug-section">
        <DebugView />
      </div>
    {/if}
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    background: #ffffff;
  }

  .app-container {
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 1rem 1.5rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .title-section {
    flex: 1;
  }

  header h1 {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 700;
  }

  header p {
    margin: 0.25rem 0 0 0;
    font-size: 0.9rem;
    opacity: 0.9;
  }

  .dev-mode-toggle {
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.3);
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 500;
    transition: all 0.2s;
  }

  .dev-mode-toggle:hover {
    background: rgba(255, 255, 255, 0.3);
    border-color: rgba(255, 255, 255, 0.5);
  }

  main {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .url-section {
    flex-shrink: 0;
  }

  .debug-section {
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  .developer-section {
    flex: 1;
    min-height: 0;
    overflow: auto;
    background: #f5f5f5;
  }
</style>
