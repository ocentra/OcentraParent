import {
  type ProviderSecretExecutionClaimState,
  ProviderSecretExecutionReadinessEntrySchema,
  type ProviderSecretExecutionReadinessEntry,
  ProviderSecretExecutionReadinessReadModelSchema,
  ProviderSecretExecutionRequiredDataClasses,
  type ProviderSecretExecutionStatusState,
} from './provider-secret-execution-readiness.js';

type ProviderSecretExecutionReadinessEntryInput = {
  statusId: string;
  readinessStatus: ProviderSecretExecutionStatusState;
  backendSecretStoreState: ProviderSecretExecutionClaimState;
  rotationState: ProviderSecretExecutionClaimState;
  revocationState: ProviderSecretExecutionClaimState;
  operatorApprovalState: ProviderSecretExecutionClaimState;
  executionState: ProviderSecretExecutionClaimState;
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-07T13:49:32.000Z';

export const ProviderSecretExecutionReadinessReadModel = ProviderSecretExecutionReadinessReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'provider-secret-execution-readiness-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-provider-secret-non-custody-boundary',
    'static-analysis-security-secret-handling-expectation',
    'production-support-provider-secret-custody-status-proof',
  ],
  entries: [
    providerSecretExecutionReadinessEntry({
      statusId: 'provider-secret-execution-boundary-recorded',
      readinessStatus: 'execution-boundary-recorded',
      backendSecretStoreState: 'not-implemented',
      rotationState: 'not-implemented',
      revocationState: 'not-implemented',
      operatorApprovalState: 'manual-required',
      executionState: 'not-implemented',
      manualProofRequirements: ['provider secret execution design review before execution readiness can be claimed'],
    }),
    providerSecretExecutionReadinessEntry({
      statusId: 'provider-secret-backend-store-preflight-required',
      readinessStatus: 'backend-secret-store-preflight-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'not-implemented',
      revocationState: 'not-implemented',
      operatorApprovalState: 'manual-required',
      executionState: 'manual-required',
      manualProofRequirements: [
        'backend secret store preflight and threat model before storage execution can be claimed',
      ],
    }),
    providerSecretExecutionReadinessEntry({
      statusId: 'provider-secret-rotation-preflight-required',
      readinessStatus: 'rotation-preflight-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'manual-required',
      revocationState: 'not-implemented',
      operatorApprovalState: 'manual-required',
      executionState: 'manual-required',
      manualProofRequirements: ['rotation preflight and smoke proof before rotation execution can be claimed'],
    }),
    providerSecretExecutionReadinessEntry({
      statusId: 'provider-secret-revocation-preflight-required',
      readinessStatus: 'revocation-preflight-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'manual-required',
      revocationState: 'manual-required',
      operatorApprovalState: 'manual-required',
      executionState: 'manual-required',
      manualProofRequirements: ['revocation preflight and smoke proof before revocation execution can be claimed'],
    }),
    providerSecretExecutionReadinessEntry({
      statusId: 'provider-secret-operator-approval-required',
      readinessStatus: 'operator-approval-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'manual-required',
      revocationState: 'manual-required',
      operatorApprovalState: 'manual-required',
      executionState: 'manual-required',
      manualProofRequirements: ['operator approval record before any provider secret execution can be claimed'],
    }),
    providerSecretExecutionReadinessEntry({
      statusId: 'provider-secret-execution-manual-required',
      readinessStatus: 'execution-manual-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'manual-required',
      revocationState: 'manual-required',
      operatorApprovalState: 'manual-required',
      executionState: 'manual-required',
      manualProofRequirements: ['manual execution evidence before provider secret operation can leave readiness mode'],
    }),
    providerSecretExecutionReadinessEntry({
      statusId: 'provider-secret-execution-audit-export-ready',
      readinessStatus: 'audit-export-ready',
      backendSecretStoreState: 'manual-required',
      rotationState: 'manual-required',
      revocationState: 'manual-required',
      operatorApprovalState: 'manual-required',
      executionState: 'manual-required',
      manualProofRequirements: [
        'support-safe provider secret execution audit export review before execution can be claimed',
      ],
    }),
  ],
});

function providerSecretExecutionReadinessEntry(
  input: ProviderSecretExecutionReadinessEntryInput
): ProviderSecretExecutionReadinessEntry {
  return ProviderSecretExecutionReadinessEntrySchema.parse({
    schemaVersion: 1,
    payloadState: 'support-safe-status-refs-only',
    disclosedDataClasses: [...ProviderSecretExecutionRequiredDataClasses],
    allowedDestinations: ['manual-security-runbook', 'support-safe-audit-export'],
    custodyStatusRefs: ['production-support-provider-secret-custody-status-proof-ref'],
    backendSecretStoreRefs: ['backend-secret-store-preflight-ref'],
    rotationRefs: ['provider-secret-rotation-preflight-ref'],
    revocationRefs: ['provider-secret-revocation-preflight-ref'],
    operatorApprovalRefs: ['provider-secret-operator-approval-ref'],
    auditRefs: ['provider-secret-execution-audit-ref'],
    containsProviderSecrets: false,
    containsPaymentProviderTokens: false,
    containsRawChildActivity: false,
    containsRawSupportBundlePayloads: false,
    containsAccountLookupResults: false,
    containsBillingProviderContactRecords: false,
    containsRemoteSupportTranscripts: false,
    backendSecretStoreExecuted: false,
    providerSecretRotationExecuted: false,
    providerSecretRevocationExecuted: false,
    providerSecretExecutionDelivered: false,
    supportBackendUploadExecuted: false,
    accountLookupExecuted: false,
    billingProviderContactExecuted: false,
    remoteSupportSessionExecuted: false,
    productionSlaClaimed: false,
    ocentraHostedFamilyDataDefault: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}
