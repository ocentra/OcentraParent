import { describe, expect, it } from 'vitest';
import { AgentEvent } from '../../src/contracts';
import { AgentProtocolDefaults } from '../../src/defaults';
import {
  parseAgentNetworkLiveCaptureStatusEvent,
  type AgentNetworkLiveCaptureStatus,
  type AgentNetworkLiveCaptureStatusRow,
} from '../../src/network-live-capture-status';

const LiveCaptureRefs = AgentProtocolDefaults.NetworkLiveCaptureStatus;

const ReadyRow = row({
  platform: 'windows-npcap',
  captureProofRef: LiveCaptureRefs.WindowsProofRef,
  proofState: 'proof-ready',
  executionRef: LiveCaptureRefs.WindowsExecutionRef,
  executionState: 'bounded-executed',
  storageState: 'custody-ready',
  missingArtifactCount: 0,
  executionMissingArtifactCount: 0,
  storageMissingArtifactCount: 0,
  captureReady: true,
  rawArtifactStorageAuthorized: true,
  driverInvoked: true,
  liveCaptureExecuted: true,
  capturedPacketCount: 3,
});

const ManualRow = row(
  {
    platform: 'windows-npcap',
    captureProofRef: LiveCaptureRefs.ManualProofRef,
    proofState: 'manual-required',
    executionRef: LiveCaptureRefs.ManualExecutionRef,
    executionState: 'manual-required',
    storageState: 'manual-required',
    missingArtifactCount: 9,
    executionMissingArtifactCount: 10,
    storageMissingArtifactCount: 9,
    captureReady: false,
    rawArtifactStorageAuthorized: false,
    driverInvoked: false,
    liveCaptureExecuted: false,
    capturedPacketCount: 0,
  },
  'none'
);

const UnavailableRow = row(
  {
    platform: 'linux-libpcap',
    captureProofRef: LiveCaptureRefs.LinuxProofRef,
    proofState: 'unavailable',
    executionRef: LiveCaptureRefs.LinuxExecutionRef,
    executionState: 'unavailable',
    storageState: 'unavailable',
    missingArtifactCount: 9,
    executionMissingArtifactCount: 10,
    storageMissingArtifactCount: 9,
    captureReady: false,
    rawArtifactStorageAuthorized: false,
    driverInvoked: false,
    liveCaptureExecuted: false,
    capturedPacketCount: 0,
  },
  'none'
);

const DegradedRow = row(
  {
    platform: 'macos-bpf-libpcap',
    captureProofRef: LiveCaptureRefs.MacosProofRef,
    proofState: 'degraded',
    executionRef: LiveCaptureRefs.MacosExecutionRef,
    executionState: 'degraded',
    storageState: 'degraded',
    missingArtifactCount: 0,
    executionMissingArtifactCount: 10,
    storageMissingArtifactCount: 9,
    captureReady: false,
    rawArtifactStorageAuthorized: false,
    driverInvoked: false,
    liveCaptureExecuted: false,
    capturedPacketCount: 0,
  },
  'live-only'
);

const LiveCaptureStatus = {
  statusRef: LiveCaptureRefs.StatusRef,
  row13StatusRef: LiveCaptureRefs.Row13StatusRef,
  executionStatusRef: LiveCaptureRefs.ExecutionStatusRef,
  rawStorageStatusRef: LiveCaptureRefs.RawStorageStatusRef,
  platformRowCount: 4,
  proofReadyCount: 1,
  manualRequiredCount: 1,
  unavailableCount: 1,
  degradedCount: 1,
  requiredArtifactCount: 36,
  missingArtifactCount: 18,
  storageCustodyReadyCount: 1,
  storageManualRequiredCount: 1,
  storageUnavailableCount: 1,
  storageDegradedCount: 1,
  storageMissingArtifactCount: 27,
  boundedExecutedCount: 1,
  executionManualRequiredCount: 1,
  executionUnavailableCount: 1,
  executionDegradedCount: 1,
  executionMissingArtifactCount: 30,
  metadataSnapshotExecutedCount: 0,
  capturedPacketCount: 3,
  rawArtifactCreatedCount: 0,
  captureReadyCount: 1,
  rawArtifactStorageAuthorizedCount: 1,
  driverInvokedCount: 1,
  liveCaptureExecutedCount: 1,
  remoteUploadEnabledCount: 0,
  rawPcapWithoutCustodyAvailableCount: 0,
  exactUrlAvailableCount: 0,
  decryptedPayloadAvailableCount: 0,
  pageContentAvailableCount: 0,
  privateMessageAvailableCount: 0,
  searchQueryAvailableCount: 0,
  policyAuthorityCount: 0,
  adapterAuthorityCount: 0,
  enforcementCommandEventCount: 0,
  netstatMetadataSubstitutionCount: 0,
  hostFilteringClaimCount: 0,
  rows: [ReadyRow, ManualRow, UnavailableRow, DegradedRow],
} satisfies AgentNetworkLiveCaptureStatus;

