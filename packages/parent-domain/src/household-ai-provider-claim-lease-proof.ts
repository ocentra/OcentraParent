import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const ClaimLeaseTextSchema = Schema.String.pipe(Schema.minLength(1));
const ClaimLeaseCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const ClaimLeasePositiveCountSchema = Schema.Number.pipe(Schema.positive(), Schema.int());

export const HouseholdAiProviderClaimLeaseProofIdSchema = ClaimLeaseTextSchema.pipe(
  Schema.brand('HouseholdAiProviderClaimLeaseProofId')
);

export const HouseholdAiProviderLeaseStateSchema = withParser(
  Schema.Literal('queued', 'claimed', 'duplicate-rejected', 'expired-requeued', 'dead-lettered')
);

export const HouseholdAiProviderMessageStateSchema = withParser(
  Schema.Literal('accepted', 'duplicate-ignored', 'stale-rejected')
);

export const HouseholdAiProviderClaimLeaseBoundarySchema = withParser(
  Schema.Struct({
    physicalLanExecutionClaimed: Schema.Boolean,
    modelExecutionClaimed: Schema.Boolean,
    modelQualityClaimed: Schema.Boolean,
    policyAuthorityClaimed: Schema.Boolean,
    enforcementClaimed: Schema.Boolean,
    rawScreenshotTransferred: Schema.Boolean,
    remoteApiAiUsed: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (boundary) =>
        Object.values(boundary).every((claim) => claim === false) ||
        'Expected household AI provider claim lease proof to keep runtime, model, policy, enforcement, raw-transfer, and remote/API claims false'
    )
  )
);

const HouseholdAiProviderLeaseAttemptSchema = Schema.Struct({
  attemptId: ClaimLeaseTextSchema,
  jobId: ClaimLeaseTextSchema,
  providerId: ClaimLeaseTextSchema,
  claimId: ClaimLeaseTextSchema,
  leaseId: ClaimLeaseTextSchema,
  state: HouseholdAiProviderLeaseStateSchema,
  leaseExpiresAt: ClaimLeaseTextSchema,
  attemptNumber: ClaimLeasePositiveCountSchema,
  acceptedClaim: Schema.Boolean,
  activeLeaseCountAfterDecision: ClaimLeaseCountSchema,
  rejectionReason: Schema.NullOr(ClaimLeaseTextSchema),
});

const HouseholdAiProviderMessageReceiptSchema = Schema.Struct({
  messageId: ClaimLeaseTextSchema,
  jobId: ClaimLeaseTextSchema,
  providerId: ClaimLeaseTextSchema,
  state: HouseholdAiProviderMessageStateSchema,
  idempotencyKey: ClaimLeaseTextSchema,
  sideEffectApplied: Schema.Boolean,
});

const HouseholdAiProviderClaimLeaseProofBaseSchema = Schema.Struct({
  proofId: HouseholdAiProviderClaimLeaseProofIdSchema,
  generatedAt: ClaimLeaseTextSchema,
  childAgentId: ClaimLeaseTextSchema,
  jobId: ClaimLeaseTextSchema,
  workKind: Schema.Literal('screen-ai-analysis'),
  custodyRef: ClaimLeaseTextSchema,
  redactedPayloadRef: ClaimLeaseTextSchema,
  maxAttempts: ClaimLeasePositiveCountSchema,
  leaseTtlMs: ClaimLeasePositiveCountSchema,
  leaseAttempts: Schema.Array(HouseholdAiProviderLeaseAttemptSchema),
  messageReceipts: Schema.Array(HouseholdAiProviderMessageReceiptSchema),
  validationSummary: Schema.Struct({
    acceptedClaimCount: ClaimLeaseCountSchema,
    duplicateClaimRejectedCount: ClaimLeaseCountSchema,
    expiredRequeueCount: ClaimLeaseCountSchema,
    deadLetterCount: ClaimLeaseCountSchema,
    duplicateMessageIgnoredCount: ClaimLeaseCountSchema,
    maxActiveLeaseCount: ClaimLeaseCountSchema,
  }),
  claimBoundaries: HouseholdAiProviderClaimLeaseBoundarySchema,
});

type HouseholdAiProviderClaimLeaseProofCandidate = Infer<typeof HouseholdAiProviderClaimLeaseProofBaseSchema>;

export const HouseholdAiProviderClaimLeaseProofSchema = withParser(
  HouseholdAiProviderClaimLeaseProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        householdAiProviderClaimLeaseProofIsReady(proof) ||
        'Expected household AI provider claim lease proof to prove one active lease, duplicate rejection, expiry requeue, dead-letter, and idempotent duplicate messages'
    )
  )
);

export type HouseholdAiProviderClaimLeaseProof = Infer<typeof HouseholdAiProviderClaimLeaseProofSchema>;
export type HouseholdAiProviderClaimLeaseBoundary = Infer<typeof HouseholdAiProviderClaimLeaseBoundarySchema>;

const generatedAt = '2026-06-08T04:45:00.000Z';
const jobId = 'screen-ai-job-child-pc-claim-lease-1';
const custodyRef = 'screen-ai-custody-deleted-image-query-store-1';
const providerId = 'household-provider-parent-desktop-1';

