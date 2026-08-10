<script lang="ts">
  import { settings } from "../stores/settings.svelte";

  type Item = { label: string; onSelect: () => void; danger?: boolean };
  let { x, y, items, onClose }: { x: number; y: number; items: Item[]; onClose: () => void } = $props();

  let menuEl: HTMLDivElement | undefined = $state();
  let adjustedX = $derived(menuEl ? Math.min(x, window.innerWidth - menuEl.getBoundingClientRect().width - 8) : x);
  let adjustedY = $derived(menuEl ? Math.min(y, window.innerHeight - menuEl.getBoundingClientRect().height - 8) : y);
</script>

<!-- Dismiss on left-click-elsewhere or Escape only. A single right-click
     fires BOTH mousedown and contextmenu (TrackRow.svelte opens on
     either) -- wiring those same two events here as dismiss triggers
     meant the second of the pair, once this menu was already mounted
     from the first, immediately closed it again within the same click,
     before it could ever be seen. Confirmed live via debug logging: the
     menu was opening and self-closing every single time, never once
     staying open. -->
<svelte:window onclick={onClose} onkeydown={(e) => e.key === "Escape" && onClose()} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div
  class="menu"
  class:performance={settings.performanceMode}
  bind:this={menuEl}
  style="left: {adjustedX}px; top: {adjustedY}px"
  onclick={(e) => e.stopPropagation()}
  onmousedown={(e) => e.stopPropagation()}
>
  {#each items as item}
    <button class:danger={item.danger} onclick={() => { item.onSelect(); onClose(); }}>{item.label}</button>
  {/each}
</div>

<style>
.menu {
  position: fixed;
  z-index: 100;
  background: var(--nav-bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.25);
  padding: 0.35rem;
  min-width: 11rem;
  display: flex;
  flex-direction: column;
  transform-origin: top left;
  animation: menu-pop-in 0.12s cubic-bezier(0.2, 0.8, 0.2, 1);
}

.menu.performance {
  animation: none;
}

@keyframes menu-pop-in {
  from {
    opacity: 0;
    transform: scale(0.94) translateY(-4px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.menu button {
  background: none;
  border: none;
  text-align: left;
  padding: 0.45rem 0.6rem;
  border-radius: 5px;
  cursor: pointer;
  color: inherit;
  font: inherit;
  font-size: 0.88rem;
}

.menu button:hover {
  background: var(--row-hover);
}

.menu button.danger {
  color: #d94848;
}
</style>
