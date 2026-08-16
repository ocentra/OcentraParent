import type { ParsedBrowserUrl, ParsedUrlShape } from './browser_url_intelligence';
import { queryParam } from './browser_url_intelligence_query';
import { pathSegments } from './browser_url_intelligence_url_text';
import {
  channelShape,
  emptyPlatformIds,
  searchShape,
  simpleShape,
  socialPostShape,
  socialRouteShape,
} from './browser_url_intelligence_social_common';

export function xTwitterShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
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

export function redditShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
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

export function discordShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments[0] === 'channels') {
    return socialRouteShape('social-messaging', 'discord', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  return simpleShape('unknown', 'discord', 'low', ['parsed-url', 'manual-required']);
}
