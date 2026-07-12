import type { BrowserGameParsedUrl } from './browser_game_url_shape_evaluator';

export function pathSegmentsFor(parsedUrl: BrowserGameParsedUrl) {
  return parsedUrl.pathname
    .split('/')
    .map((segment) => segment.trim().toLowerCase())
    .filter((segment) => segment.length > 0);
}

export function pathDepthFor(segments: ReadonlyArray<unknown>) {
  if (segments.length === 0) {
    return 'root';
  }
  if (segments.length === 1) {
    return 'one-segment';
  }
  if (segments.length === 2) {
    return 'two-segments';
  }
  return 'three-or-more-segments';
}

export function routeHintsFor(segments: ReadonlyArray<unknown>) {
  const normalizedSegments = segments.filter((segment): segment is string => typeof segment === 'string');
  return {
    hasEmbedHint: normalizedSegments.some((segment) => segment === 'embed' || segment === 'iframe'),
    hasPlayHint: normalizedSegments.some((segment) => segment === 'play' || segment === 'launch'),
    hasAccountHint: normalizedSegments.some(
      (segment) => segment === 'account' || segment === 'login' || segment === 'signup'
    ),
    hasPurchaseHint: normalizedSegments.some(
      (segment) => segment === 'buy' || segment === 'store' || segment === 'checkout'
    ),
    hasCloudSessionHint: normalizedSegments.some(
      (segment) => segment === 'cloud' || segment === 'stream' || segment === 'session'
    ),
  };
}

export function segmentLooksLikeGameId(segment: unknown): boolean {
  return typeof segment === 'string' && segment.length >= 4 && (/\d/.test(segment) || segment.includes('-'));
}
