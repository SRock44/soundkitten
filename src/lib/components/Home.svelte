<script lang="ts">
  import { api } from "../api";
  import { player } from "../stores/player.svelte";
  import { isSystemPlaylist, selectionArtwork, type Playlist, type Profile, type Selection, type SystemPlaylist, type Track } from "../types";
  import TrackRow from "./TrackRow.svelte";
  import Icon from "./Icon.svelte";

  let {
    me,
    likes,
    playlists,
    onNavigate,
    onOpenProfile,
    onOpenTrack,
    onOpenPlaylist,
    onOpenSystemPlaylist,
  }: {
    me: Profile | null;
    likes: Track[];
    playlists: Playlist[];
    onNavigate: (v: "likes" | "playlists") => void;
    onOpenProfile: (id: number) => void;
    onOpenTrack: (t: Track) => void;
    onOpenPlaylist: (p: Playlist) => void;
    onOpenSystemPlaylist: (p: SystemPlaylist) => void;
  } = $props();

  function openSelectionItem(p: Playlist | SystemPlaylist) {
    if (isSystemPlaylist(p)) onOpenSystemPlaylist(p);
    else onOpenPlaylist(p);
  }

  let feed = $state<Track[]>([]);
  let feedLoading = $state(true);
  let feedError = $state("");
  let feedExpanded = $state(false);
  const FEED_PREVIEW_COUNT = 5;

  let selections = $state<Selection[]>([]);
  let selectionsLoading = $state(true);

  api
    .feed()
    .then((f) => (feed = f))
    .catch((e) => (feedError = `Failed to load feed: ${e}`))
    .finally(() => (feedLoading = false));

  api
    .mixedSelections()
    .then((s) => (selections = s.filter((sel) => sel.items.collection.length > 0)))
    .catch((e) => console.error("failed to load mixed selections", e))
    .finally(() => (selectionsLoading = false));
</script>

<div class="home">
  <h1>{me?.username ? `Welcome back, ${me.username}` : "Welcome back"}</h1>

  <section>
    <h2>Feed</h2>
    {#if feedLoading}
      <p class="muted">Loading feed...</p>
    {:else if feedError}
      <p class="muted">{feedError}</p>
    {:else if feed.length === 0}
      <p class="muted">No recent activity from people you follow.</p>
    {:else}
      <div class="list" class:scrollable={feedExpanded}>
        {#each (feedExpanded ? feed : feed.slice(0, FEED_PREVIEW_COUNT)) as t, i}<TrackRow track={t} queue={feed} index={i} {onOpenProfile} {onOpenTrack} />{/each}
      </div>
      {#if feed.length > FEED_PREVIEW_COUNT}
        <button class="see-all" onclick={() => (feedExpanded = !feedExpanded)}>
          {feedExpanded ? "Show less ↑" : `Show more (${feed.length - FEED_PREVIEW_COUNT}) →`}
        </button>
      {/if}
    {/if}
  </section>

  {#if player.history.length > 0}
    <section>
      <h2>Recently played</h2>
      <div class="list">
        {#each player.history as t, i}<TrackRow track={t} queue={player.history} index={i} {onOpenProfile} {onOpenTrack} />{/each}
      </div>
    </section>
  {/if}

  <section>
    <div class="section-header">
      <h2>Likes</h2>
      <button class="see-all" onclick={() => onNavigate("likes")}>See all →</button>
    </div>
    {#if likes.length === 0}
      <p class="muted">No likes yet.</p>
    {:else}
      <div class="list">
        {#each likes.slice(0, 5) as t, i}<TrackRow track={t} queue={likes} index={i} {onOpenProfile} {onOpenTrack} />{/each}
      </div>
    {/if}
  </section>

  <section>
    <div class="section-header">
      <h2>Playlists</h2>
      <button class="see-all" onclick={() => onNavigate("playlists")}>See all →</button>
    </div>
    {#if playlists.length === 0}
      <p class="muted">No playlists yet.</p>
    {:else}
      <div class="playlist-row">
        {#each playlists.slice(0, 6) as p}
          <button class="playlist-card" onclick={() => onOpenPlaylist(p)}>
            {#if p.artwork_url}
              <img src={p.artwork_url} alt="" />
            {:else}
              <div class="playlist-artwork-fallback"><Icon name="queue" size={20} /></div>
            {/if}
            <span class="playlist-title">{p.title ?? "Untitled"}</span>
          </button>
        {/each}
      </div>
    {/if}
  </section>

  {#if !selectionsLoading && selections.length > 0}
    {#each selections as sel}
      <section>
        <h2>{sel.title ?? "Discover"}</h2>
        <div class="playlist-row">
          {#each sel.items.collection as p}
            <button class="playlist-card" onclick={() => openSelectionItem(p)}>
              {#if selectionArtwork(p)}
                <img src={selectionArtwork(p)} alt="" />
              {:else}
                <div class="playlist-artwork-fallback"><Icon name="queue" size={20} /></div>
              {/if}
              <span class="playlist-title">{p.title ?? "Untitled"}</span>
            </button>
          {/each}
        </div>
      </section>
    {/each}
  {/if}
</div>

<style>
.home {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

h1 {
  margin: 0;
  font-size: 1.4rem;
}

h2 {
  margin: 0 0 0.75rem;
  font-size: 1.05rem;
}

.section-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
}

.see-all {
  background: none;
  border: none;
  color: var(--muted);
  font-size: 0.85rem;
  cursor: pointer;
}

.see-all:hover {
  color: var(--accent);
}

.list {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.list.scrollable {
  max-height: 22rem;
  overflow-y: auto;
}

.muted {
  color: var(--muted);
}

.playlist-row {
  display: flex;
  gap: 1.1rem;
  overflow-x: auto;
  padding-bottom: 0.25rem;
}

.playlist-card {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  flex: 0 0 140px;
  width: 140px;
  background: none;
  border: none;
  padding: 0;
  text-align: left;
  color: inherit;
  font: inherit;
  cursor: pointer;
}

.playlist-card img,
.playlist-artwork-fallback {
  width: 100%;
  aspect-ratio: 1;
  border-radius: 6px;
  object-fit: cover;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.playlist-title {
  font-weight: 600;
  font-size: 0.88rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
