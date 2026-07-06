/* generated from crates/browser-core/src/social_video_ai_signal_aggregate.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  BrowserSocialAiAnalysisResultSchema,
  type BrowserSocialAiAnalysisResult,
} from '@ocentra-parent/schema-domain/browser-social-ai-analysis-schemas';
import {
  BrowserSocialFeedVideoRouteGatePlanSchema,
  type BrowserSocialFeedVideoRouteGatePlan,
} from '@ocentra-parent/schema-domain/browser-social-feed-video-route-gate';
import {
  BrowserSocialRiskBenefitSignalSetSchema,
  type BrowserSocialRiskBenefitSignalSet,
} from '@ocentra-parent/schema-domain/browser-social-riskbenefit-signals';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivitySubjectIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  SocialVideoSourcePrivacyEvidenceIdSchema,
  SocialVideoPermittedDownstreamUseSchema,
  SocialVideoSourcePrivacySummarySchema,
  SocialVideoSourcePrivacyTargetKindSchema,
  type SocialVideoSourcePrivacySummary,
} from './social-video-source-privacy';
import {
  BrowserAiConfidenceSchema,
  BrowserAiDegradedStateSchema,
  BrowserAiRecommendedPolicyInputSchema,
  BrowserCustodyLabelSchema,
  BrowserSocialAiAnalysisIdSchema,
  BrowserSocialFeedVideoRouteGatePlanIdSchema,
  BrowserSocialPlatformSchema,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRiskBenefitSignalSetIdSchema,
  BrowserSocialVideoMetadataEvidenceIdSchema,
} from './social_video_ai_signal_aggregate_support';
import {
  actionCandidateRefsForGate,
  aggregateSourceEvidenceIds,
  socialVideoAiSignalAggregateInputIsConsistent,
} from './social_video_ai_signal_aggregate_input_helpers';
import {
  aggregateConfidenceForInput,
  aggregateDegradedStateForInput,
  aggregateStateForInput,
  socialVideoAiSignalAggregateIsConsistent,
} from './social_video_ai_signal_aggregate_state_helpers';
const SocialVideoAggregateEvidenceRefsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video aggregate source evidence refs')
);
const SocialVideoAggregatePermittedUsesSchema = Schema.Array(SocialVideoPermittedDownstreamUseSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video aggregate downstream uses')
);

export const SocialVideoAiSignalAggregateSchemaVersion = 1;

export const SocialVideoAiSignalAggregateIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialVideoAiSignalAggregateId')
);
export const SocialVideoAiSignalAggregateActionCandidateRefSchema = withParser(
  brandedNonEmptyStringSchema('SocialVideoAiSignalAggregateActionCandidateRef')
);
export const SocialVideoAiSignalAggregateStateSchema = withParser(
  Schema.Literal('candidate-ready', 'degraded', 'manual-required', 'unavailable')
);

const SocialVideoAiSignalAggregateBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(SocialVideoAiSignalAggregateSchemaVersion),
  aggregateId: SocialVideoAiSignalAggregateIdSchema,
  aggregatedAt: ActivityTimestampSchema,
  sourcePrivacyEvidenceId: SocialVideoSourcePrivacyEvidenceIdSchema,
  childProfileRef: ActivitySubjectIdSchema,
  deviceId: ActivityDeviceIdSchema,
  platform: BrowserSocialPlatformSchema,
  targetKind: SocialVideoSourcePrivacyTargetKindSchema,
  sourceEvidenceIds: SocialVideoAggregateEvidenceRefsSchema,
  socialRouteEvidenceIds: Schema.Array(BrowserSocialRouteEvidenceIdSchema),
  socialVideoMetadataEvidenceIds: Schema.Array(BrowserSocialVideoMetadataEvidenceIdSchema),
  socialAiAnalysisIds: Schema.Array(BrowserSocialAiAnalysisIdSchema),
  socialRiskBenefitSignalSetIds: Schema.Array(BrowserSocialRiskBenefitSignalSetIdSchema),
  routeGatePlanIds: Schema.Array(BrowserSocialFeedVideoRouteGatePlanIdSchema),
  actionCandidateRefs: Schema.Array(SocialVideoAiSignalAggregateActionCandidateRefSchema),
  recommendedPolicyInput: BrowserAiRecommendedPolicyInputSchema,
  aggregateState: SocialVideoAiSignalAggregateStateSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  confidence: BrowserAiConfidenceSchema,
  degradedState: BrowserAiDegradedStateSchema,
  permittedDownstreamUses: SocialVideoAggregatePermittedUsesSchema,
  rawContentCaptured: Schema.Boolean,
  rawMessageContentCaptured: Schema.Boolean,
  rawVideoCaptured: Schema.Boolean,
  screenshotCaptured: Schema.Boolean,
  connectorTokenStored: Schema.Boolean,
  connectorApiCalled: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  alertDeliveryClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

export const SocialVideoAiSignalAggregateSchema = withParser(
  SocialVideoAiSignalAggregateBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialVideoAiSignalAggregateIsConsistent(value) ||
        'Expected social/video AI signal aggregate to stay ref-only, candidate-only, and non-enforcing'
    )
  )
);

const OptionalSocialAiAnalysisResultSchema = Schema.Union(BrowserSocialAiAnalysisResultSchema, Schema.Null);
const OptionalSocialRiskBenefitSignalSetSchema = Schema.Union(BrowserSocialRiskBenefitSignalSetSchema, Schema.Null);
const OptionalSocialRouteGatePlanSchema = Schema.Union(BrowserSocialFeedVideoRouteGatePlanSchema, Schema.Null);

const SocialVideoAiSignalAggregateInputBaseSchema = Schema.Struct({
  aggregateId: SocialVideoAiSignalAggregateIdSchema,
  aggregatedAt: ActivityTimestampSchema,
  sourcePrivacySummary: SocialVideoSourcePrivacySummarySchema,
  socialAiAnalysisResult: OptionalSocialAiAnalysisResultSchema,
  riskBenefitSignalSet: OptionalSocialRiskBenefitSignalSetSchema,
  routeGatePlan: OptionalSocialRouteGatePlanSchema,
});
const SocialVideoAiSignalAggregateInputSchema = withParser(
  SocialVideoAiSignalAggregateInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialVideoAiSignalAggregateInputIsConsistent(value) ||
        'Expected source/privacy refs to match AI, signal, and route gate candidate refs'
    )
  )
);

export const decodeSocialVideoAiSignalAggregate = Schema.decodeUnknownSync(SocialVideoAiSignalAggregateSchema);

export type SocialVideoAiSignalAggregate = Infer<typeof SocialVideoAiSignalAggregateSchema>;
export type SocialVideoAiSignalAggregateInput = Infer<typeof SocialVideoAiSignalAggregateInputSchema>;
export type SocialVideoAiSignalAggregateState = Infer<typeof SocialVideoAiSignalAggregateStateSchema>;

export function buildSocialVideoAiSignalAggregate(
  input: SocialVideoAiSignalAggregateInput
): SocialVideoAiSignalAggregate {
  const parsed = SocialVideoAiSignalAggregateInputSchema.parse(input);

  return SocialVideoAiSignalAggregateSchema.parse({
    schemaVersion: SocialVideoAiSignalAggregateSchemaVersion,
    aggregateId: parsed.aggregateId,
    aggregatedAt: parsed.aggregatedAt,
    sourcePrivacyEvidenceId: parsed.sourcePrivacySummary.sourcePrivacyEvidenceId,
    childProfileRef: parsed.sourcePrivacySummary.childProfileRef,
    deviceId: parsed.sourcePrivacySummary.deviceId,
    platform: parsed.sourcePrivacySummary.platform,
    targetKind: parsed.sourcePrivacySummary.targetKind,
    sourceEvidenceIds: aggregateSourceEvidenceIds(parsed),
    socialRouteEvidenceIds: parsed.sourcePrivacySummary.socialRouteEvidenceIds,
    socialVideoMetadataEvidenceIds: parsed.sourcePrivacySummary.socialVideoMetadataEvidenceIds,
    socialAiAnalysisIds: parsed.socialAiAnalysisResult === null ? [] : [parsed.socialAiAnalysisResult.analysisId],
    socialRiskBenefitSignalSetIds:
      parsed.riskBenefitSignalSet === null ? [] : [parsed.riskBenefitSignalSet.signalSetId],
    routeGatePlanIds: parsed.routeGatePlan === null ? [] : [parsed.routeGatePlan.gatePlanId],
    actionCandidateRefs: actionCandidateRefsForGate(parsed.routeGatePlan),
    recommendedPolicyInput: parsed.socialAiAnalysisResult?.recommendedPolicyInput ?? 'manual-review-candidate',
    aggregateState: aggregateStateForInput(parsed),
    custodyLabel: parsed.sourcePrivacySummary.custodyLabel,
    confidence: aggregateConfidenceForInput(parsed),
    degradedState: aggregateDegradedStateForInput(parsed),
    permittedDownstreamUses: parsed.sourcePrivacySummary.permittedDownstreamUses,
    rawContentCaptured: false,
    rawMessageContentCaptured: false,
    rawVideoCaptured: false,
    screenshotCaptured: false,
    connectorTokenStored: false,
    connectorApiCalled: false,
    nativeAppControlClaimed: false,
    finalPolicyDecisionClaimed: false,
    alertDeliveryClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
  });
}
