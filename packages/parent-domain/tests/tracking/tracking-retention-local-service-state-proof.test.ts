import { describe, expect, it } from 'vitest';
import {
  TrackingRetentionLocalServiceStateRowSchema,
  TrackingRetentionLocalServiceStateWriteResultSchema,
  buildTrackingRetentionLocalServiceStateProof,
  type TrackingRetentionLocalServiceStateProof,
} from '../../src/tracking-retention-local-service-state-proof';

const GeneratedAt = '2026-06-07T09:05:00.000Z';
const SourceProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json';

describe('tracking retention local service state readback proof', () => {
  it('derives local service state readback from an accepted write command result', () => {
    const proof = buildTrackingRetentionLocalServiceStateProof(GeneratedAt, SourceProofRef, writeResult());

    expect(proof.proofMode).toBe('tracking-retention-local-service-state-proof');
    expect(proof.proofClaims).toEqual({
      writeCommandAccepted: true,
      serviceMutationExecuted: true,
      localServiceStateRevisionRecorded: true,
      localServiceStateReadbackClaimed: true,
      durableSettingsPersisted: true,
    });
    expect(proof.rows).toHaveLength(1);
    expectLocalStateRow(proof);
  });

  it('rejects missing revision, missing durable persistence, and product claims', () => {
    const acceptedResult = writeResult();

    expect(
      TrackingRetentionLocalServiceStateWriteResultSchema.safeParse({
        ...acceptedResult,
        localServiceStateRevision: null,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionLocalServiceStateWriteResultSchema.safeParse({
        ...acceptedResult,
        durableSettingsPersisted: false,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionLocalServiceStateWriteResultSchema.safeParse({
        ...acceptedResult,
        productClaimReady: true,
      }).success
    ).toBe(false);
  });

  it('rejects readback rows that hide applied retention values', () => {
    const proof = buildTrackingRetentionLocalServiceStateProof(GeneratedAt, SourceProofRef, writeResult());
    const [row] = proof.rows;

    expect(
      TrackingRetentionLocalServiceStateRowSchema.safeParse({
        ...row,
        appliedRetentionWindowHours: null,
      }).success
    ).toBe(false);
  });
});

function expectLocalStateRow(proof: TrackingRetentionLocalServiceStateProof): void {
  const [row] = proof.rows;
  expect(row.sourceWriteCommandProofRef).toBe(SourceProofRef);
  expect(row.sourceReadModelProofRefs).toContain(
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json'
  );
  expect(row.sourceMutationProofRefs).toContain(
    'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json'
  );
  expect(row.localServiceStateRevision).toBe(1);
  expect(row.localServiceStateSnapshotRef).toBe('agent-service-local-retention-settings-state');
  expect(row.durableSettingsStoreRef).toBe('agent-service-local-retention-settings-durable-json');
  expect(row.appliedRetentionWindowHours).toBe(168);
  expect(row.remoteSyncEnabled).toBe(false);
  expect(row.remoteAiEnabled).toBe(false);
  expect(row.writeCommandAccepted).toBe(true);
  expect(row.serviceMutationExecuted).toBe(true);
  expect(row.localServiceStateReadbackClaimed).toBe(true);
  expectNoProductClaims(row);
}

function expectNoProductClaims(row: {
  readonly durableSettingsPersisted: boolean;
  readonly platformRuntimeClaimed: false;
  readonly childDeviceDeliveryClaimed: false;
  readonly providerDeliveryClaimed: false;
  readonly notificationReceiptClaimed: false;
  readonly physicalDeviceClaimed: false;
  readonly authorityClaimed: false;
  readonly productClaimReady: false;
}): void {
  expect(row.durableSettingsPersisted).toBe(true);
  expect(row.platformRuntimeClaimed).toBe(false);
  expect(row.childDeviceDeliveryClaimed).toBe(false);
  expect(row.providerDeliveryClaimed).toBe(false);
  expect(row.notificationReceiptClaimed).toBe(false);
  expect(row.physicalDeviceClaimed).toBe(false);
  expect(row.authorityClaimed).toBe(false);
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
