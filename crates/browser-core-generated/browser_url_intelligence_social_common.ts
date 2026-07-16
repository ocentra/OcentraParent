import type {
  BrowserUrlShapeConfidence,
  BrowserUrlShapePlatform,
  BrowserUrlShapePlatformIds,
  BrowserUrlShapeReasonCode,
  BrowserUrlShapeTargetKind,
  ParsedUrlShape,
} from './browser_url_intelligence';

export function videoShape(
  platform: BrowserUrlShapePlatform,
  videoId: string,
  reasonCodes: readonly BrowserUrlShapeReasonCode[]
): ParsedUrlShape {
  return {
    targetKind: 'video',
    platform,
    platformIds: { ...emptyPlatformIds(), videoId },
    confidence: 'high',
    reasonCodes,
  };
}

export function socialPostShape(
  targetKind: BrowserUrlShapeTargetKind,
  platform: BrowserUrlShapePlatform,
  postId: string,
  confidence: BrowserUrlShapeConfidence
): ParsedUrlShape {
  return {
    targetKind,
    platform,
    platformIds: {
      ...emptyPlatformIds(),
      postId,
      videoId: targetKind === 'video' || targetKind === 'short-video' ? postId : null,
    },
    confidence,
    reasonCodes: ['parsed-url', 'parsed-post-id', 'parsed-social-route'],
  };
}

export function channelShape(
  platform: BrowserUrlShapePlatform,
  channelId: string,
  confidence: BrowserUrlShapeConfidence
): ParsedUrlShape {
  return {
    targetKind: 'channel',
    platform,
    platformIds: { ...emptyPlatformIds(), channelId },
    confidence,
    reasonCodes: ['parsed-url', 'parsed-channel-id', 'parsed-social-route'],
  };
}

export function searchShape(platform: BrowserUrlShapePlatform, query: string): ParsedUrlShape {
  return {
    targetKind: 'search',
    platform,
    platformIds: { ...emptyPlatformIds(), query },
    confidence: 'medium',
    reasonCodes: ['parsed-url', 'parsed-search-query', 'parsed-social-route'],
  };
}

export function socialRouteShape(
  targetKind: BrowserUrlShapeTargetKind,
  platform: BrowserUrlShapePlatform,
  confidence: BrowserUrlShapeConfidence,
  reasonCodes: readonly BrowserUrlShapeReasonCode[]
): ParsedUrlShape {
  return { targetKind, platform, platformIds: emptyPlatformIds(), confidence, reasonCodes };
}

export function simpleShape(
  targetKind: BrowserUrlShapeTargetKind,
  platform: BrowserUrlShapePlatform,
  confidence: BrowserUrlShapeConfidence,
  reasonCodes: readonly BrowserUrlShapeReasonCode[]
): ParsedUrlShape {
  return { targetKind, platform, platformIds: emptyPlatformIds(), confidence, reasonCodes };
}

export function emptyPlatformIds(): BrowserUrlShapePlatformIds {
  return {
    videoId: null,
    channelId: null,
    playlistId: null,
    postId: null,
    query: null,
  };
}
