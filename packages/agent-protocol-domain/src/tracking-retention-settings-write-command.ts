import * as TrackingContracts from '@ocentra-parent/schema-domain/agent-tracking-retention-settings-write-command';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

export function defaultAgentTrackingRetentionSettingsWriteRequest():
  TrackingContracts.AgentTrackingRetentionSettingsWriteRequest {
  return TrackingContracts.AgentTrackingRetentionSettingsWriteRequestSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    commandId: TrackingContracts.AgentTrackingRetentionSettingsWriteDefaults.CommandId,
    settingsKind: TrackingContracts.AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
    requestedRetentionWindowHours: 168,
    requestedDeleteAfterAlertResolutionState:
      TrackingContracts.AgentTrackingDeleteAfterAlertResolutionState.RetainAfterAlertResolved,
    requestedParentExportState: TrackingContracts.AgentTrackingParentExportState.NotPrepared,
    requestedRemoteSyncState: TrackingContracts.AgentTrackingRemoteSyncState.Disabled,
    requestedRemoteAiState: TrackingContracts.AgentTrackingRemoteAiState.Disabled,
    sourceWriterIntentRefs: [TrackingContracts.AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef],
    sourceReadModelProofRefs: TrackingContracts.AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs,
  });
}

export function defaultAgentTrackingConfigUpdateRequest(): TrackingContracts.AgentTrackingConfigUpdateRequest {
  const retentionSettings = defaultAgentTrackingRetentionSettingsWriteRequest();
  return TrackingContracts.AgentTrackingConfigUpdateRequestSchema.parse({
    commandId: retentionSettings.commandId,
    runtimeConfig: {
      trackingEnabledState: TrackingContracts.AgentTrackingRuntimeEnabledState.Enabled,
      trackingMode: TrackingContracts.AgentTrackingRuntimeMode.ObserveOnly,
      aiBoundaryMode: TrackingContracts.AgentTrackingAiBoundaryMode.RequestWhenUncertain,
      notificationMode: TrackingContracts.AgentTrackingNotificationMode.ParentPortalOnly,
    },
    retentionSettings,
  });
}

export type AgentTrackingRetentionSettingsWriteResultFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export const AgentTrackingRetentionSettingsWriteResultParseState = {
  Parsed: 'parsed',
  Failed: 'failed',
} as const;

export type AgentTrackingRetentionSettingsWriteResultParseState =
  (typeof AgentTrackingRetentionSettingsWriteResultParseState)[
    keyof typeof AgentTrackingRetentionSettingsWriteResultParseState
  ];

export type AgentTrackingRetentionSettingsWriteResultParseResult =
  | {
      readonly parseState: typeof AgentTrackingRetentionSettingsWriteResultParseState.Parsed;
      readonly value: TrackingContracts.AgentTrackingRetentionSettingsWriteResult;
    }
  | {
      readonly parseState: typeof AgentTrackingRetentionSettingsWriteResultParseState.Failed;
      readonly reason: AgentTrackingRetentionSettingsWriteResultFailureReason;
    };

export function parseAgentTrackingRetentionSettingsWriteResultEvent(
  event: AgentEventEnvelope
): AgentTrackingRetentionSettingsWriteResultParseResult {
  if (event.event !== AgentEvent.ActivityTrackingRetentionSettingsWriteReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsWriteResult];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = TrackingContracts.AgentTrackingRetentionSettingsWriteResultSchema.safeParse(decoded);
  if (!parsed.success) {
    return adapterFailure('invalid-payload');
  }

  return {
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Parsed,
    value: parsed.data,
  };
}

function adapterFailure(
  reason: AgentTrackingRetentionSettingsWriteResultFailureReason
): AgentTrackingRetentionSettingsWriteResultParseResult {
  return {
    parseState: AgentTrackingRetentionSettingsWriteResultParseState.Failed,
    reason,
  };
}
