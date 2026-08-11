<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()

  const g = $derived(groundFor('mountain', weather))
  const winter = $derived(weather === 'winter')
  const fall = $derived(weather === 'fall')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  /** Stone / sage / pine accents — alpine, never neon */
  const rock = $derived({
    cool: fall ? '#6A7380' : summer ? '#7A8796' : winter ? '#9AA8B6' : '#748190',
    facet: fall ? '#5A6270' : summer ? '#687482' : winter ? '#8A98A6' : '#646E7A',
    scree: fall ? '#7A7068' : summer ? '#8A8490' : winter ? '#B0BCC8' : '#7A828C',
    sage: fall ? '#7A8A62' : summer ? '#6E8A58' : winter ? '#8A9A88' : '#6A8A5C',
    pine: fall ? '#3A5240' : summer ? '#2E4A34' : winter ? '#4A5E52' : '#34523A',
    pineMid: fall ? '#4A6848' : summer ? '#3E6840' : winter ? '#5A7060' : '#456848',
    snow: winter ? '#F6FAFE' : '#EEF4F8',
    snowSoft: winter ? '#E4ECF4' : '#D8E4EC',
    dirt: fall ? '#8A6E48' : summer ? '#9A8460' : winter ? '#C8D4DE' : '#8A7A58',
    stone: '#9A9E96',
    gold: '#D4A84A',
    goldDeep: '#C08A32',
    shoot: '#7EC86A',
    flower: ['#E87890', '#F0C858', '#C070B0', '#F4F0E8'] as const,
  })

  const signCream = '#F2E8D4'
  const signInk = '#1A1E24'
  const wood = '#6A5440'

  /** Static stands — mid corridor kept clear (~x 140–320) */
  const pinesStatic = [
    { cx: 42, baseY: 106, s: 0.55, layer: 'far' as const },
    { cx: 88, baseY: 104, s: 0.5, layer: 'far' as const },
    { cx: 132, baseY: 100, s: 0.65, layer: 'mid' as const },
    { cx: 248, baseY: 94, s: 0.6, layer: 'mid' as const },
    { cx: 318, baseY: 98, s: 0.55, layer: 'mid' as const },
    { cx: 348, baseY: 110, s: 0.7, layer: 'near' as const },
    { cx: 388, baseY: 112, s: 0.75, layer: 'near' as const },
    { cx: 372, baseY: 102, s: 0.5, layer: 'far' as const },
  ]

  /** Near sway cluster — left edge only (one animated group) */
  const pinesNear = [
    { cx: 28, baseY: 108, s: 0.55 },
    { cx: 58, baseY: 112, s: 0.75 },
    { cx: 92, baseY: 110, s: 0.65 },
    { cx: 118, baseY: 108, s: 0.55 },
  ]

  const shoots = [
    { x: 70, y: 116 },
    { x: 160, y: 115 },
    { x: 280, y: 116 },
    { x: 360, y: 115 },
  ]

  const flowers = [
    { x: 64, y: 112, i: 0 },
    { x: 118, y: 111, i: 2 },
    { x: 176, y: 110, i: 1 },
    { x: 268, y: 113, i: 3 },
    { x: 324, y: 112, i: 0 },
    { x: 378, y: 113, i: 2 },
  ]

  const groundShoots = [40, 100, 170, 300, 370]

  const blazePosts = [
    { x: 178, y: 98 },
    { x: 206, y: 108 },
    { x: 252, y: 112 },
    { x: 332, y: 108 },
  ]

  const ropePosts = [
    { x: 198, y: 108 },
    { x: 214, y: 112 },
    { x: 230, y: 116 },
    { x: 246, y: 118 },
  ]
</script>

