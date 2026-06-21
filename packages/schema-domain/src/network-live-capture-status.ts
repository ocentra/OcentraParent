import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const NullableNonEmptyStringSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const NetworkLiveCaptureCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

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

export const AgentNetworkLiveCaptureStatusRowSchema = withParser(
  Schema.Struct({
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
  })
);

export const AgentNetworkLiveCaptureStatusSchema = withParser(
  Schema.Struct({
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
  })
);

export type AgentNetworkLiveCaptureStatusRow = Infer<typeof AgentNetworkLiveCaptureStatusRowSchema>;
export type AgentNetworkLiveCaptureStatus = Infer<typeof AgentNetworkLiveCaptureStatusSchema>;
