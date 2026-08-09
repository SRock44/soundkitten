<script lang="ts">
  import "@fontsource-variable/archivo";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { api } from "$lib/api";
  import LoginScreen from "$lib/components/LoginScreen.svelte";
  import TopNav from "$lib/components/TopNav.svelte";
  import PlayerBar from "$lib/components/PlayerBar.svelte";
  import TrackRow from "$lib/components/TrackRow.svelte";
  import ProfileView from "$lib/components/Profile.svelte";
  import Home from "$lib/components/Home.svelte";
  import TrackDetail from "$lib/components/TrackDetail.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import SyncStatusBar from "$lib/components/SyncStatusBar.svelte";
  import { likes as likesStore } from "$lib/stores/likes.svelte";
  import { following as followingStore } from "$lib/stores/following.svelte";
  import { officialAuth } from "$lib/stores/officialAuth.svelte";
  import { player } from "$lib/stores/player.svelte";
  import { syncStatus } from "$lib/stores/syncStatus.svelte";
  import { checkForUpdates } from "$lib/updater";
  import { clearCached, delay, loadCached, saveCached, syncWithCache } from "$lib/localCache";
  import type { Playlist, Profile, SystemPlaylist, Track } from "$lib/types";

  type AuthEvent = { ok: boolean; error: string | null };
  type View = "home" | "search" | "likes" | "playlists" | "profile";

  let loggedIn = $state(false);
  let authChecked = $state(false);
  let authStatus = $state("");
  let me = $state<Profile | null>(null);

  let view = $state<View>("home");
  let searchQuery = $state("");
  let searchTab = $state<"tracks" | "people">("tracks");
  let searchResults = $state<Track[]>([]);
  let peopleResults = $state<Profile[]>([]);
  let likes = $state<Track[]>([]);
  let playlists = $state<Playlist[]>([]);
  let openPlaylist = $state<Playlist | null>(null);
  let openPlaylistLoading = $state(false);
  let profileUserId = $state<number | null>(null);
  let selectedTrack = $state<Track | null>(null);
  let loading = $state(false);
  let loadError = $state("");

  /**
   * A universal back button needs to remember every screen the user has
   * visited (not just "playlist detail -> wherever you opened it from",
   * which is all the old playlistReturnView tracked), so this snapshots the
   * handful of state vars that together define "what's on screen" before
   * each navigation, and goBack() restores the most recent one.
   */
  type NavSnapshot = { view: View; selectedTrack: Track | null; profileUserId: number | null; openPlaylist: Playlist | null };
  let navHistory = $state<NavSnapshot[]>([]);
  let canGoBack = $derived(navHistory.length > 0);

  function pushHistory() {
    navHistory = [...navHistory, { view, selectedTrack, profileUserId, openPlaylist }];
  }

  function goBack() {
    if (navHistory.length === 0) return;
    const prev = navHistory[navHistory.length - 1];
    navHistory = navHistory.slice(0, -1);
    view = prev.view;
    selectedTrack = prev.selectedTrack;
    profileUserId = prev.profileUserId;
    openPlaylist = prev.openPlaylist;
  }

  function openTrack(t: Track) {
    pushHistory();
    selectedTrack = t;
  }

  async function viewPlaylist(p: Playlist) {
    pushHistory();
    view = "playlists";
    openPlaylist = p;
    loadError = "";
    if (p.tracks.length > 0) return; // already hydrated
    openPlaylistLoading = true;
    try {
      const full = await api.playlist(p.id);
      // guard against the user navigating to a different playlist while this was in flight
      if (openPlaylist?.id === p.id) openPlaylist = full;
    } catch (e) {
      loadError = `Failed to load playlist tracks: ${e}`;
    }
    openPlaylistLoading = false;
  }

  async function viewSystemPlaylist(sp: SystemPlaylist) {
    pushHistory();
    view = "playlists";
    const shell: Playlist = {
      id: -1,
      title: sp.title,
      permalink_url: sp.permalink_url,
      artwork_url: sp.artwork_url ?? sp.calculated_artwork_url,
      track_count: sp.tracks.length,
      tracks: [],
    };
    openPlaylist = shell;
    loadError = "";
    openPlaylistLoading = true;
    try {
      const tracks = await api.systemPlaylistTracks(sp.tracks.map((t) => t.id));
      // $state wraps `shell` in a proxy on assignment, so `openPlaylist === shell`
      // never holds -- compare by permalink_url (unique per system playlist)
      // instead, to guard against the user navigating elsewhere mid-fetch.
      if (openPlaylist?.permalink_url === shell.permalink_url) openPlaylist = { ...shell, tracks };
    } catch (e) {
      loadError = `Failed to load mix tracks: ${e}`;
    }
    openPlaylistLoading = false;
  }

  /**
   * Every cold launch used to fire likes/playlists/me/followings/mixed-
   * selections/feed all at once, concurrently, with nothing cached locally
   * -- meaning that same burst repeated on literally every single launch.
   * That's what got the account rate-limited (429) repeatedly. Now: hydrate
   * instantly from whatever's cached (no network wait at all), then run a
   * background refresh that's sequential and spaced out rather than a
   * concurrent burst, and only actually hits the network when the cache is
   * missing or stale.
   */
  async function refreshAuth() {
    loggedIn = await api.isLoggedIn();
    authChecked = true;
    if (!loggedIn) return;

    officialAuth.refresh(); // cheap local keychain check, not worth blocking on

    const cachedMe = loadCached<Profile>("me");
    if (cachedMe) me = cachedMe.value;
    const cachedFollowingIds = loadCached<number[]>("followingIds");
    if (cachedFollowingIds) followingStore.seed(cachedFollowingIds.value);

    await navigate(view); // sequential internally -- see ensureLikesLoaded/ensurePlaylistsLoaded

    syncStatus.syncing();
    await syncWithCache("me", () => api.me(), (v) => (me = v), { onRateLimited: () => syncStatus.rateLimited() });
    await delay(400);
    await syncWithCache("followingIds", () => api.myFollowingsIds(), (v) => followingStore.seed(v), {
      onRateLimited: () => syncStatus.rateLimited(),
    });
    syncStatus.done();
  }

  async function ensureLikesLoaded(force = false) {
    await syncWithCache("likes", () => api.likes(), (v) => { likes = v; likesStore.seed(v.map((t) => t.id)); }, {
      maxAgeMs: force ? 0 : 5 * 60 * 1000,
      onRateLimited: () => syncStatus.rateLimited(),
      onError: (e) => (loadError = `Failed to load likes: ${e}`),
    });
  }

  let lastLikesSyncAt = 0;

  /**
   * Likes made outside the app (e.g. via the DRM/DataDome-blocked-like
   * fallback that opens a track on soundcloud.com) never touch our store.
   * Rather than polling the full, paginated likes list on a timer, this
   * checks the cheap `likes_count` on `/me` first and only re-fetches the
   * whole list when that number actually moved. Throttled to at most once
   * per 10s since it can be triggered by both a window-focus event and a
   * timer in quick succession.
   */
  async function syncLikesIfChanged() {
    if (Date.now() - lastLikesSyncAt < 10_000) return;
    lastLikesSyncAt = Date.now();
    try {
      const fresh = await api.me();
      const changed = me?.likes_count !== fresh.likes_count;
      me = fresh;
      saveCached("me", fresh);
      if (changed) {
        likes = await api.likes();
        likesStore.seed(likes.map((t) => t.id));
        saveCached("likes", likes);
      }
    } catch {
      // silent -- this is a background sync, not a user-initiated action
    }
  }

  /** Called when the player bar's "open on SoundCloud" fallback is used, as
   * a fallback in case the window-focus listener below doesn't fire (e.g.
   * the browser opened on a different monitor without an OS focus change). */
  function scheduleLikesSyncCheck() {
    setTimeout(syncLikesIfChanged, 25_000);
  }

  let refreshing = $state(false);
  // Profile and TrackDetail fetch their own data in an $effect keyed off
  // their id prop, so there's nothing to re-invoke from here directly --
  // bumping one of these and using it as part of a {#key} block forces a
  // remount, which re-runs that effect.
  let profileRefreshKey = $state(0);
  let trackDetailRefreshKey = $state(0);

  /** Force-refetches whatever's currently on screen. */
  async function refreshCurrent() {
    if (refreshing) return;
    refreshing = true;
    try {
      if (selectedTrack) {
        trackDetailRefreshKey += 1;
      } else if (view === "profile" && profileUserId !== null) {
        profileRefreshKey += 1;
      } else if (view === "home") {
        await Promise.all([ensureLikesLoaded(true), ensurePlaylistsLoaded(true)]);
      } else if (view === "likes") {
        await ensureLikesLoaded(true);
      } else if (view === "playlists") {
        if (openPlaylist && openPlaylist.id !== -1) {
          openPlaylist = await api.playlist(openPlaylist.id);
        } else if (!openPlaylist) {
          await ensurePlaylistsLoaded(true);
        }
        // system playlists (id === -1) aren't re-fetchable without their
        // original track-id list, which we don't retain after hydrating --
        // an acceptable gap for what SoundCloud presents as a fairly static mix
      } else if (view === "search" && searchQuery.trim()) {
        await runSearch();
      }
    } finally {
      refreshing = false;
    }
  }

  async function ensurePlaylistsLoaded(force = false) {
    await syncWithCache("playlists", () => api.playlists(), (v) => (playlists = v), {
      maxAgeMs: force ? 0 : 5 * 60 * 1000,
      onRateLimited: () => syncStatus.rateLimited(),
      onError: (e) => (loadError = `Failed to load playlists: ${e}`),
    });
  }

  async function onLogin() {
    authStatus = "Opening SoundCloud login...";
    try {
      await api.startLogin();
    } catch (e) {
      authStatus = `Failed to open login window: ${e}`;
    }
  }

  async function onLogout() {
    await api.logout();
    loggedIn = false;
    likes = [];
    playlists = [];
    searchResults = [];
    peopleResults = [];
    openPlaylist = null;
    me = null;
    view = "home";
    navHistory = [];
    player.clearPersistedPlayback();
    for (const key of ["likes", "playlists", "me", "followingIds", "feed", "mixedSelections"]) clearCached(key);
  }

  function openProfile(userId: number) {
    pushHistory();
    profileUserId = userId;
    view = "profile";
  }

  async function navigate(v: View) {
    // guarded (not unconditional) since refreshAuth() calls navigate(view) with
    // the *current* view on startup, purely to trigger the loading below --
    // that shouldn't also push a redundant "go back to the exact same place" entry
    if (v !== view) pushHistory();
    view = v;
    openPlaylist = null;
    if (v !== "profile") profileUserId = null;
    loadError = "";
    if (v === "home") {
      loading = likes.length === 0 && playlists.length === 0;
      // sequential, not Promise.all -- see refreshAuth's comment on why
      await ensureLikesLoaded();
      await ensurePlaylistsLoaded();
      loading = false;
    } else if (v === "likes") {
      loading = likes.length === 0;
      await ensureLikesLoaded();
      loading = false;
    } else if (v === "playlists") {
      loading = playlists.length === 0;
      await ensurePlaylistsLoaded();
      loading = false;
    }
  }

  async function runSearch() {
    if (!searchQuery.trim()) return;
    view = "search";
    loading = true;
    loadError = "";
    try {
      if (searchTab === "tracks") {
        searchResults = await api.search(searchQuery);
      } else {
        peopleResults = await api.searchUsers(searchQuery);
      }
    } catch (e) {
      loadError = `Search failed: ${e}`;
    }
    loading = false;
  }

  function switchSearchTab(tab: "tracks" | "people") {
    searchTab = tab;
    if (searchQuery.trim()) runSearch();
  }

  refreshAuth();
  checkForUpdates();
  getCurrentWindow().onFocusChanged(({ payload: focused }) => {
    if (focused) syncLikesIfChanged();
  });
  listen<AuthEvent>("auth:result", async (event) => {
    authStatus = event.payload.ok ? "" : `Login failed: ${event.payload.error ?? "unknown error"}`;
    await refreshAuth();
  });
  // Fires once the auto-chained official OAuth connect (right after
  // primary login) finishes in the background. Without this, a successful
  // onboarding connect would go unnoticed by the frontend until the app
  // was relaunched, so the very first like or follow would prompt again
  // for no reason.
  listen<boolean>("official_auth:result", (event) => {
    officialAuth.connected = event.payload;
  });
