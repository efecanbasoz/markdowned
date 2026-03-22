import { describe, expect, it } from "vitest";
import {
  calculateScrollProgress,
  mapScrollToTarget,
  type ScrollMetrics,
} from "../src/lib/utils/scroll-sync";

describe("calculateScrollProgress", () => {
  it("returns zero when nothing can scroll", () => {
    const source: ScrollMetrics = { scrollTop: 50, scrollHeight: 400, clientHeight: 400 };
    expect(calculateScrollProgress(source)).toBe(0);
  });

  it("returns normalized progress within the scroll range", () => {
    const source: ScrollMetrics = { scrollTop: 150, scrollHeight: 500, clientHeight: 200 };
    expect(calculateScrollProgress(source)).toBeCloseTo(0.5);
  });

  it("clamps progress to one when scrollTop exceeds the range", () => {
    const source: ScrollMetrics = { scrollTop: 900, scrollHeight: 500, clientHeight: 200 };
    expect(calculateScrollProgress(source)).toBe(1);
  });
});

describe("mapScrollToTarget", () => {
  it("maps source progress into the target scroll range", () => {
    const source: ScrollMetrics = { scrollTop: 150, scrollHeight: 500, clientHeight: 200 };
    expect(mapScrollToTarget(source, 1000, 400)).toBeCloseTo(300);
  });

  it("returns zero when the target cannot scroll", () => {
    const source: ScrollMetrics = { scrollTop: 150, scrollHeight: 500, clientHeight: 200 };
    expect(mapScrollToTarget(source, 400, 400)).toBe(0);
  });
});
