import { describe, expect, it } from 'vitest';
import {
  screenLinuxCaptureCapabilityProof,
  ScreenLinuxCaptureCapabilityRowSchema,
} from '../../src/screen-linux-capture-capability-proof';

const CheckedAt = '2026-06-07T16:40:00Z';

describe('screen Linux capture capability proof', () => {
  it('keeps WSLg selected-window proof separate from native Linux product readiness', () => {
    const proof = screenLinuxCaptureCapabilityProof(CheckedAt);

    expect(proof.wslgSelectedWindowCaptureProved).toBe(true);
    expect(proof.productLinuxCaptureReady).toBe(false);
    expect(proof.rows.map((row) => row.mode)).toEqual([
      'wslgX11SelectedWindow',
      'nativeX11SelectedWindow',
      'nativeX11RootDisplay',
      'waylandPortalPipeWire',
      'waylandPortalPipeWire',
      'waylandPortalPipeWire',
      'unsupportedCompositor',
    ]);
    expect(proof.rows.every((row) => !row.rootDisplayClaimed)).toBe(true);
    expect(proof.rows.every((row) => !row.rawFrameRemoteUploadAllowed)).toBe(true);
  });

  it('allows native Linux readiness only with native session and deletion proof', () => {
    const nativeX11 = screenLinuxCaptureCapabilityProof(CheckedAt).rows[1];
    const ready = ScreenLinuxCaptureCapabilityRowSchema.parse({
      ...nativeX11,
      captureState: 'ready',
      proofState: 'nativeSessionVerified',
      nativeSessionProofRef: 'screen-linux-native-x11-selected-window-proof',
      deletionProofRef: 'screen-linux-native-x11-deletion-proof',
      productLinuxCaptureReady: true,
    });

    expect(ready.productLinuxCaptureReady).toBe(true);
    expect(ready.x11CommandBackendRequired).toBe(true);
  });

  it('rejects root-display overclaim, raw upload, and Wayland rows without portal requirements', () => {
    const nativeRoot = screenLinuxCaptureCapabilityProof(CheckedAt).rows[2];
    const wayland = screenLinuxCaptureCapabilityProof(CheckedAt).rows[3];
    const withoutNativeProof = ScreenLinuxCaptureCapabilityRowSchema.safeParse({
      ...nativeRoot,
      captureState: 'ready',
      proofState: 'nativeSessionVerified',
      deletionProofRef: 'screen-linux-native-root-deletion-proof',
      productLinuxCaptureReady: true,
    });
    const rootDisplayClaim = ScreenLinuxCaptureCapabilityRowSchema.safeParse({
      ...nativeRoot,
      rootDisplayClaimed: true,
    });
    const remoteRawUpload = ScreenLinuxCaptureCapabilityRowSchema.safeParse({
      ...nativeRoot,
      rawFrameRemoteUploadAllowed: true,
    });
    const missingPipeWire = ScreenLinuxCaptureCapabilityRowSchema.safeParse({
      ...wayland,
      pipeWireRequired: false,
    });

    expect(withoutNativeProof.success).toBe(false);
    expect(rootDisplayClaim.success).toBe(false);
    expect(remoteRawUpload.success).toBe(false);
    expect(missingPipeWire.success).toBe(false);
  });
});
