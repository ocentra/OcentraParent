export function confidenceFor(routeSurfaceKind: string, hostShape: string, pathDepth: string) {
  if (hostShape === 'unknown' || pathDepth === 'unknown' || routeSurfaceKind === 'unknown-route') {
    return 'unknown';
  }
  if (routeSurfaceKind === 'home-route' || routeSurfaceKind === 'catalog-route') {
    return 'medium';
  }
  return 'high';
}
