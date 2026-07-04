import {
  type DeleteExecutorRow,
  DeleteExecutorReadModelSchema,
  DeleteExecutorRequiredDataClasses,
  DeleteExecutorRowSchema,
  type DeleteExecutorCustodyBoundary,
  type DeleteExecutorStatus,
  type DeleteExecutorTarget,
} from './delete-executor-proof.js';

const generatedAt = '2026-06-07T12:27:33.979Z';

type DeleteExecutorRowOverrides = {
  readonly custodyBoundary?: DeleteExecutorCustodyBoundary;
  readonly sourceProofRefs?: readonly string[];
  readonly manualProofRequirements?: readonly string[];
};

export const DeleteExecutorReadModel = DeleteExecutorReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'production-support-delete-executor-proof',
  generatedAt,
  sourceContractRefs: [
    'production-distribution-support-feature-doc',
    'data-custody-delete-executor-boundary',
    'production-support-data-export-delete-lifecycle-proof',
  ],
  rows: [
    deleteExecutorRow('local-output-delete-request-recorded', 'local-export-output', 'delete-request-recorded', {
      custodyBoundary: 'parent-owned-local-output-only',
      sourceProofRefs: ['production-support-data-export-delete-lifecycle-proof'],
    }),
    deleteExecutorRow(
      'local-output-delete-executor-manual-required',
      'local-export-output',
      'executor-manual-required',
      {
        custodyBoundary: 'parent-owned-local-output-only',
        manualProofRequirements: [
          'filesystem delete executor smoke proof required before local output deletion execution claim',
          'durable audit persistence proof required before production delete executor readiness claim',
        ],
        sourceProofRefs: ['production-support-data-export-delete-lifecycle-proof'],
      }
    ),
    deleteExecutorRow('support-backend-payload-delete-blocked', 'support-backend-payload', 'blocked-before-runtime', {
      custodyBoundary: 'no-hosted-payload-custody',
      manualProofRequirements: [
        'support backend payload custody proof required before any support payload delete claim',
      ],
      sourceProofRefs: ['production-support-backend-upload-custody-audit-proof'],
    }),
    deleteExecutorRow('status-backend-payload-delete-blocked', 'status-backend-payload', 'blocked-before-runtime', {
      custodyBoundary: 'no-hosted-payload-custody',
      manualProofRequirements: ['status backend payload custody proof required before status payload delete claim'],
      sourceProofRefs: ['production-support-status-backend-payload-custody-proof'],
    }),
    deleteExecutorRow('public-runtime-payload-delete-unavailable', 'public-runtime-payload', 'executor-unavailable', {
      custodyBoundary: 'not-applicable-before-runtime',
      manualProofRequirements: ['public runtime implementation required before public payload delete executor claim'],
      sourceProofRefs: ['production-release-public-runtime-handoff-proof'],
    }),
    deleteExecutorRow(
      'legal-disclosure-payload-delete-manual-required',
      'legal-disclosure-payload',
      'executor-manual-required',
      {
        custodyBoundary: 'not-applicable-before-runtime',
        manualProofRequirements: [
          'legal disclosure execution proof required before legal payload delete executor readiness claim',
          'privacy/legal operator approval proof required before legal delete execution claim',
        ],
        sourceProofRefs: ['production-support-privacy-legal-disclosure-status-proof'],
      }
    ),
  ],
});

function deleteExecutorRow(
  rowId: string,
  target: DeleteExecutorTarget,
  status: DeleteExecutorStatus,
  overrides: DeleteExecutorRowOverrides = {}
): DeleteExecutorRow {
  return DeleteExecutorRowSchema.parse({
    schemaVersion: 1,
    rowId,
    target,
    status,
    custodyBoundary: 'not-applicable-before-runtime',
    disclosedDataClasses: [...DeleteExecutorRequiredDataClasses],
    deleteRequestRefs: [`${target}-delete-request-ref`],
    authorizationRefs: [`${target}-parent-authorization-ref`],
    redactionAuditRefs: [`${target}-redaction-audit-ref`],
    custodyRefs: [`${target}-custody-boundary-ref`],
    sourceProofRefs: ['production-support-delete-executor-proof'],
    manualProofRequirements: ['manual delete executor proof required before execution claim'],
    realDeleteExecuted: false,
    durableQueueExecuted: false,
    payloadDeletionExecuted: false,
    providerExecutionOccurred: false,
    publicRuntimeExecuted: false,
    legalExecutionOccurred: false,
    backendUploadExecuted: false,
    productionSlaClaimed: false,
    childActivityCustodyClaimed: false,
    ocentraHostedFamilyDataDefault: false,
    containsRawChildActivity: false,
    containsRawSupportBundlePayload: false,
    containsProviderSecrets: false,
    containsRemoteSupportTranscripts: false,
    lastCheckedAt: generatedAt,
    ...overrides,
  });
}
