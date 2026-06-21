import { describe, expect, it } from 'vitest';
import { buildLocalAiEvidenceContext } from '@ocentra-parent/schema-domain/local-ai-context-builder';

const ObservedAt = '2026-06-05T10:22:19.824Z';
const ChildProfile = { childProfileId: 'screen-summary-child', displayName: 'Sam' };
const Device = {
  deviceId: 'screen-summary-windows-device',
  childProfileId: ChildProfile.childProfileId,
  label: 'Sam Windows PC',
  platform: 'windows',
};
const SourceEvidence = {
  evidenceReferenceId: 'screen-winrt-ocr-evidence-live-wikipedia-browser-ocr',
  kind: 'journal-event',
  observedAt: ObservedAt,
};
const RuntimeStatus = {
  runtimeReferenceId: 'screen-summary-context-runtime',
  providerId: 'screen-summary-local-provider',
  modelId: 'screen-summary-local-safety-model',
  modelReference: 'local-model-cache/screen-summary-local-safety-model',
  privacyMode: 'local-only',
  adapterBoundary: 'local-adapter-ready',
  executionState: 'dry-run-ready',
  providerSource: 'local-model-cache',
  loadState: 'loaded',
  capabilityFlags: ['classification', 'safety-decision'],
  resourceClass: 'cpu',
  degradedState: 'none',
  lastCheckedAt: ObservedAt,
  unavailableReason: null,
} as const;
const ParentRuleContextReference = {
  parentRuleRefId: 'screen-summary-parent-rule-context',
  policyVersion: 'screen-summary-policy-v1',
  family: { familyId: 'screen-summary-family' },
  childProfile: ChildProfile,
  device: Device,
  rule: {
    ruleId: 'screen-summary-school-rule',
    target: { targetId: 'screen-summary-school-target', targetType: 'category', targetValue: 'school' },
    action: 'allow',
    scheduleId: null,
    priority: 10,
    reasonCode: 'screen-summary-school-allow',
    createdBy: { actorId: 'parent-1', role: 'parent' },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  },
  targetEvidenceRefs: ['screen-summary-ref-1'],
  custody: 'parent-device-cache',
  updatedAt: ObservedAt,
  expiresAt: null,
} as const;
const BuildRequest = {
  schemaVersion: 'v0.6',
  requestId: 'screen-summary-context-request',
  requestedAt: ObservedAt,
  childProfile: ChildProfile,
  device: Device,
  requestedEvaluationKind: 'screen-summary',
  requiredEvidenceKinds: ['screen-summary'],
  parentRuleContextReferences: [ParentRuleContextReference],
  modelTaskRequirements: ['classification', 'safety-decision'],
  allowedCustody: ['child-device-query-store'],
  promptVersion: 'screen-summary-context-v1',
} as const;

function screenSummaryEvidence(custody = 'child-device-query-store', retentionState = 'deleted-source') {
  return {
    evidenceRefId: 'screen-summary-ref-1',
    evidence: {
      evidenceReferenceId: 'stored-screen-summary-ref-1',
      kind: 'query-store-summary',
      observedAt: ObservedAt,
    },
    evidenceKind: 'screen-summary',
    sourceSchemaVersion: 'v0.6',
    observedAt: ObservedAt,
    ingestedAt: ObservedAt,
    freshUntil: null,
    sourceId: 'screen-winrt-ocr-worker-result-live-wikipedia-browser-ocr',
    adapterId: 'windows-winrt-ocr-local-runtime',
    device: Device,
    childProfile: ChildProfile,
    custody,
    retentionState,
    confidence: 0.88,
    confidenceKind: 'classifier',
    capabilityStatus: 'available',
    degradedReasons: ['screen-image-deleted'],
    unknownReasons: [],
    sourceEvidenceReferences: [SourceEvidence],
  };
}

function contextInput(evidenceReferences = [screenSummaryEvidence()]) {
  return {
    contextId: 'screen-summary-context',
    request: BuildRequest,
    evidenceReferences,
    runtimeReferences: [RuntimeStatus],
    memoryReferences: [],
    graphReferences: [],
  };
}

describe('local AI screen summary context builder path', () => {
  it('builds a ready local AI context from deleted screen summary evidence', () => {
    const result = buildLocalAiEvidenceContext(contextInput());

    expect(result.state).toBe('ready');
    expect(result.context?.screenSummaryRefs).toEqual(['screen-summary-ref-1']);
    expect(result.context?.evidenceReferences.map((reference) => reference.evidenceRefId)).toEqual([
      'screen-summary-ref-1',
    ]);
    expect(result.context?.custodyLabels).toEqual(['child-device-query-store']);
    expect(result.context?.degradedReasons).toEqual(['screen-image-deleted']);
    expect(result.context?.localModelRuntimeRefs).toEqual(['screen-summary-context-runtime']);
    expect(result.context?.parentRuleReferences).toEqual(['screen-summary-school-rule']);
    expect(result.auditEvidenceReferences.map((reference) => reference.evidenceReferenceId)).toEqual([
      'stored-screen-summary-ref-1',
    ]);
  });

  it('rejects screen summary evidence that uses hosted non-activity custody', () => {
    const result = buildLocalAiEvidenceContext(contextInput([screenSummaryEvidence('ocentra-hosted-non-activity')]));

    expect(result.state).toBe('rejected');
    expect(result.context).toBeNull();
    expect(result.rejectedFields).toEqual(['evidenceReferences']);
    expect(result.degradedSourceRefs).toEqual(['screen-summary-ref-1']);
    expect(result.custodyBoundarySummary).toBe(
      'ocentra-hosted non-activity metadata cannot source child-activity evidence'
    );
  });

  it('returns insufficient when requested screen summary evidence is missing', () => {
    const result = buildLocalAiEvidenceContext(contextInput([]));

    expect(result.state).toBe('insufficient');
    expect(result.context).toBeNull();
    expect(result.missingEvidenceKinds).toEqual(['screen-summary']);
    expect(result.validationGateSummary).toBe('insufficient stored evidence for local model input');
  });
});
