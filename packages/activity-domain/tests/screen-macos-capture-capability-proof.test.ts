import { describe, expect, it } from 'vitest';
import {
  screenMacosCaptureCapabilityProof,
  ScreenMacosCaptureCapabilityRowSchema,
} from '../src/screen-macos-capture-capability-proof';

const CheckedAt = '2026-06-07T16:50:00Z';

describe('screen macOS capture capability proof', () => {
  it('records ScreenCaptureKit display and window capture as manual-required before live Mac proof', () => {
    const proof = screenMacosCaptureCapabilityProof(CheckedAt);

    expect(proof.productMacosCaptureReady).toBe(false);
    expect(proof.rows.map((row) => row.mode)).toEqual([
      'screenCaptureKitDisplay',
      'screenCaptureKitWindow',
      'screenRecordingPermission',
      'pppcMdmManaged',
    ]);
    expect(proof.rows.every((row) => !row.rawFrameRemoteUploadAllowed)).toBe(true);
    expect(proof.rows.every((row) => !row.rawFrameRetentionDefault)).toBe(true);
    expect(proof.rows.every((row) => !row.silentBackgroundCaptureClaimed)).toBe(true);
  });

  it('requires live session, permission, and deletion proof before product readiness', () => {
    const ready = ScreenMacosCaptureCapabilityRowSchema.parse({
      ...screenMacosCaptureCapabilityProof(CheckedAt).rows[0],
      captureState: 'ready',
      proofState: 'liveSessionVerified',
      liveSessionProofRef: 'screen-macos-live-display-proof',
      permissionProofRef: 'screen-macos-screen-recording-permission-proof',
      deletionProofRef: 'screen-macos-deletion-proof',
      productMacosCaptureReady: true,
      reason: 'macOS ScreenCaptureKit display capture captured pixels and deleted raw frames',
    });

    expect(ready.productMacosCaptureReady).toBe(true);
    expect(ready.permissionProofRef).toBe('screen-macos-screen-recording-permission-proof');
  });

  it('rejects macOS capture overclaims without proof, permission, scoped filter, or safe custody', () => {
    const base = screenMacosCaptureCapabilityProof(CheckedAt).rows[0];
    const withoutLiveProof = ScreenMacosCaptureCapabilityRowSchema.safeParse({
      ...base,
      captureState: 'ready',
      proofState: 'liveSessionVerified',
      permissionProofRef: 'screen-macos-screen-recording-permission-proof',
      deletionProofRef: 'screen-macos-deletion-proof',
      productMacosCaptureReady: true,
    });
    const withoutPermissionProof = ScreenMacosCaptureCapabilityRowSchema.safeParse({
      ...base,
      captureState: 'ready',
      proofState: 'liveSessionVerified',
      liveSessionProofRef: 'screen-macos-live-display-proof',
      deletionProofRef: 'screen-macos-deletion-proof',
      productMacosCaptureReady: true,
    });
    const missingContentFilter = ScreenMacosCaptureCapabilityRowSchema.safeParse({
      ...base,
      requiresScreenCaptureKitContentFilter: false,
    });
    const silentBackgroundCapture = ScreenMacosCaptureCapabilityRowSchema.safeParse({
      ...base,
      silentBackgroundCaptureClaimed: true,
    });
    const remoteRawUpload = ScreenMacosCaptureCapabilityRowSchema.safeParse({
      ...base,
      rawFrameRemoteUploadAllowed: true,
    });

    expect(withoutLiveProof.success).toBe(false);
    expect(withoutPermissionProof.success).toBe(false);
    expect(missingContentFilter.success).toBe(false);
    expect(silentBackgroundCapture.success).toBe(false);
    expect(remoteRawUpload.success).toBe(false);
  });
});
