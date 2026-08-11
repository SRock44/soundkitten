<script lang="ts">
  import { api } from "../api";
  import { settings } from "../stores/settings.svelte";
  import Icon from "./Icon.svelte";

  let { onClose }: { onClose: () => void } = $props();

  let currentOverride = $state<string | null>(null);
  let loading = $state(true);
  let input = $state("");
  let status = $state("");
  let busy = $state(false);

  api
    .getClientIdOverride()
    .then((v) => {
      currentOverride = v;
      input = v ?? "";
    })
    .finally(() => (loading = false));

  async function save() {
    const id = input.trim();
    if (!id) return;
    busy = true;
    status = "";
    try {
      await api.setClientIdOverride(id);
      currentOverride = id;
      status = "Saved.";
    } catch (e) {
      status = `Failed to save: ${e}`;
    }
    busy = false;
  }

  async function clear() {
    busy = true;
    status = "";
    try {
      await api.clearClientIdOverride();
      currentOverride = null;
      input = "";
      status = "Cleared -- back to auto-detecting from soundcloud.com.";
    } catch (e) {
      status = `Failed to clear: ${e}`;
    }
    busy = false;
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && onClose()} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="modal" onclick={(e) => e.stopPropagation()}>
    <div class="header">
      <span>Settings</span>
      <button class="close" onclick={onClose} aria-label="Close"><Icon name="close" size={13} /></button>
    </div>
    <div class="body">
      <div class="setting-row">
        <div class="setting-text">
          <h3>Performance mode</h3>
          <p class="hint">
            Turns off the mini player's dancing-cat video and sound-wave animation for a lighter-weight experience.
            Off by default.
          </p>
        </div>
        <button
          class="switch"
          class:on={settings.performanceMode}
          role="switch"
          aria-checked={settings.performanceMode}
          aria-label="Enable performance mode"
          onclick={() => settings.setPerformanceMode(!settings.performanceMode)}
        >
          <span class="switch-knob"></span>
        </button>
      </div>

      <div class="setting-row">
        <div class="setting-text">
          <h3>Home banner</h3>
          <p class="hint">
            Shows an animated scene on the Home page (a running cat, synced to your local time of day and season).
            Off by default -- turn on to replace the plain Home header with the cat animation.
          </p>
        </div>
        <button
          class="switch"
          class:on={settings.showHomeBanner}
          role="switch"
          aria-checked={settings.showHomeBanner}
          aria-label="Enable Home banner"
          onclick={() => settings.setShowHomeBanner(!settings.showHomeBanner)}
        >
          <span class="switch-knob"></span>
        </button>
      </div>

      <hr />

      <h3>Manual client ID override</h3>
      <p class="hint">
        SoundKitten normally reads a client ID automatically from soundcloud.com. If soundcloud.com is
        unreachable (blocked, rate-limited, down) this lets the app keep working by pointing it at a known-good
        ID instead -- it's only used to identify the app to SoundCloud's API, not tied to your account.
      </p>
      {#if loading}
        <p class="hint">Loading...</p>
      {:else}
        <input type="text" placeholder="Paste a client ID" bind:value={input} disabled={busy} />
        <div class="actions">
          <button class="primary" onclick={save} disabled={busy || !input.trim()}>Save</button>
          <button onclick={clear} disabled={busy || !currentOverride}>Clear override</button>
        </div>
        {#if status}<p class="status">{status}</p>{/if}
        <p class="hint small">
          {currentOverride ? "A manual override is currently active." : "No override set -- using auto-detection."}
        </p>
      {/if}
    </div>
  </div>
</div>

<style>
.backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 60;
}

.modal {
  background: var(--bg);
  color: var(--fg);
  width: 26rem;
  max-width: 90vw;
  display: flex;
  flex-direction: column;
  border-radius: 10px;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.8rem 1rem;
  border-bottom: 1px solid var(--border);
  font-weight: 600;
}

.close {
  background: none;
  border: none;
  color: var(--muted);
  cursor: pointer;
  display: flex;
}

.body {
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

h3 {
  margin: 0;
  font-size: 0.95rem;
}

.hint {
  margin: 0;
  font-size: 0.8rem;
  color: var(--muted);
  line-height: 1.4;
}

.hint.small {
  font-size: 0.75rem;
}

input[type="text"] {
  width: 100%;
  box-sizing: border-box;
  padding: 0.5rem 0.6rem;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--search-bg);
  color: var(--fg);
  font: inherit;
  font-size: 0.85rem;
}

.actions {
  display: flex;
  gap: 0.5rem;
}

.actions button {
  padding: 0.4rem 0.9rem;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: none;
  color: var(--fg);
  cursor: pointer;
  font: inherit;
  font-size: 0.85rem;
}

.actions button:disabled {
  opacity: 0.5;
  cursor: default;
}

.actions button.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}

.actions button.primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.actions button:not(.primary):hover:not(:disabled) {
  border-color: var(--accent);
  color: var(--accent);
}

.status {
  margin: 0;
  font-size: 0.8rem;
  color: var(--accent);
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.setting-text {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.switch {
  flex-shrink: 0;
  width: 34px;
  height: 20px;
  padding: 0;
  border: none;
  border-radius: 999px;
  background: var(--border);
  cursor: pointer;
  position: relative;
  transition: background 0.15s ease;
}

.switch.on {
  background: var(--accent);
}

.switch-knob {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #fff;
  transition: transform 0.15s ease;
}

.switch.on .switch-knob {
  transform: translateX(14px);
}

hr {
  border: none;
  border-top: 1px solid var(--border);
  margin: 0.4rem 0;
}
</style>
