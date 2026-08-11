<script lang="ts">
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const cow = $derived(winter ? '#E8E4DC' : '#F0E8D8')
  const cowSpot = '#3A3834'
  const chicken = '#E8D0A0'
  const comb = '#C84838'
</script>

<g class="life farm-life" aria-hidden="true">
  <!-- Grazing cow (left field edge) -->
  <g transform="translate(36, 112)">
    <ellipse cx="0" cy="0" rx="12" ry="6" fill={cow} />
    <ellipse cx="-4" cy="-1" rx="3.5" ry="2.5" fill={cowSpot} opacity="0.35" />
    <ellipse cx="5" cy="1" rx="2.8" ry="2" fill={cowSpot} opacity="0.3" />
    <rect x="9" y="-4" width="5" height="5" rx="1.5" fill={cow} />
    <circle cx="14.5" cy="-2.5" r="2.4" fill={cow} />
    <circle cx="15.4" cy="-3" r="0.45" fill="#2A2A28" />
    <path d="M-8 4 L-9 7" stroke={cowSpot} stroke-width="1.6" stroke-linecap="round" />
    <path d="M-3 5 L-3.5 7" stroke={cowSpot} stroke-width="1.6" stroke-linecap="round" />
    <path d="M4 5 L4.5 7" stroke={cowSpot} stroke-width="1.6" stroke-linecap="round" />
    <path d="M8 4 L9 7" stroke={cowSpot} stroke-width="1.6" stroke-linecap="round" />
    <path d="M-11 -1 L-14 -3" stroke={cow} stroke-width="1.4" stroke-linecap="round" />
    {#if winter}
      <ellipse cx="0" cy="-5" rx="8" ry="2" fill="#F8FBFE" opacity="0.55" />
    {/if}
  </g>

  <!-- Chickens near coop side -->
  <g class="sk-bob" transform="translate(248, 114)">
    <ellipse cx="0" cy="0" rx="3.5" ry="2.2" fill={chicken} />
    <circle cx="3.2" cy="-1.2" r="1.8" fill={chicken} />
    <path d="M4.6 -1.4 L6.2 -0.6" stroke={comb} stroke-width="0.9" stroke-linecap="round" />
    <path d="M2.4 -2.8 L2.4 -3.8 L3.2 -3.2 Z" fill={comb} />
    <path d="M-2 1.5 L-2.4 3" stroke="#8A6A40" stroke-width="0.8" />
    <path d="M1 1.5 L1.3 3" stroke="#8A6A40" stroke-width="0.8" />
  </g>
  <g class="sk-bob" style="animation-delay: -0.5s" transform="translate(262, 115)">
    <ellipse cx="0" cy="0" rx="3" ry="1.9" fill="#F0D8A8" />
    <circle cx="2.8" cy="-1" r="1.5" fill="#F0D8A8" />
    <path d="M4 -1.1 L5.4 -0.4" stroke={comb} stroke-width="0.8" />
    <path d="M2 -2.4 L2 -3.2 L2.7 -2.7 Z" fill={comb} />
  </g>

  <!-- Farmer with pitchfork (right of barn area, edge) -->
  <g transform="translate(400, 118)">
    <circle cx="0" cy="-17" r="2.5" fill="#C4A080" />
    <ellipse cx="0" cy="-19.5" rx="3.4" ry="1.3" fill="#6A4A28" />
    <path d="M-2.6 -14 L2.6 -14 L3.2 -2 L-3.2 -2 Z" fill="#3A5A78" />
    <path d="M-2.2 -2 L-3.2 0" stroke="#3A3A38" stroke-width="1.5" stroke-linecap="round" />
    <path d="M1.8 -2 L2.8 0" stroke="#3A3A38" stroke-width="1.5" stroke-linecap="round" />
    <path d="M3 -10 L8 -18" stroke="#8A6A40" stroke-width="1.1" />
    <path d="M6.5 -19 L9.5 -19 M7 -20.5 L7 -17.5 M9 -20.5 L9 -17.5" stroke="#8A6A40" stroke-width="0.9" />
  </g>

  <!-- Crow on fence (fall) / swallow (summer) -->
  {#if weather === 'fall' || weather === 'summer'}
    <g class="sk-flap" transform="translate(120, 96)" fill="#2A2A2C">
      <ellipse cx="0" cy="0" rx="3.5" ry="1.4" />
      <path d="M-2 -0.5 L-7 -3 L-2 0.5 Z" />
      <path d="M2 -0.5 L7 -2.5 L2 0.5 Z" />
      <circle cx="3.2" cy="-0.4" r="1" />
    </g>
  {/if}
</g>
