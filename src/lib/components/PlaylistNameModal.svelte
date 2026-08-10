<script lang="ts">
  import Icon from "./Icon.svelte";

  let {
    heading,
    confirmLabel,
    initialValue = "",
    onConfirm,
    onClose,
  }: {
    heading: string;
    confirmLabel: string;
    initialValue?: string;
    onConfirm: (title: string) => void;
    onClose: () => void;
  } = $props();

  // Deliberately a one-time seed, not tracked reactively -- this is the
  // component's own editable draft, re-diverging from `initialValue` on
  // every keystroke is the point (a rename modal shouldn't snap back to
  // the original title just because a parent re-render passed the same
  // prop through again).
  // svelte-ignore state_referenced_locally
  let value = $state(initialValue);
  let busy = $state(false);
  let inputEl: HTMLInputElement | undefined = $state();

  $effect(() => {
    inputEl?.focus();
  });

  function submit(e: SubmitEvent) {
    e.preventDefault();
    const trimmed = value.trim();
    if (!trimmed || busy) return;
    busy = true;
    onConfirm(trimmed);
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && !busy && onClose()} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="backdrop" onclick={() => !busy && onClose()}>
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div class="modal" onclick={(e) => e.stopPropagation()}>
    <div class="header">
      <span>{heading}</span>
      <button class="close" onclick={onClose} disabled={busy} aria-label="Close"><Icon name="close" size={13} /></button>
    </div>
    <form onsubmit={submit}>
      <input bind:this={inputEl} bind:value placeholder="Playlist name" disabled={busy} maxlength={255} />
      <div class="actions">
        <button type="button" class="cancel" onclick={onClose} disabled={busy}>Cancel</button>
        <button type="submit" class="confirm" disabled={busy || !value.trim()}>{confirmLabel}</button>
      </div>
    </form>
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
  z-index: 70;
}

.modal {
  background: var(--bg);
  color: var(--fg);
  width: 22rem;
  max-width: 90vw;
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

.close:disabled {
  opacity: 0.5;
  cursor: default;
}

form {
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
}

input {
  padding: 0.55em 0.75em;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--nav-bg);
  color: inherit;
  font: inherit;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}

.cancel,
.confirm {
  padding: 0.45em 1em;
  border-radius: 6px;
  border: none;
  cursor: pointer;
  font: inherit;
  font-weight: 600;
}

.cancel {
  background: none;
  color: var(--muted);
}

.cancel:hover {
  color: var(--fg);
}

.confirm {
  background: var(--accent);
  color: white;
}

.confirm:disabled {
  opacity: 0.5;
  cursor: default;
}
</style>
