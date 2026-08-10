<script lang="ts">
  import { api } from "../api";
  import { bannerUrl, handleOf, timeAgo } from "../types";
  import type { Playlist, Profile, Track, UserComment } from "../types";
  import TrackRow from "./TrackRow.svelte";
  import PlaylistCard from "./PlaylistCard.svelte";
  import PlaylistListRow from "./PlaylistListRow.svelte";
  import UserListModal from "./UserListModal.svelte";
  import ShareButton from "./ShareButton.svelte";
  import FollowButton from "./FollowButton.svelte";
  import { viewMode } from "../stores/viewMode.svelte";

  let {
    userId,
    onOpenProfile,
    onOpenTrack,
    onOpenPlaylist,
    isOwnProfile = false,
    me = null,
  }: {
    userId: number;
    onOpenProfile: (id: number) => void;
    onOpenTrack: (t: Track) => void;
    onOpenPlaylist: (p: Playlist) => void;
    isOwnProfile?: boolean;
    /** Logged-in user, threaded to TrackRow's context menu only for the "Add to playlist" gate -- unrelated to the profile being viewed. */
    me?: Profile | null;
  } = $props();

  type Tab = "all" | "tracks" | "reposts" | "playlists";

  let profile = $state<Profile | null>(null);
  let tracks = $state<Track[]>([]);
  let reposts = $state<Track[]>([]);
  let playlists = $state<Playlist[]>([]);
  let followings = $state<Profile[]>([]);
  let comments = $state<UserComment[]>([]);
  let tab = $state<Tab>("all");
  let loading = $state(true);
  let error = $state("");

  let listModal = $state<"followers" | "following" | null>(null);
  let followers = $state<Profile[]>([]);
  let followersLoading = $state(false);

  // Chronological, newest-first, merged tracks+reposts -- mirrors the
  // website's default profile view. Dedupes by id (a track can appear in
  // both lists if the owner reposted their own track).
  // Also guards the keyed #each below against SoundCloud returning the same
  // playlist twice (confirmed live elsewhere in the app -- see
  // PlaylistShelf.svelte -- a keyed #each throws on duplicate keys).
  let uniquePlaylists = $derived.by(() => {
    const seen = new Set<number>();
    return playlists.filter((p) => (seen.has(p.id) ? false : (seen.add(p.id), true)));
  });

  let all = $derived.by(() => {
    const seen = new Set<number>();
    return [...tracks, ...reposts]
      .filter((t) => (seen.has(t.id) ? false : (seen.add(t.id), true)))
      .sort((a, b) => new Date(b.created_at ?? 0).getTime() - new Date(a.created_at ?? 0).getTime());
  });

  $effect(() => {
    loading = true;
    error = "";
    profile = null;
    tracks = [];
    reposts = [];
    playlists = [];
    followings = [];
    comments = [];
    followers = [];
    listModal = null;
    tab = "all";
    api
      .userProfile(userId)
      .then((p) => (profile = p))
      .catch((e) => (error = `Failed to load profile: ${e}`))
      .finally(() => (loading = false));
    api.userTracks(userId).then((t) => (tracks = t)).catch((e) => console.error("failed to load user tracks", e));
    api.userReposts(userId).then((t) => (reposts = t)).catch((e) => console.error("failed to load user reposts", e));
    api.userPlaylists(userId).then((p) => (playlists = p)).catch((e) => console.error("failed to load user playlists", e));
    api.userFollowings(userId).then((f) => (followings = f)).catch((e) => console.error("failed to load followings", e));
    api.userComments(userId).then((c) => (comments = c)).catch((e) => console.error("failed to load comments", e));
  });

  function openFollowers() {
    listModal = "followers";
    if (followers.length === 0) {
      followersLoading = true;
      api
        .userFollowers(userId)
        .then((f) => (followers = f))
        .catch((e) => console.error("failed to load followers", e))
        .finally(() => (followersLoading = false));
    }
  }
</script>

