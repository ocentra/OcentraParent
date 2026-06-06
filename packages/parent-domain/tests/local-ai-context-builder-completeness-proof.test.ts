import { describe, expect, it } from 'vitest';
import { buildLocalAiContextBuilderCompletenessProof } from '../src/local-ai-context-builder-completeness-proof';
import { LocalAiDegradedState } from '../src/local-ai-primitives';

const observedAt = '2026-06-06T08:58:00.000Z';
const ingestedAt = '2026-06-06T08:58:01.000Z';
const freshUntil = '2026-06-06T09:08:00.000Z';
const childProfile = { childProfileId: 'child-context-builder-complete', displayName: 'Sam' };
const device = {
  deviceId: 'device-context-builder-complete',
  childProfileId: 'child-context-builder-complete',
  label: 'Sam Windows PC',
  platform: 'windows',
};
const sourceEvidence = {
  evidenceReferenceId: 'context-builder-complete-source',
  kind: 'journal-event',
  observedAt,
};
const parentRuleContextReference = {
  parentRuleRefId: 'context-builder-parent-rule-context',
  policyVersion: 'context-builder-policy-v1',
  family: { familyId: 'family-context-builder-complete' },
  childProfile,
  device,
  rule: {
    ruleId: 'context-builder-rule',
    target: {
      targetId: 'context-builder-target',
      targetType: 'category',
      targetValue: 'screen-safety',
    },
    action: 'warn',
    scheduleId: null,
    priority: 10,
    reasonCode: 'parent-rule-browser-safety',
    createdBy: { actorId: 'parent-context-builder', role: 'parent' },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  },
  targetEvidenceRefs: ['screen-ref-context-builder'],
  custody: 'parent-device-cache',
  updatedAt: observedAt,
  expiresAt: null,
};
const request = {
  schemaVersion: 'v0.6',
  requestId: 'context-builder-complete-request',
  requestedAt: '2026-06-06T08:58:02.000Z',
  childProfile,
  device,
  requestedEvaluationKind: 'mixed-context',
  requiredEvidenceKinds: ['browser', 'app-game', 'network-flow', 'screen-summary'],
  parentRuleContextReferences: [parentRuleContextReference],
  modelTaskRequirements: ['safety-decision'],
  allowedCustody: ['child-device-query-store', 'child-device-journal'],
  promptVersion: 'context-builder-prompt-v1',
};
const runtimeReference = {
  runtimeReferenceId: 'runtime-context-builder-complete',
  providerId: 'local-provider-context-builder',
  modelId: 'local-model-context-builder',
  modelReference: 'artifact:local-context-builder-model',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: LocalAiDegradedState.None,
  lastCheckedAt: observedAt,
  unavailableReason: null,
};
const memoryReference = {
  memoryReferenceId: 'context-builder-complete-memory',
  kind: 'recent-activity',
  sourceEvidenceReferences: [sourceEvidence],
  sourcePolicyVersion: 'context-builder-policy-v1',
  generatedAt: observedAt,
  confidence: 0.82,
  derivedIndexVersion: 'context-builder-memory-index-v1',
};
const graphReference = {
  graphReferenceId: 'context-builder-complete-graph',
  kind: 'graph-edge',
  sourceEvidenceReferences: [sourceEvidence],
  sourcePolicyVersion: 'context-builder-policy-v1',
  generatedAt: observedAt,
  confidence: 0.79,
  derivedIndexVersion: 'context-builder-graph-index-v1',
};
const claimBoundaries = {
  modelExecutionClaimed: false,
  modelQualityClaimed: false,
  policyAuthorityClaimed: false,
  enforcementClaimed: false,
  portalUiClaimed: false,
  remoteApiAiUsed: false,
  rawPromptRetained: false,
  rawEvidenceRetained: false,
};

function evidenceReference(evidenceRefId: string, evidenceKind: string, custody = 'child-device-query-store') {
  return {
    evidenceRefId,
    evidence: {
      evidenceReferenceId: `stored-${evidenceRefId}`,
      kind: 'query-store-summary',
      observedAt,
    },
    evidenceKind,
    sourceSchemaVersion: 'v0.6',
    observedAt,
    ingestedAt,
    freshUntil,
    sourceId: `source-${evidenceRefId}`,
    adapterId: `adapter-${evidenceRefId}`,
    device,
    childProfile,
    custody,
    retentionState: custody === 'ocentra-hosted-non-activity' ? 'unavailable' : 'local',
    confidence: 0.86,
    confidenceKind: 'classifier',
    capabilityStatus: custody === 'unavailable' ? 'unavailable' : 'available',
    degradedReasons: custody === 'unavailable' ? ['custody-unavailable'] : [],
    unknownReasons: [],
    sourceEvidenceReferences: [sourceEvidence],
  };
}

