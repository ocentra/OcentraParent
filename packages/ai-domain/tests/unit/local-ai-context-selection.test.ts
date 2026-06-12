import { describe, expect, it } from 'vitest';
import { buildLocalAiEvidenceContext } from '../../src/local-ai-context-builder';

const childProfile = { childProfileId: 'child-1', displayName: 'Sam' };
const device = { deviceId: 'device-1', childProfileId: 'child-1', label: 'Sam Windows PC', platform: 'windows' };
const observedAt = '2026-05-21T09:10:00.000Z';
const ingestedAt = '2026-05-21T09:10:01.000Z';
const freshUntil = '2026-05-21T09:15:00.000Z';
const family = { familyId: 'family-1' };
const sourceEvidence = {
  evidenceReferenceId: 'journal-event-1',
  kind: 'journal-event',
  observedAt,
};
const parentRuleContextReference = {
  parentRuleRefId: 'parent-rule-context-selection',
  policyVersion: 'policy-v1',
  family,
  childProfile,
  device,
  rule: {
    ruleId: 'rule-safe-search',
    target: { targetId: 'target-browser-1', targetType: 'category', targetValue: 'browser-safety' },
    action: 'warn',
    scheduleId: null,
    priority: 10,
    reasonCode: 'parent-rule-browser-safety',
    createdBy: { actorId: 'parent-1', role: 'parent' },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  },
  targetEvidenceRefs: ['browser-ref-1'],
  custody: 'parent-device-cache',
  updatedAt: observedAt,
  expiresAt: null,
};
const buildRequest = {
  schemaVersion: 'v0.6',
  requestId: 'context-request-selection',
  requestedAt: '2026-05-21T09:10:02.000Z',
  childProfile,
  device,
  requestedEvaluationKind: 'mixed-context',
  requiredEvidenceKinds: ['browser'],
  parentRuleContextReferences: [parentRuleContextReference],
  modelTaskRequirements: ['safety-decision'],
  allowedCustody: ['child-device-query-store'],
  promptVersion: 'prompt-v1',
};
const loadedRuntimeStatus = {
  runtimeReferenceId: 'runtime-1',
  providerId: 'local-provider',
  modelId: 'safety-model',
  modelReference: 'local-model-cache/safety-model',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['safety-decision', 'classification'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: '2026-05-21T09:09:00.000Z',
  unavailableReason: null,
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
    retentionState: 'local',
    confidence: 0.85,
    confidenceKind: 'classifier',
    capabilityStatus: 'available',
    degradedReasons: [],
    unknownReasons: [],
    sourceEvidenceReferences: [sourceEvidence],
  };
}

function storedContextInput(overrides = {}) {
  return {
    contextId: 'context-selection',
    request: buildRequest,
    evidenceReferences: [evidenceReference('browser-ref-1', 'browser')],
    runtimeReferences: [loadedRuntimeStatus],
    memoryReferences: [],
    graphReferences: [],
    ...overrides,
  };
}

describe('local AI evidence context read-path selection', () => {
  it('degrades unallowed custody while keeping allowed evidence grouped', () => {
    const result = buildLocalAiEvidenceContext(
      storedContextInput({
        evidenceReferences: [
          evidenceReference('browser-ref-1', 'browser'),
          evidenceReference('network-ref-1', 'network-flow', 'parent-owned-export'),
        ],
      })
    );

    expect(result.state).toBe('partial');
    expect(result.context?.browserEvidenceRefs).toEqual(['browser-ref-1']);
    expect(result.context?.networkFlowEvidenceRefs).toEqual([]);
    expect(result.context?.evidenceReferences.map((reference) => reference.evidenceRefId)).toEqual(['browser-ref-1']);
    expect(result.context?.degradedReasons).toEqual(['custody-unavailable']);
    expect(result.context?.validationSummary.unallowedCustodyReferenceCount).toBe(1);
    expect(result.rejectedFields).toEqual(['evidenceReferences']);
    expect(result.degradedSourceRefs).toEqual(['network-ref-1']);
    expect(result.auditEvidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
      'stored-browser-ref-1',
      'stored-network-ref-1',
    ]);
  });

  it('degrades unavailable runtime status instead of inventing a model runtime ref', () => {
    const result = buildLocalAiEvidenceContext(
      storedContextInput({
        runtimeReferences: [
          {
            ...loadedRuntimeStatus,
            runtimeReferenceId: 'runtime-loading-1',
            loadState: 'loading',
          },
        ],
      })
    );

    expect(result.state).toBe('partial');
    expect(result.context?.localModelRuntimeRefs).toEqual([]);
    expect(result.context?.degradedReasons).toEqual(['model-unavailable']);
    expect(result.context?.validationSummary.runtimeReferenceCount).toBe(0);
  });
});
