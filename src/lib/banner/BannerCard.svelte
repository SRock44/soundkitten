<script lang="ts">
  import RunningCat from './cat/RunningCat.svelte'
  import TimeSky from './scenes/TimeSky.svelte'
  import WeatherFX from './scenes/WeatherFX.svelte'
  import LocationScene from './scenes/LocationScene.svelte'
  import { LABELS } from './combinations'
  import { GROUND_Y, type Location, type Time, type Weather } from './types'

  let {
    time,
    weather,
    location,
    showLabel = true,
    forcePlaying,
    onCatExitRight,
    alignCatEntry = false,
    /** Disambiguate clipPath + cat director when multiple cards share a combo */
    instanceId,
  }: {
    time: Time
    weather: Weather
    location: Location
    showLabel?: boolean
    /** When set, overrides intersection-based pause (preview card stays live) */
    forcePlaying?: boolean
    onCatExitRight?: () => void
    /** Preview: spawn the cat at the left edge on mount */
    alignCatEntry?: boolean
    instanceId?: string
  } = $props()

  let root: HTMLElement | undefined = $state()
  let visible = $state(false)

  const uid = $derived(`${time}-${weather}-${location}`)
  const instance = $derived(instanceId ?? uid)
  const title = $derived(
    `${LABELS.time[time]} · ${LABELS.weather[weather]} · ${LABELS.location[location]}`,
  )
  const playing = $derived(forcePlaying ?? visible)

  /** How far below the cat's paw line (GROUND_Y) the visible crop extends --
   * a small buffer of actual ground/floor, not zero (a hard cut right at
   * the paws looked too tight) and not the full ~22 units the art has
   * below it either (that was too much bare floor). */
  const FLOOR_MARGIN = 10
  const viewBottom = GROUND_Y + FLOOR_MARGIN

  $effect(() => {
    if (forcePlaying !== undefined) return
    const el = root
    if (!el) return
    const io = new IntersectionObserver(
      ([entry]) => {
        visible = entry.isIntersecting
      },
      { rootMargin: '80px', threshold: 0.05 },
    )
    io.observe(el)
    return () => io.disconnect()
  })
</script>

<article
  class="banner"
  class:paused={!playing}
  bind:this={root}
  style:content-visibility={forcePlaying ? 'visible' : 'auto'}
  style:contain-intrinsic-size="auto 190px"
  aria-label={title}
>
  <svg
    class="scene"
    viewBox="0 0 420 {viewBottom}"
    width="420"
    height={viewBottom}
    preserveAspectRatio="xMidYMax slice"
    role="img"
    aria-hidden="true"
  >
    <!-- The visible window ends just past GROUND_Y (the cat's paw line) --
         instead of the full 140-tall art (which has ~22 units of empty
         ground below the paws), so the ground the cat runs on always sits
         close to the bottom edge of the banner instead of leaving a big gap
         of bare floor, at any window width. Combined with xMidYMax slice
         above, cropping for a wide window only ever eats into the sky
         (still never the cat) -- the source crop itself already excludes
         most of the extra floor, no separate CSS-height buffer trick
         needed for that part. -->
    <defs>
      <clipPath id="banner-clip-{instance}">
        <rect x="0" y="0" width="420" height={viewBottom} />
      </clipPath>
    </defs>
    <g clip-path="url(#banner-clip-{instance})">
      <TimeSky {time} uid={instance} {playing} />
      <LocationScene {location} {weather} {time} />

      <g transform="translate(0 {GROUND_Y})">
        <RunningCat
          {playing}
          id={`cat-${instance}`}
          onExitRight={onCatExitRight}
          alignEntry={alignCatEntry}
        />
      </g>

      <!-- Particles only while visible — huge win with 64 cards -->
      <WeatherFX {weather} {playing} />
    </g>
  </svg>

  {#if showLabel}
    <p class="label">{title}</p>
  {/if}
</article>

<style>
  .scene {
    display: block;
    width: 100%;
    /* Inherits from .banner (100%), which inherits from whatever definite
       height the actual consumer sets (HomeBanner.svelte's .home-banner
       uses a clamp() that scales gently with viewport width) -- no
       hardcoded px value here to keep in sync by hand. The viewBox above
       (not a CSS-height trick) is what keeps the floor from showing too
       much, independent of whatever this height ends up being. */
    height: 100%;
    overflow: hidden;
    background: #111;
    clip-path: inset(0);
    /* Cleaner edges when the cat scrolls */
    shape-rendering: geometricPrecision;
  }

  .banner {
    contain: layout paint style;
    width: 100%;
    height: 100%;
    margin: 0;
    overflow: hidden;
  }

  .label {
    margin: 8px 0 0;
    font-size: 12px;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: var(--muted);
  }
</style>
