import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  type BrowserUrlShapeClassificationResult,
  BrowserUrlShapeClassificationResultSchema,
  type BrowserUrlShapeTargetKind,
} from './browser-url-intelligence-schemas';
import {
  type BrowserSocialPlatform,
  type BrowserSocialRouteEvidence,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRouteEvidenceSchema,
  type BrowserSocialRouteKind,
  BrowserSocialRouteSchemaVersion,
} from './browser-social-platform-route-schemas';

const SocialUrlPatternSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social URL pattern source evidence ids')
);

export const BrowserSocialUrlPatternIdSchema = withParser(brandedNonEmptyStringSchema('BrowserSocialUrlPatternId'));

const BrowserSocialUrlPatternInputBaseSchema = Schema.Struct({
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialUrlPatternSourceEvidenceIdsSchema,
  classification: BrowserUrlShapeClassificationResultSchema,
});

const BrowserSocialUrlPatternInputSchema = withParser(
  BrowserSocialUrlPatternInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialUrlPatternInputIsSupported(value) ||
        'Expected exact managed social URL shape before building social route evidence'
    )
  )
);

export type BrowserSocialUrlPatternId = Infer<typeof BrowserSocialUrlPatternIdSchema>;
export type BrowserSocialUrlPatternInput = Infer<typeof BrowserSocialUrlPatternInputSchema>;

export type BrowserSocialUrlPatternMatch = {
  readonly patternId: BrowserSocialUrlPatternId;
  readonly platform: BrowserSocialPlatform;
  readonly routeKind: BrowserSocialRouteKind;
  readonly urlShapeTargetKind: BrowserUrlShapeTargetKind;
};

type SocialDomainPlatformRule = {
  readonly platform: BrowserSocialPlatform;
  readonly domains: readonly string[];
};

type SocialRouteSegmentRule = {
  readonly routeKind: BrowserSocialRouteKind;
  readonly segments: readonly string[];
};

const YoutubeSocialDomains = ['youtube.com', 'youtu.be'] as const;

const SocialDomainPlatformRules = [
  { platform: 'instagram', domains: ['instagram.com'] },
  { platform: 'facebook', domains: ['facebook.com'] },
  { platform: 'messenger', domains: ['messenger.com'] },
  { platform: 'tiktok', domains: ['tiktok.com'] },
  { platform: 'snapchat', domains: ['snapchat.com'] },
  { platform: 'vimeo', domains: ['vimeo.com'] },
  { platform: 'twitch', domains: ['twitch.tv'] },
  { platform: 'discord', domains: ['discord.com'] },
  { platform: 'reddit', domains: ['reddit.com'] },
  { platform: 'x-twitter', domains: ['x.com', 'twitter.com'] },
  { platform: 'pinterest', domains: ['pinterest.com'] },
  { platform: 'roblox', domains: ['roblox.com'] },
] satisfies readonly SocialDomainPlatformRule[];

const SocialRouteSegmentRules = [
  { routeKind: 'account-signup', segments: ['signup', 'register', 'join', 'r.php', 'emailsignup'] },
  { routeKind: 'login', segments: ['login', 'signin', 'sign-in'] },
  { routeKind: 'account-switch', segments: ['switch', 'switch_account', 'account_switcher'] },
  { routeKind: 'settings-privacy', segments: ['privacy', 'settings'] },
  { routeKind: 'messaging-route', segments: ['direct', 'messages', 'inbox'] },
  { routeKind: 'upload-post', segments: ['upload', 'create', 'new'] },
  { routeKind: 'livestream', segments: ['live'] },
] satisfies readonly SocialRouteSegmentRule[];

const SocialRouteKindByTargetKind: Partial<Record<BrowserUrlShapeTargetKind, BrowserSocialRouteKind>> = {
  'social-messaging': 'messaging-route',
  'social-upload-post': 'upload-post',
  'social-livestream': 'livestream',
  'social-post': 'post',
  video: 'video',
  'short-video': 'video',
  'social-feed': 'feed',
  forum: 'feed',
  channel: 'profile',
};

