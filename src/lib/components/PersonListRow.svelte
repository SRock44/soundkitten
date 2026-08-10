<script lang="ts">
  import type { Profile } from "../types";

  let { item, onOpen }: { item: Profile; onOpen: (item: Profile) => void } = $props();

  let meta = $derived(
    item.followers_count != null ? `${item.followers_count.toLocaleString()} follower${item.followers_count === 1 ? "" : "s"}` : "Artist",
  );
</script>

<button class="person-row-item" onclick={() => onOpen(item)}>
  {#if item.avatar_url}
    <img src={item.avatar_url} alt="" class="avatar" loading="lazy" />
  {:else}
    <span class="avatar avatar-fallback">{(item.username ?? "?")[0]?.toUpperCase()}</span>
  {/if}
  <span class="info">
    <span class="row-title">{item.username ?? `User #${item.id}`}</span>
    <span class="row-meta">{meta}</span>
  </span>
</button>

<style>
.person-row-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  background: none;
  border: none;
  padding: 0.4rem 0.5rem;
  border-radius: 6px;
  text-align: left;
  color: inherit;
  font: inherit;
  cursor: pointer;
  transition: background-color 0.12s ease;
}

.person-row-item:hover {
  background: var(--row-hover);
}

.avatar {
  width: 44px;
  height: 44px;
  flex-shrink: 0;
  border-radius: 50%;
  object-fit: cover;
  background: var(--artwork-bg);
}

.avatar-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
  font-size: 1rem;
}

.info {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.row-title {
  font-weight: 600;
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-meta {
  font-size: 0.78rem;
  color: var(--muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
