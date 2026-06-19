import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  AgentAppGameNotificationReadinessReason,
  AgentAppGameNotificationReadinessState,
  parseAgentAppGameNotificationReadinessEvent,
} from '../../src/app-game-notification-readiness';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const NotificationReadinessReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-04T23:00:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'notification-intent-ready',
  returned: 2,
  readyIntentCount: 1,
  manualRequiredCount: 1,
  unavailableCount: 0,
  providerDeliveryClaimed: false,
  providerReceiptIngestionClaimed: false,
  localOutboxRuntimeClaimed: false,
  schedulerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  parentUiClaimed: false,
  childDeliveryClaimed: false,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: AgentAppGameNotificationReadinessReason.TimeLimitExceeded,
      reason: AgentAppGameNotificationReadinessReason.TimeLimitExceeded,
      readinessState: AgentAppGameNotificationReadinessState.ReadyForLocalIntent,
      rowCount: 1,
      minimalPayloadRef: 'minimal-alert:time-limit-exceeded',
      evidenceReferenceIds: ['claim-1'],
      evidence: [
        {
          evidenceId: 'claim-1',
          kind: 'local-db-row',
          digest: null,
          uri: null,
        },
      ],
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: AgentAppGameNotificationReadinessReason.ManualRequired,
      reason: AgentAppGameNotificationReadinessReason.ManualRequired,
      readinessState: AgentAppGameNotificationReadinessState.ManualRequired,
      rowCount: 0,
      minimalPayloadRef: 'minimal-alert:manual-required',
      evidenceReferenceIds: [],
      evidence: [],
    },
  ],
} as const;

describe('agent app-game notification readiness parser', () => {
  it('parses the dedicated notification readiness read-model event payload', () => {
    const parsed = parseAgentAppGameNotificationReadinessEvent(
      notificationReadinessEvent(JSON.stringify(NotificationReadinessReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: NotificationReadinessReadModel,
    });
  });

  it('rejects invalid notification payloads and provider delivery claims', () => {
    expect(
      parseAgentAppGameNotificationReadinessEvent({
        ...notificationReadinessEvent(JSON.stringify(NotificationReadinessReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameNotificationReadinessEvent(notificationReadinessEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameNotificationReadinessEvent(
        notificationReadinessEvent(
          JSON.stringify({
            ...NotificationReadinessReadModel,
            providerDeliveryClaimed: true,
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function notificationReadinessEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-notification-readiness-event',
    correlationId: 'app-game-notification-readiness-command',
    sentAt: '2026-06-04T23:00:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameNotificationReadinessReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGameNotificationReadinessReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}
