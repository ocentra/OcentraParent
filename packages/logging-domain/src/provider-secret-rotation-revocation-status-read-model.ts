import {
  type ProviderSecretRotationRevocationExecutionState,
  ProviderSecretRotationRevocationRequiredDataClasses,
  type ProviderSecretRotationRevocationStatusEntry,
  ProviderSecretRotationRevocationStatusEntrySchema,
  ProviderSecretRotationRevocationStatusReadModelSchema,
  type ProviderSecretRotationRevocationStatusState,
} from './provider-secret-rotation-revocation-status.js';

type ProviderSecretRotationRevocationStatusEntryInput = {
  statusId: string;
  rotationRevocationStatus: ProviderSecretRotationRevocationStatusState;
  rotationState: ProviderSecretRotationRevocationExecutionState;
  revocationState: ProviderSecretRotationRevocationExecutionState;
  operatorApprovalState: ProviderSecretRotationRevocationExecutionState;
  rotationRefs: readonly string[];
  revocationRefs: readonly string[];
  operatorApprovalRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-07T14:24:17.000Z';

export const ProviderSecretRotationRevocationStatusReadModel =
  ProviderSecretRotationRevocationStatusReadModelSchema.parse({
    schemaVersion: 1,
    readModelId: 'production-support-provider-secret-rotation-revocation-status-proof',
    generatedAt,
    sourceContractRefs: [
      'production-distribution-support-feature-doc',
      'data-custody-provider-secret-execution-non-claim',
      'production-support-provider-secret-custody-status-proof',
      'provider-secret-execution-readiness-proof',
      'static-analysis-security-provider-secret-gate',
    ],
    entries: [
      providerSecretRotationRevocationStatusEntry({
        statusId: 'provider-secret-rotation-request-recorded',
        rotationRevocationStatus: 'rotation-requested',
        rotationState: 'manual-required',
        revocationState: 'not-applicable',
        operatorApprovalState: 'manual-required',
        rotationRefs: ['provider-secret-rotation-request-ref'],
        revocationRefs: [],
        operatorApprovalRefs: ['provider-secret-rotation-operator-approval-ref'],
        manualProofRequirements: ['rotation request authorization review before provider secret rotation can proceed'],
      }),
      providerSecretRotationRevocationStatusEntry({
        statusId: 'provider-secret-rotation-preflight-ready',
        rotationRevocationStatus: 'rotation-preflight-ready',
        rotationState: 'preflight-ready',
        revocationState: 'not-applicable',
        operatorApprovalState: 'manual-required',
        rotationRefs: ['provider-secret-rotation-preflight-ref'],
        revocationRefs: [],
        operatorApprovalRefs: ['provider-secret-rotation-operator-approval-ref'],
        manualProofRequirements: ['backend secret-store preflight and operator approval before rotation execution'],
      }),
      providerSecretRotationRevocationStatusEntry({
        statusId: 'provider-secret-rotation-manual-required',
        rotationRevocationStatus: 'rotation-manual-required',
        rotationState: 'manual-required',
        revocationState: 'not-applicable',
        operatorApprovalState: 'manual-required',
        rotationRefs: ['provider-secret-rotation-runbook-ref'],
        revocationRefs: [],
        operatorApprovalRefs: ['provider-secret-rotation-operator-approval-ref'],
        manualProofRequirements: ['manual rotation runbook and smoke proof before rotation execution can be claimed'],
      }),
      providerSecretRotationRevocationStatusEntry({
        statusId: 'provider-secret-revocation-request-recorded',
        rotationRevocationStatus: 'revocation-requested',
        rotationState: 'not-applicable',
        revocationState: 'manual-required',
        operatorApprovalState: 'manual-required',
        rotationRefs: [],
        revocationRefs: ['provider-secret-revocation-request-ref'],
        operatorApprovalRefs: ['provider-secret-revocation-operator-approval-ref'],
        manualProofRequirements: [
          'revocation request authorization review before provider secret revocation can proceed',
        ],
      }),
      providerSecretRotationRevocationStatusEntry({
        statusId: 'provider-secret-revocation-preflight-ready',
        rotationRevocationStatus: 'revocation-preflight-ready',
        rotationState: 'not-applicable',
        revocationState: 'preflight-ready',
        operatorApprovalState: 'manual-required',
        rotationRefs: [],
        revocationRefs: ['provider-secret-revocation-preflight-ref'],
        operatorApprovalRefs: ['provider-secret-revocation-operator-approval-ref'],
        manualProofRequirements: ['backend secret-store preflight and operator approval before revocation execution'],
      }),
      providerSecretRotationRevocationStatusEntry({
        statusId: 'provider-secret-revocation-manual-required',
        rotationRevocationStatus: 'revocation-manual-required',
        rotationState: 'not-applicable',
        revocationState: 'manual-required',
        operatorApprovalState: 'manual-required',
        rotationRefs: [],
        revocationRefs: ['provider-secret-revocation-runbook-ref'],
        operatorApprovalRefs: ['provider-secret-revocation-operator-approval-ref'],
        manualProofRequirements: [
          'manual revocation runbook and smoke proof before revocation execution can be claimed',
        ],
      }),
      providerSecretRotationRevocationStatusEntry({
        statusId: 'provider-secret-rotation-revocation-audit-export-ready',
        rotationRevocationStatus: 'audit-export-ready',
        rotationState: 'manual-required',
        revocationState: 'manual-required',
        operatorApprovalState: 'manual-required',
        rotationRefs: ['provider-secret-rotation-audit-ref'],
        revocationRefs: ['provider-secret-revocation-audit-ref'],
        operatorApprovalRefs: ['provider-secret-audit-export-operator-approval-ref'],
        manualProofRequirements: [
          'support-safe provider secret rotation and revocation audit export review before execution can be claimed',
        ],
      }),
    ],
  });

function providerSecretRotationRevocationStatusEntry(
  input: ProviderSecretRotationRevocationStatusEntryInput
): ProviderSecretRotationRevocationStatusEntry {
  return ProviderSecretRotationRevocationStatusEntrySchema.parse({
    schemaVersion: 1,
    backendSecretStoreState: 'manual-required',
    payloadState: 'support-safe-status-refs-only',
    disclosedDataClasses: [...ProviderSecretRotationRevocationRequiredDataClasses],
    allowedDestinations: ['support-safe-status-boundary', 'manual-security-runbook'],
    custodyStatusRefs: ['production-support-provider-secret-custody-status-proof-ref'],
    executionReadinessRefs: ['provider-secret-execution-readiness-proof-ref'],
    backendSecretStoreRefs: ['provider-secret-backend-secret-store-preflight-ref'],
    auditRefs: ['provider-secret-rotation-revocation-audit-status-ref'],
    containsProviderSecrets: false,
    containsPaymentProviderTokens: false,
    containsRawChildActivity: false,
    containsRawSupportBundlePayloads: false,
    containsAccountLookupResults: false,
    containsBillingProviderContactRecords: false,
    containsRemoteSupportTranscripts: false,
    backendSecretStoreExecuted: false,
    rotationExecuted: false,
    revocationExecuted: false,
    providerSecretDelivered: false,
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
