import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

const NullableNonEmptyStringSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const NetworkLiveCaptureCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NetworkLiveCaptureRefs = AgentProtocolDefaults.NetworkLiveCaptureStatus;
const NetworkLiveCaptureStatusErrorMessage =
  'Network live capture status must preserve row13 and row03a readiness refs, exact proof/manual/' +
  'unavailable/degraded rows, and no live capture/content/authority claims';

export const AgentNetworkLiveCapturePlatformSchema = withParser(
  Schema.Literal('windows-npcap', 'linux-libpcap', 'macos-bpf-libpcap')
);

export const AgentNetworkLiveCaptureProofStateSchema = withParser(
  Schema.Literal('proof-ready', 'manual-required', 'unavailable', 'degraded')
);

export const AgentNetworkRawCaptureStorageStateSchema = withParser(
  Schema.Literal('custody-ready', 'manual-required', 'unavailable', 'degraded')
);

export const AgentNetworkLiveCaptureExecutionStateSchema = withParser(
  Schema.Literal('bounded-executed', 'manual-required', 'unavailable', 'degraded')
);

const AgentNetworkLiveCaptureStatusRowSchema = Schema.Struct({
  platform: AgentNetworkLiveCapturePlatformSchema,
  captureProofRef: NonEmptyStringSchema,
  proofState: AgentNetworkLiveCaptureProofStateSchema,
  storageProofRef: NonEmptyStringSchema,
  storageState: AgentNetworkRawCaptureStorageStateSchema,
  interfaceRef: NullableNonEmptyStringSchema,
  driverProofRef: NullableNonEmptyStringSchema,
  permissionProofRef: NullableNonEmptyStringSchema,
  boundedCaptureRef: NullableNonEmptyStringSchema,
  cleanStopRef: NullableNonEmptyStringSchema,
  quotaRotationRef: NullableNonEmptyStringSchema,
  retentionDeleteExportRef: NullableNonEmptyStringSchema,
  custodyRef: NullableNonEmptyStringSchema,
  privateTrafficExclusionRef: NullableNonEmptyStringSchema,
  rawArtifactManifestRef: NullableNonEmptyStringSchema,
  storageLocationRef: NullableNonEmptyStringSchema,
  encryptionAtRestRef: NullableNonEmptyStringSchema,
  storageQuotaRotationRef: NullableNonEmptyStringSchema,
  retentionPolicyRef: NullableNonEmptyStringSchema,
  storageDeleteExportRef: NullableNonEmptyStringSchema,
  custodyChainRef: NullableNonEmptyStringSchema,
  storagePrivateTrafficExclusionRef: NullableNonEmptyStringSchema,
  executionRef: NullableNonEmptyStringSchema,
  executionState: AgentNetworkLiveCaptureExecutionStateSchema,
  executionMissingArtifactCount: NetworkLiveCaptureCount,
  driverInvocationRef: NullableNonEmptyStringSchema,
  interfaceObservationRef: NullableNonEmptyStringSchema,
  executionPermissionRef: NullableNonEmptyStringSchema,
  boundedWindowRef: NullableNonEmptyStringSchema,
  executionCleanStopRef: NullableNonEmptyStringSchema,
  executionCustodyRef: NullableNonEmptyStringSchema,
  executionRetentionDeleteExportRef: NullableNonEmptyStringSchema,
  metadataOnlySanitizationRef: NullableNonEmptyStringSchema,
  executionPrivateTrafficExclusionRef: NullableNonEmptyStringSchema,
  metadataSnapshotExecuted: Schema.Boolean,
  capturedPacketCount: NetworkLiveCaptureCount,
  rawArtifactCreated: Schema.Literal(false),
  missingArtifactCount: NetworkLiveCaptureCount,
  storageMissingArtifactCount: NetworkLiveCaptureCount,
  captureReady: Schema.Boolean,
  rawArtifactStorageAuthorized: Schema.Boolean,
  driverInvoked: Schema.Boolean,
  liveCaptureExecuted: Schema.Boolean,
  remoteUploadEnabled: Schema.Literal(false),
  rawPcapWithoutCustodyAvailable: Schema.Literal(false),
  exactUrlAvailable: Schema.Literal(false),
  decryptedPayloadAvailable: Schema.Literal(false),
  pageContentAvailable: Schema.Literal(false),
  privateMessageAvailable: Schema.Literal(false),
  searchQueryAvailable: Schema.Literal(false),
  policyAuthority: Schema.Literal(false),
  adapterAuthority: Schema.Literal(false),
  enforcementCommandsPublished: Schema.Literal(0),
  netstatMetadataSubstitutedForLiveCapture: Schema.Literal(false),
  hostFilteringClaimed: Schema.Literal(false),
});

