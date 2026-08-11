<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Time, Weather } from '../../types'

  let { weather, time = 'day' }: { weather: Weather; time?: Time } = $props()

  const night = $derived(time === 'night')
  const golden = $derived(time === 'sunrise' || time === 'sunset')

  const g = $derived(groundFor('tokyo', weather))
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  const neonPink = $derived(
    golden
      ? fall
        ? '#F07878'
        : winter
          ? '#F09888'
          : '#FF6A78'
      : fall
        ? '#F06090'
        : winter
          ? '#E888B0'
          : spring
            ? '#FF6AA8'
            : '#FF4A9A',
  )
  const neonCyan = $derived(
    golden
      ? winter
        ? '#90D0C0'
        : fall
          ? '#70C8A8'
          : '#68E0C0'
      : winter
        ? '#7AD0E0'
        : fall
          ? '#50C8C0'
          : '#40E8F0',
  )
  const neonRed = $derived(golden ? (fall ? '#E85838' : '#FF4838') : fall ? '#E04040' : '#FF3048')
  const neonPurple = $derived(golden ? '#B878D0' : '#A060E0')
  const lantern = $derived(fall ? '#F0A040' : winter ? '#E88860' : spring ? '#F07878' : '#F05050')
  const lanternGlow = $derived(fall ? '#F0C070' : '#F09070')
  const building = $derived(
    night ? (winter ? '#222236' : '#161624') : winter ? '#3A3A50' : '#2A2A3E',
  )
  const buildingMid = $derived(
    night ? (winter ? '#2E2E42' : '#222234') : winter ? '#4A4A62' : '#36364A',
  )
  const buildingDark = $derived(
    night ? (winter ? '#181828' : '#10101C') : winter ? '#2A2A3A' : '#1E1E30',
  )
  const concrete = $derived(winter ? '#8A8A9A' : '#5A5A6A')
  const asphalt = $derived(winter ? '#4A4A58' : g.path)
  const glass = $derived(winter ? '#A8C0D8' : fall ? '#687888' : '#405060')
  const glassLit = $derived(
    night
      ? winter
        ? '#E0F0FF'
        : fall
          ? '#F0C080'
          : summer
            ? '#70E0F8'
            : '#88C8E8'
      : golden
        ? winter
          ? '#D8E8F0'
          : fall
            ? '#E8B878'
            : '#A8C8C0'
        : winter
          ? '#C8E0F0'
          : fall
            ? '#E0A868'
            : summer
              ? '#50C8E0'
              : '#68A8C8',
  )
  const cable = $derived(winter ? '#6A6A78' : '#3A3A48')
  const blossom = ['#F8A0C8', '#F080B0', '#E868A0'] as const
  const signInk = '#101018'
  const signCream = '#F2E8E0'

  const neonOp = $derived(night ? 1 : golden ? 0.88 : 0.72)
  const neonStripOp = $derived(night ? 0.95 : golden ? 0.82 : 0.75)
  const windowOp = $derived(night ? 0.88 : golden ? 0.62 : winter ? 0.7 : 0.45)
  const shopWinOp = $derived(night ? 0.82 : golden ? 0.58 : 0.55)
  const glowOp = $derived(night ? 0.42 : golden ? 0.28 : 0.16)
  const lanternGlowOp = $derived(night ? 0.5 : golden ? 0.32 : 0.22)
  const reflectOp = $derived(night ? 0.35 : golden ? 0.18 : 0)
  const vendingLit = $derived(night ? 0.9 : golden ? 0.7 : 0.55)
</script>

