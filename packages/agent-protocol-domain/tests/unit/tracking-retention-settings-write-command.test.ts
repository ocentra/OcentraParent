import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import { AgentProtocolSchemaVersion } from '../../src/primitives';
import {
  AgentTrackingConfigUpdateEventType,
  AgentTrackingRetentionSettingsWriteRequestSchema,
  ChildTrackingConfigUpdatedEventSchema,
  ParentTrackingConfigUpdatedEventSchema,
  defaultAgentTrackingRetentionSettingsWriteRequest,
  parseAgentTrackingRetentionSettingsWriteResultEvent,
} from '../../src/tracking-retention-settings-write-command';

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
  sourceWriterIntentRefs: ['tracking-retention-settings-write-retention-window'],
  sourceReadModelProofRefs: [
    'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
  ],
  sourceMutationProofRefs: [
    'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
  ],
  appliedRetentionWindowHours: 168,
  appliedDeleteAfterAlertResolved: false,
  parentExportPrepared: false,
  remoteSyncEnabled: false,
  remoteAiEnabled: false,
  localServiceStateRevision: 1,
  localServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
  durableSettingsStoreRef: 'agent-service-local-retention-settings-durable-json',
  durableSettingsPersisted: true,
  commandTransportClaimed: true,
  serviceWritePreflightClaimed: true,
  serviceMutationExecuted: true,
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
  it('parses local service write requests without remote sync or remote AI claims', () => {
    expect(
      AgentTrackingRetentionSettingsWriteRequestSchema.parse(defaultAgentTrackingRetentionSettingsWriteRequest())
    ).toEqual({
      schemaVersion: AgentProtocolSchemaVersion,
      commandId: 'tracking-retention-settings-write-command',
      settingsKind: 'retention-window-setting',
      requestedRetentionWindowHours: 168,
      requestedDeleteAfterAlertResolved: false,
      requestedParentExport: false,
      requestedRemoteSyncEnabled: false,
      requestedRemoteAiEnabled: false,
      sourceWriterIntentRefs: ['tracking-retention-settings-write-retention-window'],
      sourceReadModelProofRefs: [
        'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
      ],
    });
  });

  it('parses parent and child tracking config update events as canonical protocol contracts', () => {
    const config = defaultAgentTrackingRetentionSettingsWriteRequest();
    const target = {
      scope: 'child-device',
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    } as const;
    const parentEvent = ParentTrackingConfigUpdatedEventSchema.parse({
      sourceCommandId: config.commandId,
      sourceMessageId: 'tracking-retention-settings-write-command',
      sourcePeerId: 'portal-dev',
      target,
      config,
    });

    expect(parentEvent.target.scope).toBe('child-device');
    expect(
      ChildTrackingConfigUpdatedEventSchema.parse({
        parentEventType: AgentTrackingConfigUpdateEventType.Parent,
        sourceCommandId: parentEvent.sourceCommandId,
        target: parentEvent.target,
        config: parentEvent.config,
      })
    ).toEqual({
      parentEventType: AgentTrackingConfigUpdateEventType.Parent,
      sourceCommandId: config.commandId,
      target,
      config,
    });
  });

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
    expect(
      parseAgentTrackingRetentionSettingsWriteResultEvent(
        writeResultEvent(JSON.stringify({ ...TrackingRetentionSettingsWriteResult, serviceMutationExecuted: false }))
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentTrackingRetentionSettingsWriteResultEvent(
        writeResultEvent(JSON.stringify({ ...TrackingRetentionSettingsWriteResult, durableSettingsPersisted: false }))
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
