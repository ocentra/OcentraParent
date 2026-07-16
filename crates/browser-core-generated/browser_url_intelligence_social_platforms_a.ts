import type { ParsedBrowserUrl, ParsedUrlShape } from './browser_url_intelligence';
import { pathSegments } from './browser_url_intelligence_url_text';
import { channelShape, socialPostShape, socialRouteShape, videoShape } from './browser_url_intelligence_social_common';

export function tiktokShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
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

const instagramFeedSegments = ['explore', 'reels'] as const;

export function instagramShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  return (
    instagramFeedShape(segments) ??
    instagramCreateShape(segments) ??
    instagramLiveShape(segments) ??
    instagramReelShape(segments) ??
    instagramPostShape(segments) ??
    instagramStoriesShape(segments) ??
    instagramDirectShape(segments) ??
    channelShape('instagram', segments[0] ?? parsed.domain, 'medium')
  );
}

function instagramFeedShape(segments: readonly string[]): ParsedUrlShape | null {
  return segments.length === 0 || instagramFeedSegments.includes(segments[0] as (typeof instagramFeedSegments)[number])
    ? socialRouteShape('social-feed', 'instagram', 'medium', ['parsed-url', 'dynamic-feed', 'parsed-social-route'])
    : null;
}

function instagramCreateShape(segments: readonly string[]): ParsedUrlShape | null {
  return segments[0] === 'create'
    ? socialRouteShape('social-upload-post', 'instagram', 'medium', ['parsed-url', 'parsed-social-route'])
    : null;
}

function instagramLiveShape(segments: readonly string[]): ParsedUrlShape | null {
  return segments[0] === 'live'
    ? socialRouteShape('social-livestream', 'instagram', 'medium', ['parsed-url', 'parsed-social-route'])
    : null;
}

function instagramReelShape(segments: readonly string[]): ParsedUrlShape | null {
  const postId = segments[1];
  return (segments[0] === 'reel' || segments[0] === 'reels') && postId !== undefined
    ? socialPostShape('short-video', 'instagram', postId, 'high')
    : null;
}

function instagramPostShape(segments: readonly string[]): ParsedUrlShape | null {
  const postId = segments[1];
  return (segments[0] === 'p' || segments[0] === 'tv') && postId !== undefined
    ? socialPostShape('social-post', 'instagram', postId, 'medium')
    : null;
}

function instagramStoriesShape(segments: readonly string[]): ParsedUrlShape | null {
  return segments[0] === 'stories'
    ? socialRouteShape('social-feed', 'instagram', 'medium', ['parsed-url', 'dynamic-feed', 'parsed-social-route'])
    : null;
}

function instagramDirectShape(segments: readonly string[]): ParsedUrlShape | null {
  return segments[0] === 'direct'
    ? socialRouteShape('social-messaging', 'instagram', 'medium', ['parsed-url', 'parsed-social-route'])
    : null;
}
