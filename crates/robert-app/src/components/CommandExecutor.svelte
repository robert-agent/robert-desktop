<script lang="ts">
  import { onMount } from 'svelte';
  import { getCommand, executeCommand, executeCdpScript } from '../lib/tauri';
  import type { CommandConfig, ExecutionReport } from '../lib/types';

  export let commandName: string;
  export let onClose: () => void;

  let command: CommandConfig | null = null;
  let parameterValues: Record<string, string> = {};
  let loading = false;
  let executing = false;
  let error = '';
  let executionReport: ExecutionReport | null = null;

  onMount(async () => {
    await loadCommand();
  });

  async function loadCommand() {
    loading = true;
    error = '';
    try {
      const result = await getCommand(commandName);
      if (result.success && result.data) {
        command = result.data;
        // Initialize parameter values with defaults
        command.parameters.forEach((param) => {
          parameterValues[param.name] = param.default_value || '';
        });
      } else {
        error = result.error || 'Failed to load command';
      }
    } catch (e) {
      error = `Error: ${e}`;
      console.error('Failed to load command:', e);
    } finally {
      loading = false;
    }
  }

  async function handleExecute() {
    if (!command) return;

    error = '';
    executionReport = null;

    // Validate required parameters
    for (const param of command.parameters) {
      if (param.required && !parameterValues[param.name]?.trim()) {
        error = `Parameter "${param.label}" is required`;
        return;
      }

      // Type validation
      if (parameterValues[param.name]) {
        const value = parameterValues[param.name];
        if (param.param_type === 'number' && isNaN(Number(value))) {
          error = `Parameter "${param.label}" must be a number`;
          return;
        }
        if (
          param.param_type === 'boolean' &&
          value !== 'true' &&
          value !== 'false' &&
          value !== ''
        ) {
          error = `Parameter "${param.label}" must be true or false`;
          return;
        }
      }
    }

    executing = true;

    try {
      // Step 1: Execute command to get substituted CDP script
      const result = await executeCommand(commandName, parameterValues);
      if (!result.success || !result.data) {
        error = result.error || 'Failed to execute command';
        return;
      }

      const cdpScript = result.data;
      console.log('Substituted CDP script:', cdpScript);

      // Step 2: Execute the CDP script
      const report = await executeCdpScript(cdpScript);
      executionReport = report;

      if (report.failed > 0) {
        error = `Command executed but ${report.failed} step(s) failed`;
      }
    } catch (e) {
      error = `Error: ${e}`;
      console.error('Failed to execute command:', e);
    } finally {
      executing = false;
    }
  }

  function formatDuration(duration: { secs: number; nanos: number }): string {
    const ms = duration.secs * 1000 + duration.nanos / 1000000;
    if (ms < 1000) {
      return `${ms.toFixed(0)}ms`;
    }
    return `${(ms / 1000).toFixed(2)}s`;
  }
</script>