const AgentNetworkLiveCaptureStatusFields = Schema.Struct({
  statusRef: NonEmptyStringSchema,
  row13StatusRef: NonEmptyStringSchema,
  executionStatusRef: NonEmptyStringSchema,
  rawStorageStatusRef: NonEmptyStringSchema,
  platformRowCount: NetworkLiveCaptureCount,
  proofReadyCount: NetworkLiveCaptureCount,
  manualRequiredCount: NetworkLiveCaptureCount,
  unavailableCount: NetworkLiveCaptureCount,
  degradedCount: NetworkLiveCaptureCount,
  requiredArtifactCount: NetworkLiveCaptureCount,
  missingArtifactCount: NetworkLiveCaptureCount,
  storageCustodyReadyCount: NetworkLiveCaptureCount,
  storageManualRequiredCount: NetworkLiveCaptureCount,
  storageUnavailableCount: NetworkLiveCaptureCount,
  storageDegradedCount: NetworkLiveCaptureCount,
  storageMissingArtifactCount: NetworkLiveCaptureCount,
  boundedExecutedCount: NetworkLiveCaptureCount,
  executionManualRequiredCount: NetworkLiveCaptureCount,
  executionUnavailableCount: NetworkLiveCaptureCount,
  executionDegradedCount: NetworkLiveCaptureCount,
  executionMissingArtifactCount: NetworkLiveCaptureCount,
  metadataSnapshotExecutedCount: NetworkLiveCaptureCount,
  capturedPacketCount: NetworkLiveCaptureCount,
  rawArtifactCreatedCount: Schema.Literal(0),
  captureReadyCount: NetworkLiveCaptureCount,
  rawArtifactStorageAuthorizedCount: NetworkLiveCaptureCount,
  driverInvokedCount: NetworkLiveCaptureCount,
  liveCaptureExecutedCount: NetworkLiveCaptureCount,
  remoteUploadEnabledCount: Schema.Literal(0),
  rawPcapWithoutCustodyAvailableCount: Schema.Literal(0),
  exactUrlAvailableCount: Schema.Literal(0),
  decryptedPayloadAvailableCount: Schema.Literal(0),
  pageContentAvailableCount: Schema.Literal(0),
  privateMessageAvailableCount: Schema.Literal(0),
  searchQueryAvailableCount: Schema.Literal(0),
  policyAuthorityCount: Schema.Literal(0),
  adapterAuthorityCount: Schema.Literal(0),
  enforcementCommandEventCount: Schema.Literal(0),
  netstatMetadataSubstitutionCount: Schema.Literal(0),
  hostFilteringClaimCount: Schema.Literal(0),
  rows: Schema.Array(AgentNetworkLiveCaptureStatusRowSchema),
});

export type AgentNetworkLiveCaptureStatusRow = Infer<typeof AgentNetworkLiveCaptureStatusRowSchema>;
export type AgentNetworkLiveCaptureStatus = Infer<typeof AgentNetworkLiveCaptureStatusFields>;

export const AgentNetworkLiveCaptureStatusSchema = withParser(
  AgentNetworkLiveCaptureStatusFields.pipe(
    Schema.filter(
      (status: AgentNetworkLiveCaptureStatus) =>
        (statusRefsMatch(status) &&
          rowCountsMatch(status) &&
          rowRefsMatch(status) &&
          readinessMatches(status) &&
          missingArtifactCountsMatch(status)) ||
        NetworkLiveCaptureStatusErrorMessage
    )
  )
);

export type AgentNetworkLiveCaptureStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkLiveCaptureStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-live-capture-status'
        | 'invalid-live-capture-status-json'
        | 'invalid-live-capture-status';
    };

export function parseAgentNetworkLiveCaptureStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkLiveCaptureStatusParseResult {
  if (event.event !== AgentEvent.NetworkLiveCaptureStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkLiveCaptureStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-live-capture-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-live-capture-status-json' };
  }

  const parsed = AgentNetworkLiveCaptureStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-live-capture-status' };
  }

  return { ok: true, status: parsed.data };
}

