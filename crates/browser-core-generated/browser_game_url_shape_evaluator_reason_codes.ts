import type { routeHintsFor } from './browser_game_url_shape_evaluator_route_hints';

export function reasonCodesFor(routeSurfaceKind: string, routeHints: ReturnType<typeof routeHintsFor>) {
  if (routeHints.hasCloudSessionHint || routeSurfaceKind === 'cloud-session-route') {
    return ['cloud-session-hint'] as const;
  }
  if (routeHints.hasEmbedHint || routeSurfaceKind === 'embed-route') {
    return ['embed-route-hint'] as const;
  }
  if (routeHints.hasPlayHint || routeSurfaceKind === 'play-route') {
    return ['game-route-hint'] as const;
  }
  if (routeHints.hasPurchaseHint || routeSurfaceKind === 'purchase-route') {
    return ['purchase-route-hint'] as const;
  }
  if (routeHints.hasAccountHint || routeSurfaceKind === 'account-route') {
    return ['account-route-hint'] as const;
  }
  return ['catalog-route-hint'] as const;
}
