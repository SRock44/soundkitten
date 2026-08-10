<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { following } from "../stores/following.svelte";
  import { officialAuth } from "../stores/officialAuth.svelte";
  import Icon from "./Icon.svelte";

  let { userId, permalinkUrl, compact = false }: { userId: number; permalinkUrl: string | null; compact?: boolean } = $props();

  let isFollowing = $derived(following.has(userId));

  // The connect -> write -> update-store flow lives on the shared
  // `following` store (see its doc comment). Falls back to opening
  // soundcloud.com if the user declines to connect or the write itself
  // fails for any reason, same graceful-degradation pattern as before this
  // existed.
  async function toggleFollow(e: MouseEvent) {
    e.stopPropagation();
    try {
      if ((await following.toggle({ id: userId })) === "declined" && permalinkUrl) openUrl(permalinkUrl);
    } catch {
      if (permalinkUrl) openUrl(permalinkUrl);
    }
  }
</script>

<button
  class="follow-btn"
  class:compact
  class:following={isFollowing}
  onclick={toggleFollow}
  disabled={following.isBusy(userId) || (!permalinkUrl && !officialAuth.connected)}
  aria-label={isFollowing ? "Unfollow" : "Follow"}
  title={isFollowing ? "Unfollow" : "Follow"}
>
  <Icon name={isFollowing ? "user-filled" : "user"} size={compact ? 15 : 14} />
  {#if !compact}<span>{isFollowing ? "Following" : "Follow"}</span>{/if}
</button>

<style>
.follow-btn {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  background: none;
  border: 1px solid var(--border);
  border-radius: 999px;
  padding: 0.4rem 0.8rem;
  cursor: pointer;
  color: inherit;
  font: inherit;
  font-size: 0.82rem;
}

.follow-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.follow-btn.following {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}

.follow-btn.following:hover {
  background: var(--accent-hover);
  border-color: var(--accent-hover);
  color: #fff;
}

.follow-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.follow-btn.compact {
  padding: 0.3rem;
  border: none;
  border-radius: 4px;
  color: #a0a0a0;
}

.follow-btn.compact:hover {
  color: white;
  background: rgba(255, 255, 255, 0.1);
  border-color: transparent;
}

.follow-btn.compact.following {
  background: none;
  color: var(--accent);
}

.follow-btn.compact.following:hover {
  background: rgba(255, 255, 255, 0.1);
}
</style>