describe('network live capture status', () => {
  registerParsingTests();
  registerNoClaimRejectionTests();
  registerMalformedPayloadTests();
  registerRefDriftTests();
  registerCountAndReadinessTests();
});

function registerParsingTests() {
  it('parses row13 live-capture service status with row03a custody readiness from a typed event', () => {
    const parsed = parseAgentNetworkLiveCaptureStatusEvent(
      eventWithPayload({
        [AgentProtocolDefaults.Field.NetworkLiveCaptureStatus]: JSON.stringify(LiveCaptureStatus),
      })
    );

    expect(parsed).toEqual({ ok: true, status: LiveCaptureStatus });
  });
}

function registerNoClaimRejectionTests() {
  it('rejects raw artifact, content, remote upload, adapter, and enforcement claims', () => {
    expectInvalid({ ...LiveCaptureStatus, driverInvokedCount: 2 });
    expectInvalid({ ...LiveCaptureStatus, liveCaptureExecutedCount: 2 });
    expectInvalid({ ...LiveCaptureStatus, remoteUploadEnabledCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, rawArtifactCreatedCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, exactUrlAvailableCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, decryptedPayloadAvailableCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, policyAuthorityCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, adapterAuthorityCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, enforcementCommandEventCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, netstatMetadataSubstitutionCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, hostFilteringClaimCount: 1 });
    expectInvalidPatch({
      rows: [ReadyRow, { ...ManualRow, driverInvoked: true }, UnavailableRow, DegradedRow],
    });
    expectInvalid({
      ...LiveCaptureStatus,
      rows: [{ ...ReadyRow, rawArtifactCreated: true }, ManualRow, UnavailableRow, DegradedRow],
    });
    expectInvalidPatch({
      rows: [{ ...ReadyRow, netstatMetadataSubstitutedForLiveCapture: true }, ManualRow, UnavailableRow, DegradedRow],
    });
  });
}

function registerMalformedPayloadTests() {
  it('rejects missing fields and malformed JSON', () => {
    expect(parseAgentNetworkLiveCaptureStatusEvent(eventWithPayload({}))).toEqual({
      ok: false,
      reason: 'missing-live-capture-status',
    });
    expect(
      parseAgentNetworkLiveCaptureStatusEvent(
        eventWithPayload({ [AgentProtocolDefaults.Field.NetworkLiveCaptureStatus]: '{' })
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-live-capture-status-json',
    });
  });
}

function registerRefDriftTests() {
  it('rejects stale refs and row identity drift', () => {
    expectInvalidPatch({ statusRef: 'network.live-capture.status.12' });
    expectInvalidPatch({ row13StatusRef: 'network.live-capture.proof-gate.12' });
    expectInvalidPatch({ executionStatusRef: 'network.live-capture.execution-status.12' });
    expectInvalidPatch({ rawStorageStatusRef: 'network.live-capture.raw-storage-custody.02a' });
    expectInvalidPatch({
      rows: [
        { ...ReadyRow, captureProofRef: 'network.live-capture.windows-npcap.12' },
        ManualRow,
        UnavailableRow,
        DegradedRow,
      ],
    });
    expectInvalidPatch({
      rows: [
        { ...ReadyRow, storageProofRef: 'network.live-capture.raw-storage-custody.02a' },
        ManualRow,
        UnavailableRow,
        DegradedRow,
      ],
    });
    expectInvalidPatch({
      rows: [
        { ...ReadyRow, executionRef: 'network.live-capture.execution.windows-npcap.12' },
        ManualRow,
        UnavailableRow,
        DegradedRow,
      ],
    });
    expectInvalidPatch({
      rows: [
        { ...ReadyRow, interfaceRef: 'network.live-capture.interface.12' },
        ManualRow,
        UnavailableRow,
        DegradedRow,
      ],
    });
    expectInvalidPatch({
      rows: [ReadyRow, { ...ManualRow, interfaceRef: LiveCaptureRefs.InterfaceRef }, UnavailableRow, DegradedRow],
    });
    expectInvalidPatch({
      rows: [ReadyRow, ManualRow, { ...UnavailableRow, platform: 'windows-npcap' }, DegradedRow],
    });
  });
}

