/* generated from crates/browser-core/src/browser_url_intelligence.rs */

import { emptyPlatformIds } from './browser_url_intelligence_social_common';
import { parseUrl } from './browser_url_intelligence_parse';
import { shapeForParsedUrl } from './browser_url_intelligence_shape_dispatch';

export type BrowserUrlShapeSourceKind =
  | 'managed-browser-exact-url'
  | 'managed-browser-target-list'
  | 'unmanaged-browser-process'
  | 'network-domain';

export type BrowserUrlShapeTargetKind =
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

export type BrowserUrlShapePlatform =
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

export type BrowserUrlShapeConfidence = 'high' | 'medium' | 'low' | 'unknown';

export type BrowserUrlShapeReasonCode =
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

export type BrowserUrlShapeParserInput = {
  readonly classificationId: string;
  readonly classifiedAt: string;
  readonly sourceEvidenceIds: readonly [string, ...string[]];
  readonly sourceKind: BrowserUrlShapeSourceKind;
  readonly url: string | null;
  readonly title?: string | null;
};

export type BrowserUrlShapePlatformIds = {
  readonly videoId: string | null;
  readonly channelId: string | null;
  readonly playlistId: string | null;
  readonly postId: string | null;
  readonly query: string | null;
};

export type ParsedUrlShape = {
  readonly targetKind: BrowserUrlShapeTargetKind;
  readonly platform: BrowserUrlShapePlatform;
  readonly platformIds: BrowserUrlShapePlatformIds;
  readonly confidence: BrowserUrlShapeConfidence;
  readonly reasonCodes: readonly BrowserUrlShapeReasonCode[];
};

export type ParsedBrowserUrl = {
  readonly normalizedUrl: string;
  readonly domain: string;
  readonly path: string;
  readonly query: string | null;
};

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

function nonExactEvidenceReason(value: BrowserUrlShapeSourceKind): BrowserUrlShapeReasonCode {
  if (value === 'unmanaged-browser-process') {
    return 'unmanaged-process-only';
  }
  if (value === 'network-domain') {
    return 'network-domain-only';
  }
  return 'no-exact-evidence';
}
