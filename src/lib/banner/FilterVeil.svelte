<script lang="ts">
  // Only the "place" (cat-run) wipe from the original SOUNDKITTEN-ART dev
  // gallery is used in the app -- the time/weather/blend variants (and
  // their bright multi-hue gradients) never render here, so this keeps
  // only what's actually shown: a short, neutral dark sweep with no color,
  // no white flash, and no debug label text.
  let { pulse = 0 }: { pulse?: number } = $props();
</script>

<!-- key={pulse} remounts so the CSS animation always restarts -->
{#key pulse}
  <div class="veil" aria-hidden="true">
    <div class="sheet"></div>
  </div>
{/key}

<style>
  .veil {
    pointer-events: none;
    position: absolute;
    inset: 0;
    z-index: 4;
    overflow: hidden;
    border-radius: 2px;
  }

  .sheet {
    position: absolute;
    inset: 0;
    opacity: 0;
    background: linear-gradient(90deg, rgba(10, 8, 6, 0.6) 0%, rgba(10, 8, 6, 0.3) 45%, transparent 100%);
    transform-origin: left center;
    animation: veil-run 380ms cubic-bezier(0.22, 1, 0.36, 1) forwards;
  }

  @keyframes veil-run {
    0% {
      opacity: 0;
      transform: scaleX(0.08) translate3d(-4%, 0, 0);
    }
    40% {
      opacity: 0.9;
      transform: scaleX(1) translate3d(0, 0, 0);
    }
    100% {
      opacity: 0;
      transform: scaleX(1) translate3d(18%, 0, 0);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .sheet {
      animation-duration: 1ms !important;
      opacity: 0 !important;
    }
  }
</style>
