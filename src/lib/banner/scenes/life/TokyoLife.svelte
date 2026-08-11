<script lang="ts">
  import type { Weather } from '../../types'

  /** Lean Tokyo sidewalk life — edges only, clear run corridor ~80–340. */
  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const umbrella = $derived(winter || spring)
  const coat = $derived(winter ? '#3A4058' : '#2A3040')
  const coat2 = $derived(winter ? '#4A3850' : '#383048')
  const skin = '#C8A888'
  const cat = $derived(winter ? '#D0C8C0' : '#C8A878')
</script>

<g class="life tokyo-life" aria-hidden="true">
  <!-- Pedestrian + umbrella (winter/spring) -->
  <g class="sk-walk" transform="translate(28, 118)">
    <circle cx="0" cy="-16" r="2.5" fill={skin} />
    <path d="M-2.4 -13 L2.4 -13 L3 -2 L-3 -2 Z" fill={coat} />
    <path d="M-2 -2 L-3 0 M1.6 -2 L2.8 0" stroke={coat} stroke-width="1.4" stroke-linecap="round" />
    <path d="M2.5 -10 L5 -7" stroke={coat} stroke-width="1.3" stroke-linecap="round" fill="none" />
    {#if umbrella}
      <line x1="-1" y1="-18" x2="-1" y2="-22" stroke="#4A4050" stroke-width="0.9" />
      <path d="M-7 -22 Q-1 -27 5 -22 Z" fill={winter ? '#4A6A88' : '#E07090'} />
    {/if}
  </g>

  <!-- Headphones vibe -->
  <g transform="translate(56, 118)">
    <circle cx="0" cy="-15.5" r="2.3" fill={skin} />
    <path d="M-2.2 -12.8 L2.2 -12.8 L2.7 -2 L-2.7 -2 Z" fill={coat2} />
    <path d="M-2 -2 L-2.8 0 M1.5 -2 L2.6 0" stroke={coat2} stroke-width="1.3" stroke-linecap="round" />
    <path d="M-3.2 -16.2 Q0 -19.5 3.2 -16.2" fill="none" stroke="#2A2A30" stroke-width="1.3" />
    <rect x="-3.8" y="-16.8" width="1.5" height="2.6" rx="0.4" fill="#2A2A30" />
    <rect x="2.3" y="-16.8" width="1.5" height="2.6" rx="0.4" fill="#2A2A30" />
  </g>

  <!-- Sitting cat -->
  <g transform="translate(72, 118)" fill={cat}>
    <ellipse cx="0" cy="-3" rx="4.5" ry="2.8" />
    <circle cx="3.5" cy="-5" r="2.2" />
    <path d="M2.4 -6.5 L1.8 -9 M4.4 -6.5 L5.2 -9" stroke={cat} stroke-width="1.1" stroke-linecap="round" />
    <path d="M-3.5 -2 Q-6 -6 -2.5 -1" fill="none" stroke={cat} stroke-width="1.3" />
    <circle cx="4.2" cy="-5.3" r="0.35" fill="#2A2A28" />
  </g>

  <!-- Scooter rider (right edge) -->
  <g class="sk-bob" transform="translate(394, 118)">
    <circle cx="-2" cy="-15" r="2.2" fill={skin} />
    <path d="M-4 -12.5 L0.5 -12.5 L1.2 -3 L-4.2 -3 Z" fill="#3A4858" />
    <path d="M-3.5 -3 L-4.2 0 M0 -3 L1.5 0" stroke="#2A3848" stroke-width="1.3" stroke-linecap="round" />
    <circle cx="-6" cy="0" r="2.8" fill="none" stroke="#3A3A48" stroke-width="1.1" />
    <circle cx="6" cy="0" r="2.8" fill="none" stroke="#3A3A48" stroke-width="1.1" />
    <path d="M-6 0 L2 -6 L6 0 M2 -6 L2 -10" fill="none" stroke="#3A3A48" stroke-width="1.2" />
  </g>
</g>
