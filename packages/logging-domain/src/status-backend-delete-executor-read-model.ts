import {
  type StatusBackendDeleteExecutorAuditExportState,
  type StatusBackendDeleteExecutorEntry,
  StatusBackendDeleteExecutorEntrySchema,
  type StatusBackendDeleteExecutorExecutionState,
  type StatusBackendDeleteExecutorPayloadDeletionState,
  StatusBackendDeleteExecutorReadModelSchema,
  StatusBackendDeleteExecutorRequiredDataClasses,
  type StatusBackendDeleteExecutorState,
} from './status-backend-delete-executor.js';

type StatusBackendDeleteExecutorEntryInput = {
  executorId: string;
  deleteExecutorState: StatusBackendDeleteExecutorState;
  executorExecutionState: StatusBackendDeleteExecutorExecutionState;
  payloadDeletionState: StatusBackendDeleteExecutorPayloadDeletionState;
  auditExportState: StatusBackendDeleteExecutorAuditExportState;
  deleteRefs: readonly string[];
  executorRefs: readonly string[];
  failureRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-07T09:25:00.000Z';

export const StatusBackendDeleteExecutorReadModel = StatusBackendDeleteExecutorReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-status-backend-delete-executor-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-status-backend-delete-executor-boundary',
    'release-installer-status-backend-delete-executor-expectation',
    'production-support-status-backend-runtime-execution-proof',
    'production-support-status-backend-payload-custody-proof',
  ],
  entries: [
    statusBackendDeleteExecutorEntry({
      executorId: 'status-backend-delete-request-recorded',
      deleteExecutorState: 'delete-request-recorded',
      executorExecutionState: 'manual-required',
      payloadDeletionState: 'manual-required',
      auditExportState: 'manual-required',
      deleteRefs: ['parent-status-backend-delete-request-ref'],
      executorRefs: ['status-backend-delete-executor-request-boundary-ref'],
      failureRefs: [],
      manualProofRequirements: ['delete executor implementation proof before status payload deletion can be claimed'],
    }),
    statusBackendDeleteExecutorEntry({
      executorId: 'status-backend-delete-executor-authorized',
      deleteExecutorState: 'delete-executor-authorized',
      executorExecutionState: 'manual-required',
      payloadDeletionState: 'manual-required',
      auditExportState: 'manual-required',
      deleteRefs: ['parent-status-backend-delete-authorization-ref'],
      executorRefs: ['status-backend-delete-executor-authorization-boundary-ref'],
      failureRefs: [],
      manualProofRequirements: ['parent authorization proof before delete executor dispatch can be claimed'],
    }),
    statusBackendDeleteExecutorEntry({
      executorId: 'status-backend-delete-executor-queued',
      deleteExecutorState: 'delete-executor-queued',
      executorExecutionState: 'manual-required',
      payloadDeletionState: 'manual-required',
      auditExportState: 'manual-required',
      deleteRefs: ['status-backend-delete-queue-request-ref'],
      executorRefs: ['status-backend-delete-executor-queue-ref'],
      failureRefs: [],
      manualProofRequirements: ['durable delete executor queue proof before queued execution can be claimed'],
    }),
    statusBackendDeleteExecutorEntry({
      executorId: 'status-backend-delete-executor-running',
      deleteExecutorState: 'delete-executor-running',
      executorExecutionState: 'manual-required',
      payloadDeletionState: 'manual-required',
      auditExportState: 'manual-required',
      deleteRefs: ['status-backend-delete-running-request-ref'],
      executorRefs: ['status-backend-delete-executor-runtime-ref'],
      failureRefs: [],
      manualProofRequirements: ['delete executor runtime smoke before running execution can be claimed'],
    }),
    statusBackendDeleteExecutorEntry({
      executorId: 'status-backend-deletion-manual-required',
      deleteExecutorState: 'deletion-manual-required',
      executorExecutionState: 'manual-required',
      payloadDeletionState: 'manual-required',
      auditExportState: 'manual-required',
      deleteRefs: ['status-backend-payload-delete-manual-proof-ref'],
      executorRefs: ['status-backend-delete-executor-manual-runbook-ref'],
      failureRefs: [],
      manualProofRequirements: ['operator deletion runbook before status backend payload deletion can be claimed'],
    }),
    statusBackendDeleteExecutorEntry({
      executorId: 'status-backend-delete-executor-failed',
      deleteExecutorState: 'delete-executor-failed',
      executorExecutionState: 'manual-required',
      payloadDeletionState: 'manual-required',
      auditExportState: 'manual-required',
      deleteRefs: ['status-backend-delete-failure-request-ref'],
      executorRefs: ['status-backend-delete-executor-failure-boundary-ref'],
      failureRefs: ['status-backend-delete-executor-failure-ref'],
      manualProofRequirements: ['delete executor failure replay proof before retry worker execution can be claimed'],
    }),
    statusBackendDeleteExecutorEntry({
      executorId: 'status-backend-delete-executor-audit-export-ready',
      deleteExecutorState: 'audit-export-ready',
      executorExecutionState: 'manual-required',
      payloadDeletionState: 'manual-required',
      auditExportState: 'support-safe-export-ready',
      deleteRefs: ['status-backend-delete-audit-request-ref'],
      executorRefs: ['status-backend-delete-executor-audit-ref'],
      failureRefs: [],
      manualProofRequirements: ['support-safe delete executor audit export review before deletion can be claimed'],
    }),
    statusBackendDeleteExecutorEntry({
      executorId: 'status-backend-delete-executor-backend-unavailable',
      deleteExecutorState: 'backend-unavailable',
      executorExecutionState: 'not-executed',
      payloadDeletionState: 'not-requested',
      auditExportState: 'manual-required',
      deleteRefs: ['status-backend-delete-backend-unavailable-ref'],
      executorRefs: ['status-backend-delete-executor-unavailable-ref'],
      failureRefs: ['status-backend-delete-executor-backend-unavailable-ref'],
      manualProofRequirements: ['status backend unavailable fallback proof before delete executor claims can be made'],
    }),
  ],
});

function statusBackendDeleteExecutorEntry(input: StatusBackendDeleteExecutorEntryInput): StatusBackendDeleteExecutorEntry {
  return StatusBackendDeleteExecutorEntrySchema.parse({
    schemaVersion: 1,
    parentConsentState: 'parent-approved',
    executionClaimState: 'status-backend-delete-executor-boundary-only',
    payloadState: 'redacted-delete-status-refs-only',
    disclosedDataClasses: [...StatusBackendDeleteExecutorRequiredDataClasses],
    consentRefs: ['parent-status-backend-delete-consent-ref'],
    targetRefs: ['production-support-status-backend-public-runtime-followthrough-proof-ref'],
    queueRefs: ['production-support-status-backend-execution-queue-proof-ref'],
    auditRefs: ['production-support-status-backend-queue-audit-persistence-proof-ref'],
    redactionRefs: ['support-bundle-redaction-proof-ref', 'status-backend-delete-executor-redaction-summary-ref'],
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
    statusBackendDeleteExecutorExecuted: false,
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
