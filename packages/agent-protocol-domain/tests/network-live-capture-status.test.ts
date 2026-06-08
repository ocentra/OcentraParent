import { describe, expect, it } from 'vitest';
import { AgentEvent } from '../src/contracts';
import { AgentProtocolDefaults } from '../src/defaults';
import {
  parseAgentNetworkLiveCaptureStatusEvent,
  type AgentNetworkLiveCaptureStatus,
  type AgentNetworkLiveCaptureStatusRow,
} from '../src/network-live-capture-status';

const LiveCaptureRefs = AgentProtocolDefaults.NetworkLiveCaptureStatus;

const ReadyRow = row({
  platform: 'windows-npcap',
  captureProofRef: LiveCaptureRefs.WindowsProofRef,
  proofState: 'proof-ready',
  storageState: 'custody-ready',
  missingArtifactCount: 0,
  storageMissingArtifactCount: 0,
  captureReady: true,
  rawArtifactStorageAuthorized: true,
});

const ManualRow = row(
  {
    platform: 'windows-npcap',
    captureProofRef: LiveCaptureRefs.ManualProofRef,
    proofState: 'manual-required',
    storageState: 'manual-required',
    missingArtifactCount: 9,
    storageMissingArtifactCount: 9,
    captureReady: false,
    rawArtifactStorageAuthorized: false,
  },
  'none'
);

const UnavailableRow = row(
  {
    platform: 'linux-libpcap',
    captureProofRef: LiveCaptureRefs.LinuxProofRef,
    proofState: 'unavailable',
    storageState: 'unavailable',
    missingArtifactCount: 9,
    storageMissingArtifactCount: 9,
    captureReady: false,
    rawArtifactStorageAuthorized: false,
  },
  'none'
);

const DegradedRow = row(
  {
    platform: 'macos-bpf-libpcap',
    captureProofRef: LiveCaptureRefs.MacosProofRef,
    proofState: 'degraded',
    storageState: 'degraded',
    missingArtifactCount: 0,
    storageMissingArtifactCount: 9,
    captureReady: false,
    rawArtifactStorageAuthorized: false,
  },
  'live-only'
);

const LiveCaptureStatus = {
  statusRef: LiveCaptureRefs.StatusRef,
  row13StatusRef: LiveCaptureRefs.Row13StatusRef,
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
  captureReadyCount: 1,
  rawArtifactStorageAuthorizedCount: 1,
  driverInvokedCount: 0,
  liveCaptureExecutedCount: 0,
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
  it('rejects live driver, content, remote upload, adapter, and enforcement claims', () => {
    expectInvalid({ ...LiveCaptureStatus, driverInvokedCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, liveCaptureExecutedCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, remoteUploadEnabledCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, exactUrlAvailableCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, decryptedPayloadAvailableCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, policyAuthorityCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, adapterAuthorityCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, enforcementCommandEventCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, netstatMetadataSubstitutionCount: 1 });
    expectInvalid({ ...LiveCaptureStatus, hostFilteringClaimCount: 1 });
    expectInvalidPatch({
      rows: [{ ...ReadyRow, driverInvoked: true }, ManualRow, UnavailableRow, DegradedRow],
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
    | 'storageState'
    | 'missingArtifactCount'
    | 'storageMissingArtifactCount'
    | 'captureReady'
    | 'rawArtifactStorageAuthorized'
  >,
  refState: 'complete' | 'live-only' | 'none' = 'complete'
): AgentNetworkLiveCaptureStatusRow {
  const hasLiveRefs = refState === 'complete' || refState === 'live-only';
  const hasStorageRefs = refState === 'complete';
  return {
    ...value,
    storageProofRef: LiveCaptureRefs.RawStorageStatusRef,
    interfaceRef: hasLiveRefs ? LiveCaptureRefs.InterfaceRef : null,
    driverProofRef: hasLiveRefs ? LiveCaptureRefs.DriverRef : null,
    permissionProofRef: hasLiveRefs ? LiveCaptureRefs.PermissionRef : null,
    boundedCaptureRef: hasLiveRefs ? LiveCaptureRefs.BoundedCaptureRef : null,
    cleanStopRef: hasLiveRefs ? LiveCaptureRefs.CleanStopRef : null,
    quotaRotationRef: hasLiveRefs ? LiveCaptureRefs.QuotaRef : null,
    retentionDeleteExportRef: hasLiveRefs ? LiveCaptureRefs.RetentionRef : null,
    custodyRef: hasLiveRefs ? LiveCaptureRefs.CustodyRef : null,
    privateTrafficExclusionRef: hasLiveRefs ? LiveCaptureRefs.PrivateTrafficExclusionRef : null,
    rawArtifactManifestRef: hasStorageRefs ? LiveCaptureRefs.RawManifestRef : null,
    storageLocationRef: hasStorageRefs ? LiveCaptureRefs.RawStorageLocationRef : null,
    encryptionAtRestRef: hasStorageRefs ? LiveCaptureRefs.RawEncryptionRef : null,
    storageQuotaRotationRef: hasStorageRefs ? LiveCaptureRefs.RawQuotaRef : null,
    retentionPolicyRef: hasStorageRefs ? LiveCaptureRefs.RawRetentionRef : null,
    storageDeleteExportRef: hasStorageRefs ? LiveCaptureRefs.RawDeleteExportRef : null,
    custodyChainRef: hasStorageRefs ? LiveCaptureRefs.RawCustodyChainRef : null,
    storagePrivateTrafficExclusionRef: hasStorageRefs ? LiveCaptureRefs.RawPrivateTrafficExclusionRef : null,
    driverInvoked: false,
    liveCaptureExecuted: false,
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
    netstatMetadataSubstitutedForLiveCapture: false,
    hostFilteringClaimed: false,
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
