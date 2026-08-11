<script lang="ts">
  import { TIME_SKY } from '../palettes'
  import type { Time } from '../types'

  let {
    time,
    uid,
    playing = true,
  }: { time: Time; uid: string; playing?: boolean } = $props()

  const sky = $derived(TIME_SKY[time])
  const gradId = $derived(`sky-${uid}`)
  const glowId = $derived(`glow-${uid}`)
  const rayId = $derived(`ray-${uid}`)
  const bandId = $derived(`band-${uid}`)
  const crescent = $derived((uid.charCodeAt(0) + uid.length) % 2 === 0)

  const stars = [
    [18, 12, 1.5],
    [42, 28, 0.8],
    [68, 16, 1.2],
    [95, 36, 0.7],
    [118, 10, 1.4],
    [148, 24, 0.9],
    [172, 14, 1.1],
    [198, 32, 0.75],
    [222, 18, 1.3],
    [250, 40, 0.7],
    [278, 12, 1.0],
    [305, 26, 0.85],
    [332, 16, 1.2],
    [358, 34, 0.7],
    [385, 20, 1.1],
    [405, 42, 0.8],
    [55, 48, 0.6],
    [140, 46, 0.7],
    [230, 50, 0.65],
    [370, 48, 0.75],
  ] as const
</script>

