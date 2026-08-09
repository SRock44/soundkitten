/**
 * Cache-first local storage for API responses. The point isn't just speed --
 * it's that every cold app launch used to fire off likes/playlists/me/
 * followings/mixed-selections/feed all at once, every single time, with
 * nothing persisted locally. That burst of concurrent requests on every
 * launch is what got the account rate-limited (429) repeatedly. Caching
 * means most launches can render instantly from what's already on disk and
 * only need a small background refresh, not a full re-fetch of everything.
 */

type CacheEntry<T> = { value: T; savedAt: number };

function safeGet(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function safeSet(key: string, value: string) {
  try {
    localStorage.setItem(key, value);
  } catch {
    // ignore -- caching is a nice-to-have, not something that should crash the app
  }
}

function cacheKey(key: string): string {
  return `sc-desktop:cache:${key}`;
}

export function loadCached<T>(key: string): { value: T; ageMs: number } | null {
  const raw = safeGet(cacheKey(key));
  if (!raw) return null;
  try {
    const entry: CacheEntry<T> = JSON.parse(raw);
    return { value: entry.value, ageMs: Date.now() - entry.savedAt };
  } catch {
    return null;
  }
}

export function saveCached<T>(key: string, value: T) {
  const entry: CacheEntry<T> = { value, savedAt: Date.now() };
  safeSet(cacheKey(key), JSON.stringify(entry));
}

export function clearCached(key: string) {
  try {
    localStorage.removeItem(cacheKey(key));
  } catch {
    // ignore
  }
}

export function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export function isRateLimited(err: unknown): boolean {
  return String(err).includes("429");
}

/**
 * Cache-first fetch: hydrates `apply` from whatever's cached immediately (no
 * network wait), then only actually fetches if the cache is missing or
 * older than `maxAgeMs`. On failure, cached data (if any) is left standing
 * rather than being wiped -- a rate limit or network blip shouldn't clear a
 * screen that already had something to show.
 */
export async function syncWithCache<T>(
  key: string,
  fetcher: () => Promise<T>,
  apply: (value: T) => void,
  opts: { maxAgeMs?: number; onRateLimited?: () => void; onError?: (e: unknown) => void } = {},
): Promise<void> {
  const maxAgeMs = opts.maxAgeMs ?? 5 * 60 * 1000;
  const cached = loadCached<T>(key);
  if (cached) {
    apply(cached.value);
    if (cached.ageMs < maxAgeMs) return; // fresh enough -- skip the network entirely
  }
  try {
    const fresh = await fetcher();
    apply(fresh);
    saveCached(key, fresh);
  } catch (e) {
    if (isRateLimited(e)) opts.onRateLimited?.();
    // Only surface a hard error if there was nothing cached to fall back on --
    // otherwise the user just keeps seeing what they already had.
    else if (!cached) opts.onError?.(e);
  }
}
