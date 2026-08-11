<script lang="ts">
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const summer = $derived(weather === 'summer')
  const skin = '#C4A080'
  const wetsuit = $derived(winter ? '#3A4A58' : '#2A3540')
  const board = $derived(summer ? '#E8C848' : winter ? '#A8B4C0' : '#D4A858')
  const towel = $derived(winter ? '#8A9AA8' : '#C84858')
</script>

<g class="life beach-life" aria-hidden="true">
  <!-- Surfer left edge (outside corridor) -->
  <g transform="translate(36, 116)">
    <ellipse cx="10" cy="2" rx="14" ry="2.2" fill={board} opacity="0.9" />
    <ellipse cx="10" cy="1" rx="12" ry="1.4" fill="#fff" opacity="0.15" />
    <g class="sk-bob">
      <circle cx="0" cy="-14" r="2.3" fill={skin} />
      <path d="M-2.2 -11.5 L2.2 -11.5 L2.6 -2 L-2.6 -2 Z" fill={wetsuit} />
      <path d="M-2 -2 L-3.5 1" stroke={wetsuit} stroke-width="1.4" stroke-linecap="round" />
      <path d="M1.8 -2 L4 1" stroke={wetsuit} stroke-width="1.4" stroke-linecap="round" />
      <path d="M2.4 -9 L8 -6" stroke={skin} stroke-width="1.3" stroke-linecap="round" />
      {#if winter}
        <ellipse cx="0" cy="-16.2" rx="2.8" ry="1.1" fill="#E8EEF4" />
      {/if}
    </g>
  </g>

  <!-- Beachgoer right edge -->
  <g transform="translate(392, 118)">
    <ellipse cx="-6" cy="-1" rx="7" ry="2" fill={towel} opacity="0.75" />
    <circle cx="0" cy="-12" r="2.2" fill={skin} />
    <path d="M-2.4 -9.5 L2.4 -9.5 L2.8 -1 L-2.8 -1 Z" fill={winter ? '#5A6A78' : '#3A7A98'} />
    <path d="M-2 -1 L-3 1" stroke="#3A3A38" stroke-width="1.3" stroke-linecap="round" />
    <path d="M1.6 -1 L2.8 1" stroke="#3A3A38" stroke-width="1.3" stroke-linecap="round" />
    <path d="M-3 -7 L-6 -4" stroke={skin} stroke-width="1.2" stroke-linecap="round" />
    {#if summer}
      <ellipse cx="0" cy="-14.2" rx="3.2" ry="1.2" fill="#E8C070" />
    {:else if winter}
      <ellipse cx="0" cy="-14" rx="2.8" ry="1.1" fill="#F0F4F8" />
    {/if}
  </g>

  <!-- Sandcastle hint (left, near hut) -->
  <g transform="translate(68, 116)" fill="#D8C090" opacity="0.85">
    <rect x="-6" y="-2" width="12" height="4" rx="0.5" />
    <rect x="-4" y="-7" width="3.5" height="5" rx="0.3" />
    <rect x="1" y="-6" width="3" height="4" rx="0.3" />
    <path d="M-4 -7 L-2.2 -10 L-0.5 -7 Z" />
    <path d="M1 -6 L2.5 -8.5 L4 -6 Z" />
    <circle cx="-2.2" cy="-10.5" r="0.7" fill="#C84848" opacity="0.7" />
  </g>

  <!-- Seagulls (sk-flap) — high, clear of corridor walkers -->
  <g class="sk-flap" transform="translate(120, 48)" fill="#E8E4DC" opacity="0.85">
    <ellipse cx="0" cy="0" rx="2.8" ry="1.1" />
    <path d="M-1.5 0 L-8 -2.5 L-1.5 0.6 Z" />
    <path d="M1.5 0 L8 -2 L1.5 0.6 Z" />
    <circle cx="2.4" cy="-0.3" r="0.9" />
    <path d="M3 -0.4 L4.5 0.2" stroke="#E8C070" stroke-width="0.7" />
  </g>
  <g class="sk-flap" style="animation-delay: -0.35s" transform="translate(300, 40)" fill="#D8D4CC" opacity="0.8">
    <ellipse cx="0" cy="0" rx="2.4" ry="1" />
    <path d="M-1.2 0 L-7 -2 L-1.2 0.5 Z" />
    <path d="M1.2 0 L7 -1.8 L1.2 0.5 Z" />
  </g>
  {#if weather === 'summer' || weather === 'fall'}
    <g transform="translate(200, 56)" fill="#C8C4BC" opacity="0.55">
      <ellipse cx="0" cy="0" rx="2" ry="0.8" />
      <path d="M-1 0 L-5 -1.5 L-1 0.4 Z" />
      <path d="M1 0 L5 -1.2 L1 0.4 Z" />
    </g>
  {/if}
</g>
