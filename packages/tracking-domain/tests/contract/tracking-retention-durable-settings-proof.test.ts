import { describe, expect, it } from 'vitest';
import {
  TrackingRetentionDurableSettingsRowSchema,
  buildTrackingRetentionDurableSettingsProof,
  type TrackingRetentionDurableSettingsProof,
} from '../../src/tracking-retention-durable-settings-proof';
import { buildTrackingRetentionLocalServiceStateProof } from '../../src/tracking-retention-local-service-state-proof';
import { AgentTrackingRetentionSettingsWriteDefaults } from '../../src/tracking-retention-settings-read-model-proof';

const GeneratedAt = '2026-06-07T10:20:00.000Z';
const SourceWriteCommandProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json';
const SourceLocalServiceStateProofRef =
  'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json';

describe('tracking retention durable settings proof', () => {
  it('derives local durable settings rows from local service state readback', () => {
    const proof = durableProof();

    expect(proof.proofMode).toBe('tracking-retention-durable-settings-proof');
    expect(proof.proofClaims).toEqual({
      localServiceStateReadbackClaimed: true,
      durablePersistenceRequirementVisible: true,
      localDurableSettingsPersisted: true,
      durabilityFailureVisible: false,
    });
    expect(proof.rows).toHaveLength(1);
    expectDurableSettingsRow(proof);
  });

  it('rejects missing durable persistence and product-ready overclaims', () => {
    const [row] = durableProof().rows;

    expect(
      TrackingRetentionDurableSettingsRowSchema.safeParse({
        ...row,
        durableSettingsPersisted: false,
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
    AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs[1]
  );
  expect(row.sourceMutationProofRefs).toContain(
    AgentTrackingRetentionSettingsWriteDefaults.MutationProofRef
  );
  expect(row.localServiceStateRevision).toBe(1);
  expect(row.localServiceStateSnapshotRef).toBe(
    AgentTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef
  );
  expect(row.durableSettingsStoreRef).toBe(AgentTrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef);
  expect(row.durableStoreRef).toBe(AgentTrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef);
  expect(row.durableSettingsPersisted).toBe(true);
  expect(row.durablePersistenceRequired).toBe(true);
  expect(row.durabilityFailureVisible).toBe(false);
  expect(row.productSettingsWritable).toBe(false);
  expect(row.productClaimReady).toBe(false);
}

function writeResult(): unknown {
  return {
    schemaVersion: 1,
    commandId: AgentTrackingRetentionSettingsWriteDefaults.CommandId,
    settingsKind: AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
    writeState: AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
    sourceWriterIntentRefs: [AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef],
    sourceReadModelProofRefs: AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs,
    sourceMutationProofRefs: [AgentTrackingRetentionSettingsWriteDefaults.MutationProofRef],
    appliedRetentionWindowHours: 168,
    appliedDeleteAfterAlertResolved: false,
    parentExportPrepared: false,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    localServiceStateRevision: 1,
    localServiceStateSnapshotRef: AgentTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef,
    durableSettingsStoreRef: AgentTrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef,
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
