<script lang="ts">
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const goat = $derived(winter ? '#E4E0D8' : '#C8C0B0')
  const goatDark = '#6A6458'
</script>

<g class="life mountain-life" aria-hidden="true">
  <!-- Mountain goat on near rock ledge -->
  <g transform="translate(48, 100)">
    <ellipse cx="0" cy="0" rx="6.5" ry="3.5" fill={goat} />
    <rect x="5" y="-4" width="2.8" height="5" rx="1" fill={goat} />
    <circle cx="9" cy="-5" r="2" fill={goat} />
    <path d="M8.2 -6.5 L7.2 -9.5 M9.5 -6.5 L10.5 -9.2" stroke={goat} stroke-width="0.9" stroke-linecap="round" />
    <path d="M-4 2.5 L-4.5 5" stroke={goatDark} stroke-width="1.2" stroke-linecap="round" />
    <path d="M0 3 L0.2 5" stroke={goatDark} stroke-width="1.2" stroke-linecap="round" />
    <path d="M3.5 2.5 L4 5" stroke={goatDark} stroke-width="1.2" stroke-linecap="round" />
    <path d="M5.5 2 L6.2 4.5" stroke={goatDark} stroke-width="1.2" stroke-linecap="round" />
    <path d="M-6 -0.5 L-8.5 -1.5" stroke={goat} stroke-width="1.1" stroke-linecap="round" />
  </g>

  <!-- Hiker on trail (right) -->
  <g class="sk-walk" transform="translate(375, 116)">
    <circle cx="0" cy="-15" r="2.3" fill="#C4A080" />
    <path d="M-2.3 -12.5 L2.3 -12.5 L2.8 -2 L-2.8 -2 Z" fill="#4A5A6A" />
    <path d="M-3.2 -10 L-5 -5" stroke="#5A3A28" stroke-width="1.5" stroke-linecap="round" />
    <ellipse cx="-5.4" cy="-4" rx="2" ry="2.5" fill="#5A3A28" />
    <path d="M2.5 -9 L4 -4" stroke="#C4A080" stroke-width="1.2" stroke-linecap="round" />
    <path d="M-2 -2 L-2.8 0" stroke="#2A3A48" stroke-width="1.4" stroke-linecap="round" />
    <path d="M1.5 -2 L2.4 0" stroke="#2A3A48" stroke-width="1.4" stroke-linecap="round" />
    {#if winter}
      <ellipse cx="0" cy="-17.2" rx="2.8" ry="1.1" fill="#F0F4F8" />
    {/if}
  </g>

  <!-- Eagle / hawk silhouette high -->
  <g class="sk-flap" transform="translate(260, 42)" fill="#3A3A3C" opacity="0.75">
    <ellipse cx="0" cy="0" rx="3" ry="1.2" />
    <path d="M-2 0 L-10 -3.5 L-2 0.8 Z" />
    <path d="M2 0 L10 -3 L2 0.8 Z" />
    <path d="M2.5 -0.2 L4.5 0.8" stroke="#3A3A3C" stroke-width="0.8" />
  </g>

  <!-- Second distant climber silhouette -->
  <g opacity="0.5" fill="#2A3540" transform="translate(300, 78)">
    <circle cx="0" cy="-5" r="1.2" />
    <rect x="-1.1" y="-3.8" width="2.2" height="3.8" rx="0.3" />
  </g>
</g>
