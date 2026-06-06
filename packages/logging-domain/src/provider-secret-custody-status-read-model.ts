import {
  type ProviderSecretCustodyExecutionState,
  ProviderSecretCustodyRequiredDataClasses,
  type ProviderSecretCustodyStatusEntry,
  ProviderSecretCustodyStatusEntrySchema,
  ProviderSecretCustodyStatusReadModelSchema,
  type ProviderSecretCustodyStatusState,
} from './provider-secret-custody-status.js';

type ProviderSecretCustodyStatusEntryInput = {
  statusId: string;
  custodyStatus: ProviderSecretCustodyStatusState;
  providerSecretCustodyState: ProviderSecretCustodyExecutionState;
  backendSecretStoreState: ProviderSecretCustodyExecutionState;
  rotationState: ProviderSecretCustodyExecutionState;
  revocationState: ProviderSecretCustodyExecutionState;
  rotationRefs: readonly string[];
  revocationRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-06T08:26:56.000Z';

export const ProviderSecretCustodyStatusReadModel = ProviderSecretCustodyStatusReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-provider-secret-custody-status-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'release-installer-legal-provider-readiness-expectation',
    'data-custody-provider-secret-non-custody-boundary',
    'production-support-legal-provider-readiness-proof',
    'billing-support-admin-status-proof',
  ],
  entries: [
    providerSecretCustodyStatusEntry({
      statusId: 'provider-secret-custody-boundary-recorded',
      custodyStatus: 'custody-boundary-recorded',
      providerSecretCustodyState: 'manual-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'not-applicable',
      revocationState: 'not-applicable',
      rotationRefs: [],
      revocationRefs: [],
      manualProofRequirements: ['provider secret custody design review before custody can be claimed'],
    }),
    providerSecretCustodyStatusEntry({
      statusId: 'provider-secret-absent-from-support-status',
      custodyStatus: 'provider-secret-absent',
      providerSecretCustodyState: 'not-implemented',
      backendSecretStoreState: 'not-applicable',
      rotationState: 'not-applicable',
      revocationState: 'not-applicable',
      rotationRefs: [],
      revocationRefs: [],
      manualProofRequirements: ['provider-secret absence proof before support status publication can proceed'],
    }),
    providerSecretCustodyStatusEntry({
      statusId: 'provider-secret-backend-store-manual-required',
      custodyStatus: 'backend-secret-store-manual-required',
      providerSecretCustodyState: 'manual-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'not-applicable',
      revocationState: 'not-applicable',
      rotationRefs: [],
      revocationRefs: [],
      manualProofRequirements: ['backend secret store threat model before provider secret storage can be claimed'],
    }),
    providerSecretCustodyStatusEntry({
      statusId: 'provider-secret-rotation-manual-required',
      custodyStatus: 'rotation-manual-required',
      providerSecretCustodyState: 'manual-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'manual-required',
      revocationState: 'not-applicable',
      rotationRefs: ['provider-secret-rotation-runbook-ref'],
      revocationRefs: [],
      manualProofRequirements: ['rotation runbook and smoke proof before provider secret rotation can be claimed'],
    }),
    providerSecretCustodyStatusEntry({
      statusId: 'provider-secret-revocation-manual-required',
      custodyStatus: 'revocation-manual-required',
      providerSecretCustodyState: 'manual-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'not-applicable',
      revocationState: 'manual-required',
      rotationRefs: [],
      revocationRefs: ['provider-secret-revocation-runbook-ref'],
      manualProofRequirements: ['revocation runbook and smoke proof before provider secret revocation can be claimed'],
    }),
    providerSecretCustodyStatusEntry({
      statusId: 'provider-secret-custody-audit-export-ready',
      custodyStatus: 'audit-export-ready',
      providerSecretCustodyState: 'manual-required',
      backendSecretStoreState: 'manual-required',
      rotationState: 'manual-required',
      revocationState: 'manual-required',
      rotationRefs: ['provider-secret-rotation-audit-ref'],
      revocationRefs: ['provider-secret-revocation-audit-ref'],
      manualProofRequirements: [
        'support-safe provider secret custody audit export review before custody can be claimed',
      ],
    }),
  ],
});

function providerSecretCustodyStatusEntry(
  input: ProviderSecretCustodyStatusEntryInput
): ProviderSecretCustodyStatusEntry {
  return ProviderSecretCustodyStatusEntrySchema.parse({
    schemaVersion: 1,
    payloadState: 'support-safe-status-refs-only',
    custodyBoundaryState: 'no-provider-secret-custody',
    disclosedDataClasses: [...ProviderSecretCustodyRequiredDataClasses],
    allowedDestinations: ['support-safe-status-boundary', 'manual-security-runbook'],
    legalProviderRefs: ['production-support-legal-provider-readiness-proof-ref'],
    billingSupportRefs: ['billing-support-admin-status-proof-ref'],
    redactionRefs: ['support-bundle-redaction-proof-ref'],
    auditRefs: ['provider-secret-custody-audit-status-ref'],
    custodyRefs: ['data-custody-provider-secret-non-custody-boundary-ref'],
    containsProviderSecrets: false,
    containsPaymentProviderTokens: false,
    containsRawChildActivity: false,
    containsRawSupportBundlePayloads: false,
    containsAccountLookupResults: false,
    containsBillingProviderContactRecords: false,
    containsRemoteSupportTranscripts: false,
    providerSecretCustodyExecuted: false,
    backendSecretStoreImplemented: false,
    rotationExecuted: false,
    revocationExecuted: false,
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
