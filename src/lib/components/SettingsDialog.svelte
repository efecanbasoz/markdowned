<script lang="ts">
  import { loadConfig, saveConfig } from "$lib/commands/config";
  import type { Provider, AppConfig } from "$lib/types";

  let { visible = $bindable(false) }: { visible: boolean } = $props();

  let provider = $state<Provider>("ollama");
  let apiKey = $state("");
  let model = $state("");
  let baseUrl = $state("");
  let customBaseUrl = $state("");
  let customModel = $state("");

  const defaults: Record<Provider, string> = {
    ollama: "llama3.2",
    google: "gemini-2.0-flash",
    openai: "gpt-4o-mini",
    anthropic: "claude-haiku-3-5",
    custom: "",
  };

  $effect(() => {
    if (visible) {
      loadConfig().then((config) => {
        provider = config.completion.provider;
        apiKey = config.completion.apiKey;
        model = config.completion.model;
        baseUrl = config.completion.baseUrl;
        customBaseUrl = config.completion.custom.baseUrl;
        customModel = config.completion.custom.model;
      });
    }
  });

  async function handleSave() {
    const config: AppConfig = {
      completion: {
        provider,
        apiKey,
        model,
        baseUrl,
        custom: { baseUrl: customBaseUrl, model: customModel },
      },
      lastWorkspace: null,
    };
    await saveConfig(config);
    visible = false;
  }

  function handleCancel() {
    visible = false;
  }

  function handleOverlayKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      handleCancel();
    }
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="overlay" onclick={handleCancel} onkeydown={handleOverlayKeydown}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="dialog" onclick={(e) => e.stopPropagation()}>
      <h2 class="dialog-title">Settings</h2>

      <div class="field">
        <label class="field-label" for="provider-select">Provider</label>
        <select id="provider-select" class="field-input" bind:value={provider}>
          <option value="ollama">Ollama</option>
          <option value="google">Google</option>
          <option value="openai">OpenAI</option>
          <option value="anthropic">Anthropic</option>
          <option value="custom">Custom</option>
        </select>
      </div>

      {#if provider !== "ollama"}
        <div class="field">
          <label class="field-label" for="api-key-input">API Key</label>
          <input
            id="api-key-input"
            class="field-input"
            type="password"
            bind:value={apiKey}
            placeholder="Enter API key"
          />
        </div>
      {/if}

      <div class="field">
        <label class="field-label" for="model-input">Model</label>
        <input
          id="model-input"
          class="field-input"
          type="text"
          bind:value={model}
          placeholder={defaults[provider]}
        />
      </div>

      {#if provider === "custom"}
        <div class="field">
          <label class="field-label" for="base-url-input">Base URL</label>
          <input
            id="base-url-input"
            class="field-input"
            type="text"
            bind:value={customBaseUrl}
            placeholder="https://api.example.com/v1"
          />
        </div>
      {/if}

      <div class="dialog-actions">
        <button class="btn btn-cancel" onclick={handleCancel}>Cancel</button>
        <button class="btn btn-save" onclick={handleSave}>Save</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    padding-top: 80px;
    z-index: 100;
  }

  .dialog {
    width: 460px;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
    padding: 24px;
    align-self: flex-start;
  }

  .dialog-title {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 20px 0;
  }

  .field {
    margin-bottom: 16px;
  }

  .field-label {
    display: block;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-dimmed);
    margin-bottom: 6px;
  }

  .field-input {
    width: 100%;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 8px 12px;
    color: var(--text-primary);
    font-size: 13px;
    font-family: inherit;
    outline: none;
    box-sizing: border-box;
  }

  .field-input:focus {
    border-color: var(--accent);
  }

  .field-input::placeholder {
    color: var(--text-dimmed);
  }

  select.field-input {
    appearance: none;
    cursor: pointer;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%236b7280' stroke-width='2'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 10px center;
    padding-right: 32px;
  }

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 24px;
  }

  .btn {
    padding: 8px 18px;
    border-radius: 6px;
    font-size: 13px;
    font-family: inherit;
    font-weight: 500;
    cursor: pointer;
    border: none;
  }

  .btn-cancel {
    background: transparent;
    color: var(--text-muted);
    border: 1px solid var(--border);
  }

  .btn-cancel:hover {
    background: rgba(255, 255, 255, 0.04);
  }

  .btn-save {
    background: var(--accent);
    color: white;
  }

  .btn-save:hover {
    background: #3b82f6;
  }
</style>
