<script lang="ts">
  export type IconName =
    | "search"
    | "heart"
    | "heart-filled"
    | "repost"
    | "comment"
    | "play"
    | "pause"
    | "skip-back"
    | "skip-forward"
    | "volume"
    | "volume-mute"
    | "queue"
    | "close"
    | "mail"
    | "bell"
    | "chevron-down"
    | "grip"
    | "arrow-left"
    | "music"
    | "playback"
    | "minimize"
    | "maximize"
    | "restore"
    | "lock"
    | "external-link"
    | "share"
    | "check"
    | "shuffle"
    | "repeat"
    | "user"
    | "user-filled"
    | "cloud"
    | "refresh";

  let { name, size = 16 }: { name: IconName; size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
  {#if name === "search"}
    <circle cx="8.5" cy="8.5" r="5.5" />
    <line x1="17" y1="17" x2="13" y2="13" />
  {:else if name === "heart"}
    <path d="M10 17s-6.5-4-6.5-9A3.9 3.9 0 0 1 10 5.5 3.9 3.9 0 0 1 16.5 8c0 5-6.5 9-6.5 9Z" />
  {:else if name === "heart-filled"}
    <path d="M10 17s-6.5-4-6.5-9A3.9 3.9 0 0 1 10 5.5 3.9 3.9 0 0 1 16.5 8c0 5-6.5 9-6.5 9Z" fill="currentColor" />
  {:else if name === "repost"}
    <polyline points="4 7 4 13 10 13" />
    <polyline points="16 13 16 7 10 7" />
    <path d="M4 13a6 6 0 0 0 10.5 3.8" />
    <path d="M16 7a6 6 0 0 0-10.5-3.8" />
  {:else if name === "comment"}
    <path d="M3 4.5h14a1 1 0 0 1 1 1v8a1 1 0 0 1-1 1H8l-3.5 3v-3H3a1 1 0 0 1-1-1v-8a1 1 0 0 1 1-1Z" />
  {:else if name === "play"}
    <path d="M6 3.5v13l11-6.5-11-6.5Z" fill="currentColor" stroke="none" />
  {:else if name === "pause"}
    <rect x="5" y="3.5" width="3.2" height="13" rx="0.6" fill="currentColor" stroke="none" />
    <rect x="11.8" y="3.5" width="3.2" height="13" rx="0.6" fill="currentColor" stroke="none" />
  {:else if name === "skip-back"}
    <path d="M6 4v12" />
    <path d="M16 5 7.5 10l8.5 5V5Z" fill="currentColor" stroke="none" />
  {:else if name === "skip-forward"}
    <path d="M14 4v12" />
    <path d="M4 5l8.5 5L4 15V5Z" fill="currentColor" stroke="none" />
  {:else if name === "volume"}
    <path d="M3 7.5h3L11 4v12l-5-3.5H3v-5Z" />
    <path d="M14 7a4 4 0 0 1 0 6" />
    <path d="M16.3 5a7.3 7.3 0 0 1 0 10" />
  {:else if name === "volume-mute"}
    <path d="M3 7.5h3L11 4v12l-5-3.5H3v-5Z" />
    <line x1="14" y1="7.5" x2="18" y2="12.5" />
    <line x1="18" y1="7.5" x2="14" y2="12.5" />
  {:else if name === "queue"}
    <line x1="3" y1="5" x2="17" y2="5" />
    <line x1="3" y1="10" x2="17" y2="10" />
    <line x1="3" y1="15" x2="12" y2="15" />
  {:else if name === "close"}
    <line x1="4" y1="4" x2="16" y2="16" />
    <line x1="16" y1="4" x2="4" y2="16" />
  {:else if name === "mail"}
    <rect x="2.5" y="4.5" width="15" height="11" rx="1.2" />
    <path d="M3 5.5l7 5.5 7-5.5" />
  {:else if name === "bell"}
    <path d="M5 8.5a5 5 0 0 1 10 0c0 3.5 1.2 4.5 1.2 4.5H3.8S5 12 5 8.5Z" />
    <path d="M8.2 15.5a1.8 1.8 0 0 0 3.6 0" />
  {:else if name === "chevron-down"}
    <polyline points="4.5 7 10 12.5 15.5 7" />
  {:else if name === "grip"}
    <circle cx="7" cy="5.5" r="1" fill="currentColor" stroke="none" />
    <circle cx="7" cy="10" r="1" fill="currentColor" stroke="none" />
    <circle cx="7" cy="14.5" r="1" fill="currentColor" stroke="none" />
    <circle cx="12" cy="5.5" r="1" fill="currentColor" stroke="none" />
    <circle cx="12" cy="10" r="1" fill="currentColor" stroke="none" />
    <circle cx="12" cy="14.5" r="1" fill="currentColor" stroke="none" />
  {:else if name === "arrow-left"}
    <line x1="16" y1="10" x2="4" y2="10" />
    <polyline points="9 5 4 10 9 15" />
  {:else if name === "music"}
    <circle cx="6.5" cy="15" r="2" />
    <circle cx="14.5" cy="13" r="2" />
    <path d="M8.5 15V5.5L16.5 4v9" />
  {:else if name === "playback"}
    <path d="M6 3.5v13l11-6.5-11-6.5Z" fill="currentColor" stroke="none" />
  {:else if name === "minimize"}
    <line x1="4" y1="9.5" x2="16" y2="9.5" stroke-width="1.2" />
  {:else if name === "maximize"}
    <rect x="4.5" y="4.5" width="11" height="11" stroke-width="1.2" />
  {:else if name === "restore"}
    <rect x="6.5" y="3.5" width="8" height="8" stroke-width="1.2" />
    <path d="M3.5 6.5V13.5H10.5" stroke-width="1.2" />
  {:else if name === "lock"}
    <rect x="4.5" y="9" width="11" height="8" rx="1.2" />
    <path d="M6.5 9V6a3.5 3.5 0 0 1 7 0v3" />
  {:else if name === "external-link"}
    <path d="M8 5H4.5a1 1 0 0 0-1 1v9.5a1 1 0 0 0 1 1H14a1 1 0 0 0 1-1V12" />
    <path d="M10 3.5h6.5V10" />
    <line x1="16.5" y1="3.5" x2="9" y2="11" />
  {:else if name === "share"}
    <circle cx="15" cy="5" r="2.2" />
    <circle cx="15" cy="15" r="2.2" />
    <circle cx="5" cy="10" r="2.2" />
    <line x1="7" y1="8.8" x2="13" y2="6.2" />
    <line x1="7" y1="11.2" x2="13" y2="13.8" />
  {:else if name === "check"}
    <polyline points="4.5 10.5 8 14 15.5 6" />
  {:else if name === "shuffle"}
    <polyline points="12.5 3.5 16.5 3.5 16.5 7.5" />
    <line x1="4" y1="16" x2="16.5" y2="3.5" />
    <path d="M4 4h2.5l3 4" />
    <path d="M9.5 12l3 4h4" />
  {:else if name === "repeat"}
    <path d="M4 7.5h9a3 3 0 0 1 3 3v1" />
    <polyline points="10.5 4.5 13.5 7.5 10.5 10.5" />
    <path d="M16 12.5H7a3 3 0 0 1-3-3v-1" />
    <polyline points="9.5 15.5 6.5 12.5 9.5 9.5" />
  {:else if name === "user"}
    <circle cx="10" cy="6.5" r="3" />
    <path d="M4 16.5a6 6 0 0 1 12 0" />
  {:else if name === "user-filled"}
    <circle cx="10" cy="6.5" r="3" fill="currentColor" stroke="none" />
    <path d="M4 16.5a6 6 0 0 1 12 0" fill="currentColor" stroke="none" />
  {:else if name === "cloud"}
    <path
      d="M6.2 15.5a3.3 3.3 0 0 1-.4-6.58 4.2 4.2 0 0 1 8.1-1.4 3.15 3.15 0 0 1 3.6 3.1 3.15 3.15 0 0 1-3.1 3.15c-.1 0-.2 0-.3-.01v1.74Z"
      fill="currentColor"
      stroke="none"
    />
  {:else if name === "refresh"}
    <path d="M4 10a6 6 0 0 1 10.24-4.24" />
    <polyline points="14.5 2.5 14.5 6.5 10.5 6.5" />
    <path d="M16 10a6 6 0 0 1-10.24 4.24" />
    <polyline points="5.5 17.5 5.5 13.5 9.5 13.5" />
  {/if}
</svg>
