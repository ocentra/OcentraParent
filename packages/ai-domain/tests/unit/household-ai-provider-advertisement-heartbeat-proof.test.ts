import { describe, expect, it } from 'vitest';
import {
  HouseholdAiProviderAdvertisementHeartbeatProof,
  HouseholdAiProviderAdvertisementHeartbeatProofSchema,
} from '../../src/household-ai-provider-advertisement-heartbeat-proof';

describe('household AI provider advertisement heartbeat proof', () => {
  it('accepts only fresh trusted local providers with the requested screen capability', () => {
    expect(HouseholdAiProviderAdvertisementHeartbeatProof.validationSummary).toEqual({
      eligibleProviderCount: 1,
      staleProviderRejectedCount: 1,
      offlineProviderRejectedCount: 1,
      revokedProviderRejectedCount: 1,
      unsupportedProviderRejectedCount: 1,
      rawPayloadAdvertisementCount: 0,
      remoteApiAdvertisementCount: 0,
    });
    expect(HouseholdAiProviderAdvertisementHeartbeatProof.advertisements[0]?.state).toBe('eligible');
    expect(HouseholdAiProviderAdvertisementHeartbeatProof.advertisements[0]?.capabilities).toContain(
      HouseholdAiProviderAdvertisementHeartbeatProof.requestedCapability
    );
  });

  it('rejects stale rows that are marked eligible', () => {
    const result = HouseholdAiProviderAdvertisementHeartbeatProofSchema.safeParse({
      ...HouseholdAiProviderAdvertisementHeartbeatProof,
      advertisements: HouseholdAiProviderAdvertisementHeartbeatProof.advertisements.map((row, index) =>
        index === 1 ? { ...row, state: 'eligible', rejectionReason: null } : row
      ),
      validationSummary: {
        ...HouseholdAiProviderAdvertisementHeartbeatProof.validationSummary,
        eligibleProviderCount: 2,
        staleProviderRejectedCount: 0,
      },
    });

    expect(result.success).toBe(false);
  });

  it('rejects unsupported providers marked eligible for screen AI work', () => {
    const result = HouseholdAiProviderAdvertisementHeartbeatProofSchema.safeParse({
      ...HouseholdAiProviderAdvertisementHeartbeatProof,
      advertisements: HouseholdAiProviderAdvertisementHeartbeatProof.advertisements.map((row, index) =>
        index === 4 ? { ...row, state: 'eligible', rejectionReason: null } : row
      ),
      validationSummary: {
        ...HouseholdAiProviderAdvertisementHeartbeatProof.validationSummary,
        eligibleProviderCount: 2,
        unsupportedProviderRejectedCount: 0,
      },
    });

    expect(result.success).toBe(false);
  });

  it('rejects raw screenshot or remote API advertisement overclaims', () => {
    const result = HouseholdAiProviderAdvertisementHeartbeatProofSchema.safeParse({
      ...HouseholdAiProviderAdvertisementHeartbeatProof,
      claimBoundaries: {
        ...HouseholdAiProviderAdvertisementHeartbeatProof.claimBoundaries,
        rawScreenshotTransferred: true,
      },
    });

    expect(result.success).toBe(false);
  });
});
