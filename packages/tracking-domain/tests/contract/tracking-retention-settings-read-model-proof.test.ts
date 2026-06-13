import { describe, expect, it } from 'vitest';
import {
  TrackingRetentionSettingsRowSchema,
  AgentTrackingRetentionSettingsWriteDefaults,
  buildTrackingRetentionSettingsReadModelProof,
  type TrackingRetentionSettingsKind,
  type TrackingRetentionSettingsProof,
  type TrackingRetentionSettingsRow,
} from '../../src/tracking-retention-settings-read-model-proof';

const GeneratedAt = '2026-06-06T05:24:00.000Z';

describe('tracking retention settings read model proof', () => {
  it('builds retention settings rows from existing retention and read-model proof refs', () => {
    const proof = buildTrackingRetentionSettingsReadModelProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-retention-settings-read-model-proof');
    expect(proof.rows.map((row) => row.settingsKind)).toEqual([
      AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
      'delete-after-alert-setting',
      'parent-export-setting',
      'remote-sync-disabled-setting',
      'remote-ai-disabled-setting',
    ]);
    expect(proof.productClaims.productClaimReady).toBe(false);
    for (const row of proof.rows) {
      expectReadModelProofRow(row);
    }
  });

  it('keeps delete, export, remote sync, and remote AI settings explicit', () => {
    const proof = buildTrackingRetentionSettingsReadModelProof(GeneratedAt);

    expect(rowFor(proof, 'delete-after-alert-setting').deleteAfterAlertResolved).toBe(true);
    expect(rowFor(proof, 'parent-export-setting').parentExportReady).toBe(true);
    expect(rowFor(proof, 'remote-sync-disabled-setting').custodyScope).toBe('remote-disabled');
    expect(rowFor(proof, 'remote-ai-disabled-setting').custodyScope).toBe('remote-disabled');
  });

  it('rejects rows without proof refs or evidence refs', () => {
    const retentionRow = rowFor(
      buildTrackingRetentionSettingsReadModelProof(GeneratedAt),
      AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow
    );

    expect(TrackingRetentionSettingsRowSchema.safeParse({ ...retentionRow, sourceProofRefs: [] }).success).toBe(false);
    expect(TrackingRetentionSettingsRowSchema.safeParse({ ...retentionRow, retentionProofRefs: [] }).success).toBe(
      false
    );
    expect(TrackingRetentionSettingsRowSchema.safeParse({ ...retentionRow, readModelProofRefs: [] }).success).toBe(
      false
    );
    expect(TrackingRetentionSettingsRowSchema.safeParse({ ...retentionRow, evidenceReferences: [] }).success).toBe(
      false
    );
  });

  it('rejects settings rows that hide required retention controls', () => {
    const proof = buildTrackingRetentionSettingsReadModelProof(GeneratedAt);

    expect(
      TrackingRetentionSettingsRowSchema.safeParse({
        ...rowFor(proof, 'delete-after-alert-setting'),
        deleteAfterAlertResolved: false,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsRowSchema.safeParse({
        ...rowFor(proof, 'parent-export-setting'),
        parentExportReady: false,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsRowSchema.safeParse({
        ...rowFor(proof, 'remote-sync-disabled-setting'),
        custodyScope: 'parent-device-local',
      }).success
    ).toBe(false);
  });
});

function expectReadModelProofRow(row: TrackingRetentionSettingsRow): void {
  expect(row.settingsState).toBe('settings-read-model-ready');
  expect(row.requiredProofTier).toBe('P2_HOSTED_CI');
  expect(row.currentProofTier).toBe('P2_HOSTED_CI');
  expect(row.sourceProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json'
  );
  expect(row.retentionProofRefs).toContain(
    'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json'
  );
  expect(row.readModelProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/14-retention-delete-proof.json'
  );
  expect(row.evidenceReferences.length).toBeGreaterThan(0);
  expect(row.remoteSyncEnabled).toBe(false);
  expect(row.remoteAiEnabled).toBe(false);
  expectNoProductClaims(row);
}

function expectNoProductClaims(row: TrackingRetentionSettingsRow): void {
  expect(row.settingsReadModelClaimed).toBe(true);
  expect(row.portalUiClaimed).toBe(false);
  expect(row.serviceMutationClaimed).toBe(false);
  expect(row.platformRuntimeClaimed).toBe(false);
  expect(row.childDeviceDeliveryClaimed).toBe(false);
  expect(row.providerDeliveryClaimed).toBe(false);
  expect(row.notificationReceiptClaimed).toBe(false);
  expect(row.physicalDeviceClaimed).toBe(false);
  expect(row.authorityClaimed).toBe(false);
  expect(row.productClaimReady).toBe(false);
}

function rowFor(
  proof: TrackingRetentionSettingsProof,
  settingsKind: TrackingRetentionSettingsKind
): TrackingRetentionSettingsRow {
  const row = proof.rows.find((entry) => entry.settingsKind === settingsKind);
  if (row === undefined) {
    throw new Error(`Missing tracking retention settings row: ${settingsKind}`);
  }
  return row;
}
