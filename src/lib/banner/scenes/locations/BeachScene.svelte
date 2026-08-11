<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()

  const g = $derived(groundFor('beach', weather))
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  const water = $derived(
    winter
      ? { deep: '#7A9AA8', mid: '#9AB4C0', foam: '#E8F0F4', cool: '#A8C0CC' }
      : fall
        ? { deep: '#3A7A90', mid: '#4A8AA0', foam: '#D0E0E4', cool: '#5A98A8' }
        : summer
          ? { deep: '#2A88A8', mid: '#3AA0C0', foam: '#E8F4F8', cool: '#48B0C8' }
          : { deep: '#3A98B0', mid: '#5AB0C8', foam: '#E0F0F4', cool: '#68B8C8' },
  )

  const wood = $derived(winter ? '#8A7A68' : fall ? '#6A5440' : '#7A5E48')
  const woodDark = $derived(winter ? '#6A5A4A' : '#4A3A2C')
  const woodLite = $derived(winter ? '#A89880' : '#9A7A58')
  const hut = $derived(
    winter
      ? { wall: '#A89078', roof: '#6A7080', trim: '#5A4A3A', door: '#4A3A30' }
      : fall
        ? { wall: '#C8A070', roof: '#6A4A38', trim: '#5A3A28', door: '#4A3020' }
        : summer
          ? { wall: '#E8C888', roof: '#2A6A78', trim: '#5A4030', door: '#3A2A20' }
          : { wall: '#D8B878', roof: '#3A7A88', trim: '#5A4030', door: '#3A2A20' },
  )

  const signCream = '#F2E8D4'

  const pilings = [
    { x: 12, h: 28 },
    { x: 28, h: 32 },
    { x: 44, h: 30 },
    { x: 60, h: 34 },
    { x: 348, h: 30 },
    { x: 364, h: 33 },
    { x: 380, h: 29 },
    { x: 396, h: 31 },
  ]

  const boardPlanks = [
    { y: 102 },
    { y: 105 },
    { y: 108 },
    { y: 111 },
    { y: 114 },
  ]

  const duneTufts = [
    { x: 18, lean: -1 },
    { x: 32, lean: 1 },
    { x: 46, lean: -1 },
    { x: 58, lean: 1 },
    { x: 70, lean: -1 },
    { x: 352, lean: 1 },
    { x: 366, lean: -1 },
    { x: 380, lean: 1 },
    { x: 394, lean: -1 },
    { x: 406, lean: 1 },
  ]

  const foamCaps = [
    { x: 40, rx: 18 },
    { x: 120, rx: 22 },
    { x: 210, rx: 16 },
    { x: 300, rx: 20 },
    { x: 380, rx: 18 },
  ]

  const snowFlecks = [
    { x: 24, y: 112 },
    { x: 52, y: 116 },
    { x: 88, y: 114 },
    { x: 160, y: 117 },
    { x: 250, y: 115 },
    { x: 310, y: 116 },
    { x: 360, y: 113 },
    { x: 400, y: 117 },
  ]

  const shells = [
    { x: 22, y: 114, r: 2.2, c: '#F0D0B8' },
    { x: 48, y: 116, r: 1.8, c: '#E8C8A0' },
    { x: 66, y: 113, r: 2, c: '#F8E8D0' },
    { x: 356, y: 115, r: 2.1, c: '#F0D8C0' },
    { x: 378, y: 114, r: 1.7, c: '#E8C8B0' },
    { x: 398, y: 116, r: 2, c: '#F4E0C8' },
  ]

  const springFlowers = [
    { x: 28, y: 110, c: '#F0A0C8' },
    { x: 54, y: 112, c: '#F0D060' },
    { x: 368, y: 111, c: '#90C8F0' },
    { x: 392, y: 112, c: '#E090C0' },
  ]

  const hutBattens = [{ x: 8 }, { x: 14 }, { x: 20 }, { x: 26 }, { x: 32 }]
  const corridorNibbles = [
    { x: 14, rx: 6 },
    { x: 48, rx: 5 },
    { x: 360, rx: 6 },
    { x: 390, rx: 5 },
  ]
