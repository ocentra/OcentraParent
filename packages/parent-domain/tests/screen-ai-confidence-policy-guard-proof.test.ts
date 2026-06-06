import { describe, expect, it } from 'vitest';
import {
  ScreenAiConfidencePolicyGuardProofSchema,
  buildScreenAiConfidencePolicyGuardProof,
  screenAiConfidencePolicyGuardSummary,
} from '../src/screen-ai-confidence-policy-guard-proof';

const generatedAt = '2026-06-06T04:55:00.000Z';
const confidenceThreshold = 0.8;

const evidenceReference = {
  evidenceReferenceId: 'screen-evidence:confidence-policy-guard',
  kind: 'activity-event',
  observedAt: generatedAt,
} as const;

const readyRuntime = {
  runtimeReferenceId: 'runtime:screen-confidence-policy-guard',
  providerId: 'screen-local-provider',
  modelId: 'screen-child-safety-v1',
  modelReference: 'artifact:screen-child-safety-v1',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: generatedAt,
  unavailableReason: null,
} as const;

function localAiResult(overrides: Record<string, unknown>) {
  return {
    schemaVersion: 'v0.6',
    resultId: 'screen-ai-result:confidence-policy-guard',
    requestId: 'screen-ai-request:confidence-policy-guard',
    action: 'warn',
    confidence: 0.91,
    unknownState: 'none',
    degradedState: 'none',
    reasonCodes: ['screen-ai:confidence-policy-guard'],
    explanationReference: 'explanation:screen-ai-confidence-policy-guard',
    evidenceReferences: [evidenceReference],
    parentRuleReferences: ['policy-rule:screen-confidence'],
    memoryReferences: [],
    graphReferences: [],
    modelRuntime: readyRuntime,
    promptVersion: 'screen-safety-template-v1',
    expiresAt: null,
    ...overrides,
  };
}

function policyDecision(localAiResultId: string, overrides: Record<string, unknown>) {
  return {
    schemaVersion: 'v0.6',
    decisionId: `policy-decision:${localAiResultId}`,
    action: 'warn',
    reasonCodes: ['screen-ai:confidence-policy-guard'],
    evidenceReferences: [evidenceReference],
    ruleIds: ['policy-rule:screen-confidence'],
    localAiResultId,
    dryRun: true,
    enforcementHandoffState: 'disabled',
    expiresAt: null,
    ...overrides,
  };
}

const highConfidenceRow = {
  rowId: 'screen-ai-confidence-row:high',
  sourceEvidenceRef: 'screen-evidence:confidence-policy-guard:high',
  localAiResult: localAiResult({ resultId: 'screen-ai-result:confidence-high', confidence: 0.91 }),
  policyDecision: policyDecision('screen-ai-result:confidence-high', {}),
  confidenceThreshold,
  confidenceBand: 'high',
  guardOutcome: 'policy-ready',
  policyEligible: true,
  enforcementAllowed: false,
  remoteProviderUsed: false,
  rawImageRetained: false,
} as const;

const mediumConfidenceParentReviewRow = {
  rowId: 'screen-ai-confidence-row:medium',
  sourceEvidenceRef: 'screen-evidence:confidence-policy-guard:medium',
  localAiResult: localAiResult({
    resultId: 'screen-ai-result:confidence-medium',
    action: 'ask-parent',
    confidence: 0.62,
    reasonCodes: ['screen-ai:confidence-below-auto-policy-threshold'],
    explanationReference: 'explanation:screen-ai-confidence-medium',
  }),
  policyDecision: policyDecision('screen-ai-result:confidence-medium', {
    action: 'ask-parent',
    reasonCodes: ['screen-ai:confidence-below-auto-policy-threshold'],
  }),
  confidenceThreshold,
  confidenceBand: 'medium',
  guardOutcome: 'parent-review-required',
  policyEligible: true,
  enforcementAllowed: false,
  remoteProviderUsed: false,
  rawImageRetained: false,
} as const;

const lowConfidenceManualRow = {
  rowId: 'screen-ai-confidence-row:low',
  sourceEvidenceRef: 'screen-evidence:confidence-policy-guard:low',
  localAiResult: localAiResult({
    resultId: 'screen-ai-result:confidence-low',
    action: 'unknown',
    confidence: 0.31,
    unknownState: 'low-confidence',
    reasonCodes: ['screen-ai:low-confidence-manual-review'],
    explanationReference: null,
  }),
  policyDecision: policyDecision('screen-ai-result:confidence-low', {
    action: 'unknown',
    reasonCodes: ['screen-ai:low-confidence-manual-review'],
    enforcementHandoffState: 'not-requested',
  }),
  confidenceThreshold,
  confidenceBand: 'low',
  guardOutcome: 'manual-required',
  policyEligible: false,
  enforcementAllowed: false,
  remoteProviderUsed: false,
  rawImageRetained: false,
} as const;

