<script lang="ts">
  import type { Playlist, Profile, SearchResultItem, Track } from "../types";
  import { api } from "../api";
  import { settings } from "../stores/settings.svelte";
  import Icon from "./Icon.svelte";
  import WindowControls from "./WindowControls.svelte";
  import SettingsModal from "./SettingsModal.svelte";

  type View = "home" | "search" | "likes" | "playlists" | "feed";
  let {
    active,
    onNavigate,
    onLogout,
    searchQuery = $bindable(""),
    onSearch,
    me,
    onOpenOwnProfile,
    canGoBack = false,
    onBack,
    onRefresh,
    refreshing = false,
    onOpenTrack,
    onOpenProfile,
    onOpenPlaylist,
  }: {
    active: View;
    onNavigate: (v: View) => void;
    onLogout: () => void;
    searchQuery?: string;
    onSearch: () => void;
    me: Profile | null;
    onOpenOwnProfile: () => void;
    canGoBack?: boolean;
    onBack?: () => void;
    onRefresh?: () => void;
    refreshing?: boolean;
    onOpenTrack: (t: Track) => void;
    onOpenProfile: (userId: number) => void;
    onOpenPlaylist: (p: Playlist) => void;
  } = $props();

  let openMenu = $state<"account" | "notifications" | null>(null);
  let showSettings = $state(false);

  function toggleMenu(menu: "account" | "notifications") {
    openMenu = openMenu === menu ? null : menu;
  }

  function closeMenus() {
    openMenu = null;
  }

  // Mac Spotlight-style search: clicking the navbar's search pill opens a
  // centered floating overlay above a blurred backdrop (the rest of the UI
  // stays dimly visible but non-interactive underneath), rather than
  // expanding the small navbar field in place. Live preview results are a
  // debounced as-you-type lookup against the same unified /search endpoint
  // the full Search page uses, so a user can jump straight to a track/
  // artist/playlist without ever leaving the overlay.
  let spotlightOpen = $state(false);
  let spotlightInputEl: HTMLInputElement | undefined = $state();
  let previewLoading = $state(false);
  let previewResults = $state<SearchResultItem[]>([]);
  let previewSeq = 0;
  let previewTimer: ReturnType<typeof setTimeout> | undefined;

  function openSpotlight() {
    spotlightOpen = true;
    if (searchQuery.trim()) onSearchInput();
  }

  function closeSpotlight() {
    spotlightOpen = false;
  }

  function onSearchInput() {
    const q = searchQuery.trim();
    if (previewTimer) clearTimeout(previewTimer);
    if (!q) {
      previewResults = [];
      previewLoading = false;
      return;
    }
    previewLoading = true;
    const seq = ++previewSeq;
    previewTimer = setTimeout(async () => {
      try {
        const results = await api.searchAll(q);
        if (seq !== previewSeq) return; // stale response from an earlier keystroke
        previewResults = results.slice(0, 6);
      } catch {
        if (seq === previewSeq) previewResults = [];
      } finally {
        if (seq === previewSeq) previewLoading = false;
      }
    }, 250);
  }

  function clearSearch() {
    searchQuery = "";
    previewResults = [];
    spotlightInputEl?.focus();
  }

  function submitSearch() {
    closeSpotlight();
    onNavigate("search");
    onSearch();
  }

  function openPreviewItem(item: SearchResultItem) {
    closeSpotlight();
    if (item.kind === "track") onOpenTrack(item);
    else if (item.kind === "user") onOpenProfile(item.id);
    else onOpenPlaylist(item);
  }

  $effect(() => {
    if (spotlightOpen) spotlightInputEl?.focus();
  });

  function onWindowKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (spotlightOpen) closeSpotlight();
      else closeMenus();
    }
  }
</script>

<svelte:window onclick={closeMenus} onkeydown={onWindowKeydown} />

