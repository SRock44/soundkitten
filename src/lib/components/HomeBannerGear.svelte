<script lang="ts">
  import { settings } from "../stores/settings.svelte";
  import { listTimezones, saveTimezone } from "../banner/environment";
  import Icon from "./Icon.svelte";

  let {
    timezone = $bindable(undefined),
    variant = "dark",
  }: {
    /** Only relevant while the banner itself is showing -- omit (leave undefined) on the plain-hero placement, where there's no scene to time. */
    timezone?: string;
    /** "dark" sits on top of the animated scene (needs a dark chip + light icon to stay legible over any background); "light" sits on the plain gradient hero, which already reads fine with the app's normal button styling. */
    variant?: "dark" | "light";
  } = $props();

  let open = $state(false);
  let wrapEl: HTMLDivElement | undefined = $state();
  // The auto-detected default (loadTimezone(), via Intl) can be a zone this
  // curated shortlist doesn't include (e.g. Asia/Jakarta, folded into the
  // "Bangkok / Jakarta" entry under Asia/Bangkok) -- add it as an extra
  // option so the <select> always has a matching value instead of silently
  // showing blank/wrong.
  const timezoneOptions = $derived.by(() => {
    const base = listTimezones();
    if (!timezone || base.some((tz) => tz.value === timezone)) return base;
    return [...base, { value: timezone, label: timezone }];
  });

  function onTimezoneChange(e: Event) {
    const tz = (e.target as HTMLSelectElement).value;
    timezone = tz;
    saveTimezone(tz);
  }

  $effect(() => {
    if (!open) return;
    function onDocPointerDown(e: MouseEvent) {
      if (wrapEl && !wrapEl.contains(e.target as Node)) open = false;
    }
    document.addEventListener("mousedown", onDocPointerDown);
    return () => document.removeEventListener("mousedown", onDocPointerDown);
  });
</script>

<div class="gear-wrap" class:dark={variant === "dark"} bind:this={wrapEl}>
  <button class="gear-btn" onclick={() => (open = !open)} aria-label="Home banner settings" title="Home banner settings">
    <Icon name="gear" size={14} />
  </button>
  {#if open}
    <div class="gear-popover">
      <div class="popover-row">
        <span>Home banner</span>
        <button
          class="switch"
          class:on={settings.showHomeBanner}
          role="switch"
          aria-checked={settings.showHomeBanner}
          aria-label="Enable Home banner"
          onclick={() => settings.setShowHomeBanner(!settings.showHomeBanner)}
        >
          <span class="switch-knob"></span>
        </button>
      </div>
      {#if settings.showHomeBanner && timezone !== undefined}
        <label class="popover-row column">
          <span>Timezone</span>
          <select value={timezone} onchange={onTimezoneChange}>
            {#each timezoneOptions as tz (tz.value)}
              <option value={tz.value}>{tz.label}</option>
            {/each}
          </select>
        </label>
      {/if}
    </div>
  {/if}
</div>

<style>
.gear-wrap {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 6;
}

.gear-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border: none;
  border-radius: 50%;
  cursor: pointer;
}

.dark .gear-btn {
  background: rgba(20, 16, 14, 0.55);
  backdrop-filter: blur(6px);
  color: rgba(255, 255, 255, 0.8);
}

.dark .gear-btn:hover {
  color: #fff;
  background: rgba(20, 16, 14, 0.72);
}

.gear-wrap:not(.dark) .gear-btn {
  background: var(--surface);
  color: var(--muted);
}

.gear-wrap:not(.dark) .gear-btn:hover {
  color: var(--fg);
}

.gear-popover {
  position: absolute;
  top: 32px;
  right: 0;
  min-width: 12rem;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
  padding: 0.65rem 0.75rem;
  border-radius: 10px;
  background: var(--surface);
  color: var(--fg);
  box-shadow: 0 12px 28px rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border);
  font-size: 0.82rem;
}

.popover-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
}

.popover-row.column {
  flex-direction: column;
  align-items: stretch;
  gap: 0.3rem;
}

.popover-row select {
  padding: 0.35em 0.5em;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--bg);
  color: inherit;
  font: inherit;
  font-size: 0.8rem;
  max-width: 100%;
}

.switch {
  position: relative;
  width: 34px;
  height: 20px;
  flex-shrink: 0;
  border-radius: 999px;
  border: none;
  background: var(--border);
  cursor: pointer;
  padding: 0;
  transition: background-color 0.15s ease;
}

.switch.on {
  background: var(--accent);
}

.switch-knob {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #fff;
  transition: transform 0.15s ease;
}

.switch.on .switch-knob {
  transform: translateX(14px);
}
</style>
