import { describe, expect, it } from 'vitest';
import {
  TrackingRetentionSettingsMutationRowSchema,
  buildTrackingRetentionSettingsMutationProof,
  type TrackingRetentionSettingsMutationProof,
  type TrackingRetentionSettingsMutationRow,
} from '../../src/tracking-retention-settings-mutation-proof';
import {
  AgentTrackingRetentionSettingsWriteDefaults,
  type TrackingRetentionSettingsKind,
} from '../../src/tracking-retention-settings-read-model-proof';

const GeneratedAt = '2026-06-06T19:40:00.000Z';

describe('tracking retention settings mutation proof rows', () => {
  it('executes local retention setting mutations from writer-boundary intents', () => {
    const proof = buildTrackingRetentionSettingsMutationProof(GeneratedAt);

    expect(proof.proofMode).toBe('tracking-retention-settings-mutation-proof');
    expect(proof.proofClaims).toEqual({
      localValidationClaimed: true,
      writerBoundaryClaimed: true,
      serviceMutationPreflightClaimed: true,
      serviceMutationExecuted: true,
    });
    expect(proof.rows.map((row) => row.settingsKind)).toEqual([
      AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
      'delete-after-alert-setting',
      'parent-export-setting',
      'remote-sync-disabled-setting',
      'remote-ai-disabled-setting',
    ]);
    for (const row of proof.rows) {
      expectMutationRow(row);
    }
  });

  it('applies retention settings while preserving remote sync and remote AI disabled', () => {
    const proof = buildTrackingRetentionSettingsMutationProof(GeneratedAt);

    expect(rowFor(proof, AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow).appliedRetentionWindowHours).toBe(168);
    expect(rowFor(proof, 'delete-after-alert-setting').appliedDeleteAfterAlertResolved).toBe(true);
    expect(rowFor(proof, 'parent-export-setting').parentExportPrepared).toBe(true);
    expect(rowFor(proof, 'remote-sync-disabled-setting').remoteSyncEnabled).toBe(false);
    expect(rowFor(proof, 'remote-ai-disabled-setting').remoteAiEnabled).toBe(false);
    expect(Object.values(proof.productClaims).every((value) => value === false)).toBe(true);
  });
});

describe('tracking retention settings mutation validation', () => {
  it('rejects mutation rows without writer intent refs or audit refs', () => {
    const retentionRow = rowFor(
      buildTrackingRetentionSettingsMutationProof(GeneratedAt),
      AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow
    );

    expect(
      TrackingRetentionSettingsMutationRowSchema.safeParse({
        ...retentionRow,
        sourceWriterIntentRefs: [],
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsMutationRowSchema.safeParse({
        ...retentionRow,
        auditRefs: [],
      }).success
    ).toBe(false);
  });

  it('rejects hidden applied setting values', () => {
    const proof = buildTrackingRetentionSettingsMutationProof(GeneratedAt);

    expect(
      TrackingRetentionSettingsMutationRowSchema.safeParse({
        ...rowFor(proof, AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow),
        appliedRetentionWindowHours: null,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsMutationRowSchema.safeParse({
        ...rowFor(proof, 'delete-after-alert-setting'),
        appliedDeleteAfterAlertResolved: false,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionSettingsMutationRowSchema.safeParse({
        ...rowFor(proof, 'parent-export-setting'),
        parentExportPrepared: false,
      }).success
    ).toBe(false);
  });
});

function expectMutationRow(row: TrackingRetentionSettingsMutationRow): void {
  expect(row.sourceWriterIntentRefs.length).toBeGreaterThan(0);
  expect(row.sourceReadModelProofRefs).toContain(
    AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs[1]
  );
  expect(row.retentionProofRefs).toContain(
    'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json'
  );
  expect(row.readModelProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json'
  );
  expect(row.evidenceReferences.length).toBeGreaterThan(0);
  expect(row.reasonCodes.length).toBeGreaterThan(0);
  expect(row.auditRefs.length).toBeGreaterThan(1);
  expect(row.localValidationClaimed).toBe(true);
  expect(row.writerBoundaryClaimed).toBe(true);
  expect(row.serviceMutationPreflightClaimed).toBe(true);
  expect(row.serviceMutationExecuted).toBe(true);
  expect(row.remoteSyncEnabled).toBe(false);
  expect(row.remoteAiEnabled).toBe(false);
  expectNoProductClaims(row);
}

function expectNoProductClaims(row: TrackingRetentionSettingsMutationRow): void {
  expect(row.portalWritableUiClaimed).toBe(false);
  expect(row.platformRuntimeClaimed).toBe(false);
  expect(row.childDeviceDeliveryClaimed).toBe(false);
  expect(row.providerDeliveryClaimed).toBe(false);
  expect(row.notificationReceiptClaimed).toBe(false);
  expect(row.physicalDeviceClaimed).toBe(false);
  expect(row.authorityClaimed).toBe(false);
  expect(row.productClaimReady).toBe(false);
}

function rowFor(
  proof: TrackingRetentionSettingsMutationProof,
  settingsKind: TrackingRetentionSettingsKind
): TrackingRetentionSettingsMutationRow {
  const row = proof.rows.find((entry) => entry.settingsKind === settingsKind);
  if (row === undefined) {
    throw new Error(`Missing tracking retention mutation row: ${settingsKind}`);
  }
  return row;
}
