import { describe, expect, it } from 'vitest';
import {
  ScreenLiveViewPermissionGateSchemaVersion,
  ScreenLiveViewPlatformPermissionGateSchema,
  ScreenLiveViewProductionReadinessEvidenceSchema,
  ScreenLiveViewProductionReadinessEvidenceSchemaVersion,
} from '../src/screen-live-view-platform-permission';

const CheckedAt = '2026-06-07T04:03:00Z';

const DisabledGate = {
  schemaVersion: ScreenLiveViewPermissionGateSchemaVersion,
  checkedAt: CheckedAt,
  platform: 'windows',
  liveViewMode: 'disabled',
  transportMode: 'none',
  permissionEvidenceKind: 'missing',
  platformProofState: 'notRequired',
  platformProofRef: null,
  viewerAuditRef: null,
  sourceLabel: 'unavailable',
  custodyState: 'unavailable',
  frameRetentionBehavior: 'noFrameRetention',
  liveTransportProofRef: null,
  explicitViewerDisclosure: false,
  cacheRawFrames: false,
  sessionRecordingAllowed: false,
  remoteInputControlAllowed: false,
  productLiveViewReady: false,
  reason: 'live view is disabled by default',
} as const;

const AndroidCaptureOnlyGate = {
  ...DisabledGate,
  platform: 'android-mediaprojection',
  liveViewMode: 'lanOnlyView',
  transportMode: 'lanMutualAuth',
  permissionEvidenceKind: 'screen-capture-only',
  platformProofState: 'operatorVerified',
  platformProofRef: 'screen-plan-android-mediaprojection-capture-proof',
  viewerAuditRef: 'screen-live-view-viewer-audit',
  sourceLabel: 'liveView',
  custodyState: 'live-lan-child-agent',
  liveTransportProofRef: 'screen-live-view-lan-transport-proof',
  explicitViewerDisclosure: true,
  reason: 'Android MediaProjection capture proof exists but live-view permission prompt proof is not present',
} as const;

const LanLiveViewReadyGate = {
  ...AndroidCaptureOnlyGate,
  permissionEvidenceKind: 'live-view-permission',
  platformProofRef: 'screen-live-view-android-mediaprojection-permission-proof',
  productLiveViewReady: true,
  reason: 'parent-approved LAN live view has platform live-view permission and transport proof',
} as const;

const RelayLiveViewReadyGate = {
  ...LanLiveViewReadyGate,
  liveViewMode: 'relayBackedView',
  transportMode: 'relayEndToEndEncrypted',
  sourceLabel: 'relay',
  custodyState: 'ocentra-hosted-non-activity',
  liveTransportProofRef: 'screen-live-view-relay-transport-proof',
  reason: 'parent-approved relay live view has platform live-view permission and transport proof',
} as const;

const LanProductionReadinessEvidence = {
  schemaVersion: ScreenLiveViewProductionReadinessEvidenceSchemaVersion,
  checkedAt: CheckedAt,
  permissionGate: LanLiveViewReadyGate,
  promptArtifact: {
    platform: 'android-mediaprojection',
    artifactKind: 'platform-permission-prompt-screenshot',
    artifactRef: 'screen-live-view-android-mediaprojection-permission-proof',
    artifactDigest: 'sha256-live-view-platform-prompt',
    capturedAt: CheckedAt,
    operatorAuditRef: 'screen-live-view-viewer-audit',
    permissionEvidenceKind: 'live-view-permission',
    rawFrameIncluded: false,
    containsUserPrivateContent: false,
  },
  liveTransportProofRef: 'screen-live-view-lan-transport-proof',
  physicalDeviceParityProofRef: 'screen-live-view-android-physical-parity-proof',
  privacyLegalApprovalRef: 'screen-live-view-privacy-legal-approval',
  productionWorkerStartProofRef: 'screen-live-view-production-worker-start-proof',
  relayCacheExecutionProofRef: null,
  productLiveViewReady: true,
} as const;

const RelayProductionReadinessEvidence = {
  ...LanProductionReadinessEvidence,
  permissionGate: RelayLiveViewReadyGate,
  liveTransportProofRef: 'screen-live-view-relay-transport-proof',
  relayCacheExecutionProofRef: 'screen-live-view-relay-cache-execution-proof',
} as const;

