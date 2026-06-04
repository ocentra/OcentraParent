import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BrowserAiConfidenceSchema,
  BrowserAiDegradedStateSchema,
  BrowserAiRecommendedPolicyInputSchema,
} from './browser-ai-analysis-schemas';
import { BrowserCustodyLabelSchema } from './browser-schemas';
import {
  BrowserSocialAiAnalysisResultSchema,
  type BrowserSocialAiAnalysisResult,
} from './browser-social-ai-analysis-schemas';
import { BrowserSocialAiAnalysisIdSchema } from './browser-social-ai-analysis-values';
import {
  BrowserSocialFeedVideoRouteGatePlanSchema,
  type BrowserSocialFeedVideoRouteGatePlan,
} from './browser-social-feed-video-route-gate';
import { BrowserSocialFeedVideoRouteGatePlanIdSchema } from './browser-social-feed-video-route-gate-values';
import {
  BrowserSocialPlatformSchema,
  BrowserSocialRouteEvidenceIdSchema,
} from './browser-social-platform-route-schemas';
import {
  BrowserSocialRiskBenefitSignalSetSchema,
  type BrowserSocialRiskBenefitSignalSet,
} from './browser-social-riskbenefit-signals';
import { BrowserSocialRiskBenefitSignalSetIdSchema } from './browser-social-riskbenefit-values';
import { BrowserSocialVideoMetadataEvidenceIdSchema } from './browser-social-video-metadata';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivitySubjectIdSchema,
  ActivityTimestampSchema,
} from './primitives';
import {
  SocialVideoPermittedDownstreamUseSchema,
  SocialVideoSourcePrivacyEvidenceIdSchema,
  SocialVideoSourcePrivacySummarySchema,
  SocialVideoSourcePrivacyTargetKindSchema,
  type SocialVideoSourcePrivacySummary,
} from './social-video-source-privacy';

const NonEmptySocialVideoAggregateText = Schema.String.pipe(Schema.minLength(1));
const SocialVideoAggregateEvidenceRefsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video aggregate source evidence refs')
);
const SocialVideoAggregatePermittedUsesSchema = Schema.Array(SocialVideoPermittedDownstreamUseSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social/video aggregate downstream uses')
);

export const SocialVideoAiSignalAggregateSchemaVersion = 1;

