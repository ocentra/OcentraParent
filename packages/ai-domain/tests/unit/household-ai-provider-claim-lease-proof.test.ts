import { describe, expect, it } from 'vitest';
import {
  HouseholdAiProviderClaimLeaseProof,
  HouseholdAiProviderClaimLeaseProofSchema,
} from '../../src/household-ai-provider-claim-lease-proof';

describe('household AI provider claim lease proof', () => {
  it('proves one active lease, duplicate rejection, expiry requeue, and dead-letter handling', () => {
    expect(HouseholdAiProviderClaimLeaseProof.validationSummary).toEqual({
      acceptedClaimCount: 1,
      duplicateClaimRejectedCount: 1,
      expiredRequeueCount: 1,
      deadLetterCount: 1,
      duplicateMessageIgnoredCount: 1,
      maxActiveLeaseCount: 1,
    });
    expect(HouseholdAiProviderClaimLeaseProof.leaseAttempts.map((row) => row.state)).toEqual([
      'claimed',
      'duplicate-rejected',
      'expired-requeued',
      'dead-lettered',
    ]);
    expect(HouseholdAiProviderClaimLeaseProof.messageReceipts[1]?.sideEffectApplied).toBe(false);
  });

  it('rejects proofs that allow more than one active provider lease for the same job', () => {
    const result = HouseholdAiProviderClaimLeaseProofSchema.safeParse({
      ...HouseholdAiProviderClaimLeaseProof,
      leaseAttempts: HouseholdAiProviderClaimLeaseProof.leaseAttempts.map((row, index) =>
        index === 1 ? { ...row, activeLeaseCountAfterDecision: 2 } : row
      ),
      validationSummary: {
        ...HouseholdAiProviderClaimLeaseProof.validationSummary,
        maxActiveLeaseCount: 2,
      },
    });

    expect(result.success).toBe(false);
  });

  it('rejects duplicate message rows that apply side effects', () => {
    const result = HouseholdAiProviderClaimLeaseProofSchema.safeParse({
      ...HouseholdAiProviderClaimLeaseProof,
      messageReceipts: HouseholdAiProviderClaimLeaseProof.messageReceipts.map((row) =>
        row.state === 'duplicate-ignored' ? { ...row, sideEffectApplied: true } : row
      ),
    });

    expect(result.success).toBe(false);
  });

  it('rejects runtime, policy, enforcement, raw-transfer, or remote/API overclaims', () => {
    const result = HouseholdAiProviderClaimLeaseProofSchema.safeParse({
      ...HouseholdAiProviderClaimLeaseProof,
      claimBoundaries: {
        ...HouseholdAiProviderClaimLeaseProof.claimBoundaries,
        physicalLanExecutionClaimed: true,
      },
    });

    expect(result.success).toBe(false);
  });
});