</script>

<div class="window">
  <div class="window-body">
{#if !authChecked}
  <div class="boot"></div>
{:else if !loggedIn}
  <LoginScreen onLogin={onLogin} status={authStatus} />
{:else}
  <div class="app">
    <TopNav
      active={view === "profile" ? "home" : view}
      onNavigate={navigate}
      {onLogout}
      bind:searchQuery
      onSearch={runSearch}
      {me}
      onOpenOwnProfile={() => me && openProfile(me.id)}
      {canGoBack}
      onBack={goBack}
      onRefresh={refreshCurrent}
      {refreshing}
    />

    <main>
      {#if selectedTrack}
        {#key trackDetailRefreshKey}
          <TrackDetail track={selectedTrack} onBack={goBack} onOpenProfile={openProfile} />
        {/key}
      {:else if view === "home"}
        <Home {me} {likes} {playlists} onNavigate={navigate} onOpenProfile={openProfile} onOpenTrack={openTrack} onOpenPlaylist={viewPlaylist} onOpenSystemPlaylist={viewSystemPlaylist} />
      {:else if view === "search"}
        <div class="search-tabs">
          <button class:active={searchTab === "tracks"} onclick={() => switchSearchTab("tracks")}>Tracks</button>
          <button class:active={searchTab === "people"} onclick={() => switchSearchTab("people")}>People</button>
        </div>
        {#if loading}
          <p class="muted">Loading...</p>
        {:else if loadError}
          <p class="error-text">{loadError}</p>
        {:else if searchTab === "tracks"}
          {#if searchResults.length === 0}
            <p class="muted">Search for tracks to get started.</p>
          {:else}
            <div class="list">
              {#each searchResults as t, i}<TrackRow track={t} queue={searchResults} index={i} onOpenProfile={openProfile} onOpenTrack={openTrack} />{/each}
            </div>
          {/if}
        {:else if peopleResults.length === 0}
          <p class="muted">Search for people to get started.</p>
        {:else}
          <div class="people-grid">
            {#each peopleResults as p}
              <button class="person-card" onclick={() => openProfile(p.id)}>
                {#if p.avatar_url}
                  <img src={p.avatar_url} alt="" loading="lazy" />
                {:else}
                  <div class="person-avatar-fallback">{(p.username ?? "?")[0]?.toUpperCase()}</div>
                {/if}
                <span class="person-name">{p.username ?? `User #${p.id}`}</span>
              </button>
            {/each}
          </div>
        {/if}
      {:else if view === "likes"}
        <h1>Likes</h1>
        {#if loading}
          <p class="muted">Loading...</p>
        {:else if loadError}
          <p class="error-text">{loadError}</p>
        {:else if likes.length === 0}
          <p class="muted">No likes found.</p>
        {:else}
          <div class="list">
            {#each likes as t, i}<TrackRow track={t} queue={likes} index={i} onOpenProfile={openProfile} onOpenTrack={openTrack} />{/each}
          </div>
        {/if}
      {:else if view === "playlists"}
        {#if openPlaylist}
          <button class="back" onclick={goBack}><Icon name="arrow-left" size={14} /> Back</button>
          <h1>{openPlaylist.title ?? "Untitled playlist"}</h1>
          {#if openPlaylistLoading}
            <p class="muted">Loading tracks...</p>
          {:else if loadError}
            <p class="error-text">{loadError}</p>
          {:else if openPlaylist.tracks.length === 0}
            <p class="muted">This playlist has no tracks.</p>
          {:else}
            <div class="list">
              {#each openPlaylist.tracks as t, i}<TrackRow track={t} queue={openPlaylist.tracks} index={i} onOpenProfile={openProfile} onOpenTrack={openTrack} />{/each}
            </div>
          {/if}
        {:else}
          <h1>Playlists</h1>
          {#if loading}
            <p class="muted">Loading...</p>
          {:else if loadError}
            <p class="error-text">{loadError}</p>
          {:else if playlists.length === 0}
            <p class="muted">No playlists found.</p>
          {:else}
            <div class="playlist-grid">
              {#each playlists as p}
                <button class="playlist-card" onclick={() => viewPlaylist(p)}>
                  {#if p.artwork_url}
                    <img src={p.artwork_url} alt="" loading="lazy" />
                  {:else}
                    <div class="playlist-artwork-fallback"><Icon name="queue" size={20} /></div>
                  {/if}
                  <span class="playlist-title">{p.title ?? "Untitled"}</span>
                  <span class="playlist-count">{p.track_count ?? p.tracks.length} tracks</span>
                </button>
              {/each}
            </div>
          {/if}
        {/if}
      {:else if view === "profile" && profileUserId !== null}
        {#key `${profileUserId}-${profileRefreshKey}`}
          <ProfileView userId={profileUserId} onOpenProfile={openProfile} onOpenTrack={openTrack} isOwnProfile={me?.id === profileUserId} />
        {/key}
      {/if}
    </main>

    <SyncStatusBar />
    <PlayerBar onOpenTrack={openTrack} onOpenProfile={openProfile} onOpenedOnSoundCloud={scheduleLikesSyncCheck} />
  </div>
{/if}
  </div>
</div>

<style>
:global(:root) {
  --bg: #f6f6f6;
  --fg: #0f0f0f;
  --muted: #767676;
  --border: #e0e0e0;
  --row-hover: #ececec;
  --artwork-bg: #ddd;
  --nav-bg: #ffffff;
  --search-bg: #f2f2f2;
  --player-bg: #0e0e0e;
  --player-fg: #ffffff;
  --accent: #ff5500;
  --accent-hover: #e04c00;
  --error-bg: #fde2e2;
  --error-text: #a32626;
  --scrollbar-thumb: #c8c8c8;
  --scrollbar-thumb-hover: #a8a8a8;
  --titlebar-bg: #2b2b2b;
  --titlebar-fg: #ffffff;
  --titlebar-fg-muted: #9a9a9a;
  --titlebar-hover: rgba(255, 255, 255, 0.12);
}

@media (prefers-color-scheme: dark) {
  :global(:root) {
    --bg: #121212;
    --fg: #f0f0f0;
    --muted: #999;
    --border: #2a2a2a;
    --row-hover: #232323;
    --artwork-bg: #333;
    --nav-bg: #181818;
    --search-bg: #232323;
    --player-bg: #0a0a0a;
    --player-fg: #ffffff;
    --accent: #ff5500;
    --accent-hover: #ff6a1f;
    --error-bg: #3a1f1f;
    --error-text: #ff9b9b;
    --scrollbar-thumb: #3a3a3a;
    --scrollbar-thumb-hover: #4d4d4d;
  }
}

:global(body) {
  margin: 0;
  background: var(--bg);
  color: var(--fg);
  font-family: "Interstate", "Archivo Variable", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
}

:global(*) {
  scrollbar-width: thin;
  scrollbar-color: var(--scrollbar-thumb) transparent;
}

:global(*::-webkit-scrollbar) {
  width: 10px;
  height: 10px;
}

:global(*::-webkit-scrollbar-track) {
  background: transparent;
}

:global(*::-webkit-scrollbar-thumb) {
  background-color: var(--scrollbar-thumb);
  border-radius: 999px;
  border: 2px solid transparent;
  background-clip: padding-box;
}

:global(*::-webkit-scrollbar-thumb:hover) {
  background-color: var(--scrollbar-thumb-hover);
}

.window {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.window-body {
  flex: 1;
  min-height: 0;
  display: flex;
}

.window-body > :global(*) {
  flex: 1;
  min-width: 0;
}

.boot {
  height: 100%;
}

.app {
  height: 100%;
  display: flex;
  flex-direction: column;
}

main {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem 2rem;
  box-sizing: border-box;
}

h1 {
  margin-top: 0;
  font-size: 1.3rem;
}

.muted {
  color: var(--muted);
}

.error-text {
  color: var(--error-text);
}

.search-tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1.25rem;
  border-bottom: 1px solid var(--border);
}

.search-tabs button {
  background: none;
  border: none;
  font: inherit;
  font-weight: 600;
  padding: 0.5rem 0.25rem;
  margin-right: 1rem;
  color: var(--muted);
  cursor: pointer;
  border-bottom: 2px solid transparent;
}

.search-tabs button.active {
  color: var(--fg);
  border-bottom-color: var(--accent);
}

.list {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

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

.playlist-grid,
.people-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 1.25rem;
}

.playlist-card,
.person-card {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.35rem;
  background: none;
  border: none;
  cursor: pointer;
  color: inherit;
  font: inherit;
  padding: 0;
  text-align: left;
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

.person-card img,
.person-avatar-fallback {
  width: 100%;
  aspect-ratio: 1;
  border-radius: 50%;
  object-fit: cover;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
  font-size: 1.8rem;
}

.playlist-title,
.person-name {
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: 100%;
  font-size: 0.9rem;
}

.playlist-count {
  font-size: 0.8rem;
  color: var(--muted);
}
</style>
