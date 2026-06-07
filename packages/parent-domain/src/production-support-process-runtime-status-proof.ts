import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ForbiddenProductionSupportProcessRuntimeStatusDataClasses,
  ProductionSupportProcessRuntimeStatusDataClassSchema,
  ProductionSupportProcessRuntimeStatusNonClaimSchema,
  ProductionSupportProcessRuntimeStatusReferenceSchema,
  ProductionSupportProcessRuntimeStatusRequirementSchema,
  ProductionSupportProcessRuntimeStatusSchemaVersionSchema,
  ProductionSupportProcessRuntimeStatusSourceProofSchema,
  ProductionSupportProcessRuntimeStatusStateSchema,
  ProductionSupportProcessRuntimeStatusSurfaceSchema,
  RequiredProductionSupportProcessRuntimeStatusNonClaims,
  RequiredProductionSupportProcessRuntimeStatusSurfaces,
} from './production-support-process-runtime-status-values';

export * from './production-support-process-runtime-status-values';

type ProductionSupportProcessRuntimeStatusProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly backendUploadExecutionState: string;
  readonly publicRuntimeExecutionState: string;
  readonly providerExecutionState: string;
  readonly incidentRuntimeExecutionState: string;
  readonly productionSlaState: string;
  readonly remoteSupportSessionState: string;
  readonly childActivityCustodyState: string;
  readonly defaultOcentraHostedFamilyDataState: string;
};

export const ProductionSupportProcessRuntimeStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportProcessRuntimeStatusSchemaVersionSchema,
    surface: ProductionSupportProcessRuntimeStatusSurfaceSchema,
    sourceProof: ProductionSupportProcessRuntimeStatusSourceProofSchema,
    runtimeState: ProductionSupportProcessRuntimeStatusStateSchema,
    parentConsentState: ProductionSupportProcessRuntimeStatusStateSchema,
    privacyLegalState: ProductionSupportProcessRuntimeStatusStateSchema,
    redactionReviewState: ProductionSupportProcessRuntimeStatusStateSchema,
    backendUploadState: ProductionSupportProcessRuntimeStatusStateSchema,
    caseResolutionState: ProductionSupportProcessRuntimeStatusStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportProcessRuntimeStatusDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportProcessRuntimeStatusDataClassSchema),
    runtimeReference: ProductionSupportProcessRuntimeStatusReferenceSchema,
    auditReference: ProductionSupportProcessRuntimeStatusReferenceSchema,
    manualRequirement: ProductionSupportProcessRuntimeStatusRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.backendUploadState !== 'executed' ||
        'Expected support process runtime rows to avoid backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenProductionSupportProcessRuntimeStatusDataClasses.includes(dataClass as never)
        ) || 'Expected support process runtime rows to exclude sensitive support data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenProductionSupportProcessRuntimeStatusDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected support process runtime rows to enumerate forbidden support data classes'
    )
  )
);

export const ProductionSupportProcessRuntimeStatusProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportProcessRuntimeStatusSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportProcessRuntimeStatusRowSchema),
    nonClaims: Schema.Array(ProductionSupportProcessRuntimeStatusNonClaimSchema),
    backendUploadExecutionState: ProductionSupportProcessRuntimeStatusStateSchema,
    publicRuntimeExecutionState: ProductionSupportProcessRuntimeStatusStateSchema,
    providerExecutionState: ProductionSupportProcessRuntimeStatusStateSchema,
    incidentRuntimeExecutionState: ProductionSupportProcessRuntimeStatusStateSchema,
    productionSlaState: ProductionSupportProcessRuntimeStatusStateSchema,
    remoteSupportSessionState: ProductionSupportProcessRuntimeStatusStateSchema,
    childActivityCustodyState: ProductionSupportProcessRuntimeStatusStateSchema,
    defaultOcentraHostedFamilyDataState: ProductionSupportProcessRuntimeStatusStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportProcessRuntimeStatusProofIsHonest(proof) ||
        'Expected support process runtime status proof to cover rows while preserving non-claims'
    )
  )
);

export type ProductionSupportProcessRuntimeStatusRow = Infer<typeof ProductionSupportProcessRuntimeStatusRowSchema>;
export type ProductionSupportProcessRuntimeStatusProof = Infer<typeof ProductionSupportProcessRuntimeStatusProofSchema>;
export type ProductionSupportProcessRuntimeStatusSurface = Infer<
  typeof ProductionSupportProcessRuntimeStatusSurfaceSchema
>;

export const decodeProductionSupportProcessRuntimeStatusProof = Schema.decodeUnknownSync(
  ProductionSupportProcessRuntimeStatusProofSchema
);

export function summarizeProductionSupportProcessRuntimeStatusRows(
  rows: ReadonlyArray<ProductionSupportProcessRuntimeStatusRow>
): Record<ProductionSupportProcessRuntimeStatusSurface, number> {
  return RequiredProductionSupportProcessRuntimeStatusSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<ProductionSupportProcessRuntimeStatusSurface, number>
  );
}

function productionSupportProcessRuntimeStatusProofIsHonest(
  proof: ProductionSupportProcessRuntimeStatusProofCandidate
): boolean {
  return (
    RequiredProductionSupportProcessRuntimeStatusSurfaces.every((surfaceName) =>
      proof.rows.some((row) => row.surface === surfaceName)
    ) &&
    RequiredProductionSupportProcessRuntimeStatusNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.backendUploadExecutionState === 'manual-required' &&
    proof.publicRuntimeExecutionState === 'not-implemented' &&
    proof.providerExecutionState === 'not-implemented' &&
    proof.incidentRuntimeExecutionState === 'manual-required' &&
    proof.productionSlaState === 'not-implemented' &&
    proof.remoteSupportSessionState === 'not-implemented' &&
    proof.childActivityCustodyState === 'not-implemented' &&
    proof.defaultOcentraHostedFamilyDataState === 'not-implemented'
  );
}