function statusRefsMatch(status: AgentNetworkLiveCaptureStatus): boolean {
  return (
    status.statusRef === NetworkLiveCaptureRefs.StatusRef &&
    status.row13StatusRef === NetworkLiveCaptureRefs.Row13StatusRef &&
    status.executionStatusRef === NetworkLiveCaptureRefs.ExecutionStatusRef &&
    status.rawStorageStatusRef === NetworkLiveCaptureRefs.RawStorageStatusRef
  );
}

function rowCountsMatch(status: AgentNetworkLiveCaptureStatus): boolean {
  return (
    status.rows.length === status.platformRowCount &&
    status.platformRowCount === 4 &&
    proofStateCountsMatch(status) &&
    storageStateCountsMatch(status) &&
    executionStateCountsMatch(status) &&
    executionObservationCountsMatch(status) &&
    status.requiredArtifactCount === status.platformRowCount * 9
  );
}

function rowRefsMatch(status: AgentNetworkLiveCaptureStatus): boolean {
  const refs = new Map(status.rows.map((row) => [row.captureProofRef, row]));
  return (
    rowIdentityMatches(refs.get(NetworkLiveCaptureRefs.WindowsProofRef), 'windows-npcap', 'proof-ready') &&
    rowIdentityMatches(refs.get(NetworkLiveCaptureRefs.ManualProofRef), 'windows-npcap', 'manual-required') &&
    rowIdentityMatches(refs.get(NetworkLiveCaptureRefs.LinuxProofRef), 'linux-libpcap', 'unavailable') &&
    rowIdentityMatches(refs.get(NetworkLiveCaptureRefs.MacosProofRef), 'macos-bpf-libpcap', 'degraded') &&
    status.rows.every((row) => row.storageProofRef === NetworkLiveCaptureRefs.RawStorageStatusRef) &&
    status.rows.every(rowExecutionIdentityMatches) &&
    status.rows.every(rowRequiredRefsMatch)
  );
}

function rowIdentityMatches(
  row: AgentNetworkLiveCaptureStatusRow | undefined,
  platform: AgentNetworkLiveCaptureStatusRow['platform'],
  proofState: AgentNetworkLiveCaptureStatusRow['proofState']
): boolean {
  return row?.platform === platform && row.proofState === proofState;
}

function rowRequiredRefsMatch(row: AgentNetworkLiveCaptureStatusRow): boolean {
  if (row.proofState === 'proof-ready') {
    return liveCaptureRefsMatch(row) && rawStorageRefsMatch(row) && boundedExecutionRefsMatch(row);
  }
  if (row.proofState === 'degraded') {
    return liveCaptureRefsMatch(row) && rawStorageRefsEmpty(row) && executionRefsEmpty(row);
  }
  return liveCaptureRefsEmpty(row) && rawStorageRefsEmpty(row) && executionRefsEmpty(row);
}

function liveCaptureRefsMatch(row: AgentNetworkLiveCaptureStatusRow): boolean {
  return (
    row.interfaceRef === NetworkLiveCaptureRefs.InterfaceRef &&
    row.driverProofRef === NetworkLiveCaptureRefs.DriverRef &&
    row.permissionProofRef === NetworkLiveCaptureRefs.PermissionRef &&
    row.boundedCaptureRef === NetworkLiveCaptureRefs.BoundedCaptureRef &&
    row.cleanStopRef === NetworkLiveCaptureRefs.CleanStopRef &&
    row.quotaRotationRef === NetworkLiveCaptureRefs.QuotaRef &&
    row.retentionDeleteExportRef === NetworkLiveCaptureRefs.RetentionRef &&
    row.custodyRef === NetworkLiveCaptureRefs.CustodyRef &&
    row.privateTrafficExclusionRef === NetworkLiveCaptureRefs.PrivateTrafficExclusionRef
  );
}

