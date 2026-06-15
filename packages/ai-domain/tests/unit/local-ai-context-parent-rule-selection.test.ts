import { describe, expect, it } from 'vitest';
import { buildLocalAiEvidenceContext } from '../../src/local-ai-context-builder';

const childProfile = { childProfileId: 'child-1', displayName: 'Sam' };
const device = { deviceId: 'device-1', childProfileId: 'child-1', label: 'Sam Windows PC', platform: 'windows' };
const observedAt = '2026-05-21T09:10:00.000Z';
const sourceEvidence = { evidenceReferenceId: 'journal-event-1', kind: 'journal-event', observedAt };
const parentRuleContextReference = {
  parentRuleRefId: 'parent-rule-context-network',
  policyVersion: 'policy-v1',
  family: { familyId: 'family-1' },
  childProfile,
  device,
  rule: {
    ruleId: 'rule-network-digest',
    target: { targetId: 'target-network-1', targetType: 'category', targetValue: 'network-risk' },
    action: 'warn',
    scheduleId: null,
    priority: 10,
    reasonCode: 'parent-rule-network-risk',
    createdBy: { actorId: 'parent-1', role: 'parent' },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  },
  targetEvidenceRefs: ['network-ref-1'],
  custody: 'parent-device-cache',
  updatedAt: observedAt,
  expiresAt: null,
};

function evidenceReference(evidenceRefId: string, evidenceKind: string, custody = 'child-device-query-store') {
  return {
    evidenceRefId,
    evidence: { evidenceReferenceId: `stored-${evidenceRefId}`, kind: 'query-store-summary', observedAt },
    evidenceKind,
    sourceSchemaVersion: 'v0.6',
    observedAt,
    ingestedAt: '2026-05-21T09:10:01.000Z',
    freshUntil: '2026-05-21T09:15:00.000Z',
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

describe('local AI parent rule context selection', () => {
  it('degrades parent rules that point at filtered evidence refs', () => {
    const result = buildLocalAiEvidenceContext({
      contextId: 'context-parent-rule-selection',
      request: {
        schemaVersion: 'v0.6',
        requestId: 'context-request-parent-rule-selection',
        requestedAt: '2026-05-21T09:10:02.000Z',
        childProfile,
        device,
        requestedEvaluationKind: 'mixed-context',
        requiredEvidenceKinds: ['browser'],
        parentRuleContextReferences: [parentRuleContextReference],
        modelTaskRequirements: [],
        allowedCustody: ['child-device-query-store'],
        promptVersion: 'prompt-v1',
      },
      evidenceReferences: [
        evidenceReference('browser-ref-1', 'browser'),
        evidenceReference('network-ref-1', 'network-flow', 'parent-owned-export'),
      ],
      runtimeReferences: [],
      memoryReferences: [],
      graphReferences: [],
    });

    expect(result.state).toBe('partial');
    expect(result.context?.parentRuleReferences).toEqual([]);
    expect(result.context?.degradedReasons).toEqual(['custody-unavailable', 'parent-rule-missing']);
    expect(result.context?.validationSummary.parentRuleReferenceCount).toBe(0);
    expect(result.context?.validationSummary.ungroundedParentRuleReferenceCount).toBe(1);
  });
});
