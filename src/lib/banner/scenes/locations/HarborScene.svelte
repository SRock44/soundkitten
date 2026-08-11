<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()

  const g = $derived(groundFor('harbor', weather))
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  const water = $derived(
    winter
      ? { deep: '#6A8A98', mid: '#8AA8B4', foam: '#E0ECF0', ice: '#D0E4EC' }
      : fall
        ? { deep: '#3A5A68', mid: '#4A6A78', foam: '#A8B8C0', ice: '#6A8088' }
        : summer
          ? { deep: '#2A6A80', mid: '#3A8098', foam: '#C8E0E8', ice: '#4A98A8' }
          : { deep: '#3A6A78', mid: '#4A8090', foam: '#C0D8E0', ice: '#5A98A0' },
  )

  const steel = $derived(winter ? '#9AA8B4' : '#5A6874')
  const steelDark = $derived(winter ? '#6A7884' : '#3A4854')
  const rust = $derived(fall ? '#8A4A32' : '#7A5240')
  const wood = $derived(winter ? '#8A7A68' : '#5A4A38')
  const woodLite = $derived(winter ? '#A89880' : '#7A6A50')
  const warehouse = $derived(
    winter
      ? { wall: '#8A949E', roof: '#6A7480', door: '#5A646E', trim: '#4A545E' }
      : fall
        ? { wall: '#7A6A58', roof: '#4A3A30', door: '#3A3028', trim: '#5A4A3A' }
        : { wall: '#8A9A8A', roof: '#4A5A58', door: '#3A4848', trim: '#5A6A68' },
  )

  const signCream = '#F2E8D4'

  const dockPlanks = [
    { y: 118 },
    { y: 121 },
    { y: 124 },
    { y: 127 },
    { y: 130 },
    { y: 133 },
  ]

  const pilings = [
    { x: 16 },
    { x: 40 },
    { x: 64 },
    { x: 356 },
    { x: 380 },
    { x: 404 },
  ]

  const reflections = [
    { x: 50, y: 108, rx: 16, ry: 2.2 },
    { x: 130, y: 112, rx: 22, ry: 1.8 },
    { x: 210, y: 106, rx: 18, ry: 2 },
    { x: 290, y: 110, rx: 20, ry: 1.6 },
    { x: 370, y: 108, rx: 14, ry: 2 },
  ]

  const ropeCoils = [
    { x: 52, y: 114, s: 1 },
    { x: 70, y: 116, s: 0.75 },
    { x: 362, y: 115, s: 0.9 },
  ]
</script>

