import { describe, expect, it } from 'vitest';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  AgentTrackingConfigUpdateEventType,
  AgentTrackingConfigAckState,
  AgentTrackingDeleteAfterAlertResolutionState,
  AgentTrackingDurableSettingsPersistenceState,
  AgentTrackingExecutionClaimState,
  AgentTrackingParentExportState,
  AgentTrackingRemoteAiState,
  AgentTrackingRemoteSyncState,
  AgentTrackingRetentionSettingsWriteDefaults,
} from '@ocentra-parent/schema-domain/agent-tracking-retention-settings-write-command';
import {
  parseAgentTrackingRetentionSettingsWriteResultEvent,
  AgentTrackingRetentionSettingsWriteResultParseState,
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
  settingsKind: AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
  writeState: AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
  acceptedAt: AgentTrackingRetentionSettingsWriteDefaults.AcceptedAt,
  sourceWriterIntentRefs: [AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef],
  sourceReadModelProofRefs: [AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs[0]],
  sourceMutationProofRefs: [AgentTrackingRetentionSettingsWriteDefaults.MutationProofRef],
  appliedRetentionWindowHours: 168,
  appliedDeleteAfterAlertResolutionState: AgentTrackingDeleteAfterAlertResolutionState.RetainAfterAlertResolved,
  parentExportState: AgentTrackingParentExportState.NotPrepared,
  remoteSyncState: AgentTrackingRemoteSyncState.Disabled,
  remoteAiState: AgentTrackingRemoteAiState.Disabled,
  localServiceStateRevision: 1,
  localServiceStateSnapshotRef: AgentTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef,
  durableSettingsStoreRef: AgentTrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef,
  durableSettingsPersistenceState: AgentTrackingDurableSettingsPersistenceState.Persisted,
  childConfigAckState: AgentTrackingConfigAckState.Received,
  commandTransportClaimState: AgentTrackingExecutionClaimState.Claimed,
  serviceWritePreflightClaimState: AgentTrackingExecutionClaimState.Claimed,
  serviceMutationExecutionState: AgentTrackingExecutionClaimState.Claimed,
  portalWritableUiClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  platformRuntimeClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  childDeviceDeliveryClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  providerDeliveryClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  notificationReceiptClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  physicalDeviceClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  authorityClaimState: AgentTrackingExecutionClaimState.Unclaimed,
  productClaimState: AgentTrackingExecutionClaimState.Unclaimed,
} as const;

describe('agent tracking retention settings write result parser', () => {
  it(
    'parses accepted service write command results without product overclaims',
    assertAcceptedServiceWriteCommandResults
  );
  it('rejects wrong events and invalid write-result payloads', assertInvalidWriteResultPayloads);
});

function writeResultEvent(serializedResult: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'tracking-retention-settings-write-result-event',
    correlationId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
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

function assertAcceptedServiceWriteCommandResults() {
  const parsed = parseAgentTrackingRetentionSettingsWriteResultEvent(
    writeResultEvent(JSON.stringify(TrackingRetentionSettingsWriteResult))
  );

  expect(parsed).toEqual({
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Parsed,
    value: TrackingRetentionSettingsWriteResult,
  });
}

function assertInvalidWriteResultPayloads() {
  expect(
    parseAgentTrackingRetentionSettingsWriteResultEvent({
      ...writeResultEvent(JSON.stringify(TrackingRetentionSettingsWriteResult)),
      event: AgentEvent.HealthReported,
    })
  ).toEqual({
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
    reason: 'wrong-event',
  });
  expect(parseAgentTrackingRetentionSettingsWriteResultEvent(writeResultEvent('{'))).toEqual({
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
    reason: 'invalid-json',
  });
  expect(
    parseAgentTrackingRetentionSettingsWriteResultEvent(
      writeResultEvent(
        JSON.stringify({
          ...TrackingRetentionSettingsWriteResult,
          productClaimState: AgentTrackingExecutionClaimState.Claimed,
        })
      )
    )
  ).toEqual({
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
    reason: 'invalid-payload',
  });
  expect(
    parseAgentTrackingRetentionSettingsWriteResultEvent(
      writeResultEvent(
        JSON.stringify({
          ...TrackingRetentionSettingsWriteResult,
          serviceMutationExecutionState: AgentTrackingExecutionClaimState.Unclaimed,
        })
      )
    )
  ).toEqual({
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
    reason: 'invalid-payload',
  });
  expect(
    parseAgentTrackingRetentionSettingsWriteResultEvent(
      writeResultEvent(
        JSON.stringify({
          ...TrackingRetentionSettingsWriteResult,
          durableSettingsPersistenceState: AgentTrackingDurableSettingsPersistenceState.NotPersisted,
        })
      )
    )
  ).toEqual({
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
    reason: 'invalid-payload',
  });
}