describe('screen live-view platform permission gate', () => {
  it('accepts disabled live view and ready LAN or relay live-view gates', () => {
    const disabled = ScreenLiveViewPlatformPermissionGateSchema.parse(DisabledGate);
    const lanReady = ScreenLiveViewPlatformPermissionGateSchema.parse(LanLiveViewReadyGate);
    const relayReady = ScreenLiveViewPlatformPermissionGateSchema.parse(RelayLiveViewReadyGate);

    expect(disabled.productLiveViewReady).toBe(false);
    expect(lanReady.productLiveViewReady).toBe(true);
    expect(lanReady.permissionEvidenceKind).toBe('live-view-permission');
    expect(relayReady.transportMode).toBe('relayEndToEndEncrypted');
  });

  it('keeps capture-only platform proof in a not-ready live-view state', () => {
    const captureOnly = ScreenLiveViewPlatformPermissionGateSchema.parse(AndroidCaptureOnlyGate);

    expect(captureOnly.productLiveViewReady).toBe(false);
    expect(captureOnly.permissionEvidenceKind).toBe('screen-capture-only');
    expect(captureOnly.platformProofRef).toBe('screen-plan-android-mediaprojection-capture-proof');
  });

  it('rejects live-view readiness without live permission, audit, transport, no-retention, and no-remote-input gates', () => {
    const captureProofOverclaim = ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...AndroidCaptureOnlyGate,
      productLiveViewReady: true,
    });
    const missingAudit = ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...LanLiveViewReadyGate,
      viewerAuditRef: null,
    });
    const missingTransportProof = ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...LanLiveViewReadyGate,
      liveTransportProofRef: null,
    });
    const retainedFrames = ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...LanLiveViewReadyGate,
      cacheRawFrames: true,
    });
    const remoteInput = ScreenLiveViewPlatformPermissionGateSchema.safeParse({
      ...LanLiveViewReadyGate,
      remoteInputControlAllowed: true,
    });

    expect(captureProofOverclaim.success).toBe(false);
    expect(missingAudit.success).toBe(false);
    expect(missingTransportProof.success).toBe(false);
    expect(retainedFrames.success).toBe(false);
    expect(remoteInput.success).toBe(false);
  });
});

describe('screen live-view production readiness evidence', () => {
  it('accepts readiness only with prompt, transport, parity, worker, and approval evidence', () => {
    const lanEvidence = ScreenLiveViewProductionReadinessEvidenceSchema.parse(LanProductionReadinessEvidence);
    const relayEvidence = ScreenLiveViewProductionReadinessEvidenceSchema.parse(RelayProductionReadinessEvidence);

    expect(lanEvidence.productLiveViewReady).toBe(true);
    expect(lanEvidence.promptArtifact.artifactRef).toBe(lanEvidence.permissionGate.platformProofRef);
    expect(lanEvidence.liveTransportProofRef).toBe(lanEvidence.permissionGate.liveTransportProofRef);
    expect(relayEvidence.relayCacheExecutionProofRef).toBe('screen-live-view-relay-cache-execution-proof');
  });

  it('rejects readiness when proof artifacts do not match the ready gate', () => {
    const captureOnlyGate = ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...LanProductionReadinessEvidence,
      permissionGate: AndroidCaptureOnlyGate,
    });
    const mismatchedPrompt = ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...LanProductionReadinessEvidence,
      promptArtifact: {
        ...LanProductionReadinessEvidence.promptArtifact,
        artifactRef: 'screen-live-view-other-platform-prompt-proof',
      },
    });
    const mismatchedTransport = ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...LanProductionReadinessEvidence,
      liveTransportProofRef: 'screen-live-view-other-transport-proof',
    });
    const relayWithoutCache = ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...RelayProductionReadinessEvidence,
      relayCacheExecutionProofRef: null,
    });
    const promptIncludesRawFrame = ScreenLiveViewProductionReadinessEvidenceSchema.safeParse({
      ...LanProductionReadinessEvidence,
      promptArtifact: {
        ...LanProductionReadinessEvidence.promptArtifact,
        rawFrameIncluded: true,
      },
    });

    expect(captureOnlyGate.success).toBe(false);
    expect(mismatchedPrompt.success).toBe(false);
    expect(mismatchedTransport.success).toBe(false);
    expect(relayWithoutCache.success).toBe(false);
    expect(promptIncludesRawFrame.success).toBe(false);
  });
});
