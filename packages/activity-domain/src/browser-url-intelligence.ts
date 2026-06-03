import {
  BrowserUrlShapeSchemaVersion,
  decodeBrowserUrlShapeClassificationResult,
  type BrowserUrlShapeClassificationResult,
  type BrowserUrlShapePlatform,
  type BrowserUrlShapeReasonCode,
  type BrowserUrlShapeTargetKind,
} from './browser-url-intelligence-schemas';

type BrowserUrlShapeSourceKind = BrowserUrlShapeClassificationResult['sourceKind'];

type BrowserUrlShapeParserInput = {
  readonly classificationId: string;
  readonly classifiedAt: string;
  readonly sourceEvidenceIds: readonly [string, ...string[]];
  readonly sourceKind: BrowserUrlShapeSourceKind;
  readonly url: string | null;
  readonly title?: string | null;
};

type ParsedUrlShape = {
  readonly targetKind: BrowserUrlShapeTargetKind;
  readonly platform: BrowserUrlShapePlatform;
  readonly platformIds: BrowserUrlShapeClassificationResult['platformIds'];
  readonly confidence: BrowserUrlShapeClassificationResult['confidence'];
  readonly reasonCodes: readonly BrowserUrlShapeReasonCode[];
};

export function parseBrowserUrlShape(input: BrowserUrlShapeParserInput): BrowserUrlShapeClassificationResult {
  if (input.sourceKind !== 'managed-browser-exact-url') {
    return decodeBrowserUrlShapeClassificationResult({
      ...baseResult(input),
      url: null,
      domain: null,
      title: null,
      targetKind: 'unknown',
      platform: 'unknown',
      platformIds: emptyPlatformIds(),
      confidence: 'low',
      reasonCodes: [nonExactEvidenceReason(input.sourceKind), 'content-not-inferred'],
      exactUrlEvidence: false,
    });
  }

  const parsed = parseUrl(input.url);
  if (parsed === null) {
    return decodeBrowserUrlShapeClassificationResult({
      ...baseResult(input),
      url: null,
      domain: null,
      title: input.title ?? null,
      targetKind: 'unknown',
      platform: 'unknown',
      platformIds: emptyPlatformIds(),
      confidence: 'low',
      reasonCodes: ['unsupported-scheme', 'content-not-inferred'],
      exactUrlEvidence: false,
    });
  }

  const shape = shapeForParsedUrl(parsed);
  return decodeBrowserUrlShapeClassificationResult({
    ...baseResult(input),
    url: parsed.normalizedUrl,
    domain: parsed.domain,
    title: input.title ?? null,
    ...shape,
    reasonCodes: [...shape.reasonCodes, 'content-not-inferred'],
    exactUrlEvidence: true,
  });
}

function baseResult(input: BrowserUrlShapeParserInput) {
  return {
    schemaVersion: BrowserUrlShapeSchemaVersion,
    classificationId: input.classificationId,
    classifiedAt: input.classifiedAt,
    sourceEvidenceIds: input.sourceEvidenceIds,
    contentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    sourceKind: input.sourceKind,
  };
}

