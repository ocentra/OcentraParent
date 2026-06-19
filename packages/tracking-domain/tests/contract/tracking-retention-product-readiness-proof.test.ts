import { describe, expect, it } from 'vitest';
import {
  ProductReadinessBlockers,
  TrackingRetentionProductReadinessRowSchema,
  buildTrackingRetentionProductReadinessProof,
  type TrackingRetentionProductReadinessProof,
} from '../../src/tracking-retention-product-readiness-proof';
import { buildTrackingRetentionDurableSettingsProof } from '../../src/tracking-retention-durable-settings-proof';
import { buildTrackingRetentionLocalServiceStateProof } from '../../src/tracking-retention-local-service-state-proof';
import {
  TrackingRetentionProofRefs,
  trackingRetentionAcceptedLocalServiceWriteResult,
} from '../../src/tracking-retention-proof-catalog';

const GeneratedAt = '2026-06-07T15:30:00.000Z';
const SourceWriteCommandProofRef = TrackingRetentionProofRefs.WriteCommand;
const SourceLocalServiceStateProofRef = TrackingRetentionProofRefs.LocalServiceState;
const SourceDurableSettingsProofRef = TrackingRetentionProofRefs.DurableSettings;

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
      buildTrackingRetentionLocalServiceStateProof(
        GeneratedAt,
        SourceWriteCommandProofRef,
        trackingRetentionAcceptedLocalServiceWriteResult()
      )
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
