import type { ParsedBrowserUrl, ParsedUrlShape } from './browser_url_intelligence';
import { hasText, pathSegments } from './browser_url_intelligence_url_text';
import { queryParam } from './browser_url_intelligence_query';
import { channelShape, emptyPlatformIds, simpleShape, videoShape } from './browser_url_intelligence_social_common';

export function vimeoShape(parsed: ParsedBrowserUrl): ParsedUrlShape | null {
  const segments = pathSegments(parsed);
  const videoId = segments[0] === 'video' ? (segments[1] ?? null) : (segments[0] ?? null);
  return videoId !== null && /^[0-9]+$/.test(videoId) ? videoShape('vimeo', videoId, ['parsed-url']) : null;
}

export function youtubeShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  return (
    youtubeShortsShape(segments) ??
    youtubeWatchShape(parsed, segments) ??
    youtubeEmbedOrLiveShape(segments) ??
    youtubeNamedChannelShape(segments) ??
    youtubeHandleShape(segments) ??
    youtubePlaylistShape(parsed, segments) ??
    youtubeSearchShape(parsed, segments) ??
    simpleShape('unknown', 'youtube', 'low', ['parsed-url'])
  );
}

function youtubeShortsShape(segments: readonly string[]): ParsedUrlShape | null {
  const videoId = segments[1];
  return segments[0] === 'shorts' && videoId !== undefined
    ? {
        ...videoShape('youtube-shorts', videoId, ['parsed-url', 'parsed-youtube-shorts-id']),
        targetKind: 'short-video',
      }
    : null;
}

function youtubeWatchShape(parsed: ParsedBrowserUrl, segments: readonly string[]): ParsedUrlShape | null {
  const videoId = queryParam(parsed, 'v');
  return (segments[0] === 'watch' || segments.length === 0) && hasText(videoId)
    ? videoShape('youtube', videoId, ['parsed-url', 'parsed-youtube-video-id'])
    : null;
}

function youtubeEmbedOrLiveShape(segments: readonly string[]): ParsedUrlShape | null {
  const videoId = segments[1];
  return (segments[0] === 'embed' || segments[0] === 'live') && videoId !== undefined
    ? videoShape('youtube', videoId, ['parsed-url', 'parsed-youtube-video-id'])
    : null;
}

function youtubeNamedChannelShape(segments: readonly string[]): ParsedUrlShape | null {
  const channelId = segments[1];
  return ['channel', 'c', 'user', '@'].includes(segments[0] ?? '') && channelId !== undefined
    ? youtubeChannelShape(channelId)
    : null;
}

function youtubeHandleShape(segments: readonly string[]): ParsedUrlShape | null {
  const channelId = segments[0];
  return channelId?.startsWith('@') === true ? youtubeChannelShape(channelId) : null;
}

function youtubePlaylistShape(parsed: ParsedBrowserUrl, segments: readonly string[]): ParsedUrlShape | null {
  const playlistId = queryParam(parsed, 'list');
  return segments[0] === 'playlist' && hasText(playlistId)
    ? {
        targetKind: 'playlist',
        platform: 'youtube',
        platformIds: { ...emptyPlatformIds(), playlistId },
        confidence: 'high',
        reasonCodes: ['parsed-url', 'parsed-playlist-id'],
      }
    : null;
}

function youtubeSearchShape(parsed: ParsedBrowserUrl, segments: readonly string[]): ParsedUrlShape | null {
  const query = queryParam(parsed, 'search_query');
  return segments[0] === 'results' && hasText(query)
    ? {
        targetKind: 'search',
        platform: 'youtube',
        platformIds: { ...emptyPlatformIds(), query },
        confidence: 'high',
        reasonCodes: ['parsed-url', 'parsed-search-query'],
      }
    : null;
}

function youtubeChannelShape(channelId: string): ParsedUrlShape {
  return channelShape('youtube', channelId, 'high');
}