{#snippet pine(cx: number, baseY: number, s: number, layer: 'far' | 'mid' | 'near')}
  {@const h = 18 * s}
  {@const w = 10 * s}
  {@const trunkH = 5 * s}
  {@const fill = layer === 'far' ? rock.pine : layer === 'mid' ? rock.pineMid : rock.pine}
  <g transform="translate({cx} {baseY})">
    <rect
      x={-1.2 * s}
      y={-trunkH}
      width={2.4 * s}
      height={trunkH + 1}
      fill={g.detail}
      opacity="0.85"
    />
    <path
      d="M0 {-h} L{w * 0.55} {-h * 0.45} L{-w * 0.55} {-h * 0.45} Z"
      {fill}
      opacity={layer === 'far' ? 0.7 : 0.92}
    />
    <path
      d="M0 {-h * 0.72} L{w * 0.7} {-h * 0.18} L{-w * 0.7} {-h * 0.18} Z"
      {fill}
      opacity={layer === 'far' ? 0.65 : 0.88}
    />
    <path
      d="M0 {-h * 0.42} L{w * 0.85} {0} L{-w * 0.85} {0} Z"
      {fill}
      opacity={layer === 'far' ? 0.6 : 0.85}
    />
    {#if winter}
      <path
        d="M0 {-h} L{w * 0.35} {-h * 0.55} L{-w * 0.35} {-h * 0.55} Z"
        fill={rock.snow}
        opacity="0.75"
      />
    {/if}
    {#if fall && layer !== 'far'}
      <circle cx={w * 0.15} cy={-h * 0.55} r={1.1 * s} fill={rock.gold} opacity="0.7" />
    {/if}
  </g>
{/snippet}

<g class="mountain" aria-hidden="true">
  <!-- Atmospheric haze — animated group 1 -->
  <rect x="0" y="48" width="420" height="70" fill={g.far} opacity="0.18" />
  <g class="sk-mist">
    <ellipse cx="90" cy="78" rx="70" ry="10" fill={g.accent} opacity="0.12" />
    <ellipse cx="280" cy="72" rx="90" ry="12" fill={g.accent} opacity="0.1" />
  </g>

  <!-- FAR ridge -->
  <path
    d="M0 118 L28 78 L58 92 L88 52 L118 82 L148 38 L178 74 L208 48 L238 70 L268 32 L298 66 L328 44 L358 72 L388 50 L420 64 L420 140 L0 140 Z"
    fill={g.far}
  />
  <path
    d="M28 78 L58 92 L88 52 L118 82 L148 38 L178 74 L208 48 L238 70 L268 32 L298 66 L328 44 L358 72 L388 50 L420 64"
    fill="none"
    stroke={rock.cool}
    stroke-width="1.2"
    opacity="0.45"
  />
  <path d="M148 38 L162 58 L138 62 Z" fill={rock.facet} opacity="0.35" />
  <path d="M268 32 L286 54 L254 56 Z" fill={rock.facet} opacity="0.4" />

  <!-- MID ridge -->
  <path
    d="M0 122 L40 88 L72 104 L110 62 L150 96 L190 54 L230 90 L270 48 L310 84 L350 58 L390 88 L420 72 L420 140 L0 140 Z"
    fill={g.mid}
  />
  <path
    d="M40 88 L72 104 L110 62 L150 96 L190 54 L230 90 L270 48 L310 84 L350 58 L390 88 L420 72"
    fill="none"
    stroke={rock.cool}
    stroke-width="1.4"
    opacity="0.5"
  />
  <path d="M110 62 L128 84 L98 88 Z" fill={rock.facet} opacity="0.45" />
  <path d="M190 54 L210 78 L176 82 Z" fill={rock.facet} opacity="0.5" />
  <path d="M270 48 L292 74 L252 76 Z" fill={rock.facet} opacity="0.48" />
  <g opacity={summer ? 0.55 : 0.28} stroke={rock.cool} stroke-width="0.8" fill="none">
    <path d="M100 78 L134 76" />
    <path d="M176 72 L214 70" />
    <path d="M250 68 L290 66" />
  </g>
  <g fill={rock.scree} opacity="0.35">
    <path d="M118 88 L132 108 L108 108 Z" />
    <path d="M198 80 L216 108 L186 108 Z" />
    <path d="M278 76 L298 106 L264 106 Z" />
  </g>

  <!-- Tiny lookout cabin / alpine hut -->
  <g transform="translate(302 52)">
    <rect x="0" y="4" width="14" height="10" fill={g.detail} />
    <path d="M-1 4 L7 -2 L15 4 Z" fill={rock.cool} />
    <rect x="5" y="8" width="4" height="6" fill={g.accent} opacity="0.7" />
    <!-- ALPINE HUT sign above door -->
    <rect x="1.5" y="5.2" width="11" height="2.8" rx="0.2" fill="#2A2824" />
    <text
      x="7"
      y="7.4"
      text-anchor="middle"
      font-size="2.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="-0.1"
      fill={signCream}>sound hut</text
    >
    <!-- chimney -->
    <rect x="11" y="0" width="2.4" height="5" fill={rock.facet} />
    <!-- static smoke puffs -->
    <ellipse cx="12.2" cy="-2" rx="2.4" ry="1.4" fill={g.accent} opacity="0.35" />
    <ellipse cx="13.6" cy="-4.2" rx="1.8" ry="1.1" fill={g.accent} opacity="0.22" />
    {#if winter}
      <path d="M-1 4 L7 -2 L15 4 L14 5.5 L7 0 L0 5.5 Z" fill={rock.snow} opacity="0.75" />
    {/if}
  </g>

  <!-- Static flag / wind sock on ridge (no new animation) -->
  <g transform="translate(268 28)">
    <line x1="0" y1="0" x2="0" y2="14" stroke={rock.cool} stroke-width="1.1" />
    <path d="M0 1 L10 3.5 L0 6 Z" fill="#C45A2A" opacity="0.9" />
    <ellipse cx="10" cy="3.5" rx="1.6" ry="2.2" fill="#D4783A" opacity="0.75" />
  </g>

  <!-- NEAR slope -->
  <path
    d="M0 128 L55 98 L95 116 L145 82 L195 110 L255 74 L310 104 L365 86 L420 102 L420 140 L0 140 Z"
    fill={g.near}
    opacity="0.92"
  />
  <path
    d="M55 98 L95 116 L145 82 L195 110 L255 74 L310 104 L365 86 L420 102"
    fill="none"
    stroke={rock.facet}
    stroke-width="1.6"
    opacity="0.4"
  />
  <path d="M145 82 L162 100 L132 104 Z" fill={rock.facet} opacity="0.42" />
  <path d="M255 74 L276 98 L240 100 Z" fill={rock.facet} opacity="0.45" />
  <g fill={rock.scree} opacity="0.4">
    <path d="M150 102 L166 118 L140 118 Z" />
    <path d="M260 98 L280 118 L248 118 Z" />
  </g>

  <!-- Snow caps -->
  {#if winter}
    <g fill={rock.snow}>
      <path d="M148 38 L172 58 L130 58 Z" />
      <path d="M268 32 L298 56 L246 56 Z" />
      <path d="M88 52 L106 68 L74 68 Z" />
      <path d="M190 54 L218 76 L170 76 Z" />
      <path d="M270 48 L300 74 L248 74 Z" />
      <path d="M255 74 L282 96 L236 96 Z" />
      <ellipse cx="160" cy="70" rx="22" ry="5" fill={rock.snowSoft} opacity="0.85" />
      <ellipse cx="280" cy="66" rx="28" ry="6" fill={rock.snowSoft} opacity="0.8" />
    </g>
  {:else if spring || summer}
    <g fill={rock.snow} opacity={summer ? 0.55 : 0.75}>
      <path d="M148 38 L164 52 L138 52 Z" />
      <path d="M268 32 L286 50 L256 50 Z" />
      <path d="M190 54 L204 66 L180 66 Z" />
      <path d="M270 48 L286 62 L258 62 Z" />
      {#if spring}
        <ellipse cx="200" cy="62" rx="6" ry="2" fill={rock.scree} opacity="0.5" />
        <ellipse cx="278" cy="58" rx="5" ry="1.8" fill={rock.scree} opacity="0.45" />
      {/if}
    </g>
  {:else}
    <g fill={rock.snow} opacity="0.7">
      <path d="M148 38 L160 50 L140 50 Z" />
      <path d="M268 32 L280 46 L260 46 Z" />
      <path d="M190 54 L200 64 L184 64 Z" />
    </g>
  {/if}

  <!-- Static evergreen stands -->
  {#each pinesStatic as p}
    {@render pine(p.cx, p.baseY, p.s, p.layer)}
  {/each}

  <!-- Near pines — animated group 2 -->
  <g class="sk-sway">
    {#each pinesNear as p}
      {@render pine(p.cx, p.baseY, p.s, 'near')}
    {/each}
  </g>

  {#if fall}
    <g opacity="0.75">
      <ellipse cx="120" cy="96" rx="14" ry="5" fill={rock.gold} opacity="0.55" />
      <ellipse cx="200" cy="92" rx="18" ry="6" fill={rock.goldDeep} opacity="0.5" />
      <ellipse cx="360" cy="98" rx="12" ry="4" fill={rock.goldDeep} opacity="0.5" />
    </g>
  {/if}

  {#if spring}
    <path
      d="M255 78 Q248 92 242 104 Q236 112 228 118"
      fill="none"
      stroke="#8EC8D8"
      stroke-width="2.2"
      stroke-linecap="round"
      opacity="0.7"
    />
    <path
      d="M255 78 Q248 92 242 104 Q236 112 228 118"
      fill="none"
      stroke="#C8E8F0"
      stroke-width="0.9"
      stroke-linecap="round"
      opacity="0.85"
    />
    <g fill={rock.shoot} opacity="0.85">
      {#each shoots as s}
        <path d="M{s.x} {s.y} L{s.x - 1.2} {s.y - 5} L{s.x + 0.4} {s.y - 3.5} Z" />
        <path d="M{s.x + 3} {s.y} L{s.x + 2} {s.y - 4} L{s.x + 4} {s.y - 3} Z" />
      {/each}
    </g>
  {/if}

  {#if summer}
    <g>
      {#each flowers as f}
        <circle cx={f.x} cy={f.y} r="1.35" fill={rock.flower[f.i]} opacity="0.9" />
      {/each}
    </g>
  {/if}

  <!-- Sage scrub — edges / lower slopes -->
  <g fill={rock.sage} opacity={winter ? 0.25 : 0.45}>
    <ellipse cx="40" cy="114" rx="22" ry="5" />
    <ellipse cx="370" cy="115" rx="24" ry="5" />
  </g>

  <!-- Trailhead kiosk (left of trail start, short) -->
  <g class="trailhead" transform="translate(148 88)">
    <rect x="6" y="8" width="2.4" height="22" fill={wood} />
    <rect x="18" y="10" width="2.4" height="20" fill={wood} />
    <rect x="4" y="4" width="20" height="14" rx="0.6" fill="#3A4238" />
    <rect x="5" y="5" width="18" height="12" rx="0.4" fill="#4A5448" opacity="0.55" />
    <text
      x="14"
      y="11"
      text-anchor="middle"
      font-size="3.6"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="-0.15"
      fill={signCream}>bass ridge</text
    >
    <text
      x="14"
      y="15.5"
      text-anchor="middle"
      font-size="2.6"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.15"
      fill="#C8D0C0">bpm 120</text
    >
    <path d="M4 4 L14 0.5 L24 4" fill="none" stroke={rock.cool} stroke-width="1" opacity="0.7" />
  </g>

  <!-- Winding trail into ground strip -->
  <path
    d="M168 86 Q182 98 194 106 Q210 114 228 118 Q260 122 300 118 Q340 114 380 118 Q400 120 420 116"
    fill="none"
    stroke={g.path}
    stroke-width="11"
    stroke-linecap="round"
    opacity="0.35"
  />
  <path
    d="M168 86 Q182 98 194 106 Q210 114 228 118 Q260 122 300 118 Q340 114 380 118 Q400 120 420 116"
    fill="none"
    stroke={rock.dirt}
    stroke-width="6.5"
    stroke-linecap="round"
    opacity="0.75"
  />
  <g fill={rock.stone} opacity="0.55">
    <ellipse cx="218" cy="114" rx="1.8" ry="0.9" />
    <ellipse cx="286" cy="119" rx="2" ry="0.85" />
    <ellipse cx="368" cy="117" rx="1.6" ry="0.8" />
  </g>

  <!-- Painted trail blaze posts -->
  <g class="blaze-posts">
    {#each blazePosts as p}
      <rect x={p.x} y={p.y} width="2" height="12" fill={wood} rx="0.3" />
      <rect x={p.x - 0.6} y={p.y + 1} width="3.2" height="4" rx="0.3" fill="#C45A2A" opacity="0.9" />
      <rect x={p.x - 0.2} y={p.y + 1.6} width="2.4" height="2.6" rx="0.2" fill="#E07040" opacity="0.55" />
    {/each}
  </g>

  <!-- Rope / guide rail along short trail section -->
  <g class="guide-rail">
    {#each ropePosts as p}
      <rect x={p.x} y={p.y} width="1.6" height="9" fill={rock.cool} rx="0.3" />
    {/each}
    <path
      d="M198.8 110 Q214 113 230.8 117 Q238 118.5 246.8 120"
      fill="none"
      stroke="#8A7A68"
      stroke-width="1.1"
      opacity="0.75"
    />
    <path
      d="M198.8 113 Q214 116 230.8 119.5"
      fill="none"
      stroke="#8A7A68"
      stroke-width="0.8"
      opacity="0.45"
    />
  </g>

  <!-- Cairn / trail marker — low, corridor-safe -->
  <g transform="translate(224 108)">
    <ellipse cx="0" cy="8" rx="6" ry="2" fill={g.detail} opacity="0.35" />
    <ellipse cx="-1.5" cy="6" rx="3.2" ry="2" fill={rock.cool} />
    <ellipse cx="1.5" cy="5" rx="2.8" ry="1.8" fill={rock.facet} />
    <ellipse cx="0" cy="2.5" rx="2.4" ry="1.6" fill={rock.scree} />
    <ellipse cx="0.5" cy="0.2" rx="1.8" ry="1.3" fill={rock.cool} />
    <rect x="8" y="-6" width="2.2" height="14" fill={g.detail} rx="0.4" />
    <path d="M7 -5 L14 -8 L14 -5.5 L7 -2.5 Z" fill="#C45A2A" opacity="0.85" />
  </g>

  <!-- Warning: falling rock (right trail edge, short) -->
  <g class="rock-warning" transform="translate(358 98)">
    <rect x="5" y="8" width="2" height="12" fill={wood} />
    <path d="M6 1 L14 14 L-2 14 Z" fill="#D4A84A" stroke="#3A3428" stroke-width="0.7" />
    <text
      x="6"
      y="8"
      text-anchor="middle"
      font-size="2.6"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signInk}>soft</text
    >
    <text
      x="6"
      y="12.5"
      text-anchor="middle"
      font-size="2.4"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="-0.1"
      fill={signInk}>paws</text
    >
  </g>

  {#if winter}
    <g fill={rock.snow} opacity="0.9">
      <ellipse cx="210" cy="116" rx="16" ry="3.5" />
      <ellipse cx="290" cy="117" rx="18" ry="3.5" />
      <ellipse cx="340" cy="116" rx="16" ry="3" />
    </g>
  {/if}

  <!-- Ground plane — top edge GROUND_Y (118) -->
  <rect x="0" y="118" width="420" height="22" fill={g.near} />
  <rect x="0" y="118" width="420" height="3" fill={g.detail} opacity="0.28" />
  <rect x="0" y="124" width="420" height="16" fill={g.path} opacity="0.55" />
  <path
    d="M210 118 Q250 122 300 118 Q360 114 420 118 L420 128 Q340 132 260 128 Q220 126 210 118 Z"
    fill={rock.dirt}
    opacity="0.5"
  />

  {#if winter}
    <rect x="0" y="116" width="420" height="6" fill={rock.snow} opacity="0.92" />
    <ellipse cx="80" cy="119" rx="40" ry="3.5" fill={rock.snowSoft} opacity="0.7" />
    <ellipse cx="360" cy="119" rx="45" ry="3.5" fill={rock.snowSoft} opacity="0.7" />
  {/if}

  {#if spring}
    <g fill={rock.shoot} opacity="0.7">
      {#each groundShoots as gx}
        <path d="M{gx} 120 L{gx - 1} 116 L{gx + 1.5} 117.5 Z" />
      {/each}
    </g>
  {/if}
</g>
