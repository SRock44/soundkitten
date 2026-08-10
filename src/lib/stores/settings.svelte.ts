const PERFORMANCE_MODE_KEY = "sc-desktop:performance-mode";

// Same defensive guard as player.svelte.ts's safeStorageGet/Set -- keeps
// this a nice-to-have rather than something that can throw during startup.
function safeStorageGet(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function safeStorageSet(key: string, value: string) {
  try {
    localStorage.setItem(key, value);
  } catch {
    // ignore
  }
}

/**
 * All Tauri windows in this app share the same webview profile (same
 * mechanism the login windows already rely on for cookie sharing), so
 * localStorage is naturally shared across windows too -- the mini player
 * reads this same key directly on mount, no cross-window event needed for
 * this one setting.
 */
class SettingsStore {
  performanceMode = $state(safeStorageGet(PERFORMANCE_MODE_KEY) === "true");

  setPerformanceMode(on: boolean) {
    this.performanceMode = on;
    safeStorageSet(PERFORMANCE_MODE_KEY, String(on));
  }
}

export const settings = new SettingsStore();