<g class="harbor" aria-hidden="true">
  <!-- Far industrial haze -->
  <rect x="0" y="40" width="420" height="50" fill={g.far} opacity="0.35" />
  <path
    d="M-10 78 C80 68, 160 74, 240 66 C320 58, 380 70, 430 64 L430 100 L-10 100 Z"
    fill={g.mid}
    opacity="0.4"
  />

  <!-- Water band -->
  <rect x="0" y="78" width="420" height="40" fill={water.deep} opacity="0.7" />
  <path
    d="M-10 90 C60 84, 140 94, 220 86 C300 78, 360 90, 430 84 L430 118 L-10 118 Z"
    fill={water.mid}
    opacity="0.75"
  />

  <!-- Cheap reflection ellipses -->
  <g fill="#fff" opacity={summer ? 0.14 : winter ? 0.08 : 0.1}>
    {#each reflections as r}
      <ellipse cx={r.x} cy={r.y} rx={r.rx} ry={r.ry} />
    {/each}
  </g>
  <g fill={water.foam} opacity="0.2">
    <ellipse cx="90" cy="100" rx="28" ry="2.5" />
    <ellipse cx="260" cy="104" rx="32" ry="2" />
    <ellipse cx="380" cy="98" rx="20" ry="2.2" />
  </g>

  {#if winter}
    <!-- ice sheets -->
    <g fill={water.ice} opacity="0.55">
      <ellipse cx="60" cy="102" rx="24" ry="4" />
      <ellipse cx="180" cy="106" rx="30" ry="3.5" />
      <ellipse cx="320" cy="100" rx="22" ry="3.8" />
      <path d="M240 98 L270 96 L265 104 L245 103 Z" opacity="0.7" />
    </g>
  {/if}

  <!-- ===== FREIGHTER (far, mid-left — above corridor) ===== -->
  <g class="freighter" transform="translate(90 0)" opacity="0.9">
    <rect x="0" y="62" width="110" height="18" rx="1" fill={steelDark} />
    <path d="M0 70 L-8 80 L0 80 Z" fill={steelDark} />
    <path d="M110 70 L122 80 L110 80 Z" fill={steel} />
    <rect x="70" y="48" width="28" height="16" fill={steel} />
    <rect x="74" y="42" width="10" height="8" fill={steelDark} />
    <rect x="90" y="44" width="6" height="6" fill={steelDark} />
    <!-- stacks -->
    <rect x="78" y="34" width="4" height="10" fill={rust} />
    <rect x="86" y="36" width="3.5" height="8" fill={rust} />
    <g fill={winter ? '#C8D4DC' : '#6A7080'} opacity="0.45">
      <ellipse cx="80" cy="30" rx="5" ry="2" />
      <ellipse cx="88" cy="32" rx="4" ry="1.6" />
    </g>
    <!-- hull stripe -->
    <rect x="4" y="72" width="100" height="2.5" fill={fall ? '#C45A3A' : '#C8A040'} opacity="0.7" />
    <!-- portholes -->
    <g fill={winter ? '#A8C0D0' : '#4A8098'} opacity="0.7">
      <circle cx="20" cy="68" r="1.6" />
      <circle cx="32" cy="68" r="1.6" />
      <circle cx="44" cy="68" r="1.6" />
      <circle cx="56" cy="68" r="1.6" />
    </g>
    <!-- waterline reflection under hull -->
    <ellipse cx="55" cy="84" rx="48" ry="3" fill="#000" opacity="0.12" />
  </g>

  <!-- ===== FISHING BOAT (right mid) ===== -->
  <g class="fishing-boat" transform="translate(300 0)">
    <path d="M0 78 L8 68 L70 68 L78 78 L70 84 L8 84 Z" fill={winter ? '#6A7A88' : '#3A4A58'} />
    <path d="M10 68 L14 58 L30 58 L34 68 Z" fill={winter ? '#8A9AA8' : '#5A6A78'} />
    <rect x="18" y="50" width="2.5" height="18" fill={wood} />
    <path d="M20 50 L38 56 L20 58 Z" fill="#C8C4B8" opacity="0.75" />
    <rect x="40" y="70" width="18" height="6" fill={rust} opacity="0.8" />
    <ellipse cx="38" cy="88" rx="28" ry="2.5" fill="#000" opacity="0.1" />
    <!-- cabin window -->
    <rect x="16" y="60" width="10" height="5" fill={winter ? '#B0C8D8' : '#7AA8C0'} opacity="0.65" />
    <!-- hanging buoy -->
    <circle cx="62" cy="74" r="2.2" fill="#C84848" />
    <line x1="62" y1="68" x2="62" y2="72" stroke={wood} stroke-width="0.8" />
  </g>

  <!-- ===== CRANE (far right skyline) ===== -->
  <g class="crane" transform="translate(370 0)" fill={steel}>
    <rect x="18" y="30" width="4" height="70" />
    <rect x="14" y="28" width="12" height="4" />
    <rect x="-20" y="32" width="60" height="3" />
    <rect x="-20" y="32" width="3" height="8" />
    <line x1="20" y1="35" x2="-10" y2="70" stroke={steelDark} stroke-width="1.2" />
    <line x1="20" y1="35" x2="50" y2="55" stroke={steelDark} stroke-width="1" />
    <!-- hook + cable (static tip; net sway is separate anim) -->
    <line x1="-10" y1="40" x2="-10" y2="72" stroke={steelDark} stroke-width="0.9" />
    <path d="M-13 72 L-7 72 L-10 76 Z" fill={rust} />
    <rect x="12" y="88" width="16" height="12" fill={steelDark} opacity="0.8" />
  </g>

  <!-- Far crane / mast silhouettes -->
  <g fill={g.far} opacity="0.55">
    <rect x="40" y="48" width="3" height="40" />
    <rect x="28" y="50" width="28" height="2.5" />
    <rect x="250" y="52" width="2.5" height="36" />
    <rect x="240" y="54" width="24" height="2" />
  </g>

  <!-- ===== WAREHOUSE (left edge) ===== -->
  <g class="warehouse" transform="translate(0 0)">
    <rect x="0" y="58" width="72" height="60" fill={warehouse.wall} />
    <rect x="0" y="58" width="72" height="5" fill={warehouse.roof} />
    <path d="M-2 63 L36 48 L74 63 Z" fill={warehouse.roof} />
    <!-- loading bay -->
    <rect x="8" y="78" width="28" height="40" fill={warehouse.door} />
    <rect x="10" y="80" width="24" height="6" fill={steelDark} opacity="0.5" />
    <line x1="22" y1="78" x2="22" y2="118" stroke={warehouse.trim} stroke-width="1" opacity="0.5" />
    <!-- windows -->
    <g fill={winter ? '#B0C4D4' : '#6A8A98'} opacity="0.55">
      <rect x="44" y="70" width="10" height="8" />
      <rect x="58" y="70" width="10" height="8" />
      <rect x="44" y="86" width="10" height="8" />
      <rect x="58" y="86" width="10" height="8" />
    </g>
    <g stroke={warehouse.trim} stroke-width="0.6" fill="none" opacity="0.5">
      <line x1="49" y1="70" x2="49" y2="78" />
      <line x1="44" y1="74" x2="54" y2="74" />
      <line x1="63" y1="70" x2="63" y2="78" />
      <line x1="58" y1="74" x2="68" y2="74" />
    </g>
    <!-- corrugated hint -->
    <g stroke={warehouse.trim} stroke-width="0.5" opacity="0.25">
      <line x1="4" y1="64" x2="4" y2="118" />
      <line x1="12" y1="64" x2="12" y2="78" />
      <line x1="40" y1="64" x2="40" y2="118" />
      <line x1="68" y1="64" x2="68" y2="118" />
    </g>
    {#if winter}
      <path d="M-2 63 L36 48 L74 63 Z" fill={g.accent} opacity="0.5" />
    {/if}
    {#if summer}
      <rect x="48" y="62" width="14" height="4" fill={steel} rx="0.4" opacity="0.7" />
    {/if}
  </g>

  <!-- ===== DOCK PLATFORMS (edges) ===== -->
  <g class="dock-left">
    <rect x="0" y="108" width="78" height="10" fill={wood} />
    <g stroke={woodLite} stroke-width="0.7" opacity="0.45">
      <line x1="0" y1="110" x2="78" y2="110" />
      <line x1="0" y1="113" x2="78" y2="113" />
      <line x1="0" y1="116" x2="78" y2="116" />
    </g>
    <!-- bollards -->
    <g fill={steelDark}>
      <rect x="20" y="102" width="4" height="8" rx="1" />
      <ellipse cx="22" cy="102" rx="3" ry="1.5" />
      <rect x="55" y="102" width="4" height="8" rx="1" />
      <ellipse cx="57" cy="102" rx="3" ry="1.5" />
    </g>
  </g>

  <g class="dock-right">
    <rect x="342" y="108" width="78" height="10" fill={wood} />
    <g stroke={woodLite} stroke-width="0.7" opacity="0.45">
      <line x1="342" y1="110" x2="420" y2="110" />
      <line x1="342" y1="113" x2="420" y2="113" />
      <line x1="342" y1="116" x2="420" y2="116" />
    </g>
    <g fill={steelDark}>
      <rect x="360" y="102" width="4" height="8" rx="1" />
      <ellipse cx="362" cy="102" rx="3" ry="1.5" />
      <rect x="395" y="102" width="4" height="8" rx="1" />
      <ellipse cx="397" cy="102" rx="3" ry="1.5" />
    </g>
  </g>

  <!-- Pilings -->
  {#each pilings as p}
    <rect x={p.x} y="100" width="5" height="22" rx="1" fill={wood} />
    <rect x={p.x + 0.8} y="100" width="1.3" height="22" fill="#fff" opacity="0.08" />
    <ellipse cx={p.x + 2.5} cy="124" rx="4.5" ry="1.6" fill={water.deep} opacity="0.35" />
  {/each}

  <!-- ===== CRATES (left dock — signs) ===== -->
  <g class="crates" transform="translate(28 98)">
    <rect x="0" y="8" width="16" height="12" fill={fall ? '#8A6A40' : '#7A5A38'} />
    <rect x="0" y="8" width="16" height="2" fill={woodLite} opacity="0.4" />
    <rect x="1" y="10" width="14" height="6" rx="0.3" fill="#2A2218" />
    <text
      x="8"
      y="14.5"
      text-anchor="middle"
      font-size="3.2"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}>dock 9</text
    >
    <rect x="14" y="0" width="14" height="10" fill={winter ? '#6A7A68' : '#5A6A48'} />
    <rect x="15" y="2" width="12" height="5" rx="0.3" fill="#1A2418" />
    <text
      x="21"
      y="5.8"
      text-anchor="middle"
      font-size="2.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#C8E0A8">catch of</text
    >
    <text
      x="21"
      y="8.4"
      text-anchor="middle"
      font-size="2.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#E8D090">the meow</text
    >
    <!-- crate nails / battens -->
    <g stroke={wood} stroke-width="0.6" opacity="0.5">
      <line x1="0" y1="14" x2="16" y2="14" />
      <line x1="8" y1="8" x2="8" y2="20" />
    </g>
  </g>

  <!-- More crates right edge -->
  <g transform="translate(378 100)" fill={woodLite} opacity="0.9">
    <rect x="0" y="6" width="12" height="12" fill={wood} />
    <rect x="10" y="10" width="10" height="8" fill="#6A5A40" />
    <rect x="4" y="0" width="9" height="8" fill="#7A6A48" />
    <g stroke={wood} stroke-width="0.5" opacity="0.45" fill="none">
      <line x1="0" y1="12" x2="12" y2="12" />
      <line x1="10" y1="14" x2="20" y2="14" />
    </g>
  </g>

  <!-- Rope piles -->
  {#each ropeCoils as r}
    <g transform="translate({r.x} {r.y}) scale({r.s})" fill="none" stroke={winter ? '#A89880' : '#8A7A58'} stroke-width="1.2" opacity="0.8">
      <ellipse cx="0" cy="0" rx="5" ry="2.2" />
      <ellipse cx="0" cy="0" rx="3" ry="1.3" />
      <path d="M4 0 Q6 2 3 3" />
    </g>
  {/each}

  <!-- Nets (animated group 1: sk-sway) — left pier edge -->
  <g class="sk-sway" transform="translate(60 88)" stroke={winter ? '#8A9AA0' : '#6A7A70'} stroke-width="0.7" fill="none" opacity="0.65">
    <path d="M0 0 Q8 10 0 22 Q-6 14 0 0" />
    <path d="M4 2 Q12 12 4 24" />
    <path d="M-3 4 L6 8 M-2 10 L7 14 M-1 16 L6 20" opacity="0.7" />
    <circle cx="2" cy="20" r="1.2" fill="#4A5A50" stroke="none" opacity="0.5" />
  </g>
  <g transform="translate(350 90)" stroke="#6A7A70" stroke-width="0.65" fill="none" opacity="0.55">
    <path d="M0 0 Q10 12 2 26" />
    <path d="M6 4 L-2 10 M8 12 L0 18 M6 20 L-1 24" />
  </g>

  <!-- Cleats / chains -->
  <g fill={steelDark} opacity="0.8">
    <rect x="34" y="106" width="6" height="2" rx="0.5" />
    <rect x="36" y="104" width="2" height="5" />
    <rect x="388" y="106" width="6" height="2" rx="0.5" />
  </g>

  <!-- ===== SEASONAL FX ===== -->
  {#if winter}
    <!-- fog (animated group 2: sk-mist) -->
    <g class="sk-mist" opacity="0.28">
      <ellipse cx="100" cy="86" rx="55" ry="7" fill="#E8F0F4" />
      <ellipse cx="220" cy="92" rx="70" ry="6" fill="#E8F0F4" />
      <ellipse cx="340" cy="88" rx="50" ry="6.5" fill="#E8F0F4" />
    </g>
    <g fill={g.accent} opacity="0.5">
      <ellipse cx="40" cy="110" rx="12" ry="2" />
      <ellipse cx="370" cy="111" rx="10" ry="1.8" />
    </g>
  {:else if summer}
    <!-- heat shimmer (animated group 2: sk-mist) -->
    <g class="sk-mist" opacity="0.12">
      <ellipse cx="120" cy="90" rx="45" ry="5" fill="#fff" />
      <ellipse cx="250" cy="94" rx="50" ry="4" fill="#fff" />
      <ellipse cx="360" cy="92" rx="35" ry="4.5" fill="#fff" />
    </g>
    <!-- sun glare on water -->
    <g class="sk-glow" opacity="0.2" fill="#E8D080">
      <ellipse cx="200" cy="100" rx="40" ry="3" />
      <ellipse cx="200" cy="106" rx="28" ry="2" />
    </g>
  {:else if spring}
    <g class="sk-mist" opacity="0.1">
      <ellipse cx="160" cy="88" rx="50" ry="5" fill="#D0E8E0" />
      <ellipse cx="300" cy="92" rx="40" ry="4" fill="#D0E8E0" />
    </g>
    <!-- spring blossoms on warehouse planter -->
    <g fill={g.accent} opacity="0.8">
      <circle cx="50" cy="112" r="1.5" />
      <circle cx="56" cy="111" r="1.3" />
      <circle cx="62" cy="113" r="1.4" />
    </g>
  {:else}
    <!-- fall cooler fog -->
    <g class="sk-mist" opacity="0.14">
      <ellipse cx="140" cy="90" rx="55" ry="5" fill="#A8B8B0" />
      <ellipse cx="280" cy="94" rx="48" ry="4.5" fill="#A8B8B0" />
    </g>
    <g fill={g.accent} opacity="0.5">
      <ellipse cx="48" cy="112" rx="2.5" ry="1.3" />
      <ellipse cx="366" cy="113" rx="2.2" ry="1.2" />
    </g>
  {/if}

  <!-- Ground / dock plank path — top y=118; corridor ~80–340 clear of props -->
  <rect x="0" y="118" width="420" height="22" fill={g.near} />
  <rect x="0" y="118" width="420" height="22" fill={g.path} opacity="0.85" />
  <g stroke={wood} stroke-width="0.8" opacity="0.35">
    {#each dockPlanks as p}
      <line x1="0" y1={p.y} x2="420" y2={p.y} />
    {/each}
  </g>
  <!-- plank seams (vertical, sparse) -->
  <g stroke={wood} stroke-width="0.6" opacity="0.25">
    <line x1="100" y1="118" x2="100" y2="140" />
    <line x1="160" y1="118" x2="160" y2="140" />
    <line x1="220" y1="118" x2="220" y2="140" />
    <line x1="280" y1="118" x2="280" y2="140" />
    <line x1="320" y1="118" x2="320" y2="140" />
  </g>
  <rect x="0" y="118" width="420" height="2.5" fill={g.detail} opacity="0.3" />

  <!-- Edge dock boards thicker -->
  <rect x="0" y="118" width="78" height="4" fill={wood} opacity="0.55" />
  <rect x="342" y="118" width="78" height="4" fill={wood} opacity="0.55" />

  {#if winter}
    <rect x="0" y="116" width="420" height="4" fill={g.accent} opacity="0.45" />
    <g fill={g.accent} opacity="0.35">
      <ellipse cx="120" cy="120" rx="20" ry="2" />
      <ellipse cx="260" cy="122" rx="24" ry="1.8" />
    </g>
  {/if}
</g>
