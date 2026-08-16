import type { ParsedBrowserUrl, ParsedUrlShape } from './browser_url_intelligence';
import { domainMatchesAny, firstPathSegment } from './browser_url_intelligence_url_text';
import { instagramShape, tiktokShape } from './browser_url_intelligence_social_platforms_a';
import { facebookShape, twitchShape } from './browser_url_intelligence_social_platforms_b';
import { discordShape, redditShape, xTwitterShape } from './browser_url_intelligence_social_platforms_c';
import { vimeoShape, youtubeShape } from './browser_url_intelligence_youtube';
import { simpleShape, videoShape } from './browser_url_intelligence_social_common';

export function shapeForParsedUrl(parsed: ParsedBrowserUrl): ParsedUrlShape {
  if (domainMatchesAny(parsed.domain, ['youtube.com'])) {
    return youtubeShape(parsed);
  }
  if (domainMatchesAny(parsed.domain, ['youtu.be'])) {
    const videoId = firstPathSegment(parsed);
    return videoId === null
      ? simpleShape('unknown', 'generic-web', 'low', ['parsed-url'])
      : videoShape('youtube', videoId, ['parsed-url', 'parsed-youtube-video-id']);
  }
  if (domainMatchesAny(parsed.domain, ['vimeo.com'])) {
    return vimeoShape(parsed) ?? simpleShape('unknown', 'generic-web', 'low', ['parsed-url']);
  }
  if (domainMatchesAny(parsed.domain, ['tiktok.com'])) {
    return tiktokShape(parsed);
  }
  if (domainMatchesAny(parsed.domain, ['instagram.com'])) {
    return instagramShape(parsed);
  }
  if (domainMatchesAny(parsed.domain, ['facebook.com'])) {
    return facebookShape(parsed);
  }
  if (domainMatchesAny(parsed.domain, ['twitch.tv'])) {
    return twitchShape(parsed);
  }
  if (domainMatchesAny(parsed.domain, ['x.com', 'twitter.com'])) {
    return xTwitterShape(parsed);
  }
  if (domainMatchesAny(parsed.domain, ['reddit.com'])) {
    return redditShape(parsed);
  }
  if (domainMatchesAny(parsed.domain, ['discord.com'])) {
    return discordShape(parsed);
  }
  return simpleShape('unknown', 'generic-web', 'low', ['parsed-url']);
}
