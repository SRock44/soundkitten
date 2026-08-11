<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()

  const g = $derived(groundFor('forest', weather))
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  /** Seasonal canopy tints — never brand #FD6309 */
  const canopy = $derived.by(() => {
    switch (weather) {
      case 'fall':
        return {
          a: '#8B3A22',
          b: '#C45A2A',
          c: '#D4783A',
          d: '#C8A038',
          e: '#6E3A1A',
          pine: '#5A6E2E',
        }
      case 'winter':
        return {
          a: '#4A6A58',
          b: '#5A7A68',
          c: '#6A8A78',
          d: '#7A9A88',
          e: '#3A5A48',
          pine: '#3E5A4A',
        }
      case 'spring':
        return {
          a: '#6AB06A',
          b: '#8ED08A',
          c: '#A8E090',
          d: '#7EC87A',
          e: '#5AAA58',
          pine: '#4A9A58',
        }
      case 'summer':
        return {
          a: '#2F6B3A',
          b: '#3E8A4A',
          c: '#4A9A52',
          d: '#2A5A32',
          e: '#245032',
          pine: '#1E4A28',
        }
    }
  })

  const bark = $derived(winter ? '#4A5560' : '#3A2E24')
  const barkDeep = $derived(winter ? '#343C46' : '#2A2018')
  const moss = $derived(winter ? '#6A8A78' : summer ? '#3A7A42' : '#4A8A52')
  const litter = $derived(
    fall ? '#A05A28' : winter ? g.accent : spring ? '#6AAA58' : '#3A6A3A',
  )
  const signCream = '#F2E8D4'
  const board = '#8A6A40'
  const boardLight = '#A88858'

  const farPines = [
    { cx: 16, top: 52, w: 14 },
    { cx: 38, top: 48, w: 16 },
    { cx: 198, top: 46, w: 15 },
    { cx: 248, top: 44, w: 17 },
    { cx: 370, top: 50, w: 14 },
    { cx: 396, top: 46, w: 15 },
  ]

  const litterPads = [
    { cx: 22, cy: 116, rx: 14, ry: 4 },
    { cx: 70, cy: 115, rx: 15, ry: 3.5 },
    { cx: 340, cy: 116, rx: 15, ry: 4 },
    { cx: 390, cy: 115, rx: 13, ry: 3.5 },
  ]

  const mossPatches = [
    { cx: 54, cy: 117, rx: 8, ry: 2.2 },
    { cx: 96, cy: 116, rx: 6, ry: 1.8 },
    { cx: 328, cy: 117, rx: 7, ry: 2 },
    { cx: 404, cy: 118, rx: 6, ry: 1.6 },
  ]

  const stumpRings = [
    { cx: 96, cy: 120, r: 5 },
    { cx: 352, cy: 122, r: 4.2 },
  ]

  const hangingVines = [
    { x: 50, y1: 82, y2: 102 },
    { x: 56, y1: 78, y2: 98 },
    { x: 372, y1: 84, y2: 104 },
    { x: 378, y1: 80, y2: 100 },
  ]

  const blazeMarks = [
    { x: 76.2, y: 92, w: 3.2, h: 5 },
    { x: 110, y: 100, w: 2.6, h: 4 },
    { x: 340, y: 90, w: 3, h: 4.5 },
  ]

  const undergrowth = [12, 42, 72, 350, 380, 408]

  const fallLeaves = [
    { cx: 30, cy: 120, fill: '#C45A2A' },
    { cx: 55, cy: 122, fill: '#D4783A' },
    { cx: 85, cy: 119, fill: '#C8A038' },
    { cx: 340, cy: 120, fill: '#D4783A' },
    { cx: 368, cy: 122, fill: '#C45A2A' },
    { cx: 395, cy: 119, fill: '#A05A28' },
  ]

  const bridgePlanks = [
    { x: 168 },
    { x: 174 },
    { x: 180 },
    { x: 186 },
    { x: 192 },
  ]
