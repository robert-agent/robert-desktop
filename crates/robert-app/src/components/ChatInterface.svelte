<script lang="ts">
  import { onMount } from 'svelte';
  import { launchBrowser, processChatMessage } from '../lib/tauri';
  import type { WorkflowType } from '../lib/types';
  import UrlInput from './UrlInput.svelte';

  let message = '';
  let messages: Array<{ role: 'user' | 'agent'; text: string; timestamp: Date }> = [];
  let loading = false;
  let browserLaunched = false;

  onMount(async () => {
    // Check if browser is already launched
    // TODO: Add a command to check browser status
  });

  async function handleSend() {
    if (!message.trim() || loading) return;

    const userMessage = message.trim();
    message = '';

    // Add user message to chat
    messages = [...messages, { role: 'user', text: userMessage, timestamp: new Date() }];

    loading = true;

    try {
      // Ensure browser is launched
      if (!browserLaunched) {
        // Get screen dimensions
        const screenWidth = window.screen.width;
        const screenHeight = window.screen.height;

        await launchBrowser(screenWidth, screenHeight);
        browserLaunched = true;
      }

      // Call the agent workflow to process the message
      const result = await processChatMessage({
        message: userMessage,
        workflow_type: 'cdp_automation' as WorkflowType,
        agent_name: 'cdp-generator',
        include_screenshot: true,
        include_html: true,
      });

      // Add agent response to chat
      messages = [
        ...messages,
        {
          role: 'agent',
          text: result.message,
          timestamp: new Date(),
        },
      ];

      // If there was an error, show it
      if (!result.success && result.error) {
        messages = [
          ...messages,
          {
            role: 'agent',
            text: `Error details: ${result.error}`,
            timestamp: new Date(),
          },
        ];
      }
    } catch (error) {
      console.error('Error processing message:', error);
      messages = [
        ...messages,
        {
          role: 'agent',
          text: `Error: ${error}`,
          timestamp: new Date(),
        },
      ];
    } finally {
      loading = false;
    }
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSend();
    }
  }

  function formatTime(date: Date): string {
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
    });
  }
</script>

<div class="chat-container">
  <UrlInput />

  <div class="messages-container">
    {#if messages.length === 0}
      <div class="empty-state">
        <div class="welcome-icon">🤖</div>
        <h3>Welcome to Robert!</h3>
        <p>I can help you automate browser tasks.</p>
        <p class="examples-title">Try asking me to:</p>
        <ul class="examples">
          <li>"Click the submit button"</li>
          <li>"Fill in the form with my name"</li>
          <li>"Take a screenshot of the page"</li>
          <li>"Scroll to the bottom"</li>
        </ul>
      </div>
    {:else}
      {#each messages as msg (msg.timestamp.getTime())}
        <div class="message {msg.role}">
          <div class="message-header">
            <span class="role-label">
              {msg.role === 'user' ? 'You' : 'Robert'}
            </span>
            <span class="timestamp">{formatTime(msg.timestamp)}</span>
          </div>
          <div class="message-text">{msg.text}</div>
        </div>
      {/each}
    {/if}
  </div>

  <div class="input-container">
    <textarea
      bind:value={message}
      on:keypress={handleKeyPress}
      placeholder="Type your message... (Shift+Enter for new line)"
      disabled={loading}
      rows="3"
    ></textarea>
    <button on:click={handleSend} disabled={!message.trim() || loading} class="send-button">
      {#if loading}
        <span class="spinner"></span> Thinking...
      {:else}
        Send
      {/if}
    </button>
  </div>
</div>

<style>
  .chat-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #ffffff;
  }

  .messages-container {
    flex: 1;
    overflow-y: auto;
    padding: 0.75rem;
    background: #f8f9fa;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    text-align: center;
    color: #666;
    padding: 1rem;
  }

  .welcome-icon {
    font-size: 2.5rem;
    margin-bottom: 0.75rem;
  }

  .empty-state h3 {
    margin: 0 0 0.4rem 0;
    font-size: 1.2rem;
    color: #333;
  }

  .empty-state p {
    margin: 0.2rem 0;
    color: #666;
    font-size: 0.9rem;
  }

  .examples-title {
    margin-top: 1rem !important;
    font-weight: 600;
    color: #667eea;
    font-size: 0.9rem;
  }

  .examples {
    list-style: none;
    padding: 0;
    margin: 0.5rem 0 0 0;
    text-align: left;
    max-width: 400px;
  }

  .examples li {
    margin: 0.4rem 0;
    padding: 0.6rem 0.8rem;
    background: white;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    color: #555;
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: 0.85rem;
  }

  .message {
    margin-bottom: 0.75rem;
    padding: 0.75rem;
    border-radius: 10px;
    max-width: 85%;
    animation: slideIn 0.2s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .message.user {
    background: #667eea;
    color: white;
    margin-left: auto;
  }

  .message.agent {
    background: white;
    border: 1px solid #e0e0e0;
    color: #333;
    margin-right: auto;
  }

  .message-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.4rem;
    font-size: 0.75rem;
  }

  .role-label {
    font-weight: 600;
    font-size: 0.8rem;
  }

  .message.user .role-label {
    color: rgba(255, 255, 255, 0.9);
  }

  .message.agent .role-label {
    color: #667eea;
  }

  .timestamp {
    opacity: 0.7;
    font-size: 0.75rem;
  }

  .message-text {
    white-space: pre-wrap;
    word-break: break-word;
    line-height: 1.5;
  }

  .input-container {
    padding: 0.75rem;
    border-top: 1px solid #e0e0e0;
    background: white;
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
  }

  textarea {
    width: 100%;
    padding: 0.6rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.9rem;
    resize: none;
    transition: border-color 0.2s;
  }

  textarea:focus {
    outline: none;
    border-color: #667eea;
  }

  textarea:disabled {
    background: #f5f5f5;
    cursor: not-allowed;
  }

  .send-button {
    align-self: flex-end;
    padding: 0.6rem 1.5rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .send-button:hover:not(:disabled) {
    background: #5568d3;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  }

  .send-button:disabled {
    background: #ccc;
    cursor: not-allowed;
    transform: none;
  }

  .spinner {
    display: inline-block;
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
