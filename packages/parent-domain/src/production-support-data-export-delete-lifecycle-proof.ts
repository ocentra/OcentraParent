import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ForbiddenProductionSupportDataExportDeleteLifecycleDataClasses,
  ProductionSupportDataExportDeleteLifecycleDataClassSchema,
  ProductionSupportDataExportDeleteLifecycleNonClaimSchema,
  ProductionSupportDataExportDeleteLifecycleOperationSchema,
  ProductionSupportDataExportDeleteLifecycleReferenceSchema,
  ProductionSupportDataExportDeleteLifecycleRequirementSchema,
  ProductionSupportDataExportDeleteLifecycleSchemaVersionSchema,
  ProductionSupportDataExportDeleteLifecycleSourceProofSchema,
  ProductionSupportDataExportDeleteLifecycleStateSchema,
  ProductionSupportDataExportDeleteLifecycleSurfaceSchema,
  RequiredProductionSupportDataExportDeleteLifecycleNonClaims,
  RequiredProductionSupportDataExportDeleteLifecycleSurfaces,
} from './production-support-data-export-delete-lifecycle-values';

export * from './production-support-data-export-delete-lifecycle-values';

type ProductionSupportDataExportDeleteLifecycleProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly backendUploadExecutionState: string;
  readonly publicRuntimeExecutionState: string;
  readonly providerExecutionState: string;
  readonly productionSlaState: string;
  readonly remoteSupportSessionState: string;
  readonly childActivityCustodyState: string;
};

export const ProductionSupportDataExportDeleteLifecycleRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportDataExportDeleteLifecycleSchemaVersionSchema,
    surface: ProductionSupportDataExportDeleteLifecycleSurfaceSchema,
    operation: ProductionSupportDataExportDeleteLifecycleOperationSchema,
    lifecycleState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    sourceProof: ProductionSupportDataExportDeleteLifecycleSourceProofSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportDataExportDeleteLifecycleDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportDataExportDeleteLifecycleDataClassSchema),
    lifecycleReferences: Schema.Array(ProductionSupportDataExportDeleteLifecycleReferenceSchema),
    manualRequirement: ProductionSupportDataExportDeleteLifecycleRequirementSchema,
    backendUploadExecutionState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    publicRuntimeExecutionState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    providerExecutionState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    productionSlaState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    remoteSupportSessionState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    childActivityCustodyState: ProductionSupportDataExportDeleteLifecycleStateSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.backendUploadExecutionState === 'not-implemented' ||
        'Expected export/delete lifecycle rows to avoid support backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.publicRuntimeExecutionState === 'not-implemented' ||
        'Expected export/delete lifecycle rows to avoid public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        row.providerExecutionState === 'not-implemented' ||
        'Expected export/delete lifecycle rows to avoid provider execution claims'
    ),
    Schema.filter(
      (row) =>
        row.productionSlaState === 'not-implemented' ||
        'Expected export/delete lifecycle rows to avoid production SLA claims'
    ),
    Schema.filter(
      (row) =>
        row.remoteSupportSessionState === 'not-implemented' ||
        'Expected export/delete lifecycle rows to avoid remote support session claims'
    ),
    Schema.filter(
      (row) =>
        row.childActivityCustodyState === 'not-implemented' ||
        'Expected export/delete lifecycle rows to avoid child activity custody claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenProductionSupportDataExportDeleteLifecycleDataClasses.includes(dataClass as never)
        ) || 'Expected export/delete lifecycle rows to exclude forbidden custody, provider, runtime, and SLA data'
    ),
    Schema.filter(
      (row) =>
        ForbiddenProductionSupportDataExportDeleteLifecycleDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected export/delete lifecycle rows to enumerate forbidden data classes'
    )
  )
);

export const ProductionSupportDataExportDeleteLifecycleProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportDataExportDeleteLifecycleSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportDataExportDeleteLifecycleRowSchema),
    nonClaims: Schema.Array(ProductionSupportDataExportDeleteLifecycleNonClaimSchema),
    backendUploadExecutionState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    publicRuntimeExecutionState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    providerExecutionState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    productionSlaState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    remoteSupportSessionState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    childActivityCustodyState: ProductionSupportDataExportDeleteLifecycleStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportDataExportDeleteLifecycleProofIsHonest(proof) ||
        'Expected export/delete lifecycle proof to cover requested, authorized, queued, running, succeeded, failed, and manual-required rows while preserving non-claims'
    )
  )
);

export type ProductionSupportDataExportDeleteLifecycleRow = Infer<
  typeof ProductionSupportDataExportDeleteLifecycleRowSchema
>;
export type ProductionSupportDataExportDeleteLifecycleProof = Infer<
  typeof ProductionSupportDataExportDeleteLifecycleProofSchema
>;
export type ProductionSupportDataExportDeleteLifecycleSurface = Infer<
  typeof ProductionSupportDataExportDeleteLifecycleSurfaceSchema
>;

export const decodeProductionSupportDataExportDeleteLifecycleProof = Schema.decodeUnknownSync(
  ProductionSupportDataExportDeleteLifecycleProofSchema
);

export function summarizeProductionSupportDataExportDeleteLifecycleRows(
  rows: ReadonlyArray<ProductionSupportDataExportDeleteLifecycleRow>
): Record<ProductionSupportDataExportDeleteLifecycleSurface, number> {
  return RequiredProductionSupportDataExportDeleteLifecycleSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<ProductionSupportDataExportDeleteLifecycleSurface, number>
  );
}

function productionSupportDataExportDeleteLifecycleProofIsHonest(
  proof: ProductionSupportDataExportDeleteLifecycleProofCandidate
): boolean {
  return (
    RequiredProductionSupportDataExportDeleteLifecycleSurfaces.every((surfaceName) =>
      proof.rows.some((row) => row.surface === surfaceName)
    ) &&
    RequiredProductionSupportDataExportDeleteLifecycleNonClaims.every((nonClaim) =>
      proof.nonClaims.includes(nonClaim)
    ) &&
    proof.backendUploadExecutionState === 'not-implemented' &&
    proof.publicRuntimeExecutionState === 'not-implemented' &&
    proof.providerExecutionState === 'not-implemented' &&
    proof.productionSlaState === 'not-implemented' &&
    proof.remoteSupportSessionState === 'not-implemented' &&
    proof.childActivityCustodyState === 'not-implemented'
  );
}