function shapeForParsedUrl(parsed: ParsedBrowserUrl): ParsedUrlShape {
  if (parsed.domain === 'youtube.com' || parsed.domain === 'www.youtube.com' || parsed.domain === 'm.youtube.com') {
    return youtubeShape(parsed);
  }
  if (parsed.domain === 'youtu.be') {
    const videoId = firstPathSegment(parsed);
    if (videoId !== null) {
      return videoShape('youtube', videoId, ['parsed-url', 'parsed-youtube-video-id']);
    }
  }
  if (parsed.domain === 'vimeo.com' || parsed.domain.endsWith('.vimeo.com')) {
    const segments = pathSegments(parsed);
    const videoId = segments[0] === 'video' ? (segments[1] ?? null) : (segments[0] ?? null);
    if (videoId !== null && /^[0-9]+$/.test(videoId)) {
      return videoShape('vimeo', videoId, ['parsed-url']);
    }
  }
  if (parsed.domain === 'tiktok.com' || parsed.domain.endsWith('.tiktok.com')) {
    return tiktokShape(parsed);
  }
  if (parsed.domain === 'instagram.com' || parsed.domain.endsWith('.instagram.com')) {
    return instagramShape(parsed);
  }
  if (parsed.domain === 'facebook.com' || parsed.domain.endsWith('.facebook.com')) {
    return facebookShape(parsed);
  }
  if (parsed.domain === 'twitch.tv' || parsed.domain.endsWith('.twitch.tv')) {
    return twitchShape(parsed);
  }
  if (
    parsed.domain === 'x.com' ||
    parsed.domain.endsWith('.x.com') ||
    parsed.domain === 'twitter.com' ||
    parsed.domain.endsWith('.twitter.com')
  ) {
    return xTwitterShape(parsed);
  }
  if (parsed.domain === 'reddit.com' || parsed.domain.endsWith('.reddit.com')) {
    return redditShape(parsed);
  }
  if (parsed.domain === 'discord.com' || parsed.domain.endsWith('.discord.com')) {
    return discordShape(parsed);
  }
  return simpleShape('unknown', 'generic-web', 'low', ['parsed-url']);
}

function youtubeShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments[0] === 'shorts' && segments[1] !== undefined) {
    return {
      ...videoShape('youtube-shorts', segments[1], ['parsed-url', 'parsed-youtube-shorts-id']),
      targetKind: 'short-video',
    };
  }
  const videoId = queryParam(parsed, 'v');
  if ((segments[0] === 'watch' || segments.length === 0) && videoId !== null && videoId.length > 0) {
    return videoShape('youtube', videoId, ['parsed-url', 'parsed-youtube-video-id']);
  }
  if ((segments[0] === 'embed' || segments[0] === 'live') && segments[1] !== undefined) {
    return videoShape('youtube', segments[1], ['parsed-url', 'parsed-youtube-video-id']);
  }
  if (
    (segments[0] === 'channel' || segments[0] === 'c' || segments[0] === 'user' || segments[0] === '@') &&
    segments[1] !== undefined
  ) {
    return {
      targetKind: 'channel',
      platform: 'youtube',
      platformIds: { ...emptyPlatformIds(), channelId: segments[1] },
      confidence: 'high',
      reasonCodes: ['parsed-url', 'parsed-channel-id'],
    };
  }
  if (segments[0]?.startsWith('@')) {
    return {
      targetKind: 'channel',
      platform: 'youtube',
      platformIds: { ...emptyPlatformIds(), channelId: segments[0] },
      confidence: 'high',
      reasonCodes: ['parsed-url', 'parsed-channel-id'],
    };
  }
  if (segments[0] === 'playlist') {
    const playlistId = queryParam(parsed, 'list');
    if (playlistId !== null && playlistId.length > 0) {
      return {
        targetKind: 'playlist',
        platform: 'youtube',
        platformIds: { ...emptyPlatformIds(), playlistId },
        confidence: 'high',
        reasonCodes: ['parsed-url', 'parsed-playlist-id'],
      };
    }
  }
  if (segments[0] === 'results') {
    const query = queryParam(parsed, 'search_query');
    if (query !== null && query.length > 0) {
      return {
        targetKind: 'search',
        platform: 'youtube',
        platformIds: { ...emptyPlatformIds(), query },
        confidence: 'high',
        reasonCodes: ['parsed-url', 'parsed-search-query'],
      };
    }
  }
  return simpleShape('unknown', 'youtube', 'low', ['parsed-url']);
}

function tiktokShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments[0] === 'upload') {
    return socialRouteShape('social-upload-post', 'tiktok', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  const videoIndex = segments.indexOf('video');
  const videoId = videoIndex >= 0 ? (segments[videoIndex + 1] ?? null) : null;
  if (videoId !== null) {
    return videoShape('tiktok', videoId, ['parsed-url']);
  }
  if (segments[0]?.startsWith('@') && segments[1] === 'live') {
    return socialRouteShape('social-livestream', 'tiktok', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  if (segments[0]?.startsWith('@') && segments.length === 1) {
    return channelShape('tiktok', segments[0], 'medium');
  }
  return socialRouteShape('social-feed', 'tiktok', 'medium', ['parsed-url', 'dynamic-feed', 'parsed-social-route']);
}

function instagramShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments.length === 0 || ['explore', 'reels'].includes(segments[0] ?? '')) {
    return socialRouteShape('social-feed', 'instagram', 'medium', [
      'parsed-url',
      'dynamic-feed',
      'parsed-social-route',
    ]);
  }
  if (segments[0] === 'create') {
    return socialRouteShape('social-upload-post', 'instagram', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  if (segments[0] === 'live') {
    return socialRouteShape('social-livestream', 'instagram', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  if ((segments[0] === 'reel' || segments[0] === 'reels') && segments[1] !== undefined) {
    return socialPostShape('short-video', 'instagram', segments[1], 'high');
  }
  if ((segments[0] === 'p' || segments[0] === 'tv') && segments[1] !== undefined) {
    return socialPostShape('social-post', 'instagram', segments[1], 'medium');
  }
  if (segments[0] === 'stories') {
    return socialRouteShape('social-feed', 'instagram', 'medium', [
      'parsed-url',
      'dynamic-feed',
      'parsed-social-route',
    ]);
  }
  if (segments[0] === 'direct') {
    return socialRouteShape('social-messaging', 'instagram', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  return channelShape('instagram', segments[0] ?? parsed.domain, 'medium');
}

function facebookShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  const watchVideoId = queryParam(parsed, 'v');
  if (segments[0] === 'watch' && watchVideoId !== null) {
    return videoShape('facebook', watchVideoId, ['parsed-url']);
  }
  if ((segments[0] === 'reel' || segments[0] === 'videos') && segments[1] !== undefined) {
    return socialPostShape(segments[0] === 'reel' ? 'short-video' : 'video', 'facebook', segments[1], 'high');
  }
  if (segments.length === 0 || ['home', 'feed', 'watch'].includes(segments[0] ?? '')) {
    return socialRouteShape('social-feed', 'facebook', 'medium', ['parsed-url', 'dynamic-feed', 'parsed-social-route']);
  }
  if (segments[0] === 'live') {
    return socialRouteShape('social-livestream', 'facebook', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  return channelShape('facebook', segments[0] ?? parsed.domain, 'medium');
}

function twitchShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments[0] === 'videos' && segments[1] !== undefined) {
    return videoShape('twitch', segments[1], ['parsed-url']);
  }
  if (segments[0] === 'directory' || segments[0] === 'following') {
    return socialRouteShape('social-feed', 'twitch', 'medium', ['parsed-url', 'dynamic-feed', 'parsed-social-route']);
  }
  if (segments.length > 0) {
    return channelShape('twitch', segments[0] ?? parsed.domain, 'medium');
  }
  return socialRouteShape('social-feed', 'twitch', 'medium', ['parsed-url', 'dynamic-feed', 'parsed-social-route']);
}

function xTwitterShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments[0] === 'search') {
    const query = queryParam(parsed, 'q');
    if (query !== null) {
      return searchShape('x-twitter', query);
    }
  }
  if (segments[1] === 'status' && segments[2] !== undefined) {
    return socialPostShape('social-post', 'x-twitter', segments[2], 'high');
  }
  if (segments.length === 0 || ['home', 'explore'].includes(segments[0] ?? '')) {
    return socialRouteShape('social-feed', 'x-twitter', 'medium', [
      'parsed-url',
      'dynamic-feed',
      'parsed-social-route',
    ]);
  }
  return channelShape('x-twitter', segments[0] ?? parsed.domain, 'medium');
}

function redditShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments[0] === 'r' && segments[1] !== undefined && segments[2] === 'comments' && segments[3] !== undefined) {
    return socialPostShape('social-post', 'reddit', segments[3], 'medium');
  }
  if (segments[0] === 'r' && segments[1] !== undefined) {
    return {
      targetKind: 'forum',
      platform: 'reddit',
      platformIds: { ...emptyPlatformIds(), channelId: segments[1] },
      confidence: 'medium',
      reasonCodes: ['parsed-url', 'parsed-channel-id', 'parsed-social-route'],
    };
  }
  if (segments.length === 0) {
    return socialRouteShape('social-feed', 'reddit', 'medium', ['parsed-url', 'dynamic-feed', 'parsed-social-route']);
  }
  return simpleShape('unknown', 'reddit', 'low', ['parsed-url', 'manual-required']);
}

function discordShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments[0] === 'channels') {
    return socialRouteShape('social-messaging', 'discord', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  return simpleShape('unknown', 'discord', 'low', ['parsed-url', 'manual-required']);
}

function videoShape(
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

function socialPostShape(
  targetKind: BrowserUrlShapeTargetKind,
  platform: BrowserUrlShapePlatform,
  postId: string,
  confidence: BrowserUrlShapeClassificationResult['confidence']
): ParsedUrlShape {
  return {
    targetKind,
    platform,
    platformIds: { ...emptyPlatformIds(), postId, videoId: isVideoKind(targetKind) ? postId : null },
    confidence,
    reasonCodes: ['parsed-url', 'parsed-post-id', 'parsed-social-route'],
  };
}

function channelShape(
  platform: BrowserUrlShapePlatform,
  channelId: string,
  confidence: BrowserUrlShapeClassificationResult['confidence']
): ParsedUrlShape {
  return {
    targetKind: 'channel',
    platform,
    platformIds: { ...emptyPlatformIds(), channelId },
    confidence,
    reasonCodes: ['parsed-url', 'parsed-channel-id', 'parsed-social-route'],
  };
}

function searchShape(platform: BrowserUrlShapePlatform, query: string): ParsedUrlShape {
  return {
    targetKind: 'search',
    platform,
    platformIds: { ...emptyPlatformIds(), query },
    confidence: 'medium',
    reasonCodes: ['parsed-url', 'parsed-search-query', 'parsed-social-route'],
  };
}

function socialRouteShape(
  targetKind: BrowserUrlShapeTargetKind,
  platform: BrowserUrlShapePlatform,
  confidence: BrowserUrlShapeClassificationResult['confidence'],
  reasonCodes: readonly BrowserUrlShapeReasonCode[]
): ParsedUrlShape {
  return {
    targetKind,
    platform,
    platformIds: emptyPlatformIds(),
    confidence,
    reasonCodes,
  };
}

function simpleShape(
  targetKind: BrowserUrlShapeTargetKind,
  platform: BrowserUrlShapePlatform,
  confidence: BrowserUrlShapeClassificationResult['confidence'],
  reasonCodes: readonly BrowserUrlShapeReasonCode[]
): ParsedUrlShape {
  return {
    targetKind,
    platform,
    platformIds: emptyPlatformIds(),
    confidence,
    reasonCodes,
  };
}

function isVideoKind(value: BrowserUrlShapeTargetKind) {
  return value === 'video' || value === 'short-video';
}

function emptyPlatformIds(): BrowserUrlShapeClassificationResult['platformIds'] {
  return {
    videoId: null,
    channelId: null,
    playlistId: null,
    postId: null,
    query: null,
  };
}

type ParsedBrowserUrl = {
  readonly normalizedUrl: string;
  readonly domain: string;
  readonly path: string;
  readonly query: string | null;
};

function parseUrl(value: string | null): ParsedBrowserUrl | null {
  if (value === null) {
    return null;
  }
  const separatorIndex = value.indexOf('://');
  if (separatorIndex <= 0) {
    return null;
  }
  const scheme = value.slice(0, separatorIndex).toLowerCase();
  if (scheme !== 'http' && scheme !== 'https') {
    return null;
  }
  const remainder = value.slice(separatorIndex + 3);
  const authorityEnd = firstSuffixIndex(remainder);
  const authority = authorityEnd === null ? remainder : remainder.slice(0, authorityEnd);
  if (authority.length === 0 || authority.includes('@')) {
    return null;
  }
  const normalized = normalizedAuthority(authority);
  if (normalized === null) {
    return null;
  }
  const suffix = authorityEnd === null ? '' : remainder.slice(authorityEnd);
  const path = pathFromSuffix(suffix);
  return {
    normalizedUrl: `${scheme}://${normalized.authority}${suffix}`,
    domain: normalized.domain,
    path,
    query: queryFromSuffix(suffix),
  };
}

function firstSuffixIndex(value: string): number | null {
  const indexes = ['/', '?', '#'].map((separator) => value.indexOf(separator)).filter((index) => index >= 0);
  if (indexes.length === 0) {
    return null;
  }
  return Math.min(...indexes);
}

function normalizedAuthority(value: string): { readonly authority: string; readonly domain: string } | null {
  const [host, port] = splitHostAndPort(value);
  const domain = normalizedHost(host);
  if (domain === null) {
    return null;
  }
  return {
    authority: port === null ? domain : `${domain}:${port}`,
    domain,
  };
}

function splitHostAndPort(value: string): readonly [string, string | null] {
  const separatorCount = value.split(':').length - 1;
  if (separatorCount === 1) {
    const separatorIndex = value.lastIndexOf(':');
    const host = value.slice(0, separatorIndex);
    const port = value.slice(separatorIndex + 1);
    if (host.length > 0 && /^[0-9]+$/.test(port)) {
      return [host, port];
    }
  }
  return [value, null];
}

function normalizedHost(value: string) {
  const normalized = value.replace(/\.+$/, '').toLowerCase();
  if (normalized.length === 0 || normalized.includes('/')) {
    return null;
  }
  return normalized;
}

function pathFromSuffix(value: string) {
  if (!value.startsWith('/')) {
    return '/';
  }
  const queryIndex = value.indexOf('?');
  const hashIndex = value.indexOf('#');
  const endIndexes = [queryIndex, hashIndex].filter((index) => index >= 0);
  const endIndex = endIndexes.length === 0 ? value.length : Math.min(...endIndexes);
  return value.slice(0, endIndex);
}

function queryFromSuffix(value: string): string | null {
  const queryStart = value.indexOf('?');
  if (queryStart < 0) {
    return null;
  }
  const hashIndex = value.indexOf('#', queryStart);
  return value.slice(queryStart + 1, hashIndex < 0 ? value.length : hashIndex);
}

function pathSegments(parsed: ParsedBrowserUrl) {
  return parsed.path
    .split('/')
    .map((segment: string) => segment.trim())
    .filter(Boolean);
}

function firstPathSegment(parsed: ParsedBrowserUrl) {
  return pathSegments(parsed)[0] ?? null;
}

function queryParam(parsed: ParsedBrowserUrl, key: string): string | null {
  if (parsed.query === null) {
    return null;
  }
  for (const part of parsed.query.split('&')) {
    const separatorIndex = part.indexOf('=');
    const rawKey = separatorIndex < 0 ? part : part.slice(0, separatorIndex);
    if (rawKey === key) {
      const value = separatorIndex < 0 ? '' : part.slice(separatorIndex + 1);
      return value.length === 0 ? null : value.replaceAll('+', ' ');
    }
  }
  return null;
}

function nonExactEvidenceReason(value: BrowserUrlShapeSourceKind): BrowserUrlShapeReasonCode {
  if (value === 'unmanaged-browser-process') {
    return 'unmanaged-process-only';
  }
  if (value === 'network-domain') {
    return 'network-domain-only';
  }
  return 'no-exact-evidence';
}