function registerCountAndReadinessTests() {
  it('rejects row count, readiness, and missing-artifact mismatches', () => {
    expectInvalidPatch({ platformRowCount: 3 });
    expectInvalidPatch({ proofReadyCount: 2 });
    expectInvalidPatch({ storageCustodyReadyCount: 2 });
    expectInvalidPatch({ requiredArtifactCount: 27 });
    expectInvalidPatch({ missingArtifactCount: 17 });
    expectInvalidPatch({ storageMissingArtifactCount: 26 });
    expectInvalidPatch({ boundedExecutedCount: 2 });
    expectInvalidPatch({ executionManualRequiredCount: 2 });
    expectInvalidPatch({ executionMissingArtifactCount: 29 });
    expectInvalidPatch({ capturedPacketCount: 2 });
    expectInvalidPatch({ captureReadyCount: 2 });
    expectInvalidPatch({ rawArtifactStorageAuthorizedCount: 2 });
    expectInvalidPatch({
      rows: [{ ...ReadyRow, captureReady: false }, ManualRow, UnavailableRow, DegradedRow],
    });
    expectInvalidPatch({
      rows: [{ ...ReadyRow, rawArtifactStorageAuthorized: false }, ManualRow, UnavailableRow, DegradedRow],
    });
    expectInvalidPatch({
      rows: [{ ...ReadyRow, missingArtifactCount: 1 }, ManualRow, UnavailableRow, DegradedRow],
    });
  });
}

function row(
  value: Pick<
    AgentNetworkLiveCaptureStatusRow,
    | 'platform'
    | 'captureProofRef'
    | 'proofState'
    | 'executionRef'
    | 'executionState'
    | 'storageState'
    | 'missingArtifactCount'
    | 'executionMissingArtifactCount'
    | 'storageMissingArtifactCount'
    | 'captureReady'
    | 'rawArtifactStorageAuthorized'
    | 'driverInvoked'
    | 'liveCaptureExecuted'
    | 'capturedPacketCount'
  >,
  refState: 'complete' | 'live-only' | 'none' = 'complete'
): AgentNetworkLiveCaptureStatusRow {
  return {
    ...value,
    storageProofRef: LiveCaptureRefs.RawStorageStatusRef,
    ...liveRefs(refState),
    ...storageRefs(refState),
    ...executionRefs(value.executionState === 'bounded-executed' ? 'complete' : 'none'),
    remoteUploadEnabled: false,
    rawPcapWithoutCustodyAvailable: false,
    exactUrlAvailable: false,
    decryptedPayloadAvailable: false,
    pageContentAvailable: false,
    privateMessageAvailable: false,
    searchQueryAvailable: false,
    policyAuthority: false,
    adapterAuthority: false,
    enforcementCommandsPublished: 0,
    metadataSnapshotExecuted: false,
    rawArtifactCreated: false,
    netstatMetadataSubstitutedForLiveCapture: false,
    hostFilteringClaimed: false,
  };
}

