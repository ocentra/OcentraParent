#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserUrlShapeEvaluationInput<'a> {
    pub classification_id: &'a str,
    pub classified_at: &'a str,
    pub source_evidence_ids: &'a [&'a str],
    pub source_kind: &'a str,
    pub url: Option<&'a str>,
    pub title: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserUrlShapePlatformIdsTemplate {
    pub video_id: Option<String>,
    pub channel_id: Option<String>,
    pub playlist_id: Option<String>,
    pub post_id: Option<String>,
    pub query: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserUrlShapeClassificationTemplate {
    pub schema_version: u8,
    pub classification_id: String,
    pub classified_at: String,
    pub source_evidence_ids: Vec<String>,
    pub source_kind: &'static str,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub target_kind: &'static str,
    pub platform: &'static str,
    pub platform_ids: BrowserUrlShapePlatformIdsTemplate,
    pub confidence: &'static str,
    pub reason_codes: Vec<&'static str>,
    pub exact_url_evidence: bool,
    pub content_semantics_claimed: bool,
    pub ai_decision_claimed: bool,
    pub policy_decision_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedUrlShape {
    target_kind: &'static str,
    platform: &'static str,
    platform_ids: BrowserUrlShapePlatformIdsTemplate,
    confidence: &'static str,
    reason_codes: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedBrowserUrl {
    normalized_url: String,
    domain: String,
    path: String,
    query: Option<String>,
}

#[path = "browser_url_intelligence_common.rs"]
mod browser_url_intelligence_common;
#[path = "browser_url_intelligence_shapes.rs"]
mod browser_url_intelligence_shapes;
#[path = "browser_url_intelligence_social_platforms.rs"]
mod browser_url_intelligence_social_platforms;
#[path = "browser_url_intelligence_social_video.rs"]
mod browser_url_intelligence_social_video;
#[path = "browser_url_intelligence_youtube.rs"]
mod browser_url_intelligence_youtube;

use self::browser_url_intelligence_common::*;
use self::browser_url_intelligence_shapes::*;
use self::browser_url_intelligence_social_platforms::*;
use self::browser_url_intelligence_social_video::*;
use self::browser_url_intelligence_youtube::*;

pub fn evaluate_browser_url_shape(
    input: &BrowserUrlShapeEvaluationInput<'_>,
) -> BrowserUrlShapeClassificationTemplate {
    if input.source_kind != "managed-browser-exact-url" {
        return browser_url_shape_classification_for_non_exact(input);
    }

    let Some(parsed) = input.url.and_then(parse_url) else {
        return browser_url_shape_classification_for_unsupported_scheme(input);
    };

    browser_url_shape_classification_for_parsed(input, parsed)
}

pub fn browser_url_intelligence_typescript() -> String {
    BROWSER_URL_INTELLIGENCE_TYPESCRIPT.to_string()
}

fn source_kind(value: &str) -> &'static str {
    match value {
        "managed-browser-exact-url" => "managed-browser-exact-url",
        "managed-browser-target-list" => "managed-browser-target-list",
        "unmanaged-browser-process" => "unmanaged-browser-process",
        "network-domain" => "network-domain",
        _ => "managed-browser-target-list",
    }
}

fn browser_url_shape_classification_for_non_exact(
    input: &BrowserUrlShapeEvaluationInput<'_>,
) -> BrowserUrlShapeClassificationTemplate {
    BrowserUrlShapeClassificationTemplate {
        schema_version: 1,
        classification_id: input.classification_id.to_string(),
        classified_at: input.classified_at.to_string(),
        source_evidence_ids: input
            .source_evidence_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        source_kind: source_kind(input.source_kind),
        url: None,
        domain: None,
        title: None,
        target_kind: "unknown",
        platform: "unknown",
        platform_ids: empty_platform_ids(),
        confidence: "low",
        reason_codes: vec![
            non_exact_evidence_reason(input.source_kind),
            "content-not-inferred",
        ],
        exact_url_evidence: false,
        content_semantics_claimed: false,
        ai_decision_claimed: false,
        policy_decision_claimed: false,
    }
}

fn browser_url_shape_classification_for_unsupported_scheme(
    input: &BrowserUrlShapeEvaluationInput<'_>,
) -> BrowserUrlShapeClassificationTemplate {
    BrowserUrlShapeClassificationTemplate {
        schema_version: 1,
        classification_id: input.classification_id.to_string(),
        classified_at: input.classified_at.to_string(),
        source_evidence_ids: input
            .source_evidence_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        source_kind: "managed-browser-exact-url",
        url: None,
        domain: None,
        title: input.title.map(str::to_string),
        target_kind: "unknown",
        platform: "unknown",
        platform_ids: empty_platform_ids(),
        confidence: "low",
        reason_codes: vec!["unsupported-scheme", "content-not-inferred"],
        exact_url_evidence: false,
        content_semantics_claimed: false,
        ai_decision_claimed: false,
        policy_decision_claimed: false,
    }
}

fn browser_url_shape_classification_for_parsed(
    input: &BrowserUrlShapeEvaluationInput<'_>,
    parsed: ParsedBrowserUrl,
) -> BrowserUrlShapeClassificationTemplate {
    let mut shape = shape_for_parsed_url(&parsed);
    shape.reason_codes.push("content-not-inferred");

    BrowserUrlShapeClassificationTemplate {
        schema_version: 1,
        classification_id: input.classification_id.to_string(),
        classified_at: input.classified_at.to_string(),
        source_evidence_ids: input
            .source_evidence_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        source_kind: "managed-browser-exact-url",
        url: Some(parsed.normalized_url),
        domain: Some(parsed.domain),
        title: input.title.map(str::to_string),
        target_kind: shape.target_kind,
        platform: shape.platform,
        platform_ids: shape.platform_ids,
        confidence: shape.confidence,
        reason_codes: shape.reason_codes,
        exact_url_evidence: true,
        content_semantics_claimed: false,
        ai_decision_claimed: false,
        policy_decision_claimed: false,
    }
}

fn shape_for_parsed_url(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    if domain_matches_any(&parsed.domain, &["youtube.com"]) {
        return youtube_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, &["youtu.be"]) {
        if let Some(video_id) = first_path_segment(parsed) {
            return video_shape(
                "youtube",
                &video_id,
                vec!["parsed-url", "parsed-youtube-video-id"],
            );
        }
        return simple_shape("unknown", "generic-web", "low", vec!["parsed-url"]);
    }
    if domain_matches_any(&parsed.domain, &["vimeo.com"]) {
        if let Some(shape) = vimeo_shape(parsed) {
            return shape;
        }
        return simple_shape("unknown", "generic-web", "low", vec!["parsed-url"]);
    }
    if domain_matches_any(&parsed.domain, &["tiktok.com"]) {
        return tiktok_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, &["instagram.com"]) {
        return instagram_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, &["facebook.com"]) {
        return facebook_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, &["twitch.tv"]) {
        return twitch_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, &["x.com", "twitter.com"]) {
        return x_twitter_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, &["reddit.com"]) {
        return reddit_shape(parsed);
    }
    if domain_matches_any(&parsed.domain, &["discord.com"]) {
        return discord_shape(parsed);
    }
    simple_shape("unknown", "generic-web", "low", vec!["parsed-url"])
}

const BROWSER_URL_INTELLIGENCE_TYPESCRIPT: &str = r#"/* generated from crates/browser-core/src/browser_url_intelligence.rs */

type BrowserUrlShapeSourceKind =
  | 'managed-browser-exact-url'
  | 'managed-browser-target-list'
  | 'unmanaged-browser-process'
  | 'network-domain';

type BrowserUrlShapeTargetKind =
  | 'video'
  | 'short-video'
  | 'channel'
  | 'playlist'
  | 'search'
  | 'article'
  | 'forum'
  | 'social-feed'
  | 'social-post'
  | 'social-messaging'
  | 'social-upload-post'
  | 'social-livestream'
  | 'game'
  | 'cloud-gaming'
  | 'download'
  | 'browser-internal'
  | 'file'
  | 'unknown';

type BrowserUrlShapePlatform =
  | 'youtube'
  | 'youtube-shorts'
  | 'vimeo'
  | 'tiktok'
  | 'instagram'
  | 'facebook'
  | 'twitch'
  | 'x-twitter'
  | 'reddit'
  | 'discord'
  | 'generic-web'
  | 'unknown';

type BrowserUrlShapeConfidence = 'high' | 'medium' | 'low' | 'unknown';

type BrowserUrlShapeReasonCode =
  | 'parsed-url'
  | 'parsed-youtube-video-id'
  | 'parsed-youtube-shorts-id'
  | 'parsed-channel-id'
  | 'parsed-playlist-id'
  | 'parsed-search-query'
  | 'parsed-post-id'
  | 'parsed-social-route'
  | 'title-domain-only'
  | 'unsupported-scheme'
  | 'unmanaged-process-only'
  | 'network-domain-only'
  | 'no-exact-evidence'
  | 'dynamic-feed'
  | 'content-not-inferred'
  | 'manual-required';

type BrowserUrlShapeParserInput = {
  readonly classificationId: string;
  readonly classifiedAt: string;
  readonly sourceEvidenceIds: readonly [string, ...string[]];
  readonly sourceKind: BrowserUrlShapeSourceKind;
  readonly url: string | null;
  readonly title?: string | null;
};

type BrowserUrlShapePlatformIds = {
  readonly videoId: string | null;
  readonly channelId: string | null;
  readonly playlistId: string | null;
  readonly postId: string | null;
  readonly query: string | null;
};

type ParsedUrlShape = {
  readonly targetKind: BrowserUrlShapeTargetKind;
  readonly platform: BrowserUrlShapePlatform;
  readonly platformIds: BrowserUrlShapePlatformIds;
  readonly confidence: BrowserUrlShapeConfidence;
  readonly reasonCodes: readonly BrowserUrlShapeReasonCode[];
};

type ParsedBrowserUrl = {
  readonly normalizedUrl: string;
  readonly domain: string;
  readonly path: string;
  readonly query: string | null;
};

const instagramFeedSegments = ['explore', 'reels'] as const;

export function browserUrlShapeClassificationResultTemplate(input: BrowserUrlShapeParserInput) {
  if (input.sourceKind !== 'managed-browser-exact-url') {
    return {
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
    } as const;
  }

  const parsed = parseUrl(input.url);
  if (parsed === null) {
    return {
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
    } as const;
  }

  const shape = shapeForParsedUrl(parsed);
  return {
    ...baseResult(input),
    url: parsed.normalizedUrl,
    domain: parsed.domain,
    title: input.title ?? null,
    ...shape,
    reasonCodes: [...shape.reasonCodes, 'content-not-inferred'],
    exactUrlEvidence: true,
  } as const;
}

function baseResult(input: BrowserUrlShapeParserInput) {
  return {
    schemaVersion: 1,
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
  if (domainMatchesAny(parsed.domain, ['youtube.com'])) {
    return youtubeShape(parsed);
  }
  if (domainMatchesAny(parsed.domain, ['youtu.be'])) {
    const videoId = firstPathSegment(parsed);
    return videoId === null ? simpleShape('unknown', 'generic-web', 'low', ['parsed-url']) : videoShape('youtube', videoId, ['parsed-url', 'parsed-youtube-video-id']);
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

function vimeoShape(parsed: ParsedBrowserUrl): ParsedUrlShape | null {
  const segments = pathSegments(parsed);
  const videoId = segments[0] === 'video' ? segments[1] ?? null : segments[0] ?? null;
  return videoId !== null && /^[0-9]+$/.test(videoId) ? videoShape('vimeo', videoId, ['parsed-url']) : null;
}

function youtubeShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
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
  return ['channel', 'c', 'user', '@'].includes(segments[0] ?? '') && channelId !== undefined ? youtubeChannelShape(channelId) : null;
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
  return {
    targetKind: 'channel',
    platform: 'youtube',
    platformIds: { ...emptyPlatformIds(), channelId },
    confidence: 'high',
    reasonCodes: ['parsed-url', 'parsed-channel-id'],
  };
}

function tiktokShape(parsed: ParsedBrowserUrl): ParsedUrlShape {
  const segments = pathSegments(parsed);
  if (segments[0] === 'upload') {
    return socialRouteShape('social-upload-post', 'tiktok', 'medium', ['parsed-url', 'parsed-social-route']);
  }
  const videoIndex = segments.indexOf('video');
  const videoId = videoIndex >= 0 ? segments[videoIndex + 1] ?? null : null;
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
    return socialRouteShape('social-feed', 'x-twitter', 'medium', ['parsed-url', 'dynamic-feed', 'parsed-social-route']);
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

function channelShape(
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
  confidence: BrowserUrlShapeConfidence,
  reasonCodes: readonly BrowserUrlShapeReasonCode[]
): ParsedUrlShape {
  return { targetKind, platform, platformIds: emptyPlatformIds(), confidence, reasonCodes };
}

function simpleShape(
  targetKind: BrowserUrlShapeTargetKind,
  platform: BrowserUrlShapePlatform,
  confidence: BrowserUrlShapeConfidence,
  reasonCodes: readonly BrowserUrlShapeReasonCode[]
): ParsedUrlShape {
  return { targetKind, platform, platformIds: emptyPlatformIds(), confidence, reasonCodes };
}

function emptyPlatformIds(): BrowserUrlShapePlatformIds {
  return {
    videoId: null,
    channelId: null,
    playlistId: null,
    postId: null,
    query: null,
  };
}

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
  return {
    normalizedUrl: `${scheme}://${normalized.authority}${suffix}`,
    domain: normalized.domain,
    path: pathFromSuffix(suffix),
    query: queryFromSuffix(suffix),
  };
}

function firstSuffixIndex(value: string): number | null {
  const indexes = ['/', '?', '#'].map((separator) => value.indexOf(separator)).filter((index) => index >= 0);
  return indexes.length === 0 ? null : Math.min(...indexes);
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
  if (value.split(':').length - 1 === 1) {
    const separatorIndex = value.lastIndexOf(':');
    const host = value.slice(0, separatorIndex);
    const port = value.slice(separatorIndex + 1);
    if (host.length > 0 && /^[0-9]+$/.test(port)) {
      return [host, port];
    }
  }
  return [value, null];
}

function normalizedHost(value: string): string | null {
  const normalized = value.replace(/\.+$/, '').toLowerCase();
  if (normalized.length === 0 || normalized.includes('/')) {
    return null;
  }
  return normalized;
}

function pathFromSuffix(value: string): string {
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
    .map((segment) => segment.trim())
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

function domainMatchesAny(domain: string, bases: readonly string[]) {
  return bases.some((base) => domain === base || domain.endsWith(`.${base}`));
}

function hasText(value: string | null): value is string {
  return value !== null && value.length > 0;
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
"#;
