/* generated support for crates/browser-core/src/social_video_ai_signal_aggregate.rs */

import type {
  SocialVideoAiSignalAggregate,
  SocialVideoAiSignalAggregateInput,
} from './social_video_ai_signal_aggregate';

export function socialVideoAiSignalAggregateIsConsistent(value: SocialVideoAiSignalAggregate) {
  return (
    !aggregateClaimsForbiddenState(value) &&
    aggregateStateIsConsistent(value) &&
    aggregateCandidateRefsAreConsistent(value)
  );
}

export function aggregateStateForInput(value: SocialVideoAiSignalAggregateInput) {
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

export function aggregateConfidenceForInput(value: SocialVideoAiSignalAggregateInput) {
  return (
    value.riskBenefitSignalSet?.confidence ??
    value.socialAiAnalysisResult?.confidence ??
    value.sourcePrivacySummary.confidence
  );
}

export function aggregateDegradedStateForInput(value: SocialVideoAiSignalAggregateInput) {
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

function aggregateClaimsForbiddenState(value: SocialVideoAiSignalAggregate) {
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

function aggregateStateIsConsistent(value: SocialVideoAiSignalAggregate) {
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

function aggregateCandidateReadyStateIsConsistent(value: SocialVideoAiSignalAggregate) {
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

function aggregateManualRequiredStateIsConsistent(value: SocialVideoAiSignalAggregate) {
  return value.degradedState === 'manual-required' && value.confidence === 'unknown';
}

function aggregateUnavailableStateIsConsistent(value: SocialVideoAiSignalAggregate) {
  return value.degradedState === 'unavailable' && !value.permittedDownstreamUses.includes('policy-candidate-input');
}

function aggregateDegradedStateIsConsistent(value: SocialVideoAiSignalAggregate) {
  return value.degradedState === 'degraded' && value.confidence !== 'high';
}

function aggregateCandidateRefsAreConsistent(value: SocialVideoAiSignalAggregate) {
  if (value.routeGatePlanIds.length === 0) {
    return value.actionCandidateRefs.length === 0;
  }
  return value.actionCandidateRefs.length > 0 && value.socialRouteEvidenceIds.length > 0;
}
