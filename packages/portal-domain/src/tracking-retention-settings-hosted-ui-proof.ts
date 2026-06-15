import type { AgentTrackingRetentionSettingsWriteResultParseResult } from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import {
  AgentTrackingRetentionSettingsWriteResultParseState,
} from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import type { DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalFormatting } from './formatting';
import { TrackingStatusProofArtifacts, type TrackingStatusProofArtifact } from './tracking-status-proof-artifacts';

type PortalDisplayText = DisplayText;
type PortalTextTokenValue = (typeof PortalDevTextToken)[keyof typeof PortalDevTextToken];

export type TrackingRetentionSettingsWritePreflight = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly commandId: PortalDetailValue;
  readonly settingsKind: PortalDetailValue;
  readonly writeState: PortalDetailValue;
  readonly acceptedAt: PortalDetailValue;
  readonly sourceMutationProofRefs: PortalDetailValue;
  readonly sourceWriterIntentRefs: PortalDetailValue;
  readonly sourceReadModelProofRefs: PortalDetailValue;
  readonly appliedRetentionWindowHours: PortalDetailValue;
  readonly appliedDeleteAfterAlertResolved: PortalDetailValue;
  readonly parentExportPrepared: PortalDetailValue;
  readonly remoteSyncEnabled: PortalDetailValue;
  readonly remoteAiEnabled: PortalDetailValue;
  readonly localServiceStateRevision: PortalDetailValue;
  readonly localServiceStateSnapshotRef: PortalDetailValue;
  readonly durableSettingsPersistedRows: PortalDetailValue;
  readonly commandTransportClaimedRows: PortalDetailValue;
  readonly serviceWritePreflightClaimedRows: PortalDetailValue;
  readonly serviceMutationExecutedRows: PortalDetailValue;
  readonly platformRuntimeClaimedRows: PortalDetailValue;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly notificationReceiptClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly parserReason: PortalDetailValue;
  readonly boundary: PortalDisplayText;
};

export type TrackingRetentionSettingsHostedUiRow = {
  readonly title: PortalDisplayText;
  readonly status: PortalDisplayText;
  readonly evidence: PortalDisplayText;
};

export type TrackingRetentionSettingsHostedUiProof = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly proofTier: PortalDisplayText;
  readonly rowsReturned: PortalDetailValue;
  readonly proofArtifact: TrackingStatusProofArtifact;
  readonly writeCommandProofArtifact: TrackingStatusProofArtifact;
  readonly localStateProofArtifact: TrackingStatusProofArtifact;
  readonly boundary: PortalDisplayText;
  readonly missingProof: PortalDisplayText;
  readonly productClaim: PortalDisplayText;
  readonly serviceMutationClaimedRows: PortalDetailValue;
  readonly platformRuntimeClaimedRows: PortalDetailValue;
  readonly childDeviceDeliveryClaimedRows: PortalDetailValue;
  readonly providerDeliveryClaimedRows: PortalDetailValue;
  readonly physicalDeviceClaimedRows: PortalDetailValue;
  readonly authorityClaimedRows: PortalDetailValue;
  readonly productClaimReadyRows: PortalDetailValue;
  readonly writePreflight: TrackingRetentionSettingsWritePreflight;
  readonly rows: readonly TrackingRetentionSettingsHostedUiRow[];
};

type TrackingRetentionSettingsHostedUiDefinition = {
  readonly titleToken: PortalTextTokenValue;
  readonly evidenceToken: PortalTextTokenValue;
};

