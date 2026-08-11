<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Time, Weather } from '../../types'

  let { weather, time = 'day' }: { weather: Weather; time?: Time } = $props()

  const g = $derived(groundFor('london', weather))
  const night = $derived(time === 'night')
  const golden = $derived(time === 'sunrise' || time === 'sunset')
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  const brick = $derived(
    night
      ? summer
        ? '#6A3A30'
        : fall
          ? '#5A3228'
          : winter
            ? '#4A4038'
            : '#5A3830'
      : summer
        ? '#9A5A48'
        : fall
          ? '#8A4E3C'
          : winter
            ? '#7A685C'
            : '#8A5848',
  )
  const brickDark = $derived(
    night ? (winter ? '#2A2420' : '#2A1814') : winter ? '#5A5048' : '#5A3830',
  )
  const brickLite = $derived(
    night ? (winter ? '#6A5A50' : '#7A4838') : winter ? '#9A8A7C' : '#A86858',
  )
  const mortar = $derived(winter ? '#B0A498' : '#C8A898')
  const sash = $derived(winter ? '#E8EEF4' : '#F0E8D8')
  const glass = $derived(
    night
      ? '#1A2430'
      : golden
        ? '#A87850'
        : winter
          ? '#C8D8EC'
          : fall
            ? '#A8B0B8'
            : spring
              ? '#7AA8B8'
              : '#5A8898',
  )
  const glassLit = $derived(
    night
      ? '#FFE8A8'
      : golden
        ? '#FFD090'
        : winter
          ? '#E0ECF8'
          : fall
            ? '#D0C8A8'
            : '#9EC8D0',
  )
  const paneOp = $derived(night ? 0.95 : golden ? 0.8 : 0.7)
  const paneLitOp = $derived(night ? 0.98 : golden ? 0.85 : 0.75)
  const slate = $derived(winter ? '#6A7888' : '#3A4450')
  const slateDark = $derived(winter ? '#4A5868' : '#2A323C')
  const cream = '#F2E8D4'
  const lamp = $derived(night ? '#FFF4C8' : golden ? '#F0D080' : winter ? '#F0E8C8' : '#E8D090')
  const lampWash = $derived(night ? '#FFE8A8' : golden ? '#FFD090' : g.accent)
  const postRed = '#B02828'
  const boothRed = '#C03028'
  const blossom = ['#F0A0C8', '#E888B0', '#D870A0'] as const
  const foliageWarm = ['#C45A2A', '#D4783A', '#E07030', '#A84828'] as const
  const pubBrick = $derived(night ? '#4A2820' : '#6A4038')
  const pubMortar = $derived(night ? '#6A4038' : '#8A6050')
  const vehicleGlass = $derived(night ? '#FFE8A8' : golden ? '#FFD090' : glass)
</script>

