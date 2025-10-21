<script lang="ts">
  import { onMount } from 'svelte';
  import { saveCommand, getCommand } from '../lib/tauri';
  import type { CommandConfig, SimpleParameter, SimpleParameterType } from '../lib/types';

  export let commandName: string | null = null; // null for new command
  export let onSave: () => void;
  export let onCancel: () => void;

  let name = '';
  let description = '';
  let script = '';
  let parameters: SimpleParameter[] = [];
  let loading = false;
  let error = '';
  let isEditing = false;

  onMount(async () => {
    if (commandName) {
      isEditing = true;
      await loadCommand(commandName);
    } else {
      // New command - set default CDP script template
      script = JSON.stringify(
        {
          name: 'My Command',
          cdp_commands: [
            {
              method: 'Page.navigate',
              params: {
                url: '{{url}}',
              },
            },
          ],
        },
        null,
        2
      );
    }
  });

  async function loadCommand(cmdName: string) {
    loading = true;
    error = '';
    try {
      const result = await getCommand(cmdName);
      if (result.success && result.data) {
        const cmd = result.data;
        name = cmd.name;
        description = cmd.description;
        script = cmd.script;
        parameters = cmd.parameters;
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

  async function handleSave() {
    error = '';

    // Validation
    if (!name.trim()) {
      error = 'Command name is required';
      return;
    }

    if (!/^[a-z0-9-]+$/.test(name)) {
      error = 'Command name must be kebab-case (lowercase letters, numbers, and dashes only)';
      return;
    }

    if (!description.trim()) {
      error = 'Description is required';
      return;
    }

    if (!script.trim()) {
      error = 'Script is required';
      return;
    }

    // Try to parse script as JSON to validate
    try {
      JSON.parse(script);
    } catch (e) {
      error = 'Script must be valid JSON';
      return;
    }

    loading = true;

    const config: CommandConfig = {
      name: name.trim(),
      description: description.trim(),
      script: script.trim(),
      parameters,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    try {
      const result = await saveCommand(config);
      if (result.success) {
        onSave();
      } else {
        error = result.error || 'Failed to save command';
      }
    } catch (e) {
      error = `Error: ${e}`;
      console.error('Failed to save command:', e);
    } finally {
      loading = false;
    }
  }

  function addParameter() {
    parameters = [
      ...parameters,
      {
        name: '',
        param_type: 'text',
        label: '',
        required: true,
        default_value: '',
      },
    ];
  }

  function removeParameter(index: number) {
    parameters = parameters.filter((_, i) => i !== index);
  }

  function formatScript() {
    try {
      const parsed = JSON.parse(script);
      script = JSON.stringify(parsed, null, 2);
      error = '';
    } catch (e) {
      error = 'Cannot format: Script must be valid JSON';
    }
  }
</script>

<div class="command-editor">
  <div class="header">
    <h2>{isEditing ? 'Edit Command' : 'New Command'}</h2>
    <button class="btn-close" on:click={onCancel}>×</button>
  </div>

  {#if loading && isEditing}
    <div class="loading">Loading command...</div>
  {:else}
    <form on:submit|preventDefault={handleSave}>
      {#if error}
        <div class="error-message">{error}</div>
      {/if}

      <div class="form-group">
        <label for="name">
          Command Name <span class="required">*</span>
          <span class="hint">(kebab-case: my-command)</span>
        </label>
        <input
          id="name"
          type="text"
          bind:value={name}
          placeholder="my-command"
          disabled={isEditing}
          required
        />
      </div>

      <div class="form-group">
        <label for="description">
          Description <span class="required">*</span>
        </label>
        <input
          id="description"
          type="text"
          bind:value={description}
          placeholder="What does this command do?"
          required
        />
      </div>

      <div class="form-group">
        <label for="script">
          CDP Script <span class="required">*</span>
          <button type="button" class="btn-format" on:click={formatScript}>Format JSON</button>
        </label>
        <textarea
          id="script"
          bind:value={script}
          placeholder="Enter CDP script JSON with placeholders"
          rows="12"
          required
        ></textarea>
        <div class="hint">
          Use double braces around parameter names for substitution, e.g. url
        </div>
      </div>

      <div class="form-group">
        <label>
          Parameters
          <button type="button" class="btn-add-param" on:click={addParameter}>+ Add Parameter</button
          >
        </label>

        {#if parameters.length === 0}
          <div class="empty-params">No parameters defined. Click "Add Parameter" to create one.</div>
        {:else}
          <div class="parameters-list">
            {#each parameters as param, i (i)}
              <div class="parameter-item">
                <div class="param-row">
                  <input
                    type="text"
                    bind:value={param.name}
                    placeholder="Parameter name (e.g., url)"
                    class="param-name"
                    required
                  />
                  <select bind:value={param.param_type} class="param-type">
                    <option value="text">Text</option>
                    <option value="number">Number</option>
                    <option value="boolean">Boolean</option>
                  </select>
                  <button
                    type="button"
                    class="btn-remove-param"
                    on:click={() => removeParameter(i)}
                    title="Remove parameter"
                  >
                    ×
                  </button>
                </div>
                <input
                  type="text"
                  bind:value={param.label}
                  placeholder="Label (shown in UI)"
                  class="param-label"
                  required
                />
                <div class="param-options">
                  <label class="checkbox-label">
                    <input type="checkbox" bind:checked={param.required} />
                    Required
                  </label>
                  <input
                    type="text"
                    bind:value={param.default_value}
                    placeholder="Default value (optional)"
                    class="param-default"
                  />
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <div class="form-actions">
        <button type="button" class="btn-cancel" on:click={onCancel} disabled={loading}>
          Cancel
        </button>
        <button type="submit" class="btn-save" disabled={loading}>
          {loading ? 'Saving...' : 'Save Command'}
        </button>
      </div>
    </form>
  {/if}
</div>

<style>
  .command-editor {
    padding: 1.5rem;
    max-width: 800px;
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
    font-size: 1.5rem;
    color: #333;
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

  .error-message {
    padding: 0.75rem;
    background: #fee;
    color: #c33;
    border-radius: 4px;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .form-group {
    margin-bottom: 1.25rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #333;
    font-size: 0.95rem;
  }

  .required {
    color: #c33;
  }

  .hint {
    font-weight: normal;
    color: #999;
    font-size: 0.85rem;
    margin-left: 0.5rem;
  }

  input[type='text'],
  textarea,
  select {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
    font-family: inherit;
  }

  textarea {
    font-family: 'Monaco', 'Courier New', monospace;
    resize: vertical;
  }

  input[type='text']:focus,
  textarea:focus,
  select:focus {
    outline: none;
    border-color: #007acc;
  }

  input:disabled {
    background: #f5f5f5;
    cursor: not-allowed;
  }

  .btn-format {
    float: right;
    padding: 0.25rem 0.75rem;
    background: #f0f0f0;
    border: 1px solid #ddd;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .btn-format:hover {
    background: #e0e0e0;
  }

  .btn-add-param {
    float: right;
    padding: 0.25rem 0.75rem;
    background: #007acc;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .btn-add-param:hover {
    background: #005a9e;
  }

  .empty-params {
    padding: 1rem;
    background: #f9f9f9;
    border: 1px dashed #ddd;
    border-radius: 4px;
    text-align: center;
    color: #666;
    font-size: 0.9rem;
  }

  .parameters-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .parameter-item {
    padding: 0.75rem;
    background: #f9f9f9;
    border: 1px solid #ddd;
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .param-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .param-name {
    flex: 2;
  }

  .param-type {
    flex: 1;
  }

  .btn-remove-param {
    font-size: 1.5rem;
    background: transparent;
    border: none;
    cursor: pointer;
    color: #c33;
    padding: 0;
    width: 2rem;
    height: 2rem;
  }

  .btn-remove-param:hover {
    color: #a11;
  }

  .param-label {
    width: 100%;
  }

  .param-options {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: normal;
    margin-bottom: 0;
  }

  .param-default {
    flex: 1;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 2px solid #f0f0f0;
  }

  .btn-cancel,
  .btn-save {
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

  .btn-save {
    background: #007acc;
    color: white;
  }

  .btn-save:hover:not(:disabled) {
    background: #005a9e;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
