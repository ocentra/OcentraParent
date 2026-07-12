/* generated support for crates/browser-core/src/social_video_ai_signal_aggregate.rs */

import type { BrowserSocialAiAnalysisResult } from '@ocentra-parent/schema-domain/browser-social-ai-analysis-schemas';
import type { BrowserSocialFeedVideoRouteGatePlan } from '@ocentra-parent/schema-domain/browser-social-feed-video-route-gate';
import type { BrowserSocialRiskBenefitSignalSet } from '@ocentra-parent/schema-domain/browser-social-riskbenefit-signals';
import type { SocialVideoSourcePrivacySummary } from './social-video-source-privacy';
import type { SocialVideoAiSignalAggregateInput } from './social_video_ai_signal_aggregate';

export function socialVideoAiSignalAggregateInputIsConsistent(value: SocialVideoAiSignalAggregateInput) {
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

export function aggregateSourceEvidenceIds(value: SocialVideoAiSignalAggregateInput) {
  return uniqueRefs(
    value.sourcePrivacySummary.sourceEvidenceIds,
    [value.sourcePrivacySummary.sourcePrivacyEvidenceId],
    value.socialAiAnalysisResult?.sourceEvidenceIds ?? [],
    value.riskBenefitSignalSet?.sourceEvidenceIds ?? [],
    value.routeGatePlan?.sourceEvidenceIds ?? []
  );
}

export function actionCandidateRefsForGate(routeGatePlan: BrowserSocialFeedVideoRouteGatePlan | null) {
  if (routeGatePlan === null) {
    return [];
  }
  return [
    routeGatePlan.policyDecisionCandidateRef,
    routeGatePlan.parentApprovalRequestRef,
    routeGatePlan.timeLimitCandidateRef,
  ].filter((value): value is unknown => value !== null);
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