function contextInput(overrides: Record<string, unknown> = {}) {
  return {
    contextId: 'context-builder-complete-context',
    request,
    evidenceReferences: [
      evidenceReference('browser-ref-context-builder', 'browser'),
      evidenceReference('app-game-ref-context-builder', 'app-game'),
      evidenceReference('network-ref-context-builder', 'network-flow'),
      evidenceReference('screen-ref-context-builder', 'screen-summary'),
    ],
    runtimeReferences: [runtimeReference],
    memoryReferences: [memoryReference],
    graphReferences: [graphReference],
    ...overrides,
  };
}

function proofInput(overrides: Record<string, unknown> = {}) {
  return {
    schemaVersion: 'v0.6',
    readyInput: contextInput(),
    partialInput: contextInput({
      evidenceReferences: [evidenceReference('screen-ref-context-builder', 'screen-summary')],
    }),
    forbiddenCustodyInput: contextInput({
      evidenceReferences: [evidenceReference('hosted-ref-context-builder', 'browser', 'ocentra-hosted-non-activity')],
    }),
    unallowedCustodyInput: contextInput({
      evidenceReferences: [evidenceReference('export-ref-context-builder', 'browser', 'parent-owned-export')],
    }),
    unavailableRuntimeInput: contextInput({ runtimeReferences: [] }),
    claimBoundaries,
    ...overrides,
  };
}

describe('local AI context builder completeness proof', () => {
  it('proves ready context preserves selected evidence, rules, runtime, memory, graph, and prompt refs', () => {
    const proof = buildLocalAiContextBuilderCompletenessProof(proofInput());

    expect(proof.readyResult.state).toBe('ready');
    expect(proof.readyResult.context?.browserEvidenceRefs).toEqual(['browser-ref-context-builder']);
    expect(proof.readyResult.context?.appGameEvidenceRefs).toEqual(['app-game-ref-context-builder']);
    expect(proof.readyResult.context?.networkFlowEvidenceRefs).toEqual(['network-ref-context-builder']);
    expect(proof.readyResult.context?.screenSummaryRefs).toEqual(['screen-ref-context-builder']);
    expect(proof.readyResult.context?.parentRuleReferences).toEqual(['context-builder-rule']);
    expect(proof.readyResult.context?.localModelRuntimeRefs).toEqual(['runtime-context-builder-complete']);
    expect(proof.readyResult.context?.memoryReferences.map((reference) => reference.memoryReferenceId)).toEqual([
      'context-builder-complete-memory',
    ]);
    expect(proof.readyResult.context?.graphReferences.map((reference) => reference.graphReferenceId)).toEqual([
      'context-builder-complete-graph',
    ]);
    expect(proof.readyResult.context?.promptVersion).toBe('context-builder-prompt-v1');
    expect(proof.summary).toEqual({
      readyEvidenceReferenceCount: 4,
      readyRuntimeReferenceCount: 1,
      readyParentRuleReferenceCount: 1,
      readyMemoryReferenceCount: 1,
      readyGraphReferenceCount: 1,
      partialMissingEvidenceKindCount: 3,
      rejectedForbiddenCustodyCount: 1,
      rejectedUnallowedCustodyCount: 1,
      unavailableRuntimeDegradedCount: 1,
    });
  });

  it('proves partial, forbidden custody, unallowed custody, and unavailable runtime states stay typed', () => {
    const proof = buildLocalAiContextBuilderCompletenessProof(proofInput());

    expect(proof.partialResult.state).toBe('partial');
    expect(proof.partialResult.missingEvidenceKinds).toEqual(['browser', 'app-game', 'network-flow']);
    expect(proof.forbiddenCustodyResult.state).toBe('rejected');
    expect(proof.forbiddenCustodyResult.degradedSourceRefs).toEqual(['hosted-ref-context-builder']);
    expect(proof.unallowedCustodyResult.state).toBe('rejected');
    expect(proof.unallowedCustodyResult.rejectedFields).toEqual(['evidenceReferences']);
    expect(proof.unavailableRuntimeResult.state).toBe('partial');
    expect(proof.unavailableRuntimeResult.context?.degradedReasons).toEqual(['model-unavailable']);
  });

  it('rejects UI, model-execution, policy-authority, enforcement, remote, prompt, or raw-evidence overclaims', () => {
    expect(() =>
      buildLocalAiContextBuilderCompletenessProof({
        ...proofInput(),
        claimBoundaries: { ...claimBoundaries, enforcementClaimed: true },
      })
    ).toThrow();
  });

  it('rejects a proof missing ready evidence completeness', () => {
    expect(() =>
      buildLocalAiContextBuilderCompletenessProof({
        ...proofInput(),
        readyInput: contextInput({ evidenceReferences: [] }),
      })
    ).toThrow('Expected local AI context builder completeness proof');
  });
});
