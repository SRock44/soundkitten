<script lang="ts">
  import type { Profile } from "../types";
  import FollowButton from "./FollowButton.svelte";

  let { item, onOpen }: { item: Profile; onOpen: (item: Profile) => void } = $props();

  let meta = $derived(
    item.followers_count != null ? `${item.followers_count.toLocaleString()} follower${item.followers_count === 1 ? "" : "s"}` : "Artist",
  );
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="person-tile" role="button" tabindex="0" onclick={() => onOpen(item)}>
  <span class="card-surface">
    <span class="avatar-wrap">
      {#if item.avatar_url}
        <img src={item.avatar_url} alt="" loading="lazy" />
      {:else}
        <span class="avatar-fallback">{(item.username ?? "?")[0]?.toUpperCase()}</span>
      {/if}
    </span>
    <span class="card-text">
      <span class="tile-title">{item.username ?? `User #${item.id}`}</span>
      <span class="tile-meta">{meta}</span>
    </span>
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <span class="follow-wrap" onclick={(e) => e.stopPropagation()}>
      <FollowButton userId={item.id} permalinkUrl={item.permalink_url} />
    </span>
  </span>
</div>

<style>
.person-tile {
  display: flex;
  flex-direction: column;
  flex: 0 0 172px;
  min-width: 0;
  width: 100%;
  background: none;
  border: none;
  padding: 0;
  text-align: left;
  color: inherit;
  font: inherit;
  cursor: pointer;
}

.card-surface {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.7rem;
  padding: 1.1rem 0.85rem 0.85rem;
  border-radius: 14px;
  background: var(--surface);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.06);
  transition: background-color 0.15s ease, transform 0.15s ease, box-shadow 0.15s ease;
}

.person-tile:hover .card-surface {
  background: var(--surface-hover);
  transform: translateY(-4px);
  box-shadow: 0 18px 32px rgba(0, 0, 0, 0.22);
}

.avatar-wrap {
  width: 68%;
  aspect-ratio: 1;
  border-radius: 50%;
  overflow: hidden;
  background: var(--artwork-bg);
}

.avatar-wrap img,
.avatar-fallback {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
  font-size: 1.6rem;
}

.card-text {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.2rem;
  min-width: 0;
  width: 100%;
  text-align: center;
}

.tile-title {
  font-weight: 700;
  font-size: 0.92rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}

.tile-meta {
  font-size: 0.78rem;
  color: var(--muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100%;
}

.follow-wrap {
  display: flex;
  justify-content: center;
}
</style>
