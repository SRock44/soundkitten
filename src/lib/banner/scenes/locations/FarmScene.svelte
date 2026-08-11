<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()

  const g = $derived(groundFor('farm', weather))
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  /** Weathered barn red — never brand orange */
  const barn = $derived(
    winter
      ? { wall: '#8A6A62', shadow: '#6E524C', trim: '#5A4440', roof: '#5A6470', door: '#4A3A36' }
      : fall
        ? { wall: '#A04032', shadow: '#7A2E24', trim: '#6A2820', roof: '#4A3A32', door: '#3A2820' }
        : summer
          ? { wall: '#C24A3A', shadow: '#9A382C', trim: '#7A2E24', roof: '#3E342C', door: '#3A2820' }
          : { wall: '#B8483A', shadow: '#8E3630', trim: '#6E2C26', roof: '#423832', door: '#3A2820' },
  )

  const silo = $derived(
    winter
      ? { body: '#A8B4C0', band: '#7A8896', cap: '#6A7888', door: '#5A6878' }
      : { body: '#B8B0A0', band: '#8A8274', cap: '#6E665A', door: '#5A5248' },
  )

  const wood = $derived(winter ? '#8A7A68' : '#6A5440')
  const rail = $derived(winter ? '#9A8A78' : '#5A4634')
  const hay = $derived(winter ? '#C8B898' : fall ? '#D4A848' : summer ? '#E8C858' : '#D8B860')
  const signCream = '#F2E8D4'
  const signInk = '#1A1E24'

  const wheatLeft = [
    { x: 24 },
    { x: 40 },
    { x: 56 },
    { x: 72 },
    { x: 88 },
    { x: 104 },
    { x: 120 },
    { x: 136 },
    { x: 152 },
  ]
  const wheatMid = [{ x: 205 }, { x: 220 }, { x: 235 }]
  const fencePosts = [
    { x: 20, last: false },
    { x: 56, last: false },
    { x: 92, last: false },
    { x: 128, last: false },
    { x: 164, last: true },
  ]
  const barnBattens = [
    { x: 300 },
    { x: 314 },
    { x: 328 },
    { x: 342 },
    { x: 356 },
  ]
  const siloRivets = [{ y: 62 }, { y: 82 }, { y: 102 }]
  const cornStalks = [
    { x: 172, lean: -1 },
    { x: 182, lean: 1 },
    { x: 192, lean: -1 },
  ]
  const springFlowers = [
    { x: 28, y: 112, c: '#F0A0C8' },
    { x: 68, y: 113, c: '#F0D060' },
    { x: 148, y: 112, c: '#90C8F0' },
    { x: 198, y: 114, c: '#E090C0' },
  ]
  const grassTufts = [{ x: 34 }, { x: 90 }, { x: 160 }, { x: 210 }]
  const pathStones = [
    { x: 118, y: 116, rx: 3 },
    { x: 158, y: 114, rx: 2.5 },
    { x: 198, y: 116, rx: 2.4 },
  ]
  const corridorNibbles = [
    { x: 14, rx: 5 },
    { x: 48, rx: 6 },
    { x: 260, rx: 5 },
    { x: 300, rx: 7 },
    { x: 340, rx: 5 },
    { x: 380, rx: 6 },
  ]
  const winterSnowCaps = [{ x: 40 }, { x: 120 }, { x: 260 }, { x: 340 }]
  const ladderRungs = [{ y: 72 }, { y: 80 }, { y: 88 }, { y: 96 }, { y: 104 }]
  const awningStripes = [
    { x: 6 },
    { x: 11 },
    { x: 16 },
    { x: 21 },
    { x: 26 },
  ]
</script>

