import {
  type SupportBackendProviderRuntimeReadinessClaimState,
  SupportBackendProviderRuntimeReadinessEntrySchema,
  type SupportBackendProviderRuntimeReadinessEntry,
  SupportBackendProviderRuntimeReadinessReadModelSchema,
  SupportBackendProviderRuntimeReadinessRequiredDataClasses,
  type SupportBackendProviderRuntimeReadinessState,
} from './support-backend-provider-runtime-readiness.js';

type SupportBackendProviderRuntimeReadinessEntryInput = {
  statusId: string;
  readinessState: SupportBackendProviderRuntimeReadinessState;
  uploadRuntimeState: SupportBackendProviderRuntimeReadinessClaimState;
  providerSecretState: SupportBackendProviderRuntimeReadinessClaimState;
  billingProviderState: SupportBackendProviderRuntimeReadinessClaimState;
  accountLookupState: SupportBackendProviderRuntimeReadinessClaimState;
  legalDisclosureState: SupportBackendProviderRuntimeReadinessClaimState;
  remoteSupportState: SupportBackendProviderRuntimeReadinessClaimState;
  productionSlaState: SupportBackendProviderRuntimeReadinessClaimState;
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-07T23:21:22.000Z';

export const SupportBackendProviderRuntimeReadinessReadModel =
  SupportBackendProviderRuntimeReadinessReadModelSchema.parse({
    schemaVersion: 1,
    readModelId: 'production-support-backend-provider-runtime-readiness-proof',
    generatedAt,
    sourceContractRefs: [
      'production-distribution-support-feature-doc',
      'production-support-backend-upload-execution-runtime-proof',
      'production-support-backend-upload-custody-audit-proof',
      'provider-secret-execution-readiness-proof',
      'production-support-account-sla-status-proof',
      'production-support-privacy-legal-disclosure-status-proof',
      'production-support-case-resolution-status-proof',
    ],
    entries: [
      supportBackendProviderRuntimeReadinessEntry({
        statusId: 'support-backend-provider-upload-runtime-linked',
        readinessState: 'upload-runtime-linked',
        uploadRuntimeState: 'readiness-only',
        providerSecretState: 'not-implemented',
        billingProviderState: 'not-implemented',
        accountLookupState: 'not-implemented',
        legalDisclosureState: 'not-implemented',
        remoteSupportState: 'not-implemented',
        productionSlaState: 'not-implemented',
        manualProofRequirements: ['real support backend upload adapter proof before upload execution can be claimed'],
      }),
      supportBackendProviderRuntimeReadinessEntry({
        statusId: 'support-backend-provider-secret-preflight-linked',
        readinessState: 'provider-secret-preflight-linked',
        uploadRuntimeState: 'readiness-only',
        providerSecretState: 'manual-required',
        billingProviderState: 'not-implemented',
        accountLookupState: 'not-implemented',
        legalDisclosureState: 'not-implemented',
        remoteSupportState: 'not-implemented',
        productionSlaState: 'not-implemented',
        manualProofRequirements: [
          'provider-secret custody and delivery proof before provider execution can be claimed',
        ],
      }),
      supportBackendProviderRuntimeReadinessEntry({
        statusId: 'support-backend-billing-provider-manual-required',
        readinessState: 'billing-provider-manual-required',
        uploadRuntimeState: 'readiness-only',
        providerSecretState: 'manual-required',
        billingProviderState: 'manual-required',
        accountLookupState: 'not-implemented',
        legalDisclosureState: 'not-implemented',
        remoteSupportState: 'not-implemented',
        productionSlaState: 'not-implemented',
        manualProofRequirements: ['billing provider contact execution proof before billing escalation can be claimed'],
      }),
      supportBackendProviderRuntimeReadinessEntry({
        statusId: 'support-backend-account-lookup-manual-required',
        readinessState: 'account-lookup-manual-required',
        uploadRuntimeState: 'readiness-only',
        providerSecretState: 'manual-required',
        billingProviderState: 'manual-required',
        accountLookupState: 'manual-required',
        legalDisclosureState: 'not-implemented',
        remoteSupportState: 'not-implemented',
        productionSlaState: 'not-implemented',
        manualProofRequirements: ['account lookup runtime proof before account support lookup can be claimed'],
      }),
      supportBackendProviderRuntimeReadinessEntry({
        statusId: 'support-backend-legal-disclosure-manual-required',
        readinessState: 'legal-disclosure-manual-required',
        uploadRuntimeState: 'readiness-only',
        providerSecretState: 'manual-required',
        billingProviderState: 'manual-required',
        accountLookupState: 'manual-required',
        legalDisclosureState: 'manual-required',
        remoteSupportState: 'not-implemented',
        productionSlaState: 'not-implemented',
        manualProofRequirements: ['privacy/legal disclosure execution proof before legal disclosure can be claimed'],
      }),
      supportBackendProviderRuntimeReadinessEntry({
        statusId: 'support-backend-remote-support-manual-required',
        readinessState: 'remote-support-manual-required',
        uploadRuntimeState: 'readiness-only',
        providerSecretState: 'manual-required',
        billingProviderState: 'manual-required',
        accountLookupState: 'manual-required',
        legalDisclosureState: 'manual-required',
        remoteSupportState: 'manual-required',
        productionSlaState: 'not-implemented',
        manualProofRequirements: [
          'remote support session approval and transcript safety proof before remote support can be claimed',
        ],
      }),
      supportBackendProviderRuntimeReadinessEntry({
        statusId: 'support-backend-sla-manual-required',
        readinessState: 'sla-manual-required',
        uploadRuntimeState: 'readiness-only',
        providerSecretState: 'manual-required',
        billingProviderState: 'manual-required',
        accountLookupState: 'manual-required',
        legalDisclosureState: 'manual-required',
        remoteSupportState: 'manual-required',
        productionSlaState: 'manual-required',
        manualProofRequirements: ['production SLA policy and operational proof before SLA commitments can be claimed'],
      }),
      supportBackendProviderRuntimeReadinessEntry({
        statusId: 'support-backend-provider-audit-export-ready',
        readinessState: 'audit-export-ready',
        uploadRuntimeState: 'readiness-only',
        providerSecretState: 'manual-required',
        billingProviderState: 'manual-required',
        accountLookupState: 'manual-required',
        legalDisclosureState: 'manual-required',
        remoteSupportState: 'manual-required',
        productionSlaState: 'manual-required',
        manualProofRequirements: [
          'support-safe provider runtime audit export review before runtime/provider execution can be claimed',
        ],
      }),
    ],
  });

function supportBackendProviderRuntimeReadinessEntry(
  input: SupportBackendProviderRuntimeReadinessEntryInput
): SupportBackendProviderRuntimeReadinessEntry {
  return SupportBackendProviderRuntimeReadinessEntrySchema.parse({
    schemaVersion: 1,
    payloadState: 'support-safe-status-refs-only',
    custodyState: 'no-ocentra-hosted-family-data',
    disclosedDataClasses: [...SupportBackendProviderRuntimeReadinessRequiredDataClasses],
    uploadRuntimeRefs: ['production-support-backend-upload-execution-runtime-proof-ref'],
    custodyAuditRefs: ['production-support-backend-upload-custody-audit-proof-ref'],
    providerSecretRefs: ['provider-secret-execution-readiness-proof-ref'],
    accountBillingRefs: ['production-support-account-sla-status-proof-ref'],
    privacyLegalRefs: ['production-support-privacy-legal-disclosure-status-proof-ref'],
    caseStatusRefs: ['production-support-case-resolution-status-proof-ref'],
    auditRefs: ['support-backend-provider-runtime-readiness-audit-ref'],
    containsProviderSecrets: false,
    containsPaymentProviderTokens: false,
    containsRawChildActivity: false,
    containsRawSupportBundlePayloads: false,
    containsAccountLookupResults: false,
    containsBillingProviderContactRecords: false,
    containsRemoteSupportTranscripts: false,
    supportBackendUploadExecuted: false,
    providerSecretDelivered: false,
    accountLookupExecuted: false,
    billingProviderContactExecuted: false,
    legalDisclosureExecuted: false,
    remoteSupportSessionExecuted: false,
    productionSlaClaimed: false,
    ocentraHostedFamilyDataDefault: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
