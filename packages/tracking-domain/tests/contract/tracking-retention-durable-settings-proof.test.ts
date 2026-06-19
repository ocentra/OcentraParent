import { describe, expect, it } from 'vitest';
import { AgentTrackingRetentionSettingsWriteDefaults } from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import {
  TrackingRetentionDurableSettingsRowSchema,
  buildTrackingRetentionDurableSettingsProof,
  type TrackingRetentionDurableSettingsProof,
} from '../../src/tracking-retention-durable-settings-proof';
import { buildTrackingRetentionLocalServiceStateProof } from '../../src/tracking-retention-local-service-state-proof';
import {
  TrackingRetentionProofRefs,
  trackingRetentionAcceptedLocalServiceWriteResult,
} from '../../src/tracking-retention-proof-catalog';

const GeneratedAt = '2026-06-07T10:20:00.000Z';
const SourceWriteCommandProofRef = TrackingRetentionProofRefs.WriteCommand;
const SourceLocalServiceStateProofRef = TrackingRetentionProofRefs.LocalServiceState;

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
    buildTrackingRetentionLocalServiceStateProof(
      GeneratedAt,
      SourceWriteCommandProofRef,
      trackingRetentionAcceptedLocalServiceWriteResult()
    )
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
