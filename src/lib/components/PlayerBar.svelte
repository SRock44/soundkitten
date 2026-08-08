<script lang="ts">
  import { player } from "../stores/player.svelte";
  import { formatDuration } from "../types";
  import type { Track } from "../types";
  import Icon from "./Icon.svelte";

  let { onOpenTrack, onOpenProfile }: { onOpenTrack: (t: Track) => void; onOpenProfile: (id: number) => void } = $props();

  let audioEl: HTMLAudioElement;
  let currentTime = $state(0);
  let duration = $state(0);
  let isPlaying = $state(false);
  let showQueue = $state(false);
  let dragIndex = $state<number | null>(null);
  let progressPct = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);

  $effect(() => {
    player.attach(audioEl);
  });
  $effect(() => {
    player.isPlaying = isPlaying;
  });

  function seek(e: MouseEvent) {
    if (!duration) return;
    const bar = e.currentTarget as HTMLElement;
    const rect = bar.getBoundingClientRect();
    const pct = (e.clientX - rect.left) / rect.width;
    audioEl.currentTime = pct * duration;
  }

  function onVolumeInput(e: Event) {
    player.setVolume(Number((e.target as HTMLInputElement).value));
  }

  function isTypingTarget(el: EventTarget | null): boolean {
    if (!(el instanceof HTMLElement)) return false;
    return el.tagName === "INPUT" || el.tagName === "TEXTAREA" || el.isContentEditable;
  }

  function onGlobalKeydown(e: KeyboardEvent) {
    if (e.code !== "Space" || isTypingTarget(e.target)) return;
    e.preventDefault();
    player.toggle();
  }

  function dragStart(absIndex: number) {
    dragIndex = absIndex;
  }

  function dragOver(e: DragEvent) {
    e.preventDefault();
  }

  function drop(absIndex: number) {
    if (dragIndex !== null && dragIndex !== absIndex) {
      player.reorderQueue(dragIndex, absIndex);
    }
    dragIndex = null;
  }
</script>

<svelte:window onkeydown={onGlobalKeydown} />

