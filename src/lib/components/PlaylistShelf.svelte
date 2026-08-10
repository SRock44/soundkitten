<script lang="ts">
  import { viewMode } from "../stores/viewMode.svelte";
  import { isSystemPlaylist, type Playlist, type SystemPlaylist } from "../types";
  import PlaylistCard from "./PlaylistCard.svelte";
  import PlaylistListRow from "./PlaylistListRow.svelte";
  import Icon from "./Icon.svelte";

  let { items, onOpen }: { items: (Playlist | SystemPlaylist)[]; onOpen: (p: Playlist | SystemPlaylist) => void } = $props();

  let scrollEl: HTMLDivElement | undefined = $state();

  function scroll(direction: 1 | -1) {
    scrollEl?.scrollBy({ left: direction * 460, behavior: "smooth" });
  }

  function keyOf(item: Playlist | SystemPlaylist): string {
    return isSystemPlaylist(item) ? item.id : `p${item.id}`;
  }

  // SoundCloud's own API has been observed returning the same playlist
  // twice (confirmed live: an "each_key_duplicate" crash on a real
  // account's playlist list). A keyed #each throws on duplicate keys --
  // deduping here means a backend quirk degrades to "shown once" instead
  // of crashing this component (and everything above it) entirely.
  let uniqueItems = $derived.by(() => {
    const seen = new Set<string>();
    return items.filter((item) => {
      const key = keyOf(item);
      if (seen.has(key)) return false;
      seen.add(key);
      return true;
    });
  });
</script>

{#if viewMode.playlistView === "tiles"}
  <div class="shelf">
    <div class="shelf-scroll" bind:this={scrollEl}>
      {#each uniqueItems as item (keyOf(item))}
        <PlaylistCard {item} {onOpen} />
      {/each}
    </div>
    <button class="shelf-nav left" onclick={() => scroll(-1)} aria-label="Scroll left"><Icon name="arrow-left" size={16} /></button>
    <button class="shelf-nav right" onclick={() => scroll(1)} aria-label="Scroll right"><Icon name="arrow-left" size={16} /></button>
  </div>
{:else}
  <div class="row-list">
    {#each uniqueItems as item (keyOf(item))}
      <PlaylistListRow {item} {onOpen} />
    {/each}
  </div>
{/if}

<style>
.shelf {
  position: relative;
}

.shelf-scroll {
  display: flex;
  gap: 1.25rem;
  overflow-x: auto;
  scroll-behavior: smooth;
  padding: 0.3rem 0.1rem 0.5rem;
  /* Default scrollbars under a horizontal card rail read as unfinished --
     every reference point for this style of shelf (this app's own vertical
     lists included) relies on drag/wheel + the hover arrows below instead. */
  scrollbar-width: none;
}

.shelf-scroll::-webkit-scrollbar {
  display: none;
}

.shelf-nav {
  position: absolute;
  top: 0;
  bottom: 0.5rem;
  width: 48px;
  display: flex;
  align-items: center;
  border: none;
  color: var(--fg);
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s ease;
}

.shelf-nav.left {
  left: -0.75rem;
  background: linear-gradient(to right, var(--bg) 40%, transparent);
  justify-content: flex-start;
  padding-left: 0.25rem;
}

.shelf-nav.right {
  right: -0.75rem;
  background: linear-gradient(to left, var(--bg) 40%, transparent);
  justify-content: flex-end;
  padding-right: 0.25rem;
}

.shelf-nav.right :global(svg) {
  transform: rotate(180deg);
}

.shelf:hover .shelf-nav {
  opacity: 1;
}

.row-list {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}
</style>