function rawStorageRefsMatch(row: AgentNetworkLiveCaptureStatusRow): boolean {
  return (
    row.rawArtifactManifestRef === NetworkLiveCaptureRefs.RawManifestRef &&
    row.storageLocationRef === NetworkLiveCaptureRefs.RawStorageLocationRef &&
    row.encryptionAtRestRef === NetworkLiveCaptureRefs.RawEncryptionRef &&
    row.storageQuotaRotationRef === NetworkLiveCaptureRefs.RawQuotaRef &&
    row.retentionPolicyRef === NetworkLiveCaptureRefs.RawRetentionRef &&
    row.storageDeleteExportRef === NetworkLiveCaptureRefs.RawDeleteExportRef &&
    row.custodyChainRef === NetworkLiveCaptureRefs.RawCustodyChainRef &&
    row.storagePrivateTrafficExclusionRef === NetworkLiveCaptureRefs.RawPrivateTrafficExclusionRef
  );
}

function liveCaptureRefsEmpty(row: AgentNetworkLiveCaptureStatusRow): boolean {
  return (
    row.interfaceRef === null &&
    row.driverProofRef === null &&
    row.permissionProofRef === null &&
    row.boundedCaptureRef === null &&
    row.cleanStopRef === null &&
    row.quotaRotationRef === null &&
    row.retentionDeleteExportRef === null &&
    row.custodyRef === null &&
    row.privateTrafficExclusionRef === null
  );
}

function rawStorageRefsEmpty(row: AgentNetworkLiveCaptureStatusRow): boolean {
  return (
    row.rawArtifactManifestRef === null &&
    row.storageLocationRef === null &&
    row.encryptionAtRestRef === null &&
    row.storageQuotaRotationRef === null &&
    row.retentionPolicyRef === null &&
    row.storageDeleteExportRef === null &&
    row.custodyChainRef === null &&
    row.storagePrivateTrafficExclusionRef === null
  );
}

function rowExecutionIdentityMatches(row: AgentNetworkLiveCaptureStatusRow): boolean {
  if (row.captureProofRef === NetworkLiveCaptureRefs.WindowsProofRef) {
    return row.executionRef === NetworkLiveCaptureRefs.WindowsExecutionRef && row.executionState === 'bounded-executed';
  }
  if (row.captureProofRef === NetworkLiveCaptureRefs.ManualProofRef) {
    return row.executionRef === NetworkLiveCaptureRefs.ManualExecutionRef && row.executionState === 'manual-required';
  }
  if (row.captureProofRef === NetworkLiveCaptureRefs.LinuxProofRef) {
    return row.executionRef === NetworkLiveCaptureRefs.LinuxExecutionRef && row.executionState === 'unavailable';
  }
  return (
    row.captureProofRef === NetworkLiveCaptureRefs.MacosProofRef &&
    row.executionRef === NetworkLiveCaptureRefs.MacosExecutionRef &&
    row.executionState === 'degraded'
  );
}

function boundedExecutionRefsMatch(row: AgentNetworkLiveCaptureStatusRow): boolean {
  return boundedExecutionArtifactRefsMatch(row) && boundedExecutionCountersMatch(row);
}

function boundedExecutionArtifactRefsMatch(row: AgentNetworkLiveCaptureStatusRow): boolean {
  return (
    row.driverInvocationRef === NetworkLiveCaptureRefs.DriverInvocationRef &&
    row.interfaceObservationRef === NetworkLiveCaptureRefs.InterfaceObservationRef &&
    row.executionPermissionRef === NetworkLiveCaptureRefs.ExecutionPermissionRef &&
    row.boundedWindowRef === NetworkLiveCaptureRefs.BoundedWindowRef &&
    row.executionCleanStopRef === NetworkLiveCaptureRefs.ExecutionCleanStopRef &&
    row.executionCustodyRef === NetworkLiveCaptureRefs.ExecutionCustodyRef &&
    row.executionRetentionDeleteExportRef === NetworkLiveCaptureRefs.ExecutionRetentionRef &&
    row.metadataOnlySanitizationRef === NetworkLiveCaptureRefs.MetadataSanitizationRef &&
    row.executionPrivateTrafficExclusionRef === NetworkLiveCaptureRefs.ExecutionPrivateTrafficExclusionRef
  );
}

function boundedExecutionCountersMatch(row: AgentNetworkLiveCaptureStatusRow): boolean {
  return (
    row.driverInvoked &&
    row.liveCaptureExecuted &&
    row.capturedPacketCount > 0 &&
    row.executionMissingArtifactCount === 0
  );
}

