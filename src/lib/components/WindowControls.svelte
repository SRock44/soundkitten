<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";

  const win = getCurrentWindow();
  let isMaximized = $state(false);

  win.isMaximized().then((v) => (isMaximized = v));
  win.onResized(() => {
    win.isMaximized().then((v) => (isMaximized = v));
  });

  function minimize() {
    win.minimize();
  }

  function toggleMaximize() {
    win.toggleMaximize();
  }

  function close() {
    win.close();
  }
</script>

<div class="controls">
  <button class="ctrl minimize" onclick={minimize} aria-label="Minimize">
    <svg width="10" height="10" viewBox="0 0 10 10"><rect x="0" y="4.5" width="10" height="1" fill="currentColor" /></svg>
  </button>
  <button class="ctrl maximize" onclick={toggleMaximize} aria-label="Maximize">
    {#if isMaximized}
      <svg width="10" height="10" viewBox="0 0 10 10">
        <rect x="1.5" y="0" width="7" height="7" fill="none" stroke="currentColor" stroke-width="1" />
        <rect x="0" y="2.5" width="7" height="7" fill="var(--nav-bg)" stroke="currentColor" stroke-width="1" />
      </svg>
    {:else}
      <svg width="10" height="10" viewBox="0 0 10 10"><rect x="0.5" y="0.5" width="9" height="9" fill="none" stroke="currentColor" stroke-width="1" /></svg>
    {/if}
  </button>
  <button class="ctrl close" onclick={close} aria-label="Close">
    <svg width="10" height="10" viewBox="0 0 10 10">
      <line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" stroke-width="1" />
      <line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" stroke-width="1" />
    </svg>
  </button>
</div>

<style>
.controls {
  display: flex;
  align-items: center;
  height: 100%;
  -webkit-app-region: no-drag;
}

.ctrl {
  width: 38px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  color: var(--muted);
  cursor: pointer;
}

.ctrl:hover {
  background: var(--row-hover);
  color: var(--fg);
}

.ctrl.close:hover {
  background: #e81123;
  color: white;
}
</style>