<g class="london" aria-hidden="true">
  <!-- animated group 1: distant mist -->
  <rect class="sk-mist" x="0" y="50" width="420" height="68" fill={g.far} opacity="0.18" />

  <!-- ===== FAR SILHOUETTE — dense London roofscape ===== -->
  <g fill={g.far} opacity="0.92">
    <rect x="0" y="68" width="16" height="50" />
    <rect x="12" y="54" width="22" height="64" />
    <rect x="30" y="62" width="18" height="56" />
    <rect x="44" y="48" width="20" height="70" />
    <rect x="60" y="58" width="14" height="60" />
    <rect x="70" y="44" width="24" height="74" />
    <rect x="90" y="56" width="16" height="62" />
    <rect x="102" y="40" width="28" height="78" />
    <rect x="126" y="52" width="18" height="66" />
    <rect x="140" y="46" width="22" height="72" />
    <rect x="158" y="58" width="16" height="60" />
    <rect x="170" y="42" width="26" height="76" />
    <rect x="192" y="54" width="18" height="64" />
    <rect x="206" y="38" width="30" height="80" />
    <rect x="232" y="50" width="20" height="68" />
    <rect x="248" y="44" width="24" height="74" />
    <rect x="268" y="56" width="16" height="62" />
    <rect x="280" y="40" width="28" height="78" />
    <rect x="304" y="52" width="18" height="66" />
    <rect x="318" y="46" width="22" height="72" />
    <rect x="336" y="58" width="16" height="60" />
    <rect x="348" y="42" width="26" height="76" />
    <rect x="370" y="54" width="20" height="64" />
    <rect x="386" y="48" width="22" height="70" />
    <rect x="404" y="60" width="16" height="58" />
  </g>
  <!-- Chimney pots on far roofs -->
  <g fill={g.detail} opacity="0.75">
    <rect x="52" y="42" width="4" height="8" />
    <rect x="116" y="34" width="4" height="8" />
    <rect x="180" y="36" width="4" height="8" />
    <rect x="218" y="32" width="5" height="9" />
    <rect x="292" y="34" width="4" height="8" />
    <rect x="358" y="36" width="4" height="8" />
  </g>

  <!-- Distant double-decker (far-mid, left of corridor) -->
  <g transform="translate(48, 86)" opacity="0.55" fill={g.detail}>
    <rect x="0" y="8" width="34" height="14" rx="1.5" />
    <rect x="2" y="0" width="30" height="10" rx="1" />
    <rect x="4" y="2" width="6" height="5" fill={vehicleGlass} opacity={night ? 0.85 : 0.5} />
    <rect x="12" y="2" width="6" height="5" fill={vehicleGlass} opacity={night ? 0.7 : 0.45} />
    <rect x="20" y="2" width="6" height="5" fill={vehicleGlass} opacity={night ? 0.8 : 0.5} />
    <rect x="4" y="11" width="6" height="5" fill={vehicleGlass} opacity={night ? 0.75 : 0.4} />
    <rect x="12" y="11" width="6" height="5" fill={night ? '#1A2430' : glass} opacity={night ? 0.9 : 0.35} />
    <rect x="20" y="11" width="6" height="5" fill={vehicleGlass} opacity={night ? 0.7 : 0.4} />
    <circle cx="6" cy="22" r="2.2" />
    <circle cx="28" cy="22" r="2.2" />
    <rect x="30" y="10" width="5" height="8" rx="0.5" />
  </g>

  <!-- Distant black cab (far-mid, right of corridor) -->
  <g transform="translate(338, 98)" opacity="0.5" fill={g.detail}>
    <path
      d="M2 12 L4 6 L10 4 L22 4 L28 7 L32 12 L32 16 L2 16 Z"
    />
    <rect x="8" y="5" width="8" height="5" fill={vehicleGlass} opacity={night ? 0.8 : 0.4} />
    <rect x="18" y="5.5" width="6" height="4.5" fill={vehicleGlass} opacity={night ? 0.7 : 0.35} />
    <circle cx="8" cy="16" r="2.4" />
    <circle cx="26" cy="16" r="2.4" />
    <rect x="0" y="11" width="3" height="4" rx="0.4" />
  </g>

  <!-- ===== BRICK TERRACE ROW ===== -->

  <!-- House 1 (left edge) -->
  <g>
    <rect x="0" y="62" width="42" height="56" fill={brick} />
    <!-- brick courses -->
    <g stroke={mortar} stroke-width="0.35" opacity="0.35">
      {#each [68, 74, 80, 86, 92, 98, 104, 110] as y}
        <line x1="0" {y} x2="42" y2={y} />
      {/each}
    </g>
    <g stroke={mortar} stroke-width="0.3" opacity="0.22">
      {#each [6, 14, 22, 30, 38] as x}
        <line {x} y1="62" x2={x} y2="118" />
      {/each}
    </g>
    <!-- parapet / cornice -->
    <rect x="-1" y="60" width="44" height="3" fill={brickDark} />
    <rect x="-1" y="58" width="44" height="2.5" fill={slate} />
    <!-- chimney stack + pots -->
    <rect x="8" y="48" width="10" height="12" fill={brickDark} />
    <rect x="7" y="46" width="5" height="4" rx="0.5" fill={brickLite} />
    <rect x="13" y="45" width="5" height="5" rx="0.5" fill={brickLite} />
    <ellipse cx="9.5" cy="46" rx="2.2" ry="0.8" fill={slateDark} />
    <ellipse cx="15.5" cy="45" rx="2.2" ry="0.8" fill={slateDark} />
    <!-- sash windows -->
    <g>
      <rect x="6" y="68" width="13" height="16" fill={brickDark} />
      <rect x="7" y="69" width="5.2" height="6.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="12.8" y="69" width="5.2" height="6.5" fill={glass} opacity={paneOp} />
      <rect x="7" y="76" width="5.2" height="6.5" fill={glass} opacity={paneOp} />
      <rect x="12.8" y="76" width="5.2" height="6.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="6" y="75.5" width="13" height="1.2" fill={sash} />
      <rect x="12.2" y="68" width="1" height="16" fill={sash} />
      <rect x="5.5" y="67.5" width="14" height="1.5" fill={sash} />
      <rect x="5.5" y="83.5" width="14" height="1.2" fill={sash} />
    </g>
    <g>
      <rect x="24" y="68" width="13" height="16" fill={brickDark} />
      <rect x="25" y="69" width="5.2" height="6.5" fill={glass} opacity={paneOp} />
      <rect x="30.8" y="69" width="5.2" height="6.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="25" y="76" width="5.2" height="6.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="30.8" y="76" width="5.2" height="6.5" fill={glass} opacity={paneOp} />
      <rect x="24" y="75.5" width="13" height="1.2" fill={sash} />
      <rect x="30.2" y="68" width="1" height="16" fill={sash} />
      <rect x="23.5" y="67.5" width="14" height="1.5" fill={sash} />
      <rect x="23.5" y="83.5" width="14" height="1.2" fill={sash} />
    </g>
    <!-- door + doorstep -->
    <rect x="14" y="96" width="14" height="22" fill="#3A2A22" />
    <rect x="15.5" y="98" width="11" height="18" fill="#4A3830" />
    <circle cx="25" cy="108" r="0.9" fill="#C8B090" />
    <rect x="12" y="116" width="18" height="2.5" fill={mortar} />
    <rect x="11" y="117.5" width="20" height="1.2" fill={brickDark} />
    {#if winter}
      <rect x="-1" y="57" width="44" height="3" fill={g.accent} opacity="0.95" />
      <ellipse cx="12" cy="46" rx="6" ry="2" fill={g.accent} />
      <ellipse cx="16" cy="45" rx="5" ry="1.8" fill={g.accent} />
    {/if}
    {#if summer}
      <rect x="6" y="86" width="13" height="4" fill="#5A4030" />
      <ellipse cx="9" cy="85" rx="2" ry="1.5" fill="#3A8A3A" />
      <ellipse cx="13" cy="84.5" rx="2.2" ry="1.6" fill="#4A9A48" />
      <circle cx="11" cy="84" r="1.1" fill="#E07080" />
      <circle cx="15" cy="84.2" r="1" fill="#E8C848" />
    {/if}
  </g>

  <!-- House 2 -->
  <g>
    <rect x="42" y="58" width="38" height="60" fill={brickLite} />
    <g stroke={mortar} stroke-width="0.35" opacity="0.32">
      {#each [64, 70, 76, 82, 88, 94, 100, 106, 112] as y}
        <line x1="42" {y} x2="80" y2={y} />
      {/each}
    </g>
    <rect x="41" y="56" width="40" height="3" fill={brickDark} />
    <rect x="41" y="54" width="40" height="2.5" fill={slateDark} />
    <rect x="52" y="42" width="12" height="14" fill={brick} />
    <rect x="51" y="40" width="5" height="4" rx="0.5" fill={brickDark} />
    <rect x="58" y="39" width="5" height="5" rx="0.5" fill={brickDark} />
    <rect x="64" y="41" width="4" height="3.5" rx="0.4" fill={brickDark} />
    <ellipse cx="53.5" cy="40" rx="2" ry="0.7" fill={slate} />
    <ellipse cx="60.5" cy="39" rx="2" ry="0.7" fill={slate} />
    <ellipse cx="66" cy="41" rx="1.6" ry="0.6" fill={slate} />
    <!-- upper sashes -->
    <g>
      <rect x="48" y="64" width="11" height="14" fill={brickDark} />
      <rect x="49" y="65" width="4.2" height="5.5" fill={glass} opacity={paneOp} />
      <rect x="53.6" y="65" width="4.2" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="49" y="71" width="4.2" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="53.6" y="71" width="4.2" height="5.5" fill={glass} opacity={paneOp} />
      <rect x="48" y="70.2" width="11" height="1" fill={sash} />
      <rect x="53" y="64" width="0.9" height="14" fill={sash} />
    </g>
    <g>
      <rect x="64" y="64" width="11" height="14" fill={brickDark} />
      <rect x="65" y="65" width="4.2" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="69.6" y="65" width="4.2" height="5.5" fill={glass} opacity={paneOp} />
      <rect x="65" y="71" width="4.2" height="5.5" fill={glass} opacity={paneOp} />
      <rect x="69.6" y="71" width="4.2" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="64" y="70.2" width="11" height="1" fill={sash} />
      <rect x="69" y="64" width="0.9" height="14" fill={sash} />
    </g>
    <!-- bay-ish ground window -->
    <rect x="48" y="86" width="12" height="14" fill={brickDark} />
    <rect x="49" y="87" width="10" height="12" fill={glassLit} opacity={night ? 0.9 : golden ? 0.7 : 0.55} />
    <line x1="54" y1="87" x2="54" y2="99" stroke={sash} stroke-width="0.8" />
    <line x1="49" y1="93" x2="59" y2="93" stroke={sash} stroke-width="0.8" />
    <!-- door -->
    <rect x="64" y="94" width="12" height="24" fill="#2A3A48" />
    <rect x="65.5" y="96" width="9" height="20" fill="#3A4A58" />
    <rect x="68" y="98" width="4" height="5" fill={glassLit} opacity={night ? 0.85 : 0.45} />
    <circle cx="73" cy="108" r="0.8" fill="#C8B090" />
    <rect x="62" y="116" width="16" height="2.5" fill={mortar} />
    <rect x="61" y="117.5" width="18" height="1.2" fill={brickDark} />
    {#if winter}
      <rect x="41" y="53" width="40" height="3" fill={g.accent} opacity="0.95" />
      <ellipse cx="58" cy="40" rx="9" ry="2.4" fill={g.accent} />
    {/if}
    {#if summer}
      <rect x="48" y="82" width="12" height="3.5" fill="#5A4030" />
      <ellipse cx="51" cy="81" rx="2.2" ry="1.4" fill="#348034" />
      <ellipse cx="56" cy="80.5" rx="2.4" ry="1.5" fill="#3A8A3A" />
      <circle cx="53" cy="80" r="1" fill="#E88848" />
      <circle cx="57" cy="80.2" r="0.9" fill="#E07080" />
    {/if}
  </g>

  <!-- ===== THE SOUND & KITTEN PUB ===== -->
  <g>
    <rect x="80" y="54" width="52" height="64" fill={pubBrick} />
    <g stroke={pubMortar} stroke-width="0.35" opacity="0.4">
      {#each [60, 66, 72, 78, 84, 90, 96, 102, 108, 114] as y}
        <line x1="80" {y} x2="132" y2={y} />
      {/each}
    </g>
    <rect x="79" y="52" width="54" height="3" fill="#4A2E28" />
    <rect x="79" y="50" width="54" height="2.5" fill={slate} />
    <!-- twin chimney -->
    <rect x="96" y="38" width="14" height="14" fill="#5A3830" />
    <rect x="95" y="36" width="6" height="4.5" rx="0.5" fill="#7A5040" />
    <rect x="103" y="35" width="6" height="5.5" rx="0.5" fill="#7A5040" />
    <ellipse cx="98" cy="36" rx="2.4" ry="0.8" fill={slateDark} />
    <ellipse cx="106" cy="35" rx="2.4" ry="0.8" fill={slateDark} />
    <!-- upper windows -->
    <g>
      <rect x="86" y="58" width="14" height="14" fill="#3A2820" />
      <rect x="87" y="59" width="5.5" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="93" y="59" width="5.5" height="5.5" fill={night ? glassLit : glass} opacity={night ? 0.9 : paneOp} />
      <rect x="87" y="65" width="5.5" height="5.5" fill={night ? glassLit : glass} opacity={night ? 0.88 : paneOp} />
      <rect x="93" y="65" width="5.5" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="86" y="64.2" width="14" height="1.1" fill={cream} />
      <rect x="92.5" y="58" width="1" height="14" fill={cream} />
    </g>
    <g>
      <rect x="112" y="58" width="14" height="14" fill="#3A2820" />
      <rect x="113" y="59" width="5.5" height="5.5" fill={night ? glassLit : glass} opacity={night ? 0.92 : paneOp} />
      <rect x="119" y="59" width="5.5" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="113" y="65" width="5.5" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="119" y="65" width="5.5" height="5.5" fill={night ? glassLit : glass} opacity={night ? 0.88 : paneOp} />
      <rect x="112" y="64.2" width="14" height="1.1" fill={cream} />
      <rect x="118.5" y="58" width="1" height="14" fill={cream} />
    </g>
    <!-- hanging pub sign -->
    <rect x="128" y="56" width="1.5" height="10" fill="#3A2A22" />
    <rect x="124" y="64" width="14" height="12" rx="0.5" fill="#2A1E14" stroke="#C8A878" stroke-width="0.8" />
    <text
      x="131"
      y="69"
      text-anchor="middle"
      font-size="3.2"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={cream}>The</text
    >
    <text
      x="131"
      y="73.5"
      text-anchor="middle"
      font-size="3.4"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={cream}>Sound &</text
    >
    <text
      x="131"
      y="78"
      text-anchor="middle"
      font-size="3.4"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={cream}>Kitten</text
    >
    <!-- striped awning -->
    <path d="M82 78 L106 72 L130 78 L130 84 L82 84 Z" fill="#2A4A68" />
    <g fill={cream} opacity="0.9">
      <rect x="86" y="76" width="3.2" height="8" transform="skewX(-16)" />
      <rect x="94" y="74.5" width="3.2" height="9.5" transform="skewX(-16)" />
      <rect x="102" y="73" width="3.2" height="11" transform="skewX(-16)" />
      <rect x="110" y="74.5" width="3.2" height="9.5" transform="skewX(-16)" />
      <rect x="118" y="76" width="3.2" height="8" transform="skewX(-16)" />
    </g>
    <path d="M82 78 L106 72 L130 78" fill="none" stroke="#1A3048" stroke-width="1" />
    <!-- fascia board -->
    <rect x="84" y="84" width="44" height="8" fill="#1A2A38" />
    <text
      x="106"
      y="90"
      text-anchor="middle"
      font-size="4.6"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={cream}>SOUND & KITTEN</text
    >
    <!-- pub doors / windows -->
    <rect x="86" y="94" width="16" height="24" fill="#2A2018" />
    <rect x="88" y="96" width="5.5" height="10" fill={glassLit} opacity={night ? 0.95 : golden ? 0.75 : 0.5} />
    <rect x="94.5" y="96" width="5.5" height="10" fill={glassLit} opacity={night ? 0.9 : golden ? 0.7 : 0.45} />
    <rect x="88" y="108" width="12" height="8" fill="#3A2E24" />
    <rect x="108" y="94" width="18" height="16" fill="#2A2018" />
    <rect x="110" y="96" width="6.5" height="12" fill={glassLit} opacity={night ? 0.95 : golden ? 0.72 : 0.48} />
    <rect x="117.5" y="96" width="6.5" height="12" fill={glassLit} opacity={night ? 0.9 : golden ? 0.68 : 0.42} />
    <rect x="106" y="116" width="22" height="2.5" fill={mortar} />
    {#if winter}
      <rect x="79" y="49" width="54" height="3" fill={g.accent} opacity="0.95" />
      <ellipse cx="102" cy="36" rx="10" ry="2.6" fill={g.accent} />
    {/if}
    {#if summer}
      <rect x="86" y="90" width="16" height="3.5" fill="#4A3828" />
      <ellipse cx="90" cy="89" rx="2.4" ry="1.5" fill="#3A8A3A" />
      <ellipse cx="96" cy="88.5" rx="2.6" ry="1.6" fill="#4A9A48" />
      <circle cx="92" cy="88" r="1.1" fill="#E07080" />
      <circle cx="97" cy="88.2" r="1" fill="#E8C040" />
    {/if}
  </g>

  <!-- Mid terrace filler: pub → corridor (set-back rooftops, keep run clear) -->
  <g>
    <!-- Shop row continuing past pub -->
    <rect x="132" y="62" width="28" height="56" fill={brick} />
    <g stroke={mortar} stroke-width="0.3" opacity="0.28">
      {#each [68, 74, 80, 86, 92, 98, 104, 110] as y}
        <line x1="132" {y} x2="160" y2={y} />
      {/each}
    </g>
    <rect x="131" y="60" width="30" height="2.5" fill={slate} />
    <rect x="140" y="48" width="10" height="14" fill={brickDark} />
    <rect x="139" y="46" width="5" height="4" rx="0.4" fill={brickLite} />
    <rect x="145" y="45" width="5" height="5" rx="0.4" fill={brickLite} />
    <rect x="136" y="68" width="10" height="12" fill={brickDark} />
    <rect x="137" y="69" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="141.2" y="69" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="137" y="74" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="141.2" y="74" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="148" y="68" width="10" height="12" fill={brickDark} />
    <rect x="149" y="69" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="153.2" y="69" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="149" y="74" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="153.2" y="74" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="140" y="96" width="12" height="22" fill="#3A2A22" />
    <rect x="141.5" y="98" width="9" height="18" fill="#4A3830" />
    <rect x="138" y="116" width="16" height="2" fill={mortar} />
    {#if winter}
      <rect x="131" y="59" width="30" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <g>
    <!-- Narrow mews house -->
    <rect x="160" y="58" width="22" height="60" fill={brickLite} />
    <g stroke={mortar} stroke-width="0.3" opacity="0.28">
      {#each [64, 70, 76, 82, 88, 94, 100, 106, 112] as y}
        <line x1="160" {y} x2="182" y2={y} />
      {/each}
    </g>
    <rect x="159" y="56" width="24" height="2.5" fill={slateDark} />
    <rect x="166" y="44" width="8" height="14" fill={brick} />
    <rect x="165" y="42" width="4" height="4" rx="0.4" fill={brickDark} />
    <rect x="170" y="41" width="4" height="5" rx="0.4" fill={brickDark} />
    <rect x="164" y="64" width="14" height="14" fill={brickDark} />
    <rect x="165" y="65" width="5.5" height="5.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="171" y="65" width="5.5" height="5.5" fill={glass} opacity={paneOp} />
    <rect x="165" y="71" width="5.5" height="5.5" fill={glass} opacity={paneOp} />
    <rect x="171" y="71" width="5.5" height="5.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="164" y="70.2" width="14" height="1" fill={sash} />
    <rect x="166" y="94" width="10" height="24" fill="#2A3A48" />
    <rect x="164" y="116" width="14" height="2" fill={mortar} />
    {#if winter}
      <rect x="159" y="55" width="24" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <g>
    <!-- Bookshop / corner — still left of clear mid corridor -->
    <rect x="182" y="64" width="28" height="54" fill={brick} />
    <g stroke={mortar} stroke-width="0.3" opacity="0.26">
      {#each [70, 76, 82, 88, 94, 100, 106, 112] as y}
        <line x1="182" {y} x2="210" y2={y} />
      {/each}
    </g>
    <rect x="181" y="62" width="30" height="2.5" fill={slate} />
    <rect x="186" y="70" width="20" height="10" fill="#1A2430" />
    <text
      x="196"
      y="77.5"
      text-anchor="middle"
      font-size="3.6"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={cream}>meowtro</text
    >
    <rect x="186" y="84" width="9" height="12" fill={glassLit} opacity={night ? 0.9 : 0.5} />
    <rect x="197" y="84" width="9" height="12" fill={glass} opacity={paneOp} />
    <path d="M184 84 L196 78 L208 84 L208 88 L184 88 Z" fill="#6A3040" />
    <rect x="190" y="100" width="12" height="18" fill="#3A2820" />
    <rect x="188" y="116" width="16" height="2" fill={mortar} />
    {#if winter}
      <rect x="181" y="61" width="30" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- Terrace continues across corridor (set back — bottoms ~y96, run strip clear) -->
  <g>
    <!-- House 3 — set back, above corridor -->
    <rect x="210" y="52" width="40" height="46" fill={brick} opacity="0.95" />
    <g stroke={mortar} stroke-width="0.3" opacity="0.28">
      {#each [58, 64, 70, 76, 82, 88] as y}
        <line x1="210" {y} x2="250" y2={y} />
      {/each}
    </g>
    <rect x="209" y="50" width="42" height="2.5" fill={slate} />
    <rect x="222" y="38" width="12" height="14" fill={brickDark} />
    <rect x="221" y="36" width="5" height="4" rx="0.4" fill={brickLite} />
    <rect x="228" y="35" width="5" height="5" rx="0.4" fill={brickLite} />
    <g>
      <rect x="216" y="56" width="12" height="14" fill={brickDark} />
      <rect x="217" y="57" width="4.6" height="5.5" fill={glass} opacity={paneOp} />
      <rect x="222.2" y="57" width="4.6" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="217" y="63" width="4.6" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="222.2" y="63" width="4.6" height="5.5" fill={glass} opacity={paneOp} />
      <rect x="216" y="62.2" width="12" height="1" fill={sash} />
    </g>
    <g>
      <rect x="232" y="56" width="12" height="14" fill={brickDark} />
      <rect x="233" y="57" width="4.6" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="238.2" y="57" width="4.6" height="5.5" fill={glass} opacity={paneOp} />
      <rect x="233" y="63" width="4.6" height="5.5" fill={glass} opacity={paneOp} />
      <rect x="238.2" y="63" width="4.6" height="5.5" fill={glassLit} opacity={paneLitOp} />
      <rect x="232" y="62.2" width="12" height="1" fill={sash} />
    </g>
    <!-- lower floor suggestion without blocking corridor -->
    <rect x="214" y="78" width="32" height="18" fill={brickDark} opacity="0.55" />
    <rect x="218" y="82" width="8" height="10" fill={glassLit} opacity={night ? 0.75 : 0.35} />
    <rect x="230" y="82" width="8" height="10" fill={glass} opacity={night ? 0.7 : 0.3} />
    {#if winter}
      <rect x="209" y="49" width="42" height="2.5" fill={g.accent} />
      <ellipse cx="228" cy="36" rx="8" ry="2" fill={g.accent} />
    {/if}
  </g>

  <g>
    <!-- House 3b — fills void toward right terrace -->
    <rect x="250" y="54" width="36" height="44" fill={brickLite} opacity="0.95" />
    <g stroke={mortar} stroke-width="0.3" opacity="0.26">
      {#each [60, 66, 72, 78, 84, 90] as y}
        <line x1="250" {y} x2="286" y2={y} />
      {/each}
    </g>
    <rect x="249" y="52" width="38" height="2.5" fill={slateDark} />
    <rect x="260" y="40" width="12" height="14" fill={brick} />
    <rect x="259" y="38" width="5" height="4" rx="0.4" fill={brickDark} />
    <rect x="266" y="37" width="5" height="5" rx="0.4" fill={brickDark} />
    <rect x="256" y="60" width="10" height="12" fill={brickDark} />
    <rect x="257" y="61" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="261.2" y="61" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="257" y="66" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="261.2" y="66" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="270" y="60" width="10" height="12" fill={brickDark} />
    <rect x="271" y="61" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="275.2" y="61" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="271" y="66" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="275.2" y="66" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="254" y="80" width="28" height="16" fill={brick} opacity="0.5" />
    <rect x="258" y="84" width="8" height="9" fill={glassLit} opacity={night ? 0.7 : 0.32} />
    <rect x="270" y="84" width="8" height="9" fill={glass} opacity={night ? 0.65 : 0.28} />
    {#if winter}
      <rect x="249" y="51" width="38" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <g>
    <!-- House 3c — connects to right terrace -->
    <rect x="286" y="58" width="30" height="50" fill={brick} />
    <g stroke={mortar} stroke-width="0.3" opacity="0.28">
      {#each [64, 70, 76, 82, 88, 94, 100] as y}
        <line x1="286" {y} x2="316" y2={y} />
      {/each}
    </g>
    <rect x="285" y="56" width="32" height="2.5" fill={slate} />
    <rect x="294" y="44" width="10" height="14" fill={brickDark} />
    <rect x="293" y="42" width="5" height="4" rx="0.4" fill={brickLite} />
    <rect x="299" y="41" width="5" height="5" rx="0.4" fill={brickLite} />
    <rect x="290" y="64" width="10" height="12" fill={brickDark} />
    <rect x="291" y="65" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="295.2" y="65" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="291" y="70" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="295.2" y="70" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="302" y="64" width="10" height="12" fill={brickDark} />
    <rect x="303" y="65" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="307.2" y="65" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="303" y="70" width="3.8" height="4.5" fill={glass} opacity={paneOp} />
    <rect x="307.2" y="70" width="3.8" height="4.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="294" y="90" width="12" height="18" fill="#3A2820" />
    <rect x="292" y="116" width="16" height="2" fill={mortar} />
    {#if winter}
      <rect x="285" y="55" width="32" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <g>
    <!-- House 3d — tight join before house 4 -->
    <rect x="316" y="60" width="24" height="58" fill={brickLite} />
    <g stroke={mortar} stroke-width="0.3" opacity="0.28">
      {#each [66, 72, 78, 84, 90, 96, 102, 108, 114] as y}
        <line x1="316" {y} x2="340" y2={y} />
      {/each}
    </g>
    <rect x="315" y="58" width="26" height="2.5" fill={slateDark} />
    <rect x="322" y="46" width="10" height="14" fill={brick} />
    <rect x="321" y="44" width="5" height="4" rx="0.4" fill={brickDark} />
    <rect x="327" y="43" width="5" height="5" rx="0.4" fill={brickDark} />
    <rect x="320" y="66" width="16" height="14" fill={brickDark} />
    <rect x="321" y="67" width="6.5" height="5.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="328.5" y="67" width="6.5" height="5.5" fill={glass} opacity={paneOp} />
    <rect x="321" y="73" width="6.5" height="5.5" fill={glass} opacity={paneOp} />
    <rect x="328.5" y="73" width="6.5" height="5.5" fill={glassLit} opacity={paneLitOp} />
    <rect x="320" y="72.2" width="16" height="1" fill={sash} />
    <rect x="322" y="94" width="12" height="24" fill="#2A3A48" />
    <rect x="320" y="116" width="16" height="2" fill={mortar} />
    {#if winter}
      <rect x="315" y="57" width="26" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- House 4 (right terrace) -->
  <g>
    <rect x="340" y="60" width="40" height="58" fill={brickLite} />
    <g stroke={mortar} stroke-width="0.35" opacity="0.3">
      {#each [66, 72, 78, 84, 90, 96, 102, 108, 114] as y}
        <line x1="340" {y} x2="380" y2={y} />
      {/each}
    </g>
    <rect x="339" y="58" width="42" height="3" fill={brickDark} />
    <rect x="339" y="56" width="42" height="2.5" fill={slateDark} />
    <rect x="352" y="44" width="12" height="14" fill={brick} />
    <rect x="351" y="42" width="5.5" height="4.5" rx="0.5" fill={brickDark} />
    <rect x="358" y="41" width="5.5" height="5.5" rx="0.5" fill={brickDark} />
    <ellipse cx="353.8" cy="42" rx="2.2" ry="0.75" fill={slate} />
    <ellipse cx="360.8" cy="41" rx="2.2" ry="0.75" fill={slate} />
    <g>
      <rect x="346" y="66" width="12" height="15" fill={brickDark} />
      <rect x="347" y="67" width="4.6" height="6" fill={glassLit} opacity={paneLitOp} />
      <rect x="352.2" y="67" width="4.6" height="6" fill={glass} opacity={paneOp} />
      <rect x="347" y="73.5" width="4.6" height="6" fill={glass} opacity={paneOp} />
      <rect x="352.2" y="73.5" width="4.6" height="6" fill={glassLit} opacity={paneLitOp} />
      <rect x="346" y="72.8" width="12" height="1.1" fill={sash} />
      <rect x="351.5" y="66" width="0.9" height="15" fill={sash} />
    </g>
    <g>
      <rect x="362" y="66" width="12" height="15" fill={brickDark} />
      <rect x="363" y="67" width="4.6" height="6" fill={glass} opacity={paneOp} />
      <rect x="368.2" y="67" width="4.6" height="6" fill={glassLit} opacity={paneLitOp} />
      <rect x="363" y="73.5" width="4.6" height="6" fill={glassLit} opacity={paneLitOp} />
      <rect x="368.2" y="73.5" width="4.6" height="6" fill={glass} opacity={paneOp} />
      <rect x="362" y="72.8" width="12" height="1.1" fill={sash} />
      <rect x="367.5" y="66" width="0.9" height="15" fill={sash} />
    </g>
    <rect x="352" y="94" width="14" height="24" fill="#3A2820" />
    <rect x="353.5" y="96" width="11" height="20" fill="#4A3830" />
    <rect x="356" y="98" width="6" height="5" fill={glassLit} opacity={night ? 0.8 : 0.4} />
    <circle cx="362" cy="108" r="0.85" fill="#C8B090" />
    <rect x="350" y="116" width="18" height="2.5" fill={mortar} />
    <rect x="349" y="117.5" width="20" height="1.2" fill={brickDark} />
    {#if winter}
      <rect x="339" y="55" width="42" height="3" fill={g.accent} opacity="0.95" />
      <ellipse cx="358" cy="42" rx="8" ry="2.2" fill={g.accent} />
    {/if}
    {#if summer}
      <rect x="346" y="84" width="12" height="3.5" fill="#5A4030" />
      <ellipse cx="349" cy="83" rx="2.2" ry="1.4" fill="#348034" />
      <ellipse cx="354" cy="82.5" rx="2.4" ry="1.5" fill="#3A8A3A" />
      <circle cx="351" cy="82" r="1" fill="#E07080" />
      <circle cx="355" cy="82.2" r="0.9" fill="#E8C848" />
    {/if}
  </g>

  <!-- House 5 (far right) -->
  <g>
    <rect x="380" y="64" width="40" height="54" fill={brick} />
    <g stroke={mortar} stroke-width="0.3" opacity="0.28">
      {#each [70, 76, 82, 88, 94, 100, 106, 112] as y}
        <line x1="380" {y} x2="420" y2={y} />
      {/each}
    </g>
    <rect x="379" y="62" width="42" height="2.5" fill={slate} />
    <rect x="396" y="50" width="10" height="14" fill={brickDark} />
    <rect x="395" y="48" width="5" height="4" rx="0.4" fill={brickLite} />
    <rect x="401" y="47" width="5" height="5" rx="0.4" fill={brickLite} />
    <g>
      <rect x="388" y="70" width="11" height="13" fill={brickDark} />
      <rect x="389" y="71" width="4.2" height="5" fill={glassLit} opacity={paneLitOp} />
      <rect x="393.6" y="71" width="4.2" height="5" fill={glass} opacity={paneOp} />
      <rect x="389" y="76.5" width="4.2" height="5" fill={glass} opacity={paneOp} />
      <rect x="393.6" y="76.5" width="4.2" height="5" fill={glassLit} opacity={paneLitOp} />
      <rect x="388" y="75.8" width="11" height="1" fill={sash} />
    </g>
    <g>
      <rect x="402" y="70" width="11" height="13" fill={brickDark} />
      <rect x="403" y="71" width="4.2" height="5" fill={glass} opacity={paneOp} />
      <rect x="407.6" y="71" width="4.2" height="5" fill={glassLit} opacity={paneLitOp} />
      <rect x="403" y="76.5" width="4.2" height="5" fill={glassLit} opacity={paneLitOp} />
      <rect x="407.6" y="76.5" width="4.2" height="5" fill={glass} opacity={paneOp} />
      <rect x="402" y="75.8" width="11" height="1" fill={sash} />
    </g>
    <rect x="392" y="96" width="12" height="22" fill="#2A3A48" />
    <rect x="393.5" y="98" width="9" height="18" fill="#3A4A58" />
    <circle cx="400.5" cy="108" r="0.8" fill="#C8B090" />
    <rect x="390" y="116" width="16" height="2.5" fill={mortar} />
    {#if winter}
      <rect x="379" y="61" width="42" height="2.5" fill={g.accent} />
      <ellipse cx="401" cy="48" rx="7" ry="2" fill={g.accent} />
    {/if}
  </g>

  <!-- meowtropolitan plaque (edge, left of corridor) -->
  <g>
    <rect x="134" y="98" width="3" height="20" fill="#5A6874" />
    <rect x="128" y="96" width="15" height="10" rx="0.5" fill="#1A3A58" />
    <text
      x="135.5"
      y="102.5"
      text-anchor="middle"
      font-size="3.1"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={cream}>meowtro-</text
    >
    <text
      x="135.5"
      y="106.5"
      text-anchor="middle"
      font-size="3.1"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={cream}>politan</text
    >
  </g>

  <!-- ===== STREET FURNITURE (edges) ===== -->

  <!-- Red phone booth (left edge) -->
  <g transform="translate(4, 78)">
    <rect x="0" y="4" width="18" height="36" fill={boothRed} />
    <rect x="1" y="0" width="16" height="5" fill="#8A2018" />
    <rect x="2" y="1.5" width="14" height="2.5" fill="#E8C040" opacity="0.85" />
    <text
      x="9"
      y="3.5"
      text-anchor="middle"
      font-size="2.4"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#2A2010">TELEPHONE</text
    >
    <rect x="2" y="8" width="14" height="18" fill="#3A1A18" />
    <g fill={glassLit} opacity={night ? 0.95 : golden ? 0.75 : 0.55}>
      <rect x="3" y="9" width="5.5" height="7.5" />
      <rect x="9.5" y="9" width="5.5" height="7.5" />
      <rect x="3" y="17.5" width="5.5" height="7.5" />
      <rect x="9.5" y="17.5" width="5.5" height="7.5" />
    </g>
    {#if night}
      <ellipse cx="9" cy="17" rx="7" ry="9" fill="#FFE8A8" opacity="0.28" />
    {:else if golden}
      <ellipse cx="9" cy="17" rx="5" ry="7" fill="#FFD090" opacity="0.18" />
    {/if}
    <rect x="2" y="16.2" width="14" height="1" fill={boothRed} />
    <rect x="8.5" y="8" width="1" height="18" fill={boothRed} />
    <rect x="3" y="28" width="12" height="10" fill="#8A2018" />
    <circle cx="13" cy="33" r="0.7" fill="#C8B090" />
    <rect x="-1" y="38" width="20" height="2" fill="#6A1814" />
    {#if winter}
      <rect x="0" y="3" width="18" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- Pillar box / post box (right of left cluster, still edge) -->
  <g transform="translate(66, 96)">
    <rect x="2" y="6" width="12" height="16" rx="1" fill={postRed} />
    <rect x="1" y="4" width="14" height="4" rx="1" fill="#8A1E1E" />
    <ellipse cx="8" cy="4" rx="7" ry="2.2" fill="#8A1E1E" />
    <rect x="4" y="8" width="8" height="3.5" rx="0.4" fill="#1A1A1C" />
    <rect x="5" y="14" width="6" height="2" fill="#8A1E1E" />
    <rect x="3" y="20" width="10" height="2" fill="#6A1814" />
    {#if winter}
      <ellipse cx="8" cy="3.5" rx="6.5" ry="1.8" fill={g.accent} />
    {/if}
  </g>

  <!-- Street lamps -->
  <g fill="#5A6874">
    <rect x="148" y="78" width="2.2" height="40" />
    <path d="M144 78 Q149 70 154 78" fill="none" stroke="#5A6874" stroke-width="2" />
    <circle cx="154" cy="80" r="2.6" fill={lamp} opacity="0.9" />
    <rect x="392" y="80" width="2.2" height="38" />
    <path d="M388 80 Q393 72 398 80" fill="none" stroke="#5A6874" stroke-width="2" />
    <circle cx="398" cy="82" r="2.4" fill={lamp} opacity="0.85" />
  </g>

  <!-- Bollards near zebra (edges of crossing, corridor-safe) -->
  <g fill="#8A9098">
    <rect x="168" y="112" width="3" height="6" rx="0.5" />
    <ellipse cx="169.5" cy="112" rx="2" ry="1" fill="#A0A8B0" />
    <rect x="248" y="112" width="3" height="6" rx="0.5" />
    <ellipse cx="249.5" cy="112" rx="2" ry="1" fill="#A0A8B0" />
  </g>

  <!-- ===== SEASONAL (animated group 3: sk-sway) ===== -->
  {#if fall}
    <g class="sk-sway">
      <ellipse cx="28" cy="112" rx="2.4" ry="1.3" fill={foliageWarm[0]} />
      <ellipse cx="38" cy="114" rx="2.1" ry="1.1" fill={foliageWarm[1]} />
      <ellipse cx="360" cy="113" rx="2.3" ry="1.2" fill={foliageWarm[2]} />
      <ellipse cx="372" cy="115" rx="2" ry="1.1" fill={foliageWarm[3]} />
      <ellipse cx="410" cy="114" rx="2.2" ry="1.2" fill={foliageWarm[1]} />
      <path
        d="M70 70 Q74 66 72 62"
        fill="none"
        stroke={foliageWarm[0]}
        stroke-width="1.4"
        stroke-linecap="round"
        opacity="0.85"
      />
      <path
        d="M350 72 Q346 68 348 64"
        fill="none"
        stroke={foliageWarm[2]}
        stroke-width="1.3"
        stroke-linecap="round"
        opacity="0.8"
      />
    </g>
  {:else if spring}
    <g class="sk-sway">
      <ellipse cx="22" cy="88" rx="8" ry="5" fill="#6AAA62" opacity="0.85" />
      <circle cx="18" cy="86" r="2" fill={blossom[0]} />
      <circle cx="24" cy="84" r="2.2" fill={blossom[1]} />
      <circle cx="28" cy="88" r="1.8" fill={blossom[2]} />
      <ellipse cx="400" cy="90" rx="7" ry="4.5" fill="#5A9A58" opacity="0.85" />
      <circle cx="396" cy="88" r="1.8" fill={blossom[2]} />
      <circle cx="402" cy="86" r="2" fill={blossom[0]} />
      <circle cx="406" cy="90" r="1.6" fill={blossom[1]} />
    </g>
    <!-- puddles -->
    <ellipse cx="160" cy="120" rx="14" ry="2.2" fill={glass} opacity="0.28" />
    <ellipse cx="280" cy="121" rx="12" ry="2" fill={glass} opacity="0.24" />
    <ellipse cx="40" cy="121" rx="10" ry="1.8" fill={glass} opacity="0.22" />
  {:else if summer}
    <g class="sk-sway">
      <ellipse cx="20" cy="86" rx="3" ry="2" fill="#3A8A3A" />
      <ellipse cx="26" cy="85" rx="2.6" ry="1.8" fill="#4A9A48" />
      <circle cx="22" cy="84" r="1.1" fill="#E07080" />
      <circle cx="27" cy="84.2" r="1" fill="#E8C040" />
      <ellipse cx="404" cy="88" rx="3" ry="2" fill="#348034" />
      <ellipse cx="410" cy="87" rx="2.5" ry="1.7" fill="#3A8A3A" />
      <circle cx="406" cy="86" r="1" fill="#E88848" />
    </g>
  {:else}
    <!-- winter: soft snow drift sway accents on ledge edges -->
    <g class="sk-sway" opacity="0.9">
      <ellipse cx="30" cy="116" rx="10" ry="2.2" fill={g.accent} />
      <ellipse cx="370" cy="116" rx="11" ry="2.2" fill={g.accent} />
      <ellipse cx="410" cy="115.5" rx="8" ry="1.8" fill={g.accent} />
    </g>
  {/if}

  <!-- Rain sheen hints (winter / spring) -->
  {#if winter || spring}
    <g fill={winter ? '#D0DCE8' : glass} opacity={winter ? 0.22 : 0.2}>
      <ellipse cx="100" cy="122" rx="16" ry="2.4" />
      <ellipse cx="190" cy="123" rx="12" ry="1.8" />
      <ellipse cx="300" cy="122.5" rx="14" ry="2" />
      <ellipse cx="50" cy="124" rx="9" ry="1.5" />
      <ellipse cx="380" cy="123" rx="11" ry="1.7" />
    </g>
  {/if}

  <!-- ===== GROUND PLANE (top y = 118) ===== -->
  <rect x="0" y="118" width="420" height="8" fill={g.near} />
  <rect x="0" y="118" width="420" height="1.5" fill={g.detail} opacity="0.4" />
  <!-- kerb cracks -->
  <g stroke={g.detail} stroke-width="0.55" opacity="0.28">
    {#each [30, 70, 150, 210, 270, 330, 390] as x}
      <line {x} y1="118" x2={x} y2="126" />
    {/each}
  </g>
  <rect x="0" y="125" width="420" height="2.5" fill={g.detail} opacity="0.55" />
  <rect x="0" y="127" width="420" height="13" fill={g.path} />

  <!-- animated group 2: lamp glow (cones + pools sit above pavement) -->
  <g class="sk-glow">
    {#if night}
      <path d="M154 80 L136 118 L172 118 Z" fill={lampWash} opacity="0.38" />
      <path d="M398 82 L382 118 L414 118 Z" fill={lampWash} opacity="0.34" />
      <ellipse cx="154" cy="80" rx="14" ry="11" fill={lampWash} opacity="0.55" />
      <ellipse cx="154" cy="80" rx="6" ry="5" fill={lamp} opacity="0.7" />
      <ellipse cx="398" cy="82" rx="13" ry="10" fill={lampWash} opacity="0.5" />
      <ellipse cx="398" cy="82" rx="5.5" ry="4.5" fill={lamp} opacity="0.65" />
      <ellipse cx="154" cy="119" rx="22" ry="5" fill={lampWash} opacity="0.42" />
      <ellipse cx="398" cy="119" rx="20" ry="4.5" fill={lampWash} opacity="0.38" />
      <ellipse cx="106" cy="119" rx="14" ry="3" fill="#FFE8A8" opacity="0.22" />
    {:else if golden}
      <path d="M154 80 L142 118 L166 118 Z" fill={lampWash} opacity="0.22" />
      <path d="M398 82 L388 118 L408 118 Z" fill={lampWash} opacity="0.2" />
      <ellipse cx="154" cy="80" rx="9" ry="7" fill={lampWash} opacity="0.4" />
      <ellipse cx="154" cy="80" rx="4" ry="3.2" fill={lamp} opacity="0.5" />
      <ellipse cx="398" cy="82" rx="8" ry="6.5" fill={lampWash} opacity="0.36" />
      <ellipse cx="398" cy="82" rx="3.5" ry="3" fill={lamp} opacity="0.45" />
      <ellipse cx="154" cy="119" rx="14" ry="3.2" fill={lampWash} opacity="0.24" />
      <ellipse cx="398" cy="119" rx="12" ry="3" fill={lampWash} opacity="0.2" />
    {:else}
      <ellipse cx="154" cy="80" rx="6" ry="5" fill={g.accent} opacity="0.32" />
      <ellipse cx="154" cy="80" rx="3" ry="2.5" fill={lamp} opacity="0.45" />
      <ellipse cx="398" cy="82" rx="5.5" ry="4.5" fill={g.accent} opacity="0.28" />
    {/if}
  </g>

  <!-- Zebra crossing (corridor zone, ground only) -->
  <g fill={winter ? g.accent : '#E8ECF0'} opacity={winter ? 0.85 : 0.7}>
    {#each [0, 1, 2, 3, 4, 5] as i}
      <rect x={178 + i * 10} y="127" width="6" height="12" />
    {/each}
  </g>

  <!-- Road lane marks -->
  <g fill={g.accent} opacity="0.3">
    {#each [50, 110, 260, 320, 380] as x}
      <rect {x} y="132" width="16" height="1.4" rx="0.4" />
    {/each}
  </g>

  {#if winter}
    <ellipse cx="36" cy="126" rx="20" ry="3.2" fill={g.accent} opacity="0.92" />
    <ellipse cx="360" cy="126" rx="22" ry="3.2" fill={g.accent} opacity="0.9" />
    <ellipse cx="400" cy="125.5" rx="16" ry="2.8" fill={g.accent} opacity="0.88" />
    <rect x="0" y="117" width="420" height="2.5" fill={g.accent} opacity="0.8" />
  {/if}
  {#if spring}
    <rect x="0" y="125" width="420" height="1.4" fill={glass} opacity="0.18" />
  {/if}
</g>