function executionRefsEmpty(row: AgentNetworkLiveCaptureStatusRow): boolean {
  return (
    row.driverInvocationRef === null &&
    row.interfaceObservationRef === null &&
    row.executionPermissionRef === null &&
    row.boundedWindowRef === null &&
    row.executionCleanStopRef === null &&
    row.executionCustodyRef === null &&
    row.executionRetentionDeleteExportRef === null &&
    row.metadataOnlySanitizationRef === null &&
    row.executionPrivateTrafficExclusionRef === null &&
    !row.driverInvoked &&
    !row.liveCaptureExecuted &&
    row.capturedPacketCount === 0
  );
}

function readinessMatches(status: AgentNetworkLiveCaptureStatus): boolean {
  const readyRows = status.rows.filter((row) => row.captureReady);
  const storageReadyRows = status.rows.filter((row) => row.rawArtifactStorageAuthorized);
  return (
    status.captureReadyCount === readyRows.length &&
    status.rawArtifactStorageAuthorizedCount === storageReadyRows.length &&
    readyRows.every((row) => row.proofState === 'proof-ready' && row.missingArtifactCount === 0) &&
    storageReadyRows.every((row) => row.storageState === 'custody-ready' && row.storageMissingArtifactCount === 0)
  );
}

function missingArtifactCountsMatch(status: AgentNetworkLiveCaptureStatus): boolean {
  return (
    status.missingArtifactCount === status.rows.reduce((sum, row) => sum + row.missingArtifactCount, 0) &&
    status.storageMissingArtifactCount === status.rows.reduce((sum, row) => sum + row.storageMissingArtifactCount, 0) &&
    status.executionMissingArtifactCount ===
      status.rows.reduce((sum, row) => sum + row.executionMissingArtifactCount, 0)
  );
}

function proofStateCountsMatch(status: AgentNetworkLiveCaptureStatus): boolean {
  return (
    status.proofReadyCount === rowCount(status, 'proof-ready') &&
    status.manualRequiredCount === rowCount(status, 'manual-required') &&
    status.unavailableCount === rowCount(status, 'unavailable') &&
    status.degradedCount === rowCount(status, 'degraded')
  );
}

function storageStateCountsMatch(status: AgentNetworkLiveCaptureStatus): boolean {
  return (
    status.storageCustodyReadyCount === storageCount(status, 'custody-ready') &&
    status.storageManualRequiredCount === storageCount(status, 'manual-required') &&
    status.storageUnavailableCount === storageCount(status, 'unavailable') &&
    status.storageDegradedCount === storageCount(status, 'degraded')
  );
}

function executionStateCountsMatch(status: AgentNetworkLiveCaptureStatus): boolean {
  return (
    status.boundedExecutedCount === executionCount(status, 'bounded-executed') &&
    status.executionManualRequiredCount === executionCount(status, 'manual-required') &&
    status.executionUnavailableCount === executionCount(status, 'unavailable') &&
    status.executionDegradedCount === executionCount(status, 'degraded')
  );
}

function executionObservationCountsMatch(status: AgentNetworkLiveCaptureStatus): boolean {
  return (
    status.driverInvokedCount === booleanCount(status, (row) => row.driverInvoked) &&
    status.liveCaptureExecutedCount === booleanCount(status, (row) => row.liveCaptureExecuted) &&
    status.metadataSnapshotExecutedCount === booleanCount(status, (row) => row.metadataSnapshotExecuted) &&
    status.capturedPacketCount === status.rows.reduce((sum, row) => sum + row.capturedPacketCount, 0) &&
    status.rawArtifactCreatedCount === 0 &&
    status.rows.every((row) => !row.rawArtifactCreated)
  );
}

function rowCount(
  status: AgentNetworkLiveCaptureStatus,
  proofState: AgentNetworkLiveCaptureStatusRow['proofState']
): number {
  return status.rows.filter((row) => row.proofState === proofState).length;
}

function storageCount(
  status: AgentNetworkLiveCaptureStatus,
  storageState: AgentNetworkLiveCaptureStatusRow['storageState']
): number {
  return status.rows.filter((row) => row.storageState === storageState).length;
}

function executionCount(
  status: AgentNetworkLiveCaptureStatus,
  executionState: AgentNetworkLiveCaptureStatusRow['executionState']
): number {
  return status.rows.filter((row) => row.executionState === executionState).length;
}

function booleanCount(
  status: AgentNetworkLiveCaptureStatus,
  predicate: (row: AgentNetworkLiveCaptureStatusRow) => boolean
): number {
  return status.rows.filter(predicate).length;
}
