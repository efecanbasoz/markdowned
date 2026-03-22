import { describe, it, expect } from "vitest";
import { clampSize, calculatePanelSize } from "../src/lib/utils/resize";

describe("clampSize", () => {
  it("returns value when within range", () => {
    expect(clampSize(150, 100, 500)).toBe(150);
  });

  it("returns min when value is below min", () => {
    expect(clampSize(50, 100, 500)).toBe(100);
  });

  it("returns max when value is above max", () => {
    expect(clampSize(600, 100, 500)).toBe(500);
  });

  it("handles min equals max", () => {
    expect(clampSize(200, 150, 150)).toBe(150);
  });
});

describe("calculatePanelSize", () => {
  // Container starts at x=0, width=1000, min=120, max=600
  it("returns mouse offset from container start", () => {
    const result = calculatePanelSize(300, 0, 1000, 120, 600);
    expect(result).toBe(300);
  });

  it("clamps to minimum size", () => {
    const result = calculatePanelSize(50, 0, 1000, 120, 600);
    expect(result).toBe(120);
  });

  it("clamps to maximum size", () => {
    const result = calculatePanelSize(800, 0, 1000, 120, 600);
    expect(result).toBe(600);
  });

  it("accounts for container offset", () => {
    // Container starts at x=200, mouse at 400 → offset = 200
    const result = calculatePanelSize(400, 200, 1000, 120, 600);
    expect(result).toBe(200);
  });

  it("clamps when offset minus container start is below min", () => {
    // Container starts at x=200, mouse at 250 → offset = 50, min = 120
    const result = calculatePanelSize(250, 200, 1000, 120, 600);
    expect(result).toBe(120);
  });

  it("uses containerSize-based max when maxPx exceeds container", () => {
    // Container is 400px, max is 600 → effective max should be 400 * 0.8 = 320
    const result = calculatePanelSize(380, 0, 400, 100, 600);
    expect(result).toBe(320);
  });
});
