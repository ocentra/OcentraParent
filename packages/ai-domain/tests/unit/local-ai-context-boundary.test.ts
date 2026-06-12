import { describe, expect, it } from 'vitest';
import { buildLocalAiEvidenceContext } from '../../src/local-ai-context-builder';

const childProfile = { childProfileId: 'child-1', displayName: 'Sam' };
const device = { deviceId: 'device-1', childProfileId: 'child-1', label: 'Sam Windows PC', platform: 'windows' };
const observedAt = '2026-05-21T09:10:00.000Z';
const sourceEvidence = { evidenceReferenceId: 'journal-event-1', kind: 'journal-event', observedAt };
const parentRuleContextReference = {
  parentRuleRefId: 'parent-rule-context-boundary',
  policyVersion: 'policy-v1',
  family: { familyId: 'family-1' },
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
  requestId: 'context-request-boundary',
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

function evidenceReference(evidenceRefId: string, custody = 'child-device-query-store') {
  return {
    evidenceRefId,
    evidence: { evidenceReferenceId: `stored-${evidenceRefId}`, kind: 'query-store-summary', observedAt },
    evidenceKind: 'browser',
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
    confidence: null,
    confidenceKind: null,
    capabilityStatus: 'available',
    degradedReasons: [],
    unknownReasons: [],
    sourceEvidenceReferences: [sourceEvidence],
  };
}

function contextInput(request = buildRequest, references = [evidenceReference('browser-ref-1')]) {
  return {
    contextId: 'context-boundary',
    request,
    evidenceReferences: references,
    runtimeReferences: [],
    memoryReferences: [],
    graphReferences: [],
  };
}

describe('local AI evidence context boundary handling', () => {
  it('degrades when parent rule refs are missing from context', () => {
    const result = buildLocalAiEvidenceContext(contextInput({ ...buildRequest, parentRuleContextReferences: [] }));

    expect(result.state).toBe('partial');
    expect(result.context?.parentRuleReferences).toEqual([]);
    expect(result.context?.parentRuleContextReferences).toEqual([]);
    expect(result.context?.degradedReasons).toEqual(['parent-rule-missing']);
    expect(result.context?.validationSummary.parentRuleReferenceCount).toBe(0);
  });

  it('rejects Ocentra-hosted non-activity metadata as evidence', () => {
    const result = buildLocalAiEvidenceContext(
      contextInput(buildRequest, [evidenceReference('hosted-ref-1', 'ocentra-hosted-non-activity')])
    );

    expect(result.state).toBe('rejected');
    expect(result.context).toBeNull();
    expect(result.rejectedFields).toEqual(['evidenceReferences']);
    expect(result.degradedSourceRefs).toEqual(['hosted-ref-1']);
    expect(result.custodyBoundarySummary).toBe(
      'ocentra-hosted non-activity metadata cannot source child-activity evidence'
    );
  });

  it('rejects custody outside the request allowlist', () => {
    const result = buildLocalAiEvidenceContext(
      contextInput({ ...buildRequest, allowedCustody: ['child-device-journal'] }, [
        evidenceReference('parent-export-ref-1', 'parent-owned-export'),
      ])
    );

    expect(result.state).toBe('rejected');
    expect(result.context).toBeNull();
    expect(result.rejectedFields).toEqual(['evidenceReferences']);
    expect(result.degradedSourceRefs).toEqual(['parent-export-ref-1']);
    expect(result.custodyBoundarySummary).toBe('evidence custody was not allowed by context request');
    expect(result.validationGateSummary).toBe('rejected unallowed custody before local model input');
  });
});