const unknownConfidenceManualRow = {
  ...lowConfidenceManualRow,
  rowId: 'screen-ai-confidence-row:unknown',
  sourceEvidenceRef: 'screen-evidence:confidence-policy-guard:unknown',
  localAiResult: localAiResult({
    resultId: 'screen-ai-result:confidence-unknown',
    action: 'unknown',
    confidence: 0,
    unknownState: 'model-unavailable',
    degradedState: 'provider-unavailable',
    reasonCodes: ['screen-ai:confidence-unavailable'],
    explanationReference: null,
    modelRuntime: {
      ...readyRuntime,
      executionState: 'failed',
      loadState: 'failed',
      degradedState: 'provider-unavailable',
      unavailableReason: 'screen-ai-confidence-unavailable',
    },
  }),
  policyDecision: policyDecision('screen-ai-result:confidence-unknown', {
    action: 'unknown',
    reasonCodes: ['screen-ai:confidence-unavailable'],
    enforcementHandoffState: 'not-requested',
  }),
  confidenceBand: 'unknown',
} as const;

const readyProof = {
  schemaVersion: 'v0.6',
  proofId: 'screen-ai-confidence-policy-guard-proof',
  generatedAt,
  rows: [highConfidenceRow, mediumConfidenceParentReviewRow, lowConfidenceManualRow, unknownConfidenceManualRow],
} as const;

function parsesWithRows(rows: readonly unknown[]): boolean {
  return ScreenAiConfidencePolicyGuardProofSchema.safeParse({ ...readyProof, rows }).success;
}

describe('screen AI confidence policy guard proof', () => {
  it('summarizes high, medium, low, and unknown confidence outcomes without enforcement authority', () => {
    expectReadyConfidenceSummary();
  });

  it('rejects low-confidence allow or block results before policy handoff', () => {
    expectLowConfidenceAllowAndBlockRejected();
  });

  it('rejects confidence rows that request enforcement or remote/raw custody', () => {
    expectEnforcementRemoteAndRawCustodyRejected();
  });

  it('requires policy decisions to cite the same local AI result and remain dry-run', () => {
    expectPolicyResultReferenceAndDryRunRequired();
  });
});

function expectReadyConfidenceSummary(): void {
  const proof = buildScreenAiConfidencePolicyGuardProof(readyProof);
  const summary = screenAiConfidencePolicyGuardSummary(proof);

  expect(summary.totalRows).toBe(4);
  expect(summary.policyReadyRows).toBe(1);
  expect(summary.parentReviewRows).toBe(1);
  expect(summary.manualRequiredRows).toBe(2);
  expect(summary.lowConfidenceRows).toBe(1);
  expect(summary.unsafeAllowOrBlockRows).toBe(0);
  expect(summary.enforcementAllowedRows).toBe(0);
}

function expectLowConfidenceAllowAndBlockRejected(): void {
  const unsafeLowAllowRow = {
    ...lowConfidenceManualRow,
    localAiResult: {
      ...lowConfidenceManualRow.localAiResult,
      action: 'allow',
    },
  };
  const unsafeLowBlockDecisionRow = {
    ...lowConfidenceManualRow,
    policyDecision: {
      ...lowConfidenceManualRow.policyDecision,
      action: 'block',
    },
  };

  expect(
    parsesWithRows([highConfidenceRow, mediumConfidenceParentReviewRow, unsafeLowAllowRow, unknownConfidenceManualRow])
  ).toBe(false);
  expect(
    parsesWithRows([
      highConfidenceRow,
      mediumConfidenceParentReviewRow,
      unsafeLowBlockDecisionRow,
      unknownConfidenceManualRow,
    ])
  ).toBe(false);
}

function expectEnforcementRemoteAndRawCustodyRejected(): void {
  const enforcementRow = {
    ...mediumConfidenceParentReviewRow,
    enforcementAllowed: true,
  };
  const remoteProviderRow = {
    ...highConfidenceRow,
    remoteProviderUsed: true,
  };
  const rawRetainedRow = {
    ...lowConfidenceManualRow,
    rawImageRetained: true,
  };

  expect(parsesWithRows([highConfidenceRow, enforcementRow, lowConfidenceManualRow, unknownConfidenceManualRow])).toBe(
    false
  );
  expect(
    parsesWithRows([
      remoteProviderRow,
      mediumConfidenceParentReviewRow,
      lowConfidenceManualRow,
      unknownConfidenceManualRow,
    ])
  ).toBe(false);
  expect(
    parsesWithRows([highConfidenceRow, mediumConfidenceParentReviewRow, rawRetainedRow, unknownConfidenceManualRow])
  ).toBe(false);
}

function expectPolicyResultReferenceAndDryRunRequired(): void {
  const mismatchedResultRow = {
    ...highConfidenceRow,
    policyDecision: policyDecision('screen-ai-result:wrong-result', {}),
  };
  const livePolicyDecisionRow = {
    ...highConfidenceRow,
    policyDecision: {
      ...highConfidenceRow.policyDecision,
      dryRun: false,
    },
  };

  expect(
    parsesWithRows([
      mismatchedResultRow,
      mediumConfidenceParentReviewRow,
      lowConfidenceManualRow,
      unknownConfidenceManualRow,
    ])
  ).toBe(false);
  expect(
    parsesWithRows([
      livePolicyDecisionRow,
      mediumConfidenceParentReviewRow,
      lowConfidenceManualRow,
      unknownConfidenceManualRow,
    ])
  ).toBe(false);
}