<div class="player-bar">
  <audio
    bind:this={audioEl}
    ontimeupdate={() => (currentTime = audioEl.currentTime)}
    ondurationchange={() => (duration = audioEl.duration || 0)}
    onplay={() => (isPlaying = true)}
    onpause={() => (isPlaying = false)}
    onended={() => player.next()}
    onerror={() => (player.error = audioEl.error?.message ?? "playback error")}
  ></audio>

  <button class="scrubber" onclick={seek} aria-label="Seek" disabled={!player.current}>
    <span class="scrubber-fill" style="width: {progressPct}%"></span>
  </button>

  {#if showQueue}
    <div class="queue-panel">
      <div class="queue-header">
        <span>Up next</span>
        <button class="close" onclick={() => (showQueue = false)} aria-label="Close queue"><Icon name="close" size={12} /></button>
      </div>
      {#if player.upcoming.length === 0}
        <p class="queue-empty">Nothing queued.</p>
      {:else}
        <ul>
          {#each player.upcoming as t, i}
            {@const absIndex = player.queueIndex + 1 + i}
            <li
              draggable="true"
              class:dragging={dragIndex === absIndex}
              ondragstart={() => dragStart(absIndex)}
              ondragover={dragOver}
              ondrop={() => drop(absIndex)}
              ondragend={() => (dragIndex = null)}
            >
              <span class="drag-handle"><Icon name="grip" size={12} /></span>
              <span class="qtitle">{t.title ?? `Track #${t.id}`}</span>
              <span class="qartist">{t.user?.username ?? ""}</span>
              <button class="qremove" onclick={() => player.removeFromQueue(absIndex)} aria-label="Remove from queue"><Icon name="close" size={11} /></button>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {/if}

  <div class="content">
    <div class="track-info">
      {#if player.current}
        <button class="artwork-btn" onclick={() => onOpenTrack(player.current!)} aria-label="View track">
          {#if player.current.artwork_url}
            <img src={player.current.artwork_url} alt="" class="artwork" />
          {:else}
            <div class="artwork artwork-fallback"><Icon name="music" size={14} /></div>
          {/if}
        </button>
        <div class="text">
          <button class="title" onclick={() => onOpenTrack(player.current!)}>{player.current.title ?? `Track #${player.current.id}`}</button>
          {#if player.current.user}
            <button class="artist" onclick={() => onOpenProfile(player.current!.user!.id)}>{player.current.user.username ?? ""}</button>
          {/if}
        </div>
      {:else}
        <span class="muted">Nothing playing</span>
      {/if}
    </div>

    <div class="controls">
      <button onclick={() => player.previous()} disabled={!player.current} aria-label="Previous"><Icon name="skip-back" /></button>
      <button class="play" onclick={() => player.toggle()} disabled={!player.current} aria-label="Play/Pause">
        <Icon name={isPlaying ? "pause" : "play"} size={18} />
      </button>
      <button onclick={() => player.next()} disabled={!player.current} aria-label="Next"><Icon name="skip-forward" /></button>
    </div>

    <div class="right-controls">
      <div class="time-display">
        <span>{formatDuration(currentTime * 1000)}</span>
        <span class="sep">/</span>
        <span>{formatDuration(duration * 1000)}</span>
      </div>
      <button class="queue-toggle" class:active={showQueue} onclick={() => (showQueue = !showQueue)} aria-label="Toggle queue"><Icon name="queue" size={15} /></button>
      <div class="volume">
        <span class="vol-icon"><Icon name={player.volume === 0 ? "volume-mute" : "volume"} size={15} /></span>
        <input type="range" min="0" max="1" step="0.01" value={player.volume} oninput={onVolumeInput} aria-label="Volume" />
      </div>
    </div>
  </div>

  {#if player.error}
    <div class="error">{player.error}</div>
  {/if}
</div>

<style>
.player-bar {
  position: relative;
  background: var(--player-bg);
  color: var(--player-fg);
  flex-shrink: 0;
}

.scrubber {
  width: 100%;
  height: 4px;
  padding: 0;
  border: none;
  background: rgba(255, 255, 255, 0.15);
  cursor: pointer;
  display: block;
}

.scrubber:disabled {
  cursor: default;
}

.scrubber-fill {
  display: block;
  height: 100%;
  background: var(--accent);
}

.content {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  gap: 1rem;
  padding: 0.65rem 1.25rem;
}

.track-info {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  min-width: 0;
}

.artwork-btn {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  flex-shrink: 0;
}

.artwork {
  width: 40px;
  height: 40px;
  border-radius: 3px;
  object-fit: cover;
  display: block;
}

.artwork-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  background: #333;
  color: #888;
}

.text {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.title,
.artist {
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  text-align: left;
  font: inherit;
  color: #e8e8e8;
}

.title {
  font-weight: 600;
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.title:hover,
.artist:hover {
  text-decoration: underline;
}

.artist {
  font-size: 0.78rem;
  color: #a0a0a0;
}

.muted {
  color: #a0a0a0;
}

.controls {
  display: flex;
  align-items: center;
  gap: 1rem;
  justify-self: center;
}

.controls button {
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  cursor: pointer;
  color: #e8e8e8;
  padding: 0.25rem;
}

.controls button:disabled {
  opacity: 0.35;
  cursor: default;
}

.controls .play {
  width: 2.2rem;
  height: 2.2rem;
  border-radius: 50%;
  background: white;
  color: #111;
  display: flex;
  align-items: center;
  justify-content: center;
}

.controls .play:disabled {
  background: #555;
  color: #999;
}

.right-controls {
  display: flex;
  align-items: center;
  gap: 0.9rem;
  justify-self: end;
}

.time-display {
  font-size: 0.75rem;
  color: #a0a0a0;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
}

.sep {
  margin: 0 0.2em;
}

.queue-toggle {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  cursor: pointer;
  color: #a0a0a0;
  padding: 0.3rem;
  border-radius: 4px;
}

.queue-toggle:hover,
.queue-toggle.active {
  color: white;
  background: rgba(255, 255, 255, 0.1);
}

.volume {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.vol-icon {
  display: flex;
  align-items: center;
}

.volume input[type="range"] {
  width: 80px;
  accent-color: var(--accent);
}

.error {
  position: absolute;
  bottom: 100%;
  left: 1.25rem;
  right: 1.25rem;
  background: var(--error-bg);
  color: var(--error-text);
  padding: 0.4rem 0.75rem;
  border-radius: 6px;
  font-size: 0.85rem;
  margin-bottom: 0.5rem;
}

.queue-panel {
  position: absolute;
  bottom: 100%;
  right: 1.25rem;
  width: 20rem;
  max-height: 22rem;
  overflow-y: auto;
  background: #161616;
  border: 1px solid #2a2a2a;
  border-radius: 8px 8px 0 0;
  box-shadow: 0 -8px 24px rgba(0, 0, 0, 0.4);
}

.queue-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.6rem 0.8rem;
  border-bottom: 1px solid #2a2a2a;
  font-size: 0.85rem;
  font-weight: 600;
  position: sticky;
  top: 0;
  background: #161616;
}

.queue-header .close {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  color: #a0a0a0;
  cursor: pointer;
}

.queue-empty {
  padding: 1rem 0.8rem;
  color: #888;
  font-size: 0.85rem;
  margin: 0;
}

.queue-panel ul {
  list-style: none;
  margin: 0;
  padding: 0.3rem;
}

.queue-panel li {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.4rem 0.5rem;
  border-radius: 5px;
  font-size: 0.82rem;
  cursor: grab;
}

.queue-panel li:hover {
  background: rgba(255, 255, 255, 0.06);
}

.queue-panel li.dragging {
  opacity: 0.4;
}

.drag-handle {
  display: flex;
  align-items: center;
  color: #666;
  flex-shrink: 0;
}

.qtitle {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 10rem;
}

.qartist {
  color: #888;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.qremove {
  display: flex;
  align-items: center;
  background: none;
  border: none;
  color: #888;
  cursor: pointer;
  flex-shrink: 0;
}

.qremove:hover {
  color: white;
}
</style>
