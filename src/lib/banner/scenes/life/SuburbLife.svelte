<script lang="ts">
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()
  const winter = $derived(weather === 'winter')
  const skin = '#C4A080'
  const kidShirt = $derived(winter ? '#4A6A88' : '#3A8AC8')
  const walkerCoat = $derived(winter ? '#5A4A68' : '#6A5A48')
  const neighborShirt = $derived(winter ? '#6A7A68' : '#C87050')
  const dog = $derived(winter ? '#C8B8A0' : '#B09060')
  const bike = $derived(winter ? '#5A6878' : '#2A3A48')
</script>

<g class="life suburb-life" aria-hidden="true">
  <!-- Kid with bike (left edge only) -->
  <g transform="translate(18, 118)">
    <!-- bike -->
    <g fill="none" stroke={bike} stroke-width="1.1" opacity="0.9">
      <circle cx="-6" cy="-3" r="4" />
      <circle cx="8" cy="-3" r="4" />
      <path d="M-6 -3 L2 -8 L8 -3 L2 -3 Z" />
      <path d="M2 -8 L2 -11" />
      <path d="M0 -11 L4 -11" stroke-width="1.3" />
      <circle cx="2" cy="-3" r="1.2" fill={bike} stroke="none" />
    </g>
    <!-- kid -->
    <g class="sk-bob">
      <circle cx="2" cy="-18" r="2.3" fill={skin} />
      {#if winter}
        <ellipse cx="2" cy="-20" rx="2.8" ry="1.2" fill="#E8EEF4" />
      {:else}
        <path d="M0 -20 L2 -22.5 L4 -20" fill="#3A3A38" />
      {/if}
      <path d="M-0.5 -15.5 L4.5 -15.5 L5 -6 L0 -6 Z" fill={kidShirt} />
      <path d="M0 -6 L-1 0" stroke="#3A4A68" stroke-width="1.4" stroke-linecap="round" />
      <path d="M4 -6 L5.5 -2 L4 0" stroke="#3A4A68" stroke-width="1.4" stroke-linecap="round" />
      <path d="M5 -12 L9 -9" stroke={skin} stroke-width="1.2" stroke-linecap="round" />
    </g>
  </g>

  <!-- Dog walker (right edge) -->
  <g transform="translate(385, 118)">
    <g class="sk-walk">
      <circle cx="0" cy="-15" r="2.4" fill={skin} />
      <path d="M-2.2 -12.5 L2.2 -12.5 L2.8 -2 L-2.8 -2 Z" fill={walkerCoat} />
      {#if winter}
        <ellipse cx="0" cy="-17.5" rx="3" ry="1.3" fill="#D8E0E8" />
        <path d="M-3 -10 L-5 -6" stroke="#D8E0E8" stroke-width="1.5" stroke-linecap="round" />
      {:else}
        <path d="M-3 -10 L-5 -6" stroke={walkerCoat} stroke-width="1.4" stroke-linecap="round" />
      {/if}
      <path d="M2.5 -9 L5 -6" stroke={skin} stroke-width="1.2" stroke-linecap="round" />
      <path d="M-2 -2 L-3 0" stroke="#3A3A38" stroke-width="1.5" stroke-linecap="round" />
      <path d="M1.6 -2 L2.8 0" stroke="#3A3A38" stroke-width="1.5" stroke-linecap="round" />
      <!-- leash -->
      <path d="M5 -8 Q10 -6 12 -3" fill="none" stroke="#6A6A68" stroke-width="0.7" />
    </g>
    <!-- dog -->
    <g transform="translate(14, 0)" fill={dog}>
      <ellipse cx="0" cy="-3.5" rx="5.5" ry="2.8" />
      <circle cx="5" cy="-5" r="2.2" />
      <path d="M6.5 -5.5 L8.5 -4" stroke={dog} stroke-width="1" stroke-linecap="round" />
      <path d="M4.2 -6.8 L3.6 -8.5 M6 -6.8 L6.6 -8.3" stroke={dog} stroke-width="0.8" />
      <path d="M-4 -1.2 L-4.5 0.5 M-1 -1 L-1 0.5 M2 -1 L2.2 0.5 M4 -1.2 L4.5 0.5" stroke="#6A5A40" stroke-width="1.1" />
      <path d="M-5 -3.5 Q-8 -6 -6 -2" fill={dog} />
      {#if winter}
        <ellipse cx="1" cy="-6" rx="4" ry="1.2" fill="#F0F4F8" opacity="0.4" />
      {/if}
    </g>
  </g>

  <!-- Neighbor waving (far right, porch edge) -->
  <g transform="translate(408, 118)">
    <circle cx="0" cy="-16" r="2.3" fill={skin} />
    <path d="M-2.2 -13.5 L2.2 -13.5 L2.7 -2 L-2.7 -2 Z" fill={neighborShirt} />
    {#if winter}
      <ellipse cx="0" cy="-18.2" rx="2.8" ry="1.1" fill="#E0E8F0" />
    {/if}
    <path d="M-2 -2 L-2.8 0" stroke="#3A3A38" stroke-width="1.4" stroke-linecap="round" />
    <path d="M1.5 -2 L2.4 0" stroke="#3A3A38" stroke-width="1.4" stroke-linecap="round" />
    <!-- waving arm (anim group 3: sk-flap) -->
    <g class="sk-flap">
      <path d="M2.5 -11 L7 -16" stroke={skin} stroke-width="1.4" stroke-linecap="round" />
      <circle cx="7.5" cy="-16.5" r="1.1" fill={skin} />
    </g>
    <path d="M-3 -10 L-5 -7" stroke={neighborShirt} stroke-width="1.3" stroke-linecap="round" />
  </g>
</g>
