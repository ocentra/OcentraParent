import { describe, expect, it } from 'vitest';
import { buildScreenAiMemoryGraphSourceGuardProof } from '../src/screen-ai-memory-graph-source-guard-proof';

const observedAt = '2026-06-05T17:15:00.000Z';
const childProfile = { childProfileId: 'child-screen-ai-memory', displayName: 'Sam' };
const device = {
  deviceId: 'screen-ai-memory-device',
  childProfileId: 'child-screen-ai-memory',
  label: 'Sam Windows PC',
  platform: 'windows',
};
const storedScreenEvidence = {
  evidenceReferenceId: 'screen-ai-memory-stored-screen-summary',
  kind: 'query-store-summary',
  observedAt,
};
const rawCaptureAuditEvidence = {
  evidenceReferenceId: 'screen-ai-memory-raw-capture-deleted',
  kind: 'journal-event',
  observedAt,
};
const ungroundedEvidence = {
  evidenceReferenceId: 'screen-ai-memory-other-child-evidence',
  kind: 'activity-event',
  observedAt,
};
const screenEvidenceReference = {
  evidenceRefId: 'screen-ai-memory-context-ref',
  evidence: storedScreenEvidence,
  evidenceKind: 'screen-summary',
  sourceSchemaVersion: 'v0.6',
  observedAt,
  ingestedAt: '2026-06-05T17:15:02.000Z',
  freshUntil: '2026-06-05T17:20:00.000Z',
  sourceId: 'screen-ai-memory-screen-service',
  adapterId: 'screen-ai-memory-winrt-ocr',
  device,
  childProfile,
  custody: 'child-device-query-store',
  retentionState: 'local',
  confidence: 0.9,
  confidenceKind: 'model',
  capabilityStatus: 'available',
  degradedReasons: [],
  unknownReasons: [],
  sourceEvidenceReferences: [rawCaptureAuditEvidence],
};
const runtimeReference = {
  runtimeReferenceId: 'screen-ai-memory-runtime',
  providerId: 'screen-ai-memory-provider',
  modelId: 'screen-ai-memory-model',
  modelReference: 'screen-ai-memory-model-local-cache',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['classification', 'safety-decision'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: observedAt,
  unavailableReason: null,
};
const claimBoundaries = {
  remoteAiUsed: false,
  apiAiUsed: false,
  policyAuthorityClaimed: false,
  enforcementClaimed: false,
  uncitedMemoryAllowed: false,
  uncitedGraphAllowed: false,
  rawEvidenceEmbedded: false,
};
const parentRuleContextReference = {
  parentRuleRefId: 'screen-ai-memory-parent-rule-context',
  policyVersion: 'screen-ai-memory-policy-v1',
  family: { familyId: 'screen-ai-memory-family' },
  childProfile,
  device,
  rule: {
    ruleId: 'screen-ai-memory-rule',
    target: {
      targetId: 'screen-ai-memory-target',
      targetType: 'category',
      targetValue: 'screen-safety',
    },
    action: 'warn',
    scheduleId: null,
    priority: 10,
    reasonCode: 'screen-ai-memory-cited-context',
    createdBy: { actorId: 'screen-ai-memory-parent', role: 'parent' },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  },
  targetEvidenceRefs: ['screen-ai-memory-context-ref'],
  custody: 'child-device-query-store',
  updatedAt: observedAt,
  expiresAt: null,
};

function contextInput(memorySource = rawCaptureAuditEvidence, graphSource = rawCaptureAuditEvidence): unknown {
  return {
    contextId: 'screen-ai-memory-context',
    request: {
      schemaVersion: 'v0.6',
      requestId: 'screen-ai-memory-request',
      requestedAt: '2026-06-05T17:15:03.000Z',
      childProfile,
      device,
      requestedEvaluationKind: 'screen-summary',
      requiredEvidenceKinds: ['screen-summary'],
      parentRuleContextReferences: [parentRuleContextReference],
      modelTaskRequirements: ['classification', 'safety-decision'],
      allowedCustody: ['child-device-query-store'],
      promptVersion: 'screen-ai-memory-prompt-v1',
    },
    evidenceReferences: [screenEvidenceReference],
    runtimeReferences: [runtimeReference],
    memoryReferences: [
      {
        memoryReferenceId: 'screen-ai-memory-recent-activity',
        kind: 'recent-activity',
        sourceEvidenceReferences: [memorySource],
        sourcePolicyVersion: null,
        generatedAt: observedAt,
        confidence: 0.82,
        derivedIndexVersion: 'screen-ai-memory-index-v1',
      },
    ],
    graphReferences: [
      {
        graphReferenceId: 'screen-ai-memory-graph-edge',
        kind: 'graph-edge',
        sourceEvidenceReferences: [graphSource],
        sourcePolicyVersion: null,
        generatedAt: observedAt,
        confidence: 0.78,
        derivedIndexVersion: 'screen-ai-graph-index-v1',
      },
    ],
  };
}

function proofInput(input = contextInput(), boundaries = claimBoundaries): unknown {
  return {
    schemaVersion: 'v0.6',
    contextInput: input,
    claimBoundaries: boundaries,
  };
}

describe('screen AI memory graph source guard proof', () => {
  it('builds ready screen context only when memory and graph refs cite stored screen evidence', () => {
    const proof = buildScreenAiMemoryGraphSourceGuardProof(proofInput());

    expect(proof.contextResult.state).toBe('ready');
    expect(proof.contextResult.context?.screenSummaryRefs).toEqual(['screen-ai-memory-context-ref']);
    expect(proof.sourceGuardSummary).toMatchObject({
      evidenceReferenceCount: 1,
      sourceEvidenceReferenceCount: 1,
      memoryReferenceCount: 1,
      graphReferenceCount: 1,
      rejectedUncitedMemoryReferenceCount: 0,
      rejectedUncitedGraphReferenceCount: 0,
    });
    expect(proof.claimBoundaries.remoteAiUsed).toBe(false);
  });

  it('rejects memory references without a stored evidence citation', () => {
    expect(() =>
      buildScreenAiMemoryGraphSourceGuardProof(
        proofInput(contextInput({ evidenceReferenceId: '', kind: 'journal-event', observedAt }))
      )
    ).toThrow();
  });

  it('rejects graph references that cite evidence outside the selected screen context', () => {
    expect(() =>
      buildScreenAiMemoryGraphSourceGuardProof(proofInput(contextInput(rawCaptureAuditEvidence, ungroundedEvidence)))
    ).toThrow();
  });

  it('rejects remote, API, policy-authority, or enforcement overclaims', () => {
    expect(() =>
      buildScreenAiMemoryGraphSourceGuardProof(proofInput(contextInput(), { ...claimBoundaries, remoteAiUsed: true }))
    ).toThrow();
  });
});
