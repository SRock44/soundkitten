<script lang="ts">
  import type { Weather } from '../../types'

  /** Lean life silhouettes for city sidewalks — edges only, clear run corridor. */
  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const coat = $derived(winter ? '#3A4A5A' : '#2A3540')
  const coat2 = $derived(winter ? '#5A3A3A' : '#4A2A2A')
  const skin = '#C8A888'
  const dog = $derived(winter ? '#8A7A68' : '#6A5A48')
</script>

<g class="life city-life" aria-hidden="true">
  <!-- Pedestrian left (walking bob) -->
  <g class="sk-walk" transform="translate(22, 118)">
    <circle cx="0" cy="-16" r="2.6" fill={skin} />
    <path d="M-2.5 -13 L2.5 -13 L3.2 -2 L-3.2 -2 Z" fill={coat} />
    {#if winter}
      <ellipse cx="0" cy="-18.5" rx="3.2" ry="1.4" fill="#E8EEF4" />
    {/if}
    <path d="M-2.2 -2 L-1.2 0 L-2.8 0 Z" fill={coat} />
    <path d="M1.4 -2 L3.2 0 L1.8 0 Z" fill={coat} />
    <path d="M-3 -10 L-6 -6" stroke={coat} stroke-width="1.4" stroke-linecap="round" fill="none" />
    <path d="M3 -10 L5.5 -7" stroke={coat} stroke-width="1.4" stroke-linecap="round" fill="none" />
  </g>

  <!-- Dog walker right -->
  <g transform="translate(388, 118)">
    <g class="sk-bob">
      <circle cx="0" cy="-15" r="2.4" fill={skin} />
      <path d="M-2.2 -12.5 L2.2 -12.5 L2.8 -2 L-2.8 -2 Z" fill={coat2} />
      {#if winter}
        <ellipse cx="0" cy="-17.5" rx="3" ry="1.3" fill="#D8E0E8" />
      {/if}
      <path d="M-2 -2 L-3 0" stroke={coat2} stroke-width="1.5" stroke-linecap="round" />
      <path d="M1.6 -2 L2.8 0" stroke={coat2} stroke-width="1.5" stroke-linecap="round" />
      <path d="M2.5 -8 L8 -5" stroke="#6A6A68" stroke-width="0.7" fill="none" />
    </g>
    <!-- dog -->
    <g class="sk-bob" style="animation-delay: -0.4s" transform="translate(10, 0)">
      <ellipse cx="0" cy="-3.5" rx="5" ry="2.6" fill={dog} />
      <circle cx="4.5" cy="-5" r="2.1" fill={dog} />
      <path d="M6.2 -5.5 L8 -4.2" stroke={dog} stroke-width="1" stroke-linecap="round" />
      <path d="M-3.5 -1.2 L-4 0" stroke={dog} stroke-width="1.1" />
      <path d="M-1 -1.2 L-1.2 0" stroke={dog} stroke-width="1.1" />
      <path d="M2 -1.2 L2.2 0" stroke={dog} stroke-width="1.1" />
      <path d="M3.8 -1.2 L4.2 0" stroke={dog} stroke-width="1.1" />
      <path d="M-4.5 -4 L-7 -6" stroke={dog} stroke-width="1.1" stroke-linecap="round" />
    </g>
  </g>

  <!-- Pigeons on curb -->
  <g transform="translate(48, 116)" fill="#6A6E72">
    <ellipse cx="0" cy="0" rx="3.2" ry="1.6" />
    <circle cx="2.6" cy="-1" r="1.3" />
    <path d="M3.6 -1.2 L5 -0.4" stroke="#6A6E72" stroke-width="0.7" />
  </g>
  <g transform="translate(58, 115)" fill="#7A7E82" opacity="0.9">
    <ellipse cx="0" cy="0" rx="2.8" ry="1.4" />
    <circle cx="2.2" cy="-0.8" r="1.1" />
  </g>

  <!-- Distant street crowd silhouettes (mid sidewalk, very small) -->
  <g opacity="0.45" fill="#2A3038">
    <g transform="translate(95, 108)">
      <circle cx="0" cy="-7" r="1.4" />
      <rect x="-1.4" y="-5.5" width="2.8" height="5.5" rx="0.4" />
    </g>
    <g transform="translate(104, 108)">
      <circle cx="0" cy="-6.5" r="1.2" />
      <rect x="-1.2" y="-5" width="2.4" height="5" rx="0.3" />
    </g>
    <g transform="translate(320, 107)">
      <circle cx="0" cy="-7" r="1.3" />
      <rect x="-1.3" y="-5.5" width="2.6" height="5.5" rx="0.4" />
    </g>
  </g>
</g>
