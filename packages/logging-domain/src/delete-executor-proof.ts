import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

import {
  supportProofHasAnyClaimUpgrade,
  supportProofRequiredValuesArePresent,
} from './support-proof-contract.js';

const deleteExecutorText = <Brand extends string>(brand: Brand) => NonEmptyStringSchema.pipe(Schema.brand(brand));

export const DeleteExecutorReadModelIdSchema = deleteExecutorText('DeleteExecutorReadModelId');
export const DeleteExecutorRowIdSchema = deleteExecutorText('DeleteExecutorRowId');
export const DeleteExecutorReferenceSchema = deleteExecutorText('DeleteExecutorReference');
export const DeleteExecutorRequirementSchema = deleteExecutorText('DeleteExecutorRequirement');
export const DeleteExecutorTimestampSchema = deleteExecutorText('DeleteExecutorTimestamp');

export const DeleteExecutorTargetSchema = withParser(
  Schema.Literal(
    'local-export-output',
    'support-backend-payload',
    'status-backend-payload',
    'public-runtime-payload',
    'legal-disclosure-payload'
  )
);
export const DeleteExecutorStatusSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'delete-request-recorded',
    'executor-manual-required',
    'executor-unavailable',
    'blocked-before-runtime'
  )
);
export const DeleteExecutorCustodyBoundarySchema = withParser(
  Schema.Literal('parent-owned-local-output-only', 'no-hosted-payload-custody', 'not-applicable-before-runtime')
);

export const DeleteExecutorDataClassSchema = withParser(
  Schema.Literal(
    'delete-request-ref',
    'authorization-ref',
    'redaction-audit-ref',
    'custody-boundary-ref',
    'manual-proof-ref',
    'source-proof-ref'
  )
);

export const DeleteExecutorRequiredDataClasses = [
  'delete-request-ref',
  'authorization-ref',
  'redaction-audit-ref',
  'custody-boundary-ref',
  'manual-proof-ref',
  'source-proof-ref',
] as const satisfies ReadonlyArray<DeleteExecutorDataClass>;

const DeleteExecutorRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  rowId: DeleteExecutorRowIdSchema,
  target: DeleteExecutorTargetSchema,
  status: DeleteExecutorStatusSchema,
  custodyBoundary: DeleteExecutorCustodyBoundarySchema,
  disclosedDataClasses: Schema.Array(DeleteExecutorDataClassSchema),
  deleteRequestRefs: Schema.Array(DeleteExecutorReferenceSchema),
  authorizationRefs: Schema.Array(DeleteExecutorReferenceSchema),
  redactionAuditRefs: Schema.Array(DeleteExecutorReferenceSchema),
  custodyRefs: Schema.Array(DeleteExecutorReferenceSchema),
  sourceProofRefs: Schema.Array(DeleteExecutorReferenceSchema),
  manualProofRequirements: Schema.Array(DeleteExecutorRequirementSchema),
  realDeleteExecuted: Schema.Boolean,
  durableQueueExecuted: Schema.Boolean,
  payloadDeletionExecuted: Schema.Boolean,
  providerExecutionOccurred: Schema.Boolean,
  publicRuntimeExecuted: Schema.Boolean,
  legalExecutionOccurred: Schema.Boolean,
  backendUploadExecuted: Schema.Boolean,
  productionSlaClaimed: Schema.Boolean,
  childActivityCustodyClaimed: Schema.Boolean,
  ocentraHostedFamilyDataDefault: Schema.Boolean,
  containsRawChildActivity: Schema.Boolean,
  containsRawSupportBundlePayload: Schema.Boolean,
  containsProviderSecrets: Schema.Boolean,
  containsRemoteSupportTranscripts: Schema.Boolean,
  lastCheckedAt: DeleteExecutorTimestampSchema,
});

export type DeleteExecutorRowCandidate = Infer<typeof DeleteExecutorRowBaseSchema>;

export const DeleteExecutorRowSchema = withParser(
  DeleteExecutorRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        deleteExecutorRowIsSupportSafe(row, DeleteExecutorRequiredDataClasses) ||
        'Expected delete executor rows to stay source-backed, manual/unavailable before runtime, custody-safe, and free of deletion/provider/public/legal execution or child activity custody claims'
    )
  )
);

