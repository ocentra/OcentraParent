import { describe, expect, it } from 'vitest';
import {
  TrackingRetentionSettingsWriterBoundaryRowSchema,
  buildTrackingRetentionSettingsWriterBoundaryProof,
  type TrackingRetentionSettingsWriterBoundaryProof,
  type TrackingRetentionSettingsWriterBoundaryRow,
} from '../../src/tracking-retention-settings-writer-boundary-proof';
import { type TrackingRetentionSettingsKind } from '../../src/tracking-retention-settings-read-model-proof';

const GeneratedAt = '2026-06-06T13:17:00.000Z';

describe('tracking retention settings writer boundary proof rows', () => {
  it('builds writer preflight rows from the existing retention settings read model', () => {
    const proof = buildTrackingRetentionSettingsWriterBoundaryProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-retention-settings-writer-boundary-proof');
    expect(proof.proofClaims).toEqual({
      localValidationClaimed: true,
      writerBoundaryClaimed: true,
      serviceMutationPreflightClaimed: true,
    });
    expect(proof.rows.map((row) => row.settingsKind)).toEqual([
      'retention-window-setting',
      'delete-after-alert-setting',
      'parent-export-setting',
      'remote-sync-disabled-setting',
      'remote-ai-disabled-setting',
    ]);
    for (const row of proof.rows) {
      expectWriterPreflightRow(row);
    }
  });

  it('keeps writable retention intents explicit without executing service mutation', () => {
    const proof = buildTrackingRetentionSettingsWriterBoundaryProof(GeneratedAt);

    expect(rowFor(proof, 'retention-window-setting').requestedRetentionWindowHours).toBe(168);
    expect(rowFor(proof, 'delete-after-alert-setting').requestedDeleteAfterAlertResolved).toBe(true);
    expect(rowFor(proof, 'parent-export-setting').requestedParentExport).toBe(true);
    expect(rowFor(proof, 'remote-sync-disabled-setting').requestedRemoteSyncEnabled).toBe(false);
    expect(rowFor(proof, 'remote-ai-disabled-setting').requestedRemoteAiEnabled).toBe(false);
    expect(proof.rows.every((row) => row.serviceMutationExecuted === false)).toBe(true);
    expect(Object.values(proof.productClaims).every((value) => value === false)).toBe(true);
  });
});

describe('tracking retention settings writer boundary proof validation', () => {
  it('rejects writer rows without proof refs, evidence, or audit refs', () => {
    const retentionRow = rowFor(
      buildTrackingRetentionSettingsWriterBoundaryProof(GeneratedAt),
      'retention-window-setting'
    );

    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...retentionRow,
        sourceReadModelProofRefs: [],
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...retentionRow,
        retentionProofRefs: [],
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...retentionRow,
        readModelProofRefs: [],
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...retentionRow,
        evidenceReferences: [],
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...retentionRow,
        auditRefs: [],
      }).success
    ).toBe(false);
  });

  it('rejects mismatched writer actions and hidden retention controls', () => {
    const proof = buildTrackingRetentionSettingsWriterBoundaryProof(GeneratedAt);

    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...rowFor(proof, 'retention-window-setting'),
        requestedRetentionWindowHours: null,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...rowFor(proof, 'delete-after-alert-setting'),
        requestedDeleteAfterAlertResolved: false,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...rowFor(proof, 'parent-export-setting'),
        requestedParentExport: false,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsWriterBoundaryRowSchema.safeParse({
        ...rowFor(proof, 'remote-sync-disabled-setting'),
        writeAction: 'set-retention-window',
      }).success
    ).toBe(false);
  });
});

function expectWriterPreflightRow(row: TrackingRetentionSettingsWriterBoundaryRow): void {
  expect(row.writerState).toBe('writer-preflight-ready');
  expect(row.sourceReadModelProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json'
  );
  expect(row.retentionProofRefs).toContain(
    'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json'
  );
  expect(row.readModelProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json'
  );
  expect(row.evidenceReferences.length).toBeGreaterThan(0);
  expect(row.reasonCodes.length).toBeGreaterThan(0);
  expect(row.auditRefs.length).toBeGreaterThan(0);
  expect(row.localValidationClaimed).toBe(true);
  expect(row.writerBoundaryClaimed).toBe(true);
  expect(row.serviceMutationPreflightClaimed).toBe(true);
  expect(row.requestedRemoteSyncEnabled).toBe(false);
  expect(row.requestedRemoteAiEnabled).toBe(false);
  expectNoProductClaims(row);
}

function expectNoProductClaims(row: TrackingRetentionSettingsWriterBoundaryRow): void {
  expect(row.serviceMutationExecuted).toBe(false);
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
  proof: TrackingRetentionSettingsWriterBoundaryProof,
  settingsKind: TrackingRetentionSettingsKind
): TrackingRetentionSettingsWriterBoundaryRow {
  const row = proof.rows.find((entry) => entry.settingsKind === settingsKind);
  if (row === undefined) {
    throw new Error(`Missing tracking retention writer row: ${settingsKind}`);
  }
  return row;
}
