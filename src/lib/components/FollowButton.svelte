<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { following } from "../stores/following.svelte";
  import Icon from "./Icon.svelte";

  let { userId, permalinkUrl }: { userId: number; permalinkUrl: string | null } = $props();

  let isFollowing = $derived(following.has(userId));

  // SoundCloud's follow/unfollow endpoint is behind DataDome bot-protection
  // and consistently 403s even with a valid oauth token (confirmed live) --
  // so this button can only reflect real follow state, not change it. It
  // sends the user to soundcloud.com to actually follow/unfollow, the same
  // graceful-degradation pattern used for DRM-locked tracks.
  function openOnSoundCloud(e: MouseEvent) {
    e.stopPropagation();
    if (permalinkUrl) openUrl(permalinkUrl);
  }
</script>

<button
  class="follow-btn"
  class:following={isFollowing}
  onclick={openOnSoundCloud}
  disabled={!permalinkUrl}
  title={isFollowing ? "Unfollow on soundcloud.com" : "Follow on soundcloud.com"}
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
