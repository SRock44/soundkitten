<script lang="ts">
  import type { Profile } from "../types";
  import Icon from "./Icon.svelte";
  import WindowControls from "./WindowControls.svelte";
  import SettingsModal from "./SettingsModal.svelte";

  type View = "home" | "search" | "likes" | "playlists";
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
  } = $props();

  let openMenu = $state<"account" | "notifications" | null>(null);
  let showSettings = $state(false);

  function toggleMenu(menu: "account" | "notifications") {
    openMenu = openMenu === menu ? null : menu;
  }

  function closeMenus() {
    openMenu = null;
  }
</script>

<svelte:window onclick={closeMenus} />

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
      <button class="logo" onclick={() => onNavigate("home")}>
        <img src="/logo.png" alt="" class="logo-mark" />
        <span class="logo-text">SoundKitten</span>
      </button>
      <nav class="links">
        <button class:active={active === "home"} onclick={() => onNavigate("home")}>Home</button>
        <button class:active={active === "likes"} onclick={() => onNavigate("likes")}>Likes</button>
        <button class:active={active === "playlists"} onclick={() => onNavigate("playlists")}>Playlists</button>
      </nav>
    </div>

    <form class="search" onsubmit={(e) => { e.preventDefault(); onNavigate("search"); onSearch(); }}>
      <input placeholder="Search tracks" bind:value={searchQuery} />
      <button type="submit" aria-label="Search"><Icon name="search" /></button>
    </form>

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

/* The main window's real floor is minWidth:760 (tauri.conf.json) -- with
   no responsive handling at all, the search column's 280px minimum ate
   into the left nav's space, and "Playlists" (the last/least space)
   could get squeezed off entirely with no visible sign anything was
   wrong. Two breakpoints progressively reclaim space: first the search
   box's floor shrinks, then (only at the narrowest supported width) the
   "SoundKitten" wordmark drops to icon-only, since keeping Home/Likes/
   Playlists visible and clickable matters more than the full logo. */
@media (max-width: 1000px) {
  .topnav {
    grid-template-columns: 1fr minmax(180px, 340px) 1fr;
  }

  .left {
    gap: 1rem;
  }

  .links button {
    padding: 0.4rem 0.55rem;
  }
}

@media (max-width: 820px) {
  .topnav {
    grid-template-columns: 1fr minmax(120px, 240px) 1fr;
    padding: 0 0.75rem;
  }

  .left {
    gap: 0.6rem;
  }

  .logo-text {
    display: none;
  }
}

.left {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  min-width: 0;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 800;
  font-size: 1.05rem;
  letter-spacing: -0.02em;
  color: var(--fg);
  flex-shrink: 0;
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
  font-family: inherit;
}

.logo-mark {
  width: 26px;
  height: 26px;
  object-fit: contain;
}

.links {
  display: flex;
  gap: 0.25rem;
}

.links button {
  background: none;
  border: none;
  font: inherit;
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--muted);
  cursor: pointer;
  padding: 0.4rem 0.7rem;
  border-radius: 999px;
}

.links button:hover {
  color: var(--fg);
}

.links button.active {
  color: var(--fg);
  background: var(--row-hover);
}

.search {
  display: flex;
  align-items: center;
  background: var(--search-bg);
  border: 1px solid var(--border);
  border-radius: 999px;
  padding: 0.35rem 0.4rem 0.35rem 0.9rem;
}

.search input {
  flex: 1;
  border: none;
  background: none;
  outline: none;
  color: inherit;
  font: inherit;
  min-width: 0;
}

.search button {
  border: none;
  background: none;
  cursor: pointer;
  color: var(--muted);
  border-radius: 999px;
  width: 1.8rem;
  height: 1.8rem;
  display: flex;
  align-items: center;
  justify-content: center;
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
