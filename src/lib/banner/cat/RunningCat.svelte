<script lang="ts">
  import { untrack } from 'svelte'
  import { registerCat } from './catDirector'
  import { SPRITE_W, SPRITE_H, FRAME_COUNT as META_FRAMES } from './spriteMeta'
  import { FRAMES } from './frames'

  const FRAME_COUNT = FRAMES.length || META_FRAMES
  const DRAW_H = 52
  const DRAW_W = Math.round((SPRITE_W / SPRITE_H) * DRAW_H)
  // good-v1 — solid double-buffer swaps (no crossfade transparency)

  for (const src of FRAMES) {
    const img = new Image()
    img.decoding = 'async'
    img.src = src
  }

  let {
    playing = true,
    id = 'cat',
    onExitRight,
    alignEntry = false,
  }: {
    playing?: boolean
    id?: string
    /** Called when the cat finishes leaving the right edge (wraps back left) */
    onExitRight?: () => void
    /** Start at the left entry (preview sync) instead of a seeded mid-phase */
    alignEntry?: boolean
  } = $props()

  let trackEl: SVGGElement | undefined = $state()
  let imgA: SVGImageElement | undefined = $state()
  let imgB: SVGImageElement | undefined = $state()
  let lastIdx = -1
  /** Which buffer is currently visible */
  let showA = true

  /** Keep exit callback stable so registerCat isn't torn down every parent render */
  const exitRef: { current?: () => void } = {}
  $effect.pre(() => {
    exitRef.current = onExitRight
  })

  function paint(x: number, _mode: string, t: number) {
    const idx = Math.floor((((t % 1) + 1) % 1) * FRAME_COUNT) % FRAME_COUNT
    if (trackEl) {
      trackEl.setAttribute('transform', `translate(${x} ${-DRAW_H})`)
    }
    if (!imgA || !imgB || idx === lastIdx) return

    // Load next frame onto the hidden buffer, then hard-cut (always opacity 0 or 1)
    const front = showA ? imgA : imgB
    const back = showA ? imgB : imgA
    back.setAttribute('href', FRAMES[idx])
    back.setAttribute('opacity', '1')
    front.setAttribute('opacity', '0')
    showA = !showA
    lastIdx = idx
  }

  $effect(() => {
    const handle = registerCat(id, paint, {
      alignEntry,
      onExitRight: () => exitRef.current?.(),
    })
    if (imgA && imgB) {
      imgA.setAttribute('href', FRAMES[0])
      imgA.setAttribute('opacity', '1')
      imgB.setAttribute('href', FRAMES[0])
      imgB.setAttribute('opacity', '0')
      showA = true
      lastIdx = 0
    }
    // Don't track `playing` here — pause/play must not remount the actor
    handle.setActive(untrack(() => playing))
    return () => handle.destroy()
  })

  $effect(() => {
    registerCat(id, paint, {
      alignEntry,
      onExitRight: () => exitRef.current?.(),
    }).setActive(playing)
  })
</script>

<g class="runner" style="pointer-events:none">
  <g bind:this={trackEl}>
    <image
      bind:this={imgA}
      href={FRAMES[0]}
      x="0"
      y="0"
      width={DRAW_W}
      height={DRAW_H}
      preserveAspectRatio="xMidYMax meet"
      opacity="1"
    />
    <image
      bind:this={imgB}
      href={FRAMES[0]}
      x="0"
      y="0"
      width={DRAW_W}
      height={DRAW_H}
      preserveAspectRatio="xMidYMax meet"
      opacity="0"
    />
  </g>
</g>
