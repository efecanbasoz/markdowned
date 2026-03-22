/** Clamp a value between min and max. */
export function clampSize(value: number, min: number, max: number): number {
  if (value < min) return min;
  if (value > max) return max;
  return value;
}

/**
 * Calculate panel size from a mouse position.
 * Returns the pixel size for the panel, clamped between minPx and
 * the lesser of maxPx or 80% of containerSize.
 */
export function calculatePanelSize(
  mousePos: number,
  containerStart: number,
  containerSize: number,
  minPx: number,
  maxPx: number,
): number {
  const offset = mousePos - containerStart;
  const effectiveMax = Math.min(maxPx, containerSize * 0.8);
  return clampSize(offset, minPx, effectiveMax);
}
