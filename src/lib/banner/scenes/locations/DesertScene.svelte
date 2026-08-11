<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()

  const g = $derived(groundFor('desert', weather))
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  /** Pale frost cactus in winter — never deep green */
  const cactus = $derived(
    winter
      ? { body: '#9AA898', ridge: '#7A8878', arm: '#8A9888', bloom: '#D8D0C8' }
      : fall
        ? { body: '#6A7A48', ridge: '#4A5A32', arm: '#5A6A3A', bloom: '#C87840' }
        : spring
          ? { body: '#5A8A4A', ridge: '#3A6A32', arm: '#4A7A3A', bloom: '#E090B0' }
          : { body: '#4A7A3A', ridge: '#2E5A28', arm: '#3A6A30', bloom: '#E8A0B8' },
  )

  const rock = $derived(
    winter
      ? { far: '#B8A898', mid: '#A89888', near: '#8A7A6A', warm: '#C8B8A8' }
      : fall
        ? { far: '#C87848', mid: '#B86838', near: '#8A4828', warm: '#E09050' }
        : summer
          ? { far: '#C89860', mid: '#B88850', near: '#8A6838', warm: '#E0B070' }
          : { far: '#B89068', mid: '#A88058', near: '#7A5A38', warm: '#D4A878' },
  )

  const motel = $derived(
    winter
      ? { wall: '#A89888', trim: '#7A6A5A', roof: '#6A7080', neon: '#A8C8D8' }
      : fall
        ? { wall: '#8A6A58', trim: '#5A4038', roof: '#4A3A32', neon: '#E8A060' }
        : { wall: '#9A7A62', trim: '#6A4A38', roof: '#3A342C', neon: '#F0C070' },
  )

  const neonPink = $derived(winter ? '#C8A0B0' : '#E878A0')
  const neonCyan = $derived(winter ? '#88B0C0' : '#60C8D0')
  const signCream = '#F2E8D4'
  const signInk = '#1A1E24'
  const asphalt = $derived(winter ? '#6A7078' : '#3A3E44')

  const cracks = [
    { d: 'M12 120 Q22 122 28 119 M22 122 L24 126' },
    { d: 'M48 119 Q54 123 62 120 M54 123 L56 128' },
    { d: 'M350 120 Q360 124 372 121 M360 124 L362 129' },
    { d: 'M385 119 Q395 122 408 120' },
    { d: 'M18 128 Q30 130 42 127' },
    { d: 'M365 127 Q378 131 395 128' },
  ]
  const sandRipples = [
    { y: 121, o: 0.2 },
    { y: 125, o: 0.15 },
    { y: 130, o: 0.12 },
  ]
  const springBlooms = [
    { x: 28, y: 112 },
    { x: 52, y: 114 },
    { x: 368, y: 113 },
    { x: 392, y: 112 },
  ]
  const fallRocks = [
    { x: 40, y: 116, rx: 5, ry: 2.4 },
    { x: 58, y: 117, rx: 3.5, ry: 1.8 },
    { x: 370, y: 116, rx: 4.5, ry: 2.2 },
    { x: 388, y: 117, rx: 3.2, ry: 1.6 },
  ]
</script>

