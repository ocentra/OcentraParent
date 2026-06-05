import { describe, expect, it } from 'vitest';
import {
  ScreenAiStricterParentRuleInputSchema,
  ScreenAiStricterParentRuleProofSchema,
  buildScreenAiStricterParentRuleProof,
} from '../src/screen-ai-stricter-parent-rule-proof';

const GeneratedAt = '2026-06-05T19:29:00.000Z';
const SourceDecision = {
  schemaVersion: 'v0.6',
  decisionId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-policy-dry-run',
  action: 'allow',
  reasonCodes: ['screen-service-winrt-ocr-school-allow'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-activity-row',
      kind: 'activity-event',
      observedAt: '2026-06-05T15:59:25.662Z',
    },
    {
      evidenceReferenceId: 'screen-service-queue-job-1780675160-1-encrypted-queue',
      kind: 'journal-event',
      observedAt: '2026-06-05T15:59:25.662Z',
    },
  ],
  ruleIds: ['screen-service-winrt-ocr-school-rule'],
  localAiResultId: 'screen-service-adapter-analysis-result-screen-service-queue-job-1780675160-1-local-ocr-result',
  dryRun: true,
  enforcementHandoffState: 'disabled',
  expiresAt: null,
} as const;

describe('screen AI stricter parent rule proof contracts', () => {
  it('keeps a stricter parent block rule over a local AI allow result', () => {
    const proof = buildScreenAiStricterParentRuleProof(proofInput(parentRule('block')));

    expect(proof.sourceLocalAiAction).toBe('allow');
    expect(proof.stricterParentRuleAction).toBe('block');
    expect(proof.finalAction).toBe('block');
    expect(proof.finalDecision.action).toBe('block');
    expect(proof.finalDecision.localAiResultId).toBe(SourceDecision.localAiResultId);
    expect(proof.finalDecision.evidenceReferences).toEqual(SourceDecision.evidenceReferences);
    expect(proof.finalDecision.reasonCodes).toEqual([
      'screen-ai-parent-rule-block-over-ai-allow',
      'screen-service-winrt-ocr-school-allow',
    ]);
    expect(proof.finalDecision.ruleIds).toEqual([
      'screen-ai-parent-rule-block',
      'screen-service-winrt-ocr-school-rule',
    ]);
    expect(proof.claimBoundaries).toEqual(noClaims());
  });

  it('keeps a stricter parent time-limit rule over a local AI allow result', () => {
    const proof = buildScreenAiStricterParentRuleProof(proofInput(parentRule('time-limit')));

    expect(proof.finalAction).toBe('time-limit');
    expect(proof.finalDecision.action).toBe('time-limit');
    expect(proof.finalDecision.ruleIds[0]).toBe('screen-ai-parent-rule-time-limit');
    expect(proof.finalDecision.enforcementHandoffState).toBe('disabled');
  });

  it('rejects equal or weaker parent rules, missing local AI refs, non-dry-run source decisions, and handed-off claims', () => {
    const invalidInputs = [
      { stricterParentRule: parentRule('allow') },
      { stricterParentRule: { ...parentRule('block'), enabled: false } },
      { sourceDecision: { ...SourceDecision, localAiResultId: null } },
      { sourceDecision: { ...SourceDecision, dryRun: false } },
      { sourceDecision: { ...SourceDecision, enforcementHandoffState: 'handed-off' } },
      { expectedFinalAction: 'allow' },
    ];

    for (const invalidInput of invalidInputs) {
      expect(
        ScreenAiStricterParentRuleInputSchema.safeParse({ ...proofInput(parentRule('block')), ...invalidInput }).success
      ).toBe(false);
    }
  });

  it('rejects proof snapshots that claim local AI authority, retained images, remote AI, or enforcement', () => {
    const proof = buildScreenAiStricterParentRuleProof(proofInput(parentRule('block')));
    const invalidClaims = [
      { localAiAuthorityClaimed: true },
      { remoteAiUsed: true },
      { apiAiUsed: true },
      { rawImageRetained: true },
      { enforcementClaimed: true },
    ];

    for (const claim of invalidClaims) {
      expect(
        ScreenAiStricterParentRuleProofSchema.safeParse({
          ...proof,
          claimBoundaries: { ...noClaims(), ...claim },
        }).success
      ).toBe(false);
    }
  });
});

function proofInput(stricterParentRule: ReturnType<typeof parentRule>) {
  return {
    schemaVersion: 'v0.6',
    proofId: 'screen-ai-stricter-parent-rule-proof',
    generatedAt: GeneratedAt,
    sourceProof: 'output/screen-ai-pipeline-proof/service-winrt-ocr-policy/proof-summary.json',
    sourceDecision: SourceDecision,
    stricterParentRule,
    expectedFinalAction: stricterParentRule.action,
    claimBoundaries: noClaims(),
  };
}

function parentRule(action: 'allow' | 'block' | 'time-limit') {
  return {
    ruleId: `screen-ai-parent-rule-${action}`,
    target: {
      targetId: 'screen-ai-school-category-target',
      targetType: 'category',
      targetValue: 'school',
    },
    action,
    scheduleId: null,
    priority: 100,
    reasonCode: `screen-ai-parent-rule-${action}-over-ai-allow`,
    createdBy: {
      actorId: 'parent-policy-author',
      role: 'parent',
    },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  } as const;
}

function noClaims() {
  return {
    localAiAuthorityClaimed: false,
    remoteAiUsed: false,
    apiAiUsed: false,
    rawImageRetained: false,
    enforcementClaimed: false,
  };
}
