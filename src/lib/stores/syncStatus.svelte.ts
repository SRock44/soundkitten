/** Drives a small persistent status strip so background syncing/rate-limiting
 * is visible instead of silent "why does this look stuck" hangs. */
class SyncStatusStore {
  message = $state<string | null>(null);
  kind = $state<"syncing" | "rate-limited" | "error">("syncing");
  private hideTimer: ReturnType<typeof setTimeout> | undefined;

  syncing(message = "Syncing...") {
    clearTimeout(this.hideTimer);
    this.kind = "syncing";
    this.message = message;
  }

  rateLimited(message = "SoundCloud is rate-limiting requests right now -- showing cached data.") {
    this.kind = "rate-limited";
    this.message = message;
    this.autoHide(6000);
  }

  error(message: string) {
    this.kind = "error";
    this.message = message;
    this.autoHide(6000);
  }

  done() {
    clearTimeout(this.hideTimer);
    this.message = null;
  }

  private autoHide(ms: number) {
    clearTimeout(this.hideTimer);
    this.hideTimer = setTimeout(() => (this.message = null), ms);
  }
}

export const syncStatus = new SyncStatusStore();
