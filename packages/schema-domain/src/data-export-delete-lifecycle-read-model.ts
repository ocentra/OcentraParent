import {
  type DataExportDeleteLifecycleEntry,
  DataExportDeleteLifecycleEntrySchema,
  DataExportDeleteLifecycleReadModelSchema,
  DataExportDeleteLifecycleRequiredDataClasses,
  type DataExportDeleteLifecycleOperation,
  type DataExportDeleteLifecycleState,
} from './data-export-delete-lifecycle.js';

const generatedAt = '2026-06-06T04:21:00.000Z';

export const DataExportDeleteLifecycleReadModel = DataExportDeleteLifecycleReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-data-export-delete-lifecycle-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-local-export-delete-runtime-status',
    'production-support-legal-provider-readiness-proof',
    'production-incident-support-status-proof',
  ],
  entries: [
    lifecycleEntry('data-export-requested', 'export', 'requested'),
    lifecycleEntry('data-export-authorized', 'export', 'authorized'),
    lifecycleEntry('data-export-queued', 'export', 'queued'),
    lifecycleEntry('data-export-running', 'export', 'running', ['local-export-runtime-ref']),
    lifecycleEntry(
      'data-export-succeeded',
      'export',
      'succeeded',
      ['local-export-runtime-ref'],
      ['local-export-output-ref']
    ),
    lifecycleEntry(
      'data-export-failed',
      'export',
      'failed',
      ['local-export-failure-ref'],
      [],
      [],
      ['manual review required before export retry can be called production ready']
    ),
    lifecycleEntry(
      'data-export-manual-required',
      'export',
      'manual-required',
      [],
      [],
      [],
      ['filesystem writer and parent-visible export control proof required before runtime execution claim']
    ),
    lifecycleEntry('data-delete-requested', 'delete', 'requested'),
    lifecycleEntry('data-delete-authorized', 'delete', 'authorized'),
    lifecycleEntry('data-delete-queued', 'delete', 'queued'),
    lifecycleEntry(
      'data-delete-running',
      'delete',
      'running',
      ['local-delete-runtime-ref'],
      [],
      ['local-delete-request-ref']
    ),
    lifecycleEntry(
      'data-delete-succeeded',
      'delete',
      'succeeded',
      ['local-delete-runtime-ref'],
      [],
      ['local-delete-confirmation-ref']
    ),
    lifecycleEntry(
      'data-delete-failed',
      'delete',
      'failed',
      ['local-delete-failure-ref'],
      [],
      ['local-delete-failure-ref'],
      ['manual review required before delete retry can be called production ready']
    ),
    lifecycleEntry(
      'data-delete-manual-required',
      'delete',
      'manual-required',
      [],
      [],
      ['local-delete-request-ref'],
      ['delete executor and durable audit proof required before deletion execution claim']
    ),
  ],
});

function lifecycleEntry(
  lifecycleId: string,
  operation: DataExportDeleteLifecycleOperation,
  lifecycleState: DataExportDeleteLifecycleState,
  runtimeRefs: readonly string[] = [],
  outputRefs: readonly string[] = [],
  deleteRefs: readonly string[] = [],
  manualProofRequirements: readonly string[] = []
): DataExportDeleteLifecycleEntry {
  return DataExportDeleteLifecycleEntrySchema.parse({
    schemaVersion: 1,
    lifecycleId,
    operation,
    lifecycleState,
    parentInitiationState: 'parent-initiated',
    parentAuthorizationState: 'parent-authorized',
    payloadState: 'redacted-runtime-status-only',
    custodyState: 'parent-owned-local-output-only',
    disclosedDataClasses: [...DataExportDeleteLifecycleRequiredDataClasses],
    requestRefs: [`${operation}-parent-request-ref`],
    authorizationRefs: [`${operation}-parent-authorization-ref`],
    queueRefs: [`${operation}-local-queue-ref`],
    runtimeRefs,
    outputRefs,
    deleteRefs,
    auditRefs: [`${operation}-redaction-audit-ref`],
    custodyRefs: ['data-custody-local-export-delete-boundary-ref'],
    manualProofRequirements,
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
    realBackendUploadExecuted: false,
    publicRuntimeExecuted: false,
    providerExecutionOccurred: false,
    productionSlaClaimed: false,
    remoteSupportSessionExecuted: false,
    childActivityCustodyClaimed: false,
    ocentraHostedFamilyDataDefault: false,
    lastCheckedAt: generatedAt,
  });
}
