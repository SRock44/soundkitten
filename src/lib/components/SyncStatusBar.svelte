<script lang="ts">
  import { syncStatus } from "../stores/syncStatus.svelte";
  import Icon from "./Icon.svelte";
</script>

{#if syncStatus.message}
  <div class="sync-bar" class:rate-limited={syncStatus.kind === "rate-limited"} class:error={syncStatus.kind === "error"}>
    {#if syncStatus.kind === "syncing"}
      <span class="spinner"><Icon name="refresh" size={12} /></span>
    {/if}
    <span>{syncStatus.message}</span>
  </div>
{/if}

<style>
.sync-bar {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.4rem 1rem;
  font-size: 0.78rem;
  background: var(--search-bg);
  color: var(--muted);
  border-top: 1px solid var(--border);
  flex-shrink: 0;
}

.sync-bar.rate-limited {
  background: var(--search-bg);
  color: var(--accent);
}

.sync-bar.error {
  background: var(--error-bg);
  color: var(--error-text);
}

.spinner {
  display: flex;
  animation: sync-bar-spin 0.9s linear infinite;
}

@keyframes sync-bar-spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
