export function estimateScopeToggleTextWidth(text: string, fontSize: number): number {
  return Math.ceil(text.length * fontSize * 0.62);
}

export function roundedRectPath(x: number, y: number, width: number, height: number, radius: number): string {
  return roundedRectPathByCorner(x, y, width, height, radius, radius, radius, radius);
}

export function roundedRectPathByCorner(
  x: number,
  y: number,
  width: number,
  height: number,
  topLeftRadius: number,
  topRightRadius: number,
  bottomRightRadius: number,
  bottomLeftRadius: number
): string {
  const safeWidth = Math.max(0, width);
  const safeHeight = Math.max(0, height);
  const maxRadius = Math.min(safeWidth * 0.5, safeHeight * 0.5);
  const tl = Math.min(topLeftRadius, maxRadius);
  const tr = Math.min(topRightRadius, maxRadius);
  const br = Math.min(bottomRightRadius, maxRadius);
  const bl = Math.min(bottomLeftRadius, maxRadius);
  const right = x + safeWidth;
  const bottom = y + safeHeight;

  return `M${x + tl} ${y}H${right - tr}C${right - tr * 0.45} ${y} ${right} ${y + tr * 0.45} ${right} ${y + tr}V${bottom - br}C${right} ${bottom - br * 0.45} ${right - br * 0.45} ${bottom} ${right - br} ${bottom}H${x + bl}C${x + bl * 0.45} ${bottom} ${x} ${bottom - bl * 0.45} ${x} ${bottom - bl}V${y + tl}C${x} ${y + tl * 0.45} ${x + tl * 0.45} ${y} ${x + tl} ${y}Z`;
}
