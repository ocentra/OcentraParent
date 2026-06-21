import { V08EnforcementProductControlSpineReadModel } from '@ocentra-parent/schema-domain/v0-8-enforcement-product-control-spine';
import { expect, it } from 'vitest';
import { parseEnforcementProductControlSpineEvent } from '../../src/enforcement-product-control-adapter';
import { AgentEvent, AgentEventEnvelopeSchema, AgentProtocolDefaults } from '../../src/contracts';

it('parses service-backed V0.8 product-control spine events', () => {
  const parsed = parseEnforcementProductControlSpineEvent(
    eventEnvelope(AgentEvent.EnforcementProductControlSpineReported, {
      [AgentProtocolDefaults.Field.EnforcementProductControlSpineReadModel]: JSON.stringify(
        V08EnforcementProductControlSpineReadModel
      ),
    })
  );

  expect(parsed.status).toBe('accepted');
  if (parsed.status === 'accepted') {
    expect(parsed.readModel.readModelId).toBe('v0-8-enforcement-product-control-spine');
    expect(parsed.readModel.entries).toHaveLength(15);
    expect(parsed.readModel.entries.every((entry) => !entry.broadAppBlockingClaimed)).toBe(true);
    expect(parsed.readModel.entries.every((entry) => !entry.networkDomainBlockingClaimed)).toBe(true);
  }
});

it('rejects wrong events and invalid product-control read-model payloads', () => {
  expect(
    parseEnforcementProductControlSpineEvent(
      eventEnvelope(AgentEvent.HealthReported, {
        [AgentProtocolDefaults.Field.EnforcementProductControlSpineReadModel]: JSON.stringify(
          V08EnforcementProductControlSpineReadModel
        ),
      })
    )
  ).toEqual({ status: 'rejected', reason: 'unexpected-event' });

  expect(
    parseEnforcementProductControlSpineEvent(
      eventEnvelope(AgentEvent.EnforcementProductControlSpineReported, {
        [AgentProtocolDefaults.Field.EnforcementProductControlSpineReadModel]: '{',
      })
    )
  ).toEqual({ status: 'rejected', reason: 'invalid-read-model-json' });

  expect(
    parseEnforcementProductControlSpineEvent(
      eventEnvelope(AgentEvent.EnforcementProductControlSpineReported, {
        [AgentProtocolDefaults.Field.EnforcementProductControlSpineReadModel]: JSON.stringify({
          ...V08EnforcementProductControlSpineReadModel,
          entries: [
            {
              ...V08EnforcementProductControlSpineReadModel.entries[0],
              broadAppBlockingClaimed: true,
            },
          ],
        }),
      })
    )
  ).toEqual({ status: 'rejected', reason: 'invalid-read-model' });
});

function eventEnvelope(eventName: (typeof AgentEvent)[keyof typeof AgentEvent], payload: Record<string, unknown>) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'evt-v0-8-product-control',
    correlationId: 'cmd-v0-8-product-control',
    sentAt: '2026-06-01T21:20:00.000Z',
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
