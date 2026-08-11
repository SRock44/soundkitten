<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Time, Weather } from '../../types'

  let { weather, time = 'day' }: { weather: Weather; time?: Time } = $props()

  const night = $derived(time === 'night')
  const golden = $derived(time === 'sunrise' || time === 'sunset')

  const g = $derived(groundFor('city', weather))
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  const glassDay = $derived(
    winter ? '#C8D8EC' : fall ? '#A8B8C8' : spring ? '#7AA8C0' : '#6A98B0',
  )
  const glassLitDay = $derived(winter ? '#E8F0FA' : fall ? '#D0C8A8' : '#9EC8D8')
  const brickDay = $derived(summer ? '#8B5A42' : fall ? '#7A4E3A' : '#6A5248')
  const brickDarkDay = $derived(summer ? '#6E4634' : '#524038')
  const concreteDay = $derived(winter ? '#B0BCC8' : '#8A949E')
  const steelDay = $derived(winter ? '#9AA8B4' : '#5A6874')

  const brick = $derived(night ? '#4A342C' : golden ? '#7A4E38' : brickDay)
  const brickDark = $derived(night ? '#34241C' : golden ? '#5A3830' : brickDarkDay)
  const concrete = $derived(night ? '#6A747E' : concreteDay)
  const steel = $derived(night ? '#3A4854' : golden ? '#4A5864' : steelDay)
  const farFill = $derived(night ? '#2A3440' : g.far)

  const glass = $derived(night ? '#1A2430' : golden ? '#8A7880' : glassDay)
  const glassLit = $derived(
    night ? '#FFE8A8' : golden ? '#FFD090' : glassLitDay,
  )
  const glassLitAlt = $derived(
    night ? '#FFF6D0' : golden ? '#FFD090' : glassLitDay,
  )
  const winLitOp = $derived(night ? 0.95 : golden ? 0.72 : winter ? 0.85 : 0.55)
  const winDimOp = $derived(night ? 0.9 : golden ? 0.55 : winter ? 0.65 : 0.4)

  const bulb = $derived(night ? '#FFF4C8' : golden ? '#FFE8B0' : winter ? '#F0E8C8' : '#E8D090')
  const lampGlow = $derived(night ? '#FFE8A8' : golden ? '#FFD090' : g.accent)
  const lampGlowOp = $derived(night ? 0.6 : golden ? 0.42 : 0.32)
  const lampRx = $derived(night ? 20 : golden ? 11 : 5)
  const lampRy = $derived(night ? 15 : golden ? 8 : 4)
  const poolOp = $derived(night ? 0.35 : golden ? 0.18 : 0)

  const signInk = '#1A1E24'
  const signCream = $derived(night ? '#FFF6E0' : '#F2E8D4')
  const neonGreen = $derived(night ? '#90F0B8' : '#6AD090')
  const neonTeal = $derived(night ? '#A0E8D8' : '#7EC8B8')
  const neonGold = $derived(night ? '#FFE070' : '#F0D060')
  const hotelNeon = $derived(night ? '#FFE8A8' : '#E8D090')
  const foliageWarm = ['#C45A2A', '#D4783A', '#E07030', '#A84828'] as const
  const blossom = ['#F0A0C8', '#E888B0', '#D870A0'] as const
</script>

