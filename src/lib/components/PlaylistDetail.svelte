<script lang="ts">
  import { ask } from "@tauri-apps/plugin-dialog";
  import type { Playlist, Profile, Track } from "../types";
  import { isOwnedPlaylist } from "../types";
  import { playlistsStore } from "../stores/playlists.svelte";
  import TrackRow from "./TrackRow.svelte";
  import Icon from "./Icon.svelte";
  import PlaylistNameModal from "./PlaylistNameModal.svelte";

  let {
    playlist,
    me,
    loading,
    loadError,
    onBack,
    onOpenProfile,
    onOpenTrack,
    onUpdated,
    onDeleted,
  }: {
    playlist: Playlist;
    me: Profile | null;
    loading: boolean;
    loadError: string;
    onBack: () => void;
    onOpenProfile: (id: number) => void;
    onOpenTrack: (t: Track) => void;
    onUpdated: (p: Playlist) => void;
    onDeleted: () => void;
  } = $props();

  let owned = $derived(isOwnedPlaylist(playlist, me));
  let showRename = $state(false);
  let deleteBusy = $state(false);
  let actionError = $state("");

  async function rename(title: string) {
    actionError = "";
    try {
      const updated = await playlistsStore.rename(playlist.id, title);
      onUpdated(updated);
      showRename = false;
    } catch (e) {
      actionError = `Failed to rename: ${e}`;
      showRename = false;
    }
  }

  async function confirmDelete() {
    const ok = await ask(`Delete "${playlist.title ?? "this playlist"}"? This can't be undone.`, { title: "Delete playlist", kind: "warning" });
    if (!ok || deleteBusy) return;
    deleteBusy = true;
    try {
      await playlistsStore.remove(playlist.id);
      onDeleted();
    } catch (e) {
      actionError = `Failed to delete: ${e}`;
      deleteBusy = false;
    }
  }

  async function removeTrack(track: Track) {
    actionError = "";
    try {
      const updated = await playlistsStore.removeTrack(playlist.id, track.id);
      onUpdated(updated);
    } catch (e) {
      actionError = `Failed to remove track: ${e}`;
    }
  }
</script>

<button class="back" onclick={onBack}><Icon name="arrow-left" size={14} /> Back</button>

<div class="header">
  <h1>{playlist.title ?? "Untitled playlist"}</h1>
  {#if owned}
    <div class="owner-actions">
      <button class="icon-btn" onclick={() => (showRename = true)} title="Rename" aria-label="Rename"><Icon name="pencil" size={14} /></button>
      <button class="icon-btn danger" onclick={confirmDelete} disabled={deleteBusy} title="Delete" aria-label="Delete"><Icon name="trash" size={14} /></button>
    </div>
  {/if}
</div>

{#if actionError}<p class="error-text">{actionError}</p>{/if}

{#if loading}
  <p class="muted">Loading tracks...</p>
{:else if loadError}
  <p class="error-text">{loadError}</p>
{:else if playlist.tracks.length === 0}
  <p class="muted">This playlist has no tracks.</p>
{:else}
  <div class="list">
    {#each playlist.tracks as t, i}
      <TrackRow track={t} queue={playlist.tracks} index={i} {onOpenProfile} {onOpenTrack} {me} onRemoveFromPlaylist={owned ? removeTrack : undefined} />
    {/each}
  </div>
{/if}

{#if showRename}
  <PlaylistNameModal heading="Rename playlist" confirmLabel="Save" initialValue={playlist.title ?? ""} onConfirm={rename} onClose={() => (showRename = false)} />
{/if}

<style>
.back {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  background: none;
  border: none;
  color: var(--muted);
  cursor: pointer;
  padding: 0;
  margin-bottom: 0.75rem;
  font: inherit;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

.header h1 {
  margin: 0;
}

.owner-actions {
  display: flex;
  gap: 0.4rem;
  flex-shrink: 0;
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: none;
  color: var(--muted);
  cursor: pointer;
}

.icon-btn:hover {
  color: var(--fg);
  border-color: var(--fg);
}

.icon-btn.danger:hover {
  color: var(--error-text);
  border-color: var(--error-text);
}

.icon-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.muted {
  color: var(--muted);
}

.error-text {
  color: var(--error-text);
}

.list {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}
</style>
