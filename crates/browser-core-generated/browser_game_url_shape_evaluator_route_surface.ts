import type { routeHintsFor } from './browser_game_url_shape_evaluator_route_hints';

export function routeSurfaceKindFor(segments: ReadonlyArray<unknown>, routeHints: ReturnType<typeof routeHintsFor>) {
  if (routeHints.hasCloudSessionHint) {
    return 'cloud-session-route';
  }
  if (routeHints.hasEmbedHint) {
    return 'embed-route';
  }
  if (routeHints.hasPlayHint) {
    return 'play-route';
  }
  if (routeHints.hasPurchaseHint) {
    return 'purchase-route';
  }
  if (routeHints.hasAccountHint) {
    return 'account-route';
  }
  if (segments.length === 0) {
    return 'home-route';
  }
  if (segments.length <= 2) {
    return 'catalog-route';
  }
  return 'game-detail-route';
}
