<script lang="ts">
  import type { Profile } from "../types";
  import Icon from "./Icon.svelte";

  let {
    title,
    users,
    loading,
    onClose,
    onOpenProfile,
  }: {
    title: string;
    users: Profile[];
    loading: boolean;
    onClose: () => void;
    onOpenProfile: (id: number) => void;
  } = $props();
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && onClose()} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="modal" onclick={(e) => e.stopPropagation()}>
    <div class="header">
      <span>{title}</span>
      <button class="close" onclick={onClose} aria-label="Close"><Icon name="close" size={13} /></button>
    </div>
    <div class="list">
      {#if loading}
        <p class="muted">Loading...</p>
      {:else if users.length === 0}
        <p class="muted">Nobody here yet.</p>
      {:else}
        {#each users as u}
          <button class="row" onclick={() => { onOpenProfile(u.id); onClose(); }}>
            {#if u.avatar_url}
              <img src={u.avatar_url} alt="" class="avatar" />
            {:else}
              <div class="avatar avatar-fallback">{(u.username ?? "?")[0]?.toUpperCase()}</div>
            {/if}
            <div class="info">
              <span class="name">{u.username ?? `User #${u.id}`}</span>
              <span class="count">{(u.followers_count ?? 0).toLocaleString()} followers</span>
            </div>
          </button>
        {/each}
      {/if}
    </div>
  </div>
</div>

<style>
.backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 60;
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

.list {
  overflow-y: auto;
  padding: 0.4rem;
}

.muted {
  color: var(--muted);
  padding: 0.5rem 0.6rem;
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
}

.row:hover {
  background: var(--row-hover);
}

.avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
  background: var(--artwork-bg);
}

.avatar-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
  font-size: 0.85rem;
}

.info {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.name {
  font-weight: 600;
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.count {
  font-size: 0.78rem;
  color: var(--muted);
}
</style>
