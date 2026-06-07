import { describe, expect, it } from 'vitest';
import {
  ProductReadinessBlockers,
  TrackingRetentionProductReadinessRowSchema,
  buildTrackingRetentionProductReadinessProof,
  type TrackingRetentionProductReadinessProof,
} from '../src/tracking-retention-product-readiness-proof';
import { buildTrackingRetentionDurableSettingsProof } from '../src/tracking-retention-durable-settings-proof';
import { buildTrackingRetentionLocalServiceStateProof } from '../src/tracking-retention-local-service-state-proof';

const GeneratedAt = '2026-06-07T15:30:00.000Z';
const SourceWriteCommandProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json';
const SourceLocalServiceStateProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json';
const SourceDurableSettingsProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/23-retention-durable-settings-proof.json';

describe('tracking retention product readiness proof', () => {
  it('keeps durable local settings separate from product-ready retention claims', () => {
    const proof = readinessProof();

    expect(proof.proofMode).toBe('tracking-retention-product-readiness-proof');
    expect(proof.proofClaims).toEqual({
      localDurableSettingsReady: true,
      productReadinessBlockersEnumerated: true,
      noProductReadyClaim: true,
    });
    expect(proof.rows).toHaveLength(1);
    expectReadinessRow(proof);
  });

  it('rejects product-ready overclaims', () => {
    const [row] = readinessProof().rows;

    expect(
      TrackingRetentionProductReadinessRowSchema.safeParse({
        ...row,
        productClaimReady: true,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionProductReadinessRowSchema.safeParse({
        ...row,
        platformRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });

  it('rejects missing product-readiness blockers', () => {
    const [row] = readinessProof().rows;

    expect(
      TrackingRetentionProductReadinessRowSchema.safeParse({
        ...row,
        productReadinessBlockers: ProductReadinessBlockers.slice(0, 1),
      }).success
    ).toBe(false);
  });
});

function readinessProof(): TrackingRetentionProductReadinessProof {
  return buildTrackingRetentionProductReadinessProof(
    GeneratedAt,
    SourceDurableSettingsProofRef,
    buildTrackingRetentionDurableSettingsProof(
      GeneratedAt,
      SourceLocalServiceStateProofRef,
      buildTrackingRetentionLocalServiceStateProof(GeneratedAt, SourceWriteCommandProofRef, writeResult())
    )
  );
}

function expectReadinessRow(proof: TrackingRetentionProductReadinessProof): void {
  const [row] = proof.rows;
  expect(row.sourceDurableSettingsProofRef).toBe(SourceDurableSettingsProofRef);
  expect(row.sourceLocalServiceStateProofRef).toBe(SourceLocalServiceStateProofRef);
  expect(row.productReadinessBlockers).toEqual([...ProductReadinessBlockers]);
  expect(row.localDurableSettingsReady).toBe(true);
  expect(row.durableSettingsPersisted).toBe(true);
  expect(row.productSettingsWritable).toBe(false);
  expect(row.providerDeliveryClaimed).toBe(false);
  expect(row.notificationReceiptClaimed).toBe(false);
  expect(row.productionWorkerClaimed).toBe(false);
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
    durableSettingsStoreRef: 'agent-service-local-retention-settings-durable-json',
    durableSettingsPersisted: true,
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
