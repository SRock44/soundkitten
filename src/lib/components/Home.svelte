<script lang="ts">
  import { api } from "../api";
  import { player } from "../stores/player.svelte";
  import { viewMode } from "../stores/viewMode.svelte";
  import { isSystemPlaylist, timeAgo, type FeedEntry, type Playlist, type Profile, type Selection, type SystemPlaylist, type Track } from "../types";
  import TrackRow from "./TrackRow.svelte";
  import PlaylistShelf from "./PlaylistShelf.svelte";
  import Icon from "./Icon.svelte";
  import { delay, syncWithCache } from "../localCache";
  import { syncStatus } from "../stores/syncStatus.svelte";

  let {
    me,
    likes,
    playlists,
    feed,
    feedLoading,
    feedError,
    onNavigate,
    onOpenProfile,
    onOpenTrack,
    onOpenPlaylist,
    onOpenSystemPlaylist,
  }: {
    me: Profile | null;
    likes: Track[];
    playlists: Playlist[];
    /** Lifted up to +page.svelte (mirrors likes/playlists) so the global refresh button can force-refetch it too -- see ensureFeedLoaded there. */
    feed: FeedEntry[];
    feedLoading: boolean;
    feedError: string;
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

  // The Playlists section only ever holds the user's own real playlists,
  // never system playlists -- narrowed here so it can share PlaylistShelf
  // (which takes the wider Playlist | SystemPlaylist type, since
  // Selections below need it).
  function openOwnPlaylist(p: Playlist | SystemPlaylist) {
    if (!isSystemPlaylist(p)) onOpenPlaylist(p);
  }

  function openReposter(entry: FeedEntry) {
    if (entry.reposted_by) onOpenProfile(entry.reposted_by.id);
  }

  function greeting(): string {
    const hour = new Date().getHours();
    if (hour < 5) return "Good night";
    if (hour < 12) return "Good morning";
    if (hour < 18) return "Good afternoon";
    return "Good evening";
  }

  let feedExpanded = $state(false);
  const FEED_PREVIEW_COUNT = 5;
  let feedTracks = $derived(feed.map((e) => e.track));

  let selections = $state<Selection[]>([]);
  let selectionsLoading = $state(true);

  // Staggered relative to the app-level likes/playlists/me/followings/feed
  // burst on login/Home-nav, which is exactly the kind of concurrent
  // request pile-up that got the account rate-limited previously. A warm
  // cache means most of the time this doesn't touch the network at all.
  (async () => {
    await delay(500);
    await syncWithCache(
      "mixedSelections",
      () => api.mixedSelections(),
      (v) => (selections = v.filter((sel) => sel.items.collection.length > 0)),
      { onRateLimited: () => syncStatus.rateLimited() },
    );
    selectionsLoading = false;
  })();
</script>

<div class="home">
  <div class="hero">
    <div>
      <p class="hero-eyebrow">{greeting()}</p>
      <h1>{me?.username ?? "Welcome back"}</h1>
    </div>
    <div class="view-toggle" role="group" aria-label="Playlist display">
      <button class:active={viewMode.playlistView === "tiles"} onclick={() => viewMode.setPlaylistView("tiles")} aria-label="Tile view" title="Tile view">
        <Icon name="grid" size={15} />
      </button>
      <button class:active={viewMode.playlistView === "rows"} onclick={() => viewMode.setPlaylistView("rows")} aria-label="Row view" title="Row view">
        <Icon name="list" size={15} />
      </button>
    </div>
  </div>

  <section class="module">
    <h2>Feed</h2>
    {#if feedLoading}
      <p class="muted">Loading feed...</p>
    {:else if feedError}
      <p class="muted">{feedError}</p>
    {:else if feed.length === 0}
      <p class="muted">No recent activity from people you follow.</p>
    {:else}
      <div class="list" class:scrollable={feedExpanded}>
        {#each (feedExpanded ? feed : feed.slice(0, FEED_PREVIEW_COUNT)) as entry, i (entry.track.id)}
          <div class="feed-item">
            {#if entry.is_repost && entry.reposted_by}
              <button class="origin" onclick={() => openReposter(entry)}>
                <Icon name="repost" size={11} />
                <span>Reposted by {entry.reposted_by.username ?? "someone"}</span>
                {#if entry.activity_at}<span class="origin-time">· {timeAgo(entry.activity_at)}</span>{/if}
              </button>
            {/if}
            <TrackRow track={entry.track} queue={feedTracks} index={i} {onOpenProfile} {onOpenTrack} {me} />
          </div>
        {/each}
      </div>
      {#if feed.length > FEED_PREVIEW_COUNT}
        <button class="see-all" onclick={() => (feedExpanded = !feedExpanded)}>
          {feedExpanded ? "Show less ↑" : `Show more (${feed.length - FEED_PREVIEW_COUNT}) →`}
        </button>
      {/if}
    {/if}
  </section>

  {#if player.history.length > 0}
    <section class="module">
      <h2>Recently played</h2>
      <div class="list">
        {#each player.history as t, i}<TrackRow track={t} queue={player.history} index={i} {onOpenProfile} {onOpenTrack} {me} />{/each}
      </div>
    </section>
  {/if}

  <section class="module">
    <div class="section-header">
      <h2>Likes</h2>
      <button class="see-all" onclick={() => onNavigate("likes")}>See all →</button>
    </div>
    {#if likes.length === 0}
      <p class="muted">No likes yet.</p>
    {:else}
      <div class="list">
        {#each likes.slice(0, 5) as t, i}<TrackRow track={t} queue={likes} index={i} {onOpenProfile} {onOpenTrack} {me} />{/each}
      </div>
    {/if}
  </section>

  <section class="module featured">
    <div class="section-header">
      <h2>Your playlists</h2>
      <button class="see-all" onclick={() => onNavigate("playlists")}>See all →</button>
    </div>
    {#if playlists.length === 0}
      <p class="muted">No playlists yet.</p>
    {:else}
      <PlaylistShelf items={playlists.slice(0, 10)} onOpen={openOwnPlaylist} />
    {/if}
  </section>

  {#if !selectionsLoading && selections.length > 0}
    {#each selections as sel (sel.urn ?? sel.title)}
      <section class="module">
        <h2>{sel.title ?? "Discover"}</h2>
        <PlaylistShelf items={sel.items.collection} onOpen={openSelectionItem} />
      </section>
    {/each}
  {/if}
</div>

<style>
.home {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.hero {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 1.5rem;
  padding: 2rem 1.75rem;
  border-radius: 20px;
  background: linear-gradient(135deg, rgba(255, 85, 0, 0.16), var(--surface) 65%);
}

.hero-eyebrow {
  margin: 0 0 0.3rem;
  font-size: 0.82rem;
  font-weight: 700;
  color: var(--muted);
  text-transform: uppercase;
  letter-spacing: 0.06em;
}

.hero h1 {
  margin: 0;
  font-size: 2.1rem;
  font-weight: 800;
  letter-spacing: -0.02em;
}

.module {
  background: var(--surface);
  border-radius: 18px;
  padding: 1.5rem 1.5rem 1.75rem;
}

.module.featured {
  background: linear-gradient(160deg, rgba(255, 85, 0, 0.09), var(--surface) 55%);
}

h2 {
  margin: 0 0 1rem;
  font-size: 1.15rem;
  font-weight: 700;
  letter-spacing: -0.005em;
}

.view-toggle {
  display: flex;
  gap: 0.2rem;
  background: var(--surface);
  border-radius: 8px;
  padding: 0.2rem;
  flex-shrink: 0;
}

.view-toggle button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 26px;
  border: none;
  background: none;
  border-radius: 6px;
  color: var(--muted);
  cursor: pointer;
}

.view-toggle button:hover:not(.active) {
  color: var(--fg);
}

.view-toggle button.active {
  background: var(--bg);
  color: var(--fg);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
}

.section-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  margin-bottom: 0.25rem;
}

.section-header h2 {
  margin-bottom: 0;
}

.see-all {
  background: none;
  border: none;
  color: var(--muted);
  font-size: 0.85rem;
  font-weight: 600;
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

.feed-item {
  display: flex;
  flex-direction: column;
}

.origin {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  background: none;
  border: none;
  padding: 0.15rem 0.6rem 0;
  margin: 0;
  color: var(--muted);
  font-size: 0.74rem;
  font-weight: 600;
  cursor: pointer;
  width: fit-content;
}

.origin:hover {
  color: var(--fg);
}

.origin-time {
  font-weight: 400;
}

.muted {
  color: var(--muted);
}
</style>
