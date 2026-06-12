import { describe, expect, it } from 'vitest';
import { BrowserSocialAiAnalysisInputSchema } from '../../src/browser-social-ai-analysis-schemas';
import { buildBrowserSocialAiAnalysisResult } from '../../src/browser-social-ai-analysis-result-builder';
import {
  BrowserSocialBenefitSignalSchema,
  BrowserSocialRiskBenefitSignalSetSchema,
  BrowserSocialRiskSignalSchema,
  buildBrowserSocialRiskBenefitSignalSet,
} from '../../src/browser-social-riskbenefit-signals';

describe('browser social risk and benefit signal model contract', () => {
  it('builds a candidate signal set from a typed social AI analysis result', buildsCandidateSignals);
  it('accepts manual-required and unavailable unknown signal rows', acceptsManualUnknownSignals);
  it('rejects raw content, identity, policy, connector, native, and enforcement claims', rejectsAuthorityClaims);
  it('rejects empty signal sets and degraded-state inconsistencies', rejectsInconsistentSignalSets);
});

function buildsCandidateSignals() {
  const analysis = socialAiAnalysisResult();
  const signalSet = buildBrowserSocialRiskBenefitSignalSet({
    signalSetId: 'social-riskbenefit-signal-set-video',
    modeledAt: '2026-06-03T06:43:00.000Z',
    socialAiAnalysisResult: analysis,
    signalSourceKind: 'social-ai-analysis',
    riskSignals: [riskSignal()],
    benefitSignals: [benefitSignal()],
  });

  expect(signalSet.socialAiAnalysisId).toBe(analysis.analysisId);
  expect(signalSet.platform).toBe('youtube');
  expect(signalSet.recommendedPolicyInput).toBe('warn-candidate');
  expect(signalSet.finalPolicyDecisionClaimed).toBe(false);
  expect(signalSet.enforcementClaimed).toBe(false);
}

function acceptsManualUnknownSignals() {
  const manualRisk = BrowserSocialRiskSignalSchema.safeParse(
    riskSignal({
      signalId: 'social-riskSignal-manual',
      kind: 'unknown-risk',
      severity: 'unknown',
      state: 'manual-required',
      confidence: 'unknown',
    })
  );
  const unavailableBenefit = BrowserSocialBenefitSignalSchema.safeParse(
    benefitSignal({
      signalId: 'social-benefit-signal-unavailable',
      kind: 'unknown-benefit',
      severity: 'unknown',
      state: 'unavailable',
      confidence: 'unknown',
    })
  );

  expect(manualRisk.success).toBe(true);
  expect(unavailableBenefit.success).toBe(true);
}

