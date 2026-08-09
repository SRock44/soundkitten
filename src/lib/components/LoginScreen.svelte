<script lang="ts">
  import WindowControls from "./WindowControls.svelte";

  let { onLogin, status }: { onLogin: () => void; status: string } = $props();
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
</style>
