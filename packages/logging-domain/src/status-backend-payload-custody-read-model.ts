import {
  type StatusBackendPayloadCustodyAuditExportState,
  type StatusBackendPayloadCustodyDeleteState,
  type StatusBackendPayloadCustodyEntry,
  StatusBackendPayloadCustodyEntrySchema,
  StatusBackendPayloadCustodyReadModelSchema,
  StatusBackendPayloadCustodyRequiredDataClasses,
  type StatusBackendPayloadCustodyState,
  type StatusBackendPayloadCustodyStorageState,
} from './status-backend-payload-custody.js';

type StatusBackendPayloadCustodyEntryInput = {
  custodyId: string;
  custodyState: StatusBackendPayloadCustodyState;
  storageState: StatusBackendPayloadCustodyStorageState;
  deleteState: StatusBackendPayloadCustodyDeleteState;
  auditExportState: StatusBackendPayloadCustodyAuditExportState;
  retentionRefs: readonly string[];
  deleteRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-06T12:50:00.000Z';

export const StatusBackendPayloadCustodyReadModel = StatusBackendPayloadCustodyReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-status-backend-payload-custody-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-status-backend-payload-boundary',
    'release-installer-status-backend-queue-expectation',
    'production-support-status-backend-public-runtime-followthrough-proof',
    'production-support-status-backend-execution-queue-proof',
  ],
  entries: [
    statusBackendPayloadCustodyEntry({
      custodyId: 'status-backend-payload-custody-boundary-recorded',
      custodyState: 'custody-boundary-recorded',
      storageState: 'manual-required',
      deleteState: 'not-requested',
      auditExportState: 'manual-required',
      retentionRefs: ['status-backend-payload-retention-manual-proof-ref'],
      deleteRefs: [],
      manualProofRequirements: ['retention policy proof before status backend payload custody can be claimed'],
    }),
    statusBackendPayloadCustodyEntry({
      custodyId: 'status-backend-payload-retention-manual-required',
      custodyState: 'retention-manual-required',
      storageState: 'manual-required',
      deleteState: 'not-requested',
      auditExportState: 'manual-required',
      retentionRefs: ['status-backend-payload-retention-runbook-ref'],
      deleteRefs: [],
      manualProofRequirements: ['published retention runbook before durable status payload storage can be claimed'],
    }),
    statusBackendPayloadCustodyEntry({
      custodyId: 'status-backend-payload-delete-request-recorded',
      custodyState: 'delete-request-recorded',
      storageState: 'manual-required',
      deleteState: 'manual-required',
      auditExportState: 'manual-required',
      retentionRefs: ['status-backend-payload-retention-delete-boundary-ref'],
      deleteRefs: ['parent-status-backend-payload-delete-request-ref'],
      manualProofRequirements: ['delete request execution proof before status backend payload deletion can be claimed'],
    }),
    statusBackendPayloadCustodyEntry({
      custodyId: 'status-backend-payload-deletion-manual-required',
      custodyState: 'deletion-manual-required',
      storageState: 'manual-required',
      deleteState: 'manual-required',
      auditExportState: 'manual-required',
      retentionRefs: ['status-backend-payload-retention-delete-boundary-ref'],
      deleteRefs: ['status-backend-payload-delete-manual-proof-ref'],
      manualProofRequirements: ['operator deletion runbook before status backend payload deletion can be claimed'],
    }),
    statusBackendPayloadCustodyEntry({
      custodyId: 'status-backend-payload-audit-export-ready',
      custodyState: 'audit-export-ready',
      storageState: 'manual-required',
      deleteState: 'manual-required',
      auditExportState: 'support-safe-export-ready',
      retentionRefs: ['status-backend-payload-retention-audit-ref'],
      deleteRefs: ['status-backend-payload-delete-audit-ref'],
      manualProofRequirements: [
        'support-safe status payload custody audit export review before status backend execution can be claimed',
      ],
    }),
    statusBackendPayloadCustodyEntry({
      custodyId: 'status-backend-payload-backend-unavailable',
      custodyState: 'backend-unavailable',
      storageState: 'not-retained',
      deleteState: 'not-requested',
      auditExportState: 'manual-required',
      retentionRefs: [],
      deleteRefs: [],
      manualProofRequirements: [
        'status backend unavailable fallback proof before status payload custody can be claimed',
      ],
    }),
  ],
});

function statusBackendPayloadCustodyEntry(
  input: StatusBackendPayloadCustodyEntryInput
): StatusBackendPayloadCustodyEntry {
  return StatusBackendPayloadCustodyEntrySchema.parse({
    schemaVersion: 1,
    parentConsentState: 'parent-approved',
    executionClaimState: 'status-backend-payload-custody-boundary-only',
    payloadState: 'redacted-status-refs-only',
    disclosedDataClasses: [...StatusBackendPayloadCustodyRequiredDataClasses],
    consentRefs: ['parent-status-backend-payload-consent-ref'],
    targetRefs: ['production-support-status-backend-public-runtime-followthrough-proof-ref'],
    queueRefs: ['production-support-status-backend-execution-queue-proof-ref'],
    auditRefs: ['production-support-status-backend-execution-queue-audit-ref'],
    redactionRefs: ['support-bundle-redaction-proof-ref', 'status-backend-payload-redaction-summary-ref'],
    custodyRefs: ['data-custody-status-backend-payload-boundary-ref'],
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
