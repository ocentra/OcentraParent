import { describe, expect, it } from 'vitest';
import {
  buildScreenAiConfidencePolicyProof,
  ScreenAiConfidencePolicyInputSchema,
} from '../src/screen-ai-confidence-policy-proof';

const evidenceReference = {
  evidenceReferenceId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-activity-row',
  kind: 'activity-event',
  observedAt: '2026-06-05T15:59:25.662Z',
};

const baseRuntime = {
  runtimeReferenceId: 'windows-winrt-ocr-local-runtime',
  providerId: 'windows-winrt-ocr-provider',
  modelId: 'windows-winrt-ocr',
  modelReference: 'windows-winrt-ocr-local-model',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'os-capability-probe',
  loadState: 'loaded',
  capabilityFlags: ['classification', 'safety-decision'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: '2026-06-05T15:59:25.662Z',
  unavailableReason: null,
};

const parentBlockRule = {
  ruleId: 'screen-service-winrt-ocr-school-rule',
  target: {
    targetId: 'screen-category-target-school',
    targetType: 'category',
    targetValue: 'school',
  },
  action: 'block',
  scheduleId: null,
  priority: 100,
  reasonCode: 'parent-explicit-school-block',
  createdBy: {
    actorId: 'screen-ai-proof-parent',
    role: 'parent',
  },
  enabled: true,
  effectiveFrom: null,
  effectiveUntil: null,
};

const parentAllowRule = {
  ...parentBlockRule,
  ruleId: 'screen-service-winrt-ocr-school-allow-rule',
  action: 'allow',
  reasonCode: 'parent-explicit-school-allow',
};

const baseLocalAiResult = {
  schemaVersion: 'v0.6',
  resultId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-local-ocr-result',
  requestId: 'screen-service-queue-job-1780675160-1-local-ai-request',
  action: 'allow',
  confidence: 0.91,
  unknownState: 'none',
  degradedState: 'none',
  reasonCodes: ['screen-service-winrt-ocr-school-allow'],
  explanationReference: 'screen-service-winrt-ocr-school-explanation',
  evidenceReferences: [evidenceReference],
  parentRuleReferences: [parentBlockRule.ruleId],
  memoryReferences: [],
  graphReferences: [],
  modelRuntime: baseRuntime,
  promptVersion: 'screen-ocr-worker-winrt-v1',
  expiresAt: null,
};

const baseSourcePolicyDecision = {
  schemaVersion: 'v0.6',
  decisionId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-policy-dry-run',
  action: 'allow',
  reasonCodes: ['screen-service-winrt-ocr-school-allow'],
  evidenceReferences: [evidenceReference],
  ruleIds: [parentBlockRule.ruleId],
  localAiResultId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-local-ocr-result',
  dryRun: true,
  enforcementHandoffState: 'disabled',
  expiresAt: null,
};

const baseInput = {
  schemaVersion: 'v0.6',
  proofId: 'screen-ai-confidence-policy-proof',
  evaluatedAt: '2026-06-05T20:22:00.000Z',
  localAiResult: baseLocalAiResult,
  parentRule: parentBlockRule,
  sourcePolicyDecision: baseSourcePolicyDecision,
  minimumConfidence: 0.7,
  claimBoundaries: {
    remoteAiUsed: false,
    apiAiUsed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    rawEvidenceEmbedded: false,
    modelQualityClaimed: false,
  },
};

describe('screen AI confidence policy proof', () => {
  it('preserves a stricter parent block rule when local screen AI suggests allow', preservesStricterParentBlock);
  it(
    'turns low-confidence AI allow output into an unknown dry-run decision under an allow rule',
    turnsLowConfidenceAllowIntoUnknown
  );
  it('keeps degraded AI output dry-run and non-enforcing', keepsDegradedOutputNonEnforcing);
  it('rejects local AI results without cited screen evidence', rejectsMissingScreenEvidence);
  it('rejects remote or enforcement overclaims in the proof boundary', rejectsRemoteOverclaim);
});

function preservesStricterParentBlock() {
  const proof = buildScreenAiConfidencePolicyProof(baseInput);

  expect(proof.confidenceState).toBe('trusted-confidence');
  expect(proof.aiSuggestedAction).toBe('allow');
  expect(proof.parentRuleAction).toBe('block');
  expect(proof.selectedPolicyAction).toBe('block');
  expect(proof.policyDecision.action).toBe('block');
  expect(proof.policyDecision.ruleIds).toEqual(['screen-service-winrt-ocr-school-rule']);
  expect(proof.policyDecision.reasonCodes).toEqual(['parent-explicit-school-block']);
  expect(proof.proofReasons).toEqual([
    'local-ai-result-schema-valid',
    'confidence-threshold-applied',
    'parent-rule-cited',
    'dry-run-policy-only',
    'remote-ai-not-used',
    'raw-evidence-not-embedded',
    'stricter-parent-rule-preserved',
  ]);
}

function turnsLowConfidenceAllowIntoUnknown() {
  const proof = buildScreenAiConfidencePolicyProof({
    ...baseInput,
    parentRule: parentAllowRule,
    sourcePolicyDecision: {
      ...baseSourcePolicyDecision,
      ruleIds: [parentAllowRule.ruleId],
    },
    localAiResult: {
      ...baseLocalAiResult,
      confidence: 0.36,
      unknownState: 'low-confidence',
      parentRuleReferences: [parentAllowRule.ruleId],
    },
  });

  expect(proof.confidenceState).toBe('low-confidence-fallback');
  expect(proof.selectedPolicyAction).toBe('unknown');
  expect(proof.policyDecision.action).toBe('unknown');
  expect(proof.policyDecision.reasonCodes).toEqual([
    'parent-explicit-school-allow',
    'screen-ai-low-confidence-fallback',
  ]);
  expect(proof.proofReasons).toContain('low-confidence-cannot-allow');
}

function keepsDegradedOutputNonEnforcing() {
  const proof = buildScreenAiConfidencePolicyProof(degradedInput());

  expect(proof.confidenceState).toBe('degraded-fallback');
  expect(proof.selectedPolicyAction).toBe('unknown');
  expect(proof.policyDecision.dryRun).toBe(true);
  expect(proof.policyDecision.enforcementHandoffState).toBe('disabled');
  expect(proof.proofReasons).toContain('degraded-output-cannot-enforce');
}

function degradedInput() {
  return {
    ...baseInput,
    parentRule: parentAllowRule,
    sourcePolicyDecision: {
      ...baseSourcePolicyDecision,
      ruleIds: [parentAllowRule.ruleId],
    },
    localAiResult: {
      ...baseLocalAiResult,
      confidence: 0.88,
      action: 'block',
      unknownState: 'model-unavailable',
      degradedState: 'provider-unavailable',
      parentRuleReferences: [parentAllowRule.ruleId],
      modelRuntime: {
        ...baseRuntime,
        loadState: 'unavailable',
        degradedState: 'provider-unavailable',
        unavailableReason: 'winrt-ocr-provider-unavailable',
      },
    },
  };
}

function rejectsMissingScreenEvidence() {
  const result = ScreenAiConfidencePolicyInputSchema.safeParse({
    ...baseInput,
    localAiResult: {
      ...baseLocalAiResult,
      evidenceReferences: [],
    },
  });

  expect(result.success).toBe(false);
}

function rejectsRemoteOverclaim() {
  const result = ScreenAiConfidencePolicyInputSchema.safeParse({
    ...baseInput,
    claimBoundaries: {
      ...baseInput.claimBoundaries,
      remoteAiUsed: true,
    },
  });

  expect(result.success).toBe(false);
}
