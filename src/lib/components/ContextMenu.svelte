<script lang="ts">
  type Item = { label: string; onSelect: () => void; danger?: boolean };
  let { x, y, items, onClose }: { x: number; y: number; items: Item[]; onClose: () => void } = $props();

  let menuEl: HTMLDivElement | undefined = $state();
  let adjustedX = $derived(menuEl ? Math.min(x, window.innerWidth - menuEl.getBoundingClientRect().width - 8) : x);
  let adjustedY = $derived(menuEl ? Math.min(y, window.innerHeight - menuEl.getBoundingClientRect().height - 8) : y);
</script>

<!-- oncontextmenu+onclick alone aren't reliable dismiss triggers on this WebView2 setup (see TrackRow.svelte's onRowMouseDown comment) -- onmousedown covers right-clicking elsewhere to switch menus even if oncontextmenu doesn't fire. -->
<svelte:window onclick={onClose} oncontextmenu={onClose} onmousedown={onClose} onkeydown={(e) => e.key === "Escape" && onClose()} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div
  class="menu"
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
