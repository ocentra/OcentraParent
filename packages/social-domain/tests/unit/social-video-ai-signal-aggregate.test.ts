import { describe, expect, it } from 'vitest';
import { BrowserSocialAiAnalysisInputSchema } from '@ocentra-parent/browser-domain/browser-social-ai-analysis-schemas';
import { buildBrowserSocialAiAnalysisResult } from '@ocentra-parent/browser-domain/browser-social-ai-analysis-result-builder';
import {
  BrowserSocialFeedVideoRouteGatePlanSchema,
  type BrowserSocialFeedVideoRouteGatePlan,
} from '@ocentra-parent/browser-domain/browser-social-feed-video-route-gate';
import { buildBrowserSocialRiskBenefitSignalSet } from '@ocentra-parent/browser-domain/browser-social-riskbenefit-signals';
import {
  buildSocialVideoAiSignalAggregate,
  SocialVideoAiSignalAggregateSchema,
} from '../../src/social-video-ai-signal-aggregate';
import { buildSocialVideoSourcePrivacySummary } from '../../src/social-video-source-privacy';

describe('social video AI signal aggregate contract', () => {
  it('aggregates source privacy, AI analysis, signal, and route gate refs without raw custody', aggregatesRefs);
  it('accepts manual-required source privacy summaries without AI or policy gate refs', acceptsManualRequiredAggregate);
  it('rejects AI analysis results that do not cite the source privacy summary ref', rejectsMismatchedAiRefs);
  it('rejects raw content, connector, native, UI, alert, policy, and enforcement claims', rejectsForbiddenClaims);
});

function aggregatesRefs() {
  const sourcePrivacySummary = sourcePrivacy();
  const socialAiAnalysisResult = socialAiResult();
  const riskBenefitSignalSet = signalSet();
  const routeGatePlan = gatePlan();
  const aggregate = buildSocialVideoAiSignalAggregate({
    aggregateId: 'social-video-ai-signal-aggregate-youtube-homework',
    aggregatedAt: '2026-06-04T04:20:00.000Z',
    sourcePrivacySummary,
    socialAiAnalysisResult,
    riskBenefitSignalSet,
    routeGatePlan,
  });

  expect(aggregate.schemaVersion).toBe(1);
  expect(aggregate.aggregateState).toBe('candidate-ready');
  expect(aggregate.sourcePrivacyEvidenceId).toBe(sourcePrivacySummary.sourcePrivacyEvidenceId);
  expect(aggregate.socialAiAnalysisIds).toEqual([socialAiAnalysisResult.analysisId]);
  expect(aggregate.socialRiskBenefitSignalSetIds).toEqual([riskBenefitSignalSet.signalSetId]);
  expect(aggregate.routeGatePlanIds).toEqual([routeGatePlan.gatePlanId]);
  expect(aggregate.actionCandidateRefs).toEqual(['social-parent-video-review-request']);
  expect(aggregate.rawContentCaptured).toBe(false);
  expect(aggregate.rawMessageContentCaptured).toBe(false);
  expect(aggregate.rawVideoCaptured).toBe(false);
  expect(aggregate.screenshotCaptured).toBe(false);
  expect(aggregate.connectorTokenStored).toBe(false);
  expect(aggregate.connectorApiCalled).toBe(false);
  expect(aggregate.nativeAppControlClaimed).toBe(false);
  expect(aggregate.finalPolicyDecisionClaimed).toBe(false);
  expect(aggregate.alertDeliveryClaimed).toBe(false);
  expect(aggregate.uiRenderedClaimed).toBe(false);
  expect(aggregate.enforcementClaimed).toBe(false);
}

function acceptsManualRequiredAggregate() {
  const aggregate = buildSocialVideoAiSignalAggregate({
    aggregateId: 'social-video-ai-signal-aggregate-native-manual',
    aggregatedAt: '2026-06-04T04:21:00.000Z',
    sourcePrivacySummary: sourcePrivacy({
      sourcePrivacyEvidenceId: 'source-privacy-native-manual-required',
      platform: 'tiktok',
      targetKind: 'native-social-app',
      sourceTypes: ['android-native-manual-required'],
      socialRouteEvidenceIds: [],
      socialVideoMetadataEvidenceIds: [],
      parentProvidedUrlRefs: [],
      parentProvidedChannelRefs: [],
      screenSummaryEvidenceRefs: [],
      connectorAuthorizationRefs: [],
      manualRequiredReason: 'native-app-source-unavailable',
      confidence: 'unknown',
      degradedState: 'manual-required',
      permittedDownstreamUses: ['manual-review', 'audit-summary'],
    }),
    socialAiAnalysisResult: null,
    riskBenefitSignalSet: null,
    routeGatePlan: null,
  });

  expect(aggregate.aggregateState).toBe('manual-required');
  expect(aggregate.socialAiAnalysisIds).toEqual([]);
  expect(aggregate.routeGatePlanIds).toEqual([]);
  expect(aggregate.permittedDownstreamUses).toEqual(['manual-review', 'audit-summary']);
}

