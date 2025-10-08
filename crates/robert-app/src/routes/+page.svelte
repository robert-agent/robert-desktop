<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { setupEventListeners, cleanupEventListeners } from '$lib/events';
  import UrlInput from '../components/UrlInput.svelte';
  import DebugView from '../components/DebugView.svelte';

  onMount(async () => {
    // Setup event listeners for debug events
    await setupEventListeners();
  });

  onDestroy(() => {
    // Cleanup event listeners
    cleanupEventListeners();
  });
</script>

<svelte:head>
  <title>Robert - Browser Automation</title>
</svelte:head>

<div class="app-container">
  <header>
    <h1>🤖 Robert</h1>
    <p>Browser Automation for Everyone</p>
  </header>

  <main>
    <div class="url-section">
      <UrlInput />
    </div>

    <div class="debug-section">
      <DebugView />
    </div>
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
</style>
