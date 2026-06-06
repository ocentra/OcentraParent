import { describe, expect, it } from 'vitest';
import {
  ScreenAiEnforcementHandoffGuardInputSchema,
  ScreenAiEnforcementHandoffGuardPayloadSchema,
  buildScreenAiEnforcementHandoffGuardPayload,
} from '../src/screen-ai-enforcement-handoff-guard-proof';

const GeneratedAt = '2026-06-06T22:02:00.000Z';
const SummaryReference = {
  evidenceReferenceId: 'screen-summary-parent-readable-school-research',
  kind: 'query-store-summary',
  observedAt: GeneratedAt,
} as const;
const LocalAiResultReference = {
  evidenceReferenceId: 'screen-local-ai-result-school-research',
  kind: 'local-ai-result',
  observedAt: GeneratedAt,
} as const;
const AuditReference = {
  evidenceReferenceId: 'screen-policy-audit-school-research',
  kind: 'journal-event',
  observedAt: GeneratedAt,
} as const;

describe('screen AI enforcement handoff guard proof', () => {
  it('builds a guarded dry-run handoff payload from summary AI policy rule and audit refs', () => {
    const payload = buildScreenAiEnforcementHandoffGuardPayload(validInput());

    expect(payload.payloadId).toBe('screen-ai-enforcement-handoff-school-research');
    expect(payload.requestedAction).toBe('time-limit');
    expect(payload.handoffMode).toBe('dry-run');
    expect(payload.confidenceState).toBe('medium');
    expect(payload.summaryReference).toEqual(SummaryReference);
    expect(payload.localAiResultReference).toEqual(LocalAiResultReference);
    expect(payload.auditReference).toEqual(AuditReference);
    expect(payload.auditEvent.eventType).toBe('screen.enforcement.handoff.guard.accepted');
    expect(payload.rawPixelsIncluded).toBe(false);
    expect(payload.rawModelTextIncluded).toBe(false);
    expect(payload.rawScreenshotRetained).toBe(false);
    expect(payload.localAiAuthorityClaimed).toBe(false);
  });

  it('rejects policy decisions that are already handed off non dry-run or missing parent rule and local AI refs', () => {
    const invalidInputs = [
      { sourcePolicyDecision: { ...policyDecision(), dryRun: false } },
      { sourcePolicyDecision: { ...policyDecision(), enforcementHandoffState: 'handed-off' } },
      { sourcePolicyDecision: { ...policyDecision(), localAiResultId: null } },
      { sourcePolicyDecision: { ...policyDecision(), ruleIds: ['other-parent-rule'] } },
      { parentPolicyRule: { ...parentRule(), enabled: false } },
    ];

    for (const invalidInput of invalidInputs) {
      expect(ScreenAiEnforcementHandoffGuardInputSchema.safeParse({ ...validInput(), ...invalidInput }).success).toBe(
        false
      );
    }
  });

  it('rejects handoff inputs without summary local AI and audit evidence refs', () => {
    const invalidMaterials = [
      { summaryReference: { ...SummaryReference, evidenceReferenceId: 'missing-summary-ref' } },
      { localAiResultReference: { ...LocalAiResultReference, kind: 'activity-event' } },
      { auditReference: { ...AuditReference, kind: 'policy-decision' } },
      { rawPixelsIncluded: true },
      { rawModelTextIncluded: true },
      { rawScreenshotRetained: true },
      { localAiAuthorityClaimed: true },
    ];

    for (const invalidMaterial of invalidMaterials) {
      expect(
        ScreenAiEnforcementHandoffGuardInputSchema.safeParse({
          ...validInput(),
          inputMaterial: { ...inputMaterial(), ...invalidMaterial },
        }).success
      ).toBe(false);
    }
  });

  it('rejects payload snapshots that mutate raw custody or audit event identity after build', () => {
    const payload = buildScreenAiEnforcementHandoffGuardPayload(validInput());
    const invalidPayloads = [
      { rawPixelsIncluded: true },
      { rawModelTextIncluded: true },
      { rawScreenshotRetained: true },
      { localAiAuthorityClaimed: true },
      {
        auditEvent: {
          ...payload.auditEvent,
          evidenceReference: { ...AuditReference, evidenceReferenceId: 'different-audit-ref' },
        },
      },
    ];

    for (const invalidPayload of invalidPayloads) {
      expect(ScreenAiEnforcementHandoffGuardPayloadSchema.safeParse({ ...payload, ...invalidPayload }).success).toBe(
        false
      );
    }
  });
});

function validInput() {
  return {
    schemaVersion: 'v0.6',
    payloadId: 'screen-ai-enforcement-handoff-school-research',
    generatedAt: GeneratedAt,
    sourcePolicyDecision: policyDecision(),
    parentPolicyRule: parentRule(),
    requestedAction: 'time-limit',
    confidenceState: 'medium',
    handoffMode: 'dry-run',
    inputMaterial: inputMaterial(),
    auditEvent: {
      auditEventId: 'screen-ai-enforcement-handoff-school-research-audit',
      eventType: 'screen.enforcement.handoff.guard.accepted',
      emittedAt: GeneratedAt,
      evidenceReference: AuditReference,
    },
    claimBoundary: 'Screen AI handoff guard carries refs only; adapter execution remains a separate proof gate.',
  } as const;
}

function inputMaterial() {
  return {
    summaryReference: SummaryReference,
    localAiResultReference: LocalAiResultReference,
    auditReference: AuditReference,
    rawPixelsIncluded: false,
    rawModelTextIncluded: false,
    rawScreenshotRetained: false,
    localAiAuthorityClaimed: false,
  } as const;
}

function policyDecision() {
  return {
    schemaVersion: 'v0.6',
    decisionId: 'screen-ai-policy-school-research-time-limit',
    action: 'time-limit',
    reasonCodes: ['screen-ai-policy-school-research'],
    evidenceReferences: [SummaryReference, LocalAiResultReference, AuditReference],
    ruleIds: ['screen-ai-parent-rule-school-research-time-limit'],
    localAiResultId: 'screen-local-ai-result-school-research',
    dryRun: true,
    enforcementHandoffState: 'disabled',
    expiresAt: null,
  } as const;
}

function parentRule() {
  return {
    ruleId: 'screen-ai-parent-rule-school-research-time-limit',
    target: {
      targetId: 'screen-ai-school-research-category',
      targetType: 'category',
      targetValue: 'school-research',
    },
    action: 'time-limit',
    scheduleId: null,
    priority: 100,
    reasonCode: 'screen-ai-policy-school-research',
    createdBy: {
      actorId: 'parent-policy-author',
      role: 'parent',
    },
    enabled: true,
    effectiveFrom: null,
    effectiveUntil: null,
  } as const;
}
