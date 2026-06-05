import {
  type SupportBackendUploadAbandonState,
  type SupportBackendUploadAvailabilityState,
  type SupportBackendUploadDestination,
  type SupportBackendUploadRetryState,
  SupportBackendUploadRequiredDataClasses,
  SupportBackendUploadStatusEntrySchema,
  SupportBackendUploadStatusReadModelSchema,
  type SupportBackendUploadStatusEntry,
  type SupportBackendUploadStatusState,
} from './support-backend-upload-status.js';

type SupportBackendUploadStatusEntryInput = {
  uploadId: string;
  uploadStatus: SupportBackendUploadStatusState;
  backendAvailabilityState: SupportBackendUploadAvailabilityState;
  providerAvailabilityState: SupportBackendUploadAvailabilityState;
  retryState: SupportBackendUploadRetryState;
  abandonState: SupportBackendUploadAbandonState;
  allowedDestinations: readonly SupportBackendUploadDestination[];
  retryRefs: readonly string[];
  abandonRefs: readonly string[];
  failureRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-05T09:47:28.892Z';

export const SupportBackendUploadStatusReadModel = SupportBackendUploadStatusReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'support-backend-upload-status-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-support-upload-boundary',
    'release-installer-support-runbook-expectation',
    'support-incident-workflow-proof',
  ],
  entries: [
    supportBackendUploadStatusEntry({
      uploadId: 'support-upload-status-queued',
      uploadStatus: 'upload-queued',
      backendAvailabilityState: 'available',
      providerAvailabilityState: 'available',
      retryState: 'not-needed',
      abandonState: 'not-requested',
      allowedDestinations: ['support-safe-upload-status-boundary'],
      retryRefs: [],
      abandonRefs: [],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    supportBackendUploadStatusEntry({
      uploadId: 'support-upload-status-running',
      uploadStatus: 'upload-running',
      backendAvailabilityState: 'available',
      providerAvailabilityState: 'available',
      retryState: 'not-needed',
      abandonState: 'not-requested',
      allowedDestinations: ['support-safe-upload-status-boundary'],
      retryRefs: [],
      abandonRefs: [],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    supportBackendUploadStatusEntry({
      uploadId: 'support-upload-status-succeeded',
      uploadStatus: 'upload-succeeded',
      backendAvailabilityState: 'available',
      providerAvailabilityState: 'available',
      retryState: 'not-needed',
      abandonState: 'not-requested',
      allowedDestinations: ['support-safe-upload-status-boundary'],
      retryRefs: [],
      abandonRefs: [],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    supportBackendUploadStatusEntry({
      uploadId: 'support-upload-status-failed-abandoned',
      uploadStatus: 'upload-failed',
      backendAvailabilityState: 'available',
      providerAvailabilityState: 'available',
      retryState: 'retry-exhausted',
      abandonState: 'abandoned',
      allowedDestinations: ['support-safe-upload-status-boundary'],
      retryRefs: ['support-upload-retry-policy-ref', 'support-upload-retry-exhausted-audit-ref'],
      abandonRefs: ['parent-abandon-decision-ref', 'support-upload-abandon-audit-ref'],
      failureRefs: ['support-upload-failure-status-ref'],
      manualProofRequirements: [],
    }),
    supportBackendUploadStatusEntry({
      uploadId: 'support-upload-status-manual-required',
      uploadStatus: 'upload-manual-required',
      backendAvailabilityState: 'manual-required',
      providerAvailabilityState: 'manual-required',
      retryState: 'manual-required',
      abandonState: 'not-applicable',
      allowedDestinations: ['manual-support-backend'],
      retryRefs: ['support-upload-manual-retry-runbook-ref'],
      abandonRefs: [],
      failureRefs: ['support-upload-manual-required-status-ref'],
      manualProofRequirements: [
        'support backend upload implementation and operator runbook before upload can be claimed',
      ],
    }),
    supportBackendUploadStatusEntry({
      uploadId: 'support-upload-status-backend-unavailable',
      uploadStatus: 'backend-unavailable',
      backendAvailabilityState: 'unavailable',
      providerAvailabilityState: 'available',
      retryState: 'retry-queued',
      abandonState: 'not-requested',
      allowedDestinations: ['support-safe-upload-status-boundary'],
      retryRefs: ['support-upload-backend-retry-queue-ref'],
      abandonRefs: [],
      failureRefs: ['support-backend-unavailable-status-ref'],
      manualProofRequirements: [],
    }),
    supportBackendUploadStatusEntry({
      uploadId: 'support-upload-status-provider-unavailable',
      uploadStatus: 'provider-unavailable',
      backendAvailabilityState: 'available',
      providerAvailabilityState: 'unavailable',
      retryState: 'retry-queued',
      abandonState: 'not-requested',
      allowedDestinations: ['support-safe-upload-status-boundary'],
      retryRefs: ['support-upload-provider-retry-queue-ref'],
      abandonRefs: [],
      failureRefs: ['support-provider-unavailable-status-ref'],
      manualProofRequirements: [],
    }),
  ],
});

function supportBackendUploadStatusEntry(input: SupportBackendUploadStatusEntryInput): SupportBackendUploadStatusEntry {
  return SupportBackendUploadStatusEntrySchema.parse({
    schemaVersion: 1,
    parentInitiationState: 'parent-initiated',
    parentConsentState: 'parent-approved',
    executionClaimState: 'status-boundary-only',
    payloadState: 'redacted-status-and-audit-refs-only',
    custodyState: 'no-ocentra-hosted-family-data',
    disclosedDataClasses: [...SupportBackendUploadRequiredDataClasses],
    consentRefs: ['parent-support-upload-consent-artifact-ref'],
    redactionRefs: ['support-bundle-redaction-proof-ref', 'support-safe-upload-summary-ref'],
    auditRefs: ['support-upload-status-audit-event-ref', 'support-upload-custody-boundary-audit-ref'],
    backendRefs: ['support-backend-status-boundary-ref'],
    providerRefs: ['support-provider-status-boundary-ref'],
    containsTokens: false,
    containsRawChildActivity: false,
    containsRawUrls: false,
    containsScreenshots: false,
    containsJournals: false,
    containsSqliteSnapshots: false,
    containsPrivatePaths: false,
    containsCommandLines: false,
    containsKeystrokes: false,
    containsClipboardData: false,
    containsMessageContents: false,
    containsProviderSecrets: false,
    containsRemoteSupportTranscripts: false,
    realSupportBackendUploadExecuted: false,
    accountLookupExecuted: false,
    billingProviderExecuted: false,
    ocentraHostedFamilyDataDefault: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
