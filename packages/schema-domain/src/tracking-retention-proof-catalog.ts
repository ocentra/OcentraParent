import {
  AgentTrackingConfigAckState,
  AgentTrackingDeleteAfterAlertResolutionState,
  AgentTrackingDurableSettingsPersistenceState,
  AgentTrackingExecutionClaimState,
  AgentTrackingParentExportState,
  AgentTrackingRemoteAiState,
  AgentTrackingRemoteSyncState,
  AgentTrackingRetentionSettingsWriteDefaults,
  AgentTrackingRetentionSettingsWriteResultSchema,
  type AgentTrackingRetentionSettingsWriteResult,
} from './agent-tracking-retention-settings-write-command';

export const TrackingRetentionProofRefs = {
  WriteCommand:
    'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json',
  LocalServiceState:
    'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json',
  DurableSettings:
    'output/tracking-plan-proof/07-retention-and-custody-model/23-retention-durable-settings-proof.json',
  ProductSettingsWritableExecution:
    'output/tracking-plan-proof/tracking-retention-product-settings-writable-execution-proof/proof.json',
} as const;

export function trackingRetentionAcceptedLocalServiceWriteResult(): AgentTrackingRetentionSettingsWriteResult {
  return AgentTrackingRetentionSettingsWriteResultSchema.parse({
    schemaVersion: 1,
    commandId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
    settingsKind: AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
    writeState: AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
    acceptedAt: AgentTrackingRetentionSettingsWriteDefaults.AcceptedAt,
    sourceWriterIntentRefs: [AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef],
    sourceReadModelProofRefs: AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs,
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
  });
}
