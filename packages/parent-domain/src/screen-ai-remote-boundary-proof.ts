import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';
import { ParentEvidenceReferenceSchema } from './references';

const NonEmptyRemoteBoundaryText = Schema.String.pipe(Schema.minLength(1));

export const ScreenAiRemoteBoundaryProofIdSchema = NonEmptyRemoteBoundaryText.pipe(
  Schema.brand('ScreenAiRemoteBoundaryProofId')
);
export const ScreenAiRemoteBoundaryRowIdSchema = NonEmptyRemoteBoundaryText.pipe(
  Schema.brand('ScreenAiRemoteBoundaryRowId')
);
export const ScreenAiRemoteBoundaryArtifactRefSchema = NonEmptyRemoteBoundaryText.pipe(
  Schema.brand('ScreenAiRemoteBoundaryArtifactRef')
);
export const ScreenAiRemoteBoundaryEvidenceKindSchema = withParser(
  Schema.Literal('screen-summary', 'screen-ai-result', 'parent-assistant-context', 'parent-report-context')
);
export const ScreenAiRemoteBoundaryPurposeSchema = withParser(
  Schema.Literal('child-safety-screen-analysis', 'parent-assistant', 'parent-report')
);
export const ScreenAiRemoteBoundaryStateSchema = withParser(
  Schema.Literal(
    'child-safety-local-only',
    'parent-assistant-api-authorized-unavailable',
    'parent-report-api-authorized-degraded'
  )
);
export const ScreenAiRemoteBoundaryDecisionSchema = withParser(
  Schema.Literal('route-child-local', 'surface-parent-unavailable', 'surface-parent-degraded')
);

export const ScreenAiRemoteBoundaryClaimFlagsSchema = withParser(
  Schema.Struct({
    remoteAiUsedForChildSafety: Schema.Boolean,
    remoteApiAllowedForChildSafety: Schema.Boolean,
    remoteResultCanSetPolicy: Schema.Boolean,
    remoteResultCanEnforce: Schema.Boolean,
    rawScreenImageRetained: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (flags) =>
        Object.values(flags).every((flag) => flag === false) ||
        'Expected screen AI remote boundary proof to reject remote child-safety, policy, enforcement, and raw-image claims'
    )
  )
);

const ScreenAiRemoteBoundaryRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: ScreenAiRemoteBoundaryRowIdSchema,
  purpose: ScreenAiRemoteBoundaryPurposeSchema,
  evidenceKind: ScreenAiRemoteBoundaryEvidenceKindSchema,
  sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  sourceArtifacts: Schema.Array(ScreenAiRemoteBoundaryArtifactRefSchema),
  boundaryState: ScreenAiRemoteBoundaryStateSchema,
  decision: ScreenAiRemoteBoundaryDecisionSchema,
  childSafetyInputAllowed: Schema.Boolean,
  parentOnlySurfaceAllowed: Schema.Boolean,
  localRuntimeRequired: Schema.Boolean,
  remoteApiCredentialState: Schema.Literal('not-used', 'authorized-unavailable', 'authorized-degraded'),
  remoteApiExecutionState: Schema.Literal('not-executed', 'unavailable', 'degraded'),
  rawImageState: Schema.Literal('deleted'),
  claimFlags: ScreenAiRemoteBoundaryClaimFlagsSchema,
});

type ScreenAiRemoteBoundaryRowCandidate = Infer<typeof ScreenAiRemoteBoundaryRowBaseSchema>;

export const ScreenAiRemoteBoundaryRowSchema = withParser(
  ScreenAiRemoteBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenAiRemoteBoundaryRowIsHonest(row) ||
        'Expected screen AI remote boundary rows to keep child safety local-only and parent remote states non-authoritative'
    )
  )
);

const ScreenAiRemoteBoundaryProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: ScreenAiRemoteBoundaryProofIdSchema,
  generatedAt: ParentTimestampSchema,
  rows: Schema.Array(ScreenAiRemoteBoundaryRowSchema),
});

type ScreenAiRemoteBoundaryProofCandidate = Infer<typeof ScreenAiRemoteBoundaryProofBaseSchema>;

export const ScreenAiRemoteBoundaryProofSchema = withParser(
  ScreenAiRemoteBoundaryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        screenAiRemoteBoundaryProofHasRequiredRows(proof) ||
        'Expected screen AI remote boundary proof to include child safety, parent assistant, and parent report rows'
    )
  )
);

export function screenAiRemoteBoundaryProofCoversRequiredRows(proof: ScreenAiRemoteBoundaryProof): boolean {
  const rowIds = new Set<string>(proof.rows.map((row) => String(row.rowId)));
  return RequiredScreenAiRemoteBoundaryRowIds.every((rowId) => rowIds.has(rowId));
}

