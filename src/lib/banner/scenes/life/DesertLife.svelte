<script lang="ts">
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const fall = $derived(weather === 'fall')
  const coyote = $derived(winter ? '#C8C0B0' : fall ? '#A88858' : '#8A7048')
  const coyoteDark = $derived(winter ? '#8A8274' : '#5A4830')
  const skin = '#C4A080'
  const shirt = $derived(winter ? '#5A6A78' : '#C86048')
  const buzzard = $derived(winter ? '#5A5858' : '#2A2A2C')
</script>

<g class="life desert-life" aria-hidden="true">
  <!-- Coyote silhouette (left edge, beyond corridor) -->
  <g transform="translate(28, 112)" fill={coyote} opacity="0.9">
    <ellipse cx="0" cy="0" rx="9" ry="4.2" />
    <rect x="7" y="-3.5" width="4" height="4.5" rx="1.2" />
    <circle cx="12.5" cy="-4.2" r="2.6" />
    <!-- ears -->
    <path d="M11 -6 L10.2 -9.5 L12.2 -6.5 Z" />
    <path d="M13.5 -6 L14.8 -9.2 L15 -6.2 Z" />
    <circle cx="13.4" cy="-4.5" r="0.4" fill={coyoteDark} />
    <!-- snout -->
    <path d="M14.5 -3.5 L17.5 -2.8 L14.5 -2.2 Z" />
    <!-- legs -->
    <path d="M-6 3 L-7 7" stroke={coyoteDark} stroke-width="1.5" stroke-linecap="round" />
    <path d="M-2 3.5 L-2.2 7" stroke={coyoteDark} stroke-width="1.5" stroke-linecap="round" />
    <path d="M3 3.5 L3.4 7" stroke={coyoteDark} stroke-width="1.5" stroke-linecap="round" />
    <path d="M6.5 3 L7.2 6.5" stroke={coyoteDark} stroke-width="1.5" stroke-linecap="round" />
    <!-- bushy tail -->
    <path d="M-8 -1 Q-14 -4 -12 2 Q-10 0 -8 1" fill={coyote} />
    {#if winter}
      <ellipse cx="2" cy="-4" rx="6" ry="1.5" fill="#F0F0EC" opacity="0.4" />
    {/if}
  </g>

  <!-- Lizard on warm rock (right of coyote, still left edge) -->
  <g transform="translate(58, 116)" fill={winter ? '#8A9088' : '#6A7A48'} opacity="0.85">
    <ellipse cx="0" cy="0" rx="4.5" ry="1.4" />
    <circle cx="4" cy="-0.4" r="1.3" />
    <path d="M-4 0 Q-7 -2 -8 1" fill="none" stroke={winter ? '#7A8078' : '#5A6A38'} stroke-width="0.9" />
    <path d="M-2 1 L-2.4 2.5 M1 1 L1.2 2.5 M2.5 0.8 L2.8 2.2" stroke={winter ? '#6A7068' : '#4A5A30'} stroke-width="0.7" />
  </g>

  <!-- Hitchhiker / tourist (right edge) -->
  <g transform="translate(400, 118)">
    <circle cx="0" cy="-16" r="2.5" fill={skin} />
    <!-- sun hat -->
    <ellipse cx="0" cy="-18" rx="4.2" ry="1.3" fill={winter ? '#D0D4D8' : '#E8D090'} />
    <rect x="-2.2" y="-21" width="4.4" height="3" rx="0.6" fill={winter ? '#C0C8D0' : '#D4B870'} />
    <path d="M-2.4 -13.5 L2.4 -13.5 L3 -2 L-3 -2 Z" fill={shirt} />
    <!-- backpack -->
    <rect x="-4.5" y="-12" width="2.8" height="6" rx="0.6" fill="#5A4A38" />
    <path d="M-2 -2 L-3 0" stroke="#3A3A38" stroke-width="1.5" stroke-linecap="round" />
    <path d="M1.6 -2 L2.8 0" stroke="#3A3A38" stroke-width="1.5" stroke-linecap="round" />
    <!-- thumb out -->
    <path d="M3 -10 L7 -12" stroke={skin} stroke-width="1.3" stroke-linecap="round" />
    <path d="M7 -12 L8 -10.5" stroke={skin} stroke-width="1.2" stroke-linecap="round" />
    <!-- cardboard sign -->
    <rect x="5" y="-20" width="14" height="7" rx="0.4" fill="#E8D8B0" />
    <text
      x="12"
      y="-16.5"
      text-anchor="middle"
      font-size="2.4"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#3A2A18">any town</text
    >
    <text
      x="12"
      y="-13.5"
      text-anchor="middle"
      font-size="2.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#5A3A28">w/ AC?</text
    >
    {#if winter}
      <ellipse cx="0" cy="-14" rx="3.2" ry="1.2" fill="#E8EEF4" opacity="0.5" />
    {/if}
  </g>

  <!-- Buzzard circling (anim group 1: sk-flap) -->
  <g class="sk-flap" transform="translate(210, 38)" fill={buzzard} opacity="0.78">
    <ellipse cx="0" cy="0" rx="3.2" ry="1.3" />
    <path d="M-2 0 L-11 -4 L-2 0.9 Z" />
    <path d="M2 0 L11 -3.5 L2 0.9 Z" />
    <circle cx="3" cy="-0.3" r="1.1" />
    <path d="M3.8 -0.4 L5.5 0.6" stroke={buzzard} stroke-width="0.8" />
    <!-- slight dihedral tip -->
    <path d="M-9 -3.2 L-12 -2" stroke={buzzard} stroke-width="0.9" stroke-linecap="round" />
    <path d="M9 -2.8 L12 -1.6" stroke={buzzard} stroke-width="0.9" stroke-linecap="round" />
  </g>

  <!-- distant second buzzard (static) -->
  <g transform="translate(280, 48)" fill={buzzard} opacity="0.4">
    <ellipse cx="0" cy="0" rx="2" ry="0.8" />
    <path d="M-1.5 0 L-7 -2 L-1.5 0.5 Z" />
    <path d="M1.5 0 L7 -1.8 L1.5 0.5 Z" />
  </g>
</g>
