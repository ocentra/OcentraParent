import { EnforcementPolicyDispatchReadModel } from '@ocentra-parent/parent-domain/enforcement-policy-dispatch';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults } from '../src/contracts';
import { parseEnforcementPolicyDispatchEvent } from '../src/enforcement-policy-dispatch-adapter';

describe('enforcement policy dispatch adapter', () => {
  it('parses the service-backed policy dispatch read model event', () => {
    const result = parseEnforcementPolicyDispatchEvent({
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      eventId: 'event-dispatch',
      correlationId: 'cmd-dispatch',
      sentAt: '2026-06-02T05:45:00.000Z',
      source: { peerId: 'local-dev-agent', role: 'agent-service' },
      target: AgentProtocolDefaults.Peer.PortalDev,
      event: AgentEvent.EnforcementPolicyDispatchReported,
      severity: 'info',
      payload: {
        [AgentProtocolDefaults.Field.EnforcementPolicyDispatchReadModel]: JSON.stringify(
          EnforcementPolicyDispatchReadModel
        ),
      },
      snapshot: null,
    });

    expect(result.status).toBe('accepted');
    if (result.status === 'accepted') {
      expect(result.readModel.entries[0]?.matrixRow.proofLevel).toBe('implemented');
      expect(result.readModel.entries[3]?.matrixRow.outcomeState).toBe('manual-required');
    }
  });

  it('rejects malformed read model json', () => {
    const result = parseEnforcementPolicyDispatchEvent({
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      eventId: 'event-dispatch',
      correlationId: 'cmd-dispatch',
      sentAt: '2026-06-02T05:45:00.000Z',
      source: { peerId: 'local-dev-agent', role: 'agent-service' },
      target: AgentProtocolDefaults.Peer.PortalDev,
      event: AgentEvent.EnforcementPolicyDispatchReported,
      severity: 'warn',
      payload: {
        [AgentProtocolDefaults.Field.EnforcementPolicyDispatchReadModel]: '{',
      },
      snapshot: null,
    });

    expect(result).toEqual({ status: 'rejected', reason: 'invalid-read-model-json' });
  });

  it('rejects unexpected events', () => {
    const result = parseEnforcementPolicyDispatchEvent({
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      eventId: 'event-health',
      correlationId: 'cmd-health',
      sentAt: '2026-06-02T05:45:00.000Z',
      source: { peerId: 'local-dev-agent', role: 'agent-service' },
      target: AgentProtocolDefaults.Peer.PortalDev,
      event: AgentEvent.HealthReported,
      severity: 'info',
      payload: {},
      snapshot: null,
    });

    expect(result).toEqual({ status: 'rejected', reason: 'unexpected-event' });
  });
});
