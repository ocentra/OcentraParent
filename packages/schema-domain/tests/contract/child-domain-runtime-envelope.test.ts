import { describe, expect, it } from 'vitest';
import {
  AgentEventDeliveryMode,
  AgentPeerRoleLiteral,
  AgentRouteLiteral,
} from '@ocentra-parent/schema-domain/event-primitives';

import {
  ChildDomainRuntimeEventEnvelopeSchema,
  ChildDomainRuntimeEventType,
  ChildRuntimeDomainLiteral,
} from '@ocentra-parent/schema-domain/child-domain-runtime-events';

describe('child domain runtime envelope contracts', () => {
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

  it('rejects child runtime envelopes with mismatched event name or wrong domain ownership', () => {
    const mismatchedEventName = ChildDomainRuntimeEventEnvelopeSchema.safeParse({
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
    const wrongDomain = ChildDomainRuntimeEventEnvelopeSchema.safeParse({
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

    expect(mismatchedEventName.success).toBe(false);
    expect(wrongDomain.success).toBe(false);
  });
});
