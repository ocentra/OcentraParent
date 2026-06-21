import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ForbiddenProductionIncidentSupportStatusDataClasses,
  ProductionIncidentSupportStatusDataClassSchema,
  ProductionIncidentSupportStatusNonClaimSchema,
  ProductionIncidentSupportStatusReferenceSchema,
  ProductionIncidentSupportStatusRequirementSchema,
  ProductionIncidentSupportStatusSchemaVersionSchema,
  ProductionIncidentSupportStatusSourceProofSchema,
  ProductionIncidentSupportStatusStateSchema,
  ProductionIncidentSupportStatusSurfaceSchema,
  RequiredProductionIncidentSupportStatusNonClaims,
  RequiredProductionIncidentSupportStatusSurfaces,
} from './production-incident-support-status-values';

type ProductionIncidentSupportStatusProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicPublicationState: string;
  readonly legalExecutionState: string;
  readonly supportBackendUploadExecutionState: string;
  readonly accountLookupExecutionState: string;
  readonly billingProviderContactState: string;
  readonly remoteSupportSessionState: string;
  readonly productionSlaState: string;
  readonly childActivityCustodyState: string;
};

export const ProductionIncidentSupportStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionIncidentSupportStatusSchemaVersionSchema,
    surface: ProductionIncidentSupportStatusSurfaceSchema,
    sourceProof: ProductionIncidentSupportStatusSourceProofSchema,
    sourceContractState: ProductionIncidentSupportStatusStateSchema,
    parentConsentState: ProductionIncidentSupportStatusStateSchema,
    privacyLegalState: ProductionIncidentSupportStatusStateSchema,
    exportDeleteState: ProductionIncidentSupportStatusStateSchema,
    publicPublicationState: ProductionIncidentSupportStatusStateSchema,
    backendUploadState: ProductionIncidentSupportStatusStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionIncidentSupportStatusDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionIncidentSupportStatusDataClassSchema),
    incidentReference: ProductionIncidentSupportStatusReferenceSchema,
    custodyReference: ProductionIncidentSupportStatusReferenceSchema,
    manualRequirement: ProductionIncidentSupportStatusRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.publicPublicationState !== 'executed' ||
        'Expected incident support status rows to avoid public publication execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.backendUploadState !== 'implemented' && row.backendUploadState !== 'executed') ||
        'Expected incident support status rows to avoid backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenProductionIncidentSupportStatusDataClasses.includes(dataClass as never)
        ) || 'Expected incident support status rows to exclude sensitive support and custody data'
    ),
    Schema.filter(
      (row) =>
        ForbiddenProductionIncidentSupportStatusDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected incident support status rows to enumerate forbidden support data classes'
    )
  )
);

export const ProductionIncidentSupportStatusProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionIncidentSupportStatusSchemaVersionSchema,
    rows: Schema.Array(ProductionIncidentSupportStatusRowSchema),
    nonClaims: Schema.Array(ProductionIncidentSupportStatusNonClaimSchema),
    publicPublicationState: ProductionIncidentSupportStatusStateSchema,
    legalExecutionState: ProductionIncidentSupportStatusStateSchema,
    supportBackendUploadExecutionState: ProductionIncidentSupportStatusStateSchema,
    accountLookupExecutionState: ProductionIncidentSupportStatusStateSchema,
    billingProviderContactState: ProductionIncidentSupportStatusStateSchema,
    remoteSupportSessionState: ProductionIncidentSupportStatusStateSchema,
    productionSlaState: ProductionIncidentSupportStatusStateSchema,
    childActivityCustodyState: ProductionIncidentSupportStatusStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionIncidentSupportStatusProofIsHonest(proof) ||
        'Expected production incident support status proof to cover all rows while preserving non-claims'
    )
  )
);

export type ProductionIncidentSupportStatusRow = Infer<typeof ProductionIncidentSupportStatusRowSchema>;
export type ProductionIncidentSupportStatusProof = Infer<typeof ProductionIncidentSupportStatusProofSchema>;
export type ProductionIncidentSupportStatusSurface = Infer<typeof ProductionIncidentSupportStatusSurfaceSchema>;

export const decodeProductionIncidentSupportStatusProof = Schema.decodeUnknownSync(
  ProductionIncidentSupportStatusProofSchema
);

export function summarizeProductionIncidentSupportStatusRows(
  rows: ReadonlyArray<ProductionIncidentSupportStatusRow>
): Record<ProductionIncidentSupportStatusSurface, number> {
  return RequiredProductionIncidentSupportStatusSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<ProductionIncidentSupportStatusSurface, number>
  );
}

function productionIncidentSupportStatusProofIsHonest(proof: ProductionIncidentSupportStatusProofCandidate): boolean {
  return (
    RequiredProductionIncidentSupportStatusSurfaces.every((surfaceName) =>
      proof.rows.some((row) => row.surface === surfaceName)
    ) &&
    RequiredProductionIncidentSupportStatusNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicPublicationState === 'publication-required' &&
    proof.legalExecutionState === 'manual-required' &&
    proof.supportBackendUploadExecutionState === 'manual-required' &&
    proof.accountLookupExecutionState === 'manual-required' &&
    proof.billingProviderContactState === 'manual-required' &&
    proof.remoteSupportSessionState === 'not-implemented' &&
    proof.productionSlaState === 'not-implemented' &&
    proof.childActivityCustodyState === 'not-implemented'
  );
}
