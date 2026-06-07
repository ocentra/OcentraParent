import { describe, expect, it } from 'vitest';
import {
  screenAndroidMediaProjectionCapabilityProof,
  ScreenAndroidMediaProjectionCapabilityRowSchema,
} from '../src/screen-android-mediaprojection-capability-proof';

const CheckedAt = '2026-06-07T16:05:00Z';

describe('screen Android MediaProjection capability proof', () => {
  it('keeps emulator proof separate from physical Android product readiness', () => {
    const proof = screenAndroidMediaProjectionCapabilityProof(CheckedAt);

    expect(proof.emulatorCaptureProved).toBe(true);
    expect(proof.productAndroidCaptureReady).toBe(false);
    expect(proof.rows.map((row) => row.mode)).toEqual([
      'emulatorMediaProjection',
      'physicalDeviceMediaProjection',
      'android14AppWindowSharing',
      'notClaimed',
    ]);
    expect(proof.rows.slice(0, 3).every((row) => row.requiresStopCallbackOnUserStop)).toBe(true);
    expect(proof.rows[3]?.requiresStopCallbackOnUserStop).toBe(false);
    expect(proof.rows.every((row) => !row.silentBackgroundCaptureClaimed)).toBe(true);
    expect(proof.rows.every((row) => !row.rawFrameRemoteUploadAllowed)).toBe(true);
  });

  it('allows physical Android readiness only with physical and deletion proof', () => {
    const physical = screenAndroidMediaProjectionCapabilityProof(CheckedAt).rows[1];
    const ready = ScreenAndroidMediaProjectionCapabilityRowSchema.parse({
      ...physical,
      captureState: 'ready',
      proofState: 'physicalDeviceVerified',
      physicalDeviceProofRef: 'screen-android-physical-mediaprojection-proof',
      deletionProofRef: 'screen-android-physical-mediaprojection-deletion-proof',
      productAndroidCaptureReady: true,
    });

    expect(ready.productAndroidCaptureReady).toBe(true);
    expect(ready.requiresUserConsentPerSession).toBe(true);
    expect(ready.requiresForegroundServiceType).toBe(true);
    expect(ready.requiresStopCallbackOnUserStop).toBe(true);
  });

  it('rejects silent background, raw upload, missing stop callback, and physical readiness overclaims', () => {
    const physical = screenAndroidMediaProjectionCapabilityProof(CheckedAt).rows[1];
    const withoutPhysicalProof = ScreenAndroidMediaProjectionCapabilityRowSchema.safeParse({
      ...physical,
      captureState: 'ready',
      proofState: 'physicalDeviceVerified',
      deletionProofRef: 'screen-android-physical-mediaprojection-deletion-proof',
      productAndroidCaptureReady: true,
    });
    const silentBackgroundCapture = ScreenAndroidMediaProjectionCapabilityRowSchema.safeParse({
      ...physical,
      silentBackgroundCaptureClaimed: true,
    });
    const remoteRawUpload = ScreenAndroidMediaProjectionCapabilityRowSchema.safeParse({
      ...physical,
      rawFrameRemoteUploadAllowed: true,
    });
    const missingConsent = ScreenAndroidMediaProjectionCapabilityRowSchema.safeParse({
      ...physical,
      requiresUserConsentPerSession: false,
    });
    const missingStopCallback = ScreenAndroidMediaProjectionCapabilityRowSchema.safeParse({
      ...physical,
      requiresStopCallbackOnUserStop: false,
    });

    expect(withoutPhysicalProof.success).toBe(false);
    expect(silentBackgroundCapture.success).toBe(false);
    expect(remoteRawUpload.success).toBe(false);
    expect(missingConsent.success).toBe(false);
    expect(missingStopCallback.success).toBe(false);
  });
});