export const DeleteExecutorReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: DeleteExecutorReadModelIdSchema,
    generatedAt: DeleteExecutorTimestampSchema,
    sourceContractRefs: Schema.Array(DeleteExecutorReferenceSchema),
    rows: Schema.Array(DeleteExecutorRowSchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length ||
        'Expected delete executor row ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        deleteExecutorReadModelCoversTargets(readModel.rows) ||
        'Expected delete executor proof to cover local export, support backend, status backend, public runtime, and legal disclosure targets'
    )
  )
);

export type DeleteExecutorTarget = Infer<typeof DeleteExecutorTargetSchema>;
export type DeleteExecutorStatus = Infer<typeof DeleteExecutorStatusSchema>;
export type DeleteExecutorCustodyBoundary = Infer<typeof DeleteExecutorCustodyBoundarySchema>;
export type DeleteExecutorDataClass = Infer<typeof DeleteExecutorDataClassSchema>;
export type DeleteExecutorRow = Infer<typeof DeleteExecutorRowSchema>;
export type DeleteExecutorReadModel = Infer<typeof DeleteExecutorReadModelSchema>;

export const decodeDeleteExecutorRow = Schema.decodeUnknownSync(DeleteExecutorRowSchema);
export const decodeDeleteExecutorReadModel = Schema.decodeUnknownSync(DeleteExecutorReadModelSchema);

export function summarizeDeleteExecutorTargets(
  rows: readonly DeleteExecutorRow[]
): Record<DeleteExecutorTarget, number> {
  return rows.reduce<Record<DeleteExecutorTarget, number>>(
    (counts, row) => ({ ...counts, [row.target]: counts[row.target] + 1 }),
    {
      'local-export-output': 0,
      'support-backend-payload': 0,
      'status-backend-payload': 0,
      'public-runtime-payload': 0,
      'legal-disclosure-payload': 0,
    }
  );
}

export function summarizeDeleteExecutorStatuses(
  rows: readonly DeleteExecutorRow[]
): Record<DeleteExecutorStatus, number> {
  return rows.reduce<Record<DeleteExecutorStatus, number>>(
    (counts, row) => ({ ...counts, [row.status]: counts[row.status] + 1 }),
    {
      'source-contract-ready': 0,
      'delete-request-recorded': 0,
      'executor-manual-required': 0,
      'executor-unavailable': 0,
      'blocked-before-runtime': 0,
    }
  );
}

function deleteExecutorRowIsSupportSafe(
  row: DeleteExecutorRowCandidate,
  requiredDataClasses: ReadonlyArray<DeleteExecutorDataClass>
): boolean {
  return (
    supportProofRequiredValuesArePresent(row.disclosedDataClasses, requiredDataClasses) &&
    deleteExecutorRefsArePresent(row) &&
    deleteExecutorManualProofIsPresent(row) &&
    !deleteExecutorHasOverclaim(row)
  );
}

function deleteExecutorReadModelCoversTargets(rows: readonly DeleteExecutorRow[]): boolean {
  const targets = new Set(rows.map((row) => row.target));
  return [
    'local-export-output',
    'support-backend-payload',
    'status-backend-payload',
    'public-runtime-payload',
    'legal-disclosure-payload',
  ].every((target) => targets.has(target as DeleteExecutorTarget));
}

function deleteExecutorRefsArePresent(row: DeleteExecutorRowCandidate): boolean {
  return (
    row.deleteRequestRefs.length > 0 &&
    row.authorizationRefs.length > 0 &&
    row.redactionAuditRefs.length > 0 &&
    row.custodyRefs.length > 0 &&
    row.sourceProofRefs.length > 0
  );
}

function deleteExecutorManualProofIsPresent(row: DeleteExecutorRowCandidate): boolean {
  return row.status !== 'executor-manual-required' || row.manualProofRequirements.length > 0;
}

function deleteExecutorHasOverclaim(row: DeleteExecutorRowCandidate): boolean {
  return supportProofHasAnyClaimUpgrade([
    row.realDeleteExecuted,
    row.durableQueueExecuted,
    row.payloadDeletionExecuted,
    row.providerExecutionOccurred,
    row.publicRuntimeExecuted,
    row.legalExecutionOccurred,
    row.backendUploadExecuted,
    row.productionSlaClaimed,
    row.childActivityCustodyClaimed,
    row.ocentraHostedFamilyDataDefault,
    row.containsRawChildActivity,
    row.containsRawSupportBundlePayload,
    row.containsProviderSecrets,
    row.containsRemoteSupportTranscripts,
  ]);
}

