<script lang="ts">
  import type { Profile, Track } from "../types";
  import { isOwnedPlaylist } from "../types";
  import { playlistsStore } from "../stores/playlists.svelte";
  import Icon from "./Icon.svelte";
  import PlaylistNameModal from "./PlaylistNameModal.svelte";

  let { track, me, onClose }: { track: Track; me: Profile | null; onClose: () => void } = $props();

  let ownedPlaylists = $derived(playlistsStore.items.filter((p) => isOwnedPlaylist(p, me)));
  let busyId = $state<number | null>(null);
  let addedId = $state<number | null>(null);
  let error = $state("");
  let showCreate = $state(false);

  async function addTo(playlistId: number) {
    if (busyId !== null) return;
    busyId = playlistId;
    error = "";
    try {
      await playlistsStore.addTrack(playlistId, track);
      addedId = playlistId;
    } catch (e) {
      error = `Failed to add track: ${e}`;
    } finally {
      busyId = null;
    }
  }

  async function createAndAdd(title: string) {
    error = "";
    try {
      const created = await playlistsStore.create(title, [track.id]);
      addedId = created.id;
      showCreate = false;
    } catch (e) {
      error = `Failed to create playlist: ${e}`;
      showCreate = false;
    }
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && onClose()} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="modal" onclick={(e) => e.stopPropagation()}>
    <div class="header">
      <span>Add to playlist</span>
      <button class="close" onclick={onClose} aria-label="Close"><Icon name="close" size={13} /></button>
    </div>
    {#if error}<p class="error-text">{error}</p>{/if}
    <div class="list">
      <button class="row create" onclick={() => (showCreate = true)}>
        <span class="row-icon"><Icon name="plus" size={14} /></span>
        <span>Create new playlist</span>
      </button>
      {#if ownedPlaylists.length === 0}
        <p class="muted">No playlists yet -- create one above.</p>
      {:else}
        {#each ownedPlaylists as p (p.id)}
          <button class="row" onclick={() => addTo(p.id)} disabled={busyId === p.id}>
            {#if p.artwork_url}
              <img src={p.artwork_url} alt="" class="row-artwork" loading="lazy" />
            {:else}
              <span class="row-artwork row-icon"><Icon name="playlists" size={14} /></span>
            {/if}
            <span class="row-title">{p.title ?? "Untitled"}</span>
            {#if addedId === p.id}<Icon name="check" size={14} />{/if}
          </button>
        {/each}
      {/if}
    </div>
  </div>
</div>

{#if showCreate}
  <PlaylistNameModal heading="New playlist" confirmLabel="Create & add" onConfirm={createAndAdd} onClose={() => (showCreate = false)} />
{/if}

<style>
.backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 65;
}

.modal {
  background: var(--bg);
  color: var(--fg);
  width: 22rem;
  max-width: 90vw;
  max-height: 75vh;
  display: flex;
  flex-direction: column;
  border-radius: 10px;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.8rem 1rem;
  border-bottom: 1px solid var(--border);
  font-weight: 600;
}

.close {
  background: none;
  border: none;
  color: var(--muted);
  cursor: pointer;
  display: flex;
}

.error-text {
  color: var(--error-text);
  font-size: 0.82rem;
  padding: 0.5rem 1rem 0;
  margin: 0;
}

.list {
  overflow-y: auto;
  padding: 0.4rem;
}

.muted {
  color: var(--muted);
  padding: 0.5rem 0.6rem;
  font-size: 0.85rem;
}

.row {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  width: 100%;
  padding: 0.5rem 0.6rem;
  background: none;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  text-align: left;
  color: inherit;
  font: inherit;
  font-size: 0.9rem;
}

.row:hover {
  background: var(--row-hover);
}

.row:disabled {
  opacity: 0.6;
  cursor: default;
}

.row.create {
  color: var(--accent);
  font-weight: 600;
  border-bottom: 1px solid var(--border);
  border-radius: 0;
  margin-bottom: 0.3rem;
  padding-bottom: 0.7rem;
}

.row-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.row-artwork {
  width: 30px;
  height: 30px;
  border-radius: 4px;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
}

.row-title {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
