<script lang="ts">
  import type { Weather } from '../../types'

  /** Lean London silhouettes — edges only, clear run corridor ~80–340. */
  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const coat = $derived(winter ? '#2A3540' : '#1A2430')
  const bobby = $derived(winter ? '#1A2A48' : '#152238')
  const skin = '#C8A888'
  const brolly = $derived(winter ? '#3A4A68' : spring ? '#4A6A58' : '#5A3A3A')
</script>

<g class="life london-life" aria-hidden="true">
  <!-- Pedestrian with brolly (left edge) -->
  <g class="sk-walk" transform="translate(28, 118)">
    <circle cx="0" cy="-16" r="2.5" fill={skin} />
    <path d="M-2.4 -13.2 L2.4 -13.2 L3 -2 L-3 -2 Z" fill={coat} />
    {#if winter}
      <ellipse cx="0" cy="-18.2" rx="3.1" ry="1.3" fill="#E8EEF4" />
    {/if}
    <path d="M-2.1 -2 L-1.2 0 L-2.7 0 Z" fill={coat} />
    <path d="M1.3 -2 L3.1 0 L1.7 0 Z" fill={coat} />
    <!-- arm + brolly stem -->
    <path d="M2.6 -10 L6 -7" stroke={coat} stroke-width="1.3" stroke-linecap="round" fill="none" />
    <line x1="6" y1="-7" x2="6" y2="-18" stroke="#3A3A3C" stroke-width="0.9" />
    <!-- canopy -->
    <path
      d="M1 -18 Q6 -22 11 -18 L10 -17 Q6 -19.5 2 -17 Z"
      fill={brolly}
      opacity="0.9"
    />
  </g>

  <!-- Bobby / tourist (right edge) -->
  <g transform="translate(398, 118)">
    <g class="sk-bob">
      <circle cx="0" cy="-15.5" r="2.4" fill={skin} />
      <!-- custodian helmet -->
      <ellipse cx="0" cy="-18.2" rx="3.4" ry="1.6" fill={bobby} />
      <rect x="-1.2" y="-21" width="2.4" height="3" rx="0.4" fill={bobby} />
      <rect x="-2.8" y="-18.6" width="5.6" height="1.4" rx="0.3" fill="#0E1A30" />
      <path d="M-2.2 -13 L2.2 -13 L2.7 -2 L-2.7 -2 Z" fill={bobby} />
      <!-- belt / buttons hint -->
      <rect x="-2" y="-8" width="4" height="1" fill="#C8B878" opacity="0.7" />
      <circle cx="0" cy="-11" r="0.45" fill="#C8B878" />
      <circle cx="0" cy="-9.5" r="0.45" fill="#C8B878" />
      <path d="M-2 -2 L-2.8 0" stroke={bobby} stroke-width="1.4" stroke-linecap="round" />
      <path d="M1.5 -2 L2.6 0" stroke={bobby} stroke-width="1.4" stroke-linecap="round" />
      <!-- camera strap tourist hint -->
      <path
        d="M-2.5 -11 Q-5 -8 -3.5 -5"
        stroke="#5A5A58"
        stroke-width="0.7"
        fill="none"
      />
      <rect x="-5.2" y="-6.5" width="3.2" height="2.2" rx="0.3" fill="#3A3A3C" />
    </g>
  </g>

  <!-- Pigeon cluster (left curb, edge) -->
  <g transform="translate(52, 116)" fill="#6A6E72">
    <g class="sk-bob" style="animation-delay: -0.3s">
      <ellipse cx="0" cy="0" rx="3" ry="1.5" />
      <circle cx="2.4" cy="-0.9" r="1.2" />
      <path d="M3.4 -1.1 L4.8 -0.3" stroke="#6A6E72" stroke-width="0.65" />
    </g>
  </g>
  <g transform="translate(60, 115.5)" fill="#7A7E82" opacity="0.92">
    <ellipse cx="0" cy="0" rx="2.6" ry="1.3" />
    <circle cx="2.1" cy="-0.75" r="1.05" />
  </g>
  <g transform="translate(67, 116.2)" fill="#5A5E62" opacity="0.88">
    <ellipse cx="0" cy="0" rx="2.4" ry="1.2" />
    <circle cx="-2" cy="-0.7" r="1" />
    <path d="M-2.8 -0.9 L-4 -0.2" stroke="#5A5E62" stroke-width="0.6" />
  </g>

  <!-- Tiny distant sidewalk figures (very small, mid opacity, corridor edges) -->
  <g opacity="0.4" fill="#2A3038">
    <g transform="translate(78, 108)">
      <circle cx="0" cy="-6.5" r="1.2" />
      <rect x="-1.2" y="-5" width="2.4" height="5" rx="0.3" />
    </g>
    <g transform="translate(344, 107)">
      <circle cx="0" cy="-6.8" r="1.25" />
      <rect x="-1.25" y="-5.2" width="2.5" height="5.2" rx="0.3" />
    </g>
  </g>
</g>
