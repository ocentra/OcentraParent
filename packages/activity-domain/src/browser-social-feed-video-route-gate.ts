import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import {
  BrowserSocialFeedRouteClassification,
  BrowserSocialFeedRouteClassificationIdSchema,
  BrowserSocialFeedRouteClassificationSchema,
  BrowserSocialFeedSurfaceKindSchema,
} from './browser-social-feed-route-classification';
import {
  BrowserSocialVideoMetadataEvidence,
  BrowserSocialVideoMetadataEvidenceIdSchema,
  BrowserSocialVideoMetadataEvidenceSchema,
  BrowserSocialVideoMetadataStateSchema,
} from './browser-social-video-metadata';
import {
  BrowserSocialPlatformSchema,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRouteKindSchema,
} from './browser-social-platform-route-schemas';
import {
  BrowserSocialFeedVideoRouteGateActionSchema,
  BrowserSocialFeedVideoRouteGatePlanIdSchema,
  BrowserSocialFeedVideoRouteGateReasonsSchema,
  BrowserSocialFeedVideoRouteGateSchemaVersion,
  BrowserSocialFeedVideoRouteGateStateSchema,
  BrowserSocialFeedVideoRouteGateTargetKind,
  BrowserSocialFeedVideoRouteGateTargetKindSchema,
  OptionalSocialFeedVideoRouteGateTextSchema,
  SocialFeedVideoRouteGateSourceEvidenceIdsSchema,
} from './browser-social-feed-video-route-gate-values';
import { browserSocialFeedVideoRouteGateClaimsRuntime } from './browser-social-feed-video-route-gate-guards';

const OptionalFeedRouteClassificationIdSchema = Schema.Union(BrowserSocialFeedRouteClassificationIdSchema, Schema.Null);
const OptionalVideoMetadataEvidenceIdSchema = Schema.Union(BrowserSocialVideoMetadataEvidenceIdSchema, Schema.Null);
const OptionalFeedSurfaceKindSchema = Schema.Union(BrowserSocialFeedSurfaceKindSchema, Schema.Null);
const OptionalVideoMetadataStateSchema = Schema.Union(BrowserSocialVideoMetadataStateSchema, Schema.Null);

const BrowserSocialFeedVideoRouteGateInputBaseSchema = Schema.Struct({
  gatePlanId: BrowserSocialFeedVideoRouteGatePlanIdSchema,
  plannedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialFeedVideoRouteGateSourceEvidenceIdsSchema,
  feedRouteClassification: Schema.Union(BrowserSocialFeedRouteClassificationSchema, Schema.Null),
  videoMetadataEvidence: Schema.Union(BrowserSocialVideoMetadataEvidenceSchema, Schema.Null),
  policyDecisionCandidateRef: OptionalSocialFeedVideoRouteGateTextSchema,
  parentApprovalRequestRef: OptionalSocialFeedVideoRouteGateTextSchema,
  timeLimitCandidateRef: OptionalSocialFeedVideoRouteGateTextSchema,
  routeGateAction: BrowserSocialFeedVideoRouteGateActionSchema,
  parentApprovalRequired: Schema.Boolean,
  reasons: BrowserSocialFeedVideoRouteGateReasonsSchema,
});
export const BrowserSocialFeedVideoRouteGateInputSchema = withParser(
  BrowserSocialFeedVideoRouteGateInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        feedVideoRouteGateInputIsConsistent(value) ||
        'Expected managed social feed/video route evidence and non-final policy candidate refs'
    )
  )
);