export function matchBrowserSocialUrlPattern(
  classification: BrowserUrlShapeClassificationResult
): BrowserSocialUrlPatternMatch | null {
  if (!classification.exactUrlEvidence || classification.sourceKind !== 'managed-browser-exact-url') {
    return null;
  }
  if (classification.url === null || classification.domain === null) {
    return null;
  }

  const platform = platformForSocialDomain(classification.domain, classification.targetKind);
  if (platform === null) {
    return null;
  }

  const routeKind = routeKindForUrlShape(classification, parsedPathSegments(classification.url));
  return {
    patternId: BrowserSocialUrlPatternIdSchema.parse(`${platform}-${routeKind}-url-pattern`),
    platform,
    routeKind,
    urlShapeTargetKind: classification.targetKind,
  };
}

export function buildBrowserSocialRouteEvidenceFromUrlPattern(
  input: BrowserSocialUrlPatternInput
): BrowserSocialRouteEvidence {
  const parsed = BrowserSocialUrlPatternInputSchema.parse(input);
  const match = matchBrowserSocialUrlPattern(parsed.classification);
  if (match === null) {
    throw new Error('Expected supported social URL pattern match');
  }

  return BrowserSocialRouteEvidenceSchema.parse({
    schemaVersion: BrowserSocialRouteSchemaVersion,
    socialRouteEvidenceId: parsed.socialRouteEvidenceId,
    observedAt: parsed.observedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    urlShapeClassificationId: parsed.classification.classificationId,
    urlShapeTargetKind: match.urlShapeTargetKind,
    sourceKind: 'managed-browser-url-shape',
    proofState: 'route-evidence',
    platform: match.platform,
    routeKind: match.routeKind,
    platformAccountRef: null,
    parentApprovalRequestRef: null,
    exactManagedBrowserRouteEvidence: true,
    unmanagedBypassOnly: false,
    manualRequired: false,
    accountIdentityClaimed: false,
    messageContentClaimed: false,
    feedContentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  });
}

function socialUrlPatternInputIsSupported(value: Infer<typeof BrowserSocialUrlPatternInputBaseSchema>) {
  return matchBrowserSocialUrlPattern(value.classification) !== null;
}

function platformForSocialDomain(domain: string, targetKind: BrowserUrlShapeTargetKind): BrowserSocialPlatform | null {
  if (domainMatchesAny(domain, YoutubeSocialDomains)) {
    return targetKind === 'short-video' ? 'youtube-shorts' : 'youtube';
  }
  return SocialDomainPlatformRules.find((rule) => domainMatchesAny(domain, rule.domains))?.platform ?? null;
}

function routeKindForUrlShape(
  classification: BrowserUrlShapeClassificationResult,
  segments: readonly string[]
): BrowserSocialRouteKind {
  const segmentRule = SocialRouteSegmentRules.find((rule) => matchesAnySegment(segments, rule.segments));
  return segmentRule?.routeKind ?? SocialRouteKindByTargetKind[classification.targetKind] ?? 'platform-access';
}

function parsedPathSegments(url: string) {
  const pathStart = url.indexOf('/', url.indexOf('://') + 3);
  if (pathStart < 0) {
    return [];
  }
  const suffix = url.slice(pathStart);
  const queryIndex = suffix.indexOf('?');
  const hashIndex = suffix.indexOf('#');
  const endIndexes = [queryIndex, hashIndex].filter((index) => index >= 0);
  const path = suffix.slice(0, endIndexes.length === 0 ? suffix.length : Math.min(...endIndexes));
  return path
    .split('/')
    .map((segment) => segment.trim().toLowerCase())
    .filter(Boolean);
}

function matchesAnySegment(segments: readonly string[], candidates: readonly string[]) {
  return segments.some((segment) => candidates.includes(segment));
}

function domainMatchesAny(domain: string, bases: readonly string[]) {
  return bases.some((base) => domain === base || domain.endsWith(`.${base}`));
}
