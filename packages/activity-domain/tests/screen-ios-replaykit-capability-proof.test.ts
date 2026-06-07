import { describe, expect, it } from 'vitest';
import {
  screenIosReplayKitCapabilityProof,
  ScreenIosReplayKitCapabilityRowSchema,
} from '../src/screen-ios-replaykit-capability-proof';

const CheckedAt = '2026-06-07T15:55:00Z';

describe('screen iOS ReplayKit capability proof', () => {
  it('records in-app and broadcast paths as manual-required before physical device proof', () => {
    const proof = screenIosReplayKitCapabilityProof(CheckedAt);

    expect(proof.productIosCaptureReady).toBe(false);
    expect(proof.rows).toHaveLength(3);
    expect(proof.rows.map((row) => row.mode)).toEqual([
      'inAppReplayKitSession',
      'broadcastUploadExtension',
      'notClaimed',
    ]);
    expect(proof.rows.every((row) => !row.rawFrameRemoteUploadAllowed)).toBe(true);
    expect(proof.rows.every((row) => !row.rawFrameRetentionDefault)).toBe(true);
    expect(proof.rows.every((row) => !row.arbitraryBackgroundOtherAppCaptureClaimed)).toBe(true);
  });

  it('requires physical device and deletion proof before iOS ReplayKit product readiness', () => {
    const ready = ScreenIosReplayKitCapabilityRowSchema.parse({
      ...screenIosReplayKitCapabilityProof(CheckedAt).rows[0],
      captureState: 'ready',
      proofState: 'physicalDeviceVerified',
      physicalDeviceProofRef: 'screen-ios-replaykit-physical-device-proof',
      deletionProofRef: 'screen-ios-replaykit-deletion-proof',
      productCaptureReady: true,
      reason: 'physical iOS ReplayKit session captured pixels and deleted raw frames',
    });

    expect(ready.productCaptureReady).toBe(true);
    expect(ready.physicalDeviceProofRef).toBe('screen-ios-replaykit-physical-device-proof');
  });

  it('rejects iOS overclaims without physical proof, deletion proof, explicit session, or safe custody', () => {
    const base = screenIosReplayKitCapabilityProof(CheckedAt).rows[0];
    const withoutPhysicalProof = ScreenIosReplayKitCapabilityRowSchema.safeParse({
      ...base,
      captureState: 'ready',
      proofState: 'physicalDeviceVerified',
      deletionProofRef: 'screen-ios-replaykit-deletion-proof',
      productCaptureReady: true,
    });
    const withoutDeletionProof = ScreenIosReplayKitCapabilityRowSchema.safeParse({
      ...base,
      captureState: 'ready',
      proofState: 'physicalDeviceVerified',
      physicalDeviceProofRef: 'screen-ios-replaykit-physical-device-proof',
      productCaptureReady: true,
    });
    const silentBackgroundCapture = ScreenIosReplayKitCapabilityRowSchema.safeParse({
      ...base,
      arbitraryBackgroundOtherAppCaptureClaimed: true,
    });
    const remoteRawUpload = ScreenIosReplayKitCapabilityRowSchema.safeParse({
      ...base,
      rawFrameRemoteUploadAllowed: true,
    });
    const missingExplicitUserStart = ScreenIosReplayKitCapabilityRowSchema.safeParse({
      ...base,
      requiresExplicitUserStart: false,
    });

    expect(withoutPhysicalProof.success).toBe(false);
    expect(withoutDeletionProof.success).toBe(false);
    expect(silentBackgroundCapture.success).toBe(false);
    expect(remoteRawUpload.success).toBe(false);
    expect(missingExplicitUserStart.success).toBe(false);
  });
});
