import type { Time, Weather as Season } from "./types";

// Same defensive guard as player.svelte.ts/settings.svelte.ts's
// safeStorageGet/Set -- keeps persistence a nice-to-have, not something
// that can throw during startup.
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

const TIMEZONE_KEY = "sc-desktop:home-banner-timezone";

/**
 * No geolocation permission is ever requested for this -- `Intl`'s own
 * resolved timezone (the OS/browser's configured zone, e.g.
 * "America/New_York") is available with zero permission prompt and reveals
 * far less than GPS coordinates would. Users can override it manually (see
 * HomeBannerGear.svelte) if they want something different or more precise;
 * this is just the starting default.
 */
export function loadTimezone(): string {
  const saved = safeStorageGet(TIMEZONE_KEY);
  if (saved) return saved;
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone;
  } catch {
    return "UTC";
  }
}

export function saveTimezone(timezone: string) {
  safeStorageSet(TIMEZONE_KEY, timezone);
}

export interface TimezoneOption {
  value: string;
  label: string;
}

/**
 * Curated by population, not an exhaustive IANA dump -- picking a timezone
 * for a decorative day/night cat scene doesn't need all ~400 zones (nobody
 * is choosing Antarctica/McMurdo here). Roughly one representative zone per
 * major population center/offset, city-labeled since a raw "America/New_York"
 * means less to most people than "New York (EDT)".
 */
const TIMEZONE_PRESETS: { zone: string; city: string }[] = [
  { zone: "UTC", city: "UTC" },
  { zone: "America/Los_Angeles", city: "Los Angeles" },
  { zone: "America/Denver", city: "Denver" },
  { zone: "America/Chicago", city: "Chicago" },
  { zone: "America/New_York", city: "New York" },
  { zone: "America/Mexico_City", city: "Mexico City" },
  { zone: "America/Sao_Paulo", city: "São Paulo" },
  { zone: "Europe/London", city: "London" },
  { zone: "Europe/Paris", city: "Paris / Berlin" },
  { zone: "Europe/Moscow", city: "Moscow" },
  { zone: "Africa/Lagos", city: "Lagos" },
  { zone: "Africa/Cairo", city: "Cairo" },
  { zone: "Europe/Istanbul", city: "Istanbul" },
  { zone: "Asia/Dubai", city: "Dubai" },
  { zone: "Asia/Karachi", city: "Karachi" },
  { zone: "Asia/Kolkata", city: "India" },
  { zone: "Asia/Dhaka", city: "Dhaka" },
  { zone: "Asia/Bangkok", city: "Bangkok / Jakarta" },
  { zone: "Asia/Shanghai", city: "China" },
  { zone: "Asia/Tokyo", city: "Tokyo" },
  { zone: "Asia/Seoul", city: "Seoul" },
  { zone: "Australia/Sydney", city: "Sydney" },
];

/** Current abbreviation for a zone (e.g. "EDT", "CET") -- computed live so it always reflects today's DST state rather than a hardcoded guess. */
function abbreviationFor(timezone: string): string | undefined {
  try {
    const parts = new Intl.DateTimeFormat("en-US", { timeZone: timezone, timeZoneName: "short" }).formatToParts(new Date());
    return parts.find((p) => p.type === "timeZoneName")?.value;
  } catch {
    return undefined;
  }
}

/** Short, population-weighted timezone picklist for the settings gear -- city name plus its live abbreviation, e.g. "New York (EDT)". */
export function listTimezones(): TimezoneOption[] {
  return TIMEZONE_PRESETS.map(({ zone, city }) => {
    const abbr = abbreviationFor(zone);
    return { value: zone, label: abbr && abbr !== city ? `${city} (${abbr})` : city };
  });
}

function localHourInTimezone(now: Date, timezone: string): number {
  try {
    const parts = new Intl.DateTimeFormat("en-US", { timeZone: timezone, hour: "numeric", hourCycle: "h23" }).formatToParts(now);
    const hour = parts.find((p) => p.type === "hour")?.value;
    return hour !== undefined ? Number(hour) : now.getHours();
  } catch {
    return now.getHours(); // invalid/unrecognized timezone string -- fall back to the device's own local hour rather than throwing
  }
}

function localMonthInTimezone(now: Date, timezone: string): number {
  try {
    const parts = new Intl.DateTimeFormat("en-US", { timeZone: timezone, month: "numeric" }).formatToParts(now);
    const month = parts.find((p) => p.type === "month")?.value;
    return month !== undefined ? Number(month) - 1 : now.getMonth(); // 0-11
  } catch {
    return now.getMonth();
  }
}

/** Simple fixed local-hour buckets -- without coordinates there's no sunrise/sunset math to do, just a reasonable approximation, same spirit as Home.svelte's own greeting(). */
export function computeTime(now: Date, timezone: string): Time {
  const hour = localHourInTimezone(now, timezone);
  if (hour < 6) return "night";
  if (hour < 9) return "sunrise";
  if (hour < 18) return "day";
  if (hour < 21) return "sunset";
  return "night";
}

/**
 * IANA zone prefixes for places actually in the southern hemisphere --
 * used only to flip the season mapping below. Deliberately conservative
 * (skips near-equatorial zones like Lima/Nairobi/Kinshasa, where seasonal
 * swing is minimal anyway and hemisphere is a toss-up) rather than
 * exhaustive; a few edge cases being "wrong" here just means an
 * occasionally-mismatched decorative season, not a functional bug.
 */
const SOUTHERN_HEMISPHERE_PREFIXES = [
  "Australia/",
  "Antarctica/",
  "Pacific/Auckland",
  "Pacific/Chatham",
  "Pacific/Fiji",
  "Pacific/Noumea",
  "Pacific/Guadalcanal",
  "Pacific/Port_Moresby",
  "Pacific/Tongatapu",
  "Pacific/Apia",
  "Pacific/Norfolk",
  "America/Argentina/",
  "America/Sao_Paulo",
  "America/Santiago",
  "America/Asuncion",
  "America/Montevideo",
  "America/La_Paz",
  "Africa/Johannesburg",
  "Africa/Windhoek",
  "Africa/Maputo",
  "Africa/Harare",
  "Africa/Gaborone",
  "Indian/Antananarivo",
  "Indian/Mauritius",
  "Indian/Reunion",
];

function isSouthernHemisphere(timezone: string): boolean {
  return SOUTHERN_HEMISPHERE_PREFIXES.some((prefix) => timezone.startsWith(prefix));
}

/** Meteorological season (Mar/Jun/Sep/Dec boundaries) from the calendar date in the given timezone, flipped for the southern hemisphere. */
export function computeSeason(now: Date, timezone: string): Season {
  const month = localMonthInTimezone(now, timezone);
  const northern: Season = month <= 1 || month === 11 ? "winter" : month <= 4 ? "spring" : month <= 7 ? "summer" : "fall";
  if (!isSouthernHemisphere(timezone)) return northern;
  const flipped: Record<Season, Season> = { winter: "summer", summer: "winter", spring: "fall", fall: "spring" };
  return flipped[northern];
}
