import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingAuthorityEnrollmentModes,
  TrackingAuthorityEnrollmentRowSchema,
  buildTrackingAuthorityEnrollmentManualRequiredProof,
} from '../../src/tracking-authority-enrollment-manual-required-proof';

describe('tracking authority enrollment manual required proof', () => {
  it('enumerates every authority enrollment mode without claiming authority', () => {
    const proof = buildTrackingAuthorityEnrollmentManualRequiredProof('2026-06-07T18:05:00.000Z');

    expect(proof.rows).toHaveLength(RequiredTrackingAuthorityEnrollmentModes.length);
    expect(proof.proofClaims.authorityEvidenceRequirementsEnumerated).toBe(true);
    expect(proof.proofClaims.noAuthorityClaim).toBe(true);
    expect(proof.productClaims.authorityEnrollmentClaimed).toBe(false);
    expect(proof.productClaims.hardControlRuntimeClaimed).toBe(false);
    expect(proof.productClaims.physicalDeviceClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
    expect(proof.rows.every((row) => row.requiredProofTier === 'P4_PHYSICAL_DEVICE')).toBe(true);
    expect(proof.rows.every((row) => row.currentProofTier === 'P0_CONTRACT')).toBe(true);
  });

  it('keeps mobile hard-control states authority-required until device evidence exists', () => {
    const proof = buildTrackingAuthorityEnrollmentManualRequiredProof('2026-06-07T18:05:00.000Z');
    const mobileRows = proof.rows.filter((row) => row.platform === 'android' || row.platform === 'ios');

    expect(mobileRows).toHaveLength(4);
    expect(mobileRows.every((row) => row.state === 'authority-required')).toBe(true);
    expect(mobileRows.every((row) => row.requiredEvidenceRefs.length >= 4)).toBe(true);
    expect(mobileRows.every((row) => row.authorityEnrollmentClaimed === false)).toBe(true);
    expect(mobileRows.every((row) => row.hardControlRuntimeClaimed === false)).toBe(true);
  });

  it('rejects authority rows that claim product readiness', () => {
    const invalid = TrackingAuthorityEnrollmentRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-authority-invalid',
      generatedAt: '2026-06-07T18:05:00.000Z',
      platform: 'android',
      enrollmentMode: 'android-device-owner',
      state: 'authority-required',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: 'P0_CONTRACT',
      requiredEvidenceRefs: [
        'tracking-authority-invalid-device-identity-proof',
        'tracking-authority-invalid-enrollment-state-proof',
        'tracking-authority-invalid-approved-capability-proof',
      ],
      manualProofCommand: 'collect invalid authority evidence',
      auditRefs: ['tracking-authority-invalid-audit'],
      authorityEnrollmentClaimed: true,
      hardControlRuntimeClaimed: false,
      physicalDeviceClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});