<g class="farm" aria-hidden="true">
  <!-- Far hills -->
  <path
    d="M-10 118 C40 78, 90 72, 150 88 C200 102, 240 76, 300 82 C350 88, 390 70, 430 90 L430 118 Z"
    fill={g.far}
    opacity="0.55"
  />
  <path
    d="M20 100 Q70 88 120 96"
    fill="none"
    stroke={g.mid}
    stroke-width="1.2"
    opacity="0.25"
  />
  <path
    d="M260 92 Q310 80 360 90"
    fill="none"
    stroke={g.mid}
    stroke-width="1.2"
    opacity="0.22"
  />

  <!-- Mid hills / fields -->
  <path
    d="M-10 118 C50 92, 110 86, 170 96 C220 104, 270 88, 330 94 C380 100, 410 92, 430 100 L430 118 Z"
    fill={g.mid}
    opacity="0.85"
  />
  <path
    d="M0 108 Q60 98 110 106"
    fill="none"
    stroke={g.near}
    stroke-width="1.4"
    opacity="0.28"
  />
  <path
    d="M140 104 Q200 94 250 102"
    fill="none"
    stroke={g.near}
    stroke-width="1.3"
    opacity="0.24"
  />

  <!-- Near rolling rise — left pasture -->
  <path
    d="M-10 118 C30 104, 70 100, 120 108 C150 114, 180 106, 220 110 L220 118 Z"
    fill={g.near}
    opacity="0.7"
  />

  <!-- Crop / field texture (animated group 1: sk-sway) -->
  {#if summer}
    <g class="sk-sway" stroke={g.accent} stroke-width="1" opacity="0.55" stroke-linecap="round">
      {#each wheatLeft as w, i}
        <line x1={w.x} y1={104 + (i % 3)} x2={w.x + 1} y2={98 + (i % 4)} />
        <line x1={w.x + 3} y1={106 + (i % 2)} x2={w.x + 4} y2={100 + (i % 3)} />
      {/each}
      {#each wheatMid as w}
        <line x1={w.x} y1="108" x2={w.x + 1} y2="102" />
      {/each}
    </g>
  {:else if fall}
    <g fill="none" stroke={g.accent} stroke-width="2.2" opacity="0.35">
      <path d="M0 106 Q70 98 140 106" />
      <path d="M10 110 Q80 102 150 110" />
    </g>
  {:else if spring}
    <g fill="none" stroke={g.accent} stroke-width="1.5" opacity="0.3">
      <path d="M5 108 Q55 100 105 108" />
      <path d="M15 112 Q65 104 115 112" />
    </g>
  {:else}
    <g fill="none" stroke={g.detail} stroke-width="1.2" opacity="0.2">
      <path d="M0 108 Q80 100 160 108" />
      <path d="M10 112 Q90 104 170 112" />
    </g>
  {/if}

  <!-- Silo + weather vane -->
  <g class="silo">
    <rect x="268" y="42" width="26" height="76" rx="2" fill={silo.body} />
    <rect x="268" y="42" width="7" height="76" fill="#000" opacity="0.08" />
    <rect x="287" y="42" width="5" height="76" fill="#fff" opacity="0.08" />
    <rect x="267" y="58" width="28" height="2.5" fill={silo.band} />
    <rect x="267" y="78" width="28" height="2.5" fill={silo.band} />
    <rect x="267" y="98" width="28" height="2.5" fill={silo.band} />
    <g fill={silo.band} opacity="0.7">
      {#each siloRivets as r}
        <circle cx="271" cy={r.y} r="0.8" />
        <circle cx="291" cy={r.y} r="0.8" />
      {/each}
    </g>
    <rect x="276" y="88" width="10" height="14" rx="1" fill={silo.door} />
    <rect x="278" y="92" width="3" height="2" fill={g.accent} opacity="0.4" />
    <ellipse cx="281" cy="42" rx="15" ry="7" fill={silo.cap} />
    <ellipse cx="281" cy="40" rx="12" ry="4.5" fill={silo.body} opacity="0.55" />
    <rect x="279" y="32" width="4" height="6" fill={silo.band} />
    <ellipse cx="281" cy="32" rx="3" ry="1.5" fill={silo.cap} />
    <!-- weather vane (static) -->
    <line x1="281" y1="22" x2="281" y2="32" stroke={silo.band} stroke-width="1.2" />
    <circle cx="281" cy="22" r="1.3" fill={rail} />
    <path d="M281 22 L292 20 L281 24 Z" fill={rail} />
    <path d="M281 22 L272 23.5 L281 25 Z" fill={wood} opacity="0.85" />
    <line x1="281" y1="18" x2="281" y2="26" stroke={silo.band} stroke-width="0.7" />
    <line x1="277" y1="22" x2="285" y2="22" stroke={silo.band} stroke-width="0.7" />
    {#if winter}
      <ellipse cx="281" cy="38" rx="14" ry="4" fill={g.accent} opacity="0.85" />
      <rect x="268" y="42" width="26" height="4" fill={g.accent} opacity="0.55" />
    {/if}
  </g>

  <!-- Red barn -->
  <g class="barn">
    <path d="M372 78 L402 88 L402 118 L372 118 Z" fill={barn.shadow} />
    <path d="M372 78 L402 88 L372 88 Z" fill={barn.roof} opacity="0.9" />

    <rect x="292" y="58" width="84" height="60" fill={barn.wall} />
    <g stroke={barn.shadow} stroke-width="1" opacity="0.35">
      {#each barnBattens as b}
        <line x1={b.x} y1="58" x2={b.x} y2="118" />
      {/each}
    </g>
    <rect x="348" y="72" width="14" height="22" fill={barn.shadow} opacity="0.25" />
    <rect x="298" y="90" width="18" height="12" fill={barn.trim} opacity="0.2" />

    <path d="M292 58 L334 28 L376 58 Z" fill={barn.wall} />
    <path d="M292 58 L334 28 L376 58 Z" fill={barn.shadow} opacity="0.12" />

    <path d="M286 60 L334 26 L338 28 L290 62 Z" fill={barn.roof} />
    <path d="M330 28 L382 60 L378 64 L334 30 Z" fill={barn.roof} />
    <g stroke={barn.trim} stroke-width="0.8" opacity="0.45">
      <line x1="292" y1="54" x2="332" y2="30" />
      <line x1="304" y1="50" x2="340" y2="32" />
      <line x1="338" y1="30" x2="372" y2="52" />
      <line x1="346" y1="34" x2="378" y2="56" />
    </g>
    <line x1="334" y1="26" x2="334" y2="30" stroke={barn.trim} stroke-width="1.5" />
    <path d="M286 60 L292 58 L376 58 L382 60" fill="none" stroke={barn.trim} stroke-width="1.2" opacity="0.6" />

    <!-- lightning rod on peak -->
    <line x1="334" y1="18" x2="334" y2="26" stroke={silo.band} stroke-width="1.1" />
    <circle cx="334" cy="18" r="1.1" fill={silo.band} />
    <path d="M334 16 L332.5 19 L335.5 19 Z" fill={rail} />

    {#if winter}
      <path d="M286 60 L334 26 L338 28 L290 62 Z" fill={g.accent} opacity="0.75" />
      <path d="M330 28 L382 60 L378 64 L334 30 Z" fill={g.accent} opacity="0.65" />
      <ellipse cx="334" cy="30" rx="6" ry="2.5" fill={g.accent} />
    {/if}

    <!-- loft door -->
    <rect x="322" y="40" width="24" height="20" rx="1" fill={barn.door} />
    <line x1="334" y1="40" x2="334" y2="60" stroke={barn.trim} stroke-width="1" opacity="0.5" />
    <circle cx="330" cy="50" r="1.2" fill={g.accent} opacity="0.45" />
    <circle cx="338" cy="50" r="1.2" fill={g.accent} opacity="0.45" />
    <rect x="318" y="58" width="32" height="2.5" fill={wood} />
    <rect x="320" y="56" width="2" height="5" fill={wood} />
    <rect x="346" y="56" width="2" height="5" fill={wood} />

    <!-- SUNNY ACRES roof / gable sign -->
    <rect x="316" y="30" width="36" height="8" rx="0.6" fill={barn.door} opacity="0.92" />
    <rect x="317" y="31" width="34" height="6" rx="0.4" fill={signCream} opacity="0.92" />
    <text
      x="334"
      y="36"
      text-anchor="middle"
      font-size="4.6"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="-0.2"
      fill={signInk}>purr acres</text
    >

    <!-- side wall BARN plate -->
    <rect x="356" y="88" width="18" height="7" rx="0.4" fill={barn.door} />
    <text
      x="365"
      y="93.2"
      text-anchor="middle"
      font-size="4.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.4"
      fill={signCream}>hi-fi</text
    >

    <!-- windows with pane crosses -->
    <g>
      <rect x="302" y="68" width="14" height="12" fill="#2A3A48" />
      <rect x="302" y="68" width="14" height="12" fill="none" stroke={barn.trim} stroke-width="1.5" />
      <line x1="309" y1="68" x2="309" y2="80" stroke={barn.trim} stroke-width="1" />
      <line x1="302" y1="74" x2="316" y2="74" stroke={barn.trim} stroke-width="1" />
      <line x1="305.5" y1="68" x2="305.5" y2="80" stroke={barn.trim} stroke-width="0.5" opacity="0.55" />
      <line x1="312.5" y1="68" x2="312.5" y2="80" stroke={barn.trim} stroke-width="0.5" opacity="0.55" />
      <line x1="302" y1="71" x2="316" y2="71" stroke={barn.trim} stroke-width="0.5" opacity="0.55" />
      <line x1="302" y1="77" x2="316" y2="77" stroke={barn.trim} stroke-width="0.5" opacity="0.55" />

      <rect x="352" y="68" width="14" height="12" fill="#2A3A48" />
      <rect x="352" y="68" width="14" height="12" fill="none" stroke={barn.trim} stroke-width="1.5" />
      <line x1="359" y1="68" x2="359" y2="80" stroke={barn.trim} stroke-width="1" />
      <line x1="352" y1="74" x2="366" y2="74" stroke={barn.trim} stroke-width="1" />
      <line x1="355.5" y1="68" x2="355.5" y2="80" stroke={barn.trim} stroke-width="0.5" opacity="0.55" />
      <line x1="362.5" y1="68" x2="362.5" y2="80" stroke={barn.trim} stroke-width="0.5" opacity="0.55" />
      <line x1="352" y1="71" x2="366" y2="71" stroke={barn.trim} stroke-width="0.5" opacity="0.55" />
      <line x1="352" y1="77" x2="366" y2="77" stroke={barn.trim} stroke-width="0.5" opacity="0.55" />
    </g>

    <rect x="318" y="82" width="32" height="36" fill={barn.door} />
    <line x1="334" y1="82" x2="334" y2="118" stroke={barn.trim} stroke-width="1.5" opacity="0.55" />
    <line x1="320" y1="84" x2="332" y2="114" stroke={barn.trim} stroke-width="1.2" opacity="0.5" />
    <line x1="332" y1="84" x2="320" y2="114" stroke={barn.trim} stroke-width="1.2" opacity="0.5" />
    <line x1="336" y1="84" x2="348" y2="114" stroke={barn.trim} stroke-width="1.2" opacity="0.5" />
    <line x1="348" y1="84" x2="336" y2="114" stroke={barn.trim} stroke-width="1.2" opacity="0.5" />
    <circle cx="330" cy="98" r="1.4" fill={g.accent} opacity="0.4" />
    <circle cx="338" cy="98" r="1.4" fill={g.accent} opacity="0.4" />

    <!-- ladder on barn side -->
    <g stroke={wood} stroke-width="1.3" stroke-linecap="round">
      <line x1="378" y1="70" x2="378" y2="116" />
      <line x1="384" y1="70" x2="384" y2="116" />
    </g>
    <g stroke={wood} stroke-width="1.1" stroke-linecap="round">
      {#each ladderRungs as r}
        <line x1="378" y1={r.y} x2="384" y2={r.y} />
      {/each}
    </g>

    <rect x="290" y="116" width="88" height="2.5" fill={winter ? g.detail : '#5A4A3A'} opacity="0.7" />
  </g>

  <!-- Split-rail fence (~40% fewer posts) -->
  <g class="fence" fill={rail}>
    {#each fencePosts as p}
      <rect x={p.x} y="96" width="3.5" height="22" rx="0.5" />
      <rect x={p.x - 0.5} y="95" width="4.5" height="2.5" fill={wood} />
      {#if !p.last}
        <rect x={p.x + 3} y="100" width="33" height="2.2" rx="0.4" />
        <rect x={p.x + 3} y="108" width="33" height="2" rx="0.4" />
        {#if !winter}
          <rect x={p.x + 3} y="100" width="33" height="2.2" fill={wood} opacity="0.25" />
        {/if}
      {/if}
    {/each}
  </g>

  <!-- Mailbox on post near fence end -->
  <g class="mailbox">
    <rect x="174" y="100" width="2.2" height="18" fill={wood} />
    <rect x="170" y="96" width="11" height="7" rx="1" fill={winter ? '#7A8894' : '#6A5A4A'} />
    <rect x="171" y="97" width="9" height="3.5" rx="0.4" fill={winter ? '#9AA8B4' : '#8A7A68'} opacity="0.55" />
    <rect x="179.5" y="98" width="1.4" height="3" fill={rail} />
    <path d="M170 96 L175.5 93 L181 96 Z" fill={winter ? '#5A6874' : '#4A3A30'} />
  </g>

  <!-- PICK YOUR OWN wooden sign (on fence line) -->
  <g class="yard-sign">
    <rect x="44" y="100" width="2" height="18" fill={wood} />
    <rect x="36" y="92" width="18" height="12" rx="0.5" fill="#C8A878" />
    <rect x="37" y="93" width="16" height="10" rx="0.3" fill="#D8B888" opacity="0.55" />
    <text
      x="45"
      y="97.5"
      text-anchor="middle"
      font-size="3.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="-0.15"
      fill={signInk}>mix your</text
    >
    <text
      x="45"
      y="102"
      text-anchor="middle"
      font-size="3.4"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.1"
      fill={signInk}>tunes</text
    >
  </g>

  <!-- NO TRESPASSING (right of fence, low) -->
  <g class="trespass-sign">
    <rect x="188" y="104" width="1.8" height="14" fill={wood} />
    <rect x="178" y="98" width="22" height="8" rx="0.4" fill="#6A4A28" />
    <text
      x="189"
      y="103.5"
      text-anchor="middle"
      font-size="2.8"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="-0.15"
      fill={signCream}>cats welcome</text
    >
  </g>

  <!-- Dirt path → ground strip -->
  <path
    d="M90 118
       C110 112, 130 108, 155 110
       C185 112, 210 116, 240 118
       L240 140 L90 140 Z"
    fill={g.path}
    opacity="0.55"
  />
  <path
    d="M100 118
       C125 111, 150 109, 175 112
       C200 115, 220 117, 235 118"
    fill="none"
    stroke={g.detail}
    stroke-width="1.2"
    opacity="0.35"
  />
  <g fill={g.detail} opacity="0.35">
    {#each pathStones as s}
      <ellipse cx={s.x} cy={s.y} rx={s.rx} ry="1.1" />
    {/each}
  </g>

  <!-- Hay bales -->
  <g class="hay">
    <ellipse cx="52" cy="114" rx="14" ry="6.5" fill={hay} />
    <ellipse cx="52" cy="111" rx="13" ry="5.5" fill={hay} opacity="0.85" />
    <g stroke={wood} stroke-width="0.9" opacity="0.45" fill="none">
      <ellipse cx="52" cy="112" rx="10" ry="4" />
      <ellipse cx="52" cy="112" rx="5" ry="2" />
      <line x1="42" y1="112" x2="62" y2="112" />
    </g>
    <ellipse cx="78" cy="116" rx="11" ry="5" fill={hay} />
    <ellipse cx="78" cy="113.5" rx="10" ry="4.2" fill={hay} opacity="0.9" />
    <ellipse cx="72" cy="110" rx="9" ry="4" fill={hay} />
    <g stroke={wood} stroke-width="0.8" opacity="0.4" fill="none">
      <ellipse cx="78" cy="114" rx="7" ry="2.8" />
      <ellipse cx="72" cy="110" rx="6" ry="2.4" />
    </g>
  </g>

  <!-- Roadside egg stand (left of run corridor, beside hay) -->
  <g class="egg-stand" transform="translate(64 0)">
    <rect x="4" y="108" width="26" height="10" fill={wood} />
    <rect x="6" y="104" width="22" height="5" fill={winter ? '#8A7A6A' : '#A89068'} />
    <path d="M2 104 L17 96 L32 104 Z" fill={fall ? '#8A4A32' : '#C45A3A'} />
    <g fill={signCream} opacity="0.75">
      {#each awningStripes as s}
        <rect x={s.x} y="98" width="2.2" height="7" transform="skewX(-14)" />
      {/each}
    </g>
    <path d="M2 104 L17 96 L32 104" fill="none" stroke={barn.trim} stroke-width="0.9" opacity="0.7" />
    <rect x="8" y="106" width="18" height="6" rx="0.4" fill="#2A2218" />
    <text
      x="17"
      y="110.5"
      text-anchor="middle"
      font-size="4"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.3"
      fill={signCream}>EGGS</text
    >
    <rect x="10" y="100" width="14" height="4" rx="0.3" fill="#F0E8C8" />
    <text
      x="17"
      y="103.2"
      text-anchor="middle"
      font-size="2.8"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.2"
      fill="#8A3828">purrfect</text
    >
    <ellipse cx="12" cy="115" rx="2" ry="1.4" fill="#F0E8D0" opacity="0.85" />
    <ellipse cx="18" cy="115.5" rx="1.8" ry="1.2" fill="#F0E8D0" opacity="0.8" />
    <ellipse cx="24" cy="115" rx="2" ry="1.3" fill="#E8D8B8" opacity="0.8" />
  </g>

  <!-- Chicken coop -->
  <g class="coop">
    <rect x="8" y="98" width="28" height="20" fill={winter ? '#8A7A6A' : '#8B6540'} />
    <path d="M6 98 L22 86 L38 98 Z" fill={winter ? '#6A7080' : '#5A4634'} />
    {#if winter}
      <path d="M6 98 L22 86 L38 98 Z" fill={g.accent} opacity="0.7" />
    {/if}
    <rect x="14" y="104" width="8" height="10" fill={barn.door} opacity="0.85" />
    <rect x="26" y="102" width="6" height="5" fill="#2A3A48" />
    <rect x="26" y="102" width="6" height="5" fill="none" stroke={wood} stroke-width="0.8" />
    <line x1="29" y1="102" x2="29" y2="107" stroke={wood} stroke-width="0.6" />
    <line x1="26" y1="104.5" x2="32" y2="104.5" stroke={wood} stroke-width="0.6" />
    <path d="M14 114 L8 118 L22 118 L22 114 Z" fill={wood} opacity="0.7" />
  </g>

  <!-- Water trough -->
  <g class="trough">
    <ellipse cx="210" cy="115" rx="16" ry="4" fill={winter ? '#7A8894' : '#6A5A48'} />
    <rect x="194" y="108" width="32" height="7" rx="1.5" fill={winter ? '#8A98A4' : '#7A6A54'} />
    <ellipse cx="210" cy="108" rx="16" ry="3.5" fill={winter ? '#A8C0D0' : '#5A8AAA'} opacity="0.65" />
    <rect x="194" y="108" width="32" height="2" fill="#000" opacity="0.1" />
  </g>

  <!-- Tractor silhouette -->
  <g class="tractor" fill={winter ? '#5A6470' : '#3A4238'} opacity="0.85">
    <rect x="238" y="100" width="22" height="10" rx="1.5" />
    <rect x="252" y="94" width="10" height="8" rx="1" />
    <rect x="246" y="96" width="3" height="5" />
    <circle cx="244" cy="112" r="5.5" fill={winter ? '#4A5460' : '#2A3028'} />
    <circle cx="244" cy="112" r="2.2" fill={g.path} opacity="0.5" />
    <circle cx="258" cy="113" r="3.8" fill={winter ? '#4A5460' : '#2A3028'} />
    <circle cx="258" cy="113" r="1.5" fill={g.path} opacity="0.5" />
    <rect x="254" y="98" width="5" height="3" rx="0.5" opacity="0.7" />
  </g>

  <!-- Scarecrow (animated group 2: sk-flap on sleeves) -->
  <g class="scarecrow">
    <line x1="124" y1="88" x2="124" y2="112" stroke={wood} stroke-width="2" />
    <line x1="112" y1="94" x2="136" y2="94" stroke={wood} stroke-width="2" />
    <path d="M114 94 L124 98 L134 94 L132 108 L116 108 Z" fill={fall ? '#6A7A4A' : spring ? '#5A8A6A' : '#6A6A58'} />
    <circle cx="124" cy="86" r="5" fill={hay} />
    <ellipse cx="124" cy="82" rx="7" ry="2" fill={rail} />
    <rect x="120" y="76" width="8" height="6" rx="1" fill={rail} />
    {#if !winter}
      <g class="sk-flap" fill={fall ? '#8A4A32' : '#7A5A40'} opacity="0.8">
        <path d="M112 94 L108 102 L114 96 Z" />
        <path d="M136 94 L140 102 L134 96 Z" />
      </g>
    {/if}
  </g>

  <!-- Seasonal props -->
  {#if fall}
    <g class="pumpkins">
      <ellipse cx="96" cy="114" rx="6" ry="4.5" fill="#C45A2A" />
      <ellipse cx="96" cy="113" rx="5" ry="3.5" fill="#D4783A" opacity="0.55" />
      <path d="M96 108 Q97 106 98 108" fill="none" stroke="#5A4634" stroke-width="1.2" />
      <ellipse cx="108" cy="115" rx="4.5" ry="3.5" fill="#A84828" />
      <path d="M108 110 Q109 108.5 110 110" fill="none" stroke="#5A4634" stroke-width="1" />
      <ellipse cx="198" cy="114" rx="5" ry="3.8" fill="#B05020" />
      <ellipse cx="206" cy="115" rx="3.5" ry="2.8" fill="#D4783A" />
      <path d="M198 109 Q199 107.5 200 109" fill="none" stroke="#5A4634" stroke-width="1" />
    </g>
    <!-- corn reuses animated group 1 slot (mutually exclusive with wheat) -->
    <g class="sk-sway" stroke="#6A7A38" stroke-width="1.3" fill="none" stroke-linecap="round">
      {#each cornStalks as c}
        <line x1={c.x} y1="116" x2={c.x + c.lean} y2="92" />
        <path d={`M${c.x} 100 Q${c.x + 5} 98 ${c.x + 4} 104`} stroke="#C4A040" stroke-width="2" />
        <path d={`M${c.x} 104 Q${c.x - 5} 102 ${c.x - 4} 108`} stroke="#A89038" stroke-width="1.5" />
      {/each}
    </g>
  {:else if winter}
    <path
      d="M30 92 Q70 78 110 90"
      fill="none"
      stroke={g.accent}
      stroke-width="3"
      opacity="0.55"
      stroke-linecap="round"
    />
    <path
      d="M250 88 Q300 76 350 86"
      fill="none"
      stroke={g.accent}
      stroke-width="3"
      opacity="0.5"
      stroke-linecap="round"
    />
    <g fill={g.accent} opacity="0.85">
      {#each fencePosts as p}
        <ellipse cx={p.x + 1.75} cy="95" rx="3" ry="1.6" />
      {/each}
    </g>
    <ellipse cx="52" cy="108" rx="11" ry="3" fill={g.accent} opacity="0.7" />
    <ellipse cx="78" cy="110" rx="8" ry="2.5" fill={g.accent} opacity="0.65" />
  {:else if spring}
    <g class="flowers">
      {#each springFlowers as fl}
        <g>
          <line x1={fl.x} y1={fl.y} x2={fl.x} y2={fl.y - 5} stroke="#4A8A48" stroke-width="0.9" />
          <circle cx={fl.x} cy={fl.y - 6} r="1.8" fill={fl.c} />
          <circle cx={fl.x - 1.4} cy={fl.y - 5.2} r="1.1" fill={fl.c} opacity="0.85" />
          <circle cx={fl.x + 1.4} cy={fl.y - 5.2} r="1.1" fill={fl.c} opacity="0.85" />
        </g>
      {/each}
    </g>
    <g stroke={g.near} stroke-width="1.1" stroke-linecap="round" opacity="0.7">
      {#each grassTufts as t}
        <line x1={t.x} y1="117" x2={t.x - 1} y2="112" />
        <line x1={t.x + 2} y1="117" x2={t.x + 3} y2="111" />
      {/each}
    </g>
  {:else}
    <!-- summer haze (animated group 3: sk-mist) -->
    <g class="sk-mist" opacity="0.14">
      <ellipse cx="80" cy="100" rx="36" ry="5" fill="#fff" />
      <ellipse cx="160" cy="104" rx="42" ry="4.5" fill="#fff" />
      <ellipse cx="230" cy="102" rx="28" ry="4" fill="#fff" />
    </g>
  {/if}

  <!-- Ground plane — top edge y=118; clear mid-screen run corridor -->
  <rect x="0" y="118" width="420" height="22" fill={g.near} />
  <rect x="0" y="118" width="420" height="3" fill={g.detail} opacity="0.25" />
  <rect x="0" y="124" width="420" height="16" fill={g.path} opacity="0.55" />
  <g fill={g.near} opacity="0.45">
    {#each corridorNibbles as n}
      <ellipse cx={n.x} cy="119" rx={n.rx} ry="1.8" />
    {/each}
  </g>
  <path
    d="M95 118 C130 122, 170 126, 220 124 C260 122, 280 120, 300 118 L300 128 C260 130, 200 132, 140 128 C110 126, 95 122, 95 118 Z"
    fill={g.path}
    opacity="0.7"
  />

  {#if winter}
    <rect x="0" y="116" width="420" height="5" fill={g.accent} opacity="0.9" />
    <g fill={g.accent} opacity="0.5">
      {#each winterSnowCaps as s}
        <ellipse cx={s.x} cy="120" rx="18" ry="2.5" />
      {/each}
    </g>
  {/if}
</g>
