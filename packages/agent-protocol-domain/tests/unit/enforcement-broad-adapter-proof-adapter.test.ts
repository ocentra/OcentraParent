import { V08BroadOsAdapterRuntimeProofReadModel } from '@ocentra-parent/schema-domain/v0-8-broad-os-adapter-runtime-proof';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { parseEnforcementBroadAdapterProofEvent } from '../../src/enforcement-broad-adapter-proof-adapter';

describe('enforcement broad adapter proof adapter', () => {
  it('parses service-backed broad adapter proof events without claim upgrades', () => {
    const parsed = parseEnforcementBroadAdapterProofEvent(
      eventEnvelope(AgentEvent.EnforcementBroadAdapterProofReported, {
        [AgentProtocolDefaults.Field.EnforcementBroadAdapterProofReadModel]: JSON.stringify(
          V08BroadOsAdapterRuntimeProofReadModel
        ),
      })
    );

    expect(parsed.status).toBe('accepted');
    if (parsed.status === 'accepted') {
      expect(parsed.readModel.readModelId).toBe('v0-8-broad-os-adapter-runtime-proof');
      expect(parsed.readModel.entries).toHaveLength(10);
      expect(parsed.readModel.entries.every((entry) => !entry.broadInstalledAppBlockingClaimed)).toBe(true);
      expect(parsed.readModel.entries.every((entry) => !entry.networkDomainBlockingClaimed)).toBe(true);
      expect(parsed.readModel.entries.every((entry) => !entry.managedBrowserExactUrlClaimed)).toBe(true);
    }
  });

  it('rejects unexpected events malformed json and invalid claim upgrades', () => {
    expect(
      parseEnforcementBroadAdapterProofEvent(
        eventEnvelope(AgentEvent.HealthReported, {
          [AgentProtocolDefaults.Field.EnforcementBroadAdapterProofReadModel]: JSON.stringify(
            V08BroadOsAdapterRuntimeProofReadModel
          ),
        })
      )
    ).toEqual({ status: 'rejected', reason: 'unexpected-event' });

    expect(
      parseEnforcementBroadAdapterProofEvent(
        eventEnvelope(AgentEvent.EnforcementBroadAdapterProofReported, {
          [AgentProtocolDefaults.Field.EnforcementBroadAdapterProofReadModel]: '{',
        })
      )
    ).toEqual({ status: 'rejected', reason: 'invalid-read-model-json' });

    expect(
      parseEnforcementBroadAdapterProofEvent(
        eventEnvelope(AgentEvent.EnforcementBroadAdapterProofReported, {
          [AgentProtocolDefaults.Field.EnforcementBroadAdapterProofReadModel]: JSON.stringify({
            ...V08BroadOsAdapterRuntimeProofReadModel,
            entries: [
              {
                ...V08BroadOsAdapterRuntimeProofReadModel.entries[0],
                broadInstalledAppBlockingClaimed: true,
              },
            ],
          }),
        })
      )
    ).toEqual({ status: 'rejected', reason: 'invalid-read-model' });
  });
});

function eventEnvelope(eventName: (typeof AgentEvent)[keyof typeof AgentEvent], payload: Record<string, unknown>) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'evt-v0-8-broad-adapter-proof',
    correlationId: 'cmd-v0-8-broad-adapter-proof',
    sentAt: '2026-06-02T07:10:00.000Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event: eventName,
    severity: 'info',
    payload,
    snapshot: null,
  });
}