function rejectsMismatchedAiRefs() {
  expect(() =>
    buildSocialVideoAiSignalAggregate({
      aggregateId: 'social-video-ai-signal-aggregate-mismatched-ai',
      aggregatedAt: '2026-06-04T04:22:00.000Z',
      sourcePrivacySummary: sourcePrivacy(),
      socialAiAnalysisResult: socialAiResult({
        requestId: 'social-ai-request-mismatched',
        sourceEvidenceIds: ['social-route-evidence-youtube-video', 'social-video-metadata-youtube-video'],
      }),
      riskBenefitSignalSet: null,
      routeGatePlan: null,
    })
  ).toThrow();
}

function rejectsForbiddenClaims() {
  const valid = buildAggregate();
  const invalidRows = [
    { ...valid, rawContentCaptured: true },
    { ...valid, rawMessageContentCaptured: true },
    { ...valid, rawVideoCaptured: true },
    { ...valid, screenshotCaptured: true },
    { ...valid, connectorTokenStored: true },
    { ...valid, connectorApiCalled: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, finalPolicyDecisionClaimed: true },
    { ...valid, alertDeliveryClaimed: true },
    { ...valid, uiRenderedClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, routeGatePlanIds: [], actionCandidateRefs: ['dangling-action-candidate-ref'] },
  ];

  for (const invalid of invalidRows) {
    expect(SocialVideoAiSignalAggregateSchema.safeParse(invalid).success).toBe(false);
  }
}

function buildAggregate() {
  return buildSocialVideoAiSignalAggregate({
    aggregateId: 'social-video-ai-signal-aggregate-youtube-homework',
    aggregatedAt: '2026-06-04T04:20:00.000Z',
    sourcePrivacySummary: sourcePrivacy(),
    socialAiAnalysisResult: socialAiResult(),
    riskBenefitSignalSet: signalSet(),
    routeGatePlan: gatePlan(),
  });
}

function sourcePrivacy(overrides = {}) {
  return buildSocialVideoSourcePrivacySummary({
    sourcePrivacyEvidenceId: 'source-privacy-youtube-homework-video',
    summarizedAt: '2026-06-04T04:18:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'device-managed-laptop',
    sourceEvidenceIds: [
      'social-route-evidence-youtube-video',
      'social-video-metadata-youtube-video',
      'screen-summary-youtube-video-ref',
    ],
    platform: 'youtube',
    targetKind: 'video-url',
    sourceTypes: [
      'managed-browser-social-route-ref',
      'managed-browser-video-metadata-ref',
      'parent-provided-url-ref',
      'parent-provided-channel-ref',
      'screen-summary-ref',
      'connector-authorization-ref',
    ],
    socialRouteEvidenceIds: ['social-route-evidence-youtube-video'],
    socialVideoMetadataEvidenceIds: ['social-video-metadata-youtube-video'],
    parentProvidedUrlRefs: ['parent-provided-url-ref-youtube-video'],
    parentProvidedChannelRefs: ['parent-provided-channel-ref-teacher'],
    screenSummaryEvidenceRefs: ['screen-summary-youtube-video-ref'],
    connectorAuthorizationRefs: ['connector-authorization-ref-youtube-supervision'],
    manualRequiredReason: null,
    custodyLabel: 'child-device-local',
    confidence: 'medium',
    degradedState: 'none',
    permittedDownstreamUses: ['ai-analysis-input', 'policy-candidate-input', 'parent-explanation', 'audit-summary'],
    ...overrides,
  });
}

