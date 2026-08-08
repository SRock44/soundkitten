<script lang="ts">
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

<div class="login-screen">
  <h1>SoundCloud Desktop</h1>
  <p class="subtitle">An unofficial, open-source, streaming-only client.</p>
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

<style>
.login-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
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