</script>

<g class="beach" aria-hidden="true">
  <!-- Far sea / horizon haze -->
  <rect x="0" y="58" width="420" height="40" fill={water.deep} opacity="0.35" />
  <path
    d="M-10 88 C60 78, 140 82, 210 76 C280 70, 350 78, 430 72 L430 100 L-10 100 Z"
    fill={water.mid}
    opacity="0.45"
  />

  <!-- Water band (beside / below ground plane) -->
  <rect x="0" y="100" width="420" height="40" fill={water.deep} opacity="0.55" />
  <path
    d="M-10 108 C40 102, 90 110, 150 104 C210 98, 270 108, 330 102 C370 98, 410 106, 430 104 L430 140 L-10 140 Z"
    fill={water.mid}
    opacity="0.7"
  />
  <path
    d="M-10 118 C50 112, 110 120, 180 114 C250 108, 320 118, 390 112 L430 116 L430 140 L-10 140 Z"
    fill={water.cool}
    opacity="0.55"
  />

  <!-- Foam line -->
  <g fill={water.foam} opacity={winter ? 0.35 : 0.5}>
    {#each foamCaps as f}
      <ellipse cx={f.x} cy="116" rx={f.rx} ry="2.2" />
    {/each}
  </g>
  <path
    d="M0 118 Q40 114 80 118 Q140 122 200 116 Q260 112 320 118 Q370 122 420 116"
    fill="none"
    stroke={water.foam}
    stroke-width="1.4"
    opacity="0.45"
  />

  <!-- Cheap water shimmer ellipses (static — anim budget saved for mist/sway/flag) -->
  <g fill="#fff" opacity={summer ? 0.12 : winter ? 0.06 : 0.08}>
    <ellipse cx="70" cy="122" rx="14" ry="2" />
    <ellipse cx="160" cy="126" rx="18" ry="1.8" />
    <ellipse cx="250" cy="124" rx="12" ry="1.6" />
    <ellipse cx="340" cy="128" rx="16" ry="2" />
  </g>

  {#if fall}
    <!-- cooler water wash -->
    <rect x="0" y="100" width="420" height="40" fill="#2A5A68" opacity="0.12" />
  {/if}

  <!-- Distant dunes -->
  <path
    d="M-10 118 C30 96, 70 92, 110 102 C140 110, 160 104, 190 108 L190 118 Z"
    fill={g.far}
    opacity="0.4"
  />
  <path
    d="M300 118 C330 100, 360 94, 400 104 C420 110, 430 108, 430 118 Z"
    fill={g.far}
    opacity="0.4"
  />

  <!-- Mid dune mounds (edges) -->
  <path
    d="M-10 118 C20 104, 50 100, 78 110 L78 118 Z"
    fill={g.mid}
    opacity="0.85"
  />
  <path
    d="M340 118 C360 104, 390 98, 430 108 L430 118 Z"
    fill={g.mid}
    opacity="0.85"
  />

  <!-- ===== LEFT PIER + BOARDWALK ===== -->
  <g class="pier-left">
    <!-- deck -->
    <path d="M0 98 L78 98 L78 118 L0 118 Z" fill={wood} opacity="0.92" />
    <g stroke={woodDark} stroke-width="0.7" opacity="0.55">
      {#each boardPlanks as p}
        <line x1="0" y1={p.y} x2="78" y2={p.y} />
      {/each}
    </g>
    <rect x="0" y="98" width="78" height="2" fill={woodLite} opacity="0.5" />
    <!-- rail -->
    <rect x="0" y="90" width="78" height="2" fill={woodDark} />
    <g fill={woodDark}>
      <rect x="8" y="90" width="2" height="10" />
      <rect x="28" y="90" width="2" height="10" />
      <rect x="48" y="90" width="2" height="10" />
      <rect x="68" y="90" width="2" height="10" />
    </g>
    <!-- pilings in water -->
    {#each pilings.filter((p) => p.x < 100) as p}
      <rect x={p.x} y={118 - p.h * 0.35} width="5" height={p.h * 0.55} rx="1" fill={woodDark} />
      <rect x={p.x + 0.8} y={118 - p.h * 0.35} width="1.4" height={p.h * 0.55} fill="#fff" opacity="0.08" />
      <ellipse cx={p.x + 2.5} cy="124" rx="4" ry="1.6" fill={water.deep} opacity="0.35" />
    {/each}
  </g>

  <!-- ===== RIGHT PIER STUB ===== -->
  <g class="pier-right">
    <path d="M342 100 L420 100 L420 118 L342 118 Z" fill={wood} opacity="0.9" />
    <g stroke={woodDark} stroke-width="0.7" opacity="0.5">
      <line x1="342" y1="104" x2="420" y2="104" />
      <line x1="342" y1="108" x2="420" y2="108" />
      <line x1="342" y1="112" x2="420" y2="112" />
    </g>
    <rect x="342" y="92" width="78" height="2" fill={woodDark} />
    <g fill={woodDark}>
      <rect x="354" y="92" width="2" height="10" />
      <rect x="376" y="92" width="2" height="10" />
      <rect x="398" y="92" width="2" height="10" />
    </g>
    {#each pilings.filter((p) => p.x >= 340) as p}
      <rect x={p.x} y={118 - p.h * 0.3} width="5" height={p.h * 0.5} rx="1" fill={woodDark} />
      <ellipse cx={p.x + 2.5} cy="124" rx="4" ry="1.5" fill={water.deep} opacity="0.3" />
    {/each}
  </g>

  <!-- ===== SURF HUT (left edge) ===== -->
  <g class="surf-hut" transform="translate(6 0)">
    <rect x="6" y="72" width="42" height="36" fill={hut.wall} />
    <rect x="6" y="72" width="6" height="36" fill="#000" opacity="0.08" />
    <g stroke={hut.trim} stroke-width="0.8" opacity="0.45">
      {#each hutBattens as b}
        <line x1={b.x + 6} y1="74" x2={b.x + 6} y2="106" />
      {/each}
    </g>
    <!-- palm-leaf thatch roof -->
    <path d="M2 74 L27 58 L52 74 Z" fill={hut.roof} />
    <path d="M2 74 L27 58 L52 74" fill="none" stroke={hut.trim} stroke-width="1" opacity="0.6" />
    <g stroke={hut.trim} stroke-width="0.7" opacity="0.4" fill="none">
      <path d="M8 72 Q18 66 27 62" />
      <path d="M16 72 Q24 66 32 64" />
      <path d="M30 70 Q36 66 44 70" />
    </g>
    <!-- window + door -->
    <rect x="12" y="82" width="12" height="10" fill={winter ? '#A8C0D0' : '#7AB0C8'} opacity="0.65" />
    <line x1="18" y1="82" x2="18" y2="92" stroke={hut.trim} stroke-width="0.7" />
    <line x1="12" y1="87" x2="24" y2="87" stroke={hut.trim} stroke-width="0.7" />
    <rect x="30" y="86" width="12" height="20" fill={hut.door} />
    <circle cx="39" cy="96" r="0.9" fill={g.accent} opacity="0.6" />
    <!-- SURF HUT sign -->
    <rect x="10" y="76" width="34" height="7" rx="0.4" fill="#2A2218" />
    <text
      x="27"
      y="81.2"
      text-anchor="middle"
      font-size="4.2"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.4"
      fill={signCream}>surf hut</text
    >
    <!-- board rack -->
    <g stroke={woodDark} stroke-width="1.4" stroke-linecap="round" opacity="0.85">
      <line x1="50" y1="88" x2="58" y2="108" />
      <line x1="54" y1="86" x2="62" y2="108" />
    </g>
    <ellipse cx="54" cy="86" rx="2.5" ry="1.2" fill={summer ? '#3A8AA0' : '#5A7A88'} />
    <ellipse cx="58" cy="84" rx="2.2" ry="1" fill={fall ? '#C45A3A' : '#E8C848'} opacity="0.85" />
    {#if winter}
      <path d="M2 74 L27 58 L52 74 Z" fill={g.accent} opacity="0.55" />
    {/if}
  </g>

  <!-- ===== LOTION KIOSK (right of hut, still left of corridor) ===== -->
  <g class="kiosk" transform="translate(58 0)">
    <rect x="0" y="96" width="22" height="22" fill={winter ? '#8A7A6A' : '#A88860'} />
    <path d="M-2 96 L11 88 L24 96 Z" fill={fall ? '#8A4A32' : summer ? '#C45A3A' : '#6A8A78'} />
    <rect x="2" y="100" width="18" height="8" rx="0.4" fill="#2A2218" />
    <text
      x="11"
      y="105.5"
      text-anchor="middle"
      font-size="2.6"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}>meowtan</text
    >
    <text
      x="11"
      y="109"
      text-anchor="middle"
      font-size="2.4"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#E8C070">lotion</text
    >
    <!-- bottles -->
    <rect x="4" y="110" width="3" height="5" rx="0.4" fill="#E8D090" />
    <rect x="9" y="111" width="2.5" height="4" rx="0.3" fill="#F0A0C0" opacity="0.85" />
    <rect x="14" y="110" width="3" height="5" rx="0.4" fill="#90C8E0" />
  </g>

  <!-- ===== LIFEGUARD STAND (right edge) ===== -->
  <g class="lifeguard" transform="translate(368 0)">
    <rect x="8" y="70" width="3" height="48" fill={woodDark} />
    <rect x="36" y="70" width="3" height="48" fill={woodDark} />
    <rect x="6" y="88" width="36" height="3" fill={wood} />
    <rect x="6" y="100" width="36" height="3" fill={wood} />
    <!-- chair deck -->
    <rect x="4" y="58" width="40" height="16" rx="1" fill={woodLite} />
    <rect x="4" y="58" width="40" height="3" fill={wood} />
    <path d="M2 58 L24 48 L46 58 Z" fill={summer ? '#C84848' : fall ? '#8A4A32' : '#6A7080'} />
    <!-- ladder -->
    <g stroke={woodDark} stroke-width="1.2" fill="none">
      <line x1="2" y1="72" x2="2" y2="118" />
      <line x1="8" y1="72" x2="8" y2="118" />
      <line x1="2" y1="80" x2="8" y2="80" />
      <line x1="2" y1="90" x2="8" y2="90" />
      <line x1="2" y1="100" x2="8" y2="100" />
      <line x1="2" y1="110" x2="8" y2="110" />
    </g>
    <!-- rescue buoy -->
    <ellipse cx="28" cy="66" rx="5" ry="2.2" fill="#C84848" />
    <rect x="26.5" y="63" width="3" height="6" fill="#F2E8D4" opacity="0.7" />
    <!-- flag (animated group 1: sk-flap) -->
    <line x1="42" y1="48" x2="42" y2="58" stroke={woodDark} stroke-width="1.2" />
    <g class="sk-flap" fill={summer ? '#C84848' : winter ? '#A0B0C0' : '#E07830'} opacity="0.9">
      <path d="M42 48 L54 51 L42 54 Z" />
    </g>
    {#if winter}
      <rect x="4" y="56" width="40" height="3" fill={g.accent} opacity="0.75" />
    {/if}
  </g>

  <!-- ===== DUNE GRASS (animated group 2: sk-sway) ===== -->
  <g class="sk-sway" stroke={winter ? '#8A9A88' : fall ? '#8A7A48' : '#6A8A48'} stroke-width="1.1" fill="none" stroke-linecap="round" opacity="0.75">
    {#each duneTufts as t}
      <line x1={t.x} y1="116" x2={t.x + t.lean} y2={winter ? 110 : 106} />
      <line x1={t.x + 2} y1="116" x2={t.x + 2 + t.lean} y2={winter ? 111 : 107} />
      <line x1={t.x - 2} y1="116" x2={t.x - 2 + t.lean * 0.5} y2={winter ? 112 : 108} />
    {/each}
  </g>

  <!-- Near sand rises (edges only — corridor clear) -->
  <path
    d="M-10 118 C25 110, 55 108, 78 114 L78 118 Z"
    fill={g.near}
    opacity="0.75"
  />
  <path
    d="M342 118 C365 110, 395 106, 430 112 L430 118 Z"
    fill={g.near}
    opacity="0.75"
  />

  <!-- Beach path texture along ground edge -->
  <g fill={g.detail} opacity="0.25">
    <ellipse cx="30" cy="117" rx="8" ry="1.4" />
    <ellipse cx="60" cy="116" rx="6" ry="1.2" />
    <ellipse cx="360" cy="117" rx="7" ry="1.3" />
    <ellipse cx="395" cy="116" rx="6" ry="1.2" />
  </g>

  <!-- ===== SEASONAL PROPS ===== -->
  {#if summer}
    <!-- umbrellas (edges) -->
    <g class="umbrellas">
      <g transform="translate(64 108)">
        <line x1="0" y1="0" x2="0" y2="-22" stroke={woodDark} stroke-width="1.3" />
        <path d="M0 -22 L-12 -18 L-4 -20 Z" fill="#C84848" opacity="0.95" />
        <path d="M0 -22 L-4 -20 L4 -20 Z" fill="#F2E8D4" opacity="0.95" />
        <path d="M0 -22 L4 -20 L12 -18 Z" fill="#3A6A98" opacity="0.95" />
        <ellipse cx="0" cy="-20" rx="13" ry="4.5" fill="#C84848" opacity="0.3" />
        <ellipse cx="4" cy="2" rx="6" ry="2" fill="#E8C070" opacity="0.5" />
      </g>
      <g transform="translate(352 110)">
        <line x1="0" y1="0" x2="0" y2="-20" stroke={woodDark} stroke-width="1.2" />
        <ellipse cx="0" cy="-20" rx="12" ry="4.5" fill="#3A6A98" opacity="0.85" />
        <path d="M-12 -20 L0 -24 L12 -20 Z" fill="#F2E8D4" opacity="0.35" />
        <ellipse cx="-3" cy="2" rx="5" ry="1.8" fill="#E8C070" opacity="0.45" />
      </g>
    </g>
    <!-- heat haze (animated group 3: sk-mist) -->
    <g class="sk-mist" opacity="0.12">
      <ellipse cx="100" cy="100" rx="40" ry="5" fill="#fff" />
      <ellipse cx="210" cy="104" rx="48" ry="4" fill="#fff" />
      <ellipse cx="320" cy="102" rx="36" ry="4.5" fill="#fff" />
    </g>
  {:else if winter}
    <!-- muted sand wash -->
    <rect x="0" y="108" width="420" height="12" fill="#C8D0D8" opacity="0.25" />
    <g fill={g.accent} opacity="0.7">
      {#each snowFlecks as s}
        <circle cx={s.x} cy={s.y} r="1.1" />
      {/each}
    </g>
    <ellipse cx="40" cy="112" rx="16" ry="2.5" fill={g.accent} opacity="0.45" />
    <ellipse cx="380" cy="113" rx="14" ry="2.2" fill={g.accent} opacity="0.4" />
    <!-- cold mist (animated group 3: sk-mist) -->
    <g class="sk-mist" opacity="0.22">
      <ellipse cx="80" cy="96" rx="50" ry="6" fill="#E8F0F4" />
      <ellipse cx="220" cy="100" rx="60" ry="5" fill="#E8F0F4" />
      <ellipse cx="360" cy="98" rx="45" ry="5.5" fill="#E8F0F4" />
    </g>
  {:else if spring}
    <g class="shells">
      {#each shells as s}
        <ellipse cx={s.x} cy={s.y} rx={s.r} ry={s.r * 0.7} fill={s.c} opacity="0.9" />
        <path
          d={`M${s.x - s.r * 0.5} ${s.y} Q${s.x} ${s.y - s.r} ${s.x + s.r * 0.5} ${s.y}`}
          fill="none"
          stroke="#C8A888"
          stroke-width="0.5"
          opacity="0.6"
        />
      {/each}
    </g>
    <g class="flowers">
      {#each springFlowers as fl}
        <line x1={fl.x} y1={fl.y} x2={fl.x} y2={fl.y - 5} stroke="#4A8A48" stroke-width="0.9" />
        <circle cx={fl.x} cy={fl.y - 6} r="1.7" fill={fl.c} />
        <circle cx={fl.x - 1.3} cy={fl.y - 5.2} r="1" fill={fl.c} opacity="0.85" />
        <circle cx={fl.x + 1.3} cy={fl.y - 5.2} r="1" fill={fl.c} opacity="0.85" />
      {/each}
    </g>
    <!-- soft sea mist -->
    <g class="sk-mist" opacity="0.1">
      <ellipse cx="140" cy="102" rx="44" ry="4" fill="#fff" />
      <ellipse cx="280" cy="104" rx="40" ry="3.5" fill="#fff" />
    </g>
  {:else}
    <!-- fall: cooler foam + driftwood -->
    <g fill={woodDark} opacity="0.7">
      <ellipse cx="70" cy="115" rx="8" ry="2" />
      <ellipse cx="74" cy="114" rx="3" ry="1.4" />
      <ellipse cx="350" cy="116" rx="7" ry="1.8" />
    </g>
    <g fill={g.accent} opacity="0.55">
      <ellipse cx="42" cy="112" rx="2.5" ry="1.4" />
      <ellipse cx="56" cy="114" rx="2" ry="1.2" />
      <ellipse cx="372" cy="113" rx="2.2" ry="1.3" />
    </g>
    <g class="sk-mist" opacity="0.1">
      <ellipse cx="180" cy="100" rx="50" ry="4.5" fill="#B0C8D0" />
      <ellipse cx="300" cy="104" rx="40" ry="3.5" fill="#B0C8D0" />
    </g>
  {/if}

  <!-- Ground plane — top edge y=118; clear mid-screen run corridor ~80–340 -->
  <rect x="0" y="118" width="420" height="22" fill={g.near} />
  <rect x="0" y="118" width="420" height="3" fill={g.detail} opacity="0.22" />
  <rect x="0" y="124" width="420" height="16" fill={g.path} opacity="0.5" />
  <!-- wet sand strip meeting water -->
  <rect x="0" y="118" width="420" height="4" fill={water.cool} opacity="0.2" />
  <g fill={g.near} opacity="0.4">
    {#each corridorNibbles as n}
      <ellipse cx={n.x} cy="119" rx={n.rx} ry="1.6" />
    {/each}
  </g>
  <!-- soft path through corridor (no props) -->
  <path
    d="M85 118 C130 122, 180 126, 220 124 C260 122, 300 120, 335 118 L335 128 C290 130, 220 132, 150 128 C110 126, 85 122, 85 118 Z"
    fill={g.path}
    opacity="0.65"
  />

  {#if winter}
    <rect x="0" y="116" width="420" height="4" fill={g.accent} opacity="0.55" />
  {/if}
</g>
