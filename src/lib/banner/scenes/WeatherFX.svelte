<script lang="ts">
  import {
    FALL_LEAF_COLORS,
    SPRING_PETAL_COLORS,
    SUMMER_MOTE_COLORS,
    WEATHER_PARTICLE,
  } from '../palettes'
  import type { Weather } from '../types'

  let { weather, playing = true }: { weather: Weather; playing?: boolean } = $props()
  const color = $derived(WEATHER_PARTICLE[weather])

  // Sparse particles — visual density via color variety, not node count
  const leaves = [
    { x: 28, dur: 3.2, delay: -0.4, rx: 5, ry: 2.3, rot: 18, c: 0 },
    { x: 78, dur: 3.6, delay: -1.2, rx: 4.2, ry: 2, rot: 70, c: 2 },
    { x: 132, dur: 3.0, delay: -2.0, rx: 5.4, ry: 2.5, rot: 120, c: 3 },
    { x: 188, dur: 3.5, delay: -0.7, rx: 3.8, ry: 1.8, rot: 40, c: 1 },
    { x: 248, dur: 3.8, delay: -1.6, rx: 4.8, ry: 2.2, rot: 95, c: 4 },
    { x: 308, dur: 3.1, delay: -2.4, rx: 4.0, ry: 2.0, rot: 150, c: 5 },
    { x: 362, dur: 3.4, delay: -0.9, rx: 5.0, ry: 2.4, rot: 55, c: 6 },
    { x: 400, dur: 3.7, delay: -1.8, rx: 3.6, ry: 1.7, rot: 110, c: 0 },
  ]

  const flakes = [
    { x: 24, drift: 10, dur: 2.8, delay: -0.2, r: 1.2 },
    { x: 70, drift: -8, dur: 3.2, delay: -0.9, r: 1.6 },
    { x: 118, drift: 18, dur: 2.6, delay: -1.5, r: 1.0 },
    { x: 168, drift: 6, dur: 3.4, delay: -0.5, r: 1.5 },
    { x: 220, drift: -14, dur: 2.9, delay: -2.0, r: 1.1 },
    { x: 272, drift: 20, dur: 3.1, delay: -1.1, r: 1.7 },
    { x: 324, drift: -6, dur: 2.7, delay: -1.7, r: 0.9 },
    { x: 372, drift: 12, dur: 3.3, delay: -0.6, r: 1.4 },
    { x: 404, drift: 8, dur: 3.0, delay: -2.3, r: 1.2 },
    { x: 96, drift: -16, dur: 3.5, delay: -2.6, r: 1.3 },
  ]

  const petals = [
    { x: 40, dur: 3.4, delay: -0.3, rx: 3.2, ry: 1.9, c: 0 },
    { x: 100, dur: 3.8, delay: -1.0, rx: 2.8, ry: 1.7, c: 1 },
    { x: 170, dur: 3.2, delay: -1.7, rx: 3.5, ry: 2.0, c: 2 },
    { x: 240, dur: 3.6, delay: -0.6, rx: 3.0, ry: 1.8, c: 3 },
    { x: 310, dur: 3.3, delay: -2.1, rx: 3.4, ry: 2.0, c: 4 },
    { x: 380, dur: 3.7, delay: -1.3, rx: 2.7, ry: 1.6, c: 0 },
  ]

  const motes = [
    { x: 55, dur: 4.4, delay: -0.4, r: 1.1, c: 0 },
    { x: 145, dur: 5.0, delay: -1.4, r: 0.8, c: 1 },
    { x: 235, dur: 3.9, delay: -2.2, r: 1.3, c: 2 },
    { x: 325, dur: 4.6, delay: -0.8, r: 0.9, c: 3 },
    { x: 390, dur: 4.2, delay: -1.9, r: 1.0, c: 0 },
  ]
</script>

{#if playing}
  {#if weather === 'fall'}
    <g class="wx" aria-hidden="true">
      {#each leaves as leaf}
        <ellipse
          class="flake leaf"
          cx="0"
          cy="0"
          rx={leaf.rx}
          ry={leaf.ry}
          fill={FALL_LEAF_COLORS[leaf.c]}
          transform="rotate({leaf.rot})"
          style="--x: {leaf.x}px; --dur: {leaf.dur}s; --delay: {leaf.delay}s"
        />
      {/each}
    </g>
  {:else if weather === 'winter'}
    <g class="wx" aria-hidden="true">
      {#each flakes as flake}
        <circle
          class="flake snow"
          cx="0"
          cy="0"
          r={flake.r}
          fill={color}
          opacity="0.75"
          style="--x: {flake.x}px; --drift: {flake.drift}px; --dur: {flake.dur}s; --delay: {flake.delay}s"
        />
      {/each}
    </g>
  {:else if weather === 'spring'}
    <g class="wx" aria-hidden="true">
      {#each petals as petal}
        <ellipse
          class="flake petal"
          cx="0"
          cy="0"
          rx={petal.rx}
          ry={petal.ry}
          fill={SPRING_PETAL_COLORS[petal.c]}
          style="--x: {petal.x}px; --dur: {petal.dur}s; --delay: {petal.delay}s"
        />
      {/each}
    </g>
  {:else}
    <g class="wx" aria-hidden="true">
      {#each motes as mote}
        <circle
          class="flake mote"
          cx="0"
          cy="0"
          r={mote.r}
          fill={SUMMER_MOTE_COLORS[mote.c]}
          opacity="0.55"
          style="--x: {mote.x}px; --drift: 14px; --dur: {mote.dur}s; --delay: {mote.delay}s"
        />
      {/each}
      <circle class="sparkle" cx="90" cy="48" r="1.3" fill="#FFF8D8" />
      <circle class="sparkle s1" cx="220" cy="36" r="1.2" fill="#FFF8D8" />
      <circle class="sparkle s2" cx="350" cy="55" r="1.4" fill="#FFF8D8" />
    </g>
  {/if}
{/if}

<style>
  .flake {
    animation: sk-fall var(--dur) linear infinite;
    animation-delay: var(--delay);
  }

  .leaf,
  .petal {
    animation-name: sk-fall-spin;
  }

  .snow,
  .mote {
    animation-name: sk-fall-drift;
  }

  .sparkle {
    animation: sk-sparkle 2.4s ease-in-out infinite;
  }
  .s1 {
    animation-delay: -0.8s;
  }
  .s2 {
    animation-delay: -1.5s;
  }

  @media (prefers-reduced-motion: reduce) {
    .flake,
    .sparkle {
      animation: none;
      display: none;
    }
  }
</style>
