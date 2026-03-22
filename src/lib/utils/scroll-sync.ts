export interface ScrollMetrics {
  scrollTop: number;
  scrollHeight: number;
  clientHeight: number;
}

function clampUnit(value: number): number {
  if (value <= 0) return 0;
  if (value >= 1) return 1;
  return value;
}

function maxScrollable(scrollHeight: number, clientHeight: number): number {
  return Math.max(scrollHeight - clientHeight, 0);
}

export function calculateScrollProgress({
  scrollTop,
  scrollHeight,
  clientHeight,
}: ScrollMetrics): number {
  const maxScroll = maxScrollable(scrollHeight, clientHeight);
  if (maxScroll === 0) return 0;
  return clampUnit(scrollTop / maxScroll);
}

export function mapScrollToTarget(
  source: ScrollMetrics,
  targetScrollHeight: number,
  targetClientHeight: number,
): number {
  const targetMaxScroll = maxScrollable(targetScrollHeight, targetClientHeight);
  if (targetMaxScroll === 0) return 0;
  return calculateScrollProgress(source) * targetMaxScroll;
}