<g class="tokyo" aria-hidden="true">
  <rect class="sk-mist" x="0" y="40" width="420" height="78" fill={g.far} opacity="0.22" />

  <!-- ===== FAR SKYLINE + ANTENNAS ===== -->
  <g fill={g.far}>
    <rect x="0" y="52" width="20" height="66" />
    <rect x="16" y="38" width="26" height="80" />
    <rect x="38" y="48" width="16" height="70" />
    <rect x="50" y="28" width="30" height="90" />
    <rect x="62" y="14" width="4" height="16" />
    <rect x="63.5" y="4" width="1.2" height="12" />
    <rect x="78" y="44" width="22" height="74" />
    <rect x="96" y="32" width="34" height="86" />
    <rect x="108" y="18" width="6" height="16" />
    <rect x="110" y="8" width="1.5" height="12" />
    <rect x="128" y="46" width="20" height="72" />
    <rect x="146" y="36" width="38" height="82" />
    <rect x="158" y="20" width="10" height="18" />
    <rect x="161" y="8" width="1.5" height="14" />
    <rect x="182" y="50" width="24" height="68" />
    <rect x="202" y="30" width="36" height="88" />
    <rect x="214" y="16" width="8" height="16" />
    <rect x="217" y="6" width="1.5" height="12" />
    <rect x="236" y="42" width="28" height="76" />
    <rect x="260" y="26" width="40" height="92" />
    <rect x="274" y="12" width="8" height="16" />
    <rect x="277" y="2" width="1.5" height="12" />
    <rect x="298" y="40" width="26" height="78" />
    <rect x="320" y="34" width="34" height="84" />
    <rect x="332" y="18" width="6" height="18" />
    <rect x="334" y="8" width="1.4" height="12" />
    <rect x="352" y="46" width="24" height="72" />
    <rect x="372" y="32" width="30" height="86" />
    <rect x="398" y="42" width="22" height="76" />
  </g>

  <!-- Distant neon strips on towers -->
  <g opacity={neonStripOp}>
    <rect x="56" y="34" width="18" height="2.5" fill={neonPink} />
    <rect x="152" y="28" width="22" height="2" fill={neonCyan} />
    <rect x="266" y="30" width="26" height="2.5" fill={neonRed} />
    <rect x="376" y="36" width="16" height="2" fill={neonPurple} />
  </g>

  <!-- Antenna dishes -->
  <g fill={g.detail} opacity="0.85">
    <ellipse cx="64" cy="14" rx="5" ry="2" fill={g.accent} opacity="0.55" />
    <rect x="62.5" y="14" width="3" height="8" />
    <ellipse cx="278" cy="4" rx="4.5" ry="1.8" fill={neonCyan} opacity="0.45" />
    <rect x="276.5" y="4" width="2.5" height="8" />
  </g>

  <!-- ===== POWER LINES / CABLE ARCS ===== -->
  <g fill="none" stroke={cable} stroke-width="0.9" opacity="0.75">
    <path d="M8 42 Q70 28 130 40" />
    <path d="M8 45 Q70 32 130 43" stroke-width="0.55" opacity="0.6" />
    <path d="M130 40 Q210 24 290 38" />
    <path d="M130 43 Q210 28 290 41" stroke-width="0.55" opacity="0.55" />
    <path d="M290 38 Q350 26 412 36" />
    <path d="M290 41 Q350 30 412 39" stroke-width="0.55" opacity="0.5" />
  </g>
  <g fill={cable}>
    <rect x="8" y="40" width="2" height="20" opacity="0.7" />
    <rect x="128" y="38" width="2" height="18" opacity="0.7" />
    <rect x="288" y="36" width="2" height="20" opacity="0.7" />
    <rect x="410" y="34" width="2" height="18" opacity="0.65" />
  </g>

  <!-- ===== LEFT BLOCK: ramen + vertical signs ===== -->
  <g>
    <rect x="0" y="58" width="48" height="60" fill={building} />
    <rect x="0" y="58" width="48" height="4" fill={buildingDark} />
    <!-- window grid -->
    <g fill={glassLit} opacity={windowOp}>
      {#each [0, 1, 2] as row}
        {#each [0, 1] as col}
          <rect x={6 + col * 18} y={66 + row * 12} width="14" height="9" />
        {/each}
      {/each}
    </g>
    <g stroke={buildingDark} stroke-width="0.5" fill="none" opacity="0.65">
      {#each [0, 1, 2] as row}
        {#each [0, 1] as col}
          <line x1={13 + col * 18} y1={66 + row * 12} x2={13 + col * 18} y2={75 + row * 12} />
          <line x1={6 + col * 18} y1={70.5 + row * 12} x2={20 + col * 18} y2={70.5 + row * 12} />
        {/each}
      {/each}
    </g>
    <!-- AC units -->
    <rect x="4" y="86" width="10" height="5" rx="0.5" fill={concrete} />
    <rect x="30" y="98" width="10" height="5" rx="0.5" fill={concrete} />
    <g stroke={buildingDark} stroke-width="0.4" opacity="0.5">
      <line x1="5" y1="87.5" x2="13" y2="87.5" />
      <line x1="5" y1="89" x2="13" y2="89" />
    </g>
    <!-- horizontal neon sign -->
    <rect x="4" y="100" width="36" height="8" rx="0.5" fill={signInk} />
    <rect x="4" y="100" width="36" height="1.2" fill={neonPink} opacity={neonOp} />
    <text
      x="22"
      y="106.5"
      text-anchor="middle"
      font-size="5"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={neonPink}
      opacity={neonOp}>purr ramen</text
    >
    <rect x="8" y="108" width="12" height="10" fill="#1A1420" />
    <rect x="10" y="110" width="8" height="5" fill={glassLit} opacity={shopWinOp} />
    <rect x="24" y="108" width="18" height="10" fill={glass} opacity="0.5" />
    <!-- vertical noren-ish strip -->
    <rect x="42" y="72" width="7" height="38" fill={neonRed} opacity={neonOp} />
    <g
      fill={signCream}
      font-size="3.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      text-anchor="middle"
    >
      <text x="45.5" y="80">ら</text>
      <text x="45.5" y="86">ー</text>
      <text x="45.5" y="92">メ</text>
      <text x="45.5" y="98">ン</text>
    </g>
    {#if winter}
      <rect x="0" y="56" width="48" height="3" fill={g.accent} opacity="0.9" />
    {/if}
  </g>

  <!-- hanging lanterns (left edge, mid height) -->
  <g class="sk-sway" style="transform-origin: 26px 72px">
    <line x1="18" y1="70" x2="18" y2="78" stroke="#3A3030" stroke-width="0.7" />
    <ellipse cx="18" cy="82" rx="4.5" ry="5.5" fill={lantern} />
    <ellipse cx="18" cy="78" rx="3.5" ry="1.2" fill={lanternGlow} />
    <rect x="17.2" y="87" width="1.6" height="2.5" fill="#3A2820" />
    <line x1="34" y1="72" x2="34" y2="80" stroke="#3A3030" stroke-width="0.7" />
    <ellipse cx="34" cy="84" rx="4" ry="5" fill={lantern} opacity="0.9" />
    <ellipse cx="34" cy="80" rx="3" ry="1" fill={lanternGlow} />
    {#if spring}
      <ellipse cx="12" cy="72" rx="14" ry="9" fill="#E8A0B8" opacity="0.45" />
      <circle cx="6" cy="70" r="2.8" fill={blossom[0]} />
      <circle cx="14" cy="66" r="3.2" fill={blossom[1]} />
      <circle cx="20" cy="72" r="2.4" fill={blossom[2]} />
    {/if}
  </g>

  <!-- ===== ARCADE storefront ===== -->
  <g>
    <rect x="52" y="62" width="42" height="56" fill={buildingMid} />
    <rect x="52" y="62" width="42" height="5" fill={neonPurple} opacity={neonOp * 0.85} />
    <!-- marquee lights -->
    <g fill={neonCyan} opacity={neonOp}>
      {#each [56, 64, 72, 80, 88] as x}
        <circle cx={x} cy="64.5" r="1.3" opacity="0.9" />
      {/each}
    </g>
    <rect x="56" y="72" width="34" height="10" fill={signInk} />
    <text
      x="73"
      y="79.5"
      text-anchor="middle"
      font-size="5.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={neonCyan}
      opacity={neonOp}>neko arcade</text
    >
    <!-- lit game windows -->
    <g fill={glassLit} opacity={shopWinOp}>
      <rect x="56" y="86" width="14" height="16" />
      <rect x="74" y="86" width="14" height="16" fill={neonPink} opacity={night ? 0.55 : 0.35} />
    </g>
    <rect x="58" y="104" width="10" height="14" fill="#141420" />
    <rect x="72" y="104" width="16" height="14" fill={glass} opacity="0.45" />
    <!-- vertical neon -->
    <rect x="90" y="70" width="5" height="40" fill={neonPink} opacity={neonOp} />
    <g
      fill="#FFF0F8"
      font-size="3"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      text-anchor="middle"
    >
      <text x="92.5" y="80">G</text>
      <text x="92.5" y="86">A</text>
      <text x="92.5" y="92">M</text>
      <text x="92.5" y="98">E</text>
    </g>
    {#if winter}
      <rect x="52" y="60" width="42" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- ===== MID: dense alley stack ===== -->
  <g>
    <rect x="98" y="54" width="36" height="64" fill={buildingDark} />
    <rect x="98" y="54" width="36" height="3" fill={concrete} />
    <!-- fire escape hint -->
    <g fill="none" stroke={concrete} stroke-width="0.9" opacity="0.7">
      <rect x="104" y="68" width="20" height="8" />
      <rect x="104" y="82" width="20" height="8" />
      <rect x="104" y="96" width="20" height="8" />
      <line x1="104" y1="76" x2="104" y2="82" />
      <line x1="124" y1="90" x2="124" y2="96" />
    </g>
    <g fill={glass} opacity={night ? 0.55 : winter ? 0.65 : 0.4}>
      <rect x="106" y="70" width="6" height="5" />
      <rect x="116" y="70" width="6" height="5" fill={glassLit} />
      <rect x="106" y="84" width="6" height="5" fill={glassLit} />
      <rect x="116" y="84" width="6" height="5" />
      <rect x="106" y="98" width="6" height="5" />
      <rect x="116" y="98" width="6" height="5" fill={glassLit} />
    </g>
    <rect x="102" y="108" width="28" height="6" fill={signInk} />
    <text
      x="116"
      y="112.5"
      text-anchor="middle"
      font-size="3.6"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={neonCyan}
      opacity={neonOp}>kissaten</text
    >
    {#if winter}
      <rect x="98" y="52" width="36" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- ===== CENTER-LEFT: music / sound kit shop ===== -->
  <g>
    <rect x="138" y="66" width="40" height="52" fill={building} />
    <rect x="138" y="66" width="40" height="4" fill={neonCyan} opacity={neonOp * 0.7} />
    <g fill={glassLit} opacity={windowOp}>
      <rect x="144" y="74" width="12" height="12" />
      <rect x="160" y="74" width="12" height="12" fill={glass} />
    </g>
    <g stroke={buildingDark} stroke-width="0.55" fill="none" opacity="0.6">
      <line x1="150" y1="74" x2="150" y2="86" />
      <line x1="144" y1="80" x2="156" y2="80" />
      <line x1="166" y1="74" x2="166" y2="86" />
      <line x1="160" y1="80" x2="172" y2="80" />
    </g>
    <rect x="142" y="92" width="32" height="8" rx="0.4" fill={signInk} />
    <text
      x="158"
      y="98"
      text-anchor="middle"
      font-size="4.8"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={neonPink}
      opacity={neonOp}>sound kit</text
    >
    <rect x="144" y="102" width="10" height="16" fill="#14141C" />
    <rect x="158" y="102" width="16" height="16" fill={glass} opacity={night ? 0.65 : 0.5} />
    <!-- AC -->
    <rect x="166" y="88" width="9" height="4.5" rx="0.4" fill={concrete} />
    {#if winter}
      <rect x="138" y="64" width="40" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- ===== RIGHT-MID: convenience / konbini ===== -->
  <g>
    <rect x="260" y="64" width="46" height="54" fill={night ? '#1A2830' : '#2A3840'} />
    <rect x="260" y="64" width="46" height="6" fill="#E8E0C8" />
    <rect x="260" y="70" width="46" height="3" fill="#2A8A68" />
    <g fill={glassLit} opacity={shopWinOp}>
      <rect x="266" y="78" width="14" height="18" />
      <rect x="284" y="78" width="14" height="18" />
    </g>
    <rect x="268" y="100" width="12" height="18" fill="#1A2828" />
    <rect x="284" y="100" width="16" height="18" fill={glass} opacity={night ? 0.6 : 0.45} />
    <text
      x="283"
      y="68.5"
      text-anchor="middle"
      font-size="4"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#2A8A68">7-meow</text
    >
    {#if winter}
      <rect x="260" y="62" width="46" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- vertical pink sign (konbini edge) -->
  <g>
    <rect x="302" y="68" width="6" height="42" fill={neonPink} opacity={neonOp} />
    <g
      fill="#FFF8FC"
      font-size="3.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      text-anchor="middle"
    >
      <text x="305" y="78">開</text>
      <text x="305" y="86">店</text>
      <text x="305" y="94">中</text>
    </g>
  </g>

  <!-- ===== RIGHT: izakaya + lanterns ===== -->
  <g>
    <rect x="312" y="58" width="44" height="60" fill={buildingMid} />
    <rect x="312" y="58" width="44" height="4" fill={buildingDark} />
    <g fill={glass} opacity={night ? 0.55 : winter ? 0.65 : 0.4}>
      {#each [0, 1, 2] as row}
        <rect x="318" y={66 + row * 12} width="12" height="9" />
        <rect x="334" y={66 + row * 12} width="12" height="9" fill={glassLit} />
      {/each}
    </g>
    <g stroke={buildingDark} stroke-width="0.45" fill="none" opacity="0.55">
      {#each [0, 1, 2] as row}
        <line x1="324" y1={66 + row * 12} x2="324" y2={75 + row * 12} />
        <line x1="318" y1={70.5 + row * 12} x2="330" y2={70.5 + row * 12} />
        <line x1="340" y1={66 + row * 12} x2="340" y2={75 + row * 12} />
        <line x1="334" y1={70.5 + row * 12} x2="346" y2={70.5 + row * 12} />
      {/each}
    </g>
    <!-- noren curtain -->
    <g>
      <rect x="316" y="100" width="36" height="10" fill={neonRed} opacity={neonOp * 0.9} />
      <g fill={buildingDark} opacity="0.35">
        <rect x="324" y="100" width="1.2" height="10" />
        <rect x="333" y="100" width="1.2" height="10" />
        <rect x="342" y="100" width="1.2" height="10" />
      </g>
      <text
        x="334"
        y="107.5"
        text-anchor="middle"
        font-size="4.2"
        font-weight="700"
        font-family="ui-rounded, system-ui, sans-serif"
        fill={signCream}>izakitten</text
      >
    </g>
    <rect x="320" y="110" width="10" height="8" fill="#181820" />
    <rect x="334" y="110" width="16" height="8" fill={glass} opacity={night ? 0.55 : 0.4} />
    <!-- AC row -->
    <rect x="318" y="92" width="8" height="4" rx="0.3" fill={concrete} />
    <rect x="340" y="92" width="8" height="4" rx="0.3" fill={concrete} />
    {#if winter}
      <rect x="312" y="56" width="44" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- hanging lanterns right -->
  <g>
    <line x1="322" y1="74" x2="322" y2="82" stroke="#3A3030" stroke-width="0.7" />
    <ellipse cx="322" cy="86" rx="4" ry="5" fill={lantern} />
    <ellipse cx="322" cy="82" rx="3" ry="1" fill={lanternGlow} />
    <line x1="348" y1="76" x2="348" y2="84" stroke="#3A3030" stroke-width="0.7" />
    <ellipse cx="348" cy="88" rx="4.2" ry="5.2" fill={lantern} opacity="0.95" />
    <ellipse cx="348" cy="84" rx="3.2" ry="1.1" fill={lanternGlow} />
  </g>

  <!-- ===== FAR RIGHT: slim tower + hotel ===== -->
  <g>
    <rect x="360" y="50" width="36" height="68" fill={building} />
    <rect x="368" y="36" width="20" height="16" fill={buildingMid} />
    <rect x="376" y="22" width="3" height="16" fill={cable} />
    <rect x="377" y="12" width="1.2" height="12" fill={cable} />
    <g fill={glassLit} opacity={windowOp}>
      {#each [0, 1, 2, 3] as row}
        <rect x="366" y={56 + row * 12} width="10" height="8" />
        <rect x="380" y={56 + row * 12} width="10" height="8" fill={glass} />
      {/each}
    </g>
    <rect x="390" y="58" width="6" height="40" fill={neonCyan} opacity={neonOp} />
    <g
      fill="#F0FFFE"
      font-size="3.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      text-anchor="middle"
    >
      <text x="393" y="68">ホ</text>
      <text x="393" y="76">テ</text>
      <text x="393" y="84">ル</text>
    </g>
    {#if winter}
      <rect x="368" y="34" width="20" height="2.5" fill={g.accent} />
      <rect x="360" y="48" width="36" height="2.5" fill={g.accent} opacity="0.9" />
    {/if}
  </g>

  <!-- tiny far-right building -->
  <g>
    <rect x="398" y="68" width="22" height="50" fill={buildingDark} />
    <g fill={glass} opacity={night ? 0.55 : 0.45}>
      <rect x="402" y="74" width="6" height="7" />
      <rect x="412" y="74" width="6" height="7" fill={glassLit} />
      <rect x="402" y="88" width="6" height="7" fill={glassLit} />
      <rect x="412" y="88" width="6" height="7" />
      <rect x="402" y="102" width="6" height="7" />
      <rect x="412" y="102" width="6" height="7" fill={glassLit} />
    </g>
  </g>

  <!-- ===== STREET PROPS (edges / mid heights — corridor clear) ===== -->

  <!-- Vending machine (left curb) -->
  <g>
    <rect x="48" y="92" width="14" height="26" rx="1" fill={night ? '#1A3A58' : '#2A4A68'} />
    <rect x="49.5" y="94" width="11" height="8" fill={neonCyan} opacity={vendingLit} />
    <rect x="49.5" y="104" width="11" height="5" fill="#1A2838" />
    <g fill={neonPink} opacity={night ? 0.95 : 0.7}>
      <circle cx="52" cy="106.5" r="1" />
      <circle cx="55.5" cy="106.5" r="1" />
      <circle cx="59" cy="106.5" r="1" />
    </g>
    <rect x="50" y="111" width="10" height="4" fill="#3A5A78" />
    <text
      x="55"
      y="100"
      text-anchor="middle"
      font-size="3"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#E8F8FC">缶</text
    >
    {#if winter}
      <ellipse cx="55" cy="92" rx="7" ry="2" fill={g.accent} />
    {/if}
  </g>

  <!-- Subway stair entrance (right edge) -->
  <g>
    <rect x="358" y="98" width="30" height="20" fill={g.detail} />
    <rect x="360" y="88" width="26" height="12" fill="#1A6B4A" />
    <rect x="362" y="90" width="22" height="8" fill="#E8E8E0" opacity="0.92" />
    <text
      x="373"
      y="96"
      text-anchor="middle"
      font-size="4.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#1A6B4A">駅</text
    >
    <!-- stair steps descending -->
    <g fill={concrete} opacity="0.85">
      <rect x="364" y="104" width="18" height="2" />
      <rect x="366" y="108" width="14" height="2" />
      <rect x="368" y="112" width="10" height="2" />
    </g>
    <rect x="360" y="98" width="2.5" height="20" fill={cable} />
    <rect x="383.5" y="98" width="2.5" height="20" fill={cable} />
    {#if winter}
      <rect x="360" y="86" width="26" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- Parked bike (left of corridor) -->
  <g transform="translate(68, 112)" fill="none" stroke="#3A3A48" stroke-width="1.1">
    <circle cx="0" cy="4" r="4" />
    <circle cx="14" cy="4" r="4" />
    <path d="M0 4 L5 -4 L12 -4 L14 4" />
    <path d="M5 -4 L5 -8" />
    <path d="M3 -8 L8 -8" stroke-width="1.3" />
    <path d="M12 -4 L8 2" />
  </g>
  <circle cx="68" cy="116" r="1.2" fill="#4A4A58" />
  <circle cx="82" cy="116" r="1.2" fill="#4A4A58" />

  <!-- Utility pole + transformer -->
  <g fill={cable}>
    <rect x="182" y="48" width="2.5" height="70" />
    <rect x="176" y="52" width="14" height="3" />
    <rect x="178" y="58" width="10" height="6" rx="0.5" fill={concrete} />
  </g>

  <!-- Short street lamp (edge of corridor, high enough) -->
  <g>
    <rect x="248" y="78" width="2" height="40" fill={cable} />
    <path d="M244 78 Q249 72 254 78" fill="none" stroke={cable} stroke-width="1.5" />
    <circle
      cx="254"
      cy="80"
      r="2.8"
      fill={winter ? '#F0E8C8' : neonCyan}
      opacity={night ? 0.95 : 0.85}
    />
  </g>

  <!-- Soft neon / lantern glows (anim group: sk-glow) -->
  <g class="sk-glow">
    <ellipse cx="254" cy="80" rx="7" ry="6" fill={neonCyan} opacity={glowOp} />
    <ellipse cx="22" cy="104" rx="14" ry="5" fill={neonPink} opacity={glowOp * 0.85} />
    <ellipse cx="45.5" cy="90" rx="6" ry="14" fill={neonRed} opacity={glowOp * 0.75} />
    <ellipse cx="73" cy="77" rx="12" ry="5" fill={neonCyan} opacity={glowOp * 0.8} />
    <ellipse cx="92.5" cy="90" rx="5" ry="14" fill={neonPink} opacity={glowOp * 0.8} />
    <ellipse cx="158" cy="96" rx="12" ry="4.5" fill={neonPink} opacity={glowOp * 0.75} />
    <ellipse cx="305" cy="88" rx="5" ry="14" fill={neonPink} opacity={glowOp * 0.8} />
    <ellipse cx="393" cy="78" rx="5" ry="14" fill={neonCyan} opacity={glowOp * 0.8} />
    <ellipse cx="18" cy="82" rx="8" ry="7" fill={lanternGlow} opacity={lanternGlowOp} />
    <ellipse cx="34" cy="84" rx="7" ry="6" fill={lanternGlow} opacity={lanternGlowOp * 0.9} />
    <ellipse cx="322" cy="86" rx="7" ry="6" fill={lanternGlow} opacity={lanternGlowOp} />
    <ellipse cx="348" cy="88" rx="7.5" ry="6.5" fill={lanternGlow} opacity={lanternGlowOp * 0.95} />
    {#if night || golden}
      <ellipse cx="55" cy="98" rx="8" ry="6" fill={neonCyan} opacity={glowOp * 0.7} />
      <ellipse cx="334" cy="105" rx="14" ry="5" fill={neonRed} opacity={glowOp * 0.65} />
    {/if}
  </g>

  <!-- Trash bins edge -->
  <g fill={g.detail}>
    <rect x="308" y="106" width="8" height="12" rx="1" />
    <rect x="307" y="105" width="10" height="2" rx="0.4" fill={concrete} />
  </g>

  <!-- ===== SEASONAL DETAILS ===== -->
  {#if spring}
    <g>
      <ellipse cx="408" cy="74" rx="14" ry="9" fill="#E8A0B8" opacity="0.5" />
      <circle cx="402" cy="72" r="2.8" fill={blossom[2]} />
      <circle cx="410" cy="68" r="3.2" fill={blossom[0]} />
      <circle cx="414" cy="76" r="2.4" fill={blossom[1]} />
    </g>
    <g fill={blossom[1]} opacity="0.7">
      <ellipse cx="90" cy="116" rx="2" ry="1.1" />
      <ellipse cx="340" cy="115" rx="1.8" ry="1" />
    </g>
  {:else if winter}
    <!-- snow caps on ledges -->
    <ellipse cx="24" cy="58" rx="20" ry="2.5" fill={g.accent} />
    <ellipse cx="120" cy="54" rx="16" ry="2.2" fill={g.accent} />
    <ellipse cx="280" cy="64" rx="18" ry="2.2" fill={g.accent} />
    <ellipse cx="334" cy="58" rx="18" ry="2.2" fill={g.accent} />
    <ellipse cx="378" cy="36" rx="10" ry="1.8" fill={g.accent} />
    <rect x="0" y="117" width="420" height="2.5" fill={g.accent} opacity="0.85" />
  {:else if fall}
    <!-- warmer lantern emphasis + leaf flecks at edges -->
    <g fill="#C45A2A" opacity="0.75">
      <ellipse cx="10" cy="110" rx="2.2" ry="1.2" />
      <ellipse cx="40" cy="100" rx="1.8" ry="1" />
      <ellipse cx="400" cy="108" rx="2" ry="1.1" />
      <ellipse cx="410" cy="96" rx="1.6" ry="0.9" />
    </g>
    <rect x="316" y="100" width="36" height="10" fill="#C84828" opacity="0.15" />
  {:else if summer}
    <!-- festival flags / heat banners along wires -->
    <g>
      {#each [40, 70, 100, 160, 220, 300, 340, 380] as x, i}
        <path
          d={`M${x} ${34 + (i % 3)} L${x + 5} ${42 + (i % 3)} L${x} ${50 + (i % 3)} Z`}
          fill={i % 3 === 0 ? neonPink : i % 3 === 1 ? neonCyan : neonRed}
          opacity="0.8"
        />
      {/each}
    </g>
    <rect x="0" y="100" width="420" height="18" fill="#F0C090" opacity="0.08" />
  {/if}

  <!-- Ground: sidewalk + asphalt -->
  <rect x="0" y="118" width="420" height="7" fill={g.near} />
  <rect x="0" y="118" width="420" height="1.4" fill={g.detail} opacity="0.4" />
  <!-- sidewalk tiles -->
  <g stroke={g.detail} stroke-width="0.5" opacity="0.28">
    {#each [30, 70, 110, 150, 190, 230, 270, 310, 350, 390] as x}
      <line {x} y1="118" x2={x} y2="125" />
    {/each}
  </g>
  <rect x="0" y="125" width="420" height="2" fill={g.detail} opacity="0.5" />
  <rect x="0" y="127" width="420" height="13" fill={asphalt} />
  <!-- road dashes -->
  <g fill={winter ? g.accent : '#C8C0A8'} opacity={winter ? 0.55 : 0.4}>
    {#each [36, 96, 156, 216, 276, 336, 396] as x}
      <rect {x} y="132" width="16" height="1.4" rx="0.4" />
    {/each}
  </g>
  <!-- crosswalk hint near subway -->
  <g fill={winter ? g.accent : '#D8D8E0'} opacity={winter ? 0.65 : 0.5}>
    {#each [0, 1, 2, 3] as i}
      <rect x={348 + i * 8} y="127" width="5" height="12" />
    {/each}
  </g>
  {#if winter}
    <ellipse cx="50" cy="126" rx="20" ry="3" fill={g.accent} opacity="0.9" />
    <ellipse cx="300" cy="126" rx="22" ry="3" fill={g.accent} opacity="0.88" />
    <ellipse cx="390" cy="125.5" rx="18" ry="2.8" fill={g.accent} opacity="0.9" />
  {/if}
  {#if spring}
    <rect x="0" y="125" width="420" height="1.2" fill={blossom[0]} opacity="0.15" />
  {/if}

  <!-- Wet-street reflection hints under neon (above asphalt) -->
  {#if night || golden}
    <g opacity={reflectOp}>
      <ellipse cx="22" cy="126" rx="16" ry="1.6" fill={neonPink} />
      <ellipse cx="73" cy="126.5" rx="14" ry="1.4" fill={neonCyan} />
      <ellipse cx="92" cy="127" rx="8" ry="1.3" fill={neonPink} />
      <ellipse cx="158" cy="126" rx="12" ry="1.4" fill={neonPink} />
      <ellipse cx="305" cy="126.5" rx="7" ry="1.3" fill={neonPink} />
      <ellipse cx="334" cy="126" rx="14" ry="1.5" fill={neonRed} />
      <ellipse cx="393" cy="126.5" rx="8" ry="1.3" fill={neonCyan} />
      <ellipse cx="55" cy="126" rx="9" ry="1.4" fill={neonCyan} />
    </g>
  {/if}
</g>