<g class="desert" aria-hidden="true">
  <!-- ===== FAR MESAS / BUTTES ===== -->
  <g fill={g.far} opacity="0.7">
    <path d="M-10 118 L20 78 L48 86 L70 62 L95 88 L120 72 L145 90 L160 118 Z" />
    <path d="M200 118 L230 74 L255 82 L280 58 L310 84 L340 68 L370 92 L400 76 L430 118 Z" />
  </g>
  <g fill={rock.far} opacity="0.55">
    <path d="M30 118 L50 90 L68 96 L85 78 L105 98 L120 118 Z" />
    <path d="M280 118 L300 86 L318 92 L340 70 L362 94 L385 82 L410 118 Z" />
  </g>

  <!-- Mesa strata lines -->
  <g stroke={rock.mid} stroke-width="0.9" opacity="0.35" fill="none">
    <path d="M35 100 Q55 98 75 102" />
    <path d="M45 108 Q65 106 90 110" />
    <path d="M295 96 Q320 94 350 98" />
    <path d="M310 106 Q340 104 375 108" />
  </g>

  <!-- Mid butte with flat top -->
  <g transform="translate(0,0)">
    <path
      d="M155 118 L168 88 L178 86 L195 86 L208 90 L218 118 Z"
      fill={g.mid}
      opacity="0.65"
    />
    <rect x="172" y="84" width="30" height="4" rx="0.5" fill={rock.warm} opacity="0.5" />
    <path d="M170 95 L210 95" stroke={rock.near} stroke-width="0.8" opacity="0.3" />
    <path d="M168 104 L212 104" stroke={rock.near} stroke-width="0.8" opacity="0.25" />
  </g>

  <!-- Distant butte right of corridor (background only) -->
  <g opacity="0.5">
    <path d="M240 118 L252 92 L268 88 L285 94 L295 118 Z" fill={g.far} />
    <rect x="255" y="86" width="22" height="3" fill={rock.warm} opacity="0.45" />
  </g>

  <!-- ===== LAST MEOW MOTEL (left edge) ===== -->
  <g class="motel" transform="translate(0,0)">
    <!-- building -->
    <rect x="4" y="78" width="58" height="40" fill={motel.wall} />
    <rect x="4" y="78" width="58" height="4" fill={motel.trim} />
    <path d="M2 78 L33 64 L64 78 Z" fill={motel.roof} />
    {#if winter}
      <path d="M2 78 L33 64 L64 78 Z" fill={g.accent} opacity="0.55" />
    {/if}
    <!-- rooms -->
    <rect x="10" y="88" width="12" height="14" fill="#2A2830" />
    <rect x="11" y="89" width="10" height="6" fill={winter ? '#C8D8E8' : neonCyan} opacity="0.45" />
    <rect x="26" y="88" width="12" height="14" fill="#2A2830" />
    <rect x="27" y="89" width="10" height="6" fill={winter ? '#B8C8D8' : '#E8D090'} opacity="0.4" />
    <rect x="42" y="88" width="12" height="14" fill="#2A2830" />
    <rect x="43" y="89" width="10" height="6" fill={winter ? '#C0D0E0' : neonPink} opacity="0.35" />
    <!-- door -->
    <rect x="28" y="104" width="10" height="14" fill={motel.trim} />
    <circle cx="36" cy="111" r="0.7" fill={signCream} opacity="0.7" />
    <!-- balcony rail -->
    <rect x="8" y="102" width="50" height="1.2" fill={motel.trim} opacity="0.8" />
    <g stroke={motel.trim} stroke-width="0.7" opacity="0.6">
      <line x1="14" y1="102" x2="14" y2="108" />
      <line x1="24" y1="102" x2="24" y2="108" />
      <line x1="40" y1="102" x2="40" y2="108" />
      <line x1="50" y1="102" x2="50" y2="108" />
    </g>

    <!-- rooftop sign: Last Meow Motel -->
    <rect x="8" y="52" width="50" height="14" rx="1" fill="#2A2430" />
    <rect x="9" y="53" width="48" height="12" rx="0.5" fill="#1A1820" />
    <text
      x="33"
      y="59"
      text-anchor="middle"
      font-size="4.2"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.4"
      fill={neonPink}>LAST MEOW</text
    >
    <text
      x="33"
      y="64.5"
      text-anchor="middle"
      font-size="3.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="1.2"
      fill={neonCyan}>MOTEL</text
    >
    <rect x="30" y="66" width="6" height="12" fill="#4A4850" />

    <!-- VACANCY neon wink (anim group 2: sk-glow) -->
    <g transform="translate(14, 70)">
      <rect x="0" y="0" width="28" height="7" rx="0.8" fill="#1A1820" />
      <g class="sk-glow">
        <text
          x="14"
          y="5.2"
          text-anchor="middle"
          font-size="4"
          font-weight="800"
          font-family="ui-rounded, system-ui, sans-serif"
          letter-spacing="0.6"
          fill={neonPink}>VACANCY</text
        >
      </g>
      <!-- wink eye -->
      <ellipse cx="26" cy="3.5" rx="1.4" ry="1.1" fill={neonCyan} opacity="0.85" />
      <path d="M24.8 3.2 Q26 4.4 27.2 3.2" fill="none" stroke="#1A1820" stroke-width="0.5" />
    </g>

    <!-- office awning -->
    <path d="M22 104 L33 98 L44 104" fill="#C45A6A" opacity="0.85" />
    <text
      x="33"
      y="103"
      text-anchor="middle"
      font-size="2.4"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}>office</text
    >
  </g>

  <!-- ===== GAS PUMP (near motel, left of corridor) ===== -->
  <g class="gas-pump" transform="translate(66, 0)">
    <rect x="0" y="96" width="14" height="22" rx="1" fill={winter ? '#6A7888' : '#4A5460'} />
    <rect x="1" y="98" width="12" height="8" rx="0.5" fill="#1A2830" />
    <rect x="2" y="99" width="10" height="5" fill={summer ? '#88E090' : '#70C878'} opacity="0.7" />
    <text
      x="7"
      y="103"
      text-anchor="middle"
      font-size="2.6"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signInk}>$$</text
    >
    <rect x="2" y="108" width="4" height="3" rx="0.3" fill="#C84848" />
    <rect x="8" y="108" width="4" height="3" rx="0.3" fill="#4880C8" />
    <path d="M14 100 Q20 102 18 110" fill="none" stroke="#2A3038" stroke-width="1.4" />
    <rect x="16" y="109" width="3.5" height="5" rx="0.6" fill="#2A3038" />
    <text
      x="7"
      y="94"
      text-anchor="middle"
      font-size="2.5"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}
      opacity="0.85">meowthane</text
    >
  </g>

  <!-- ===== ROUTE-STYLE SIGN (right edge) ===== -->
  <g class="route-sign" transform="translate(372, 0)">
    <rect x="10" y="72" width="3" height="46" fill="#6A6A68" />
    <path
      d="M2 58 L24 58 L28 72 L24 86 L2 86 L-2 72 Z"
      fill="#2A5A3A"
      stroke={signCream}
      stroke-width="1.2"
    />
    <text
      x="13"
      y="68"
      text-anchor="middle"
      font-size="3.2"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}>ROUTE</text
    >
    <text
      x="13"
      y="78"
      text-anchor="middle"
      font-size="8"
      font-weight="900"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}>9</text
    >
    <text
      x="13"
      y="84"
      text-anchor="middle"
      font-size="2.8"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      letter-spacing="0.3"
      fill={signCream}>LIVES</text
    >
  </g>

  <!-- roadside plaque -->
  <g transform="translate(348, 100)">
    <rect x="0" y="0" width="22" height="10" rx="0.5" fill="#3A4A38" />
    <text
      x="11"
      y="4.5"
      text-anchor="middle"
      font-size="2.4"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}>purr 66</text
    >
    <text
      x="11"
      y="8.5"
      text-anchor="middle"
      font-size="2.2"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#A8C8A0">next exit</text
    >
  </g>

  <!-- ===== SAGUARO CACTI (edges) ===== -->
  <!-- left saguaro -->
  <g class="saguaro" transform="translate(42, 0)">
    <rect x="0" y="78" width="7" height="40" rx="3" fill={cactus.body} />
    <g stroke={cactus.ridge} stroke-width="0.55" opacity="0.55">
      <line x1="1.5" y1="82" x2="1.5" y2="114" />
      <line x1="3.5" y1="80" x2="3.5" y2="116" />
      <line x1="5.5" y1="82" x2="5.5" y2="114" />
    </g>
    <!-- left arm -->
    <path d="M0 92 L-8 92 L-8 84 Q-8 80 -4 80 L0 80" fill={cactus.arm} />
    <path d="M-7.2 84 L-7.2 91" stroke={cactus.ridge} stroke-width="0.5" opacity="0.5" />
    <!-- right arm -->
    <path d="M7 98 L14 98 L14 88 Q14 84 10 84 L7 84" fill={cactus.arm} />
    <path d="M13.2 88 L13.2 97" stroke={cactus.ridge} stroke-width="0.5" opacity="0.5" />
    {#if spring}
      <circle cx="-6" cy="80" r="1.6" fill={cactus.bloom} />
      <circle cx="12" cy="84" r="1.4" fill={cactus.bloom} opacity="0.9" />
      <circle cx="3.5" cy="76" r="1.5" fill="#F0C060" />
    {/if}
    {#if winter}
      <ellipse cx="3.5" cy="78" rx="4" ry="1.4" fill={g.accent} opacity="0.5" />
      <ellipse cx="-6" cy="82" rx="2.5" ry="1" fill={g.accent} opacity="0.4" />
    {/if}
  </g>

  <!-- right tall saguaro -->
  <g class="saguaro" transform="translate(398, 0)">
    <rect x="0" y="70" width="8" height="48" rx="3.5" fill={cactus.body} />
    <g stroke={cactus.ridge} stroke-width="0.55" opacity="0.5">
      <line x1="2" y1="74" x2="2" y2="114" />
      <line x1="4" y1="72" x2="4" y2="116" />
      <line x1="6" y1="74" x2="6" y2="114" />
    </g>
    <path d="M0 88 L-10 88 L-10 78 Q-10 74 -5 74 L0 74" fill={cactus.arm} />
    <path d="M8 100 L16 100 L16 90 Q16 86 12 86 L8 86" fill={cactus.arm} />
    {#if spring}
      <circle cx="-8" cy="74" r="1.5" fill={cactus.bloom} />
      <circle cx="14" cy="86" r="1.3" fill="#F0B070" />
    {/if}
    {#if winter}
      <ellipse cx="4" cy="70" rx="4.5" ry="1.5" fill={g.accent} opacity="0.45" />
    {/if}
  </g>

  <!-- smaller barrel + prickly pear (left) -->
  <g transform="translate(22, 108)">
    <ellipse cx="0" cy="0" rx="6" ry="8" fill={cactus.body} opacity="0.9" />
    <g stroke={cactus.ridge} stroke-width="0.45" opacity="0.45" fill="none">
      <ellipse cx="0" cy="0" rx="4" ry="6" />
      <ellipse cx="0" cy="0" rx="2" ry="4" />
    </g>
    {#if spring}
      <circle cx="0" cy="-7" r="1.3" fill="#F0A848" />
    {/if}
  </g>
  <g transform="translate(355, 110)" fill={cactus.arm} opacity="0.85">
    <ellipse cx="0" cy="0" rx="4" ry="3" />
    <ellipse cx="5" cy="-2" rx="3.5" ry="2.8" />
    <ellipse cx="3" cy="3" rx="3" ry="2.4" />
    {#if spring}
      <circle cx="5" cy="-4" r="1.2" fill={cactus.bloom} />
    {/if}
  </g>

  <!-- ===== TUMBLEWEED (anim group 1: sk-walk) — right edge ===== -->
  <g class="sk-walk" transform="translate(360, 114)">
    <g fill={winter ? '#B0A890' : fall ? '#A87848' : '#8A7048'} opacity="0.85">
      <circle cx="0" cy="0" r="6" fill="none" stroke={winter ? '#A09880' : '#7A6040'} stroke-width="1.2" />
      <circle cx="0" cy="0" r="4" fill="none" stroke={winter ? '#988870' : '#6A5030'} stroke-width="0.9" />
      <path d="M-5 -2 Q0 -6 5 -1 M-4 3 Q0 6 4 2 M-6 0 L6 0 M0 -6 L0 6" stroke={winter ? '#908070' : '#6A5440'} stroke-width="0.7" fill="none" />
      <circle cx="-2" cy="-2" r="1.2" opacity="0.5" />
      <circle cx="2.5" cy="1.5" r="1" opacity="0.45" />
    </g>
  </g>

  <!-- ===== DRY BRUSH / YUCCA (edges) ===== -->
  <g stroke={winter ? '#A09888' : '#8A7A50'} stroke-width="1" stroke-linecap="round" fill="none" opacity="0.7">
    <path d="M14 118 L12 108 M14 118 L16 106 M14 118 L10 110 M14 118 L18 109" />
    <path d="M408 118 L406 107 M408 118 L410 105 M408 118 L404 110" />
  </g>

  <!-- ===== SEASONAL ===== -->
  {#if fall}
    <g class="warm-rocks" fill={rock.warm}>
      {#each fallRocks as r}
        <ellipse cx={r.x} cy={r.y} rx={r.rx} ry={r.ry} opacity="0.85" />
      {/each}
    </g>
    <g fill={rock.near} opacity="0.55">
      <ellipse cx="72" cy="116" rx="4" ry="2" />
      <ellipse cx="345" cy="117" rx="3.5" ry="1.6" />
    </g>
  {:else if winter}
    <!-- frost dusting — pale, not green -->
    <g fill={g.accent} opacity="0.45">
      <ellipse cx="33" cy="116" rx="14" ry="2.2" />
      <ellipse cx="70" cy="117" rx="10" ry="1.8" />
      <ellipse cx="380" cy="116" rx="16" ry="2.4" />
      <ellipse cx="175" cy="86" rx="12" ry="1.6" opacity="0.35" />
    </g>
    <path
      d="M20 90 Q50 82 75 92"
      fill="none"
      stroke={g.accent}
      stroke-width="2"
      opacity="0.35"
      stroke-linecap="round"
    />
  {:else if spring}
    <g class="sparse-blooms">
      {#each springBlooms as b}
        <g>
          <line x1={b.x} y1={b.y} x2={b.x} y2={b.y - 4} stroke="#6A8A48" stroke-width="0.7" />
          <circle cx={b.x} cy={b.y - 5} r="1.5" fill={g.accent} />
          <circle cx={b.x - 1.2} cy={b.y - 4.2} r="1" fill={cactus.bloom} opacity="0.85" />
          <circle cx={b.x + 1.2} cy={b.y - 4.2} r="1" fill={cactus.bloom} opacity="0.85" />
        </g>
      {/each}
    </g>
    <g fill="#7A9A58" opacity="0.4">
      <ellipse cx="36" cy="117" rx="8" ry="1.5" />
      <ellipse cx="390" cy="117" rx="7" ry="1.4" />
    </g>
  {:else if summer}
    <!-- heat haze (anim group 3: sk-mist) -->
    <g class="sk-mist" opacity="0.18">
      <ellipse cx="100" cy="100" rx="40" ry="5" fill="#fff" />
      <ellipse cx="200" cy="104" rx="50" ry="4.5" fill="#fff" />
      <ellipse cx="300" cy="98" rx="36" ry="5" fill="#fff" />
      <ellipse cx="160" cy="90" rx="28" ry="3.5" fill="#FFE8A0" opacity="0.5" />
    </g>
  {/if}

  <!-- ===== GROUND PLANE — top y=118; clear corridor ~80–340 ===== -->
  <rect x="0" y="118" width="420" height="22" fill={g.near} />
  <rect x="0" y="118" width="420" height="3" fill={g.detail} opacity="0.3" />
  <rect x="0" y="124" width="420" height="16" fill={g.path} opacity="0.5" />

  <!-- sand ripples -->
  <g fill="none" stroke={g.detail} stroke-width="0.8">
    {#each sandRipples as r}
      <path
        d="M0 {r.y} Q40 {r.y - 1.5} 80 {r.y} Q120 {r.y + 1.5} 160 {r.y} Q200 {r.y - 1} 240 {r.y} Q280 {r.y + 1.2} 340 {r.y} Q380 {r.y - 1} 420 {r.y}"
        opacity={r.o}
      />
    {/each}
  </g>

  <!-- cracked earth (edges + light mid texture, not blocking) -->
  <g fill="none" stroke={g.detail} stroke-width="0.85" opacity="0.4" stroke-linecap="round">
    {#each cracks as c}
      <path d={c.d} />
    {/each}
  </g>
  <g fill="none" stroke={rock.near} stroke-width="0.7" opacity="0.28">
    <path d="M90 122 Q110 124 130 122 M110 124 L112 128" />
    <path d="M200 121 Q220 125 245 122" />
    <path d="M280 123 Q300 126 320 123 M300 126 L301 130" />
  </g>

  <!-- asphalt apron near motel / pump -->
  <path
    d="M0 118 L78 118 L78 128 L0 132 Z"
    fill={asphalt}
    opacity="0.35"
  />
  <path
    d="M340 118 L420 118 L420 132 L340 128 Z"
    fill={asphalt}
    opacity="0.25"
  />

  <!-- soft dunes nibbling corridor edges only -->
  <g fill={g.mid} opacity="0.4">
    <ellipse cx="20" cy="119" rx="18" ry="2.2" />
    <ellipse cx="60" cy="119" rx="12" ry="1.8" />
    <ellipse cx="360" cy="119" rx="14" ry="2" />
    <ellipse cx="400" cy="119" rx="16" ry="2.2" />
  </g>

  {#if winter}
    <rect x="0" y="116" width="420" height="4" fill={g.accent} opacity="0.55" />
    <g fill={g.accent} opacity="0.35">
      <ellipse cx="50" cy="120" rx="20" ry="2.5" />
      <ellipse cx="380" cy="120" rx="22" ry="2.5" />
    </g>
  {/if}
</g>
