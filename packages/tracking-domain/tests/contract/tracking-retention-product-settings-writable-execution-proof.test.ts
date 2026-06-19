import { describe, expect, it } from 'vitest';
import { AgentTrackingRetentionSettingsWriteDefaults } from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import { buildTrackingRetentionLocalServiceStateProof } from '../../src/tracking-retention-local-service-state-proof';
import {
  TrackingRetentionProductSettingsWritableExecutionArtifactRef,
  TrackingRetentionProductSettingsWritableExecutionRowSchema,
  buildTrackingRetentionProductSettingsWritableExecutionProof,
} from '../../src/tracking-retention-product-settings-writable-execution-proof';
import {
  TrackingRetentionProofRefs,
  trackingRetentionAcceptedLocalServiceWriteResult,
} from '../../src/tracking-retention-proof-catalog';

const GeneratedAt = '2026-06-08T03:45:00.000Z';
const SourceLocalServiceStateProofRef = TrackingRetentionProofRefs.LocalServiceState;
const SourceWriteCommandProofRef = TrackingRetentionProofRefs.WriteCommand;

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
    expect(row.localServiceStateSnapshotRef).toBe(
      AgentTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef
    );
    expect(row.durableSettingsStoreRef).toBe(AgentTrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef);
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
    localServiceStateSnapshotRef: AgentTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef,
    durableSettingsStoreRef: AgentTrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef,
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
  return buildTrackingRetentionLocalServiceStateProof(
    GeneratedAt,
    SourceWriteCommandProofRef,
    trackingRetentionAcceptedLocalServiceWriteResult()
  );
}