</script>

<g class="forest" aria-hidden="true">
  <!-- Far mist band (1/2 animated groups) -->
  <g class="sk-mist" opacity="0.42">
    <ellipse cx="70" cy="58" rx="95" ry="28" fill={g.far} />
    <ellipse cx="210" cy="48" rx="120" ry="32" fill={g.far} />
    <ellipse cx="360" cy="56" rx="85" ry="26" fill={g.far} />
    <ellipse cx="140" cy="68" rx="70" ry="18" fill={g.mid} opacity="0.55" />
    <ellipse cx="300" cy="66" rx="80" ry="20" fill={g.mid} opacity="0.5" />
  </g>

  <!-- Far depth pines (with trunks) -->
  <g opacity="0.75">
    {#each farPines as p}
      <rect
        x={p.cx - 1.6}
        y={p.top + 26}
        width="3.2"
        height={118 - (p.top + 26)}
        fill={bark}
        opacity="0.9"
      />
      <path d={`M${p.cx} ${p.top} L${p.cx + p.w / 2} ${p.top + 28} L${p.cx - p.w / 2} ${p.top + 28} Z`} fill={g.far} />
      <path
        d={`M${p.cx} ${p.top + 10} L${p.cx + p.w / 2 + 2} ${p.top + 38} L${p.cx - p.w / 2 - 2} ${p.top + 38} Z`}
        fill={g.far}
        opacity="0.9"
      />
      <path
        d={`M${p.cx} ${p.top + 20} L${p.cx + p.w / 2 + 3} ${p.top + 48} L${p.cx - p.w / 2 - 3} ${p.top + 48} Z`}
        fill={g.mid}
        opacity="0.55"
      />
    {/each}
  </g>

  <!-- Mid-back deciduous silhouettes (trunks + canopy) -->
  <g opacity="0.7">
    <rect x="90" y="78" width="4.5" height="40" fill={bark} />
    <rect x="91" y="88" width="2" height="5" fill={barkDeep} opacity="0.6" />
    <ellipse cx="95" cy="72" rx="28" ry="22" fill={fall ? canopy.e : g.far} opacity="0.85" />
    <ellipse cx="108" cy="70" rx="16" ry="14" fill={fall ? canopy.a : g.mid} opacity="0.7" />

    <rect x="268" y="80" width="4.5" height="38" fill={bark} />
    <rect x="269" y="90" width="2" height="5" fill={barkDeep} opacity="0.6" />
    <ellipse cx="275" cy="70" rx="30" ry="20" fill={fall ? canopy.e : g.far} opacity="0.85" />
    <ellipse cx="292" cy="68" rx="18" ry="16" fill={fall ? canopy.b : g.mid} opacity="0.65" />

    <rect x="118" y="86" width="3.5" height="32" fill={bark} opacity="0.85" />
    <ellipse cx="122" cy="76" rx="18" ry="16" fill={fall ? canopy.d : g.far} opacity="0.55" />
  </g>

  <!-- Far-mid pines peeking over corridor (with trunks) -->
  <g opacity="0.55">
    <rect x="188.5" y="86" width="3" height="32" fill={bark} />
    <path d="M190 56 L198 86 L182 86 Z" fill={g.far} />
    <path d="M190 68 L200 98 L180 98 Z" fill={g.mid} opacity="0.8" />
    <rect x="226.5" y="86" width="3" height="32" fill={bark} />
    <path d="M228 54 L237 86 L219 86 Z" fill={g.far} />
    <path d="M228 66 L239 98 L217 98 Z" fill={g.mid} opacity="0.8" />
  </g>

  <!-- Left pine — trunk reaches into lowest canopy tier -->
  <g>
    <rect x="18" y="72" width="5" height="46" fill={bark} />
    <rect x="19" y="86" width="2" height="4" fill={barkDeep} opacity="0.7" />
    <path d="M20.5 42 L34 78 L7 78 Z" fill={canopy.pine} />
    <path d="M20.5 54 L36 88 L5 88 Z" fill={canopy.pine} opacity="0.92" />
    <path d="M20.5 66 L38 100 L3 100 Z" fill={canopy.a} opacity="0.88" />
    {#if winter}
      <path d="M20.5 42 L28 62 L13 62 Z" fill={g.accent} opacity="0.85" />
      <path d="M12 88 L20 86 L29 90" fill="none" stroke={g.accent} stroke-width="2.2" opacity="0.75" />
    {/if}
  </g>

  <!-- Near canopy sway (2/2 animated groups) — left deciduous -->
  <g class="sk-sway">
    <rect x="48" y="70" width="6" height="48" fill={bark} />
    <rect x="49" y="90" width="2" height="5" fill={barkDeep} opacity="0.65" />
    {#if winter}
      <path
        d="M51 58 L51 82 M51 66 L40 74 M51 66 L62 72 M51 74 L42 84 M51 74 L60 82"
        fill="none"
        stroke={barkDeep}
        stroke-width="1.6"
        stroke-linecap="round"
      />
      <circle cx="40" cy="74" r="1.2" fill={g.accent} opacity="0.7" />
      <circle cx="62" cy="72" r="1" fill={g.accent} opacity="0.65" />
    {:else}
      <ellipse cx="42" cy="70" rx="16" ry="14" fill={canopy.b} />
      <ellipse cx="58" cy="66" rx="14" ry="13" fill={canopy.c} />
      <ellipse cx="51" cy="60" rx="12" ry="11" fill={canopy.a} />
      <ellipse cx="36" cy="76" rx="10" ry="9" fill={canopy.d} opacity="0.9" />
      <ellipse cx="64" cy="74" rx="9" ry="8" fill={canopy.b} opacity="0.88" />
    {/if}
    {#if spring}
      <g fill={g.accent}>
        <circle cx="40" cy="66" r="2.2" />
        <circle cx="56" cy="60" r="1.8" />
        <circle cx="62" cy="72" r="2" />
      </g>
    {/if}
  </g>

  <!-- Left mid pine -->
  <g>
    <rect x="74" y="72" width="4.5" height="46" fill={bark} />
    <path d="M76.2 46 L88 80 L64 80 Z" fill={canopy.pine} />
    <path d="M76.2 58 L90 92 L62 92 Z" fill={canopy.a} opacity="0.9" />
    <path d="M76.2 70 L92 104 L60 104 Z" fill={canopy.pine} opacity="0.85" />
    {#if winter}
      <ellipse cx="76" cy="80" rx="10" ry="3" fill={g.accent} opacity="0.7" />
    {/if}
  </g>

  <!-- Right grove — pine + deciduous -->
  <g>
    <rect x="338" y="70" width="5" height="48" fill={bark} />
    <rect x="339" y="86" width="2" height="4" fill={barkDeep} opacity="0.7" />
    <path d="M340.5 40 L356 78 L325 78 Z" fill={canopy.pine} />
    <path d="M340.5 52 L358 88 L323 88 Z" fill={canopy.pine} opacity="0.92" />
    <path d="M340.5 64 L360 100 L321 100 Z" fill={canopy.a} opacity="0.88" />
    {#if winter}
      <path d="M340.5 40 L350 60 L331 60 Z" fill={g.accent} opacity="0.85" />
      <path d="M328 88 L340 86 L352 90" fill="none" stroke={g.accent} stroke-width="2.2" opacity="0.75" />
    {/if}
  </g>
  <g>
    <rect x="368" y="72" width="6" height="46" fill={bark} />
    <rect x="369" y="92" width="2" height="5" fill={barkDeep} opacity="0.65" />
    {#if winter}
      <path
        d="M371 60 L371 84 M371 68 L360 76 M371 68 L382 74 M371 76 L362 86 M371 76 L380 84"
        fill="none"
        stroke={barkDeep}
        stroke-width="1.6"
        stroke-linecap="round"
      />
      <circle cx="360" cy="76" r="1.1" fill={g.accent} opacity="0.7" />
    {:else}
      <ellipse cx="362" cy="70" rx="15" ry="13" fill={fall ? canopy.c : canopy.b} />
      <ellipse cx="378" cy="66" rx="14" ry="12" fill={fall ? canopy.d : canopy.c} />
      <ellipse cx="371" cy="58" rx="11" ry="10" fill={fall ? canopy.b : canopy.a} />
      <ellipse cx="356" cy="76" rx="9" ry="8" fill={fall ? canopy.a : canopy.d} opacity="0.9" />
    {/if}
    {#if spring}
      <g fill={g.accent}>
        <circle cx="360" cy="64" r="2" />
        <circle cx="376" cy="58" r="1.8" />
        <circle cx="382" cy="72" r="2.1" />
      </g>
    {/if}
  </g>

  <!-- Extra mid pines with trunks (fill gaps) -->
  <g opacity="0.85">
    <rect x="158" y="88" width="3.2" height="30" fill={bark} />
    <path d="M159.6 58 L168 88 L151 88 Z" fill={canopy.pine} />
    <path d="M159.6 70 L170 100 L149 100 Z" fill={canopy.a} opacity="0.88" />
    <rect x="258" y="86" width="3.2" height="32" fill={bark} />
    <path d="M259.6 54 L269 86 L250 86 Z" fill={canopy.pine} />
    <path d="M259.6 66 L271 98 L248 98 Z" fill={canopy.a} opacity="0.88" />
  </g>

  <!-- Corridor shoulder trees (short, clear run band) -->
  <g opacity="0.92">
    <g>
      <rect x="108" y="78" width="3.5" height="40" fill={bark} />
      {#if winter}
        <path
          d="M109.7 72 L109.7 92 M109.7 78 L102 84 M109.7 78 L117 82"
          fill="none"
          stroke={barkDeep}
          stroke-width="1.3"
          stroke-linecap="round"
        />
      {:else}
        <ellipse cx="104" cy="82" rx="11" ry="10" fill={fall ? canopy.d : canopy.b} />
        <ellipse cx="114" cy="78" rx="10" ry="9" fill={fall ? canopy.b : canopy.c} />
        <ellipse cx="109" cy="72" rx="8" ry="7" fill={canopy.a} />
      {/if}
    </g>
    <g>
      <rect x="300" y="76" width="3.5" height="42" fill={bark} />
      {#if winter}
        <path
          d="M301.7 70 L301.7 90 M301.7 76 L294 82 M301.7 76 L309 80"
          fill="none"
          stroke={barkDeep}
          stroke-width="1.3"
          stroke-linecap="round"
        />
      {:else}
        <ellipse cx="296" cy="80" rx="11" ry="10" fill={fall ? canopy.c : canopy.a} />
        <ellipse cx="306" cy="76" rx="10" ry="9" fill={fall ? canopy.d : canopy.b} />
        <ellipse cx="301" cy="70" rx="8" ry="7" fill={canopy.c} />
      {/if}
    </g>
    <g>
      <rect x="142" y="78" width="3" height="40" fill={bark} />
      <path d="M143.5 68 L151 96 L136 96 Z" fill={canopy.pine} opacity="0.9" />
      <path d="M143.5 78 L152 104 L135 104 Z" fill={canopy.a} opacity="0.85" />
    </g>
  </g>

  <!-- Trail blazes on trunks (paint marks) -->
  <g>
    {#each blazeMarks as b}
      <rect x={b.x} y={b.y} width={b.w} height={b.h} rx="0.4" fill="#C45A2A" opacity="0.9" />
      <rect x={b.x + 0.4} y={b.y + 0.5} width={b.w - 0.8} height={b.h - 1} rx="0.3" fill="#E07040" opacity="0.55" />
    {/each}
  </g>

  <!-- Birdhouse on left deciduous trunk (edge) -->
  <g class="birdhouse">
    <rect x="54" y="96" width="8" height="7" fill={board} />
    <path d="M53 96 L58 91 L63 96 Z" fill={boardLight} />
    <circle cx="58" cy="99.5" r="1.4" fill={barkDeep} />
    <rect x="57.2" y="103" width="1.6" height="4" fill={bark} />
    <line x1="54" y1="100" x2="51" y2="100" stroke={bark} stroke-width="1" />
  </g>

  <!-- Hanging vines (cheap static paths, edges) -->
  <g stroke={moss} stroke-width="1.1" fill="none" opacity="0.65" stroke-linecap="round">
    {#each hangingVines as v}
      <path d={`M${v.x} ${v.y1} Q${v.x + 2} ${(v.y1 + v.y2) / 2} ${v.x - 1} ${v.y2}`} />
    {/each}
  </g>

  {#if spring}
    <!-- Static light shafts (no local animation) -->
    <g opacity="0.12">
      <rect x="120" y="40" width="18" height="78" fill="#FFF8E0" transform="skewX(-12)" />
      <rect x="260" y="42" width="16" height="76" fill="#FFF8E0" transform="skewX(-14)" />
    </g>
  {/if}

  <!-- Wooden trail signpost (left edge) -->
  <g class="trail-sign">
    <rect x="88" y="86" width="3" height="32" fill={bark} rx="0.4" />
    <!-- TRAIL board -->
    <g transform="translate(91 88) rotate(-8)">
      <rect x="0" y="0" width="28" height="7" rx="0.5" fill={board} />
      <rect x="1" y="1" width="26" height="5" rx="0.3" fill={boardLight} opacity="0.45" />
      <text
        x="14"
        y="5.2"
        text-anchor="middle"
        font-size="4.2"
        font-weight="700"
        font-family="ui-rounded, system-ui, sans-serif"
        letter-spacing="0.3"
        fill={signCream}>kit path</text
      >
    </g>
    <!-- SUMMIT 2mi -->
    <g transform="translate(91 97) rotate(6)">
      <rect x="0" y="0" width="38" height="7" rx="0.5" fill={board} />
      <text
        x="19"
        y="5.1"
        text-anchor="middle"
        font-size="3.2"
        font-weight="700"
        font-family="ui-rounded, system-ui, sans-serif"
        letter-spacing="-0.15"
        fill={signCream}>purr peak 2mi</text
      >
    </g>
    <!-- CAMP -->
    <g transform="translate(91 106) rotate(-5)">
      <rect x="0" y="0" width="26" height="6.5" rx="0.5" fill={board} />
      <text
        x="13"
        y="4.8"
        text-anchor="middle"
        font-size="3.6"
        font-weight="700"
        font-family="ui-rounded, system-ui, sans-serif"
        letter-spacing="0.2"
        fill={signCream}>nap camp</text
      >
    </g>
  </g>

  <!-- LEAVE NO TRACE plaque (right edge) -->
  <g class="rules-plaque">
    <rect x="402" y="96" width="2.2" height="22" fill={bark} />
    <rect x="390" y="90" width="26" height="14" rx="0.6" fill="#4A5A3A" />
    <rect x="391.5" y="91.5" width="23" height="11" rx="0.4" fill="#5A6A48" opacity="0.55" />
    <text
      x="403"
      y="97"
      text-anchor="middle"
      font-size="3"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="-0.1"
      fill={signCream}>leave no</text
    >
    <text
      x="403"
      y="101.5"
      text-anchor="middle"
      font-size="3.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.2"
      fill={signCream}>scratch</text
    >
  </g>

  <!-- Forest path into ground strip -->
  <path
    d="M0 118 Q80 112 160 116 Q240 120 300 114 Q360 110 420 116 L420 140 L0 140 Z"
    fill={g.path}
    opacity="0.72"
  />
  <path
    d="M40 118 Q120 114 200 118 Q280 122 380 116"
    fill="none"
    stroke={g.detail}
    stroke-width="1.5"
    opacity="0.28"
  />

  <!-- Tiny plank bridge over path dip (low, corridor-safe) -->
  <g class="plank-bridge">
    <ellipse cx="182" cy="118" rx="18" ry="3" fill={barkDeep} opacity="0.35" />
    <path d="M164 116 Q182 120 200 116" fill="none" stroke={g.detail} stroke-width="2" opacity="0.25" />
    <g fill={board}>
      {#each bridgePlanks as p}
        <rect x={p.x} y="114" width="5" height="3.2" rx="0.3" opacity="0.9" />
      {/each}
    </g>
    <rect x="166" y="113" width="2" height="6" fill={bark} />
    <rect x="196" y="113" width="2" height="6" fill={bark} />
    <line x1="167" y1="113.5" x2="197" y2="113.5" stroke={boardLight} stroke-width="0.8" opacity="0.7" />
  </g>

  <!-- Moss / leaf-litter edges -->
  <g fill={litter} opacity={fall ? 0.75 : 0.45}>
    {#each litterPads as p}
      <ellipse cx={p.cx} cy={p.cy} rx={p.rx} ry={p.ry} />
    {/each}
  </g>
  <g fill={moss} opacity="0.4">
    {#each mossPatches as m}
      <ellipse cx={m.cx} cy={m.cy} rx={m.rx} ry={m.ry} />
    {/each}
  </g>

  <!-- Stump rings -->
  <g>
    {#each stumpRings as s}
      <ellipse cx={s.cx} cy={s.cy} rx={s.r} ry={s.r * 0.45} fill={bark} />
      <ellipse cx={s.cx} cy={s.cy - 0.6} rx={s.r * 0.85} ry={s.r * 0.35} fill={barkDeep} opacity="0.55" />
      <ellipse
        cx={s.cx}
        cy={s.cy - 0.8}
        rx={s.r * 0.45}
        ry={s.r * 0.18}
        fill="none"
        stroke={boardLight}
        stroke-width="0.6"
        opacity="0.55"
      />
      <ellipse
        cx={s.cx}
        cy={s.cy - 0.8}
        rx={s.r * 0.22}
        ry={s.r * 0.09}
        fill="none"
        stroke={boardLight}
        stroke-width="0.5"
        opacity="0.45"
      />
    {/each}
  </g>

  {#if fall}
    <g opacity="0.85">
      {#each fallLeaves as leaf}
        <ellipse
          cx={leaf.cx}
          cy={leaf.cy}
          rx="3.2"
          ry="1.6"
          fill={leaf.fill}
          transform={`rotate(${(leaf.cx * 7) % 50} ${leaf.cx} ${leaf.cy})`}
        />
      {/each}
    </g>
  {/if}

  <!-- Ground plane — top edge GROUND_Y = 118 -->
  <rect x="0" y="118" width="420" height="22" fill={g.near} />
  <rect x="0" y="118" width="420" height="3" fill={g.detail} opacity="0.22" />
  <rect x="0" y="124" width="420" height="16" fill={g.path} opacity="0.5" />

  {#if winter}
    <rect x="0" y="116" width="420" height="6" fill={g.accent} opacity="0.92" />
    <ellipse cx="60" cy="120" rx="22" ry="3" fill={g.accent} opacity="0.55" />
    <ellipse cx="360" cy="120" rx="26" ry="3" fill={g.accent} opacity="0.5" />
  {/if}

  <!-- Foreground undergrowth (edges only) -->
  <g fill={summer ? canopy.e : g.near}>
    {#each undergrowth as x}
      <ellipse cx={x} cy="116" rx={summer ? 16 : 12} ry={summer ? 7 : 5} />
    {/each}
  </g>

  <!-- Ferns — edges -->
  <g fill={moss} opacity="0.9">
    <path d="M8 118 Q14 104 10 96 Q18 106 16 118 Z" />
    <path d="M18 118 Q26 102 24 94 Q30 108 28 118 Z" />
    <path d="M388 118 Q396 102 394 94 Q400 108 398 118 Z" />
    <path d="M400 118 Q408 104 406 96 Q412 110 410 118 Z" />
  </g>

  {#if summer}
    <g fill={canopy.d} opacity="0.75">
      <path d="M48 118 Q54 108 52 102 Q58 112 56 118 Z" />
      <path d="M360 118 Q366 106 364 100 Q370 112 368 118 Z" />
    </g>
  {/if}

  <!-- Rocks — edges -->
  <g>
    <ellipse cx="36" cy="122" rx="10" ry="5" fill={g.detail} />
    <ellipse cx="34" cy="120" rx="7" ry="3.5" fill={g.accent} opacity="0.35" />
    <ellipse cx="48" cy="124" rx="6" ry="3.2" fill={barkDeep} />
  </g>
  <g>
    <ellipse cx="378" cy="122" rx="9" ry="4.5" fill={g.detail} />
    <ellipse cx="376" cy="120" rx="6" ry="3" fill={g.accent} opacity="0.3" />
    <ellipse cx="390" cy="124" rx="5.5" ry="3" fill={barkDeep} />
  </g>

  <!-- Fallen log (left foreground) -->
  <g>
    <ellipse cx="70" cy="124" rx="28" ry="7" fill={barkDeep} />
    <ellipse cx="70" cy="122" rx="28" ry="5.5" fill={bark} />
    <ellipse cx="46" cy="122" rx="4" ry="4.5" fill={barkDeep} />
    <path d="M55 119 Q70 117 88 120" fill="none" stroke={barkDeep} stroke-width="1.2" opacity="0.55" />
    <ellipse cx="62" cy="120" rx="6" ry="2" fill={moss} opacity="0.65" />
    <ellipse cx="82" cy="121" rx="5" ry="1.8" fill={moss} opacity="0.5" />
    {#if winter}
      <ellipse cx="70" cy="118" rx="22" ry="2.5" fill={g.accent} opacity="0.8" />
    {/if}
  </g>

  <!-- Mushrooms by log + right edge -->
  <g>
    <ellipse cx="52" cy="126" rx="3.5" ry="1.6" fill={barkDeep} opacity="0.45" />
    <rect x="50.5" y="120" width="2.2" height="6" fill="#E8D8C0" />
    <ellipse cx="51.6" cy="119" rx="4" ry="2.2" fill={fall ? '#A84030' : '#C06050'} />
    <rect x="58" y="122" width="1.6" height="4" fill="#E8D8C0" />
    <ellipse cx="58.8" cy="121.5" rx="2.6" ry="1.5" fill={fall ? '#8B4030' : '#B05048'} />
    <rect x="86" y="121" width="2" height="5" fill="#E8D8C0" />
    <ellipse cx="87" cy="120.5" rx="3.2" ry="1.8" fill="#C8A878" />

    <rect x="402" y="122" width="1.5" height="4" fill="#E8D8C0" />
    <ellipse cx="402.8" cy="121.5" rx="2.4" ry="1.4" fill="#B05048" />
    <rect x="408" y="123" width="1.3" height="3.5" fill="#E8D8C0" />
    <ellipse cx="408.7" cy="122.8" rx="2" ry="1.2" fill="#C8A878" />
  </g>

  {#if spring}
    <g fill={g.accent}>
      <circle cx="44" cy="108" r="2" />
      <circle cx="78" cy="104" r="1.6" />
      <circle cx="350" cy="106" r="2" />
      <circle cx="384" cy="102" r="1.7" />
    </g>
  {/if}
</g>