function rejectsAuthorityClaims() {
  const valid = buildSignalSet();
  const invalidSignals = [
    { ...riskSignal(), rawMessageContentUsed: true },
    { ...riskSignal(), rawFeedContentUsed: true },
    { ...riskSignal(), accountIdentityVerifiedClaimed: true },
    { ...riskSignal(), policyDecisionClaimed: true },
    { ...riskSignal(), enforcementClaimed: true },
    { ...benefitSignal(), rawPageBodyUsed: true },
  ];
  const invalidSets = [
    { ...valid, finalPolicyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, rawModelTextUsed: true },
    { ...valid, rawMessageContentUsed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
  ];

  for (const invalid of invalidSignals) {
    expect(BrowserSocialRiskSignalSchema.safeParse(invalid).success).toBe(false);
    expect(BrowserSocialBenefitSignalSchema.safeParse(invalid).success).toBe(false);
  }
  for (const invalid of invalidSets) {
    expect(BrowserSocialRiskBenefitSignalSetSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsInconsistentSignalSets() {
  const valid = buildSignalSet();
  const invalidRows = [
    { ...valid, riskSignals: [], benefitSignals: [] },
    { ...valid, degradedState: 'degraded', confidence: 'high', uncertaintyReasons: ['low-confidence'] },
    { ...valid, confidence: 'unknown' },
    { ...valid, riskSignals: [{ ...riskSignal(), kind: 'unknown-risk' }] },
    { ...valid, benefitSignals: [{ ...benefitSignal(), severity: 'unknown' }] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserSocialRiskBenefitSignalSetSchema.safeParse(invalid).success).toBe(false);
  }
}

function buildSignalSet() {
  return buildBrowserSocialRiskBenefitSignalSet({
    signalSetId: 'social-riskbenefit-signal-set-video',
    modeledAt: '2026-06-03T06:43:00.000Z',
    socialAiAnalysisResult: socialAiAnalysisResult(),
    signalSourceKind: 'social-ai-analysis',
    riskSignals: [riskSignal()],
    benefitSignals: [benefitSignal()],
  });
}

function riskSignal(overrides = {}) {
  return {
    signalId: 'social-riskSignal-addictive-feed',
    kind: 'addictive-feed',
    severity: 'medium',
    state: 'candidate',
    confidence: 'medium',
    evidenceRefs: ['social-ai-analysis-video-result'],
    rawMessageContentUsed: false,
    rawFeedContentUsed: false,
    rawPageBodyUsed: false,
    accountIdentityVerifiedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function benefitSignal(overrides = {}) {
  return {
    signalId: 'social-benefit-signal-homework-help',
    kind: 'homework-help',
    severity: 'low',
    state: 'candidate',
    confidence: 'medium',
    evidenceRefs: ['social-ai-analysis-video-result'],
    rawMessageContentUsed: false,
    rawFeedContentUsed: false,
    rawPageBodyUsed: false,
    accountIdentityVerifiedClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function socialAiAnalysisResult() {
  const input = BrowserSocialAiAnalysisInputSchema.parse({
    schemaVersion: 1,
    requestId: 'social-ai-request-video',
    requestedAt: '2026-06-03T06:42:00.000Z',
    childProfileRef: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    sourceEvidenceIds: ['social-route-evidence-video', 'social-video-metadata-evidence-youtube'],
    socialRouteEvidenceId: 'social-route-evidence-video',
    urlShapeClassificationId: 'url-shape-social-video-youtube',
    platform: 'youtube',
    routeKind: 'video',
    feedRouteClassificationIds: [],
    metadataEvidenceIds: ['social-video-metadata-evidence-youtube'],
    accountFlowEvidenceIds: [],
    accountIdentityRefs: [],
    screenSummaryEvidenceRefs: [],
    parentRuleRefs: ['parent-rule-homework-window'],
    memoryHitIds: ['memory-hit-known-education-video'],
    requestedTask: 'video-safety',
    modelRuntimePreference: 'local-preferred',
    promptTemplate: {
      promptTemplateId: 'social-ai-prompt-template-video-safety',
      promptTemplateVersion: 'social-ai-prompt-template-video-safety-v1',
      requestedTask: 'video-safety',
      allowedInputFieldRefs: ['route-evidence', 'metadata-evidence', 'parent-rule', 'memory-hit'],
      rawPromptTextIncluded: false,
      capturesRawPageBody: false,
      capturesTranscriptText: false,
      capturesMessageContent: false,
      capturesFeedContent: false,
      capturesScreenshot: false,
    },
    custodyLabel: 'child-device-local',
    rawBrowserStateIncluded: false,
    rawPageBodyIncluded: false,
    rawMessageContentIncluded: false,
    rawFeedContentIncluded: false,
    transcriptTextIncluded: false,
    screenshotIncluded: false,
    nativeAppStateIncluded: false,
    platformConnectorIncluded: false,
  });

  return buildBrowserSocialAiAnalysisResult({
    analysisId: 'social-ai-analysis-video-result',
    analyzedAt: '2026-06-03T06:42:30.000Z',
    expiresAt: '2026-06-03T07:42:30.000Z',
    input,
    classifications: ['video-watch'],
    riskSignalRefs: ['social-riskSignal-ref-video'],
    benefitSignalRefs: ['social-benefit-signal-ref-homework'],
    recommendedPolicyInput: 'warn-candidate',
    confidence: 'medium',
    uncertaintyReasons: [],
    parentSummaryRef: 'parent-summary-ref-video-analysis',
    childSafeSummaryRef: null,
    modelRuntimeRef: 'local-social-ai-runtime-ref',
    degradedState: 'none',
  });
}