<g class="sky" aria-hidden="true">
  <defs>
    <linearGradient id={gradId} x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color={sky.top} />
      <stop offset="28%" stop-color={sky.upper} />
      <stop offset="55%" stop-color={sky.mid} />
      <stop offset="78%" stop-color={sky.lower} />
      <stop offset="100%" stop-color={sky.bottom} />
    </linearGradient>
    <radialGradient id={glowId} cx="50%" cy="50%" r="50%">
      <stop offset="0%" stop-color={sky.sunGlow} stop-opacity="0.9" />
      <stop offset="40%" stop-color={sky.sunGlow} stop-opacity="0.35" />
      <stop offset="75%" stop-color={sky.horizon} stop-opacity="0.12" />
      <stop offset="100%" stop-color={sky.horizon} stop-opacity="0" />
    </radialGradient>
    <linearGradient id={bandId} x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color={sky.horizon} stop-opacity="0" />
      <stop offset="45%" stop-color={sky.horizon} stop-opacity="0.55" />
      <stop offset="100%" stop-color={sky.accent} stop-opacity="0.35" />
    </linearGradient>
    <linearGradient id={rayId} x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color={sky.sunGlow} stop-opacity="0.35" />
      <stop offset="100%" stop-color={sky.sunGlow} stop-opacity="0" />
    </linearGradient>
  </defs>

  <rect width="420" height="140" fill={`url(#${gradId})`} />

  {#if time === 'sunrise'}
    <!-- Warm horizon bloom -->
    <ellipse cx="90" cy="120" rx="180" ry="36" fill={`url(#${bandId})`} />
    <ellipse cx="90" cy="118" rx="110" ry="18" fill={sky.horizon} opacity="0.35" />

    <!-- Soft god rays -->
    <g opacity="0.28" fill={`url(#${rayId})`}>
      <path d="M72 100 L20 0 L48 0 Z" />
      <path d="M72 100 L70 0 L95 0 Z" />
      <path d="M72 100 L120 0 L150 0 Z" />
      <path d="M72 100 L175 0 L210 0 Z" />
    </g>

    <circle cx="72" cy="100" r="58" fill={`url(#${glowId})`} />
    <circle cx="72" cy="100" r="28" fill={sky.sunGlow} opacity="0.45" />
    <circle cx="72" cy="100" r="15" fill={sky.sun} />
    <circle cx="72" cy="100" r="7" fill={sky.accent} opacity="0.7" />

    <!-- Lit cloud undersides -->
    <g>
      <ellipse cx="175" cy="48" rx="32" ry="11" fill={sky.cloud} opacity="0.85" />
      <ellipse cx="195" cy="44" rx="16" ry="9" fill={sky.cloudShade} opacity="0.55" />
      <ellipse cx="160" cy="46" rx="14" ry="8" fill={sky.cloud} opacity="0.7" />
      <ellipse cx="175" cy="54" rx="26" ry="5" fill={sky.horizon} opacity="0.4" />
      <ellipse cx="310" cy="36" rx="28" ry="10" fill={sky.cloud} opacity="0.8" />
      <ellipse cx="328" cy="32" rx="14" ry="8" fill={sky.cloudShade} opacity="0.5" />
      <ellipse cx="310" cy="42" rx="22" ry="4" fill={sky.horizon} opacity="0.35" />
      <ellipse cx="380" cy="58" rx="20" ry="7" fill={sky.cloud} opacity="0.55" />
    </g>
    <!-- Specks of light in haze -->
    <g fill={sky.accent} opacity="0.45">
      <circle cx="40" cy="88" r="1.2" />
      <circle cx="130" cy="72" r="0.9" />
      <circle cx="200" cy="90" r="1.1" />
    </g>
  {:else if time === 'day'}
    <circle cx="338" cy="34" r="42" fill={`url(#${glowId})`} />
    <circle cx="338" cy="34" r="14" fill={sky.sun} />
    <g fill={sky.cloud} opacity="0.9">
      <ellipse cx="60" cy="42" rx="30" ry="11" />
      <ellipse cx="78" cy="36" rx="16" ry="10" />
      <ellipse cx="44" cy="38" rx="14" ry="8" />
      <ellipse cx="180" cy="28" rx="26" ry="10" />
      <ellipse cx="196" cy="22" rx="14" ry="9" />
      <ellipse cx="390" cy="40" rx="18" ry="8" opacity="0.75" />
    </g>
  {:else if time === 'sunset'}
    <!-- Dramatic stacked horizon bands -->
    <ellipse cx="340" cy="122" rx="200" ry="40" fill={`url(#${bandId})`} />
    <ellipse cx="300" cy="118" rx="160" ry="22" fill={sky.horizon} opacity="0.4" />
    <ellipse cx="360" cy="115" rx="90" ry="14" fill={sky.accent} opacity="0.25" />

    <!-- Long god rays -->
    <g opacity="0.32" fill={`url(#${rayId})`}>
      <path d="M345 96 L250 0 L280 0 Z" />
      <path d="M345 96 L300 0 L330 0 Z" />
      <path d="M345 96 L345 0 L375 0 Z" />
      <path d="M345 96 L390 0 L420 0 Z" />
      <path d="M345 96 L200 0 L230 0 Z" />
    </g>

    <circle cx="345" cy="96" r="72" fill={`url(#${glowId})`} />
    <circle cx="345" cy="96" r="38" fill={sky.sunGlow} opacity="0.4" />
    <circle cx="345" cy="96" r="20" fill={sky.sun} />
    <circle cx="345" cy="96" r="9" fill={sky.accent} opacity="0.75" />

    <!-- Streak / shelf clouds catching color -->
    <g>
      <ellipse cx="55" cy="36" rx="50" ry="5" fill={sky.cloud} opacity="0.55" transform="rotate(-8 55 36)" />
      <ellipse cx="55" cy="39" rx="42" ry="2.5" fill={sky.horizon} opacity="0.45" transform="rotate(-8 55 39)" />
      <ellipse cx="150" cy="48" rx="40" ry="4" fill={sky.cloudShade} opacity="0.65" transform="rotate(-5 150 48)" />
      <ellipse cx="150" cy="51" rx="34" ry="2" fill={sky.lower} opacity="0.4" transform="rotate(-5 150 51)" />
      <ellipse cx="230" cy="30" rx="44" ry="4.5" fill={sky.cloud} opacity="0.5" transform="rotate(-9 230 30)" />
      <ellipse cx="230" cy="33" rx="36" ry="2.2" fill={sky.horizon} opacity="0.4" transform="rotate(-9 230 33)" />
      <ellipse cx="100" cy="62" rx="30" ry="3" fill={sky.cloudShade} opacity="0.4" transform="rotate(-3 100 62)" />
    </g>
  {:else}
    <!-- Deeper night + city light pollution band -->
    <rect y="95" width="420" height="45" fill={sky.horizon} opacity="0.18" />
    <ellipse cx="210" cy="125" rx="200" ry="22" fill="#5A6A90" opacity="0.12" />

    <g fill={sky.accent}>
      {#each stars as [cx, cy, r], i}
        <circle
          class:twinkle={playing && i % 3 !== 2}
          {cx}
          {cy}
          {r}
          opacity={0.5 + (i % 4) * 0.12}
          style={playing && i % 3 !== 2 ? `animation-delay: ${-(i % 5) * 0.55}s` : undefined}
        />
      {/each}
    </g>
    <!-- A few brighter “planets” -->
    <circle cx="88" cy="22" r="1.8" fill="#F0F4FF" opacity="0.95" />
    <circle cx="300" cy="18" r="1.5" fill="#E8ECFF" opacity="0.9" />

    <circle cx="352" cy="34" r="30" fill={`url(#${glowId})`} />
    <circle cx="352" cy="34" r="14" fill={sky.sun} />
    {#if crescent}
      <circle cx="358" cy="29" r="12" fill={sky.mid} />
    {:else}
      <circle cx="346" cy="30" r="2.2" fill={sky.cloudShade} opacity="0.4" />
      <circle cx="356" cy="38" r="1.5" fill={sky.cloudShade} opacity="0.35" />
      <circle cx="350" cy="40" r="1" fill={sky.cloudShade} opacity="0.3" />
    {/if}
  {/if}
</g>

<style>
  .twinkle {
    animation: sk-twinkle 2.8s ease-in-out infinite;
  }

  :global(.banner.paused) .twinkle {
    animation-play-state: paused;
  }

  @media (prefers-reduced-motion: reduce) {
    .twinkle {
      animation: none;
    }
  }
</style>
