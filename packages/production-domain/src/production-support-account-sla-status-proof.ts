import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ForbiddenProductionSupportAccountSlaStatusDataClasses,
  ProductionSupportAccountSlaStatusDataClassSchema,
  ProductionSupportAccountSlaStatusNonClaimSchema,
  ProductionSupportAccountSlaStatusReferenceSchema,
  ProductionSupportAccountSlaStatusRequirementSchema,
  ProductionSupportAccountSlaStatusSchemaVersionSchema,
  ProductionSupportAccountSlaStatusSourceProofSchema,
  ProductionSupportAccountSlaStatusStateSchema,
  ProductionSupportAccountSlaStatusSurfaceSchema,
  RequiredProductionSupportAccountSlaStatusNonClaims,
  RequiredProductionSupportAccountSlaStatusSurfaces,
} from './production-support-account-sla-status-values';

export * from './production-support-account-sla-status-values';

type ProductionSupportAccountSlaStatusProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly accountLookupExecutionState: string;
  readonly billingProviderContactState: string;
  readonly remoteSupportSessionState: string;
  readonly productionSlaState: string;
  readonly supportBackendUploadExecutionState: string;
  readonly familyOcentraRuntimeState: string;
  readonly childActivityCustodyState: string;
};

export const ProductionSupportAccountSlaStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportAccountSlaStatusSchemaVersionSchema,
    surface: ProductionSupportAccountSlaStatusSurfaceSchema,
    sourceProof: ProductionSupportAccountSlaStatusSourceProofSchema,
    sourceContractState: ProductionSupportAccountSlaStatusStateSchema,
    accountLookupState: ProductionSupportAccountSlaStatusStateSchema,
    billingProviderContactState: ProductionSupportAccountSlaStatusStateSchema,
    remoteSupportSessionState: ProductionSupportAccountSlaStatusStateSchema,
    productionSlaState: ProductionSupportAccountSlaStatusStateSchema,
    parentVisibleState: ProductionSupportAccountSlaStatusStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportAccountSlaStatusDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportAccountSlaStatusDataClassSchema),
    accountReference: ProductionSupportAccountSlaStatusReferenceSchema,
    supportReference: ProductionSupportAccountSlaStatusReferenceSchema,
    manualRequirement: ProductionSupportAccountSlaStatusRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.accountLookupState !== 'executed' ||
        'Expected account/SLA status rows to avoid account lookup execution claims'
    ),
    Schema.filter(
      (row) =>
        row.billingProviderContactState !== 'executed' ||
        'Expected account/SLA status rows to avoid billing provider contact claims'
    ),
    Schema.filter(
      (row) =>
        row.remoteSupportSessionState !== 'executed' ||
        'Expected account/SLA status rows to avoid remote support session claims'
    ),
    Schema.filter(
      (row) =>
        (row.productionSlaState !== 'implemented' && row.productionSlaState !== 'executed') ||
        'Expected account/SLA status rows to avoid production SLA commitments'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenProductionSupportAccountSlaStatusDataClasses.includes(dataClass as never)
        ) || 'Expected account/SLA rows to exclude forbidden provider, custody, and support transcript data'
    ),
    Schema.filter(
      (row) =>
        ForbiddenProductionSupportAccountSlaStatusDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected account/SLA rows to enumerate forbidden provider, custody, and support transcript data'
    )
  )
);

export const ProductionSupportAccountSlaStatusProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportAccountSlaStatusSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportAccountSlaStatusRowSchema),
    nonClaims: Schema.Array(ProductionSupportAccountSlaStatusNonClaimSchema),
    accountLookupExecutionState: ProductionSupportAccountSlaStatusStateSchema,
    billingProviderContactState: ProductionSupportAccountSlaStatusStateSchema,
    remoteSupportSessionState: ProductionSupportAccountSlaStatusStateSchema,
    productionSlaState: ProductionSupportAccountSlaStatusStateSchema,
    supportBackendUploadExecutionState: ProductionSupportAccountSlaStatusStateSchema,
    familyOcentraRuntimeState: ProductionSupportAccountSlaStatusStateSchema,
    childActivityCustodyState: ProductionSupportAccountSlaStatusStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportAccountSlaStatusProofIsHonest(proof) ||
        'Expected production support account/SLA status proof to cover all rows while preserving non-claims'
    )
  )
);

export type ProductionSupportAccountSlaStatusRow = Infer<typeof ProductionSupportAccountSlaStatusRowSchema>;
export type ProductionSupportAccountSlaStatusProof = Infer<typeof ProductionSupportAccountSlaStatusProofSchema>;
export type ProductionSupportAccountSlaStatusSurface = Infer<typeof ProductionSupportAccountSlaStatusSurfaceSchema>;

export const decodeProductionSupportAccountSlaStatusProof = Schema.decodeUnknownSync(
  ProductionSupportAccountSlaStatusProofSchema
);

export function summarizeProductionSupportAccountSlaStatusRows(
  rows: ReadonlyArray<ProductionSupportAccountSlaStatusRow>
): Record<ProductionSupportAccountSlaStatusSurface, number> {
  return RequiredProductionSupportAccountSlaStatusSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<ProductionSupportAccountSlaStatusSurface, number>
  );
}

function productionSupportAccountSlaStatusProofIsHonest(
  proof: ProductionSupportAccountSlaStatusProofCandidate
): boolean {
  return (
    RequiredProductionSupportAccountSlaStatusSurfaces.every((surfaceName) =>
      proof.rows.some((row) => row.surface === surfaceName)
    ) &&
    RequiredProductionSupportAccountSlaStatusNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.accountLookupExecutionState === 'manual-required' &&
    proof.billingProviderContactState === 'manual-required' &&
    proof.remoteSupportSessionState === 'not-implemented' &&
    proof.productionSlaState === 'not-implemented' &&
    proof.supportBackendUploadExecutionState === 'manual-required' &&
    proof.familyOcentraRuntimeState === 'not-implemented' &&
    proof.childActivityCustodyState === 'not-implemented'
  );
}
