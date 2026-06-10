import { describe, expect, it } from 'vitest';
import {
  screenOptionalVisibilityCapabilityStatusProof,
  ScreenOptionalVisibilityCapabilityStatusSchema,
} from '../src/screen-evidence';

const GeneratedAt = '2026-06-07T05:55:00Z';

describe('screen optional visibility capability status', () => {
  it('summarizes disabled and blocked optional visibility modes without enabling raw frames', () => {
    const proof = screenOptionalVisibilityCapabilityStatusProof(GeneratedAt);

    expect(proof.rows.map((row) => row.readinessState)).toEqual([
      'disabled',
      'manualRequired',
      'ready',
      'disabled',
      'blocked',
    ]);
    expect(proof.rows.every((row) => !row.rawFramesRetained)).toBe(true);
    expect(proof.rows.every((row) => !row.rawRemoteUploadAllowed)).toBe(true);
    expect(proof.rows.every((row) => !row.remoteInputAllowed)).toBe(true);
    expect(proof.rows.some((row) => row.capabilityKind === 'liveView' && row.productModeReady)).toBe(false);
  });

  it('rejects raw retention readiness without runtime and deletion proof', () => {
    const proof = screenOptionalVisibilityCapabilityStatusProof(GeneratedAt);
    const rawRetentionRow = proof.rows.find(
      (row) => row.capabilityKind === 'rawScreenshotRetention' && row.readinessState === 'manualRequired'
    );

    expect(rawRetentionRow).toBeDefined();
    const parsed = ScreenOptionalVisibilityCapabilityStatusSchema.safeParse({
      ...rawRetentionRow,
      readinessState: 'ready',
      childDisclosureReady: true,
      childDeviceCapabilityReady: true,
      productModeReady: true,
      runtimeProofRef: null,
      deletionProofRef: null,
    });

    expect(parsed.success).toBe(false);
  });

  it('allows raw retention readiness only with runtime and deletion proof', () => {
    const proof = screenOptionalVisibilityCapabilityStatusProof(GeneratedAt);
    const readyRetentionRow = proof.rows.find(
      (row) => row.capabilityKind === 'rawScreenshotRetention' && row.readinessState === 'ready'
    );

    expect(readyRetentionRow).toBeDefined();
    expect(readyRetentionRow?.runtimeProofRef).toBe(
      'output/screen-plan-proof/screen-settings-service-command/proof-summary.json'
    );
    expect(readyRetentionRow?.deletionProofRef).toBe(
      'output/screen-plan-proof/screen-service-deletion-event-producer/proof-summary.json'
    );
    expect(readyRetentionRow?.childDisclosureReady).toBe(true);
    expect(readyRetentionRow?.childDeviceCapabilityReady).toBe(true);
    expect(readyRetentionRow?.productModeReady).toBe(true);
    expect(readyRetentionRow?.rawFramesRetained).toBe(false);
    expect(readyRetentionRow?.rawRemoteUploadAllowed).toBe(false);
  });

  it('rejects live view readiness when the platform gate only proves capture permission', () => {
    const proof = screenOptionalVisibilityCapabilityStatusProof(GeneratedAt);
    const liveViewRow = proof.rows.find((row) => row.capabilityKind === 'liveView' && row.readinessState === 'blocked');

    expect(liveViewRow).toBeDefined();
    const parsed = ScreenOptionalVisibilityCapabilityStatusSchema.safeParse({
      ...liveViewRow,
      readinessState: 'ready',
      childDisclosureReady: true,
      childDeviceCapabilityReady: true,
      productModeReady: true,
    });

    expect(parsed.success).toBe(false);
  });
});
