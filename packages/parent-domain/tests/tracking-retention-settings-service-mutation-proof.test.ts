import { describe, expect, it } from 'vitest';
import {
  TrackingRetentionSettingsServiceMutationProofSchema,
  TrackingRetentionSettingsServiceMutationRowSchema,
  buildTrackingRetentionSettingsServiceMutationProof,
  type TrackingRetentionSettingsServiceMutationProof,
} from '../src/tracking-retention-settings-service-mutation-proof';

const GeneratedAt = '2026-06-06T15:41:00.000Z';

describe('tracking retention settings service mutation proof rows', () => {
  it('builds service mutation rows from the writer-boundary proof', () => {
    const proof = buildTrackingRetentionSettingsServiceMutationProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-retention-settings-service-mutation-proof');
    expect(proof.proofClaims).toEqual({
      serviceCommandRegisteredClaimed: true,
      serviceMutationExecuted: true,
      writerBoundaryProofConsumed: true,
    });
    expect(proof.rows.map((row) => row.settingsKind)).toEqual([
      'retention-window-setting',
      'delete-after-alert-setting',
      'parent-export-setting',
      'remote-sync-disabled-setting',
      'remote-ai-disabled-setting',
    ]);
    for (const row of proof.rows) {
      expect(row.mutationState).toBe('accepted');
      expect(row.requestId).toContain(row.intentId);
      expect(row.mutationId).toContain(row.requestId);
      expect(row.sourceReadModelProofRefs.length).toBeGreaterThan(0);
      expect(row.writerBoundaryProofRefs).toContain(
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-settings-writer-boundary-proof.json'
      );
      expect(row.evidenceReferences.length).toBeGreaterThan(0);
      expect(row.auditRefs.length).toBeGreaterThan(0);
      expect(row.serviceCommandRegisteredClaimed).toBe(true);
      expect(row.serviceMutationExecuted).toBe(true);
      expectNoProductClaims(row);
    }
  });

  it('keeps requested values explicit for each retained setting mutation', () => {
    const proof = buildTrackingRetentionSettingsServiceMutationProof(GeneratedAt);

    expect(rowFor(proof, 'retention-window-setting').requestedValue).toBe('168');
    expect(rowFor(proof, 'delete-after-alert-setting').requestedValue).toBe('true');
    expect(rowFor(proof, 'parent-export-setting').requestedValue).toBe('true');
    expect(rowFor(proof, 'remote-sync-disabled-setting').requestedValue).toBe('false');
    expect(rowFor(proof, 'remote-ai-disabled-setting').requestedValue).toBe('false');
    expect(Object.values(proof.productClaims).every((value) => value === false)).toBe(true);
  });
});

describe('tracking retention settings service mutation proof validation', () => {
  it('rejects missing proof refs and product overclaims', () => {
    const row = rowFor(buildTrackingRetentionSettingsServiceMutationProof(GeneratedAt), 'retention-window-setting');

    expect(
      TrackingRetentionSettingsServiceMutationRowSchema.safeParse({ ...row, sourceReadModelProofRefs: [] }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsServiceMutationRowSchema.safeParse({ ...row, writerBoundaryProofRefs: [] }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsServiceMutationRowSchema.safeParse({ ...row, evidenceReferences: [] }).success
    ).toBe(false);
    expect(TrackingRetentionSettingsServiceMutationRowSchema.safeParse({ ...row, auditRefs: [] }).success).toBe(false);
    expect(
      TrackingRetentionSettingsServiceMutationRowSchema.safeParse({ ...row, productClaimReady: true }).success
    ).toBe(false);
  });

  it('requires all five setting rows in the proof', () => {
    const proof = buildTrackingRetentionSettingsServiceMutationProof(GeneratedAt);

    expect(
      TrackingRetentionSettingsServiceMutationProofSchema.safeParse({ ...proof, rows: proof.rows.slice(0, 4) }).success
    ).toBe(false);
  });
});

function expectNoProductClaims(row: TrackingRetentionSettingsServiceMutationProof['rows'][number]): void {
  expect(row.durablePersistenceClaimed).toBe(false);
  expect(row.portalUiClaimed).toBe(false);
  expect(row.platformRuntimeClaimed).toBe(false);
  expect(row.childDeviceDeliveryClaimed).toBe(false);
  expect(row.providerDeliveryClaimed).toBe(false);
  expect(row.notificationReceiptClaimed).toBe(false);
  expect(row.physicalDeviceClaimed).toBe(false);
  expect(row.authorityClaimed).toBe(false);
  expect(row.productClaimReady).toBe(false);
}

function rowFor(
  proof: TrackingRetentionSettingsServiceMutationProof,
  settingsKind: TrackingRetentionSettingsServiceMutationProof['rows'][number]['settingsKind']
): TrackingRetentionSettingsServiceMutationProof['rows'][number] {
  const row = proof.rows.find((entry) => entry.settingsKind === settingsKind);
  if (row === undefined) {
    throw new Error(`Missing tracking retention service mutation row: ${settingsKind}`);
  }
  return row;
}