<div class="command-executor">
  <div class="header">
    <h2>Execute: {commandName}</h2>
    <button class="btn-close" on:click={onClose}>×</button>
  </div>

  {#if loading}
    <div class="loading">Loading command...</div>
  {:else if command}
    <div class="command-info">
      <p class="description">{command.description}</p>
    </div>

    <form on:submit|preventDefault={handleExecute}>
      {#if error}
        <div class="error-message">{error}</div>
      {/if}

      {#if command.parameters.length > 0}
        <div class="parameters">
          <h3>Parameters</h3>
          {#each command.parameters as param}
            <div class="param-group">
              <label for={param.name}>
                {param.label}
                {#if param.required}
                  <span class="required">*</span>
                {/if}
              </label>

              {#if param.param_type === 'boolean'}
                <select id={param.name} bind:value={parameterValues[param.name]} required={param.required}>
                  <option value="">-- Select --</option>
                  <option value="true">True</option>
                  <option value="false">False</option>
                </select>
              {:else if param.param_type === 'number'}
                <input
                  id={param.name}
                  type="number"
                  bind:value={parameterValues[param.name]}
                  placeholder={param.default_value || 'Enter a number'}
                  required={param.required}
                />
              {:else}
                <input
                  id={param.name}
                  type="text"
                  bind:value={parameterValues[param.name]}
                  placeholder={param.default_value || 'Enter value'}
                  required={param.required}
                />
              {/if}
            </div>
          {/each}
        </div>
      {:else}
        <div class="no-params">This command has no parameters.</div>
      {/if}

      <div class="actions">
        <button type="button" class="btn-cancel" on:click={onClose} disabled={executing}>
          Cancel
        </button>
        <button type="submit" class="btn-execute" disabled={executing}>
          {executing ? 'Executing...' : '▶ Execute Command'}
        </button>
      </div>
    </form>

    {#if executionReport}
      <div class="execution-report">
        <h3>Execution Report</h3>

        <div class="report-summary">
          <div class="summary-item">
            <span class="label">Status:</span>
            <span class="value" class:success={executionReport.failed === 0} class:failure={executionReport.failed > 0}>
              {executionReport.failed === 0 ? 'Success' : 'Failed'}
            </span>
          </div>
          <div class="summary-item">
            <span class="label">Duration:</span>
            <span class="value">{formatDuration(executionReport.total_duration)}</span>
          </div>
          <div class="summary-item">
            <span class="label">Steps:</span>
            <span class="value">
              {executionReport.successful} success, {executionReport.failed} failed, {executionReport.skipped}
              skipped
            </span>
          </div>
        </div>

        <div class="report-details">
          <h4>Command Results:</h4>
          {#each executionReport.results as result}
            <div class="result-item" class:success={result.status === 'success'} class:failed={result.status === 'failed'}>
              <div class="result-header">
                <span class="step-num">Step {result.step}:</span>
                <span class="method">{result.method}</span>
                <span class="status">{result.status}</span>
                <span class="duration">{formatDuration(result.duration)}</span>
              </div>
              {#if result.error}
                <div class="result-error">{result.error}</div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {:else}
    <div class="error-message">{error || 'Command not found'}</div>
  {/if}
</div>

<style>
  .command-executor {
    padding: 1.5rem;
    max-width: 700px;
    margin: 0 auto;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    border-bottom: 2px solid #f0f0f0;
    padding-bottom: 0.75rem;
  }

  .header h2 {
    margin: 0;
    font-size: 1.4rem;
    color: #333;
    font-family: 'Monaco', 'Courier New', monospace;
  }

  .btn-close {
    font-size: 2rem;
    background: transparent;
    border: none;
    cursor: pointer;
    color: #999;
    padding: 0;
    width: 2rem;
    height: 2rem;
    line-height: 1;
  }

  .btn-close:hover {
    color: #333;
  }

  .loading {
    padding: 2rem;
    text-align: center;
    color: #666;
  }

  .command-info {
    margin-bottom: 1.5rem;
  }

  .description {
    color: #666;
    font-size: 0.95rem;
    margin: 0;
  }

  .error-message {
    padding: 0.75rem;
    background: #fee;
    color: #c33;
    border-radius: 4px;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .parameters {
    margin-bottom: 1.5rem;
  }

  .parameters h3 {
    font-size: 1.1rem;
    margin-bottom: 1rem;
    color: #333;
  }

  .param-group {
    margin-bottom: 1rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #333;
    font-size: 0.9rem;
  }

  .required {
    color: #c33;
  }

  input,
  select {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
    font-family: inherit;
  }

  input:focus,
  select:focus {
    outline: none;
    border-color: #007acc;
  }

  .no-params {
    padding: 1rem;
    background: #f9f9f9;
    border: 1px dashed #ddd;
    border-radius: 4px;
    text-align: center;
    color: #666;
    margin-bottom: 1.5rem;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding-top: 1rem;
    border-top: 2px solid #f0f0f0;
  }

  .btn-cancel,
  .btn-execute {
    padding: 0.6rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.95rem;
    font-weight: 500;
  }

  .btn-cancel {
    background: #f0f0f0;
    color: #666;
  }

  .btn-cancel:hover:not(:disabled) {
    background: #e0e0e0;
  }

  .btn-execute {
    background: #28a745;
    color: white;
  }

  .btn-execute:hover:not(:disabled) {
    background: #218838;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .execution-report {
    margin-top: 1.5rem;
    padding-top: 1.5rem;
    border-top: 2px solid #f0f0f0;
  }

  .execution-report h3 {
    font-size: 1.2rem;
    margin-bottom: 1rem;
    color: #333;
  }

  .report-summary {
    background: #f9f9f9;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
  }

  .summary-item {
    display: flex;
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
  }

  .summary-item:last-child {
    margin-bottom: 0;
  }

  .summary-item .label {
    font-weight: 500;
    width: 100px;
    color: #666;
  }

  .summary-item .value {
    color: #333;
  }

  .summary-item .value.success {
    color: #28a745;
    font-weight: 600;
  }

  .summary-item .value.failure {
    color: #dc3545;
    font-weight: 600;
  }

  .report-details h4 {
    font-size: 1rem;
    margin-bottom: 0.75rem;
    color: #333;
  }

  .result-item {
    padding: 0.75rem;
    border-left: 3px solid #ddd;
    margin-bottom: 0.5rem;
    background: #f9f9f9;
    border-radius: 0 4px 4px 0;
  }

  .result-item.success {
    border-left-color: #28a745;
    background: #f8fff9;
  }

  .result-item.failed {
    border-left-color: #dc3545;
    background: #fff8f8;
  }

  .result-header {
    display: flex;
    gap: 0.75rem;
    align-items: center;
    font-size: 0.85rem;
  }

  .step-num {
    font-weight: 600;
    color: #666;
  }

  .method {
    font-family: 'Monaco', 'Courier New', monospace;
    color: #333;
    flex: 1;
  }

  .status {
    padding: 0.2rem 0.5rem;
    border-radius: 3px;
    font-size: 0.8rem;
    font-weight: 500;
    text-transform: uppercase;
  }

  .result-item.success .status {
    background: #d4edda;
    color: #155724;
  }

  .result-item.failed .status {
    background: #f8d7da;
    color: #721c24;
  }

  .duration {
    color: #999;
    font-size: 0.8rem;
  }

  .result-error {
    margin-top: 0.5rem;
    padding: 0.5rem;
    background: #fff;
    border-radius: 3px;
    color: #dc3545;
    font-size: 0.85rem;
    font-family: 'Monaco', 'Courier New', monospace;
  }
</style>