export function summarizeScreenAiRemoteBoundaryProof(proof: ScreenAiRemoteBoundaryProof) {
  return {
    rowCount: proof.rows.length,
    childSafetyLocalOnlyRows: proof.rows.filter((row) => row.boundaryState === 'child-safety-local-only').length,
    parentOnlyRemoteStateRows: proof.rows.filter((row) => row.parentOnlySurfaceAllowed).length,
    childSafetyRemoteClaimRows: proof.rows.filter((row) => row.claimFlags.remoteAiUsedForChildSafety).length,
    remotePolicyAuthorityRows: proof.rows.filter((row) => row.claimFlags.remoteResultCanSetPolicy).length,
    remoteEnforcementRows: proof.rows.filter((row) => row.claimFlags.remoteResultCanEnforce).length,
    rawImageRetainedRows: proof.rows.filter((row) => row.claimFlags.rawScreenImageRetained).length,
  };
}

function screenAiRemoteBoundaryProofHasRequiredRows(proof: ScreenAiRemoteBoundaryProofCandidate): boolean {
  const rowIds = new Set<string>(proof.rows.map((row) => String(row.rowId)));
  return RequiredScreenAiRemoteBoundaryRowIds.every((rowId) => rowIds.has(rowId));
}

function screenAiRemoteBoundaryRowIsHonest(row: ScreenAiRemoteBoundaryRowCandidate): boolean {
  if (
    row.sourceEvidenceReferences.length === 0 ||
    row.sourceArtifacts.length === 0 ||
    row.rawImageState !== 'deleted'
  ) {
    return false;
  }

  switch (row.purpose) {
    case 'child-safety-screen-analysis':
      return childSafetyRemoteBoundaryRowIsHonest(row);
    case 'parent-assistant':
      return parentAssistantRemoteBoundaryRowIsHonest(row);
    case 'parent-report':
      return parentReportRemoteBoundaryRowIsHonest(row);
  }
}

function childSafetyRemoteBoundaryRowIsHonest(row: ScreenAiRemoteBoundaryRowCandidate): boolean {
  return (
    row.boundaryState === 'child-safety-local-only' &&
    row.decision === 'route-child-local' &&
    row.childSafetyInputAllowed === true &&
    row.parentOnlySurfaceAllowed === false &&
    row.localRuntimeRequired === true &&
    row.remoteApiCredentialState === 'not-used' &&
    row.remoteApiExecutionState === 'not-executed'
  );
}

function parentAssistantRemoteBoundaryRowIsHonest(row: ScreenAiRemoteBoundaryRowCandidate): boolean {
  return (
    row.boundaryState === 'parent-assistant-api-authorized-unavailable' &&
    row.decision === 'surface-parent-unavailable' &&
    parentOnlyRemoteBoundaryRowIsHonest(row, 'authorized-unavailable', 'unavailable')
  );
}

function parentReportRemoteBoundaryRowIsHonest(row: ScreenAiRemoteBoundaryRowCandidate): boolean {
  return (
    row.boundaryState === 'parent-report-api-authorized-degraded' &&
    row.decision === 'surface-parent-degraded' &&
    parentOnlyRemoteBoundaryRowIsHonest(row, 'authorized-degraded', 'degraded')
  );
}

function parentOnlyRemoteBoundaryRowIsHonest(
  row: ScreenAiRemoteBoundaryRowCandidate,
  credentialState: 'authorized-unavailable' | 'authorized-degraded',
  executionState: 'unavailable' | 'degraded'
): boolean {
  return (
    row.childSafetyInputAllowed === false &&
    row.parentOnlySurfaceAllowed === true &&
    row.localRuntimeRequired === false &&
    row.remoteApiCredentialState === credentialState &&
    row.remoteApiExecutionState === executionState
  );
}

const RequiredScreenAiRemoteBoundaryRowIds = [
  'screen-ai-child-safety-local-only',
  'screen-ai-parent-assistant-api-unavailable',
  'screen-ai-parent-report-api-degraded',
] as const;

export type ScreenAiRemoteBoundaryProofId = typeof ScreenAiRemoteBoundaryProofIdSchema.Type;
export type ScreenAiRemoteBoundaryRowId = typeof ScreenAiRemoteBoundaryRowIdSchema.Type;
export type ScreenAiRemoteBoundaryArtifactRef = typeof ScreenAiRemoteBoundaryArtifactRefSchema.Type;
export type ScreenAiRemoteBoundaryEvidenceKind = Infer<typeof ScreenAiRemoteBoundaryEvidenceKindSchema>;
export type ScreenAiRemoteBoundaryPurpose = Infer<typeof ScreenAiRemoteBoundaryPurposeSchema>;
export type ScreenAiRemoteBoundaryState = Infer<typeof ScreenAiRemoteBoundaryStateSchema>;
export type ScreenAiRemoteBoundaryDecision = Infer<typeof ScreenAiRemoteBoundaryDecisionSchema>;
export type ScreenAiRemoteBoundaryClaimFlags = Infer<typeof ScreenAiRemoteBoundaryClaimFlagsSchema>;
export type ScreenAiRemoteBoundaryRow = Infer<typeof ScreenAiRemoteBoundaryRowSchema>;
export type ScreenAiRemoteBoundaryProof = Infer<typeof ScreenAiRemoteBoundaryProofSchema>;