<div class="topnav-wrapper">
  <div class="utility-bar" data-tauri-drag-region>
    <div class="nav-controls">
      <button class="nav-btn" onclick={onBack} disabled={!canGoBack} aria-label="Back" title="Back"><Icon name="arrow-left" size={15} /></button>
      <button class="nav-btn" class:spinning={refreshing} onclick={onRefresh} disabled={refreshing} aria-label="Refresh" title="Refresh">
        <Icon name="refresh" size={15} />
      </button>
    </div>
    <WindowControls />
  </div>

  <header class="topnav">
    <div class="left">
      <button class="logo" onclick={() => onNavigate("home")} aria-label="SoundKitten home" title="SoundKitten">
        <img src="/logo.png" alt="" class="logo-mark" />
      </button>
      <nav class="links">
        <button class:active={active === "home"} onclick={() => onNavigate("home")} aria-label="Home" title="Home"><Icon name="home" size={18} /></button>
        <button class:active={active === "likes"} onclick={() => onNavigate("likes")} aria-label="Likes" title="Likes"><Icon name="heart" size={18} /></button>
        <button class:active={active === "playlists"} onclick={() => onNavigate("playlists")} aria-label="Playlists" title="Playlists"><Icon name="playlists" size={18} /></button>
      </nav>
    </div>

    <div class="search-wrap">
      <button type="button" class="search-trigger" onclick={(e) => { e.stopPropagation(); openSpotlight(); }} aria-label="Search" title="Search">
        <Icon name="search" size={15} />
        <span class="search-trigger-text">{searchQuery.trim() || "Search"}</span>
      </button>
    </div>

    <div class="right">
      <div class="menu-wrap">
        <button
          class="icon-btn"
          aria-label="Notifications"
          onclick={(e) => { e.stopPropagation(); toggleMenu("notifications"); }}
        ><Icon name="bell" /></button>
        {#if openMenu === "notifications"}
          <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
          <div class="dropdown" onclick={(e) => e.stopPropagation()} role="presentation">
            <p class="empty">No new notifications.</p>
          </div>
        {/if}
      </div>

      <div class="menu-wrap">
        <button
          class="account-btn"
          onclick={(e) => { e.stopPropagation(); toggleMenu("account"); }}
          aria-label="Account menu"
        >
          {#if me?.avatar_url}
            <img src={me.avatar_url} alt="" class="avatar" />
          {:else}
            <span class="avatar avatar-fallback">{(me?.username ?? "?")[0]?.toUpperCase()}</span>
          {/if}
          <span class="chevron"><Icon name="chevron-down" size={12} /></span>
        </button>
        {#if openMenu === "account"}
          <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
          <div class="dropdown account-dropdown" onclick={(e) => e.stopPropagation()} role="presentation">
            <button onclick={() => { onOpenOwnProfile(); closeMenus(); }}>Profile</button>
            <button onclick={() => { showSettings = true; closeMenus(); }}>Settings</button>
            <button onclick={() => { onLogout(); closeMenus(); }}>Log out</button>
          </div>
        {/if}
      </div>
    </div>
  </header>
</div>

{#if spotlightOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="spotlight-backdrop" class:performance={settings.performanceMode} onclick={closeSpotlight} role="presentation">
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="spotlight-panel" class:performance={settings.performanceMode} onclick={(e) => e.stopPropagation()} role="presentation">
      <form class="spotlight-search" class:has-results={searchQuery.trim().length > 0} onsubmit={(e) => { e.preventDefault(); submitSearch(); }}>
        <Icon name="search" size={20} />
        <input
          bind:this={spotlightInputEl}
          placeholder="Search tracks, artists, playlists"
          bind:value={searchQuery}
          oninput={onSearchInput}
        />
        {#if searchQuery.trim()}
          <button type="button" class="spotlight-clear" onclick={clearSearch} aria-label="Clear search"><Icon name="close" size={13} /></button>
        {/if}
      </form>

      {#if searchQuery.trim()}
        <div class="spotlight-results">
          {#if previewLoading && previewResults.length === 0}
            <p class="preview-empty">Searching…</p>
          {:else if previewResults.length === 0}
            <p class="preview-empty">No results for "{searchQuery.trim()}"</p>
          {:else}
            <ul class="preview-list">
              {#each previewResults as item (item.kind + ":" + item.id)}
                <li>
                  <button class="preview-item" onclick={() => openPreviewItem(item)}>
                    {#if item.kind === "track"}
                      <span class="preview-art">
                        {#if item.artwork_url}<img src={item.artwork_url} alt="" />{:else}<Icon name="music" size={16} />{/if}
                      </span>
                      <span class="preview-text">
                        <span class="preview-title">{item.title ?? "Untitled"}</span>
                        <span class="preview-meta">Track · {item.user?.username ?? "Unknown artist"}</span>
                      </span>
                    {:else if item.kind === "user"}
                      <span class="preview-art round">
                        {#if item.avatar_url}<img src={item.avatar_url} alt="" />{:else}<Icon name="user" size={16} />{/if}
                      </span>
                      <span class="preview-text">
                        <span class="preview-title">{item.username ?? "Unknown"}</span>
                        <span class="preview-meta">Artist</span>
                      </span>
                    {:else}
                      <span class="preview-art">
                        {#if item.artwork_url}<img src={item.artwork_url} alt="" />{:else}<Icon name="playlists" size={16} />{/if}
                      </span>
                      <span class="preview-text">
                        <span class="preview-title">{item.title ?? "Untitled"}</span>
                        <span class="preview-meta">Playlist{item.track_count != null ? ` · ${item.track_count} tracks` : ""}</span>
                      </span>
                    {/if}
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
          <button class="preview-see-all" onclick={submitSearch}>See all results for "{searchQuery.trim()}"</button>
        </div>
      {/if}
    </div>
  </div>
{/if}

{#if showSettings}
  <SettingsModal onClose={() => (showSettings = false)} />
{/if}

<style>
.topnav-wrapper {
  background: var(--nav-bg);
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
  position: relative;
  z-index: 10;
}

.utility-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 30px;
  padding: 0 0.35rem 0 1.25rem;
}

.nav-controls {
  display: flex;
  align-items: center;
  height: 100%;
  -webkit-app-region: no-drag;
}

.nav-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 38px;
  height: 30px;
  background: none;
  border: none;
  color: var(--muted);
  cursor: pointer;
}

.nav-btn:hover:not(:disabled) {
  color: var(--fg);
  background: var(--row-hover);
}

.nav-btn:disabled {
  opacity: 0.35;
  cursor: default;
}

.nav-btn.spinning :global(svg) {
  animation: nav-btn-spin 0.7s linear infinite;
}

@keyframes nav-btn-spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.topnav {
  display: grid;
  grid-template-columns: 1fr minmax(280px, 480px) 1fr;
  align-items: center;
  height: 56px;
  padding: 0 1.25rem;
}

/* The main window's real floor is minWidth:760 (tauri.conf.json). The
   icon-only nav is far more compact than the old text links, so a single
   breakpoint narrowing the search column's floor is enough to keep
   everything comfortably visible all the way down to 760px. */
@media (max-width: 820px) {
  .topnav {
    grid-template-columns: 1fr minmax(160px, 300px) 1fr;
    padding: 0 0.75rem;
  }

  .left {
    gap: 0.4rem;
  }
}

.left {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  min-width: 0;
}

.logo {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.3rem;
  border-radius: 50%;
}

.logo:hover {
  background: var(--row-hover);
}

.logo-mark {
  width: 24px;
  height: 24px;
  object-fit: contain;
  border-radius: 50%;
}

.links {
  display: flex;
  gap: 0.4rem;
}

.links button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.2rem;
  height: 2.2rem;
  background: none;
  border: none;
  color: var(--muted);
  cursor: pointer;
  border-radius: 50%;
}

.links button:hover {
  color: var(--fg);
  background: var(--row-hover);
}

.links button.active {
  color: var(--fg);
  background: var(--row-hover);
}

.search-wrap {
  justify-self: center;
  width: 100%;
  max-width: 480px;
}

.search-trigger {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background: var(--search-bg);
  border: 1px solid var(--border);
  border-radius: 999px;
  padding: 0.4rem 0.9rem;
  color: var(--muted);
  width: 100%;
  box-sizing: border-box;
  cursor: pointer;
  font: inherit;
}

.search-trigger:hover {
  border-color: var(--muted);
  color: var(--fg);
}

.search-trigger-text {
  font-size: 0.88rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  text-align: left;
}

/* Mac Spotlight-style overlay: a fixed, full-viewport backdrop (blurred so
   the rest of the app stays visible-but-dimmed underneath, not hidden)
   with a floating panel centered near the top of the screen -- not an
   in-navbar expansion. Performance Mode drops the blur/scale-in entirely
   (backdrop-filter is one of the more expensive compositor effects, same
   reasoning as the mini player's waveform/video skip -- see settings.svelte.ts). */
.spotlight-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  backdrop-filter: blur(6px) saturate(1.1);
  -webkit-backdrop-filter: blur(6px) saturate(1.1);
  display: flex;
  justify-content: center;
  padding-top: 14vh;
  z-index: 200;
  animation: spotlight-fade-in 0.14s ease;
}

.spotlight-backdrop.performance {
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: none;
  -webkit-backdrop-filter: none;
  animation: none;
  opacity: 1;
}

@keyframes spotlight-fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.spotlight-panel {
  width: min(560px, calc(100vw - 3rem));
  max-height: 60vh;
  background: var(--nav-bg);
  border: 1px solid var(--border);
  border-radius: 16px;
  box-shadow: 0 24px 60px rgba(0, 0, 0, 0.35);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: fit-content;
  animation: spotlight-scale-in 0.16s cubic-bezier(0.2, 0.8, 0.2, 1);
}

.spotlight-panel.performance {
  animation: none;
  opacity: 1;
  transform: none;
}

@keyframes spotlight-scale-in {
  from {
    opacity: 0;
    transform: scale(0.96) translateY(-6px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.spotlight-search {
  display: flex;
  align-items: center;
  gap: 0.7rem;
  padding: 1rem 1.1rem;
  border-bottom: 1px solid transparent;
  flex-shrink: 0;
}

.spotlight-search.has-results {
  border-bottom-color: var(--border);
}

.spotlight-search input {
  flex: 1;
  border: none;
  background: none;
  outline: none;
  color: var(--fg);
  font: inherit;
  font-size: 1.05rem;
  min-width: 0;
}

.spotlight-clear {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 1.7rem;
  height: 1.7rem;
  border: none;
  background: var(--row-hover);
  color: var(--muted);
  border-radius: 50%;
  cursor: pointer;
  flex-shrink: 0;
}

.spotlight-clear:hover {
  color: var(--fg);
}

.spotlight-results {
  overflow-y: auto;
  padding: 0.4rem;
}

.preview-empty {
  margin: 0;
  padding: 0.75rem 0.6rem;
  color: var(--muted);
  font-size: 0.85rem;
  text-align: center;
}

.preview-list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.preview-item {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  width: 100%;
  background: none;
  border: none;
  text-align: left;
  padding: 0.4rem 0.5rem;
  border-radius: 8px;
  cursor: pointer;
  color: inherit;
  font: inherit;
}

.preview-item:hover {
  background: var(--row-hover);
}

.preview-art {
  flex-shrink: 0;
  width: 34px;
  height: 34px;
  border-radius: 6px;
  background: var(--artwork-bg);
  color: var(--muted);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.preview-art.round {
  border-radius: 50%;
}

.preview-art img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.preview-text {
  display: flex;
  flex-direction: column;
  min-width: 0;
  gap: 0.1rem;
}

.preview-title {
  font-size: 0.88rem;
  font-weight: 600;
  color: var(--fg);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.preview-meta {
  font-size: 0.76rem;
  color: var(--muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.preview-see-all {
  display: block;
  width: 100%;
  background: none;
  border: none;
  border-top: 1px solid var(--border);
  margin-top: 0.3rem;
  padding: 0.6rem 0.5rem 0.3rem;
  color: var(--muted);
  font: inherit;
  font-size: 0.82rem;
  font-weight: 600;
  text-align: center;
  cursor: pointer;
  border-radius: 8px;
}

.preview-see-all:hover {
  color: var(--fg);
  background: var(--row-hover);
}

.right {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 0.4rem;
}

.menu-wrap {
  position: relative;
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--muted);
  padding: 0.45rem;
  border-radius: 50%;
}

.icon-btn:hover {
  color: var(--fg);
  background: var(--row-hover);
}

.account-btn {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  border: none;
  background: none;
  cursor: pointer;
  padding: 0.25rem 0.4rem 0.25rem 0.25rem;
  border-radius: 999px;
}

.account-btn:hover {
  background: var(--row-hover);
}

.avatar {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
  color: var(--muted);
}

.chevron {
  font-size: 0.7rem;
  color: var(--muted);
}

.dropdown {
  position: absolute;
  top: calc(100% + 0.5rem);
  right: 0;
  background: var(--nav-bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  min-width: 12rem;
  padding: 0.4rem;
  z-index: 20;
}

.dropdown .empty {
  margin: 0;
  padding: 0.5rem 0.6rem;
  color: var(--muted);
  font-size: 0.85rem;
  white-space: nowrap;
}

.account-dropdown {
  display: flex;
  flex-direction: column;
  min-width: 9rem;
}

.account-dropdown button {
  background: none;
  border: none;
  text-align: left;
  padding: 0.5rem 0.6rem;
  border-radius: 5px;
  cursor: pointer;
  color: inherit;
  font: inherit;
  font-size: 0.9rem;
}

.account-dropdown button:hover {
  background: var(--row-hover);
}
</style>
