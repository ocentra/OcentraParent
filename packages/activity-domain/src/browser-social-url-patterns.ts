import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  BrowserUrlShapeClassificationResult,
  BrowserUrlShapeClassificationResultSchema,
  BrowserUrlShapeTargetKind,
} from './browser-url-intelligence-schemas';
import {
  BrowserSocialPlatform,
  BrowserSocialRouteEvidence,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRouteEvidenceSchema,
  BrowserSocialRouteKind,
  BrowserSocialRouteSchemaVersion,
} from './browser-social-platform-route-schemas';

const NonEmptySocialUrlPatternText = Schema.String.pipe(Schema.minLength(1));

const SocialUrlPatternSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social URL pattern source evidence ids')
);

export const BrowserSocialUrlPatternIdSchema = withParser(
  NonEmptySocialUrlPatternText.pipe(Schema.brand('BrowserSocialUrlPatternId'))
);

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
  if (domain === 'instagram.com' || domain.endsWith('.instagram.com')) {
    return 'instagram';
  }
  if (domain === 'facebook.com' || domain.endsWith('.facebook.com')) {
    return 'facebook';
  }
  if (domain === 'messenger.com' || domain.endsWith('.messenger.com')) {
    return 'messenger';
  }
  if (domain === 'tiktok.com' || domain.endsWith('.tiktok.com')) {
    return 'tiktok';
  }
  if (domain === 'snapchat.com' || domain.endsWith('.snapchat.com')) {
    return 'snapchat';
  }
  if (domain === 'youtube.com' || domain.endsWith('.youtube.com') || domain === 'youtu.be') {
    return targetKind === 'short-video' ? 'youtube-shorts' : 'youtube';
  }
  if (domain === 'vimeo.com' || domain.endsWith('.vimeo.com')) {
    return 'vimeo';
  }
  if (domain === 'twitch.tv' || domain.endsWith('.twitch.tv')) {
    return 'twitch';
  }
  if (domain === 'discord.com' || domain.endsWith('.discord.com')) {
    return 'discord';
  }
  if (domain === 'reddit.com' || domain.endsWith('.reddit.com')) {
    return 'reddit';
  }
  if (domain === 'x.com' || domain.endsWith('.x.com') || domain === 'twitter.com' || domain.endsWith('.twitter.com')) {
    return 'x-twitter';
  }
  if (domain === 'pinterest.com' || domain.endsWith('.pinterest.com')) {
    return 'pinterest';
  }
  if (domain === 'roblox.com' || domain.endsWith('.roblox.com')) {
    return 'roblox';
  }
  return null;
}

function routeKindForUrlShape(
  classification: BrowserUrlShapeClassificationResult,
  segments: readonly string[]
): BrowserSocialRouteKind {
  if (matchesAnySegment(segments, ['signup', 'register', 'join', 'r.php', 'emailsignup'])) {
    return 'account-signup';
  }
  if (matchesAnySegment(segments, ['login', 'signin', 'sign-in'])) {
    return 'login';
  }
  if (matchesAnySegment(segments, ['switch', 'switch_account', 'account_switcher'])) {
    return 'account-switch';
  }
  if (matchesAnySegment(segments, ['privacy', 'settings'])) {
    return 'settings-privacy';
  }
  if (
    classification.targetKind === 'social-messaging' ||
    matchesAnySegment(segments, ['direct', 'messages', 'inbox'])
  ) {
    return 'messaging-route';
  }
  if (classification.targetKind === 'social-upload-post' || matchesAnySegment(segments, ['upload', 'create', 'new'])) {
    return 'upload-post';
  }
  if (classification.targetKind === 'social-livestream' || matchesAnySegment(segments, ['live'])) {
    return 'livestream';
  }
  if (classification.targetKind === 'social-post') {
    return 'post';
  }
  if (classification.targetKind === 'video' || classification.targetKind === 'short-video') {
    return 'video';
  }
  if (classification.targetKind === 'social-feed' || classification.targetKind === 'forum') {
    return 'feed';
  }
  if (classification.targetKind === 'channel') {
    return 'profile';
  }
  return 'platform-access';
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