function executionRefs(refState: 'complete' | 'none') {
  if (refState === 'none') {
    return {
      driverInvocationRef: null,
      interfaceObservationRef: null,
      executionPermissionRef: null,
      boundedWindowRef: null,
      executionCleanStopRef: null,
      executionCustodyRef: null,
      executionRetentionDeleteExportRef: null,
      metadataOnlySanitizationRef: null,
      executionPrivateTrafficExclusionRef: null,
    };
  }
  return {
    driverInvocationRef: LiveCaptureRefs.DriverInvocationRef,
    interfaceObservationRef: LiveCaptureRefs.InterfaceObservationRef,
    executionPermissionRef: LiveCaptureRefs.ExecutionPermissionRef,
    boundedWindowRef: LiveCaptureRefs.BoundedWindowRef,
    executionCleanStopRef: LiveCaptureRefs.ExecutionCleanStopRef,
    executionCustodyRef: LiveCaptureRefs.ExecutionCustodyRef,
    executionRetentionDeleteExportRef: LiveCaptureRefs.ExecutionRetentionRef,
    metadataOnlySanitizationRef: LiveCaptureRefs.MetadataSanitizationRef,
    executionPrivateTrafficExclusionRef: LiveCaptureRefs.ExecutionPrivateTrafficExclusionRef,
  };
}

function liveRefs(refState: 'complete' | 'live-only' | 'none') {
  if (refState === 'none') {
    return {
      interfaceRef: null,
      driverProofRef: null,
      permissionProofRef: null,
      boundedCaptureRef: null,
      cleanStopRef: null,
      quotaRotationRef: null,
      retentionDeleteExportRef: null,
      custodyRef: null,
      privateTrafficExclusionRef: null,
    };
  }
  return {
    interfaceRef: LiveCaptureRefs.InterfaceRef,
    driverProofRef: LiveCaptureRefs.DriverRef,
    permissionProofRef: LiveCaptureRefs.PermissionRef,
    boundedCaptureRef: LiveCaptureRefs.BoundedCaptureRef,
    cleanStopRef: LiveCaptureRefs.CleanStopRef,
    quotaRotationRef: LiveCaptureRefs.QuotaRef,
    retentionDeleteExportRef: LiveCaptureRefs.RetentionRef,
    custodyRef: LiveCaptureRefs.CustodyRef,
    privateTrafficExclusionRef: LiveCaptureRefs.PrivateTrafficExclusionRef,
  };
}

function storageRefs(refState: 'complete' | 'live-only' | 'none') {
  if (refState !== 'complete') {
    return {
      rawArtifactManifestRef: null,
      storageLocationRef: null,
      encryptionAtRestRef: null,
      storageQuotaRotationRef: null,
      retentionPolicyRef: null,
      storageDeleteExportRef: null,
      custodyChainRef: null,
      storagePrivateTrafficExclusionRef: null,
    };
  }
  return {
    rawArtifactManifestRef: LiveCaptureRefs.RawManifestRef,
    storageLocationRef: LiveCaptureRefs.RawStorageLocationRef,
    encryptionAtRestRef: LiveCaptureRefs.RawEncryptionRef,
    storageQuotaRotationRef: LiveCaptureRefs.RawQuotaRef,
    retentionPolicyRef: LiveCaptureRefs.RawRetentionRef,
    storageDeleteExportRef: LiveCaptureRefs.RawDeleteExportRef,
    custodyChainRef: LiveCaptureRefs.RawCustodyChainRef,
    storagePrivateTrafficExclusionRef: LiveCaptureRefs.RawPrivateTrafficExclusionRef,
  };
}

function expectInvalidPatch(patch: Partial<AgentNetworkLiveCaptureStatus>) {
  expectInvalid({ ...LiveCaptureStatus, ...patch });
}

function expectInvalid(value: unknown) {
  expect(
    parseAgentNetworkLiveCaptureStatusEvent(
      eventWithPayload({ [AgentProtocolDefaults.Field.NetworkLiveCaptureStatus]: JSON.stringify(value) })
    )
  ).toEqual({ ok: false, reason: 'invalid-live-capture-status' });
}

function eventWithPayload(payload: Record<string, unknown>) {
  return {
    schemaVersion: 1,
    eventId: 'network-live-capture-status-reported',
    correlationId: 'cmd-network-live-capture-status',
    sentAt: '2026-06-08T06:45:00Z',
    source: { peerId: 'local-dev-agent', role: 'agent-service' },
    target: { peerId: 'portal-dev', role: 'portal' },
    event: AgentEvent.NetworkLiveCaptureStatusReported,
    severity: 'info',
    payload,
    snapshot: null,
  } as const;
}