<g class="city" aria-hidden="true">
  <rect class="sk-mist" x="0" y="48" width="420" height="72" fill={farFill} opacity={night ? 0.35 : 0.2} />

  <!-- ===== FAR SKYLINE ===== -->
  <g fill={farFill}>
    <rect x="0" y="58" width="22" height="60" />
    <rect x="18" y="42" width="28" height="76" />
    <rect x="42" y="52" width="18" height="66" />
    <rect x="56" y="36" width="32" height="82" />
    <rect x="68" y="22" width="5" height="16" />
    <rect x="70" y="12" width="1.5" height="12" />
    <rect x="86" y="48" width="20" height="70" />
    <rect x="104" y="30" width="36" height="88" />
    <rect x="114" y="16" width="8" height="16" />
    <rect x="118" y="6" width="2" height="12" />
    <rect x="138" y="44" width="24" height="74" />
    <rect x="158" y="34" width="40" height="84" />
    <rect x="170" y="20" width="14" height="16" />
    <rect x="174" y="10" width="2" height="12" />
    <rect x="196" y="50" width="22" height="68" />
    <rect x="214" y="38" width="30" height="80" />
    <rect x="242" y="46" width="26" height="72" />
    <rect x="264" y="28" width="42" height="90" />
    <rect x="278" y="14" width="10" height="16" />
    <rect x="282" y="4" width="2" height="12" />
    <rect x="304" y="42" width="28" height="76" />
    <rect x="328" y="34" width="36" height="84" />
    <rect x="340" y="18" width="8" height="18" />
    <rect x="362" y="48" width="24" height="70" />
    <rect x="382" y="36" width="38" height="82" />
  </g>

  <!-- Scattered lit windows on far skyline -->
  {#if night}
    <g fill="#FFE8A8" opacity="0.85">
      <rect x="22" y="50" width="2.5" height="2" />
      <rect x="28" y="58" width="2" height="2" />
      <rect x="60" y="48" width="2.5" height="2" />
      <rect x="72" y="54" width="2" height="2" />
      <rect x="110" y="42" width="2.5" height="2" />
      <rect x="122" y="52" width="2" height="2" />
      <rect x="164" y="46" width="2.5" height="2" />
      <rect x="178" y="56" width="2" height="2" />
      <rect x="220" y="48" width="2.5" height="2" />
      <rect x="270" y="40" width="2" height="2" />
      <rect x="286" y="50" width="2.5" height="2" />
      <rect x="332" y="46" width="2" height="2" />
      <rect x="350" y="54" width="2.5" height="2" />
      <rect x="390" y="48" width="2" height="2" />
    </g>
    <g fill="#FFF6D0" opacity="0.7">
      <rect x="48" y="62" width="2" height="2" />
      <rect x="148" y="54" width="2" height="2" />
      <rect x="250" y="56" width="2" height="2" />
      <rect x="310" y="52" width="2" height="2" />
      <rect x="370" y="58" width="2" height="2" />
    </g>
  {:else if golden}
    <g fill="#FFD090" opacity="0.55">
      <rect x="60" y="48" width="2.5" height="2" />
      <rect x="164" y="46" width="2.5" height="2" />
      <rect x="270" y="40" width="2" height="2" />
      <rect x="350" y="54" width="2.5" height="2" />
    </g>
  {/if}

  <!-- Distant billboard -->
  <g>
    <rect x="148" y="28" width="36" height="14" fill="#2A3340" />
    <rect x="150" y="30" width="32" height="10" fill={night ? '#FFE8A8' : '#E8D090'} />
    <text
      x="166"
      y="38"
      text-anchor="middle"
      font-size="5.5"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#3A2A18">purr FM</text
    >
    <rect x="164" y="42" width="3" height="12" fill={steel} />
  </g>

  <g fill={g.detail} opacity="0.85">
    <rect x="228" y="30" width="12" height="8" />
    <rect x="231" y="38" width="6" height="6" />
    <ellipse cx="234" cy="30" rx="7" ry="2.5" fill={g.accent} opacity="0.7" />
    <rect x="350" y="26" width="10" height="7" />
    <rect x="352.5" y="33" width="5" height="5" />
    <ellipse cx="355" cy="26" rx="6" ry="2.2" fill={g.accent} opacity="0.7" />
  </g>

  <!-- ===== STOREFRONTS (street level) ===== -->

  <!-- COFFEE SHOP (left brownstone) -->
  <g>
    <rect x="0" y="70" width="44" height="48" fill={brick} />
    <rect x="0" y="70" width="44" height="4" fill={brickDark} />
    <rect x="-1" y="68" width="46" height="3" fill={concrete} />
    <!-- upper windows with muntins -->
    <g>
      <rect x="6" y="76" width="14" height="12" fill={glassLit} opacity={winLitOp} />
      <rect
        x="24"
        y="76"
        width="14"
        height="12"
        fill={night ? '#1A2430' : glass}
        opacity={night ? 1 : winLitOp}
      />
    </g>
    <g stroke={brickDark} stroke-width="0.7" fill="none" opacity="0.7">
      <line x1="13" y1="76" x2="13" y2="88" />
      <line x1="6" y1="82" x2="20" y2="82" />
      <line x1="31" y1="76" x2="31" y2="88" />
      <line x1="24" y1="82" x2="38" y2="82" />
    </g>
    <!-- striped awning -->
    <path d="M2 94 L22 86 L42 94 L42 98 L2 98 Z" fill="#3A5A78" />
    <g fill="#E8E0D0" opacity="0.85">
      <rect x="6" y="92" width="3" height="6" transform="skewX(-18)" />
      <rect x="14" y="90" width="3" height="8" transform="skewX(-18)" />
      <rect x="22" y="88" width="3" height="10" transform="skewX(-18)" />
      <rect x="30" y="90" width="3" height="8" transform="skewX(-18)" />
    </g>
    <path d="M2 94 L22 86 L42 94" fill="none" stroke="#2A4058" stroke-width="1" />
    <!-- sign board -->
    <rect x="6" y="98" width="32" height="8" rx="0.5" fill="#2A1E14" />
    <text
      x="22"
      y="104"
      text-anchor="middle"
      font-size="5.2"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}>meowcha</text
    >
    <!-- door + window -->
    <rect x="8" y="106" width="12" height="12" fill="#3A2A20" />
    <rect x="10" y="108" width="8" height="6" fill={glassLitAlt} opacity={night ? 0.9 : 0.5} />
    <rect
      x="24"
      y="106"
      width="16"
      height="12"
      fill={night ? glassLit : glass}
      opacity={night ? 0.88 : 0.55}
    />
    <line x1="32" y1="106" x2="32" y2="118" stroke={brickDark} stroke-width="0.8" />
    {#if winter}
      <rect x="-1" y="66" width="46" height="3" fill={g.accent} />
    {/if}
    {#if summer}
      <rect x="34" y="82" width="7" height="5" fill={steel} rx="0.5" />
    {/if}
  </g>

  <!-- Scaffolding + BOOK NOOK peek -->
  <g>
    <g fill={steel} opacity="0.75">
      <rect x="46" y="78" width="2" height="40" />
      <rect x="70" y="78" width="2" height="40" />
      <rect x="46" y="78" width="26" height="2" />
      <rect x="46" y="94" width="26" height="1.5" />
      <rect x="46" y="108" width="26" height="1.5" />
    </g>
    <rect x="48" y="96" width="22" height="22" fill={brickDark} />
    <rect x="50" y="98" width="18" height="6" fill={night ? '#0A2038' : '#1A3048'} />
    <text
      x="59"
      y="103"
      text-anchor="middle"
      font-size="3.8"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={night ? '#E8F4FF' : '#D0E0F0'}>catlog</text
    >
    <rect x="52" y="106" width="7" height="10" fill="#4A3020" />
    <rect
      x="61"
      y="106"
      width="7"
      height="10"
      fill={night ? '#1A2430' : glass}
      opacity={night ? 1 : 0.5}
    />
  </g>

  <!-- JAIL / precinct -->
  <g>
    <rect x="76" y="58" width="40" height="60" fill={night ? '#3A4550' : '#5A6570'} />
    <rect x="76" y="58" width="40" height="6" fill="#3A4450" />
    <rect x="78" y="50" width="36" height="10" fill="#4A5460" />
    <rect x="88" y="42" width="16" height="10" fill="#3A4450" />
    <rect x="94" y="34" width="2.5" height="10" fill={steel} />
    <!-- barred windows -->
    <g>
      <rect x="80" y="68" width="14" height="14" fill={night ? '#1A2430' : glass} opacity={winDimOp} />
      <rect x="98" y="68" width="14" height="14" fill={glassLit} opacity={winLitOp} />
      <rect x="80" y="88" width="14" height="14" fill={glassLitAlt} opacity={winLitOp} />
      <rect x="98" y="88" width="14" height="14" fill={night ? '#1A2430' : glass} opacity={winDimOp} />
    </g>
    <g stroke="#2A3038" stroke-width="0.9" opacity="0.85">
      {#each [83, 87, 91] as x}
        <line {x} y1="68" x2={x} y2="82" />
        <line {x} y1="88" x2={x} y2="102" />
      {/each}
      {#each [101, 105, 109] as x}
        <line {x} y1="68" x2={x} y2="82" />
        <line {x} y1="88" x2={x} y2="102" />
      {/each}
    </g>
    <!-- JAIL sign -->
    <rect x="82" y="104" width="28" height="9" fill="#1A2028" />
    <text
      x="96"
      y="111"
      text-anchor="middle"
      font-size="6"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={night ? '#FFF4D8' : '#E8E0C8'}
      letter-spacing="0.4">pound</text
    >
    {#if winter}
      <rect x="78" y="48" width="36" height="3" fill={g.accent} />
    {/if}
  </g>

  <!-- PIZZA place (short mid) -->
  <g>
    <rect x="120" y="74" width="48" height="44" fill={brick} />
    <rect x="120" y="74" width="48" height="3" fill={concrete} />
    <path d="M120 74 L144 62 L168 74 Z" fill={brickDark} />
    <!-- red/white awning -->
    <path d="M122 94 L144 88 L166 94 L166 99 L122 99 Z" fill="#B03828" />
    <g fill="#F0E8E0">
      <rect x="126" y="92" width="3.5" height="7" />
      <rect x="134" y="90" width="3.5" height="9" />
      <rect x="142" y="89" width="3.5" height="10" />
      <rect x="150" y="90" width="3.5" height="9" />
      <rect x="158" y="92" width="3.5" height="7" />
    </g>
    <rect x="128" y="100" width="32" height="7" fill="#F0E0C8" />
    <text
      x="144"
      y="105.5"
      text-anchor="middle"
      font-size="5"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#B03828">purrza</text
    >
    <rect x="126" y="108" width="14" height="10" fill="#3A2820" />
    <rect x="128" y="110" width="10" height="5" fill={glassLit} opacity={night ? 0.92 : 0.45} />
    <rect
      x="144"
      y="108"
      width="20"
      height="10"
      fill={night ? glassLitAlt : glass}
      opacity={night ? 0.9 : 0.5}
    />
    <!-- open neon-ish OPEN -->
    <rect x="148" y="80" width="16" height="5" fill="#1A4030" />
    <text
      x="156"
      y="84"
      text-anchor="middle"
      font-size="3.5"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={neonGreen}>purr</text
    >
    {#if winter}
      <path d="M120 74 L144 62 L168 74 L168 77 L144 66 L120 77 Z" fill={g.accent} />
    {/if}
  </g>

  <!-- Elevated track -->
  <g opacity="0.7">
    <path d="M168 66 Q210 50 252 66" fill="none" stroke={steel} stroke-width="3" />
    <path d="M168 70 Q210 56 252 70" fill="none" stroke={g.detail} stroke-width="1.4" opacity="0.6" />
    <rect x="178" y="66" width="3" height="52" fill={steel} opacity="0.5" />
    <rect x="236" y="66" width="3" height="52" fill={steel} opacity="0.5" />
    <rect x="176" y="64" width="7" height="3" fill={g.detail} />
    <rect x="234" y="64" width="7" height="3" fill={g.detail} />
  </g>

  <!-- Street sign pole (mid corridor, short) -->
  <g>
    <rect x="208" y="100" width="2" height="18" fill={steel} />
    <rect x="200" y="98" width="18" height="7" rx="0.5" fill="#2A6A48" />
    <text
      x="209"
      y="103.5"
      text-anchor="middle"
      font-size="4"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#F0F4F0">kit st</text
    >
  </g>

  <!-- BARBER shop -->
  <g>
    <rect x="256" y="70" width="40" height="48" fill={night ? '#4A5A68' : '#6A7A88'} />
    <rect x="256" y="70" width="40" height="4" fill="#4A5A68" />
    <rect x="260" y="58" width="32" height="14" fill={farFill} />
    <!-- barber pole -->
    <rect x="258" y="88" width="4" height="16" fill="#F0F0F0" rx="1" />
    <g>
      <rect x="258" y="90" width="4" height="3" fill="#C04040" />
      <rect x="258" y="96" width="4" height="3" fill="#3A63A0" />
      <rect x="258" y="102" width="4" height="3" fill="#C04040" />
    </g>
    <!-- awning -->
    <path d="M264 94 L276 88 L292 94 L292 98 L264 98 Z" fill="#3F63A1" />
    <g fill="#E8E8F0" opacity="0.8">
      <rect x="268" y="92" width="2.5" height="6" />
      <rect x="274" y="90" width="2.5" height="8" />
      <rect x="280" y="90" width="2.5" height="8" />
      <rect x="286" y="92" width="2.5" height="6" />
    </g>
    <rect x="266" y="99" width="26" height="7" fill="#1A2430" />
    <text
      x="279"
      y="104.5"
      text-anchor="middle"
      font-size="4.5"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={night ? '#FFF4E0' : '#E8E0D0'}>claw cut</text
    >
    <rect x="268" y="107" width="10" height="11" fill="#2A3038" />
    <rect x="280" y="107" width="12" height="11" fill={glassLit} opacity={night ? 0.92 : 0.5} />
    <g>
      <rect x="264" y="76" width="12" height="10" fill={glassLitAlt} opacity={winLitOp} />
      <rect
        x="280"
        y="76"
        width="12"
        height="10"
        fill={night ? '#1A2430' : glassLit}
        opacity={night ? 1 : winDimOp}
      />
    </g>
    {#if winter}
      <rect x="260" y="56" width="32" height="3" fill={g.accent} />
      <rect x="256" y="68" width="40" height="3" fill={g.accent} opacity="0.9" />
    {/if}
    {#if summer}
      <rect x="288" y="82" width="6" height="5" fill={steel} />
    {/if}
  </g>

  <!-- DINER -->
  <g>
    <rect x="300" y="72" width="36" height="46" fill={night ? '#8A7858' : '#C8B090'} />
    <rect x="300" y="72" width="36" height="3" fill="#A89070" />
    <rect x="298" y="70" width="40" height="3" fill={concrete} />
    <!-- chrome diner roof curve -->
    <ellipse cx="318" cy="72" rx="18" ry="5" fill={night ? '#8A949E' : '#D0D8E0'} />
    <path d="M302 94 L318 88 L334 94 L334 99 L302 99 Z" fill="#C84838" />
    <g fill="#F8F0E0">
      <rect x="306" y="92" width="3" height="7" />
      <rect x="314" y="90" width="3" height="9" />
      <rect x="322" y="90" width="3" height="9" />
    </g>
    <rect x="306" y="100" width="24" height="7" fill="#1A2030" />
    <text
      x="318"
      y="105.5"
      text-anchor="middle"
      font-size="4.8"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={neonGold}>9 lives</text
    >
    <rect x="306" y="108" width="10" height="10" fill="#3A3028" />
    <rect
      x="318"
      y="108"
      width="14"
      height="10"
      fill={night ? glassLit : glass}
      opacity={night ? 0.9 : 0.55}
    />
    <g>
      <rect x="306" y="78" width="10" height="10" fill={glassLit} opacity={winLitOp} />
      <rect x="320" y="78" width="10" height="10" fill={glassLitAlt} opacity={winLitOp} />
    </g>
    {#if winter}
      <rect x="298" y="68" width="40" height="3" fill={g.accent} />
    {/if}
  </g>

  <!-- MUSIC shop + guitar -->
  <g>
    <rect x="340" y="70" width="44" height="48" fill={brickDark} />
    <rect x="340" y="70" width="44" height="3" fill={brick} />
    <rect x="338" y="68" width="48" height="3" fill={concrete} />
    <!-- teal awning -->
    <path d="M342 94 L362 87 L380 94 L380 99 L342 99 Z" fill="#2A7A6A" />
    <g fill="#E0F0E8" opacity="0.75">
      <rect x="348" y="91" width="3" height="8" />
      <rect x="356" y="89" width="3" height="10" />
      <rect x="364" y="89" width="3" height="10" />
      <rect x="372" y="91" width="3" height="8" />
    </g>
    <rect x="348" y="100" width="28" height="7" fill="#142028" />
    <text
      x="362"
      y="105.5"
      text-anchor="middle"
      font-size="4.5"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={neonTeal}>soundkit</text
    >
    <rect x="346" y="108" width="12" height="10" fill="#2A2420" />
    <rect x="360" y="108" width="18" height="10" fill={glassLit} opacity={night ? 0.9 : 0.45} />
    <g>
      <rect x="346" y="76" width="12" height="12" fill={glassLitAlt} opacity={winLitOp} />
      <rect
        x="364"
        y="76"
        width="14"
        height="12"
        fill={night ? '#1A2430' : glass}
        opacity={night ? 1 : winDimOp}
      />
    </g>
    <!-- guitar glyph in window -->
    <ellipse cx="352" cy="82" rx="2" ry="3.5" fill="none" stroke="#C8A060" stroke-width="0.8" />
    <line x1="352" y1="78.5" x2="352" y2="74" stroke="#C8A060" stroke-width="0.8" />
    {#if winter}
      <rect x="338" y="66" width="48" height="3" fill={g.accent} />
    {/if}
  </g>

  <!-- Far-right slim + HOTEL letters -->
  <g>
    <rect x="388" y="62" width="32" height="56" fill={farFill} />
    <rect x="392" y="50" width="24" height="14" fill={night ? '#3A4450' : g.mid} />
    <rect x="402" y="38" width="2" height="14" fill={steel} />
    <g>
      <rect x="392" y="68" width="10" height="12" fill={glassLit} opacity={winLitOp} />
      <rect
        x="406"
        y="68"
        width="10"
        height="12"
        fill={night ? '#1A2430' : glass}
        opacity={night ? 1 : winDimOp}
      />
      <rect
        x="392"
        y="90"
        width="10"
        height="12"
        fill={night ? '#1A2430' : glass}
        opacity={night ? 1 : winDimOp}
      />
      <rect x="406" y="90" width="10" height="12" fill={glassLitAlt} opacity={winLitOp} />
    </g>
    <!-- vertical HOTEL -->
    <rect x="414" y="64" width="6" height="34" fill="#1A2430" />
    <g
      fill={hotelNeon}
      font-size="3.4"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      text-anchor="middle"
    >
      <text x="417" y="70">N</text>
      <text x="417" y="75">A</text>
      <text x="417" y="80">P</text>
      <text x="417" y="86">I</text>
      <text x="417" y="91">N</text>
      <text x="417" y="96">N</text>
    </g>
    {#if winter}
      <rect x="392" y="48" width="24" height="2.5" fill={g.accent} />
    {/if}
  </g>

  <!-- ===== STREET LAMPS ===== -->
  <g fill={steel}>
    <!-- edge lamp left -->
    <rect x="16" y="88" width="2.5" height="30" />
    <path d="M12 88 Q17.5 82 23 88" fill="none" stroke={steel} stroke-width="2" />
    <circle cx="23" cy="90" r="2.5" fill={bulb} opacity="0.95" />
    <!-- mid-left lamp -->
    <rect x="54" y="88" width="2.5" height="30" />
    <path d="M50 88 Q55.5 82 61 88" fill="none" stroke={steel} stroke-width="2" />
    <circle cx="61" cy="90" r="2.5" fill={bulb} opacity="0.95" />
    <!-- mid-right lamp -->
    <rect x="248" y="88" width="2.5" height="30" />
    <path d="M244 88 Q249.5 82 255 88" fill="none" stroke={steel} stroke-width="2" />
    <circle cx="255" cy="90" r="2.5" fill={bulb} opacity="0.9" />
    <!-- edge lamp right -->
    <rect x="400" y="88" width="2.5" height="30" />
    <path d="M396 88 Q401.5 82 407 88" fill="none" stroke={steel} stroke-width="2" />
    <circle cx="407" cy="90" r="2.5" fill={bulb} opacity="0.95" />
  </g>
  <g class="sk-glow">
    <ellipse cx="23" cy="96" rx={lampRx} ry={lampRy} fill={lampGlow} opacity={lampGlowOp} />
    <ellipse
      cx="61"
      cy="96"
      rx={lampRx}
      ry={lampRy}
      fill={lampGlow}
      opacity={night ? 0.68 : lampGlowOp}
    />
    <ellipse
      cx="255"
      cy="96"
      rx={lampRx}
      ry={lampRy}
      fill={lampGlow}
      opacity={night ? 0.62 : lampGlowOp * 0.9}
    />
    <ellipse
      cx="407"
      cy="96"
      rx={lampRx}
      ry={lampRy}
      fill={lampGlow}
      opacity={night ? 0.55 : lampGlowOp * 0.85}
    />
    {#if night}
      <ellipse cx="23" cy="102" rx="12" ry="10" fill="#FFF4C8" opacity="0.45" />
      <ellipse cx="61" cy="102" rx="14" ry="11" fill="#FFF4C8" opacity="0.5" />
      <ellipse cx="255" cy="102" rx="14" ry="11" fill="#FFF4C8" opacity="0.48" />
      <ellipse cx="407" cy="102" rx="12" ry="10" fill="#FFF4C8" opacity="0.42" />
    {/if}
  </g>

  <!-- Fire hydrant -->
  <g fill="#B03828">
    <rect x="40" y="108" width="7" height="10" rx="1" />
    <rect x="38" y="106" width="11" height="3" rx="1" fill="#8A2A1E" />
    <rect x="42" y="104" width="3" height="3" fill="#8A2A1E" />
    <circle cx="38.5" cy="111" r="1.8" />
    <circle cx="48.5" cy="111" r="1.8" />
    {#if winter}
      <ellipse cx="43.5" cy="105" rx="5" ry="2" fill={g.accent} />
    {/if}
  </g>

  <!-- Trash + mailbox -->
  <g fill={steel}>
    <rect x="112" y="104" width="10" height="14" rx="1" fill={g.detail} />
    <rect x="111" y="103" width="12" height="2.5" rx="0.5" />
  </g>
  <g>
    <rect x="190" y="102" width="3" height="16" fill={steel} />
    <rect x="186" y="100" width="11" height="9" rx="1" fill="#3A5A88" />
    <rect x="188" y="102" width="7" height="5" fill="#2A4060" />
    <circle cx="194.5" cy="104.5" r="0.8" fill="#C8D0D8" />
  </g>

  <!-- Newsstand -->
  <g>
    <rect x="218" y="96" width="24" height="22" fill={g.detail} />
    <rect x="216" y="94" width="28" height="4" fill="#8A3A28" />
    <rect x="220" y="100" width="8" height="10" fill={glassLit} opacity={night ? 0.85 : 0.45} />
    <rect x="230" y="100" width="8" height="10" fill="#C8A060" opacity="0.65" />
    <text
      x="230"
      y="116"
      text-anchor="middle"
      font-size="3.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signCream}
      opacity="0.9">mews</text
    >
    {#if winter}
      <rect x="216" y="92" width="28" height="3" fill={g.accent} />
    {/if}
  </g>

  <!-- Subway entrance with label -->
  <g>
    <rect x="360" y="100" width="28" height="18" fill={g.detail} />
    <rect x="362" y="90" width="24" height="12" fill="#1A6B4A" />
    <rect x="364" y="92" width="20" height="8" fill="#E8E0C8" opacity="0.9" />
    <text
      x="374"
      y="98"
      text-anchor="middle"
      font-size="4.5"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#1A6B4A">meowtro</text
    >
    <rect x="366" y="106" width="16" height="2" fill={steel} opacity="0.55" />
    <rect x="370" y="112" width="8" height="2" fill={steel} opacity="0.4" />
    <circle cx="362" cy="96" r="2" fill={night ? '#FFF4C8' : '#C8E0D0'} opacity="0.9" />
    <circle cx="386" cy="96" r="2" fill={night ? '#FFF4C8' : '#C8E0D0'} opacity="0.9" />
    {#if winter}
      <rect x="362" y="88" width="24" height="3" fill={g.accent} />
    {/if}
  </g>

  <!-- Parking sign -->
  <g>
    <rect x="330" y="96" width="2" height="22" fill={steel} />
    <rect x="326" y="94" width="10" height="10" rx="1" fill="#3A63A1" />
    <text
      x="331"
      y="101.5"
      text-anchor="middle"
      font-size="6"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#F0F4F8">P</text
    >
  </g>

  <!-- ===== SEASONAL PLANTERS ===== -->
  {#if fall}
    <g>
      <rect x="8" y="110" width="14" height="8" fill={concrete} />
      <rect x="13" y="92" width="3" height="20" fill="#5A4030" />
      <g class="sk-sway">
        <ellipse cx="14.5" cy="88" rx="14" ry="10" fill={foliageWarm[0]} />
        <ellipse cx="10" cy="92" rx="8" ry="6" fill={foliageWarm[1]} />
        <ellipse cx="20" cy="91" rx="7" ry="5" fill={foliageWarm[2]} />
      </g>
    </g>
    <g>
      <rect x="398" y="110" width="14" height="8" fill={concrete} />
      <rect x="403" y="94" width="3" height="18" fill="#5A4030" />
      <ellipse cx="404.5" cy="90" rx="12" ry="9" fill={foliageWarm[3]} />
      <ellipse cx="400" cy="94" rx="7" ry="5" fill={foliageWarm[1]} />
    </g>
    <g>
      <ellipse cx="70" cy="117" rx="2.2" ry="1.2" fill={foliageWarm[1]} />
      <ellipse cx="350" cy="116" rx="2.1" ry="1.2" fill={foliageWarm[3]} />
    </g>
  {:else if spring}
    <g>
      <rect x="8" y="110" width="14" height="8" fill={concrete} />
      <rect x="13" y="94" width="3" height="18" fill="#5A4030" />
      <g class="sk-sway">
        <ellipse cx="14.5" cy="90" rx="13" ry="9" fill="#6AAA62" />
        <circle cx="8" cy="88" r="2.2" fill={blossom[0]} />
        <circle cx="14" cy="84" r="2.5" fill={blossom[1]} />
        <circle cx="21" cy="89" r="2" fill={blossom[2]} />
      </g>
    </g>
    <g>
      <rect x="398" y="110" width="14" height="8" fill={concrete} />
      <rect x="403" y="96" width="3" height="16" fill="#5A4030" />
      <ellipse cx="404.5" cy="92" rx="11" ry="8" fill="#5A9A58" />
      <circle cx="399" cy="90" r="2" fill={blossom[2]} />
      <circle cx="405" cy="86" r="2.3" fill={blossom[0]} />
    </g>
    <ellipse cx="200" cy="128" rx="28" ry="3.5" fill={glassDay} opacity="0.25" />
  {:else if summer}
    <g>
      <rect x="8" y="110" width="14" height="8" fill={concrete} />
      <rect x="13" y="94" width="3" height="18" fill="#5A4030" />
      <g class="sk-sway">
        <ellipse cx="14.5" cy="90" rx="13" ry="10" fill="#3A8A3A" />
        <ellipse cx="10" cy="94" rx="7" ry="5" fill="#4A9A48" />
      </g>
    </g>
    <g>
      <rect x="398" y="110" width="14" height="8" fill={concrete} />
      <rect x="403" y="96" width="3" height="16" fill="#5A4030" />
      <ellipse cx="404.5" cy="92" rx="11" ry="8" fill="#348034" />
    </g>
  {:else}
    <g>
      <rect x="8" y="110" width="14" height="8" fill={concrete} />
      <ellipse cx="15" cy="110" rx="8" ry="3" fill={g.accent} />
      <rect x="13" y="92" width="3" height="20" fill="#5A4030" />
      <path d="M14.5 92 L8 84 M14.5 92 L20 86 M14.5 96 L10 90" stroke="#4A3830" stroke-width="1.2" fill="none" />
    </g>
    <g>
      <rect x="398" y="110" width="14" height="8" fill={concrete} />
      <ellipse cx="405" cy="110" rx="8" ry="3" fill={g.accent} />
      <rect x="403" y="94" width="3" height="18" fill="#5A4030" />
      <path d="M404.5 94 L399 87 M404.5 94 L410 88" stroke="#4A3830" stroke-width="1.2" fill="none" />
    </g>
  {/if}

  <!-- Ground -->
  <rect x="0" y="118" width="420" height="8" fill={g.near} />
  <rect x="0" y="118" width="420" height="1.5" fill={g.detail} opacity="0.35" />
  {#if night || golden}
    <g fill={lampGlow} opacity={poolOp}>
      <ellipse cx="18" cy="120" rx="16" ry="3.5" />
      <ellipse cx="61" cy="120" rx="18" ry="4" />
      <ellipse cx="255" cy="120" rx="18" ry="4" />
      <ellipse cx="402" cy="120" rx="16" ry="3.5" />
    </g>
  {/if}
  <g stroke={g.detail} stroke-width="0.6" opacity="0.3">
    {#each [50, 120, 190, 260, 330, 400] as x}
      <line {x} y1="118" x2={x} y2="126" />
    {/each}
  </g>
  <rect x="0" y="125" width="420" height="2.5" fill={g.detail} opacity="0.55" />
  <rect x="0" y="127" width="420" height="13" fill={g.path} />
  <g fill={g.accent} opacity="0.35">
    {#each [40, 100, 160, 220, 280, 340] as x}
      <rect {x} y="132" width="18" height="1.5" rx="0.5" />
    {/each}
  </g>
  <g fill={winter ? g.accent : '#D8DCE0'} opacity={winter ? 0.7 : 0.55}>
    {#each [0, 1, 2, 3] as i}
      <rect x={186 + i * 10} y="127" width="6" height="12" />
    {/each}
  </g>
  {#if winter}
    <ellipse cx="40" cy="126" rx="22" ry="3.5" fill={g.accent} opacity="0.95" />
    <ellipse cx="280" cy="126" rx="24" ry="3.5" fill={g.accent} opacity="0.92" />
    <ellipse cx="380" cy="125.5" rx="20" ry="3" fill={g.accent} opacity="0.9" />
    <rect x="0" y="117" width="420" height="3" fill={g.accent} opacity="0.85" />
  {/if}
  {#if spring}
    <rect x="0" y="125" width="420" height="1.5" fill={glassDay} opacity="0.2" />
  {/if}
</g>
