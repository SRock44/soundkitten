/** Shared rAF director for banner cats. Wall-clock timed for steady video cadence. */

export type CatActor = {
  id: string
  active: boolean
  apply: (x: number, mode: 'trot', t: number) => void
  /** Phase offsets so cards don't sync identically */
  x0: number
  gait0: number
  /** Fires once per lap when the cat finishes leaving the right edge */
  onExitRight?: () => void
  lastX: number
}

const actors = new Map<string, CatActor>()
let raf = 0

/** Video: 15 frames @ 30fps → 2 cycles/sec (solid frame cuts, no crossfade). */
export const CYCLE_HZ = 2
/** Scroll speed (px/s) */
export const SPEED = 150
/** Wrap: enter from left, leave right (viewBox 420) */
export const WRAP_START = -80
export const WRAP_END = 420
const WRAP_SPAN = WRAP_END - WRAP_START

function hashSeed(s: string): number {
  let h = 2166136261
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i)
    h = Math.imul(h, 16777619)
  }
  return h >>> 0
}

function wrapX(xRaw: number): number {
  return WRAP_START + ((((xRaw - WRAP_START) % WRAP_SPAN) + WRAP_SPAN) % WRAP_SPAN)
}

function tick(now: number) {
  const t = now * 0.001
  let anyActive = false

  for (const actor of actors.values()) {
    if (!actor.active) continue
    anyActive = true

    // Wall-clock motion — no dt accumulation jitter
    const xRaw = actor.x0 + t * SPEED
    const x = wrapX(xRaw)
    const gaitT = actor.gait0 + t * CYCLE_HZ

    // Detect wrap: was near the right exit, now back on the left entry
    if (actor.lastX > WRAP_END - 60 && x < WRAP_START + 60) {
      const cb = actor.onExitRight
      if (cb) queueMicrotask(cb)
    }
    actor.lastX = x

    // Sub-pixel scroll — smoother than integer snapping with crossfaded sprites
    actor.apply(x, 'trot', gaitT)
  }

  raf = anyActive ? requestAnimationFrame(tick) : 0
}

function ensureLoop() {
  if (raf) return
  raf = requestAnimationFrame(tick)
}

export function registerCat(
  id: string,
  apply: CatActor['apply'],
  opts: { startX?: number; onExitRight?: () => void; alignEntry?: boolean } = {},
): { setActive: (v: boolean) => void; destroy: () => void } {
  const startX = opts.startX ?? WRAP_START
  const seed = hashSeed(id)
  let actor = actors.get(id)
  if (!actor) {
    const now = performance.now() * 0.001
    // alignEntry: place the cat just off the left edge at mount time
    const x0 = opts.alignEntry ? WRAP_START - now * SPEED : startX + (seed % 110)
    actor = {
      id,
      active: false,
      apply,
      x0,
      gait0: opts.alignEntry ? 0 : (seed % 100) / 100,
      onExitRight: opts.onExitRight,
      lastX: opts.alignEntry ? WRAP_START : startX,
    }
    actors.set(id, actor)
  } else {
    actor.apply = apply
    actor.onExitRight = opts.onExitRight
  }

  return {
    setActive(v: boolean) {
      const a = actors.get(id)
      if (!a) return
      a.active = v
      if (v) {
        ensureLoop()
        // immediate paint at current clock
        const now = performance.now() * 0.001
        const x = wrapX(a.x0 + now * SPEED)
        a.lastX = x
        a.apply(x, 'trot', a.gait0 + now * CYCLE_HZ)
      }
    },
    destroy() {
      actors.delete(id)
      if (![...actors.values()].some((a) => a.active) && raf) {
        cancelAnimationFrame(raf)
        raf = 0
      }
    },
  }
}

if (import.meta.hot) {
  import.meta.hot.dispose(() => {
    if (raf) cancelAnimationFrame(raf)
    raf = 0
    actors.clear()
  })
}
