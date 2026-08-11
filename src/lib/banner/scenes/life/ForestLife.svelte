<script lang="ts">
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const deer = $derived(winter ? '#C8B8A0' : '#A87848')
  const deerDark = $derived(winter ? '#8A7A68' : '#6A4830')
</script>

<g class="life forest-life" aria-hidden="true">
  <!-- Deer at left treeline -->
  <g transform="translate(28, 112)">
    <ellipse cx="0" cy="0" rx="8" ry="4.5" fill={deer} />
    <rect x="6" y="-6" width="3.2" height="7" rx="1.2" fill={deer} />
    <circle cx="10.5" cy="-7" r="2.3" fill={deer} />
    <path d="M11.5 -8.5 L12.5 -11" stroke={deer} stroke-width="1" stroke-linecap="round" />
    <path d="M9.5 -9 L8.5 -12.5 M8.5 -12.5 L7.5 -12 M8.5 -12.5 L9.2 -13.2" stroke={deerDark} stroke-width="0.9" fill="none" />
    <path d="M10.8 -9 L11.5 -12.5 M11.5 -12.5 L10.6 -12.2 M11.5 -12.5 L12.2 -13" stroke={deerDark} stroke-width="0.9" fill="none" />
    <path d="M-5 3 L-6 6" stroke={deerDark} stroke-width="1.3" stroke-linecap="round" />
    <path d="M-1 3.5 L-1.2 6" stroke={deerDark} stroke-width="1.3" stroke-linecap="round" />
    <path d="M3 3.5 L3.5 6" stroke={deerDark} stroke-width="1.3" stroke-linecap="round" />
    <path d="M6 3 L7 6" stroke={deerDark} stroke-width="1.3" stroke-linecap="round" />
    <path d="M-7 -1 L-10 -2" stroke={deer} stroke-width="1.2" stroke-linecap="round" />
  </g>

  <!-- Squirrel on log area -->
  <g class="sk-bob" transform="translate(72, 108)" fill="#8A5A38">
    <ellipse cx="0" cy="0" rx="3" ry="2" />
    <circle cx="2.6" cy="-1.2" r="1.6" />
    <path d="M-2.5 -0.5 Q-6 -4 -3.5 1.5" fill="none" stroke="#8A5A38" stroke-width="1.4" />
    <circle cx="3.2" cy="-1.5" r="0.35" fill="#2A2A28" />
  </g>

  <!-- Hiker right edge -->
  <g transform="translate(402, 118)">
    <circle cx="0" cy="-16" r="2.4" fill="#C4A080" />
    <path d="M-2.4 -13.5 L2.4 -13.5 L3 -2 L-3 -2 Z" fill="#3A5A48" />
    <path d="M-3.5 -11 L-5.5 -6" stroke="#5A3A28" stroke-width="1.6" stroke-linecap="round" />
    <ellipse cx="-5.8" cy="-5" rx="2.2" ry="2.8" fill="#5A3A28" />
    <path d="M-2 -2 L-3 0" stroke="#2A3A30" stroke-width="1.4" stroke-linecap="round" />
    <path d="M1.6 -2 L2.6 0" stroke="#2A3A30" stroke-width="1.4" stroke-linecap="round" />
    {#if winter}
      <ellipse cx="0" cy="-18.2" rx="3" ry="1.2" fill="#E8EEF2" />
    {/if}
  </g>

  <!-- Bird in canopy -->
  <g class="sk-flap" transform="translate(200, 52)" fill={weather === 'spring' ? '#E87890' : '#4A6A48'}>
    <ellipse cx="0" cy="0" rx="2.8" ry="1.2" />
    <path d="M-1.5 0 L-5 -2 L-1.5 0.6 Z" />
    <path d="M1.5 0 L5 -1.8 L1.5 0.6 Z" />
  </g>

  <!-- Rabbit (spring/summer) -->
  {#if weather === 'spring' || weather === 'summer'}
    <g transform="translate(350, 114)" fill="#D8D0C4">
      <ellipse cx="0" cy="0" rx="3.5" ry="2.4" />
      <circle cx="3" cy="-1.5" r="1.8" />
      <path d="M2.2 -3 L1.6 -7" stroke="#D8D0C4" stroke-width="1.1" stroke-linecap="round" />
      <path d="M3.4 -3 L3.8 -6.5" stroke="#D8D0C4" stroke-width="1.1" stroke-linecap="round" />
      <circle cx="3.6" cy="-1.8" r="0.35" fill="#2A2A28" />
    </g>
  {/if}
</g>
