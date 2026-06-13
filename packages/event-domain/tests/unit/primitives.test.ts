import { describe, expect, it } from 'vitest';

import {
  AgentEventAcknowledgementState,
  AgentEventAcknowledgementSchema,
  AgentEventDeliveryMode,
  AgentEventEnvelopeSchema,
  AgentMessageTargetSchema,
  AgentPeerRoleLiteral,
  AgentProtocolSchemaVersion,
  AgentRouteLiteral,
} from '../../src/primitives';

describe('event-domain primitives', () => {
  it('parses shared agent message targets through the canonical event domain package', () => {
    const target = AgentMessageTargetSchema.parse({
      deviceId: 'device-alpha',
      platform: 'windows',
      route: AgentRouteLiteral.Localhost,
    });

    expect(target.deviceId).toBe('device-alpha');
    expect(AgentProtocolSchemaVersion).toBe(1);
  });

  it('parses fire-and-forget and request-response event envelopes without local UI event shapes', () => {
    const envelope = AgentEventEnvelopeSchema.parse({
      eventId: 'event-1',
      eventName: 'agent.example.event',
      correlationId: 'correlation-1',
      occurredAt: '2026-06-12T10:00:00.000Z',
      source: { peerId: 'portal-runtime', role: AgentPeerRoleLiteral.Portal },
      target: { deviceId: 'child-device-1', platform: 'android', route: AgentRouteLiteral.LocalNetwork },
      deliveryMode: AgentEventDeliveryMode.RequestResponse,
    });

    expect(envelope.deliveryMode).toBe(AgentEventDeliveryMode.RequestResponse);
    expect(envelope.eventName).toBe('agent.example.event');
  });

  it('keeps event acknowledgements tied to the same correlation id', () => {
    const acknowledgement = AgentEventAcknowledgementSchema.parse({
      eventId: 'event-1',
      correlationId: 'correlation-1',
      state: AgentEventAcknowledgementState.Accepted,
      acknowledgedAt: '2026-06-12T10:00:01.000Z',
      responder: { peerId: 'child-runtime', role: AgentPeerRoleLiteral.AgentService },
    });

    expect(acknowledgement.state).toBe(AgentEventAcknowledgementState.Accepted);
    expect(acknowledgement.responder.role).toBe(AgentPeerRoleLiteral.AgentService);
  });
});