{#if loading}
  <p class="muted">Loading profile...</p>
{:else if error}
  <p class="error-text">{error}</p>
{:else if profile}
  {#if bannerUrl(profile)}
    <div class="banner" style="background-image: url('{bannerUrl(profile)}')"></div>
  {/if}

  <div class="profile-header" class:with-banner={!!bannerUrl(profile)}>
    {#if profile.avatar_url}
      <img src={profile.avatar_url} alt="" class="avatar" />
    {:else}
      <div class="avatar avatar-fallback">{(profile.username ?? "?")[0]?.toUpperCase()}</div>
    {/if}
    <div class="details">
      <h1>
        {profile.full_name || profile.username || `User #${profile.id}`}
        {#if isOwnProfile}<span class="you-badge">You</span>{/if}
      </h1>
      <div class="subline">
        {#if handleOf(profile)}<span>@{handleOf(profile)}</span>{/if}
        {#if profile.city || profile.country_code}
          <span>{[profile.city, profile.country_code].filter(Boolean).join(", ")}</span>
        {/if}
      </div>
      {#if profile.description}
        <p class="bio">{profile.description}</p>
      {/if}
      <div class="stats">
        <span><strong>{(profile.track_count ?? tracks.length).toLocaleString()}</strong> tracks</span>
        <button class="stat-btn" onclick={openFollowers}><strong>{(profile.followers_count ?? 0).toLocaleString()}</strong> followers</button>
        <button class="stat-btn" onclick={() => (listModal = "following")}><strong>{(profile.followings_count ?? 0).toLocaleString()}</strong> following</button>
      </div>
    </div>
    <div class="header-actions">
      {#if !isOwnProfile}
        <FollowButton userId={profile.id} permalinkUrl={profile.permalink_url} />
      {/if}
      <ShareButton url={profile.permalink_url} />
    </div>
  </div>

  <div class="profile-layout">
    <div class="main-col">
      <div class="tabs">
        <button class:active={tab === "all"} onclick={() => (tab = "all")}>All</button>
        <button class:active={tab === "tracks"} onclick={() => (tab = "tracks")}>Tracks ({tracks.length})</button>
        <button class:active={tab === "reposts"} onclick={() => (tab = "reposts")}>Reposts ({reposts.length})</button>
        <button class:active={tab === "playlists"} onclick={() => (tab = "playlists")}>Playlists ({playlists.length})</button>
      </div>

      {#if tab === "all"}
        {#if all.length === 0}
          <p class="muted">Nothing here yet.</p>
        {:else}
          <div class="list">
            {#each all as t, i}<TrackRow track={t} queue={all} index={i} {onOpenProfile} {onOpenTrack} {me} />{/each}
          </div>
        {/if}
      {:else if tab === "tracks"}
        {#if tracks.length === 0}
          <p class="muted">No public tracks.</p>
        {:else}
          <div class="list">
            {#each tracks as t, i}<TrackRow track={t} queue={tracks} index={i} {onOpenProfile} {onOpenTrack} {me} />{/each}
          </div>
        {/if}
      {:else if tab === "reposts"}
        {#if reposts.length === 0}
          <p class="muted">No reposts.</p>
        {:else}
          <div class="list">
            {#each reposts as t, i}<TrackRow track={t} queue={reposts} index={i} {onOpenProfile} {onOpenTrack} {me} />{/each}
          </div>
        {/if}
      {:else if tab === "playlists"}
        {#if uniquePlaylists.length === 0}
          <p class="muted">No public playlists.</p>
        {:else if viewMode.playlistView === "tiles"}
          <div class="playlist-grid">
            {#each uniquePlaylists as p (p.id)}
              <PlaylistCard item={p} onOpen={() => onOpenPlaylist(p)} />
            {/each}
          </div>
        {:else}
          <div class="list">
            {#each uniquePlaylists as p (p.id)}
              <PlaylistListRow item={p} onOpen={() => onOpenPlaylist(p)} />
            {/each}
          </div>
        {/if}
      {/if}
    </div>

    <aside class="sidebar">
      <section>
        <div class="sidebar-header">
          <h2>Following</h2>
          {#if followings.length > 0}<button class="view-all" onclick={() => (listModal = "following")}>View all</button>{/if}
        </div>
        {#if followings.length === 0}
          <p class="muted small">Not following anyone.</p>
        {:else}
          <div class="mini-list">
            {#each followings.slice(0, 5) as f}
              <button class="mini-row" onclick={() => onOpenProfile(f.id)}>
                {#if f.avatar_url}
                  <img src={f.avatar_url} alt="" class="mini-avatar" loading="lazy" />
                {:else}
                  <div class="mini-avatar avatar-fallback">{(f.username ?? "?")[0]?.toUpperCase()}</div>
                {/if}
                <span class="mini-name">{f.username ?? `User #${f.id}`}</span>
              </button>
            {/each}
          </div>
        {/if}
      </section>

      <section>
        <div class="sidebar-header">
          <h2>Latest comments</h2>
        </div>
        {#if comments.length === 0}
          <p class="muted small">No comments yet.</p>
        {:else}
          <div class="comment-list">
            {#each comments as c}
              <div class="comment-item">
                <div class="comment-meta">
                  <span class="comment-on">on <strong>{c.track_title ?? `Track #${c.track_id}`}</strong></span>
                  <span class="comment-time">{timeAgo(c.created_at)}</span>
                </div>
                <p class="comment-body">"{c.body}"</p>
              </div>
            {/each}
          </div>
        {/if}
      </section>
    </aside>
  </div>

  {#if listModal === "followers"}
    <UserListModal
      title="Followers"
      users={followers}
      loading={followersLoading}
      onClose={() => (listModal = null)}
      {onOpenProfile}
    />
  {:else if listModal === "following"}
    <UserListModal title="Following" users={followings} loading={false} onClose={() => (listModal = null)} {onOpenProfile} />
  {/if}
{/if}

<style>
.banner {
  height: 140px;
  margin: -1.5rem -2rem 0;
  background-size: cover;
  background-position: center;
  background-color: var(--artwork-bg);
}

.profile-header {
  display: flex;
  gap: 1.25rem;
  align-items: flex-start;
  margin-bottom: 1.25rem;
}

.header-actions {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 0.5rem;
  margin-left: auto;
  flex-shrink: 0;
}

.profile-header.with-banner {
  margin-top: -2.5rem;
}

.profile-header.with-banner .avatar {
  border: 3px solid var(--bg);
}

.avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
}

.avatar-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.7rem;
  color: var(--muted);
}

.details {
  min-width: 0;
}

.profile-header.with-banner .details {
  padding-top: 2.5rem;
}

.details h1 {
  margin: 0;
  font-size: 1.25rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.you-badge {
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--accent);
  border: 1px solid var(--accent);
  border-radius: 999px;
  padding: 0.1rem 0.5rem;
}

.subline {
  display: flex;
  gap: 0.75rem;
  color: var(--muted);
  font-size: 0.85rem;
  margin: 0.15rem 0;
}

.bio {
  font-size: 0.88rem;
  max-width: 40rem;
  white-space: pre-wrap;
  margin: 0.4rem 0;
}

.stats {
  display: flex;
  gap: 1.25rem;
  margin-top: 0.4rem;
  font-size: 0.85rem;
  color: var(--muted);
}

.stats strong {
  color: var(--fg);
}

.stat-btn {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  color: var(--muted);
  font: inherit;
}

.stat-btn:hover {
  text-decoration: underline;
}

.profile-layout {
  display: grid;
  grid-template-columns: 1fr 260px;
  gap: 2rem;
  align-items: start;
}

.main-col {
  min-width: 0;
}

.tabs {
  display: flex;
  gap: 0.5rem;
  border-bottom: 1px solid var(--border);
  margin-bottom: 1rem;
}

.tabs button {
  background: none;
  border: none;
  font: inherit;
  font-weight: 600;
  font-size: 0.88rem;
  padding: 0.5rem 0.25rem;
  margin-right: 1rem;
  color: var(--muted);
  cursor: pointer;
  border-bottom: 2px solid transparent;
}

.tabs button.active {
  color: var(--fg);
  border-bottom-color: var(--accent);
}

.muted {
  color: var(--muted);
}

.muted.small {
  font-size: 0.82rem;
}

.error-text {
  color: var(--error-text);
}

.list {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.playlist-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 1.5rem;
}

.sidebar {
  display: flex;
  flex-direction: column;
  gap: 1.75rem;
  padding-top: 0.4rem;
}

.sidebar-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.sidebar-header h2 {
  margin: 0;
  font-size: 0.85rem;
  text-transform: uppercase;
  letter-spacing: 0.03em;
  color: var(--muted);
}

.view-all {
  background: none;
  border: none;
  color: var(--muted);
  font-size: 0.78rem;
  cursor: pointer;
}

.view-all:hover {
  color: var(--accent);
}

.mini-list {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.mini-row {
  display: flex;
  align-items: center;
  gap: 0.55rem;
  width: 100%;
  padding: 0.35rem 0.3rem;
  background: none;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  text-align: left;
  color: inherit;
  font: inherit;
}

.mini-row:hover {
  background: var(--row-hover);
}

.mini-avatar {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
  font-size: 0.75rem;
}

.mini-name {
  font-size: 0.85rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.comment-list {
  display: flex;
  flex-direction: column;
  gap: 0.9rem;
}

.comment-item {
  font-size: 0.85rem;
}

.comment-meta {
  display: flex;
  justify-content: space-between;
  gap: 0.5rem;
  color: var(--muted);
  font-size: 0.78rem;
  margin-bottom: 0.2rem;
}

.comment-meta strong {
  color: var(--fg);
  font-weight: 600;
}

.comment-body {
  margin: 0;
  overflow-wrap: break-word;
}
</style>