const BrowserSocialFeedVideoRouteGatePlanBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialFeedVideoRouteGateSchemaVersion),
  gatePlanId: BrowserSocialFeedVideoRouteGatePlanIdSchema,
  plannedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialFeedVideoRouteGateSourceEvidenceIdsSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  feedRouteClassificationId: OptionalFeedRouteClassificationIdSchema,
  videoMetadataEvidenceId: OptionalVideoMetadataEvidenceIdSchema,
  platform: BrowserSocialPlatformSchema,
  routeKind: BrowserSocialRouteKindSchema,
  surfaceKind: OptionalFeedSurfaceKindSchema,
  metadataState: OptionalVideoMetadataStateSchema,
  routeGateTargetKind: BrowserSocialFeedVideoRouteGateTargetKindSchema,
  routeGateState: BrowserSocialFeedVideoRouteGateStateSchema,
  routeGateAction: BrowserSocialFeedVideoRouteGateActionSchema,
  parentApprovalRequired: Schema.Boolean,
  policyDecisionCandidateRef: OptionalSocialFeedVideoRouteGateTextSchema,
  parentApprovalRequestRef: OptionalSocialFeedVideoRouteGateTextSchema,
  timeLimitCandidateRef: OptionalSocialFeedVideoRouteGateTextSchema,
  reasons: BrowserSocialFeedVideoRouteGateReasonsSchema,
  browserNavigationBlockedClaimed: Schema.Boolean,
  browserRedirectClaimed: Schema.Boolean,
  cssDomHiddenClaimed: Schema.Boolean,
  tabClosedClaimed: Schema.Boolean,
  timeLimitAppliedClaimed: Schema.Boolean,
  childUiRenderedClaimed: Schema.Boolean,
  parentUiNotifiedClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
  feedContentCaptured: Schema.Boolean,
  videoContentCaptured: Schema.Boolean,
  recommendationModelClaimed: Schema.Boolean,
});
export const BrowserSocialFeedVideoRouteGatePlanSchema = withParser(
  BrowserSocialFeedVideoRouteGatePlanBaseSchema.pipe(
    Schema.filter(
      (value) =>
        feedVideoRouteGatePlanIsConsistent(value) ||
        'Expected social feed video route gate plan to remain planned, auditable, and non-enforcing'
    )
  )
);

export const decodeBrowserSocialFeedVideoRouteGatePlan = Schema.decodeUnknownSync(
  BrowserSocialFeedVideoRouteGatePlanSchema
);

export type BrowserSocialFeedVideoRouteGateInput = Infer<typeof BrowserSocialFeedVideoRouteGateInputSchema>;
export type BrowserSocialFeedVideoRouteGatePlan = Infer<typeof BrowserSocialFeedVideoRouteGatePlanSchema>;

export function planBrowserSocialFeedVideoRouteGate(
  input: BrowserSocialFeedVideoRouteGateInput
): BrowserSocialFeedVideoRouteGatePlan {
  const parsed = BrowserSocialFeedVideoRouteGateInputSchema.parse(input);
  const source = sourceForGateInput(parsed);

  return BrowserSocialFeedVideoRouteGatePlanSchema.parse({
    schemaVersion: BrowserSocialFeedVideoRouteGateSchemaVersion,
    gatePlanId: parsed.gatePlanId,
    plannedAt: parsed.plannedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    socialRouteEvidenceId: source.socialRouteEvidenceId,
    feedRouteClassificationId: parsed.feedRouteClassification?.feedRouteClassificationId ?? null,
    videoMetadataEvidenceId: parsed.videoMetadataEvidence?.metadataEvidenceId ?? null,
    platform: source.platform,
    routeKind: source.routeKind,
    surfaceKind: parsed.feedRouteClassification?.surfaceKind ?? null,
    metadataState: parsed.videoMetadataEvidence?.metadataState ?? null,
    routeGateTargetKind: gateTargetKindForInput(parsed),
    routeGateState: parsed.routeGateAction === 'manual-review-required' ? 'manual-required' : 'planned',
    routeGateAction: parsed.routeGateAction,
    parentApprovalRequired: parsed.parentApprovalRequired,
    policyDecisionCandidateRef: parsed.policyDecisionCandidateRef,
    parentApprovalRequestRef: parsed.parentApprovalRequestRef,
    timeLimitCandidateRef: parsed.timeLimitCandidateRef,
    reasons: parsed.reasons,
    browserNavigationBlockedClaimed: false,
    browserRedirectClaimed: false,
    cssDomHiddenClaimed: false,
    tabClosedClaimed: false,
    timeLimitAppliedClaimed: false,
    childUiRenderedClaimed: false,
    parentUiNotifiedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    feedContentCaptured: false,
    videoContentCaptured: false,
    recommendationModelClaimed: false,
  });
}

