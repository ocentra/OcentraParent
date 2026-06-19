import { describe, expect, it } from 'vitest';
import { buildTrackingRetentionLocalServiceStateProof } from '../../src/tracking-retention-local-service-state-proof';
import { AgentTrackingRetentionSettingsWriteDefaults } from '../../src/tracking-retention-settings-read-model-proof';
import {
  TrackingRetentionAppliedSettingsRuntimeBridgeRowSchema,
  buildTrackingRetentionAppliedSettingsRuntimeBridgeProof,
} from '../../src/tracking-retention-applied-settings-runtime-bridge-proof';
import { buildTrackingRetentionProductSettingsWritableExecutionProof } from '../../src/tracking-retention-product-settings-writable-execution-proof';
import {
  TrackingRetentionProofRefs,
  trackingRetentionAcceptedLocalServiceWriteResult,
} from '../../src/tracking-retention-proof-catalog';

const GeneratedAt = '2026-06-08T20:10:00.000Z';
const SourceLocalServiceStateProofRef = TrackingRetentionProofRefs.LocalServiceState;
const SourceWritableExecutionProofRef = TrackingRetentionProofRefs.ProductSettingsWritableExecution;

describe('tracking retention applied settings runtime bridge proof', () => {
  it('bridges local applied settings into runtime artifact accounting without claiming product readiness', () => {
    const proof = buildTrackingRetentionAppliedSettingsRuntimeBridgeProof(
      GeneratedAt,
      SourceWritableExecutionProofRef,
      writableExecutionProof()
    );

    expect(proof.proofMode).toBe('tracking-retention-applied-settings-runtime-bridge-proof');
    expect(proof.proofClaims).toEqual({
      writableExecutionProofConsumed: true,
      localAppliedSettingsObserved: true,
      localDurableSettingsPersisted: true,
      runtimeArtifactAccountingUpdated: true,
      platformRuntimeRetentionEnforcementMissing: true,
      noProductReadyClaim: true,
    });
    expect(proof.runtimeArtifactInventory.presentArtifacts).toEqual([
      'tracking-retention/product-settings-writable-execution.json',
    ]);
    expect(proof.runtimeArtifactInventory.missingArtifacts).toEqual([
      'tracking-retention/platform-runtime-retention-enforcement.json',
    ]);
    expect(proof.runtimeArtifactInventory.artifactSetComplete).toBe(false);
    expect(Object.values(proof.productClaims).every((claim) => claim === false)).toBe(true);

    const [row] = proof.rows;
    expect(row.sourceWritableExecutionProofRef).toBe(SourceWritableExecutionProofRef);
    expect(row.sourceLocalServiceStateProofRef).toBe(SourceLocalServiceStateProofRef);
    expect(row.localServiceStateRevision).toBe(1);
    expect(row.durableSettingsStoreRef).toBe(AgentTrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef);
    expect(row.appliedRetentionWindowHours).toBe(168);
    expect(row.localAppliedSettingsObserved).toBe(true);
    expect(row.writableExecutionArtifactPresent).toBe(true);
    expect(row.platformRuntimeRetentionEnforcementPresent).toBe(false);
    expect(row.productClaimReady).toBe(false);
  });

  it('rejects product-ready or platform runtime overclaims on bridge rows', () => {
    const [row] = buildTrackingRetentionAppliedSettingsRuntimeBridgeProof(
      GeneratedAt,
      SourceWritableExecutionProofRef,
      writableExecutionProof()
    ).rows;

    expect(
      TrackingRetentionAppliedSettingsRuntimeBridgeRowSchema.safeParse({
        ...row,
        productClaimReady: true,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionAppliedSettingsRuntimeBridgeRowSchema.safeParse({
        ...row,
        platformRuntimeRetentionEnforcementPresent: true,
      }).success
    ).toBe(false);
    expect(
      TrackingRetentionAppliedSettingsRuntimeBridgeRowSchema.safeParse({
        ...row,
        missingRuntimeArtifacts: [],
      }).success
    ).toBe(false);
  });
});

function writableExecutionProof(): unknown {
  return buildTrackingRetentionProductSettingsWritableExecutionProof(
    GeneratedAt,
    SourceLocalServiceStateProofRef,
    localServiceStateProof()
  );
}

function localServiceStateProof(): unknown {
  return buildTrackingRetentionLocalServiceStateProof(
    GeneratedAt,
    TrackingRetentionProofRefs.WriteCommand,
    trackingRetentionAcceptedLocalServiceWriteResult()
  );
}
