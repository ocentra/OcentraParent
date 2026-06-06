import { describe, expect, it } from 'vitest';
import {
  buildLocalAiParentRuleContextBuilderProof,
  buildLocalAiParentRuleContextBuilderProofRow,
  LocalAiParentRuleContextBuilderProofRowSchema,
} from '../src/local-ai-parent-rule-context-builder-proof';

const observedAt = '2026-06-06T09:56:00.000Z';
const childProfile = { childProfileId: 'child:parent-rule-context', displayName: 'Maya' };
const device = {
  deviceId: 'device:parent-rule-context-windows',
  childProfileId: childProfile.childProfileId,
  label: 'Maya Windows laptop',
  platform: 'windows',
};
const sourceEvidence = {
  evidenceReferenceId: 'journal:screen-summary-parent-rule',
  kind: 'journal-event',
  observedAt,
};
const baseInput = {
  contextId: 'context:parent-rule-builder',
  request: {
    schemaVersion: 'v0.6',
    requestId: 'request:parent-rule-builder',
    requestedAt: observedAt,
    childProfile,
    device,
    requestedEvaluationKind: 'screen-summary',
    requiredEvidenceKinds: ['screen-summary'],
    parentRuleContextReferences: [parentRuleContextReference(['screen-summary:school-video'])],
    modelTaskRequirements: [],
    allowedCustody: ['child-device-query-store'],
    promptVersion: 'prompt:parent-rule-context-v1',
  },
  evidenceReferences: [
    {
      evidenceRefId: 'screen-summary:school-video',
      evidence: {
        evidenceReferenceId: 'query-store:screen-summary-school-video',
        kind: 'query-store-summary',
        observedAt,
      },
      evidenceKind: 'screen-summary',
      sourceSchemaVersion: 'v0.6',
      observedAt,
      ingestedAt: '2026-06-06T09:56:02.000Z',
      freshUntil: null,
      sourceId: 'source:screen-summary-school-video',
      adapterId: 'adapter:winrt-ocr',
      device,
      childProfile,
      custody: 'child-device-query-store',
      retentionState: 'deleted-source',
      confidence: 0.91,
      confidenceKind: 'classifier',
      capabilityStatus: 'available',
      degradedReasons: ['screen-image-deleted'],
      unknownReasons: [],
      sourceEvidenceReferences: [sourceEvidence],
    },
  ],
  runtimeReferences: [],
  memoryReferences: [],
  graphReferences: [],
};

describe('local AI parent-rule context builder proof', () => {
  it('selects only parent-rule context refs grounded in selected evidence before local AI input', () => {
    const row = buildLocalAiParentRuleContextBuilderProofRow(baseInput);

    expect(row.state).toBe('ready');
    expect(row.selectedParentRuleContextRefs).toEqual(['parent-rule-context:screen-school']);
    expect(row.selectedParentRuleRefs).toEqual(['rule:screen-school']);
    expect(row.selectedTargetEvidenceRefs).toEqual(['screen-summary:school-video']);
    expect(row.selectedEvidenceRefs).toEqual(['screen-summary:school-video']);
    expect(row.ungroundedParentRuleReferenceCount).toBe(0);
    expect(row.rawEvidenceRetained).toBe(false);
    expect(row.remoteAiUsed).toBe(false);
    expect(row.policyAuthorityClaimed).toBe(false);
    expect(row.enforcementClaimed).toBe(false);
  });

  it('omits ungrounded parent-rule refs and keeps the context partial instead of inventing authority', () => {
    const row = buildLocalAiParentRuleContextBuilderProofRow({
      ...baseInput,
      request: {
        ...baseInput.request,
        parentRuleContextReferences: [parentRuleContextReference(['missing-screen-summary'])],
      },
    });

    expect(row.state).toBe('partial');
    expect(row.selectedParentRuleContextRefs).toEqual([]);
    expect(row.selectedParentRuleRefs).toEqual([]);
    expect(row.selectedTargetEvidenceRefs).toEqual([]);
    expect(row.ungroundedParentRuleReferenceCount).toBe(1);
    expect(row.degradedReasons).toContain('parent-rule-missing');
  });

  it('rejects rows that overclaim raw retention, remote AI, policy authority, or enforcement', () => {
    const row = buildLocalAiParentRuleContextBuilderProofRow(baseInput);
    const rejected = LocalAiParentRuleContextBuilderProofRowSchema.safeParse({
      ...row,
      rawEvidenceRetained: true,
      remoteAiUsed: true,
      policyAuthorityClaimed: true,
      enforcementClaimed: true,
    });

    expect(rejected.success).toBe(false);
  });

  it('builds a complete proof from ready and ungrounded scenarios', () => {
    const proof = buildLocalAiParentRuleContextBuilderProof(
      baseInput,
      {
        ...baseInput,
        request: {
          ...baseInput.request,
          parentRuleContextReferences: [parentRuleContextReference(['missing-screen-summary'])],
        },
      },
      observedAt
    );

    expect(proof.validationSummary.readySelectedParentRuleCount).toBe(1);
    expect(proof.validationSummary.readySelectedEvidenceCount).toBe(1);
    expect(proof.validationSummary.ungroundedRejectedParentRuleCount).toBe(1);
    expect(proof.validationSummary.remoteAiRows).toBe(0);
    expect(proof.validationSummary.rawEvidenceRetainedRows).toBe(0);
    expect(proof.validationSummary.policyAuthorityRows).toBe(0);
    expect(proof.validationSummary.enforcementRows).toBe(0);
  });
});

function parentRuleContextReference(targetEvidenceRefs: string[]) {
  return {
    parentRuleRefId: 'parent-rule-context:screen-school',
    policyVersion: 'policy:screen-v1',
    family: { familyId: 'family:parent-rule-context' },
    childProfile,
    device,
    rule: {
      ruleId: 'rule:screen-school',
      target: { targetId: 'target:school', targetType: 'category', targetValue: 'school' },
      action: 'allow',
      scheduleId: null,
      priority: 20,
      reasonCode: 'schoolwork-allowed',
      createdBy: { actorId: 'parent:maya', role: 'parent' },
      enabled: true,
      effectiveFrom: null,
      effectiveUntil: null,
    },
    targetEvidenceRefs,
    custody: 'parent-device-cache',
    updatedAt: observedAt,
    expiresAt: null,
  };
}
