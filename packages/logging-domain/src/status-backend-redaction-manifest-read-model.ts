import {
  type StatusBackendRedactionManifestEntry,
  StatusBackendRedactionManifestEntrySchema,
  StatusBackendRedactionManifestReadModelSchema,
  type StatusBackendRedactionManifestReadinessState,
  type StatusBackendRedactionManifestReviewState,
  StatusBackendRedactionManifestRequiredDataClasses,
  type StatusBackendRedactionManifestState,
} from './status-backend-redaction-manifest.js';

type StatusBackendRedactionManifestEntryInput = {
  manifestId: string;
  manifestState: StatusBackendRedactionManifestState;
  redactionManifestState: StatusBackendRedactionManifestReadinessState;
  redactionReviewState: StatusBackendRedactionManifestReviewState;
  redactionSummaryRefs: readonly string[];
  redactionReviewRefs: readonly string[];
  failureRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-07T09:50:00.000Z';

export const StatusBackendRedactionManifestReadModel = StatusBackendRedactionManifestReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-status-backend-redaction-manifest-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-status-backend-redaction-manifest-boundary',
    'release-installer-status-backend-redaction-manifest-expectation',
    'production-support-status-backend-runtime-execution-proof',
    'production-support-status-backend-payload-custody-proof',
  ],
  entries: [
    statusBackendRedactionManifestEntry({
      manifestId: 'status-backend-redaction-manifest-ready',
      manifestState: 'redaction-manifest-ready',
      redactionManifestState: 'support-safe-manifest-ready',
      redactionReviewState: 'reviewed',
      redactionSummaryRefs: ['status-backend-redaction-summary-reviewed-ref'],
      redactionReviewRefs: ['status-backend-redaction-review-approved-ref'],
      failureRefs: ['status-backend-redaction-manifest-no-failure-ref'],
      manualProofRequirements: ['publish redaction manifest review before status backend execution can be claimed'],
    }),
    statusBackendRedactionManifestEntry({
      manifestId: 'status-backend-redaction-manifest-manual-required',
      manifestState: 'redaction-manifest-manual-required',
      redactionManifestState: 'manual-required',
      redactionReviewState: 'manual-required',
      redactionSummaryRefs: ['status-backend-redaction-summary-manual-ref'],
      redactionReviewRefs: ['status-backend-redaction-review-manual-ref'],
      failureRefs: ['status-backend-redaction-manual-required-ref'],
      manualProofRequirements: ['manual redaction manifest proof before status backend payload custody can be claimed'],
    }),
    statusBackendRedactionManifestEntry({
      manifestId: 'status-backend-redaction-review-queued',
      manifestState: 'redaction-review-queued',
      redactionManifestState: 'manual-required',
      redactionReviewState: 'queued',
      redactionSummaryRefs: ['status-backend-redaction-summary-queued-ref'],
      redactionReviewRefs: ['status-backend-redaction-review-queue-ref'],
      failureRefs: ['status-backend-redaction-review-queue-pending-ref'],
      manualProofRequirements: ['redaction review queue proof before status backend execution can be claimed'],
    }),
    statusBackendRedactionManifestEntry({
      manifestId: 'status-backend-redaction-review-running',
      manifestState: 'redaction-review-running',
      redactionManifestState: 'manual-required',
      redactionReviewState: 'running',
      redactionSummaryRefs: ['status-backend-redaction-summary-running-ref'],
      redactionReviewRefs: ['status-backend-redaction-review-running-ref'],
      failureRefs: ['status-backend-redaction-review-running-pending-ref'],
      manualProofRequirements: ['redaction review runtime proof before support backend upload can be claimed'],
    }),
    statusBackendRedactionManifestEntry({
      manifestId: 'status-backend-redaction-review-failed',
      manifestState: 'redaction-review-failed',
      redactionManifestState: 'manual-required',
      redactionReviewState: 'failed',
      redactionSummaryRefs: ['status-backend-redaction-summary-failed-ref'],
      redactionReviewRefs: ['status-backend-redaction-review-failed-ref'],
      failureRefs: ['status-backend-redaction-review-failure-ref'],
      manualProofRequirements: ['failed redaction review triage before any backend upload can be claimed'],
    }),
    statusBackendRedactionManifestEntry({
      manifestId: 'status-backend-redaction-backend-unavailable',
      manifestState: 'backend-unavailable',
      redactionManifestState: 'manual-required',
      redactionReviewState: 'manual-required',
      redactionSummaryRefs: ['status-backend-redaction-summary-unavailable-ref'],
      redactionReviewRefs: ['status-backend-redaction-review-unavailable-ref'],
      failureRefs: ['status-backend-redaction-backend-unavailable-ref'],
      manualProofRequirements: ['status backend unavailable fallback proof before manifest execution can be claimed'],
    }),
  ],
});

function statusBackendRedactionManifestEntry(
  input: StatusBackendRedactionManifestEntryInput
): StatusBackendRedactionManifestEntry {
  return StatusBackendRedactionManifestEntrySchema.parse({
    schemaVersion: 1,
    parentConsentState: 'parent-approved',
    executionClaimState: 'status-backend-redaction-manifest-boundary-only',
    payloadState: 'redacted-status-refs-only',
    disclosedDataClasses: [...StatusBackendRedactionManifestRequiredDataClasses],
    consentRefs: ['parent-status-backend-redaction-manifest-consent-ref'],
    targetRefs: ['production-support-status-backend-runtime-execution-proof-ref'],
    queueRefs: ['production-support-status-backend-execution-queue-proof-ref'],
    auditRefs: ['production-support-status-backend-queue-audit-persistence-proof-ref'],
    redactionManifestRefs: ['support-bundle-redaction-proof-ref', 'status-backend-redaction-manifest-ref'],
    containsTokens: false,
    containsRawChildActivity: false,
    containsRawSupportBundles: false,
    containsProviderSecrets: false,
    containsAccountLookupResults: false,
    containsBillingContactRecords: false,
    containsBackendUploadPayloads: false,
    containsStatusBackendPayloads: false,
    containsPublicRuntimePayloads: false,
    containsRemoteSupportTranscripts: false,
    realStatusBackendExecution: false,
    statusBackendPayloadCustodyClaimed: false,
    durableStatusBackendPayloadStorage: false,
    statusBackendPayloadDeletionExecuted: false,
    retryWorkerExecution: false,
    auditPersistenceExecuted: false,
    publicRuntimeExecution: false,
    supportBackendUploadExecution: false,
    providerExecution: false,
    accountLookupExecuted: false,
    billingProviderContactExecuted: false,
    remoteSupportSessionExecuted: false,
    productionSlaClaimed: false,
    ocentraHostedFamilyDataDefault: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
