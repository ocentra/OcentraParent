import {
  V08EnforcementIntegrityRuntimeAuditReadModel,
  V08SupportedAdapterRuntimeProofReadModel,
} from '@ocentra-parent/parent-domain/v0-8-supported-adapter-runtime-proof';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema, AgentProtocolDefaults } from '../src/contracts';
import { parseEnforcementSupportedAdapterRuntimeProofEvent } from '../src/enforcement-supported-adapter-runtime-proof-adapter';

describe('enforcement supported adapter runtime proof adapter', () => {
  it('parses service-backed supported adapter proof events without unsupported claim upgrades', () => {
    const parsed = parseEnforcementSupportedAdapterRuntimeProofEvent(
      eventEnvelope(AgentEvent.EnforcementSupportedAdapterRuntimeProofReported, {
        [AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel]: JSON.stringify(
          V08SupportedAdapterRuntimeProofReadModel
        ),
        [AgentProtocolDefaults.Field.EnforcementIntegrityRuntimeAuditReadModel]: JSON.stringify(
          V08EnforcementIntegrityRuntimeAuditReadModel
        ),
      })
    );

    expect(parsed.status).toBe('accepted');
    if (parsed.status === 'accepted') {
      expect(parsed.readModel.readModelId).toBe('v0-8-supported-adapter-runtime-proof');
      expect(parsed.readModel.entries).toHaveLength(10);
      expect(countBy(parsed.readModel.entries.map((entry) => entry.runtimeState))).toEqual({
        'implemented-boundary': 2,
        'manual-required': 4,
        'not-claimed': 1,
        degraded: 1,
        unavailable: 1,
        unsupported: 1,
      });
      expect(parsed.readModel.entries.every((entry) => !entry.broadInstalledAppBlockingClaimed)).toBe(true);
      expect(parsed.readModel.entries.every((entry) => !entry.networkDomainBlockingClaimed)).toBe(true);
      expect(parsed.readModel.entries.every((entry) => !entry.exactActiveTabEnforcementClaimed)).toBe(true);
      expect(parsed.readModel.entries.every((entry) => !entry.notificationDeliveryClaimed)).toBe(true);
      expect(parsed.readModel.entries.every((entry) => !entry.tamperHardeningClaimed)).toBe(true);
      expect(parsed.readModel.entries.every((entry) => !entry.mobileControlClaimed)).toBe(true);
      expect(parsed.readModel.entries.every((entry) => !entry.unsupportedPlatformBehaviorClaimed)).toBe(true);
      expect(parsed.integrityAuditReadModel.readModelId).toBe('v0-8-enforcement-integrity-runtime-audit');
      expect(parsed.integrityAuditReadModel.entries).toHaveLength(14);
      expect(countBy(parsed.integrityAuditReadModel.entries.map((entry) => entry.result))).toEqual({
        succeeded: 1,
        expired: 1,
        'rolled-back': 1,
        superseded: 1,
        'no-op': 1,
        failed: 2,
        'observe-only': 1,
        'manual-required': 2,
        unavailable: 3,
        unsupported: 1,
      });
      expect(parsed.integrityAuditReadModel.entries.every((entry) => !entry.tamperHardeningClaimed)).toBe(true);
      expect(parsed.integrityAuditReadModel.entries.every((entry) => !entry.stealthPersistenceClaimed)).toBe(true);
      expect(parsed.integrityAuditReadModel.entries.every((entry) => !entry.privilegeEscalationClaimed)).toBe(true);
    }
  });

  it('rejects unexpected events malformed json missing read models and invalid supported adapter claim upgrades', () => {
    expect(
      parseEnforcementSupportedAdapterRuntimeProofEvent(
        eventEnvelope(AgentEvent.HealthReported, {
          [AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel]: JSON.stringify(
            V08SupportedAdapterRuntimeProofReadModel
          ),
          [AgentProtocolDefaults.Field.EnforcementIntegrityRuntimeAuditReadModel]: JSON.stringify(
            V08EnforcementIntegrityRuntimeAuditReadModel
          ),
        })
      )
    ).toEqual({ status: 'rejected', reason: 'unexpected-event' });

    expect(
      parseEnforcementSupportedAdapterRuntimeProofEvent(
        eventEnvelope(AgentEvent.EnforcementSupportedAdapterRuntimeProofReported, {
          [AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel]: '{',
          [AgentProtocolDefaults.Field.EnforcementIntegrityRuntimeAuditReadModel]: JSON.stringify(
            V08EnforcementIntegrityRuntimeAuditReadModel
          ),
        })
      )
    ).toEqual({ status: 'rejected', reason: 'invalid-read-model-json' });

    expect(
      parseEnforcementSupportedAdapterRuntimeProofEvent(
        eventEnvelope(AgentEvent.EnforcementSupportedAdapterRuntimeProofReported, {})
      )
    ).toEqual({ status: 'rejected', reason: 'missing-read-model' });

    expect(
      parseEnforcementSupportedAdapterRuntimeProofEvent(
        eventEnvelope(AgentEvent.EnforcementSupportedAdapterRuntimeProofReported, {
          [AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel]: JSON.stringify({
            ...V08SupportedAdapterRuntimeProofReadModel,
            entries: [
              {
                ...V08SupportedAdapterRuntimeProofReadModel.entries[0],
                broadInstalledAppBlockingClaimed: true,
              },
            ],
          }),
          [AgentProtocolDefaults.Field.EnforcementIntegrityRuntimeAuditReadModel]: JSON.stringify(
            V08EnforcementIntegrityRuntimeAuditReadModel
          ),
        })
      )
    ).toEqual({ status: 'rejected', reason: 'invalid-read-model' });
  });

  it('rejects missing malformed and invalid integrity runtime audit read models', () => {
    expect(
      parseEnforcementSupportedAdapterRuntimeProofEvent(
        eventEnvelope(AgentEvent.EnforcementSupportedAdapterRuntimeProofReported, {
          [AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel]: JSON.stringify(
            V08SupportedAdapterRuntimeProofReadModel
          ),
        })
      )
    ).toEqual({ status: 'rejected', reason: 'missing-integrity-audit-read-model' });

    expect(
      parseEnforcementSupportedAdapterRuntimeProofEvent(
        eventEnvelope(AgentEvent.EnforcementSupportedAdapterRuntimeProofReported, {
          [AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel]: JSON.stringify(
            V08SupportedAdapterRuntimeProofReadModel
          ),
          [AgentProtocolDefaults.Field.EnforcementIntegrityRuntimeAuditReadModel]: '{',
        })
      )
    ).toEqual({ status: 'rejected', reason: 'invalid-integrity-audit-read-model-json' });

    expect(
      parseEnforcementSupportedAdapterRuntimeProofEvent(
        eventEnvelope(AgentEvent.EnforcementSupportedAdapterRuntimeProofReported, {
          [AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel]: JSON.stringify(
            V08SupportedAdapterRuntimeProofReadModel
          ),
          [AgentProtocolDefaults.Field.EnforcementIntegrityRuntimeAuditReadModel]: JSON.stringify({
            ...V08EnforcementIntegrityRuntimeAuditReadModel,
            entries: [
              {
                ...V08EnforcementIntegrityRuntimeAuditReadModel.entries[0],
                tamperHardeningClaimed: true,
              },
            ],
          }),
        })
      )
    ).toEqual({ status: 'rejected', reason: 'invalid-integrity-audit-read-model' });
  });
});

function eventEnvelope(eventName: (typeof AgentEvent)[keyof typeof AgentEvent], payload: Record<string, unknown>) {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    eventId: 'evt-v0-8-supported-adapter-runtime-proof',
    correlationId: 'cmd-v0-8-supported-adapter-runtime-proof',
    sentAt: '2026-06-02T09:03:36.000Z',
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

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
