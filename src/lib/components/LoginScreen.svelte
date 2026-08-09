<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import WindowControls from "./WindowControls.svelte";

  let { onLogin, status }: { onLogin: () => void; status: string } = $props();

  let agreed = $state(false);

  function link(e: MouseEvent, url: string) {
    e.preventDefault();
    e.stopPropagation();
    openUrl(url);
  }
</script>

<div class="page">
  <div class="titlebar" data-tauri-drag-region>
    <WindowControls />
  </div>

  <div class="login-screen">
    <img src="/logo.png" alt="" class="logo" />
    <h1>SoundKitten</h1>
    <p class="subtitle">An unofficial, open-source, streaming-only client for SoundCloud.</p>

    <label class="agree">
      <input type="checkbox" bind:checked={agreed} />
      <span>
        I agree to the <a href="https://soundkitten.org/terms" onclick={(e) => link(e, "https://soundkitten.org/terms")}>Terms</a>
        and <a href="https://soundkitten.org/privacy" onclick={(e) => link(e, "https://soundkitten.org/privacy")}>Privacy Policy</a>.
        SoundKitten isn't affiliated with SoundCloud, and doesn't collect or share your data.
      </span>
    </label>

    <button class="login-btn" onclick={onLogin} disabled={!agreed}>Log in with SoundCloud</button>
    {#if status}
      <p class="status">{status}</p>
    {/if}
    <a class="site-link" href="https://soundkitten.org" onclick={(e) => link(e, "https://soundkitten.org")}>soundkitten.org</a>
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

.login-btn:disabled {
  opacity: 0.4;
  cursor: default;
}

.agree {
  display: flex;
  align-items: flex-start;
  gap: 0.5rem;
  max-width: 22rem;
  margin-top: 0.75rem;
  text-align: left;
  font-size: 0.8rem;
  color: var(--muted);
  cursor: pointer;
}

.agree input {
  margin-top: 0.2rem;
  flex-shrink: 0;
}

.agree a {
  color: inherit;
  text-decoration: underline;
}

.agree a:hover {
  color: var(--accent);
}

.status {
  color: var(--muted);
  max-width: 24rem;
}

.site-link {
  margin-top: 1.5rem;
  color: var(--muted);
  font-size: 0.78rem;
  text-decoration: none;
}

.site-link:hover {
  color: var(--accent);
  text-decoration: underline;
}
</style>
