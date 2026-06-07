import { describe, expect, it } from 'vitest';
import {
  ScreenLiveViewPermissionGateSchemaVersion,
  ScreenLiveViewPlatformPermissionGateSchema,
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