export const SocialVideoAiSignalAggregateIdSchema = withParser(
  NonEmptySocialVideoAggregateText.pipe(Schema.brand('SocialVideoAiSignalAggregateId'))
);
export const SocialVideoAiSignalAggregateActionCandidateRefSchema = withParser(
  NonEmptySocialVideoAggregateText.pipe(Schema.brand('SocialVideoAiSignalAggregateActionCandidateRef'))
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

type SocialVideoAiSignalAggregateCandidate = Infer<typeof SocialVideoAiSignalAggregateBaseSchema>;
type SocialVideoAiSignalAggregateInputCandidate = Infer<typeof SocialVideoAiSignalAggregateInputBaseSchema>;

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

function socialVideoAiSignalAggregateInputIsConsistent(value: SocialVideoAiSignalAggregateInputCandidate) {
  return (
    aggregateAiResultMatchesSourcePrivacy(value.sourcePrivacySummary, value.socialAiAnalysisResult) &&
    aggregateSignalSetMatchesAiResult(
      value.sourcePrivacySummary,
      value.socialAiAnalysisResult,
      value.riskBenefitSignalSet
    ) &&
    aggregateRouteGateMatchesSourcePrivacy(value.sourcePrivacySummary, value.routeGatePlan)
  );
}

function aggregateAiResultMatchesSourcePrivacy(
  sourcePrivacySummary: SocialVideoSourcePrivacySummary,
  socialAiAnalysisResult: BrowserSocialAiAnalysisResult | null
) {
  if (socialAiAnalysisResult === null) {
    return true;
  }
  return (
    sourcePrivacySummary.permittedDownstreamUses.includes('ai-analysis-input') &&
    socialAiAnalysisResult.platform === sourcePrivacySummary.platform &&
    refListIncludes(socialAiAnalysisResult.sourceEvidenceIds, sourcePrivacySummary.sourcePrivacyEvidenceId) &&
    refListIncludes(sourcePrivacySummary.socialRouteEvidenceIds, socialAiAnalysisResult.socialRouteEvidenceId)
  );
}

function aggregateSignalSetMatchesAiResult(
  sourcePrivacySummary: SocialVideoSourcePrivacySummary,
  socialAiAnalysisResult: BrowserSocialAiAnalysisResult | null,
  riskBenefitSignalSet: BrowserSocialRiskBenefitSignalSet | null
) {
  if (riskBenefitSignalSet === null) {
    return true;
  }
  if (socialAiAnalysisResult === null) {
    return false;
  }
  return (
    riskBenefitSignalSet.socialAiAnalysisId === socialAiAnalysisResult.analysisId &&
    riskBenefitSignalSet.socialRouteEvidenceId === socialAiAnalysisResult.socialRouteEvidenceId &&
    refListIncludes(riskBenefitSignalSet.sourceEvidenceIds, sourcePrivacySummary.sourcePrivacyEvidenceId)
  );
}

function aggregateRouteGateMatchesSourcePrivacy(
  sourcePrivacySummary: SocialVideoSourcePrivacySummary,
  routeGatePlan: BrowserSocialFeedVideoRouteGatePlan | null
) {
  if (routeGatePlan === null) {
    return true;
  }
  return (
    sourcePrivacySummary.permittedDownstreamUses.includes('policy-candidate-input') &&
    refListIncludes(routeGatePlan.sourceEvidenceIds, sourcePrivacySummary.sourcePrivacyEvidenceId) &&
    refListIncludes(sourcePrivacySummary.socialRouteEvidenceIds, routeGatePlan.socialRouteEvidenceId) &&
    routeGateMetadataMatchesSourcePrivacy(sourcePrivacySummary, routeGatePlan)
  );
}

function routeGateMetadataMatchesSourcePrivacy(
  sourcePrivacySummary: SocialVideoSourcePrivacySummary,
  routeGatePlan: BrowserSocialFeedVideoRouteGatePlan
) {
  return (
    routeGatePlan.videoMetadataEvidenceId === null ||
    refListIncludes(sourcePrivacySummary.socialVideoMetadataEvidenceIds, routeGatePlan.videoMetadataEvidenceId)
  );
}

function socialVideoAiSignalAggregateIsConsistent(value: SocialVideoAiSignalAggregateCandidate) {
  return (
    !aggregateClaimsForbiddenState(value) &&
    aggregateStateIsConsistent(value) &&
    aggregateCandidateRefsAreConsistent(value)
  );
}

function aggregateClaimsForbiddenState(value: SocialVideoAiSignalAggregateCandidate) {
  return (
    value.rawContentCaptured ||
    value.rawMessageContentCaptured ||
    value.rawVideoCaptured ||
    value.screenshotCaptured ||
    value.connectorTokenStored ||
    value.connectorApiCalled ||
    value.nativeAppControlClaimed ||
    value.finalPolicyDecisionClaimed ||
    value.alertDeliveryClaimed ||
    value.uiRenderedClaimed ||
    value.enforcementClaimed
  );
}

function aggregateStateIsConsistent(value: SocialVideoAiSignalAggregateCandidate) {
  if (value.aggregateState === 'candidate-ready') {
    return aggregateCandidateReadyStateIsConsistent(value);
  }
  if (value.aggregateState === 'manual-required') {
    return aggregateManualRequiredStateIsConsistent(value);
  }
  if (value.aggregateState === 'unavailable') {
    return aggregateUnavailableStateIsConsistent(value);
  }
  return aggregateDegradedStateIsConsistent(value);
}

function aggregateCandidateReadyStateIsConsistent(value: SocialVideoAiSignalAggregateCandidate) {
  return (
    value.degradedState === 'none' &&
    value.confidence !== 'unknown' &&
    value.socialAiAnalysisIds.length > 0 &&
    value.socialRiskBenefitSignalSetIds.length > 0 &&
    value.routeGatePlanIds.length > 0 &&
    value.permittedDownstreamUses.includes('ai-analysis-input') &&
    value.permittedDownstreamUses.includes('policy-candidate-input')
  );
}

function aggregateManualRequiredStateIsConsistent(value: SocialVideoAiSignalAggregateCandidate) {
  return value.degradedState === 'manual-required' && value.confidence === 'unknown';
}

function aggregateUnavailableStateIsConsistent(value: SocialVideoAiSignalAggregateCandidate) {
  return value.degradedState === 'unavailable' && !value.permittedDownstreamUses.includes('policy-candidate-input');
}

function aggregateDegradedStateIsConsistent(value: SocialVideoAiSignalAggregateCandidate) {
  return value.degradedState === 'degraded' && value.confidence !== 'high';
}

function aggregateCandidateRefsAreConsistent(value: SocialVideoAiSignalAggregateCandidate) {
  if (value.routeGatePlanIds.length === 0) {
    return value.actionCandidateRefs.length === 0;
  }
  return value.actionCandidateRefs.length > 0 && value.socialRouteEvidenceIds.length > 0;
}

function aggregateSourceEvidenceIds(value: SocialVideoAiSignalAggregateInputCandidate) {
  return uniqueRefs(
    value.sourcePrivacySummary.sourceEvidenceIds,
    [value.sourcePrivacySummary.sourcePrivacyEvidenceId],
    value.socialAiAnalysisResult?.sourceEvidenceIds ?? [],
    value.riskBenefitSignalSet?.sourceEvidenceIds ?? [],
    value.routeGatePlan?.sourceEvidenceIds ?? []
  );
}

function actionCandidateRefsForGate(routeGatePlan: BrowserSocialFeedVideoRouteGatePlan | null) {
  if (routeGatePlan === null) {
    return [];
  }
  const refs: Array<unknown> = [];
  if (routeGatePlan.policyDecisionCandidateRef !== null) {
    refs.push(routeGatePlan.policyDecisionCandidateRef);
  }
  if (routeGatePlan.parentApprovalRequestRef !== null) {
    refs.push(routeGatePlan.parentApprovalRequestRef);
  }
  if (routeGatePlan.timeLimitCandidateRef !== null) {
    refs.push(routeGatePlan.timeLimitCandidateRef);
  }
  return refs;
}

function aggregateStateForInput(value: SocialVideoAiSignalAggregateInputCandidate) {
  if (value.sourcePrivacySummary.degradedState === 'manual-required') {
    return 'manual-required';
  }
  if (value.sourcePrivacySummary.degradedState === 'unavailable') {
    return 'unavailable';
  }
  if (
    value.socialAiAnalysisResult === null ||
    value.riskBenefitSignalSet === null ||
    value.routeGatePlan === null ||
    value.socialAiAnalysisResult.degradedState !== 'none' ||
    value.riskBenefitSignalSet.degradedState !== 'none' ||
    value.routeGatePlan.routeGateState !== 'planned'
  ) {
    return 'degraded';
  }
  return 'candidate-ready';
}

function aggregateConfidenceForInput(value: SocialVideoAiSignalAggregateInputCandidate) {
  return (
    value.riskBenefitSignalSet?.confidence ??
    value.socialAiAnalysisResult?.confidence ??
    value.sourcePrivacySummary.confidence
  );
}

function aggregateDegradedStateForInput(value: SocialVideoAiSignalAggregateInputCandidate) {
  if (value.sourcePrivacySummary.degradedState !== 'none') {
    return value.sourcePrivacySummary.degradedState;
  }
  if (
    value.socialAiAnalysisResult?.degradedState !== undefined &&
    value.socialAiAnalysisResult.degradedState !== 'none'
  ) {
    return value.socialAiAnalysisResult.degradedState;
  }
  if (value.riskBenefitSignalSet?.degradedState !== undefined && value.riskBenefitSignalSet.degradedState !== 'none') {
    return value.riskBenefitSignalSet.degradedState;
  }
  if (value.socialAiAnalysisResult === null || value.riskBenefitSignalSet === null || value.routeGatePlan === null) {
    return 'degraded';
  }
  return 'none';
}

function refListIncludes(values: ReadonlyArray<unknown>, expected: unknown) {
  return values.some((value) => value === expected);
}

function uniqueRefs(...valueGroups: ReadonlyArray<ReadonlyArray<unknown>>) {
  const refs: Array<unknown> = [];
  for (const valueGroup of valueGroups) {
    for (const value of valueGroup) {
      if (!refs.includes(value)) {
        refs.push(value);
      }
    }
  }
  return refs;
}
