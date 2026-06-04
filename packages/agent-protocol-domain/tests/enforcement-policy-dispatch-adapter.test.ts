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
      expect(result.readModel.entries.some((entry) => entry.matrixRow.proofLevel === 'implemented')).toBe(true);
      expect(
        result.readModel.entries.some(
          (entry) =>
            entry.intent.intentId === 'dispatch-app-game-category-risk-dry-run' &&
            entry.matrixRow.outcomeState === 'dry-run-only' &&
            entry.intent.dryRun &&
            entry.dispatchedAt === null
        )
      ).toBe(true);
      expect(result.readModel.entries.some((entry) => entry.matrixRow.outcomeState === 'manual-required')).toBe(true);
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