export const HouseholdAiProviderClaimLeaseProof = HouseholdAiProviderClaimLeaseProofSchema.parse({
  proofId: 'household-ai-provider-claim-lease-proof',
  generatedAt,
  childAgentId: 'child-agent-sam-windows-pc',
  jobId,
  workKind: 'screen-ai-analysis',
  custodyRef,
  redactedPayloadRef: 'redacted-screen-summary-crop-ref-1',
  maxAttempts: 3,
  leaseTtlMs: 30_000,
  leaseAttempts: [
    {
      attemptId: 'attempt-1',
      jobId,
      providerId,
      claimId: 'claim-parent-desktop-1',
      leaseId: 'lease-parent-desktop-1',
      state: 'claimed',
      leaseExpiresAt: generatedAt,
      attemptNumber: 1,
      acceptedClaim: true,
      activeLeaseCountAfterDecision: 1,
      rejectionReason: null,
    },
    {
      attemptId: 'attempt-duplicate',
      jobId,
      providerId: 'household-provider-laptop-1',
      claimId: 'claim-laptop-duplicate-1',
      leaseId: 'lease-laptop-duplicate-1',
      state: 'duplicate-rejected',
      leaseExpiresAt: generatedAt,
      attemptNumber: 1,
      acceptedClaim: false,
      activeLeaseCountAfterDecision: 1,
      rejectionReason: 'job-already-leased',
    },
    {
      attemptId: 'attempt-expired-requeue',
      jobId,
      providerId,
      claimId: 'claim-parent-desktop-expired-1',
      leaseId: 'lease-parent-desktop-expired-1',
      state: 'expired-requeued',
      leaseExpiresAt: generatedAt,
      attemptNumber: 2,
      acceptedClaim: false,
      activeLeaseCountAfterDecision: 0,
      rejectionReason: 'lease-expired',
    },
    {
      attemptId: 'attempt-dead-letter',
      jobId,
      providerId,
      claimId: 'claim-parent-desktop-final-1',
      leaseId: 'lease-parent-desktop-final-1',
      state: 'dead-lettered',
      leaseExpiresAt: generatedAt,
      attemptNumber: 3,
      acceptedClaim: false,
      activeLeaseCountAfterDecision: 0,
      rejectionReason: 'max-attempts-exhausted',
    },
  ],
  messageReceipts: [
    {
      messageId: 'mesh-message-1',
      jobId,
      providerId,
      state: 'accepted',
      idempotencyKey: `${jobId}:mesh-message-1`,
      sideEffectApplied: true,
    },
    {
      messageId: 'mesh-message-1',
      jobId,
      providerId,
      state: 'duplicate-ignored',
      idempotencyKey: `${jobId}:mesh-message-1`,
      sideEffectApplied: false,
    },
    {
      messageId: 'mesh-message-stale-1',
      jobId,
      providerId,
      state: 'stale-rejected',
      idempotencyKey: `${jobId}:mesh-message-stale-1`,
      sideEffectApplied: false,
    },
  ],
  validationSummary: {
    acceptedClaimCount: 1,
    duplicateClaimRejectedCount: 1,
    expiredRequeueCount: 1,
    deadLetterCount: 1,
    duplicateMessageIgnoredCount: 1,
    maxActiveLeaseCount: 1,
  },
  claimBoundaries: {
    physicalLanExecutionClaimed: false,
    modelExecutionClaimed: false,
    modelQualityClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    rawScreenshotTransferred: false,
    remoteApiAiUsed: false,
  },
});

function householdAiProviderClaimLeaseProofIsReady(proof: HouseholdAiProviderClaimLeaseProofCandidate): boolean {
  return (
    oneActiveLeaseAtMost(proof) &&
    summaryMatchesRows(proof) &&
    duplicateMessagesAreIdempotent(proof) &&
    proof.workKind === 'screen-ai-analysis' &&
    proof.custodyRef.length > 0 &&
    proof.redactedPayloadRef.length > 0
  );
}

function oneActiveLeaseAtMost(proof: HouseholdAiProviderClaimLeaseProofCandidate): boolean {
  return proof.leaseAttempts.every((attempt) => attempt.activeLeaseCountAfterDecision <= 1);
}

function summaryMatchesRows(proof: HouseholdAiProviderClaimLeaseProofCandidate): boolean {
  return (
    proof.validationSummary.acceptedClaimCount ===
      proof.leaseAttempts.filter((row) => row.state === 'claimed').length &&
    proof.validationSummary.duplicateClaimRejectedCount ===
      proof.leaseAttempts.filter((row) => row.state === 'duplicate-rejected').length &&
    proof.validationSummary.expiredRequeueCount ===
      proof.leaseAttempts.filter((row) => row.state === 'expired-requeued').length &&
    proof.validationSummary.deadLetterCount ===
      proof.leaseAttempts.filter((row) => row.state === 'dead-lettered').length &&
    proof.validationSummary.maxActiveLeaseCount ===
      Math.max(...proof.leaseAttempts.map((row) => row.activeLeaseCountAfterDecision))
  );
}

function duplicateMessagesAreIdempotent(proof: HouseholdAiProviderClaimLeaseProofCandidate): boolean {
  const duplicateRows = proof.messageReceipts.filter((row) => row.state === 'duplicate-ignored');
  return (
    proof.validationSummary.duplicateMessageIgnoredCount === duplicateRows.length &&
    duplicateRows.every((row) => row.sideEffectApplied === false)
  );
}

export const decodeHouseholdAiProviderClaimLeaseProof = Schema.decodeUnknownSync(
  HouseholdAiProviderClaimLeaseProofSchema
);