const TrackingRetentionSettingsHostedUiDefinitions = [
  {
    titleToken: PortalDevTextToken.TrackingRetentionSettingsWindow,
    evidenceToken: PortalDevTextToken.TrackingRetentionSettingsWindowEvidence,
  },
  {
    titleToken: PortalDevTextToken.TrackingRetentionSettingsDeleteAfterAlert,
    evidenceToken: PortalDevTextToken.TrackingRetentionSettingsDeleteAfterAlertEvidence,
  },
  {
    titleToken: PortalDevTextToken.TrackingRetentionSettingsParentExport,
    evidenceToken: PortalDevTextToken.TrackingRetentionSettingsParentExportEvidence,
  },
  {
    titleToken: PortalDevTextToken.TrackingRetentionSettingsRemoteSyncDisabled,
    evidenceToken: PortalDevTextToken.TrackingRetentionSettingsRemoteSyncEvidence,
  },
  {
    titleToken: PortalDevTextToken.TrackingRetentionSettingsRemoteAiDisabled,
    evidenceToken: PortalDevTextToken.TrackingRetentionSettingsRemoteAiEvidence,
  },
] as const satisfies readonly TrackingRetentionSettingsHostedUiDefinition[];

export function trackingRetentionSettingsHostedUiProof(
  writeResult: AgentTrackingRetentionSettingsWriteResultParseResult | null = null
): TrackingRetentionSettingsHostedUiProof {
  const rows = TrackingRetentionSettingsHostedUiDefinitions.map((definition) =>
    retentionSettingsHostedUiRow(definition)
  );
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingRetentionSettingsHostedUi),
    body: resolvePortalDevText(PortalDevTextToken.TrackingRetentionSettingsHostedUiBody),
    proofTier: resolvePortalDevText(PortalDevTextToken.TrackingProofService),
    rowsReturned: detailFromValue(rows.length),
    proofArtifact: TrackingStatusProofArtifacts.RetentionSettingsReadModel,
    writeCommandProofArtifact: TrackingStatusProofArtifacts.RetentionSettingsWriteCommand,
    localStateProofArtifact: TrackingStatusProofArtifacts.RetentionLocalServiceState,
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingRetentionSettingsHostedBoundary),
    missingProof: resolvePortalDevText(PortalDevTextToken.TrackingManualRequired),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
    serviceMutationClaimedRows: detailFromValue(0),
    platformRuntimeClaimedRows: detailFromValue(0),
    childDeviceDeliveryClaimedRows: detailFromValue(0),
    providerDeliveryClaimedRows: detailFromValue(0),
    physicalDeviceClaimedRows: detailFromValue(0),
    authorityClaimedRows: detailFromValue(0),
    productClaimReadyRows: detailFromValue(0),
    writePreflight: retentionSettingsWritePreflight(writeResult),
    rows,
  };
}

function retentionSettingsHostedUiRow(
  definition: TrackingRetentionSettingsHostedUiDefinition
): TrackingRetentionSettingsHostedUiRow {
  return {
    title: resolvePortalDevText(definition.titleToken),
    status: resolvePortalDevText(PortalDevTextToken.TrackingRetentionSettingsReadModelReady),
    evidence: resolvePortalDevText(definition.evidenceToken),
  };
}

