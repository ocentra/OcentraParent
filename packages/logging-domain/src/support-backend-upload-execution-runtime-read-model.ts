import {
  type SupportBackendUploadExecutionRuntimeAbandonState,
  type SupportBackendUploadExecutionRuntimeAvailabilityState,
  SupportBackendUploadExecutionRuntimeEntrySchema,
  SupportBackendUploadExecutionRuntimeReadModelSchema,
  SupportBackendUploadExecutionRuntimeRequiredDataClasses,
  type SupportBackendUploadExecutionRuntimeEntry,
  type SupportBackendUploadExecutionRuntimeRetryState,
  type SupportBackendUploadExecutionRuntimeState,
} from './support-backend-upload-execution-runtime.js';

type SupportBackendUploadExecutionRuntimeEntryInput = {
  runtimeId: string;
  runtimeState: SupportBackendUploadExecutionRuntimeState;
  backendAvailabilityState: SupportBackendUploadExecutionRuntimeAvailabilityState;
  providerAvailabilityState: SupportBackendUploadExecutionRuntimeAvailabilityState;
  retryState: SupportBackendUploadExecutionRuntimeRetryState;
  abandonState: SupportBackendUploadExecutionRuntimeAbandonState;
  retryRefs: readonly string[];
  abandonRefs: readonly string[];
  failureRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-05T14:24:28.488Z';

export const SupportBackendUploadExecutionRuntimeReadModel = SupportBackendUploadExecutionRuntimeReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'support-backend-upload-execution-runtime-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-support-upload-boundary',
    'release-installer-support-backend-upload-expectation',
    'production-support-backend-upload-status-proof',
    'production-support-publication-workflow-proof',
  ],
  entries: [
    supportBackendUploadExecutionRuntimeEntry({
      runtimeId: 'support-upload-execution-request-recorded',
      runtimeState: 'execution-request-recorded',
      backendAvailabilityState: 'available',
      providerAvailabilityState: 'available',
      retryState: 'not-needed',
      abandonState: 'not-requested',
      retryRefs: [],
      abandonRefs: [],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    supportBackendUploadExecutionRuntimeEntry({
      runtimeId: 'support-upload-redaction-preflight-ready',
      runtimeState: 'redaction-preflight-ready',
      backendAvailabilityState: 'available',
      providerAvailabilityState: 'available',
      retryState: 'not-needed',
      abandonState: 'not-requested',
      retryRefs: [],
      abandonRefs: [],
      failureRefs: [],
      manualProofRequirements: [],
    }),
    supportBackendUploadExecutionRuntimeEntry({
      runtimeId: 'support-upload-dispatch-manual-required',
      runtimeState: 'dispatch-manual-required',
      backendAvailabilityState: 'manual-required',
      providerAvailabilityState: 'manual-required',
      retryState: 'manual-required',
      abandonState: 'not-applicable',
      retryRefs: ['support-upload-runtime-manual-retry-runbook-ref'],
      abandonRefs: [],
      failureRefs: ['support-upload-runtime-manual-required-status-ref'],
      manualProofRequirements: [
        'support backend upload adapter implementation before execution can be claimed',
        'operator runbook and retention/delete proof before production upload can be claimed',
      ],
    }),
    supportBackendUploadExecutionRuntimeEntry({
      runtimeId: 'support-upload-execution-backend-unavailable',
      runtimeState: 'backend-unavailable',
      backendAvailabilityState: 'unavailable',
      providerAvailabilityState: 'available',
      retryState: 'retry-scheduled',
      abandonState: 'not-requested',
      retryRefs: ['support-upload-runtime-backend-retry-schedule-ref'],
      abandonRefs: [],
      failureRefs: ['support-upload-runtime-backend-unavailable-status-ref'],
      manualProofRequirements: [],
    }),
    supportBackendUploadExecutionRuntimeEntry({
      runtimeId: 'support-upload-execution-provider-unavailable',
      runtimeState: 'provider-unavailable',
      backendAvailabilityState: 'available',
      providerAvailabilityState: 'unavailable',
      retryState: 'retry-scheduled',
      abandonState: 'not-requested',
      retryRefs: ['support-upload-runtime-provider-retry-schedule-ref'],
      abandonRefs: [],
      failureRefs: ['support-upload-runtime-provider-unavailable-status-ref'],
      manualProofRequirements: [],
    }),
    supportBackendUploadExecutionRuntimeEntry({
      runtimeId: 'support-upload-execution-retry-scheduled',
      runtimeState: 'retry-scheduled',
      backendAvailabilityState: 'unavailable',
      providerAvailabilityState: 'available',
      retryState: 'retry-scheduled',
      abandonState: 'not-requested',
      retryRefs: ['support-upload-runtime-retry-schedule-ref'],
      abandonRefs: [],
      failureRefs: ['support-upload-runtime-retry-source-status-ref'],
      manualProofRequirements: [],
    }),
    supportBackendUploadExecutionRuntimeEntry({
      runtimeId: 'support-upload-execution-operator-abandoned',
      runtimeState: 'operator-abandoned',
      backendAvailabilityState: 'unavailable',
      providerAvailabilityState: 'available',
      retryState: 'retry-exhausted',
      abandonState: 'abandoned',
      retryRefs: ['support-upload-runtime-retry-exhausted-ref'],
      abandonRefs: ['support-upload-runtime-operator-abandon-ref', 'parent-abandon-decision-ref'],
      failureRefs: ['support-upload-runtime-abandoned-status-ref'],
      manualProofRequirements: [],
    }),
  ],
});

function supportBackendUploadExecutionRuntimeEntry(
  input: SupportBackendUploadExecutionRuntimeEntryInput
): SupportBackendUploadExecutionRuntimeEntry {
  return SupportBackendUploadExecutionRuntimeEntrySchema.parse({
    schemaVersion: 1,
    parentInitiationState: 'parent-initiated',
    parentConsentState: 'parent-approved',
    executionClaimState: 'runtime-boundary-only',
    payloadState: 'redacted-runtime-refs-only',
    custodyState: 'no-ocentra-hosted-family-data',
    disclosedDataClasses: [...SupportBackendUploadExecutionRuntimeRequiredDataClasses],
    consentRefs: ['parent-support-upload-consent-artifact-ref'],
    redactionRefs: ['support-bundle-redaction-proof-ref', 'support-upload-redaction-preflight-ref'],
    auditRefs: ['support-upload-runtime-audit-event-ref', 'support-upload-custody-boundary-audit-ref'],
    statusRefs: ['production-support-backend-upload-status-proof-ref'],
    runtimeRefs: ['support-upload-runtime-boundary-ref'],
    backendRefs: ['support-backend-runtime-boundary-ref'],
    providerRefs: ['support-provider-runtime-boundary-ref'],
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
    billingProviderContactExecuted: false,
    remoteSupportSessionExecuted: false,
    productionSlaClaimed: false,
    ocentraHostedFamilyDataDefault: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
