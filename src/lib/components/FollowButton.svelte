<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { api } from "../api";
  import { following } from "../stores/following.svelte";
  import { officialAuth } from "../stores/officialAuth.svelte";
  import Icon from "./Icon.svelte";

  let { userId, permalinkUrl }: { userId: number; permalinkUrl: string | null } = $props();

  let isFollowing = $derived(following.has(userId));
  let busy = $state(false);

  // Unofficial-API follow writes are DataDome-blocked (confirmed live), but
  // official OAuth's /me/followings/{id} works cleanly -- see
  // docs/oauth-migration.md. Connects lazily on first use rather than
  // upfront, and falls back to opening soundcloud.com if the user declines
  // to connect or the write itself fails for any reason, same
  // graceful-degradation pattern as before this existed.
  async function toggleFollow(e: MouseEvent) {
    e.stopPropagation();
    if (busy) return;
    busy = true;
    const next = !isFollowing;
    try {
      const connected = await officialAuth.ensureConnected();
      if (!connected) {
        if (permalinkUrl) openUrl(permalinkUrl);
        return;
      }
      if (next) await api.followUserV2(userId);
      else await api.unfollowUserV2(userId);
      following.set(userId, next);
    } catch {
      if (permalinkUrl) openUrl(permalinkUrl);
    } finally {
      busy = false;
    }
  }
</script>

<button
  class="follow-btn"
  class:following={isFollowing}
  onclick={toggleFollow}
  disabled={busy || (!permalinkUrl && !officialAuth.connected)}
  title={isFollowing ? "Unfollow" : "Follow"}
>
  <Icon name={isFollowing ? "user-filled" : "user"} size={14} />
  <span>{isFollowing ? "Following" : "Follow"}</span>
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
</style>
