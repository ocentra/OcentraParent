import { describe, expect, it } from 'vitest';
import {
  TrackingRetentionDurableSettingsRowSchema,
  buildTrackingRetentionDurableSettingsProof,
  type TrackingRetentionDurableSettingsProof,
} from '../src/tracking-retention-durable-settings-proof';
import { buildTrackingRetentionLocalServiceStateProof } from '../src/tracking-retention-local-service-state-proof';

const GeneratedAt = '2026-06-07T10:20:00.000Z';
const SourceWriteCommandProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json';
const SourceLocalServiceStateProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json';

describe('tracking retention durable settings proof', () => {
  it('derives durable settings manual-required rows from local service state readback', () => {
    const proof = durableProof();

    expect(proof.proofMode).toBe('tracking-retention-durable-settings-proof');
    expect(proof.proofClaims).toEqual({
      localServiceStateReadbackClaimed: true,
      durablePersistenceRequirementVisible: true,
      durabilityFailureVisible: true,
    });
    expect(proof.rows).toHaveLength(1);
    expectDurableSettingsRow(proof);
  });

  it('rejects durable persistence and product-ready overclaims', () => {
    const [row] = durableProof().rows;

    expect(
      TrackingRetentionDurableSettingsRowSchema.safeParse({
        ...row,
        durableSettingsPersisted: true,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionDurableSettingsRowSchema.safeParse({
        ...row,
        productClaimReady: true,
      }).success
    ).toBe(false);
  });

  it('rejects durable rows without source mutation proof refs', () => {
    const [row] = durableProof().rows;

    expect(
      TrackingRetentionDurableSettingsRowSchema.safeParse({
        ...row,
        sourceMutationProofRefs: [],
      }).success
    ).toBe(false);
  });
});

function durableProof(): TrackingRetentionDurableSettingsProof {
  return buildTrackingRetentionDurableSettingsProof(
    GeneratedAt,
    SourceLocalServiceStateProofRef,
    buildTrackingRetentionLocalServiceStateProof(GeneratedAt, SourceWriteCommandProofRef, writeResult())
  );
}

function expectDurableSettingsRow(proof: TrackingRetentionDurableSettingsProof): void {
  const [row] = proof.rows;
  expect(row.sourceLocalServiceStateProofRef).toBe(SourceLocalServiceStateProofRef);
  expect(row.sourceReadModelProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json'
  );
  expect(row.sourceMutationProofRefs).toContain(
    'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json'
  );
  expect(row.localServiceStateRevision).toBe(1);
  expect(row.localServiceStateSnapshotRef).toBe('agent-service-local-retention-settings-state');
  expect(row.durableStoreRef).toBe('retention-settings-durable-store-required');
  expect(row.durableSettingsPersisted).toBe(false);
  expect(row.durablePersistenceRequired).toBe(true);
  expect(row.durabilityFailureVisible).toBe(true);
  expect(row.productSettingsWritable).toBe(false);
  expect(row.productClaimReady).toBe(false);
}

function writeResult(): unknown {
  return {
    schemaVersion: 1,
    commandId: 'tracking-retention-settings-write-command',
    settingsKind: 'retention-window-setting',
    writeState: 'service-write-command-accepted',
    sourceWriterIntentRefs: ['tracking-retention-settings-write-retention-window'],
    sourceReadModelProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
    ],
    sourceMutationProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
    ],
    appliedRetentionWindowHours: 168,
    appliedDeleteAfterAlertResolved: false,
    parentExportPrepared: false,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    localServiceStateRevision: 1,
    localServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
    durableSettingsPersisted: false,
    commandTransportClaimed: true,
    serviceMutationExecuted: true,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  };
}
