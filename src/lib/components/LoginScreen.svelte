<script lang="ts">
  import WindowControls from "./WindowControls.svelte";

  let {
    onLogin,
    status,
    showManualFallback,
    onSubmitManualToken,
  }: {
    onLogin: () => void;
    status: string;
    showManualFallback: boolean;
    onSubmitManualToken: (token: string) => void;
  } = $props();

  let manualToken = $state("");
</script>

<div class="page">
  <div class="titlebar" data-tauri-drag-region>
    <WindowControls />
  </div>

  <div class="login-screen">
    <img src="/logo.png" alt="" class="logo" />
    <h1>SoundKitten</h1>
    <p class="subtitle">An unofficial, open-source, streaming-only client for SoundCloud.</p>
    <button class="login-btn" onclick={onLogin}>Log in with SoundCloud</button>
    {#if status}
      <p class="status">{status}</p>
    {/if}
    {#if showManualFallback}
      <div class="fallback">
        <p>If the login window didn't work, paste your <code>oauth_token</code> cookie value manually:</p>
        <input type="password" placeholder="oauth_token value" bind:value={manualToken} />
        <button onclick={() => onSubmitManualToken(manualToken)}>Save token</button>
      </div>
    {/if}
  </div>
</div>

<style>
.page {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.titlebar {
  display: flex;
  justify-content: flex-end;
  flex-shrink: 0;
}

.logo {
  width: 72px;
  height: 72px;
  object-fit: contain;
  margin-bottom: 0.25rem;
}

.login-screen {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  text-align: center;
  padding: 2rem;
  box-sizing: border-box;
}

.subtitle {
  color: var(--muted);
  margin-top: -0.5rem;
}

.login-btn {
  margin-top: 1rem;
  padding: 0.7em 1.6em;
  border-radius: 999px;
  border: none;
  background: var(--accent);
  color: white;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
}

.login-btn:hover {
  background: var(--accent-hover);
}

.status {
  color: var(--muted);
  max-width: 24rem;
}

.fallback {
  margin-top: 1rem;
  padding: 1rem;
  border: 1px solid var(--border);
  border-radius: 8px;
  max-width: 24rem;
}

.fallback input {
  width: 100%;
  box-sizing: border-box;
  padding: 0.5em;
  margin: 0.5em 0;
}
</style>
