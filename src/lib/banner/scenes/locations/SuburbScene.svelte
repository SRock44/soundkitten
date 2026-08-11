<script lang="ts">
  import { groundFor } from '../../palettes'
  import type { Weather } from '../../types'

  let { weather }: { weather: Weather } = $props()

  const g = $derived(groundFor('suburb', weather))
  const fall = $derived(weather === 'fall')
  const winter = $derived(weather === 'winter')
  const spring = $derived(weather === 'spring')
  const summer = $derived(weather === 'summer')

  const houseA = $derived(
    winter
      ? { wall: '#D0D4D8', trim: '#A0A8B0', roof: '#6A7888', door: '#5A6A78' }
      : fall
        ? { wall: '#E8D0B0', trim: '#C8A888', roof: '#6A4030', door: '#6A3A28' }
        : { wall: '#F0E4D0', trim: '#D0C0A8', roof: '#5A4A42', door: '#6A4840' },
  )
  const houseB = $derived(
    winter
      ? { wall: '#C8D0D8', trim: '#98A0A8', roof: '#5A6878', door: '#4A5A68' }
      : fall
        ? { wall: '#D8C8B0', trim: '#B0A088', roof: '#4A3830', door: '#5A3A30' }
        : { wall: '#E8F0E8', trim: '#C0D0C0', roof: '#4A5450', door: '#3A5A48' },
  )
  const houseC = $derived(
    winter
      ? { wall: '#D8DCE0', trim: '#A8B0B8', roof: '#708088', door: '#5A6870' }
      : { wall: '#F8F0E0', trim: '#E0D0B8', roof: '#6A5040', door: '#8A5040' },
  )

  const lawn = $derived(
    winter ? '#C8D4C8' : fall ? '#8A9A48' : spring ? '#6ABA58' : '#5AAA48',
  )
  const lawnDark = $derived(winter ? '#A8B8A8' : fall ? '#6A7A38' : '#3A8A38')
  const windowGlass = $derived(winter ? '#C8D8EC' : summer ? '#7AB0C8' : '#8AB8C8')
  const windowLit = $derived(winter ? '#E8F0FA' : '#E8D090')
  const fence = $derived(winter ? '#E8E4DC' : '#F0E8D8')
  const fencePost = $derived(winter ? '#C8C0B0' : '#D8D0C0')
  const signInk = '#1A1E24'
  const signCream = '#F2E8D4'
  const asphalt = $derived(winter ? '#7A8288' : '#4A4E54')
  const carBody = $derived(winter ? '#6A7888' : '#3A4A5A')

  const fencePosts = [8, 18, 28, 38, 48, 58, 68]
  const fallLeaves = [
    { x: 30, y: 114, c: '#C45A2A' },
    { x: 48, y: 116, c: '#D4783A' },
    { x: 62, y: 113, c: '#E07030' },
    { x: 355, y: 115, c: '#A84828' },
    { x: 372, y: 114, c: '#C45A2A' },
    { x: 390, y: 116, c: '#D4783A' },
    { x: 20, y: 117, c: '#B05020' },
    { x: 400, y: 117, c: '#E07830' },
  ]
  const springFlowers = [
    { x: 24, y: 114, c: '#F0A0C8' },
    { x: 40, y: 115, c: '#F0D060' },
    { x: 56, y: 114, c: '#90C8F0' },
    { x: 360, y: 114, c: '#E090C0' },
    { x: 378, y: 115, c: '#F0A0C8' },
    { x: 395, y: 114, c: '#F0D060' },
  ]
  const roofSnow = [
    { x: 10, w: 52 },
    { x: 348, w: 48 },
  ]
</script>

