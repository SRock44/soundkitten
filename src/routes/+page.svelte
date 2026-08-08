<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  type AuthEvent = { ok: boolean; error: string | null };

  let loggedIn = $state(false);
  let status = $state("");
  let manualToken = $state("");
  let showManualFallback = $state(false);

  async function refreshStatus() {
    loggedIn = await invoke<boolean>("is_logged_in");
  }

  async function login() {
    status = "Opening SoundCloud login...";
    showManualFallback = false;
    try {
      await invoke("start_login");
    } catch (e) {
      status = `Failed to open login window: ${e}`;
      showManualFallback = true;
    }
  }

  async function verify() {
    status = "Verifying...";
    try {
      const ok = await invoke<boolean>("verify_auth");
      status = ok ? "Verified: authenticated request succeeded." : "Not logged in.";
      loggedIn = ok;
    } catch (e) {
      status = `Verification failed: ${e}`;
      showManualFallback = true;
    }
  }

  async function logout() {
    await invoke("logout");
    await refreshStatus();
    status = "Logged out.";
  }

  async function submitManualToken() {
    try {
      await invoke("set_manual_token", { token: manualToken });
      manualToken = "";
      showManualFallback = false;
      await refreshStatus();
      status = "Manual token saved.";
    } catch (e) {
      status = `Failed to save token: ${e}`;
    }
  }

  refreshStatus();
  listen<AuthEvent>("auth:result", async (event) => {
    if (event.payload.ok) {
      status = "Login successful.";
      showManualFallback = false;
    } else {
      status = `Login failed: ${event.payload.error ?? "unknown error"}`;
      showManualFallback = true;
    }
    await refreshStatus();
  });
</script>

<main class="container">
  <h1>SoundCloud Desktop</h1>

  <p class="status">Status: {loggedIn ? "Logged in" : "Not logged in"}</p>

  <div class="row">
    <button onclick={login}>Log in with SoundCloud</button>
    <button onclick={verify}>Verify auth</button>
    <button onclick={logout}>Log out</button>
  </div>

  {#if status}
    <p class="status-msg">{status}</p>
  {/if}

  {#if showManualFallback}
    <div class="fallback">
      <p>
        If the login window didn't work, paste your <code>oauth_token</code> cookie value from soundcloud.com manually:
      </p>
      <input type="password" placeholder="oauth_token value" bind:value={manualToken} />
      <button onclick={submitManualToken}>Save token</button>
    </div>
  {/if}
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  color: #0f0f0f;
  background-color: #f6f6f6;
}

.container {
  margin: 0;
  padding: 4rem 2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 1rem;
}

.row {
  display: flex;
  gap: 0.5rem;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 0.95em;
  font-weight: 500;
  cursor: pointer;
  background-color: #ff5500;
  color: white;
}

button:hover {
  background-color: #e04c00;
}

.status {
  font-weight: 600;
}

.status-msg {
  max-width: 32rem;
  color: #555;
}

.fallback {
  margin-top: 1rem;
  padding: 1rem;
  border: 1px solid #ddd;
  border-radius: 8px;
  max-width: 28rem;
}

.fallback input {
  width: 100%;
  padding: 0.5em;
  margin: 0.5em 0;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
  .status-msg {
    color: #bbb;
  }
  .fallback {
    border-color: #444;
  }
}
</style>
