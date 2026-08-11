import { describe, expect, it } from "vitest";
import { computeSeason, computeTime } from "./environment";

const NYC = "America/New_York";
const SYDNEY = "Australia/Sydney";

describe("computeSeason", () => {
  it("returns the correct meteorological season for the northern hemisphere", () => {
    expect(computeSeason(new Date(Date.UTC(2026, 0, 15, 17)), NYC)).toBe("winter"); // Jan
    expect(computeSeason(new Date(Date.UTC(2026, 1, 15, 17)), NYC)).toBe("winter"); // Feb
    expect(computeSeason(new Date(Date.UTC(2026, 2, 15, 17)), NYC)).toBe("spring"); // Mar
    expect(computeSeason(new Date(Date.UTC(2026, 4, 15, 17)), NYC)).toBe("spring"); // May
    expect(computeSeason(new Date(Date.UTC(2026, 5, 15, 17)), NYC)).toBe("summer"); // Jun
    expect(computeSeason(new Date(Date.UTC(2026, 7, 15, 17)), NYC)).toBe("summer"); // Aug
    expect(computeSeason(new Date(Date.UTC(2026, 8, 15, 17)), NYC)).toBe("fall"); // Sep
    expect(computeSeason(new Date(Date.UTC(2026, 10, 15, 17)), NYC)).toBe("fall"); // Nov
    expect(computeSeason(new Date(Date.UTC(2026, 11, 15, 17)), NYC)).toBe("winter"); // Dec
  });

  it("flips to the opposite season in the southern hemisphere", () => {
    expect(computeSeason(new Date(Date.UTC(2026, 0, 15, 3)), SYDNEY)).toBe("summer"); // Jan is summer in Sydney
    expect(computeSeason(new Date(Date.UTC(2026, 5, 15, 3)), SYDNEY)).toBe("winter"); // Jun is winter in Sydney
    expect(computeSeason(new Date(Date.UTC(2026, 2, 15, 3)), SYDNEY)).toBe("fall");
    expect(computeSeason(new Date(Date.UTC(2026, 8, 15, 3)), SYDNEY)).toBe("spring");
  });

  it("does not flip for a northern-hemisphere timezone", () => {
    expect(computeSeason(new Date(Date.UTC(2026, 0, 15, 17)), NYC)).toBe("winter");
    expect(computeSeason(new Date(Date.UTC(2026, 0, 15, 17)), "Europe/London")).toBe("winter");
  });

  it("falls back to the northern-hemisphere mapping (using the device's own local date) for an invalid timezone string, instead of throwing", () => {
    expect(() => computeSeason(new Date(), "Not/A_Real_Zone")).not.toThrow();
  });
});

describe("computeTime", () => {
  it("buckets a known local hour in the given timezone correctly", () => {
    // 17:00 UTC is 12:00 (noon) in America/New_York during EST (UTC-5, mid-January -- no DST ambiguity).
    expect(computeTime(new Date(Date.UTC(2026, 0, 15, 17, 0, 0)), NYC)).toBe("day");
    // 08:00 UTC is 03:00 local NYC -- solidly night.
    expect(computeTime(new Date(Date.UTC(2026, 0, 15, 8, 0, 0)), NYC)).toBe("night");
    // 12:00 UTC is 07:00 local NYC -- the sunrise bucket (6-9).
    expect(computeTime(new Date(Date.UTC(2026, 0, 15, 12, 0, 0)), NYC)).toBe("sunrise");
    // 23:00 UTC is 18:00 local NYC -- the sunset bucket (18-21).
    expect(computeTime(new Date(Date.UTC(2026, 0, 15, 23, 0, 0)), NYC)).toBe("sunset");
  });

  it("the same instant buckets differently in a different timezone", () => {
    // 12:00 UTC is 07:00 in NYC (sunrise) but 22:00 in Tokyo (night) the same moment.
    const instant = new Date(Date.UTC(2026, 0, 15, 12, 0, 0));
    expect(computeTime(instant, NYC)).toBe("sunrise");
    expect(computeTime(instant, "Asia/Tokyo")).toBe("night");
  });

  it("falls back to the device's own local hour for an invalid timezone string, instead of throwing", () => {
    const now = new Date();
    expect(() => computeTime(now, "Not/A_Real_Zone")).not.toThrow();
    expect(computeTime(now, "Not/A_Real_Zone")).toBe(computeTime(now, Intl.DateTimeFormat().resolvedOptions().timeZone));
  });
});
