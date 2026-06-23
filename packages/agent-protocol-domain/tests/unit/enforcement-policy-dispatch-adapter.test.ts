import { EnforcementPolicyDispatchReadModel } from '@ocentra-parent/schema-domain/enforcement-policy-dispatch';
import { describe, expect, it } from 'vitest';
import { AgentEvent } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { parseEnforcementPolicyDispatchEvent } from '../../src/enforcement-policy-dispatch-adapter';

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
      expect(result.readModel.entries).toHaveLength(8);
      expect(entryForIntent(result.readModel, 'dispatch-owned-process-time-limit')?.matrixRow.proofLevel).toBe(
        'implemented'
      );
      expect(entryForIntent(result.readModel, 'dispatch-ask-parent-dry-run')?.matrixRow.outcomeState).toBe(
        'dry-run-only'
      );
      expect(
        entryForIntent(result.readModel, 'dispatch-stale-policy-version-rejected')?.matrixRow.rejectionReason
      ).toBe('stale-policy-version');
      expect(entryForIntent(result.readModel, 'dispatch-missing-source-rejected')?.intent.sourceState).toBe('missing');
      expect(entryForIntent(result.readModel, 'dispatch-network-domain-manual-required')?.matrixRow.outcomeState).toBe(
        'manual-required'
      );
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

function entryForIntent(readModel: typeof EnforcementPolicyDispatchReadModel, intentId: string) {
  return readModel.entries.find((entry) => entry.intent.intentId === intentId);
}
