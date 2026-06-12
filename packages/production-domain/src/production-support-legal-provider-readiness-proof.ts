import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ForbiddenProductionSupportLegalProviderReadinessDataClasses,
  ProductionSupportLegalProviderReadinessDataClassSchema,
  ProductionSupportLegalProviderReadinessNonClaimSchema,
  ProductionSupportLegalProviderReadinessReferenceSchema,
  ProductionSupportLegalProviderReadinessRequirementSchema,
  ProductionSupportLegalProviderReadinessSchemaVersionSchema,
  ProductionSupportLegalProviderReadinessSourceProofSchema,
  ProductionSupportLegalProviderReadinessStateSchema,
  ProductionSupportLegalProviderReadinessSurfaceSchema,
  RequiredProductionSupportLegalProviderReadinessNonClaims,
  RequiredProductionSupportLegalProviderReadinessSurfaces,
} from './production-support-legal-provider-readiness-values';

export * from './production-support-legal-provider-readiness-values';

type ProductionSupportLegalProviderReadinessProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly surface: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly legalDisclosureExecutionState: string;
  readonly dataExportDeleteRuntimeState: string;
  readonly providerSecretCustodyState: string;
  readonly billingProviderContactExecutionState: string;
  readonly accountLookupExecutionState: string;
  readonly remoteSupportSessionState: string;
  readonly productionSlaState: string;
  readonly supportBackendUploadExecutionState: string;
  readonly publicRuntimeExecutionState: string;
  readonly childActivityCustodyState: string;
};

export const ProductionSupportLegalProviderReadinessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportLegalProviderReadinessSchemaVersionSchema,
    surface: ProductionSupportLegalProviderReadinessSurfaceSchema,
    sourceProof: ProductionSupportLegalProviderReadinessSourceProofSchema,
    sourceContractState: ProductionSupportLegalProviderReadinessStateSchema,
    legalDisclosureState: ProductionSupportLegalProviderReadinessStateSchema,
    dataExportDeleteState: ProductionSupportLegalProviderReadinessStateSchema,
    providerSecretCustodyState: ProductionSupportLegalProviderReadinessStateSchema,
    billingProviderContactState: ProductionSupportLegalProviderReadinessStateSchema,
    remoteSupportSessionState: ProductionSupportLegalProviderReadinessStateSchema,
    productionSlaState: ProductionSupportLegalProviderReadinessStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportLegalProviderReadinessDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportLegalProviderReadinessDataClassSchema),
    readinessReferences: Schema.Array(ProductionSupportLegalProviderReadinessReferenceSchema),
    manualRequirement: ProductionSupportLegalProviderReadinessRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.legalDisclosureState !== 'executed' ||
        'Expected legal/provider readiness rows to avoid legal disclosure execution claims'
    ),
    Schema.filter(
      (row) =>
        row.dataExportDeleteState !== 'executed' ||
        'Expected legal/provider readiness rows to avoid export/delete runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        row.providerSecretCustodyState !== 'implemented' ||
        'Expected legal/provider readiness rows to avoid provider secret custody claims'
    ),
    Schema.filter(
      (row) =>
        row.billingProviderContactState !== 'executed' ||
        'Expected legal/provider readiness rows to avoid billing provider contact execution claims'
    ),
    Schema.filter(
      (row) =>
        row.remoteSupportSessionState !== 'executed' ||
        'Expected legal/provider readiness rows to avoid remote support session claims'
    ),
    Schema.filter(
      (row) =>
        (row.productionSlaState !== 'implemented' && row.productionSlaState !== 'executed') ||
        'Expected legal/provider readiness rows to avoid production SLA commitments'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenProductionSupportLegalProviderReadinessDataClasses.includes(dataClass as never)
        ) || 'Expected legal/provider readiness rows to exclude provider, custody, transcript, and SLA payloads'
    ),
    Schema.filter(
      (row) =>
        ForbiddenProductionSupportLegalProviderReadinessDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected legal/provider readiness rows to enumerate forbidden provider, custody, transcript, and SLA data'
    )
  )
);

export const ProductionSupportLegalProviderReadinessProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportLegalProviderReadinessSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportLegalProviderReadinessRowSchema),
    nonClaims: Schema.Array(ProductionSupportLegalProviderReadinessNonClaimSchema),
    legalDisclosureExecutionState: ProductionSupportLegalProviderReadinessStateSchema,
    dataExportDeleteRuntimeState: ProductionSupportLegalProviderReadinessStateSchema,
    providerSecretCustodyState: ProductionSupportLegalProviderReadinessStateSchema,
    billingProviderContactExecutionState: ProductionSupportLegalProviderReadinessStateSchema,
    accountLookupExecutionState: ProductionSupportLegalProviderReadinessStateSchema,
    remoteSupportSessionState: ProductionSupportLegalProviderReadinessStateSchema,
    productionSlaState: ProductionSupportLegalProviderReadinessStateSchema,
    supportBackendUploadExecutionState: ProductionSupportLegalProviderReadinessStateSchema,
    publicRuntimeExecutionState: ProductionSupportLegalProviderReadinessStateSchema,
    childActivityCustodyState: ProductionSupportLegalProviderReadinessStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportLegalProviderReadinessProofIsHonest(proof) ||
        'Expected production support legal/provider readiness proof to cover all rows while preserving non-claims'
    )
  )
);

export type ProductionSupportLegalProviderReadinessRow = Infer<typeof ProductionSupportLegalProviderReadinessRowSchema>;
export type ProductionSupportLegalProviderReadinessProof = Infer<
  typeof ProductionSupportLegalProviderReadinessProofSchema
>;
export type ProductionSupportLegalProviderReadinessSurface = Infer<
  typeof ProductionSupportLegalProviderReadinessSurfaceSchema
>;

export const decodeProductionSupportLegalProviderReadinessProof = Schema.decodeUnknownSync(
  ProductionSupportLegalProviderReadinessProofSchema
);

export function summarizeProductionSupportLegalProviderReadinessRows(
  rows: ReadonlyArray<ProductionSupportLegalProviderReadinessRow>
): Record<ProductionSupportLegalProviderReadinessSurface, number> {
  return RequiredProductionSupportLegalProviderReadinessSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<ProductionSupportLegalProviderReadinessSurface, number>
  );
}

function productionSupportLegalProviderReadinessProofIsHonest(
  proof: ProductionSupportLegalProviderReadinessProofCandidate
): boolean {
  return (
    RequiredProductionSupportLegalProviderReadinessSurfaces.every((surfaceName) =>
      proof.rows.some((row) => row.surface === surfaceName)
    ) &&
    RequiredProductionSupportLegalProviderReadinessNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.legalDisclosureExecutionState === 'manual-required' &&
    proof.dataExportDeleteRuntimeState === 'manual-required' &&
    proof.providerSecretCustodyState === 'not-implemented' &&
    proof.billingProviderContactExecutionState === 'manual-required' &&
    proof.accountLookupExecutionState === 'manual-required' &&
    proof.remoteSupportSessionState === 'not-implemented' &&
    proof.productionSlaState === 'not-implemented' &&
    proof.supportBackendUploadExecutionState === 'manual-required' &&
    proof.publicRuntimeExecutionState === 'not-implemented' &&
    proof.childActivityCustodyState === 'not-implemented'
  );
}
