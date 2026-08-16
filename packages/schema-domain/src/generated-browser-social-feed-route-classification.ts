/* generated from crates/browser-core/src/browser_generated_social_ts.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserSocialPlatformSchema,
  type BrowserSocialRouteEvidence,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRouteEvidenceSchema,
  BrowserSocialRouteKindSchema,
} from './generated-browser-social-platform-route-schemas';
const SocialFeedRouteSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social feed route source evidence ids')
);

export const BrowserSocialFeedRouteSchemaVersion = 1;

export const BrowserSocialFeedRouteClassificationIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialFeedRouteClassificationId')
);

export const BrowserSocialFeedSurfaceHintSchema = withParser(
  Schema.Literal('home-feed', 'following-feed', 'explore-feed', 'reels-feed', 'shorts-feed', 'single-short-video')
);

export const BrowserSocialFeedSurfaceKindSchema = withParser(
  Schema.Literal('dynamic-feed', 'short-video-surface', 'single-short-video', 'manual-required')
);

const BrowserSocialFeedRouteClassifierInputBaseSchema = Schema.Struct({
  feedRouteClassificationId: BrowserSocialFeedRouteClassificationIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialFeedRouteSourceEvidenceIdsSchema,
  routeEvidence: BrowserSocialRouteEvidenceSchema,
  surfaceHint: BrowserSocialFeedSurfaceHintSchema,
});

const BrowserSocialFeedRouteClassifierInputSchema = withParser(
  BrowserSocialFeedRouteClassifierInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialFeedRouteClassifierInputIsConsistent(value) ||
        'Expected managed social feed, reels, shorts, or single-short-video route evidence'
    )
  )
);

const BrowserSocialFeedRouteClassificationBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialFeedRouteSchemaVersion),
  feedRouteClassificationId: BrowserSocialFeedRouteClassificationIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialFeedRouteSourceEvidenceIdsSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  platform: BrowserSocialPlatformSchema,
  routeKind: BrowserSocialRouteKindSchema,
  surfaceHint: BrowserSocialFeedSurfaceHintSchema,
  surfaceKind: BrowserSocialFeedSurfaceKindSchema,
  routeOnly: Schema.Boolean,
  dynamicFeed: Schema.Boolean,
  shortVideoSurface: Schema.Boolean,
  feedContentSemanticsClaimed: Schema.Boolean,
  recommendationSemanticsClaimed: Schema.Boolean,
  messageContentClaimed: Schema.Boolean,
  aiDecisionClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
});

export const BrowserSocialFeedRouteClassificationSchema = withParser(
  BrowserSocialFeedRouteClassificationBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialFeedRouteClassificationIsConsistent(value) ||
        'Expected social feed route classification to preserve route-only boundaries'
    )
  )
);

export const decodeBrowserSocialFeedRouteClassification = Schema.decodeUnknownSync(
  BrowserSocialFeedRouteClassificationSchema
);

export type BrowserSocialFeedRouteClassification = Infer<typeof BrowserSocialFeedRouteClassificationSchema>;
export type BrowserSocialFeedRouteClassificationId = Infer<typeof BrowserSocialFeedRouteClassificationIdSchema>;
export type BrowserSocialFeedRouteClassifierInput = Infer<typeof BrowserSocialFeedRouteClassifierInputSchema>;
export type BrowserSocialFeedSurfaceHint = Infer<typeof BrowserSocialFeedSurfaceHintSchema>;
export type BrowserSocialFeedSurfaceKind = Infer<typeof BrowserSocialFeedSurfaceKindSchema>;

export function classifyBrowserSocialFeedRoute(
  input: BrowserSocialFeedRouteClassifierInput
): BrowserSocialFeedRouteClassification {
  const parsed = BrowserSocialFeedRouteClassifierInputSchema.parse(input);
  const surfaceKind = surfaceKindForHint(parsed.surfaceHint);

  return BrowserSocialFeedRouteClassificationSchema.parse({
    schemaVersion: BrowserSocialFeedRouteSchemaVersion,
    feedRouteClassificationId: parsed.feedRouteClassificationId,
    observedAt: parsed.observedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    socialRouteEvidenceId: parsed.routeEvidence.socialRouteEvidenceId,
    platform: parsed.routeEvidence.platform,
    routeKind: parsed.routeEvidence.routeKind,
    surfaceHint: parsed.surfaceHint,
    surfaceKind,
    routeOnly: true,
    dynamicFeed: surfaceKind === 'dynamic-feed' || surfaceKind === 'short-video-surface',
    shortVideoSurface: surfaceKind === 'short-video-surface' || surfaceKind === 'single-short-video',
    feedContentSemanticsClaimed: false,
    recommendationSemanticsClaimed: false,
    messageContentClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  });
}

function socialFeedRouteClassifierInputIsConsistent(
  value: Infer<typeof BrowserSocialFeedRouteClassifierInputBaseSchema>
) {
  if (!routeEvidenceCanClassifyFeed(value.routeEvidence)) {
    return false;
  }
  if (value.surfaceHint === 'single-short-video') {
    return value.routeEvidence.routeKind === 'video';
  }
  return value.routeEvidence.routeKind === 'feed';
}

function browserSocialFeedRouteClassificationIsConsistent(
  value: Infer<typeof BrowserSocialFeedRouteClassificationBaseSchema>
) {
  if (socialFeedRouteClassificationClaimsAuthority(value)) {
    return false;
  }
  if (!value.routeOnly) {
    return false;
  }
  const surfaceKind = surfaceKindForHint(value.surfaceHint);
  return (
    value.surfaceKind === surfaceKind &&
    value.dynamicFeed === (surfaceKind === 'dynamic-feed' || surfaceKind === 'short-video-surface') &&
    value.shortVideoSurface === (surfaceKind === 'short-video-surface' || surfaceKind === 'single-short-video')
  );
}

function socialFeedRouteClassificationClaimsAuthority(
  value: Infer<typeof BrowserSocialFeedRouteClassificationBaseSchema>
) {
  return (
    value.feedContentSemanticsClaimed ||
    value.recommendationSemanticsClaimed ||
    value.messageContentClaimed ||
    value.aiDecisionClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed
  );
}

function routeEvidenceCanClassifyFeed(value: BrowserSocialRouteEvidence) {
  return (
    value.sourceKind === 'managed-browser-url-shape' &&
    value.exactManagedBrowserRouteEvidence &&
    value.proofState === 'route-evidence' &&
    (value.routeKind === 'feed' || value.routeKind === 'video')
  );
}

function surfaceKindForHint(value: BrowserSocialFeedSurfaceHint): BrowserSocialFeedSurfaceKind {
  if (value === 'reels-feed' || value === 'shorts-feed') {
    return 'short-video-surface';
  }
  if (value === 'single-short-video') {
    return 'single-short-video';
  }
  return 'dynamic-feed';
}
