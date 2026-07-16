import { describe, expect, it } from 'vitest';

import {
  ChildDomainEvidenceRecordedEventSchema,
  ChildDomainNotificationRequestedEventSchema,
  ChildDomainPolicyEvaluationRequestedEventSchema,
  ChildDomainPolicyViolationDetectedEventSchema,
  ChildDomainRuntimeEventType,
  ChildRuntimeDomainLiteral,
} from '@ocentra-parent/schema-domain/child-domain-runtime-events';

describe('child domain runtime timestamp boundary contracts', () => {
  it('parses timestamp-bearing evidence-policy-notification boundary payloads', () => {
    const policyViolationId = `${ChildDomainRuntimeEventType.PolicyViolationDetected}:browser:policy-request:1`;

    const evidenceRecorded = ChildDomainEvidenceRecordedEventSchema.parse({
      eventType: ChildDomainRuntimeEventType.BrowserEvidenceRecorded,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      evidenceRef: 'browser:evidence:1',
      sourceObservationId: 'browser:observation:1',
      sourceObservedAt: '2026-06-12T10:00:00.000Z',
      signal: 'requires-policy',
      aiAnalysisRequirement: 'not-required',
      policyEvaluationRequirement: 'required',
    });
    const policyRequested = ChildDomainPolicyEvaluationRequestedEventSchema.parse({
      eventType: ChildDomainRuntimeEventType.BrowserPolicyEvaluationRequested,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      policyRequestId: 'browser:policy-request:1',
      evidenceRefs: [evidenceRecorded.evidenceRef],
      sourceObservedAt: evidenceRecorded.sourceObservedAt,
      sourceFactRef: 'browser:fact:1',
    });
    const policyViolation = ChildDomainPolicyViolationDetectedEventSchema.parse({
      eventType: ChildDomainRuntimeEventType.PolicyViolationDetected,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      violationId: policyViolationId,
      policyRuleRef: 'default',
      severity: 'review',
      detectedAt: policyRequested.sourceObservedAt,
      evidenceRefs: policyRequested.evidenceRefs,
    });
    const notificationRequested = ChildDomainNotificationRequestedEventSchema.parse({
      eventType: ChildDomainRuntimeEventType.NotificationRequested,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      notificationId: `${ChildDomainRuntimeEventType.NotificationRequested}:${policyViolationId}`,
      sourcePolicyViolationId: policyViolation.violationId,
      channel: 'parent-portal',
      requestedAt: policyViolation.detectedAt,
      evidenceRefs: policyViolation.evidenceRefs,
    });

    expect(policyRequested.sourceObservedAt).toBe(evidenceRecorded.sourceObservedAt);
    expect(policyViolation.detectedAt).toBe(policyRequested.sourceObservedAt);
    expect(notificationRequested.requestedAt).toBe(policyViolation.detectedAt);
  });

  it('rejects notification payloads without a typed request timestamp', () => {
    const result = ChildDomainNotificationRequestedEventSchema.safeParse({
      eventType: ChildDomainRuntimeEventType.NotificationRequested,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      notificationId: `${ChildDomainRuntimeEventType.NotificationRequested}:1`,
      sourcePolicyViolationId: `${ChildDomainRuntimeEventType.PolicyViolationDetected}:1`,
      channel: 'parent-portal',
      evidenceRefs: ['browser:evidence:1'],
    });

    expect(result.success).toBe(false);
  });
});