function feedVideoRouteGateInputIsConsistent(value: Infer<typeof BrowserSocialFeedVideoRouteGateInputBaseSchema>) {
  if (value.feedRouteClassification === null && value.videoMetadataEvidence === null) {
    return false;
  }
  if (!gateEvidenceMatches(value.feedRouteClassification, value.videoMetadataEvidence)) {
    return false;
  }
  return gateActionRefsAreConsistent(value);
}

function feedVideoRouteGatePlanIsConsistent(value: Infer<typeof BrowserSocialFeedVideoRouteGatePlanBaseSchema>) {
  if (browserSocialFeedVideoRouteGateClaimsRuntime(value)) {
    return false;
  }
  if (value.routeGateAction === 'manual-review-required') {
    return value.routeGateState === 'manual-required' && value.reasons.includes('manual-required');
  }
  if (value.routeGateAction === 'ask-parent-candidate') {
    return (
      value.routeGateState === 'planned' &&
      value.parentApprovalRequired &&
      value.parentApprovalRequestRef !== null &&
      value.reasons.includes('parent-review-required')
    );
  }
  if (value.routeGateAction === 'block-route-candidate') {
    return value.routeGateState === 'planned' && value.policyDecisionCandidateRef !== null;
  }
  if (value.routeGateAction === 'limit-route-candidate') {
    return value.routeGateState === 'planned' && value.timeLimitCandidateRef !== null;
  }
  if (value.routeGateAction === 'unknown-route-warn-only') {
    return value.routeGateState === 'planned' && value.reasons.includes('unknown-evidence');
  }
  return value.routeGateState === 'planned' && value.policyDecisionCandidateRef !== null;
}

function gateEvidenceMatches(
  feedRouteClassification: BrowserSocialFeedRouteClassification | null,
  videoMetadataEvidence: BrowserSocialVideoMetadataEvidence | null
) {
  if (feedRouteClassification === null || videoMetadataEvidence === null) {
    return true;
  }
  return (
    feedRouteClassification.socialRouteEvidenceId === videoMetadataEvidence.socialRouteEvidenceId &&
    feedRouteClassification.platform === videoMetadataEvidence.platform &&
    feedRouteClassification.routeKind === videoMetadataEvidence.routeKind
  );
}

function gateActionRefsAreConsistent(value: Infer<typeof BrowserSocialFeedVideoRouteGateInputBaseSchema>) {
  if (value.routeGateAction === 'ask-parent-candidate') {
    return value.parentApprovalRequired && value.parentApprovalRequestRef !== null;
  }
  if (value.routeGateAction === 'block-route-candidate') {
    return value.policyDecisionCandidateRef !== null && value.reasons.includes('policy-block-candidate');
  }
  if (value.routeGateAction === 'limit-route-candidate') {
    return value.policyDecisionCandidateRef !== null && value.timeLimitCandidateRef !== null;
  }
  if (value.routeGateAction === 'manual-review-required') {
    return value.reasons.includes('manual-required');
  }
  if (value.routeGateAction === 'unknown-route-warn-only') {
    return value.reasons.includes('unknown-evidence');
  }
  return value.policyDecisionCandidateRef !== null;
}

function sourceForGateInput(value: Infer<typeof BrowserSocialFeedVideoRouteGateInputBaseSchema>) {
  if (value.feedRouteClassification !== null) {
    return value.feedRouteClassification;
  }
  return value.videoMetadataEvidence as BrowserSocialVideoMetadataEvidence;
}

function gateTargetKindForInput(
  value: Infer<typeof BrowserSocialFeedVideoRouteGateInputBaseSchema>
): BrowserSocialFeedVideoRouteGateTargetKind {
  if (value.feedRouteClassification !== null) {
    return value.feedRouteClassification.surfaceKind === 'dynamic-feed'
      ? 'social-feed-route'
      : 'social-short-video-route';
  }
  return value.videoMetadataEvidence?.routeKind === 'video' ? 'social-video-route' : 'manual-required';
}
