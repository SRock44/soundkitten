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

<div class="titlebar" data-tauri-drag-region>
  <span class="app-name" data-tauri-drag-region>SoundCloud Desktop</span>
  <div class="controls">
    <button class="ctrl minimize" onclick={minimize} aria-label="Minimize">
      <svg width="10" height="10" viewBox="0 0 10 10"><rect x="0" y="4.5" width="10" height="1" fill="currentColor" /></svg>
    </button>
    <button class="ctrl maximize" onclick={toggleMaximize} aria-label="Maximize">
      {#if isMaximized}
        <svg width="10" height="10" viewBox="0 0 10 10">
          <rect x="1.5" y="0" width="7" height="7" fill="none" stroke="currentColor" stroke-width="1" />
          <rect x="0" y="2.5" width="7" height="7" fill="var(--titlebar-bg)" stroke="currentColor" stroke-width="1" />
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
</div>

<style>
.titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 32px;
  background: var(--titlebar-bg);
  color: var(--titlebar-fg);
  flex-shrink: 0;
  user-select: none;
}

.app-name {
  padding-left: 0.75rem;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--titlebar-fg-muted);
  flex: 1;
}

.controls {
  display: flex;
  height: 100%;
}

.ctrl {
  width: 42px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  color: var(--titlebar-fg-muted);
  cursor: pointer;
}

.ctrl:hover {
  background: var(--titlebar-hover);
  color: var(--titlebar-fg);
}

.ctrl.close:hover {
  background: #e81123;
  color: white;
}
</style>
