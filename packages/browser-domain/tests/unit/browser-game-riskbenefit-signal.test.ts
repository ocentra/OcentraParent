import { describe, expect, it } from 'vitest';
import {
  BrowserGameBenefitSignalSchema,
  BrowserGameRiskBenefitSignalSetSchema,
  type BrowserGameRiskBenefitSignalSet,
  BrowserGameRiskSignalSchema,
} from '@ocentra-parent/schema-domain/browser-game-riskbenefit-signal';

describe('browser-game risk and benefit signal contracts', () => {
  it('accepts an evidence-backed candidate risk/benefit signal set', acceptsCandidateSignalSet);
  it('accepts manual-required and unavailable unknown signal rows', acceptsUnknownSignalRows);
  it('rejects raw page/game/chat/model, policy, native, cloud-frame, and enforcement claims', rejectsAuthorityClaims);
  it('rejects empty, degraded, and unsupported recommended policy signal sets', rejectsInconsistentSignalSets);
});

function acceptsCandidateSignalSet() {
  const parsed = BrowserGameRiskBenefitSignalSetSchema.parse(signalSet());

  expect(parsed.schemaVersion).toBe('browser-game-riskbenefit-signal-contract');
  expect(parsed.recommendedPolicyInput).toBe('parent-review-candidate');
  expect(parsed.finalPolicyDecisionClaimed).toBe(false);
  expect(parsed.runtimeGateExecutedClaimed).toBe(false);
  expect(parsed.enforcementClaimed).toBe(false);
}

function acceptsUnknownSignalRows() {
  expect(
    BrowserGameRiskSignalSchema.safeParse(
      riskSignal({
        signalId: 'browser-game-riskSignal-manual',
        kind: 'unknown-risk',
        severity: 'unknown',
        state: 'manual-required',
        confidence: 'unknown',
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameBenefitSignalSchema.safeParse(
      benefitSignal({
        signalId: 'browser-game-benefit-signal-unavailable',
        kind: 'unknown-benefit',
        severity: 'unknown',
        state: 'unavailable',
        confidence: 'unknown',
      })
    ).success
  ).toBe(true);
}

function rejectsAuthorityClaims() {
  const invalidSignals = [
    { rawGamePayloadUsed: true },
    { rawChatContentUsed: true },
    { rawPageBodyUsed: true },
    { rawModelTextUsed: true },
    { accountOrPurchaseExecutionClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { nativeGameControlClaimed: true },
    { policyDecisionClaimed: true },
    { enforcementClaimed: true },
  ];
  const invalidSets = [
    { finalPolicyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { enforcementClaimed: true },
    { rawGamePayloadUsed: true },
    { rawChatContentUsed: true },
    { rawPageBodyUsed: true },
    { rawModelTextUsed: true },
    { accountOrPurchaseExecutionClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
  ];

  for (const invalid of invalidSignals) {
    expect(BrowserGameRiskSignalSchema.safeParse(riskSignal(invalid)).success).toBe(false);
    expect(BrowserGameBenefitSignalSchema.safeParse(benefitSignal(invalid)).success).toBe(false);
  }
  for (const invalid of invalidSets) {
    expect(BrowserGameRiskBenefitSignalSetSchema.safeParse(signalSet(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentSignalSets() {
  const invalidRows = [
    { riskSignals: [], benefitSignals: [] },
    { degradedState: 'degraded', confidence: 'high', uncertaintyReasons: ['low-confidence'] },
    { confidence: 'unknown' },
    { signalSourceKind: 'manual-required' },
    { recommendedPolicyInput: 'allow-candidate', riskSignals: [riskSignal({ severity: 'high' })] },
    { recommendedPolicyInput: 'block-candidate', riskSignals: [], benefitSignals: [benefitSignal()] },
    { riskSignals: [riskSignal({ kind: 'unknown-risk' })] },
    { benefitSignals: [benefitSignal({ severity: 'unknown' })] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameRiskBenefitSignalSetSchema.safeParse(signalSet(invalid)).success).toBe(false);
  }
}

function signalSet(overrides = {}): BrowserGameRiskBenefitSignalSet {
  return {
    schemaVersion: 'browser-game-riskbenefit-signal-contract',
    signalSetId: 'browser-game-signal-set-ugc-risk',
    familyId: 'family-browser-game-signals',
    childProfileId: 'child-browser-game-signals',
    deviceId: 'device-browser-game-signals',
    modeledAt: '2026-06-03T10:10:00.000Z',
    sourceEvidenceRefs: ['browser-game-route-evidence', 'browser-game-analysis-evidence'],
    signalSourceKind: 'game-ai-analysis',
    analysisRef: 'browser-game-ai-analysis-ref',
    metadataRef: 'browser-game-metadata-ref',
    parentRuleRef: 'browser-game-parent-rule-ref',
    riskSignals: [riskSignal()],
    benefitSignals: [benefitSignal()],
    recommendedPolicyInput: 'parent-review-candidate',
    confidence: 'medium',
    degradedState: 'none',
    uncertaintyReasons: [],
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    enforcementClaimed: false,
    rawGamePayloadUsed: false,
    rawChatContentUsed: false,
    rawPageBodyUsed: false,
    rawModelTextUsed: false,
    accountOrPurchaseExecutionClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    ...overrides,
  };
}

function riskSignal(overrides = {}) {
  return {
    signalId: 'browser-game-riskSignal-ugc',
    kind: 'user-generated-content-risk',
    severity: 'medium',
    state: 'candidate',
    confidence: 'medium',
    evidenceRefs: ['browser-game-analysis-evidence'],
    analysisRef: 'browser-game-ai-analysis-ref',
    rawGamePayloadUsed: false,
    rawChatContentUsed: false,
    rawPageBodyUsed: false,
    rawModelTextUsed: false,
    accountOrPurchaseExecutionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function benefitSignal(overrides = {}) {
  return {
    signalId: 'browser-game-benefit-signal-problem-solving',
    kind: 'problem-solving',
    severity: 'low',
    state: 'candidate',
    confidence: 'medium',
    evidenceRefs: ['browser-game-analysis-evidence'],
    analysisRef: 'browser-game-ai-analysis-ref',
    rawGamePayloadUsed: false,
    rawChatContentUsed: false,
    rawPageBodyUsed: false,
    rawModelTextUsed: false,
    accountOrPurchaseExecutionClaimed: false,
    cloudFrameAnalysisClaimed: false,
    nativeGameControlClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
