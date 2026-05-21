import { describe, expect, it } from 'vitest';
import { buildLocalAiEvidenceContext } from '../src/local-ai-context-builder';

const childProfile = { childProfileId: 'child-1', displayName: 'Sam' };
const device = { deviceId: 'device-1', childProfileId: 'child-1', label: 'Sam Windows PC', platform: 'windows' };
const observedAt = '2026-05-21T09:10:00.000Z';
const family = { familyId: 'family-1' };
const sourceEvidence = { evidenceReferenceId: 'journal-event-1', kind: 'journal-event', observedAt };
const ungroundedEvidence = { evidenceReferenceId: 'journal-event-ungrounded', kind: 'journal-event', observedAt };
const parentActionReference = {
  actionReferenceId: 'parent-action-memory-graph',
  actor: { actorId: 'parent-1', role: 'parent' },
  policyVersion: 'policy-v1',
  createdAt: observedAt,
};
const ungroundedParentActionReference = {
  actionReferenceId: 'parent-action-ungrounded-memory-graph',
  actor: { actorId: 'parent-1', role: 'parent' },
  policyVersion: 'policy-unselected',
  createdAt: observedAt,
};
const parentRuleContextReference = {
  parentRuleRefId: 'parent-rule-context-memory-graph',
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
  requestId: 'context-request-memory-graph',
  requestedAt: '2026-05-21T09:10:02.000Z',
  childProfile,
  device,
  requestedEvaluationKind: 'mixed-context',
  requiredEvidenceKinds: ['browser'],
  parentRuleContextReferences: [parentRuleContextReference],
  modelTaskRequirements: [],
  allowedCustody: ['child-device-query-store'],
  promptVersion: 'prompt-v1',
};
const evidenceReference = {
  evidenceRefId: 'browser-ref-1',
  evidence: { evidenceReferenceId: 'stored-browser-ref-1', kind: 'query-store-summary', observedAt },
  evidenceKind: 'browser',
  sourceSchemaVersion: 'v0.6',
  observedAt,
  ingestedAt: '2026-05-21T09:10:01.000Z',
  freshUntil: '2026-05-21T09:15:00.000Z',
  sourceId: 'source-browser-ref-1',
  adapterId: 'adapter-browser-ref-1',
  device,
  childProfile,
  custody: 'child-device-query-store',
  retentionState: 'local',
  confidence: 0.85,
  confidenceKind: 'classifier',
  capabilityStatus: 'available',
  degradedReasons: [],
  unknownReasons: [],
  sourceEvidenceReferences: [sourceEvidence],
};

function assertExcludesUngroundedMemoryAndGraphReferences(): void {
  const result = buildLocalAiEvidenceContext({
    contextId: 'context-memory-graph',
    request: buildRequest,
    evidenceReferences: [evidenceReference],
    runtimeReferences: [],
    memoryReferences: [
      {
        memoryReferenceId: 'memory-grounded-1',
        kind: 'evidence-memory',
        sourceEvidenceReferences: [sourceEvidence],
        sourcePolicyVersion: null,
        sourceParentActionReferences: [],
        generatedAt: observedAt,
        confidence: 0.75,
        derivedIndexVersion: 'memory-index-v1',
      },
      {
        memoryReferenceId: 'memory-ungrounded-1',
        kind: 'evidence-memory',
        sourceEvidenceReferences: [ungroundedEvidence],
        sourcePolicyVersion: null,
        sourceParentActionReferences: [],
        generatedAt: observedAt,
        confidence: 0.75,
        derivedIndexVersion: 'memory-index-v1',
      },
    ],
    graphReferences: [
      {
        graphReferenceId: 'graph-grounded-1',
        kind: 'graph-edge',
        sourceEvidenceReferences: [sourceEvidence],
        sourcePolicyVersion: null,
        sourceParentActionReferences: [],
        generatedAt: observedAt,
        confidence: 0.7,
        derivedIndexVersion: 'graph-index-v1',
      },
      {
        graphReferenceId: 'graph-ungrounded-1',
        kind: 'graph-edge',
        sourceEvidenceReferences: [ungroundedEvidence],
        sourcePolicyVersion: null,
        sourceParentActionReferences: [],
        generatedAt: observedAt,
        confidence: 0.7,
        derivedIndexVersion: 'graph-index-v1',
      },
    ],
  });

  expect(result.state).toBe('partial');
  expect(result.context?.memoryReferences.map((reference) => reference.memoryReferenceId)).toEqual([
    'memory-grounded-1',
  ]);
  expect(result.context?.graphReferences.map((reference) => reference.graphReferenceId)).toEqual(['graph-grounded-1']);
  expect(result.context?.degradedReasons).toEqual(['memory-ungrounded', 'graph-ungrounded']);
  expect(result.context?.validationSummary.memoryReferenceCount).toBe(1);
  expect(result.context?.validationSummary.graphReferenceCount).toBe(1);
}

function assertKeepsOnlyGroundedPolicyAndParentActionReferences(): void {
  const result = buildLocalAiEvidenceContext({
    contextId: 'context-policy-action-memory-graph',
    request: buildRequest,
    evidenceReferences: [evidenceReference],
    runtimeReferences: [],
    memoryReferences: [
      {
        memoryReferenceId: 'memory-policy-grounded-1',
        kind: 'policy-memory',
        sourceEvidenceReferences: [],
        sourcePolicyVersion: 'policy-v1',
        sourceParentActionReferences: [],
        generatedAt: observedAt,
        confidence: 0.8,
        derivedIndexVersion: 'memory-index-v1',
      },
      {
        memoryReferenceId: 'memory-policy-ungrounded-1',
        kind: 'policy-memory',
        sourceEvidenceReferences: [],
        sourcePolicyVersion: 'policy-unselected',
        sourceParentActionReferences: [],
        generatedAt: observedAt,
        confidence: 0.8,
        derivedIndexVersion: 'memory-index-v1',
      },
    ],
    graphReferences: [
      {
        graphReferenceId: 'graph-action-grounded-1',
        kind: 'graph-edge',
        sourceEvidenceReferences: [],
        sourcePolicyVersion: null,
        sourceParentActionReferences: [parentActionReference],
        generatedAt: observedAt,
        confidence: 0.7,
        derivedIndexVersion: 'graph-index-v1',
      },
      {
        graphReferenceId: 'graph-action-ungrounded-1',
        kind: 'graph-edge',
        sourceEvidenceReferences: [],
        sourcePolicyVersion: null,
        sourceParentActionReferences: [ungroundedParentActionReference],
        generatedAt: observedAt,
        confidence: 0.7,
        derivedIndexVersion: 'graph-index-v1',
      },
    ],
  });

  expect(result.context?.memoryReferences.map((reference) => reference.memoryReferenceId)).toEqual([
    'memory-policy-grounded-1',
  ]);
  expect(result.context?.graphReferences.map((reference) => reference.graphReferenceId)).toEqual([
    'graph-action-grounded-1',
  ]);
  expect(result.context?.degradedReasons).toEqual(['memory-ungrounded', 'graph-ungrounded']);
}

describe('local AI memory and graph read-path selection', () => {
  it('excludes memory and graph references that are not grounded in selected evidence', () => {
    assertExcludesUngroundedMemoryAndGraphReferences();
  });

  it('keeps policy and parent-action memory only when it matches selected parent-rule context', () => {
    assertKeepsOnlyGroundedPolicyAndParentActionReferences();
  });
});
