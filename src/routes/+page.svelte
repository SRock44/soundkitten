<script lang="ts">
  import "@fontsource-variable/archivo";
  import { listen } from "@tauri-apps/api/event";
  import { api } from "$lib/api";
  import LoginScreen from "$lib/components/LoginScreen.svelte";
  import TopNav from "$lib/components/TopNav.svelte";
  import PlayerBar from "$lib/components/PlayerBar.svelte";
  import TrackRow from "$lib/components/TrackRow.svelte";
  import ProfileView from "$lib/components/Profile.svelte";
  import Home from "$lib/components/Home.svelte";
  import TrackDetail from "$lib/components/TrackDetail.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import { likes as likesStore } from "$lib/stores/likes.svelte";
  import type { Playlist, Profile, Track } from "$lib/types";

  type AuthEvent = { ok: boolean; error: string | null };
  type View = "home" | "search" | "likes" | "playlists" | "profile";

  let loggedIn = $state(false);
  let authChecked = $state(false);
  let authStatus = $state("");
  let showManualFallback = $state(false);
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

  async function viewPlaylist(p: Playlist) {
    view = "playlists";
    openPlaylist = p;
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

  async function refreshAuth() {
    loggedIn = await api.isLoggedIn();
    authChecked = true;
    if (loggedIn) {
      navigate(view);
      api.me().then((p) => (me = p)).catch(() => {});
    }
  }

  async function ensureLikesLoaded() {
    if (likes.length > 0) return;
    try {
      likes = await api.likes();
      likesStore.seed(likes.map((t) => t.id));
    } catch (e) {
      loadError = `Failed to load likes: ${e}`;
    }
  }

  async function ensurePlaylistsLoaded() {
    if (playlists.length > 0) return;
    try {
      playlists = await api.playlists();
    } catch (e) {
      loadError = `Failed to load playlists: ${e}`;
    }
  }

  async function onLogin() {
    authStatus = "Opening SoundCloud login...";
    showManualFallback = false;
    try {
      await api.startLogin();
    } catch (e) {
      authStatus = `Failed to open login window: ${e}`;
      showManualFallback = true;
    }
  }

  async function onSubmitManualToken(token: string) {
    try {
      await api.setManualToken(token);
      showManualFallback = false;
      authStatus = "Manual token saved.";
      await refreshAuth();
    } catch (e) {
      authStatus = `Failed to save token: ${e}`;
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
  }

  function openProfile(userId: number) {
    profileUserId = userId;
    view = "profile";
  }

  async function navigate(v: View) {
    view = v;
    openPlaylist = null;
    if (v !== "profile") profileUserId = null;
    loadError = "";
    if (v === "home") {
      loading = likes.length === 0 && playlists.length === 0;
      await Promise.all([ensureLikesLoaded(), ensurePlaylistsLoaded()]);
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
  listen<AuthEvent>("auth:result", async (event) => {
    if (event.payload.ok) {
      authStatus = "";
      showManualFallback = false;
    } else {
      authStatus = `Login failed: ${event.payload.error ?? "unknown error"}`;
      showManualFallback = true;
    }
    await refreshAuth();
  });
</script>

<div class="window">
  <TitleBar />
  <div class="window-body">
{#if !authChecked}
  <div class="boot"></div>
{:else if !loggedIn}
  <LoginScreen onLogin={onLogin} status={authStatus} {showManualFallback} {onSubmitManualToken} />
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
    />

    <main>
      {#if selectedTrack}
        <TrackDetail track={selectedTrack} onBack={() => (selectedTrack = null)} onOpenProfile={(id) => { selectedTrack = null; openProfile(id); }} />
      {:else if view === "home"}
        <Home {me} {likes} {playlists} onNavigate={navigate} onOpenProfile={openProfile} onOpenTrack={(tr) => (selectedTrack = tr)} onOpenPlaylist={viewPlaylist} />
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
              {#each searchResults as t, i}<TrackRow track={t} queue={searchResults} index={i} onOpenProfile={openProfile} onOpenTrack={(tr) => (selectedTrack = tr)} />{/each}
            </div>
          {/if}
        {:else if peopleResults.length === 0}
          <p class="muted">Search for people to get started.</p>
        {:else}
          <div class="people-grid">
            {#each peopleResults as p}
              <button class="person-card" onclick={() => openProfile(p.id)}>
                {#if p.avatar_url}
                  <img src={p.avatar_url} alt="" />
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
            {#each likes as t, i}<TrackRow track={t} queue={likes} index={i} onOpenProfile={openProfile} onOpenTrack={(tr) => (selectedTrack = tr)} />{/each}
          </div>
        {/if}
      {:else if view === "playlists"}
        {#if openPlaylist}
          <button class="back" onclick={() => (openPlaylist = null)}><Icon name="arrow-left" size={14} /> Playlists</button>
          <h1>{openPlaylist.title ?? "Untitled playlist"}</h1>
          {#if openPlaylistLoading}
            <p class="muted">Loading tracks...</p>
          {:else if openPlaylist.tracks.length === 0}
            <p class="muted">This playlist has no tracks.</p>
          {:else}
            <div class="list">
              {#each openPlaylist.tracks as t, i}<TrackRow track={t} queue={openPlaylist.tracks} index={i} onOpenProfile={openProfile} onOpenTrack={(tr) => (selectedTrack = tr)} />{/each}
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
                    <img src={p.artwork_url} alt="" />
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
        <ProfileView userId={profileUserId} onOpenProfile={openProfile} onOpenTrack={(tr) => (selectedTrack = tr)} isOwnProfile={me?.id === profileUserId} />
      {/if}
    </main>

    <PlayerBar onOpenTrack={(tr) => (selectedTrack = tr)} onOpenProfile={openProfile} />
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
