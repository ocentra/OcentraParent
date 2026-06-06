import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import { AgentProtocolSchemaVersion } from '../src/primitives';
import { parseAgentTrackingRetentionSettingsWriteResultEvent } from '../src/tracking-retention-settings-write-command';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const TrackingRetentionSettingsWriteResult = {
  schemaVersion: AgentProtocolSchemaVersion,
  commandId: 'tracking-retention-write-command-1',
  settingsKind: 'retention-window-setting',
  writeState: 'service-write-command-accepted',
  acceptedAt: '2026-06-06T19:50:00Z',
  sourceMutationProofRefs: [
    'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
  ],
  commandTransportClaimed: true,
  serviceWritePreflightClaimed: true,
  serviceMutationExecuted: false,
  portalWritableUiClaimed: false,
  platformRuntimeClaimed: false,
  childDeviceDeliveryClaimed: false,
  providerDeliveryClaimed: false,
  notificationReceiptClaimed: false,
  physicalDeviceClaimed: false,
  authorityClaimed: false,
  productClaimReady: false,
} as const;

describe('agent tracking retention settings write result parser', () => {
  it('parses accepted service write command results without product overclaims', () => {
    const parsed = parseAgentTrackingRetentionSettingsWriteResultEvent(
      writeResultEvent(JSON.stringify(TrackingRetentionSettingsWriteResult))
    );

    expect(parsed).toEqual({
      ok: true,
      value: TrackingRetentionSettingsWriteResult,
    });
  });

  it('rejects wrong events and invalid write-result payloads', () => {
    expect(
      parseAgentTrackingRetentionSettingsWriteResultEvent({
        ...writeResultEvent(JSON.stringify(TrackingRetentionSettingsWriteResult)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentTrackingRetentionSettingsWriteResultEvent(writeResultEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentTrackingRetentionSettingsWriteResultEvent(
        writeResultEvent(JSON.stringify({ ...TrackingRetentionSettingsWriteResult, productClaimReady: true }))
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function writeResultEvent(serializedResult: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'tracking-retention-settings-write-result-event',
    correlationId: 'tracking-retention-settings-write-command',
    sentAt: '2026-06-06T19:50:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityTrackingRetentionSettingsWriteReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsWriteResult]: serializedResult,
    },
    snapshot: null,
  };
}
