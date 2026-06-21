import { describe, expect, it } from 'vitest';

import {
  ChildDomainAiAnalysisCompletedEventSchema,
  ChildDomainAiAnalysisRequestedEventSchema,
  ChildDomainPolicyEvaluationRequirement,
  ChildDomainPrivatePayloadState,
  ChildDomainRuntimeEventType,
  ChildRuntimeDomainLiteral,
} from '@ocentra-parent/schema-domain/child-domain-runtime-events';

describe('child domain runtime AI boundary contracts', () => {
  it('parses the canonical AI completed boundary payload without domain-owned local shapes', () => {
    const event = ChildDomainAiAnalysisCompletedEventSchema.parse({
      eventType: ChildDomainRuntimeEventType.AiAnalysisCompleted,
      domain: ChildRuntimeDomainLiteral.Screen,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      sourceAiRequestId: 'screen:ai-request:1',
      evidenceRefs: ['screen:evidence:1'],
      sourceObservedAt: '2026-06-12T10:00:00.000Z',
      resultFactRef: 'screen:ai-fact:1',
      privatePayloadState: ChildDomainPrivatePayloadState.Excluded,
      policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirement.Required,
    });

    expect(event.eventType).toBe(ChildDomainRuntimeEventType.AiAnalysisCompleted);
    expect(event.privatePayloadState).toBe(ChildDomainPrivatePayloadState.Excluded);
  });

  it('rejects malformed AI completed boundary payloads', () => {
    const emptyEvidenceRefs = ChildDomainAiAnalysisCompletedEventSchema.safeParse({
      eventType: ChildDomainRuntimeEventType.AiAnalysisCompleted,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      sourceAiRequestId: 'browser:ai-request:1',
      evidenceRefs: [],
      sourceObservedAt: '2026-06-12T10:00:00.000Z',
      resultFactRef: 'browser:ai-fact:1',
      privatePayloadState: ChildDomainPrivatePayloadState.Excluded,
      policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirement.Required,
    });
    const wrongEventType = ChildDomainAiAnalysisCompletedEventSchema.safeParse({
      eventType: ChildDomainRuntimeEventType.BrowserAiAnalysisRequested,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      sourceAiRequestId: 'browser:ai-request:1',
      evidenceRefs: ['browser:evidence:1'],
      sourceObservedAt: '2026-06-12T10:00:00.000Z',
      resultFactRef: 'browser:ai-fact:1',
      privatePayloadState: ChildDomainPrivatePayloadState.Excluded,
      policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirement.Required,
    });

    expect(emptyEvidenceRefs.success).toBe(false);
    expect(wrongEventType.success).toBe(false);
  });

  it('parses the canonical AI request boundary payload without AI implementation fields', () => {
    const event = ChildDomainAiAnalysisRequestedEventSchema.parse({
      eventType: ChildDomainRuntimeEventType.AppAiAnalysisRequested,
      domain: ChildRuntimeDomainLiteral.App,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      aiRequestId: 'app:ai-request:1',
      evidenceRefs: ['app:evidence:1'],
      sourceObservedAt: '2026-06-12T10:00:00.000Z',
      allowedAnalysisPurpose: 'classification',
      privatePayloadState: ChildDomainPrivatePayloadState.Excluded,
      policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirement.Required,
    });

    expect(event.eventType).toBe(ChildDomainRuntimeEventType.AppAiAnalysisRequested);
    expect(event.privatePayloadState).toBe(ChildDomainPrivatePayloadState.Excluded);
  });
});