<g class="suburb" aria-hidden="true">
  <!-- distant tree line -->
  <g fill={g.far} opacity="0.45">
    <ellipse cx="40" cy="95" rx="22" ry="14" />
    <ellipse cx="100" cy="98" rx="18" ry="12" />
    <ellipse cx="300" cy="96" rx="20" ry="13" />
    <ellipse cx="360" cy="94" rx="24" ry="15" />
    <ellipse cx="400" cy="97" rx="16" ry="11" />
  </g>

  <!-- ===== RANCH HOUSE A (left) ===== -->
  <g class="house-a">
    <rect x="6" y="82" width="64" height="36" fill={houseA.wall} />
    <path d="M4 82 L38 64 L72 82 Z" fill={houseA.roof} />
    {#if winter}
      <path d="M4 82 L38 64 L72 82 Z" fill={g.accent} opacity="0.7" />
    {/if}
    <!-- chimney -->
    <rect x="52" y="66" width="6" height="12" fill={houseA.trim} />
    <rect x="51" y="64" width="8" height="3" fill={houseA.roof} />
    <!-- windows -->
    <rect x="14" y="90" width="12" height="10" fill="#3A4048" />
    <rect x="15" y="91" width="10" height="8" fill={windowGlass} opacity="0.85" />
    <line x1="20" y1="91" x2="20" y2="99" stroke={houseA.trim} stroke-width="0.7" />
    <line x1="15" y1="95" x2="25" y2="95" stroke={houseA.trim} stroke-width="0.7" />
    <rect x="48" y="90" width="12" height="10" fill="#3A4048" />
    <rect x="49" y="91" width="10" height="8" fill={winter ? windowLit : windowGlass} opacity="0.85" />
    <line x1="54" y1="91" x2="54" y2="99" stroke={houseA.trim} stroke-width="0.7" />
    <line x1="49" y1="95" x2="59" y2="95" stroke={houseA.trim} stroke-width="0.7" />
    <!-- door -->
    <rect x="32" y="96" width="10" height="22" fill={houseA.door} />
    <circle cx="40" cy="107" r="0.7" fill={signCream} opacity="0.8" />
    <rect x="34" y="98" width="6" height="4" fill={windowGlass} opacity="0.5" />
    <!-- porch -->
    <rect x="28" y="114" width="18" height="4" fill={houseA.trim} />
    <line x1="30" y1="108" x2="30" y2="114" stroke={houseA.trim} stroke-width="1.2" />
    <line x1="44" y1="108" x2="44" y2="114" stroke={houseA.trim} stroke-width="1.2" />
    <!-- shutters -->
    <rect x="11" y="91" width="2.5" height="8" fill={fall ? '#8A4A32' : '#5A7A68'} />
    <rect x="26" y="91" width="2.5" height="8" fill={fall ? '#8A4A32' : '#5A7A68'} />
  </g>

  <!-- lawn A -->
  <path d="M4 118 Q20 112 38 114 Q55 116 72 118 L72 118 L4 118 Z" fill={lawn} opacity="0.75" />
  <path d="M8 118 Q25 115 40 116" fill="none" stroke={lawnDark} stroke-width="0.8" opacity="0.35" />

  <!-- ===== GARAGE + HOUSE B (right cluster) ===== -->
  <g class="house-b">
    <!-- garage -->
    <rect x="348" y="88" width="36" height="30" fill={houseB.wall} />
    <path d="M346 88 L366 74 L386 88 Z" fill={houseB.roof} />
    {#if winter}
      <path d="M346 88 L366 74 L386 88 Z" fill={g.accent} opacity="0.65" />
    {/if}
    <rect x="354" y="96" width="24" height="22" fill="#2A3038" />
    <rect x="355" y="97" width="22" height="20" fill={asphalt} opacity="0.7" />
    <!-- garage door panels -->
    <g stroke={houseB.trim} stroke-width="0.6" opacity="0.7" fill="none">
      <line x1="355" y1="102" x2="377" y2="102" />
      <line x1="355" y1="107" x2="377" y2="107" />
      <line x1="355" y1="112" x2="377" y2="112" />
    </g>
    <rect x="364" y="104" width="4" height="2" rx="0.3" fill="#8A9098" />

    <!-- ranch house C attached / beside -->
    <rect x="382" y="84" width="36" height="34" fill={houseC.wall} />
    <path d="M380 84 L400 68 L420 84 Z" fill={houseC.roof} />
    {#if winter}
      <path d="M380 84 L400 68 L420 84 Z" fill={g.accent} opacity="0.65" />
    {/if}
    <rect x="388" y="92" width="10" height="8" fill="#3A4048" />
    <rect x="389" y="93" width="8" height="6" fill={windowGlass} opacity="0.85" />
    <line x1="393" y1="93" x2="393" y2="99" stroke={houseC.trim} stroke-width="0.6" />
    <rect x="402" y="92" width="10" height="8" fill="#3A4048" />
    <rect x="403" y="93" width="8" height="6" fill={windowLit} opacity="0.8" />
    <rect x="394" y="102" width="8" height="16" fill={houseC.door} />
    <circle cx="400" cy="110" r="0.6" fill={signCream} />
  </g>

  <!-- lawn B -->
  <path d="M348 118 Q370 112 390 114 Q405 116 420 118 L348 118 Z" fill={lawn} opacity="0.7" />

  <!-- ===== MID house peek (background, above corridor — not blocking) ===== -->
  <g opacity="0.55">
    <rect x="200" y="78" width="40" height="22" fill={houseB.wall} />
    <path d="M198 78 L220 66 L242 78 Z" fill={houseB.roof} />
    <rect x="208" y="86" width="8" height="6" fill={windowGlass} opacity="0.7" />
    <rect x="224" y="86" width="8" height="6" fill={windowGlass} opacity="0.7" />
  </g>

  <!-- ===== PICKET FENCE (left yard edge) ===== -->
  <g class="picket">
    <line x1="6" y1="112" x2="74" y2="112" stroke={fencePost} stroke-width="1.2" />
    <line x1="6" y1="116" x2="74" y2="116" stroke={fencePost} stroke-width="1" />
    {#each fencePosts as x}
      <path d={`M${x} 118 L${x} 106 L${x + 3} 104 L${x + 6} 106 L${x + 6} 118 Z`} fill={fence} stroke={fencePost} stroke-width="0.4" />
    {/each}
  </g>

  <!-- short fence right -->
  <g class="picket-right">
    <line x1="348" y1="114" x2="380" y2="114" stroke={fencePost} stroke-width="1" />
    {#each [350, 360, 370] as x}
      <path d={`M${x} 118 L${x} 108 L${x + 2.5} 106.5 L${x + 5} 108 L${x + 5} 118 Z`} fill={fence} stroke={fencePost} stroke-width="0.4" />
    {/each}
  </g>

  <!-- ===== MAILBOX (left of corridor) ===== -->
  <g class="mailbox" transform="translate(74, 0)">
    <rect x="0" y="100" width="2.5" height="18" fill="#6A6A68" />
    <rect x="-4" y="96" width="12" height="8" rx="1" fill={fall ? '#8A4030' : '#3A5A88'} />
    <rect x="-3" y="97" width="10" height="5" rx="0.4" fill="#2A3A58" opacity="0.5" />
    <!-- flag (anim group 1: sk-bob) -->
    <g class="sk-bob">
      <rect x="8" y="97" width="5" height="1.4" fill="#C84838" />
      <circle cx="13" cy="97.7" r="1.2" fill="#C84838" />
    </g>
  </g>

  <!-- ===== BASKETBALL HOOP (right edge near garage) ===== -->
  <g class="hoop" transform="translate(340, 0)">
    <rect x="0" y="78" width="2.5" height="40" fill="#6A7078" />
    <rect x="-8" y="76" width="14" height="10" rx="0.5" fill="#E8E4DC" />
    <rect x="-6" y="78" width="10" height="6" fill="#3A4A6A" opacity="0.35" />
    <ellipse cx="-1" cy="88" rx="5" ry="2.2" fill="none" stroke="#D48030" stroke-width="1.3" />
    <path d="M-5 88 L-5 94 M-3 88 L-3 95 M-1 88 L-1 95.5 M1 88 L1 95 M3 88 L3 94" stroke="#E8E4DC" stroke-width="0.45" opacity="0.7" />
  </g>

  <!-- ===== FIRE HYDRANT ===== -->
  <g class="hydrant" transform="translate(330, 108)">
    <rect x="-2.5" y="2" width="5" height="8" rx="1" fill={winter ? '#A84848' : '#C84840'} />
    <rect x="-4" y="4" width="2" height="3" rx="0.4" fill={winter ? '#984040' : '#B04038'} />
    <rect x="2" y="4" width="2" height="3" rx="0.4" fill={winter ? '#984040' : '#B04038'} />
    <ellipse cx="0" cy="2" rx="3" ry="1.5" fill={winter ? '#B85858' : '#D85850'} />
    <rect x="-1.2" y="-1" width="2.4" height="3" fill={winter ? '#984848' : '#A83830'} />
    {#if winter}
      <ellipse cx="0" cy="0" rx="2.5" ry="1" fill={g.accent} opacity="0.55" />
    {/if}
  </g>

  <!-- ===== PARKED CAR SILHOUETTE (far right edge) ===== -->
  <g class="car" transform="translate(400, 110)" fill={carBody} opacity="0.88">
    <path d="M-2 6 L0 0 L8 -2 L22 -2 L28 2 L30 6 Z" />
    <rect x="6" y="-1" width="8" height="4" rx="0.5" fill={windowGlass} opacity="0.55" />
    <rect x="16" y="-1" width="6" height="4" rx="0.5" fill={windowGlass} opacity="0.45" />
    <circle cx="6" cy="7" r="3" fill="#2A2A2C" />
    <circle cx="6" cy="7" r="1.3" fill="#6A6A68" />
    <circle cx="24" cy="7" r="3" fill="#2A2A2C" />
    <circle cx="24" cy="7" r="1.3" fill="#6A6A68" />
    {#if winter}
      <ellipse cx="14" cy="-2.5" rx="10" ry="1.5" fill={g.accent} opacity="0.5" />
    {/if}
  </g>

  <!-- driveway strip -->
  <path d="M354 118 L386 118 L390 128 L350 128 Z" fill={asphalt} opacity="0.45" />
  <path d="M360 120 L384 120" stroke={signCream} stroke-width="0.6" stroke-dasharray="3 2" opacity="0.35" />

  <!-- ===== YARD SALE SIGN ===== -->
  <g class="yard-sale" transform="translate(82, 0)">
    <rect x="4" y="100" width="1.8" height="18" fill="#8A7A60" />
    <rect x="-4" y="92" width="20" height="12" rx="0.5" fill="#F0E060" stroke="#C8A830" stroke-width="0.7" />
    <text
      x="6"
      y="97"
      text-anchor="middle"
      font-size="3.2"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signInk}>YARD</text
    >
    <text
      x="6"
      y="102"
      text-anchor="middle"
      font-size="3.2"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill={signInk}>SALE</text
    >
  </g>

  <!-- ===== LOST CAT? flyer on pole ===== -->
  <g class="flyer-pole" transform="translate(318, 0)">
    <rect x="0" y="70" width="2.2" height="48" fill="#7A7A78" />
    <!-- street lamp cap -->
    <path d="M-4 70 L1 64 L6 70 Z" fill="#5A5A58" />
    <ellipse cx="1" cy="64" rx="3" ry="1.5" fill="#8A8A88" />
    <!-- flyer -->
    <rect x="4" y="88" width="14" height="16" rx="0.4" fill={signCream} stroke="#C8C0B0" stroke-width="0.5" />
    <text
      x="11"
      y="93"
      text-anchor="middle"
      font-size="2.6"
      font-weight="800"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#C84838">LOST CAT?</text
    >
    <!-- tiny cat doodle -->
    <ellipse cx="11" cy="98" rx="3.5" ry="2.2" fill="#3A3A38" opacity="0.7" />
    <circle cx="13.5" cy="96.5" r="1.6" fill="#3A3A38" opacity="0.7" />
    <path d="M12.5 95 L12 93.5 M14.2 95 L14.8 93.5" stroke="#3A3A38" stroke-width="0.6" opacity="0.7" />
    <text
      x="11"
      y="103"
      text-anchor="middle"
      font-size="2.2"
      font-weight="700"
      font-family="ui-rounded, system-ui, sans-serif"
      fill="#5A4A38">“beep”?</text
    >
  </g>
  <!-- tape -->
  <rect x="324" y="87" width="4" height="1.2" fill="#C8C8C0" opacity="0.8" />

  <!-- shade tree (left, anim group 2: sk-sway) -->
  <g transform="translate(58, 0)">
    <rect x="0" y="95" width="3" height="23" fill={winter ? '#8A7A68' : '#5A4634'} />
    <g class="sk-sway" fill={winter ? '#C8D0C8' : fall ? '#C45A2A' : spring ? '#6ABA58' : '#3A8A40'} opacity="0.85">
      <ellipse cx="1.5" cy="88" rx="14" ry="12" />
      <ellipse cx="-6" cy="92" rx="8" ry="7" />
      <ellipse cx="10" cy="92" rx="7" ry="6" />
    </g>
    {#if fall}
      <g fill="#D4783A" opacity="0.7">
        <ellipse cx="-4" cy="100" rx="1.2" ry="0.8" />
        <ellipse cx="6" cy="102" rx="1" ry="0.7" />
        <ellipse cx="2" cy="105" rx="1.1" ry="0.75" />
      </g>
    {/if}
  </g>

  <!-- ===== SEASONAL PROPS ===== -->
  {#if fall}
    <g class="leaves">
      {#each fallLeaves as L}
        <ellipse cx={L.x} cy={L.y} rx="2.2" ry="1.3" fill={L.c} opacity="0.85" transform={`rotate(${L.x % 40} ${L.x} ${L.y})`} />
      {/each}
    </g>
  {:else if winter}
    {#each roofSnow as s}
      <ellipse cx={s.x + s.w / 2} cy="118" rx={s.w / 2} ry="2" fill={g.accent} opacity="0.4" />
    {/each}
    <g fill={g.accent} opacity="0.55">
      <ellipse cx="40" cy="117" rx="16" ry="2.2" />
      <ellipse cx="370" cy="117" rx="18" ry="2.4" />
    </g>
  {:else if spring}
    <g class="flowers">
      {#each springFlowers as fl}
        <g>
          <line x1={fl.x} y1={fl.y} x2={fl.x} y2={fl.y - 5} stroke="#4A8A48" stroke-width="0.8" />
          <circle cx={fl.x} cy={fl.y - 6} r="1.7" fill={fl.c} />
          <circle cx={fl.x - 1.3} cy={fl.y - 5.2} r="1.1" fill={fl.c} opacity="0.85" />
          <circle cx={fl.x + 1.3} cy={fl.y - 5.2} r="1.1" fill={fl.c} opacity="0.85" />
          <circle cx={fl.x} cy={fl.y - 5.5} r="0.7" fill="#F0E060" />
        </g>
      {/each}
    </g>
  {:else if summer}
    <!-- lemonade stand (left edge, clear of corridor) -->
    <g class="lemonade" transform="translate(20, 0)">
      <rect x="0" y="104" width="28" height="14" fill="#E8D8A8" />
      <path d="M-2 104 L14 94 L30 104 Z" fill="#F0E060" />
      <g fill={signCream} opacity="0.7">
        <rect x="2" y="96" width="2" height="8" transform="skewX(-12)" />
        <rect x="7" y="96" width="2" height="8" transform="skewX(-12)" />
        <rect x="12" y="96" width="2" height="8" transform="skewX(-12)" />
        <rect x="17" y="96" width="2" height="8" transform="skewX(-12)" />
        <rect x="22" y="96" width="2" height="8" transform="skewX(-12)" />
      </g>
      <rect x="4" y="108" width="20" height="6" rx="0.4" fill="#C84848" />
      <text
        x="14"
        y="112.5"
        text-anchor="middle"
        font-size="3"
        font-weight="800"
        font-family="ui-rounded, system-ui, sans-serif"
        fill={signCream}>LEMONADE</text
      >
      <text
        x="14"
        y="102"
        text-anchor="middle"
        font-size="2.4"
        font-weight="700"
        font-family="ui-rounded, system-ui, sans-serif"
        fill="#8A5020">purrfect</text
      >
      <!-- pitcher -->
      <path d="M8 106 L10 100 L16 100 L18 106 Z" fill="#F0F0A0" opacity="0.85" />
      <ellipse cx="13" cy="100" rx="3.2" ry="1" fill="#F8F8C0" />
      <rect x="20" y="102" width="3" height="4" rx="0.3" fill="#F0E8D0" />
    </g>
  {/if}

  <!-- ===== GROUND — top y=118; clear corridor ~80–340 ===== -->
  <rect x="0" y="118" width="420" height="22" fill={g.near} />
  <rect x="0" y="118" width="420" height="3" fill={g.detail} opacity="0.22" />
  <!-- sidewalk -->
  <rect x="0" y="122" width="420" height="6" fill={g.path} opacity="0.55" />
  <g stroke={g.detail} stroke-width="0.5" opacity="0.25">
    <line x1="60" y1="122" x2="60" y2="128" />
    <line x1="120" y1="122" x2="120" y2="128" />
    <line x1="180" y1="122" x2="180" y2="128" />
    <line x1="240" y1="122" x2="240" y2="128" />
    <line x1="300" y1="122" x2="300" y2="128" />
    <line x1="360" y1="122" x2="360" y2="128" />
  </g>
  <rect x="0" y="128" width="420" height="12" fill={g.path} opacity="0.4" />

  <!-- curb grass nibbles at edges only -->
  <g fill={lawn} opacity="0.5">
    <ellipse cx="24" cy="119" rx="16" ry="2" />
    <ellipse cx="60" cy="119" rx="12" ry="1.6" />
    <ellipse cx="360" cy="119" rx="14" ry="1.8" />
    <ellipse cx="400" cy="119" rx="16" ry="2" />
  </g>

  {#if winter}
    <rect x="0" y="116" width="420" height="5" fill={g.accent} opacity="0.85" />
    <g fill={g.accent} opacity="0.45">
      <ellipse cx="50" cy="120" rx="22" ry="2.5" />
      <ellipse cx="380" cy="120" rx="24" ry="2.5" />
    </g>
  {/if}
</g>
