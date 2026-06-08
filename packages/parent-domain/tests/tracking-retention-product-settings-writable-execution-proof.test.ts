import { describe, expect, it } from 'vitest';
import { buildTrackingRetentionLocalServiceStateProof } from '../src/tracking-retention-local-service-state-proof';
import {
  TrackingRetentionProductSettingsWritableExecutionArtifactRef,
  TrackingRetentionProductSettingsWritableExecutionRowSchema,
  buildTrackingRetentionProductSettingsWritableExecutionProof,
} from '../src/tracking-retention-product-settings-writable-execution-proof';

const GeneratedAt = '2026-06-08T03:45:00.000Z';
const SourceLocalServiceStateProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json';
const SourceWriteCommandProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json';

describe('tracking retention product settings writable execution proof', () => {
  it('derives the writable execution artifact from accepted local service state', () => {
    const proof = buildTrackingRetentionProductSettingsWritableExecutionProof(
      GeneratedAt,
      SourceLocalServiceStateProofRef,
      localServiceStateProof()
    );

    expect(proof.proofMode).toBe('tracking-retention-product-settings-writable-execution-proof');
    expect(proof.rows).toHaveLength(1);
    expect(proof.proofClaims).toEqual({
      writeCommandAccepted: true,
      serviceMutationExecuted: true,
      localServiceStateReadbackClaimed: true,
      durableSettingsPersisted: true,
      localProductSettingsWritableExecutionObserved: true,
      noProductReadyClaim: true,
    });
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.derivationMatrix).toHaveLength(1);

    const [row] = proof.rows;
    const [derivation] = proof.derivationMatrix;
    expect(row.outputArtifactRef).toBe(TrackingRetentionProductSettingsWritableExecutionArtifactRef);
    expect(row.sourceLocalServiceStateProofRef).toBe(SourceLocalServiceStateProofRef);
    expect(row.sourceWriteCommandProofRef).toBe(SourceWriteCommandProofRef);
    expect(row.localServiceStateRevision).toBe(1);
    expect(row.localServiceStateSnapshotRef).toBe('agent-service-local-retention-settings-state');
    expect(row.durableSettingsStoreRef).toBe('agent-service-local-retention-settings-durable-json');
    expect(row.appliedRetentionWindowHours).toBe(168);
    expect(row.remoteSyncEnabled).toBe(false);
    expect(row.remoteAiEnabled).toBe(false);
    expect(row.localProductSettingsWritableExecutionObserved).toBe(true);
    expect(row.portalWritableUiClaimed).toBe(false);
    expect(row.platformRuntimeRetentionEnforcementClaimed).toBe(false);
    expect(row.productClaimReady).toBe(false);
    expectDerivationMatrixEntry(derivation, row);
  });

  it('rejects product-ready or platform enforcement overclaims', () => {
    const [row] = buildTrackingRetentionProductSettingsWritableExecutionProof(
      GeneratedAt,
      SourceLocalServiceStateProofRef,
      localServiceStateProof()
    ).rows;

    expect(
      TrackingRetentionProductSettingsWritableExecutionRowSchema.safeParse({
        ...row,
        productClaimReady: true,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionProductSettingsWritableExecutionRowSchema.safeParse({
        ...row,
        platformRuntimeRetentionEnforcementClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionProductSettingsWritableExecutionRowSchema.safeParse({
        ...row,
        physicalDeviceProofClaimed: true,
      }).success
    ).toBe(false);
  });
});

function expectDerivationMatrixEntry(
  derivation: unknown,
  row: ReturnType<typeof buildTrackingRetentionProductSettingsWritableExecutionProof>['rows'][number]
) {
  expect(derivation).toEqual({
    rowId: row.rowId,
    sourceLocalServiceStateProofRef: SourceLocalServiceStateProofRef,
    sourceWriteCommandProofRef: SourceWriteCommandProofRef,
    sourceReadModelProofRefs: row.sourceReadModelProofRefs,
    sourceMutationProofRefs: row.sourceMutationProofRefs,
    localServiceStateRevision: 1,
    localServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
    durableSettingsStoreRef: 'agent-service-local-retention-settings-durable-json',
    appliedRetentionWindowHours: 168,
    appliedDeleteAfterAlertResolved: false,
    outputArtifactRef: TrackingRetentionProductSettingsWritableExecutionArtifactRef,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    portalWritableUiClaimed: false,
    platformRuntimeRetentionEnforcementClaimed: false,
    productClaimReady: false,
  });
}

function localServiceStateProof(): unknown {
  return buildTrackingRetentionLocalServiceStateProof(GeneratedAt, SourceWriteCommandProofRef, writeResult());
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
