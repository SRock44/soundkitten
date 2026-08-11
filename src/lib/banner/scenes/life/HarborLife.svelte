<script lang="ts">
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const skin = '#C4A080'
  const coverall = $derived(winter ? '#4A5A68' : '#3A6A58')
  const fisher = $derived(winter ? '#5A4A3A' : '#3A4A58')
  const cat = $derived(winter ? '#D8D0C4' : '#C8A878')
  const catDark = '#5A4A38'
</script>

<g class="life harbor-life" aria-hidden="true">
  <!-- Dock worker left (edge) -->
  <g class="sk-walk" transform="translate(42, 118)">
    <circle cx="0" cy="-15" r="2.4" fill={skin} />
    <path d="M-2.4 -12.5 L2.4 -12.5 L3 -2 L-3 -2 Z" fill={coverall} />
    <rect x="-3.2" y="-10" width="6.4" height="3" fill={winter ? '#6A7A88' : '#C8A040'} opacity="0.7" />
    <path d="M-2.2 -2 L-3.2 0" stroke="#2A2A28" stroke-width="1.4" stroke-linecap="round" />
    <path d="M1.6 -2 L2.8 0" stroke="#2A2A28" stroke-width="1.4" stroke-linecap="round" />
    <path d="M3 -9 L7 -5" stroke={skin} stroke-width="1.3" stroke-linecap="round" />
    <!-- clipboard / crate hook -->
    <rect x="6" y="-8" width="4" height="5" rx="0.3" fill="#E8E0D0" opacity="0.85" />
    {#if winter}
      <ellipse cx="0" cy="-17.2" rx="2.8" ry="1.1" fill="#E8EEF4" />
    {/if}
  </g>

  <!-- Fisher right edge with pole -->
  <g transform="translate(398, 118)">
    <circle cx="0" cy="-14" r="2.2" fill={skin} />
    <path d="M-2.2 -11.5 L2.2 -11.5 L2.7 -2 L-2.7 -2 Z" fill={fisher} />
    <ellipse cx="0" cy="-16" rx="3" ry="1.2" fill={winter ? '#4A5A68' : '#2A4A58'} />
    <path d="M-2 -2 L-2.8 0" stroke="#2A2A28" stroke-width="1.3" stroke-linecap="round" />
    <path d="M1.5 -2 L2.4 0" stroke="#2A2A28" stroke-width="1.3" stroke-linecap="round" />
    <path d="M2.5 -10 L12 -28" stroke="#6A5A40" stroke-width="1.1" />
    <path d="M12 -28 L16 -26" stroke="#A8B0B8" stroke-width="0.6" opacity="0.7" />
    {#if winter}
      <ellipse cx="0" cy="-16.8" rx="2.6" ry="1" fill="#F0F4F8" opacity="0.7" />
    {/if}
  </g>

  <!-- Cat near crates (left edge, low) -->
  <g class="sk-bob" transform="translate(72, 116)">
    <ellipse cx="0" cy="0" rx="5" ry="2.6" fill={cat} />
    <circle cx="4.2" cy="-1.5" r="2" fill={cat} />
    <path d="M2.8 -3 L2.2 -5.2 L3.6 -3.4 Z" fill={cat} />
    <path d="M4.6 -3.2 L5.4 -5 L5.8 -3 Z" fill={cat} />
    <circle cx="5" cy="-1.8" r="0.35" fill={catDark} />
    <path d="M-4 -0.5 L-7 -2" stroke={cat} stroke-width="1.2" stroke-linecap="round" />
    <path d="M-3 1.5 L-3.4 3" stroke={catDark} stroke-width="1" />
    <path d="M0 1.8 L0.2 3.2" stroke={catDark} stroke-width="1" />
    <path d="M2.5 1.5 L2.8 3" stroke={catDark} stroke-width="1" />
  </g>

  <!-- Gulls over water -->
  <g class="sk-flap" transform="translate(160, 50)" fill="#E8E4DC" opacity="0.85">
    <ellipse cx="0" cy="0" rx="2.6" ry="1" />
    <path d="M-1.4 0 L-7.5 -2.2 L-1.4 0.5 Z" />
    <path d="M1.4 0 L7.5 -1.8 L1.4 0.5 Z" />
    <circle cx="2.2" cy="-0.3" r="0.85" />
  </g>
  <g class="sk-flap" style="animation-delay: -0.4s" transform="translate(280, 44)" fill="#D0CCC4" opacity="0.8">
    <ellipse cx="0" cy="0" rx="2.2" ry="0.9" />
    <path d="M-1.2 0 L-6.5 -2 L-1.2 0.45 Z" />
    <path d="M1.2 0 L6.5 -1.6 L1.2 0.45 Z" />
  </g>

  <!-- Distant deckhand silhouette on freighter area -->
  <g opacity="0.45" fill="#2A3540" transform="translate(150, 78)">
    <circle cx="0" cy="-5" r="1.2" />
    <rect x="-1.1" y="-3.8" width="2.2" height="3.8" rx="0.3" />
  </g>
</g>
