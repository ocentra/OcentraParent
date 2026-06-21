import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ProductionSupportProofStatusMatrixClosureAreaSchema,
  ProductionSupportProofStatusMatrixClosureNonClaimSchema,
  ProductionSupportProofStatusMatrixClosureSchemaVersionSchema,
  ProductionSupportProofStatusMatrixClosureSourceProofSchema,
  ProductionSupportProofStatusMatrixClosureStateSchema,
  ProofStatusMatrixClosureReferenceSchema,
  RequiredProofStatusMatrixClosureAreas,
  RequiredProofStatusMatrixClosureNonClaims,
  RequiredProofStatusMatrixClosureSourceProofs,
} from './production-support-proof-status-matrix-closure-values';

type MatrixClosureCandidate = {
  readonly rows: ReadonlyArray<{ readonly area: string; readonly sourceProofRefs: ReadonlyArray<string> }>;
  readonly sourceProofRefs: ReadonlyArray<string>;
  readonly nonClaims: ReadonlyArray<string>;
};

type MatrixClosureRowCandidate = {
  readonly proofState: string;
  readonly runtimeState: string;
  readonly backendExecutionState: string;
  readonly publicRuntimeState: string;
  readonly legalExecutionState: string;
  readonly providerSecretCustodyState: string;
  readonly childActivityCustodyState: string;
};

export const ProductionSupportProofStatusMatrixClosureRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportProofStatusMatrixClosureSchemaVersionSchema,
    area: ProductionSupportProofStatusMatrixClosureAreaSchema,
    proofState: ProductionSupportProofStatusMatrixClosureStateSchema,
    runtimeState: ProductionSupportProofStatusMatrixClosureStateSchema,
    backendExecutionState: ProductionSupportProofStatusMatrixClosureStateSchema,
    publicRuntimeState: ProductionSupportProofStatusMatrixClosureStateSchema,
    legalExecutionState: ProductionSupportProofStatusMatrixClosureStateSchema,
    providerSecretCustodyState: ProductionSupportProofStatusMatrixClosureStateSchema,
    childActivityCustodyState: ProductionSupportProofStatusMatrixClosureStateSchema,
    sourceProofRefs: Schema.Array(ProductionSupportProofStatusMatrixClosureSourceProofSchema),
    matrixRef: ProofStatusMatrixClosureReferenceSchema,
    nextManualProofRef: ProofStatusMatrixClosureReferenceSchema,
  }).pipe(
    Schema.filter(
      (row) => row.sourceProofRefs.length > 0 || 'Expected every proof/status matrix closure row to link source proofs'
    ),
    Schema.filter(
      (row) =>
        matrixClosureRowPreservesNonClaims(row) ||
        'Expected proof/status matrix closure rows to preserve runtime, backend, legal, provider-secret, and child-custody non-claims'
    )
  )
);

export const ProductionSupportProofStatusMatrixClosureProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportProofStatusMatrixClosureSchemaVersionSchema,
    sourceProofRefs: Schema.Array(ProductionSupportProofStatusMatrixClosureSourceProofSchema),
    rows: Schema.Array(ProductionSupportProofStatusMatrixClosureRowSchema),
    nonClaims: Schema.Array(ProductionSupportProofStatusMatrixClosureNonClaimSchema),
    publicRuntimeClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    statusBackendExecutionClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    signingStoreClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    updaterExecutionClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    accountBillingProviderExecutionClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    productionSlaClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    providerSecretCustodyClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    childActivityCustodyClaim: ProductionSupportProofStatusMatrixClosureStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportProofStatusMatrixClosureIsComplete(proof) ||
        'Expected proof/status matrix closure to cover all required areas, source proofs, and non-claims'
    )
  )
);

export type ProductionSupportProofStatusMatrixClosureRow = Infer<
  typeof ProductionSupportProofStatusMatrixClosureRowSchema
>;
export type ProductionSupportProofStatusMatrixClosureProof = Infer<
  typeof ProductionSupportProofStatusMatrixClosureProofSchema
>;
export type ProductionSupportProofStatusMatrixClosureArea = Infer<
  typeof ProductionSupportProofStatusMatrixClosureAreaSchema
>;

export const decodeProductionSupportProofStatusMatrixClosureProof = Schema.decodeUnknownSync(
  ProductionSupportProofStatusMatrixClosureProofSchema
);

export function summarizeProductionSupportProofStatusMatrixClosureRows(
  rows: ReadonlyArray<ProductionSupportProofStatusMatrixClosureRow>
): Record<ProductionSupportProofStatusMatrixClosureArea, number> {
  return RequiredProofStatusMatrixClosureAreas.reduce(
    (summary, area) => ({
      ...summary,
      [area]: rows.filter((row) => row.area === area).length,
    }),
    {} as Record<ProductionSupportProofStatusMatrixClosureArea, number>
  );
}

function matrixClosureRowPreservesNonClaims(row: MatrixClosureRowCandidate): boolean {
  return (
    row.proofState === 'source-proof-present' &&
    row.runtimeState !== 'source-proof-present' &&
    row.backendExecutionState !== 'source-proof-present' &&
    row.publicRuntimeState !== 'source-proof-present' &&
    row.legalExecutionState !== 'source-proof-present' &&
    row.providerSecretCustodyState !== 'source-proof-present' &&
    row.childActivityCustodyState !== 'source-proof-present'
  );
}

function productionSupportProofStatusMatrixClosureIsComplete(proof: MatrixClosureCandidate): boolean {
  return (
    RequiredProofStatusMatrixClosureAreas.every((area) => proof.rows.some((row) => row.area === area)) &&
    RequiredProofStatusMatrixClosureSourceProofs.every(
      (sourceProof) =>
        proof.sourceProofRefs.includes(sourceProof) &&
        proof.rows.some((row) => row.sourceProofRefs.includes(sourceProof))
    ) &&
    RequiredProofStatusMatrixClosureNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim))
  );
}
