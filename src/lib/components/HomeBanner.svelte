<script lang="ts">
  import BannerCard from "../banner/BannerCard.svelte";
  import FilterVeil from "../banner/FilterVeil.svelte";
  import { LOCATIONS, type Location } from "../banner/types";
  import { computeSeason, computeTime, loadTimezone } from "../banner/environment";
  import { viewMode } from "../stores/viewMode.svelte";
  import Icon from "./Icon.svelte";
  import HomeBannerGear from "./HomeBannerGear.svelte";
  import type { Profile } from "../types";

  let { me, greeting }: { me: Profile | null; greeting: string } = $props();

  /** Real time/season change slowly -- no need to recompute more than this. */
  const CLOCK_TICK_MS = 60_000;

  function shuffledLocations(): Location[] {
    const arr = [...LOCATIONS];
    for (let i = arr.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [arr[i], arr[j]] = [arr[j], arr[i]];
    }
    return arr;
  }

  let order = $state(shuffledLocations());
  let orderIdx = $state(0);
  let location = $derived(order[orderIdx]);

  let veilPulse = $state(0);
  let veilOn = $state(false);
  let veilTimer: ReturnType<typeof setTimeout> | undefined;

  /** Wired to RunningCat's onExitRight (via BannerCard) -- fires once per
   * lap, exactly when the cat has fully left frame on the right and wrapped
   * back to the left edge. Every lap gets a new place, "quickly switching
   * between scenes" as intended, always timed to a moment nothing is
   * mid-frame. */
  function onCatExitRight() {
    orderIdx += 1;
    if (orderIdx >= order.length) {
      // Reshuffle for the next lap through all 10 -- keeps it from ever
      // repeating the same location twice in a row (the just-shown one was
      // last in the old order, so it can't land first in the new one... but
      // just in case a reshuffle puts it right back at the front, nudge it).
      const justShown = order[order.length - 1];
      let next = shuffledLocations();
      if (next[0] === justShown) [next[0], next[1]] = [next[1], next[0]];
      order = next;
      orderIdx = 0;
    }
    veilPulse += 1;
    veilOn = true;
    clearTimeout(veilTimer);
    veilTimer = setTimeout(() => (veilOn = false), 420);
  }

  let now = $state(new Date());
  // No geolocation permission is ever requested -- see environment.ts's
  // loadTimezone(). Defaults to the OS/browser's own configured timezone,
  // user-overridable via the gear below (HomeBannerGear.svelte), which
  // also persists the choice.
  let timezone = $state(loadTimezone());

  $effect(() => {
    const id = setInterval(() => (now = new Date()), CLOCK_TICK_MS);
    return () => clearInterval(id);
  });

  let time = $derived(computeTime(now, timezone));
  let season = $derived(computeSeason(now, timezone));
</script>

<div class="home-banner">
  <!-- instanceId is load-bearing, not cosmetic: without it BannerCard falls
       back to `${time}-${weather}-${location}` as the cat's director id,
       which changes on every scene swap. catDirector.ts keys its running
       actors by that id, so a changing id tears down and recreates the
       actor on every swap instead of continuing the same one -- the new
       actor's position is a fresh wall-clock-derived spot, occasionally
       already near the wrap point, which could immediately re-fire
       onCatExitRight and cascade through several scenes in a row before
       settling. A fixed id keeps it the same actor across every swap. -->
  <BannerCard {time} weather={season} {location} showLabel={false} {onCatExitRight} instanceId="home" />
  {#if veilOn}
    <FilterVeil pulse={veilPulse} />
  {/if}

  <HomeBannerGear bind:timezone variant="dark" />

  <div class="overlay">
    <div>
      <p class="hero-eyebrow">{greeting}</p>
      <h1>{me?.username ?? "Welcome back"}</h1>
    </div>
    <div class="view-toggle" role="group" aria-label="Playlist display">
      <button class:active={viewMode.playlistView === "tiles"} onclick={() => viewMode.setPlaylistView("tiles")} aria-label="Tile view" title="Tile view">
        <Icon name="grid" size={15} />
      </button>
      <button class:active={viewMode.playlistView === "rows"} onclick={() => viewMode.setPlaylistView("rows")} aria-label="Row view" title="Row view">
        <Icon name="list" size={15} />
      </button>
    </div>
  </div>
</div>

<style>
.home-banner {
  position: relative;
  /* Always full width, no cap -- instead the HEIGHT now scales gently with
     the viewport too (via clamp, not a fixed px value), so a wider window
     doesn't just stretch the same fixed-height crop further and further
     (which is what kept "looking bad once expanded" no matter what width
     cap got picked -- the crop itself was getting more aggressive the
     whole time, the cap only ever delayed where that became visible). 17vw
     roughly matches the ~190px that looked right at the app's default
     ~1100px window; clamp keeps it from getting too short on a narrow
     window or absurdly tall on a very wide one. */
  width: 100%;
  height: clamp(170px, 17vw, 240px);
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.18);
  user-select: none;
}

.overlay {
  position: absolute;
  inset: 0;
  z-index: 5;
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.7rem 0.9rem;
  pointer-events: none;
}

.overlay :global(button) {
  pointer-events: auto;
}

.hero-eyebrow {
  margin: 0 0 0.2rem;
  font-size: 0.72rem;
  font-weight: 700;
  color: rgba(255, 255, 255, 0.75);
  text-transform: uppercase;
  letter-spacing: 0.06em;
  text-shadow: 0 1px 4px rgba(0, 0, 0, 0.4);
}

.overlay h1 {
  margin: 0;
  font-size: 1.4rem;
  font-weight: 800;
  letter-spacing: -0.02em;
  color: #fff;
  text-shadow: 0 1px 6px rgba(0, 0, 0, 0.45);
}

.view-toggle {
  display: flex;
  gap: 0.2rem;
  background: rgba(20, 16, 14, 0.55);
  backdrop-filter: blur(6px);
  border-radius: 8px;
  padding: 0.2rem;
  flex-shrink: 0;
}

.view-toggle button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 26px;
  border: none;
  background: none;
  border-radius: 6px;
  color: rgba(255, 255, 255, 0.7);
  cursor: pointer;
}

.view-toggle button:hover:not(.active) {
  color: #fff;
}

.view-toggle button.active {
  background: rgba(255, 255, 255, 0.18);
  color: #fff;
}
</style>
