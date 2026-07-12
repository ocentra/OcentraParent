import type { ParsedBrowserUrl, ParsedUrlShape } from './browser_url_intelligence';
import { queryParam } from './browser_url_intelligence_query';
import { pathSegments } from './browser_url_intelligence_url_text';
import { channelShape, socialPostShape, socialRouteShape, videoShape } from './browser_url_intelligence_social_common';

export function facebookShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
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

export function twitchShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
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
