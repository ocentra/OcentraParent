import { describe, expect, it } from 'vitest';
import { ScreenOptionalVisibilityCapabilityStatusSchema } from '@ocentra-parent/schema-domain/screen-optional-visibility-capability-status';
import { screenOptionalVisibilityCapabilityStatusProof } from '@ocentra-parent/schema-domain/screen-optional-visibility-capability-proof';

const GeneratedAt = '2026-06-07T05:55:00Z';

describe('screen optional visibility capability status', () => {
  registerSummaryTest();
  registerRejectsRawRetentionReadinessTest();
  registerAllowsRawRetentionReadinessTest();
  registerRejectsLiveViewReadinessTest();
});

function registerSummaryTest(): void {
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
}

function registerRejectsRawRetentionReadinessTest(): void {
  it('rejects raw retention readiness without runtime and deletion proof', () => {
    const proof = screenOptionalVisibilityCapabilityStatusProof(GeneratedAt);
    const rawRetentionRow = proof.rows.find(
      (row) => row.capabilityKind === 'rawScreenshotRetention' && row.readinessState === 'manualRequired'
    );

    expect(rawRetentionRow).toEqual(
      expect.objectContaining({
        capabilityKind: 'rawScreenshotRetention',
        readinessState: 'manualRequired',
      })
    );
    if (rawRetentionRow === undefined) {
      throw new Error('Expected a manual-required raw screenshot retention row');
    }
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
}

function registerAllowsRawRetentionReadinessTest(): void {
  it('allows raw retention readiness only with runtime and deletion proof', () => {
    const proof = screenOptionalVisibilityCapabilityStatusProof(GeneratedAt);
    const readyRetentionRow = proof.rows.find(
      (row) => row.capabilityKind === 'rawScreenshotRetention' && row.readinessState === 'ready'
    );

    expect(readyRetentionRow).toEqual(
      expect.objectContaining({
        capabilityKind: 'rawScreenshotRetention',
        readinessState: 'ready',
      })
    );
    if (readyRetentionRow === undefined) {
      throw new Error('Expected a ready raw screenshot retention row');
    }
    expect(readyRetentionRow.runtimeProofRef).toBe(
      'output/screen-plan-proof/screen-settings-service-command/proof-summary.json'
    );
    expect(readyRetentionRow.deletionProofRef).toBe(
      'output/screen-plan-proof/screen-service-deletion-event-producer/proof-summary.json'
    );
    expect(readyRetentionRow.childDisclosureReady).toBe(true);
    expect(readyRetentionRow.childDeviceCapabilityReady).toBe(true);
    expect(readyRetentionRow.productModeReady).toBe(true);
    expect(readyRetentionRow.rawFramesRetained).toBe(false);
    expect(readyRetentionRow.rawRemoteUploadAllowed).toBe(false);
  });
}

function registerRejectsLiveViewReadinessTest(): void {
  it('rejects live view readiness when the platform gate only proves capture permission', () => {
    const proof = screenOptionalVisibilityCapabilityStatusProof(GeneratedAt);
    const liveViewRow = proof.rows.find((row) => row.capabilityKind === 'liveView' && row.readinessState === 'blocked');

    expect(liveViewRow).toEqual(
      expect.objectContaining({
        capabilityKind: 'liveView',
        readinessState: 'blocked',
      })
    );
    if (liveViewRow === undefined) {
      throw new Error('Expected a blocked live-view capability row');
    }
    const parsed = ScreenOptionalVisibilityCapabilityStatusSchema.safeParse({
      ...liveViewRow,
      readinessState: 'ready',
      childDisclosureReady: true,
      childDeviceCapabilityReady: true,
      productModeReady: true,
    });

    expect(parsed.success).toBe(false);
  });
}