function retentionSettingsWritePreflight(
  writeResult: AgentTrackingRetentionSettingsWriteResultParseResult | null
): TrackingRetentionSettingsWritePreflight {
  const base = emptyRetentionSettingsWritePreflight();
  if (writeResult === null) {
    return base;
  }
  if (writeResult.parseState === AgentTrackingRetentionSettingsWriteResultParseState.Failed) {
    return {
      ...base,
      parserReason: detailFromValue(writeResult.reason),
    };
  }
  const value = writeResult.value;
  return {
    ...base,
    commandId: detailFromValue(value.commandId),
    settingsKind: detailFromValue(value.settingsKind),
    writeState: detailFromValue(value.writeState),
    acceptedAt: detailFromValue(value.acceptedAt),
    sourceMutationProofRefs: detailFromValue(value.sourceMutationProofRefs.join(PortalFormatting.EventDetailSeparator)),
    sourceWriterIntentRefs: detailFromValue(value.sourceWriterIntentRefs.join(PortalFormatting.EventDetailSeparator)),
    sourceReadModelProofRefs: detailFromValue(
      value.sourceReadModelProofRefs.join(PortalFormatting.EventDetailSeparator)
    ),
    appliedRetentionWindowHours: detailFromValue(value.appliedRetentionWindowHours ?? 0),
    appliedDeleteAfterAlertResolved: detailFromValue(value.appliedDeleteAfterAlertResolutionState),
    parentExportPrepared: detailFromValue(value.parentExportState),
    remoteSyncEnabled: detailFromValue(value.remoteSyncState),
    remoteAiEnabled: detailFromValue(value.remoteAiState),
    localServiceStateRevision: detailFromValue(value.localServiceStateRevision ?? 0),
    localServiceStateSnapshotRef: detailFromValue(value.localServiceStateSnapshotRef),
    durableSettingsPersistedRows: detailFromValue(value.durableSettingsPersistenceState),
    commandTransportClaimedRows: detailFromValue(value.commandTransportClaimState),
    serviceWritePreflightClaimedRows: detailFromValue(value.serviceWritePreflightClaimState),
    serviceMutationExecutedRows: detailFromValue(value.serviceMutationExecutionState),
    platformRuntimeClaimedRows: detailFromValue(value.platformRuntimeClaimState),
    childDeviceDeliveryClaimedRows: detailFromValue(value.childDeviceDeliveryClaimState),
    providerDeliveryClaimedRows: detailFromValue(value.providerDeliveryClaimState),
    notificationReceiptClaimedRows: detailFromValue(value.notificationReceiptClaimState),
    physicalDeviceClaimedRows: detailFromValue(value.physicalDeviceClaimState),
    authorityClaimedRows: detailFromValue(value.authorityClaimState),
    productClaimReadyRows: detailFromValue(value.productClaimState),
  };
}

function emptyRetentionSettingsWritePreflight(): TrackingRetentionSettingsWritePreflight {
  return {
    title: resolvePortalDevText(PortalDevTextToken.TrackingRetentionSettingsWritePreflight),
    body: resolvePortalDevText(PortalDevTextToken.TrackingRetentionSettingsWritePreflightBody),
    commandId: notReported(),
    settingsKind: notReported(),
    writeState: notReported(),
    acceptedAt: notReported(),
    sourceMutationProofRefs: notReported(),
    sourceWriterIntentRefs: notReported(),
    sourceReadModelProofRefs: notReported(),
    appliedRetentionWindowHours: notReported(),
    appliedDeleteAfterAlertResolved: detailFromValue(0),
    parentExportPrepared: detailFromValue(0),
    remoteSyncEnabled: detailFromValue(0),
    remoteAiEnabled: detailFromValue(0),
    localServiceStateRevision: detailFromValue(0),
    localServiceStateSnapshotRef: notReported(),
    durableSettingsPersistedRows: detailFromValue(0),
    commandTransportClaimedRows: detailFromValue(0),
    serviceWritePreflightClaimedRows: detailFromValue(0),
    serviceMutationExecutedRows: detailFromValue(0),
    platformRuntimeClaimedRows: detailFromValue(0),
    childDeviceDeliveryClaimedRows: detailFromValue(0),
    providerDeliveryClaimedRows: detailFromValue(0),
    notificationReceiptClaimedRows: detailFromValue(0),
    physicalDeviceClaimedRows: detailFromValue(0),
    authorityClaimedRows: detailFromValue(0),
    productClaimReadyRows: detailFromValue(0),
    parserReason: notReported(),
    boundary: resolvePortalDevText(PortalDevTextToken.TrackingRetentionSettingsWritePreflightBoundary),
  };
}

function notReported(): PortalDetailValue {
  return detailFromValue(resolvePortalDevText(PortalDevTextToken.NotReported));
}

function detailFromValue(value: unknown): PortalDetailValue {
  return decodePortalDetailValue(String(value));
}
