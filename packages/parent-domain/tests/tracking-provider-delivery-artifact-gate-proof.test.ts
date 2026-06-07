import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingProviderDeliveryArtifactPlan,
  TrackingProviderDeliveryArtifactGateRowSchema,
  buildTrackingProviderDeliveryArtifactGateProof,
} from '../src/tracking-provider-delivery-artifact-gate-proof';

describe('tracking provider delivery artifact gate proof', () => {
  it('keeps provider delivery manual-required when artifacts are missing', () => {
    const proof = buildTrackingProviderDeliveryArtifactGateProof('2026-06-07T19:20:00.000Z', {
      presentArtifacts: [],
    });

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('manual-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_MANUAL_PROVIDER_RUNTIME');
    expect(proof.rows[0].currentProofTier).toBe('P3_LOCAL_DEV_MACHINE');
    expect(proof.rows[0].providerDeliveryArtifactSetComplete).toBe(false);
    expect(proof.rows[0].missingArtifacts).toEqual([...RequiredTrackingProviderDeliveryArtifactPlan.requiredArtifacts]);
    expect(proof.productClaims.providerDeliveryRuntimeClaimed).toBe(false);
    expect(proof.productClaims.webhookReceiptIngestionRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('marks artifact-set-present only when every required artifact is present', () => {
    const proof = buildTrackingProviderDeliveryArtifactGateProof('2026-06-07T19:20:00.000Z', {
      presentArtifacts: RequiredTrackingProviderDeliveryArtifactPlan.requiredArtifacts,
    });

    expect(proof.rows[0].status).toBe('artifact-set-present');
    expect(proof.rows[0].providerDeliveryArtifactSetComplete).toBe(true);
    expect(proof.rows[0].missingArtifacts).toEqual([]);
    expect(proof.productClaims.providerCredentialsClaimed).toBe(false);
    expect(proof.productClaims.parentNotificationUiRuntimeClaimed).toBe(false);
  });

  it('rejects rows that claim provider delivery without the artifact set', () => {
    const invalid = TrackingProviderDeliveryArtifactGateRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-provider-delivery-artifacts-invalid',
      generatedAt: '2026-06-07T19:20:00.000Z',
      proofRoot: RequiredTrackingProviderDeliveryArtifactPlan.proofRoot,
      requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      status: 'manual-required',
      requiredArtifacts: ['00-run-metadata.json'],
      presentArtifacts: [],
      missingArtifacts: ['00-run-metadata.json'],
      auditRefs: ['tracking-provider-delivery-artifacts-invalid-audit'],
      providerDeliveryArtifactSetComplete: false,
      providerDeliveryRuntimeClaimed: true,
      webhookReceiptIngestionRuntimeClaimed: false,
      providerCredentialsClaimed: false,
      adapterDispatchClaimed: false,
      retryExecutionRuntimeClaimed: false,
      quietHoursTimerRuntimeClaimed: false,
      parentNotificationUiRuntimeClaimed: false,
      productionDurableOutboxStorageClaimed: false,
      childDeviceDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});
