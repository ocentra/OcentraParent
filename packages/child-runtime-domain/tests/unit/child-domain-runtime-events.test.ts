import { describe, expect, it } from 'vitest';
import {
  AgentEventDeliveryMode,
  AgentPeerRoleLiteral,
  AgentRouteLiteral,
} from '@ocentra-parent/event-domain/primitives';

import {
  ChildDomainAiAnalysisCompletedEventSchema,
  ChildDomainAiAnalysisRequestedEventSchema,
  ChildDomainRuntimeEventEnvelopeSchema,
  ChildDomainRuntimeEventType,
  ChildDomainRuntimeEventTypeLiteral,
  ChildDomainRuntimeEventTypeSchema,
  ChildDomainPolicyEvaluationRequirement,
  ChildDomainPrivatePayloadState,
  ChildRuntimeDomainLiteral,
  ChildRuntimeDomainSchema,
} from '../../src/child-domain-runtime-events';

describe('child domain runtime event contracts', () => {
  it('parses known child runtime domains', () => {
    expect(ChildRuntimeDomainSchema.parse(ChildRuntimeDomainLiteral.App)).toBe(
      ChildRuntimeDomainLiteral.App
    );
    expect(ChildRuntimeDomainSchema.parse(ChildRuntimeDomainLiteral.Browser)).toBe(
      ChildRuntimeDomainLiteral.Browser
    );
  });

  it('parses known child domain runtime event types', () => {
    expect(
      ChildDomainRuntimeEventTypeSchema.parse(
        ChildDomainRuntimeEventTypeLiteral.AppAiAnalysisRequested
      )
    ).toBe(ChildDomainRuntimeEventTypeLiteral.AppAiAnalysisRequested);
    expect(
      ChildDomainRuntimeEventTypeSchema.parse(
        ChildDomainRuntimeEventTypeLiteral.ScreenAiAnalysisRequested
      )
    ).toBe(ChildDomainRuntimeEventTypeLiteral.ScreenAiAnalysisRequested);
  });

  it('rejects unowned child domain runtime event types', () => {
    expect(() => ChildDomainRuntimeEventTypeSchema.parse('child-domain.unowned.event')).toThrow();
  });

  it('composes child runtime event names with the shared event-domain envelope', () => {
    const event = ChildDomainRuntimeEventEnvelopeSchema.parse({
      envelope: {
        eventId: 'event-1',
        eventName: ChildDomainRuntimeEventType.BrowserAiAnalysisRequested,
        correlationId: 'correlation-1',
        occurredAt: '2026-06-12T10:00:00.000Z',
        source: { peerId: 'child-runtime', role: AgentPeerRoleLiteral.AgentService },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.FireAndForget,
      },
      domain: ChildRuntimeDomainLiteral.Browser,
      eventType: ChildDomainRuntimeEventType.BrowserAiAnalysisRequested,
    });

    expect(event.envelope.eventName).toBe(ChildDomainRuntimeEventType.BrowserAiAnalysisRequested);
  });

  it('rejects child runtime envelopes with mismatched event name and typed event', () => {
    const result = ChildDomainRuntimeEventEnvelopeSchema.safeParse({
      envelope: {
        eventId: 'event-1',
        eventName: ChildDomainRuntimeEventType.BrowserAiAnalysisRequested,
        correlationId: 'correlation-1',
        occurredAt: '2026-06-12T10:00:00.000Z',
        source: { peerId: 'child-runtime', role: AgentPeerRoleLiteral.AgentService },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.FireAndForget,
      },
      domain: ChildRuntimeDomainLiteral.Browser,
      eventType: ChildDomainRuntimeEventType.BrowserPolicyEvaluationRequested,
    });

    expect(result.success).toBe(false);
  });

  it('rejects domain-specific child runtime events under the wrong domain', () => {
    const result = ChildDomainRuntimeEventEnvelopeSchema.safeParse({
      envelope: {
        eventId: 'event-1',
        eventName: ChildDomainRuntimeEventType.ScreenAiAnalysisRequested,
        correlationId: 'correlation-1',
        occurredAt: '2026-06-12T10:00:00.000Z',
        source: { peerId: 'child-runtime', role: AgentPeerRoleLiteral.AgentService },
        target: {
          deviceId: 'child-device-1',
          platform: 'android',
          route: AgentRouteLiteral.LocalNetwork,
        },
        deliveryMode: AgentEventDeliveryMode.FireAndForget,
      },
      domain: ChildRuntimeDomainLiteral.Browser,
      eventType: ChildDomainRuntimeEventType.ScreenAiAnalysisRequested,
    });

    expect(result.success).toBe(false);
  });

  it('parses the canonical AI completed boundary payload without domain-owned local shapes', () => {
    const event = ChildDomainAiAnalysisCompletedEventSchema.parse({
      eventType: ChildDomainRuntimeEventType.AiAnalysisCompleted,
      domain: ChildRuntimeDomainLiteral.Screen,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      sourceAiRequestId: 'screen:ai-request:1',
      evidenceRefs: ['screen:evidence:1'],
      resultFactRef: 'screen:ai-fact:1',
      privatePayloadState: ChildDomainPrivatePayloadState.Excluded,
      policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirement.Required,
    });

    expect(event.eventType).toBe(ChildDomainRuntimeEventType.AiAnalysisCompleted);
    expect(event.privatePayloadState).toBe(ChildDomainPrivatePayloadState.Excluded);
  });

  it('rejects empty evidence references in AI completed boundary payloads', () => {
    const result = ChildDomainAiAnalysisCompletedEventSchema.safeParse({
      eventType: ChildDomainRuntimeEventType.AiAnalysisCompleted,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      sourceAiRequestId: 'browser:ai-request:1',
      evidenceRefs: [],
      resultFactRef: 'browser:ai-fact:1',
      privatePayloadState: ChildDomainPrivatePayloadState.Excluded,
      policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirement.Required,
    });

    expect(result.success).toBe(false);
  });

  it('rejects AI completed payloads that use a domain-specific request event name', () => {
    const result = ChildDomainAiAnalysisCompletedEventSchema.safeParse({
      eventType: ChildDomainRuntimeEventType.BrowserAiAnalysisRequested,
      domain: ChildRuntimeDomainLiteral.Browser,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      sourceAiRequestId: 'browser:ai-request:1',
      evidenceRefs: ['browser:evidence:1'],
      resultFactRef: 'browser:ai-fact:1',
      privatePayloadState: ChildDomainPrivatePayloadState.Excluded,
      policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirement.Required,
    });

    expect(result.success).toBe(false);
  });

  it('parses the canonical AI request boundary payload without AI implementation fields', () => {
    const event = ChildDomainAiAnalysisRequestedEventSchema.parse({
      eventType: ChildDomainRuntimeEventType.AppAiAnalysisRequested,
      domain: ChildRuntimeDomainLiteral.App,
      childDeviceId: 'child-device-1',
      childProfileId: 'child-profile-1',
      aiRequestId: 'app:ai-request:1',
      evidenceRefs: ['app:evidence:1'],
      allowedAnalysisPurpose: 'classification',
      privatePayloadState: ChildDomainPrivatePayloadState.Excluded,
      policyEvaluationRequirement: ChildDomainPolicyEvaluationRequirement.Required,
    });

    expect(event.eventType).toBe(ChildDomainRuntimeEventType.AppAiAnalysisRequested);
    expect(event.privatePayloadState).toBe(ChildDomainPrivatePayloadState.Excluded);
  });
});
