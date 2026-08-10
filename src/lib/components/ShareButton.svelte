<script lang="ts">
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import Icon from "./Icon.svelte";

  let { url, label = "Share", compact = false }: { url: string | null; label?: string; compact?: boolean } = $props();

  let copied = $state(false);
  let timeoutId: ReturnType<typeof setTimeout> | undefined;

  async function share(e: MouseEvent) {
    e.stopPropagation();
    if (!url) return;
    try {
      await writeText(url);
      copied = true;
      clearTimeout(timeoutId);
      timeoutId = setTimeout(() => (copied = false), 1800);
    } catch (err) {
      console.error("failed to copy link", err);
    }
  }
</script>

<button class="share-btn" class:compact class:copied onclick={share} disabled={!url} aria-label={copied ? "Copied!" : label} title="Copy SoundCloud link">
  <Icon name={copied ? "check" : "share"} size={compact ? 15 : 14} />
  {#if !compact}<span>{copied ? "Copied!" : label}</span>{/if}
</button>

<style>
.share-btn {
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

.share-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.share-btn.copied {
  border-color: var(--accent);
  color: var(--accent);
}

.share-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

/* Mirrors FollowButton.svelte's .compact treatment -- icon-only, no
   border, for tight action rows like FeedPost's action bar. */
.share-btn.compact {
  padding: 0.4rem;
  border: none;
  border-radius: 6px;
  color: var(--muted);
}

.share-btn.compact:hover {
  color: var(--fg);
  background: var(--row-hover);
  border-color: transparent;
}

.share-btn.compact.copied {
  color: var(--accent);
  background: none;
}
</style>