function socialAiResult(inputOverrides = {}) {
  const input = BrowserSocialAiAnalysisInputSchema.parse({
    schemaVersion: 1,
    requestId: 'social-ai-request-youtube-homework-video',
    requestedAt: '2026-06-04T04:18:30.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'device-managed-laptop',
    sourceEvidenceIds: [
      'source-privacy-youtube-homework-video',
      'social-route-evidence-youtube-video',
      'social-video-metadata-youtube-video',
    ],
    socialRouteEvidenceId: 'social-route-evidence-youtube-video',
    urlShapeClassificationId: 'url-shape-social-video-youtube',
    platform: 'youtube',
    routeKind: 'video',
    feedRouteClassificationIds: [],
    metadataEvidenceIds: ['social-video-metadata-youtube-video'],
    accountFlowEvidenceIds: [],
    accountIdentityRefs: [],
    screenSummaryEvidenceRefs: ['screen-summary-youtube-video-ref'],
    parentRuleRefs: ['parent-rule-homework-window'],
    memoryHitIds: ['memory-hit-known-education-video'],
    requestedTask: 'video-safety',
    modelRuntimePreference: 'local-preferred',
    promptTemplate: socialPromptTemplate(),
    custodyLabel: 'child-device-local',
    rawBrowserStateIncluded: false,
    rawPageBodyIncluded: false,
    rawMessageContentIncluded: false,
    rawFeedContentIncluded: false,
    transcriptTextIncluded: false,
    screenshotIncluded: false,
    nativeAppStateIncluded: false,
    platformConnectorIncluded: false,
    ...inputOverrides,
  });

  return buildBrowserSocialAiAnalysisResult({
    analysisId: 'social-ai-analysis-youtube-homework-video',
    analyzedAt: '2026-06-04T04:19:00.000Z',
    expiresAt: '2026-06-04T05:19:00.000Z',
    input,
    classifications: ['video-watch', 'educational-video'],
    riskSignalRefs: ['social-risk-signal-ref-video-low'],
    benefitSignalRefs: ['social-benefit-signal-ref-homework'],
    recommendedPolicyInput: 'parent-review-candidate',
    confidence: 'medium',
    uncertaintyReasons: [],
    parentSummaryRef: 'parent-summary-ref-video-analysis',
    childSafeSummaryRef: 'child-safe-summary-ref-video-analysis',
    modelRuntimeRef: 'local-social-ai-runtime-ref',
    degradedState: 'none',
  });
}

function socialPromptTemplate() {
  return {
    promptTemplateId: 'social-ai-prompt-template-video-safety',
    promptTemplateVersion: 'social-ai-prompt-template-video-safety-v1',
    requestedTask: 'video-safety',
    allowedInputFieldRefs: ['source-privacy-summary', 'route-evidence', 'metadata-evidence', 'parent-rule'],
    rawPromptTextIncluded: false,
    capturesRawPageBody: false,
    capturesTranscriptText: false,
    capturesMessageContent: false,
    capturesFeedContent: false,
    capturesScreenshot: false,
  };
}

function signalSet() {
  return buildBrowserSocialRiskBenefitSignalSet({
    signalSetId: 'social-riskbenefit-signal-set-youtube-homework-video',
    modeledAt: '2026-06-04T04:19:30.000Z',
    socialAiAnalysisResult: socialAiResult(),
    signalSourceKind: 'social-ai-analysis',
    riskSignals: [
      {
        signalId: 'social-risk-signal-youtube-watch',
        kind: 'addictive-feed',
        severity: 'low',
        state: 'candidate',
        confidence: 'medium',
        evidenceRefs: ['social-ai-analysis-youtube-homework-video'],
        rawMessageContentUsed: false,
        rawFeedContentUsed: false,
        rawPageBodyUsed: false,
        accountIdentityVerifiedClaimed: false,
        policyDecisionClaimed: false,
        enforcementClaimed: false,
      },
    ],
    benefitSignals: [
      {
        signalId: 'social-benefit-signal-youtube-homework',
        kind: 'homework-help',
        severity: 'low',
        state: 'candidate',
        confidence: 'medium',
        evidenceRefs: ['social-ai-analysis-youtube-homework-video'],
        rawMessageContentUsed: false,
        rawFeedContentUsed: false,
        rawPageBodyUsed: false,
        accountIdentityVerifiedClaimed: false,
        policyDecisionClaimed: false,
        enforcementClaimed: false,
      },
    ],
  });
}

function gatePlan(): BrowserSocialFeedVideoRouteGatePlan {
  return BrowserSocialFeedVideoRouteGatePlanSchema.parse({
    schemaVersion: 1,
    gatePlanId: 'social-video-route-gate-youtube-homework-parent-review',
    plannedAt: '2026-06-04T04:19:45.000Z',
    sourceEvidenceIds: [
      'source-privacy-youtube-homework-video',
      'social-route-evidence-youtube-video',
      'social-video-metadata-youtube-video',
    ],
    socialRouteEvidenceId: 'social-route-evidence-youtube-video',
    feedRouteClassificationId: null,
    videoMetadataEvidenceId: 'social-video-metadata-youtube-video',
    platform: 'youtube',
    routeKind: 'video',
    surfaceKind: null,
    metadataState: 'available',
    routeGateTargetKind: 'social-video-route',
    routeGateState: 'planned',
    routeGateAction: 'parent-review-candidate',
    parentApprovalRequired: true,
    policyDecisionCandidateRef: null,
    parentApprovalRequestRef: 'social-parent-video-review-request',
    timeLimitCandidateRef: null,
    reasons: ['single-video-route', 'metadata-available', 'parent-review-required'],
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
