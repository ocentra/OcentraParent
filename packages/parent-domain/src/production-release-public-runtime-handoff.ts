import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ForbiddenPublicRuntimeDataClasses,
  ProductionReleasePublicRuntimeAdapterSchema,
  ProductionReleasePublicRuntimeDataClassSchema,
  ProductionReleasePublicRuntimeExecutionClaimSchema,
  ProductionReleasePublicRuntimeHandoffSchemaVersionSchema,
  ProductionReleasePublicRuntimeNonClaimSchema,
  ProductionReleasePublicRuntimeSourceProofSchema,
  ProductionReleasePublicRuntimeStateSchema,
  ProductionReleasePublicRuntimeSurfaceSchema,
  ProductionReleasePublicRuntimeTargetSchema,
  PublicRuntimeHandoffReferenceSchema,
  PublicRuntimeHandoffRequirementSchema,
  RequiredPublicRuntimeAdapters,
  RequiredPublicRuntimeNonClaims,
  RequiredPublicRuntimeSurfaces,
} from './production-release-public-runtime-handoff-values';

export * from './production-release-public-runtime-handoff-values';

type PublicRuntimeHandoffProofCandidate = {
  readonly handoffRows: ReadonlyArray<{ readonly surface: string }>;
  readonly adapterRows: ReadonlyArray<{ readonly adapter: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicWebsiteRuntimeClaim: string;
  readonly accountBackendRuntimeClaim: string;
  readonly billingProviderRuntimeClaim: string;
  readonly supportBackendUploadClaim: string;
  readonly productionPublishingState: string;
  readonly updaterExecutionState: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionReleasePublicRuntimeHandoffRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicRuntimeHandoffSchemaVersionSchema,
    surface: ProductionReleasePublicRuntimeSurfaceSchema,
    handoffTarget: ProductionReleasePublicRuntimeTargetSchema,
    routeState: ProductionReleasePublicRuntimeStateSchema,
    runtimeAdapterState: ProductionReleasePublicRuntimeStateSchema,
    backendAdapterState: ProductionReleasePublicRuntimeStateSchema,
    parentVisibleState: ProductionReleasePublicRuntimeStateSchema,
    sourceProof: ProductionReleasePublicRuntimeSourceProofSchema,
    supportSafeDataClasses: Schema.Array(ProductionReleasePublicRuntimeDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionReleasePublicRuntimeDataClassSchema),
    handoffReference: PublicRuntimeHandoffReferenceSchema,
    evidenceReference: PublicRuntimeHandoffReferenceSchema,
    manualRequirement: PublicRuntimeHandoffRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.routeState !== 'implemented' ||
        'Expected public website/download/account/status handoff rows to avoid live route implementation claims'
    ),
    Schema.filter(
      (row) =>
        row.runtimeAdapterState !== 'implemented' ||
        'Expected public runtime handoff rows to avoid runtime adapter implementation claims'
    ),
    Schema.filter(
      (row) =>
        row.backendAdapterState !== 'implemented' ||
        'Expected public runtime handoff rows to avoid backend adapter implementation claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenPublicRuntimeDataClasses.includes(dataClass as never)
        ) || 'Expected public runtime handoff rows to exclude child activity and sensitive support data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenPublicRuntimeDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected public runtime handoff rows to enumerate forbidden custody and sensitive data classes'
    )
  )
);

export const ProductionReleasePublicRuntimeAdapterRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicRuntimeHandoffSchemaVersionSchema,
    adapter: ProductionReleasePublicRuntimeAdapterSchema,
    adapterState: ProductionReleasePublicRuntimeStateSchema,
    executionClaim: ProductionReleasePublicRuntimeExecutionClaimSchema,
    providerSecretCustody: Schema.Literal('not-present'),
    childActivityCustody: Schema.Literal('not-included'),
    evidenceReference: PublicRuntimeHandoffReferenceSchema,
    requiredProof: PublicRuntimeHandoffRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        row.adapterState !== 'implemented' ||
        'Expected public runtime adapter rows to avoid implementation claims before backend/runtime proof exists'
    ),
    Schema.filter(
      (row) =>
        row.executionClaim !== 'executed' || 'Expected public runtime adapter rows to avoid production execution claims'
    )
  )
);

export const ProductionReleasePublicRuntimeHandoffProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionReleasePublicRuntimeHandoffSchemaVersionSchema,
    handoffRows: Schema.Array(ProductionReleasePublicRuntimeHandoffRowSchema),
    adapterRows: Schema.Array(ProductionReleasePublicRuntimeAdapterRowSchema),
    nonClaims: Schema.Array(ProductionReleasePublicRuntimeNonClaimSchema),
    publicWebsiteRuntimeClaim: ProductionReleasePublicRuntimeStateSchema,
    accountBackendRuntimeClaim: ProductionReleasePublicRuntimeStateSchema,
    billingProviderRuntimeClaim: ProductionReleasePublicRuntimeStateSchema,
    supportBackendUploadClaim: ProductionReleasePublicRuntimeStateSchema,
    productionPublishingState: ProductionReleasePublicRuntimeStateSchema,
    signingStoreProofState: ProductionReleasePublicRuntimeStateSchema,
    updaterExecutionState: ProductionReleasePublicRuntimeStateSchema,
    childActivityCustodyClaim: ProductionReleasePublicRuntimeStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionReleasePublicRuntimeHandoffProofIsHonest(proof) ||
        'Expected public runtime handoff proof to cover required surfaces/adapters and retain explicit non-claims'
    )
  )
);

export type ProductionReleasePublicRuntimeHandoffRow = Infer<typeof ProductionReleasePublicRuntimeHandoffRowSchema>;
export type ProductionReleasePublicRuntimeAdapterRow = Infer<typeof ProductionReleasePublicRuntimeAdapterRowSchema>;
export type ProductionReleasePublicRuntimeHandoffProof = Infer<typeof ProductionReleasePublicRuntimeHandoffProofSchema>;
export type ProductionReleasePublicRuntimeSurface = Infer<typeof ProductionReleasePublicRuntimeSurfaceSchema>;
export type ProductionReleasePublicRuntimeAdapter = Infer<typeof ProductionReleasePublicRuntimeAdapterSchema>;

export const decodeProductionReleasePublicRuntimeHandoffProof = Schema.decodeUnknownSync(
  ProductionReleasePublicRuntimeHandoffProofSchema
);

export function summarizeProductionReleasePublicRuntimeHandoffs(
  rows: ReadonlyArray<ProductionReleasePublicRuntimeHandoffRow>
): Record<ProductionReleasePublicRuntimeSurface, number> {
  return RequiredPublicRuntimeSurfaces.reduce(
    (summary, surfaceName) => ({
      ...summary,
      [surfaceName]: rows.filter((row) => row.surface === surfaceName).length,
    }),
    {} as Record<ProductionReleasePublicRuntimeSurface, number>
  );
}

export function summarizeProductionReleasePublicRuntimeAdapters(
  rows: ReadonlyArray<ProductionReleasePublicRuntimeAdapterRow>
): Record<ProductionReleasePublicRuntimeAdapter, number> {
  return RequiredPublicRuntimeAdapters.reduce(
    (summary, adapterName) => ({
      ...summary,
      [adapterName]: rows.filter((row) => row.adapter === adapterName).length,
    }),
    {} as Record<ProductionReleasePublicRuntimeAdapter, number>
  );
}

function productionReleasePublicRuntimeHandoffProofIsHonest(proof: PublicRuntimeHandoffProofCandidate): boolean {
  return (
    RequiredPublicRuntimeSurfaces.every((surfaceName) =>
      proof.handoffRows.some((row) => row.surface === surfaceName)
    ) &&
    RequiredPublicRuntimeAdapters.every((adapterName) =>
      proof.adapterRows.some((row) => row.adapter === adapterName)
    ) &&
    RequiredPublicRuntimeNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicWebsiteRuntimeClaim === 'not-implemented' &&
    proof.accountBackendRuntimeClaim === 'backend-required' &&
    proof.billingProviderRuntimeClaim === 'not-implemented' &&
    proof.supportBackendUploadClaim === 'manual-required' &&
    proof.productionPublishingState === 'production-promotion-required' &&
    proof.updaterExecutionState === 'manual-required' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
