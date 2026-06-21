import { describe, expect, it } from 'vitest';
import { buildLocalAiEvidenceContext } from '@ocentra-parent/schema-domain/local-ai-context-builder';
import { LocalAiDegradedState } from '@ocentra-parent/schema-domain/ai-primitives';

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
  parentRuleRefId: 'parent-rule-context-1',
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
const runtimeStatus = {
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
  degradedState: LocalAiDegradedState.None,
  lastCheckedAt: '2026-05-21T09:09:00.000Z',
  unavailableReason: null,
};
const buildRequest = {
  schemaVersion: 'v0.6',
  requestId: 'context-request-1',
  requestedAt: '2026-05-21T09:10:02.000Z',
  childProfile,
  device,
  requestedEvaluationKind: 'mixed-context',
  requiredEvidenceKinds: ['browser', 'app-game', 'network-flow', 'screen-summary'],
  parentRuleContextReferences: [parentRuleContextReference],
  modelTaskRequirements: ['safety-decision'],
  allowedCustody: ['child-device-query-store', 'child-device-journal'],
  promptVersion: 'prompt-v1',
};

function evidenceReference(evidenceRefId: string, evidenceKind: string) {
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
    custody: 'child-device-query-store',
    retentionState: 'local',
    confidence: evidenceKind === 'browser' ? null : 0.85,
    confidenceKind: evidenceKind === 'browser' ? null : 'classifier',
    capabilityStatus: 'available',
    degradedReasons: [],
    unknownReasons: [],
    sourceEvidenceReferences: [sourceEvidence],
  };
}

const storedContextInput = {
  contextId: 'context-1',
  request: buildRequest,
  evidenceReferences: [
    evidenceReference('browser-ref-1', 'browser'),
    evidenceReference('app-game-ref-1', 'app-game'),
    evidenceReference('network-ref-1', 'network-flow'),
    evidenceReference('screen-ref-1', 'screen-summary'),
  ],
  runtimeReferences: [runtimeStatus],
  memoryReferences: [],
  graphReferences: [],
};

describe('local AI evidence context builder contracts', () => {
  it('buildLocalAiEvidenceContext: builds a ready context from stored evidence refs', () => {
    const result = buildLocalAiEvidenceContext(storedContextInput);

    expect(result.state).toBe('ready');
    expect(result.context?.browserEvidenceRefs).toEqual(['browser-ref-1']);
    expect(result.context?.appGameEvidenceRefs).toEqual(['app-game-ref-1']);
    expect(result.context?.networkFlowEvidenceRefs).toEqual(['network-ref-1']);
    expect(result.context?.screenSummaryRefs).toEqual(['screen-ref-1']);
    expect(result.context?.localModelRuntimeRefs).toEqual(['runtime-1']);
    expect(result.context?.parentRuleReferences).toEqual(['rule-safe-search']);
    expect(result.context?.parentRuleContextReferences).toEqual([parentRuleContextReference]);
    expect(result.context?.validationSummary).toEqual({
      evidenceReferenceCount: 4,
      sourceEvidenceReferenceCount: 4,
      runtimeReferenceCount: 1,
      memoryReferenceCount: 0,
      graphReferenceCount: 0,
      parentRuleReferenceCount: 1,
      ungroundedParentRuleReferenceCount: 0,
      forbiddenCustodyReferenceCount: 0,
      unallowedCustodyReferenceCount: 0,
    });
    expect(result.auditEvidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
      'stored-browser-ref-1',
      'stored-app-game-ref-1',
      'stored-network-ref-1',
      'stored-screen-ref-1',
    ]);
  });

  it('buildLocalAiEvidenceContext: returns partial when requested stored evidence is missing', () => {
    const result = buildLocalAiEvidenceContext({
      ...storedContextInput,
      evidenceReferences: [evidenceReference('browser-ref-1', 'browser')],
    });

    expect(result.state).toBe('partial');
    expect(result.context?.browserEvidenceRefs).toEqual(['browser-ref-1']);
    expect(result.missingEvidenceKinds).toEqual(['app-game', 'network-flow', 'screen-summary']);
    expect(result.validationGateSummary).toBe('partial context built with explicit missing evidence kinds');
  });
});
