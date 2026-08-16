export function fingerprintFor(parts: {
  protocolShape: string;
  hostShape: string;
  pathDepth: string;
  routeSurfaceKind: string;
  hasGameIdLikeSegment: boolean;
  hasQueryShape: boolean;
  hasFragmentShape: boolean;
  hasEmbedHint: boolean;
  hasPlayHint: boolean;
  hasAccountHint: boolean;
  hasPurchaseHint: boolean;
  hasCloudSessionHint: boolean;
}) {
  return [
    'url-shape',
    parts.protocolShape,
    parts.hostShape,
    parts.pathDepth,
    parts.routeSurfaceKind,
    parts.hasGameIdLikeSegment ? 'game-id-like' : 'no-game-id',
    parts.hasQueryShape ? 'query' : 'no-query',
    parts.hasFragmentShape ? 'fragment' : 'no-fragment',
    parts.hasEmbedHint ? 'embed' : 'no-embed',
    parts.hasPlayHint ? 'play' : 'no-play',
    parts.hasAccountHint ? 'account' : 'no-account',
    parts.hasPurchaseHint ? 'purchase' : 'no-purchase',
    parts.hasCloudSessionHint ? 'cloud-session' : 'no-cloud-session',
  ].join(':');
}
