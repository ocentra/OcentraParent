import { describe, expect, it } from 'vitest';
import {
  BrowserGameAiAnalysisInputSchema,
  type BrowserGameAiAnalysisInput,
  BrowserGameAiAnalysisResultSchema,
  type BrowserGameAiAnalysisResult,
} from '../src/browser-game-ai-analysis';

describe('browser-game AI analysis contracts', () => {
  it('accepts typed-evidence input and candidate result contracts', acceptsTypedEvidenceAiContracts);
  it('accepts unmanaged browser bypass input and degraded manual result states', acceptsBypassAndDegradedStates);
  it('rejects raw payload, model text, cloud-frame, runtime, UI, and enforcement claims', rejectsAuthorityClaims);
  it('rejects inconsistent candidate outputs', rejectsInconsistentAiResults);
});

function acceptsTypedEvidenceAiContracts() {
  const parsedInput = BrowserGameAiAnalysisInputSchema.parse(input());
  const parsedResult = BrowserGameAiAnalysisResultSchema.parse(result());

  expect(parsedInput.task).toBe('risk-classification');
  expect(parsedResult.isGame).toBe(true);
  expect(parsedResult.recommendedPolicyInput).toBe('parent-review-candidate');
  expect(parsedResult.finalPolicyDecisionClaimed).toBe(false);
}

function acceptsBypassAndDegradedStates() {
  expect(
    BrowserGameAiAnalysisInputSchema.safeParse(
      input({
        requestId: 'browser-game-ai-request-unmanaged',
        custodyLabel: 'unmanaged-browser-bypass',
        browserEvidenceRef: null,
        recentActivityRef: 'browser-game-unmanaged-activity-ref',
      })
    ).success
  ).toBe(true);

  expect(
    BrowserGameAiAnalysisResultSchema.safeParse(
      result({
        analysisId: 'browser-game-ai-result-degraded',
        confidence: 'low',
        degradedState: 'degraded',
        uncertaintyReasons: ['model-unavailable'],
        recommendedPolicyInput: 'manual-review-candidate',
      })
    ).success
  ).toBe(true);
}

function rejectsAuthorityClaims() {
  const invalidInputs = [
    { rawUrlIncluded: true },
    { rawPageBodyIncluded: true },
    { rawGamePayloadIncluded: true },
    { rawScreenFrameIncluded: true },
    { rawModelTextIncluded: true },
    { accountOrPurchaseExecutionClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { enforcementClaimed: true },
  ];
  const invalidResults = [
    { rawModelTextStored: true },
    { rawPageBodyStored: true },
    { rawGamePayloadStored: true },
    { rawScreenFrameStored: true },
    { accountOrPurchaseExecutionClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { uiRenderedClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidInputs) {
    expect(BrowserGameAiAnalysisInputSchema.safeParse(input(invalid)).success).toBe(false);
  }
  for (const invalid of invalidResults) {
    expect(BrowserGameAiAnalysisResultSchema.safeParse(result(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentAiResults() {
  const invalidRows = [
    { confidence: 'unknown' },
    { expiresAt: null },
    { isGame: true, gameSurfaceKind: 'unknown' },
    { recommendedPolicyInput: 'allow-candidate', riskSignals: ['violence'] },
    { recommendedPolicyInput: 'block-candidate', riskSignals: [], benefitSignals: [] },
    { isGame: false, gameSurfaceKind: 'browser-game' },
    { degradedState: 'degraded', confidence: 'high', uncertaintyReasons: ['low-confidence'] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameAiAnalysisResultSchema.safeParse(result(invalid)).success).toBe(false);
  }
}

function input(overrides = {}): BrowserGameAiAnalysisInput {
  return {
    schemaVersion: 'browser-game-ai-analysis-contract',
    requestId: 'browser-game-ai-request-risk',
    familyId: 'family-browser-game-ai',
    childProfileId: 'child-browser-game-ai',
    deviceId: 'device-browser-game-ai',
    requestedAt: '2026-06-03T11:02:00.000Z',
    sourceEvidenceRefs: ['browser-game-ai-source-ref'],
    browserEvidenceRef: 'browser-game-managed-browser-ref',
    urlShapeRef: 'browser-game-url-shape-ref',
    runtimeSignalRef: 'browser-game-runtime-signal-ref',
    metadataEvidenceRefs: ['browser-game-metadata-ref'],
    screenSummaryRefs: ['browser-game-screen-summary-ref'],
    parentRuleRefs: ['browser-game-parent-rule-ref'],
    recentActivityRef: 'browser-game-recent-activity-ref',
    memoryRefs: ['browser-game-memory-ref'],
    task: 'risk-classification',
    custodyLabel: 'managed-browser',
    rawUrlIncluded: false,
    rawPageBodyIncluded: false,
    rawGamePayloadIncluded: false,
    rawScreenFrameIncluded: false,
    rawModelTextIncluded: false,
    accountOrPurchaseExecutionClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function result(overrides = {}): BrowserGameAiAnalysisResult {
  return {
    schemaVersion: 'browser-game-ai-analysis-contract',
    analysisId: 'browser-game-ai-result-risk',
    requestId: 'browser-game-ai-request-risk',
    familyId: 'family-browser-game-ai',
    childProfileId: 'child-browser-game-ai',
    deviceId: 'device-browser-game-ai',
    analyzedAt: '2026-06-03T11:03:00.000Z',
    expiresAt: '2026-06-03T12:03:00.000Z',
    sourceEvidenceRefs: ['browser-game-ai-source-ref'],
    parentRuleRefs: ['browser-game-parent-rule-ref'],
    task: 'risk-classification',
    isGame: true,
    gameSurfaceKind: 'browser-game',
    modifiers: ['webgl', 'multiplayer'],
    benefitSignals: ['problem-solving'],
    riskSignals: ['multiplayer-contact'],
    recommendedPolicyInput: 'parent-review-candidate',
    confidence: 'medium',
    uncertaintyReasons: [],
    parentSummaryRef: 'browser-game-parent-summary-ref',
    childSafeSummaryRef: 'browser-game-child-safe-summary-ref',
    modelRuntimeRef: 'browser-game-model-runtime-ref',
    promptTemplateVersion: 'browser-game-prompt-template-v1',
    degradedState: 'none',
    rawModelTextStored: false,
    rawPageBodyStored: false,
    rawGamePayloadStored: false,
    rawScreenFrameStored: false,
    accountOrPurchaseExecutionClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
