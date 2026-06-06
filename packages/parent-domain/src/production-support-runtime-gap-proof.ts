import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ForbiddenRuntimeGapDataClasses,
  ProductionSupportRuntimeGapDataClassSchema,
  ProductionSupportRuntimeGapItemSchema,
  ProductionSupportRuntimeGapNonClaimSchema,
  ProductionSupportRuntimeGapSchemaVersionSchema,
  ProductionSupportRuntimeGapSourceProofSchema,
  ProductionSupportRuntimeGapStateSchema,
  RequiredRuntimeGapItems,
  RequiredRuntimeGapNonClaims,
  RuntimeGapReferenceSchema,
  RuntimeGapRequirementSchema,
} from './production-support-runtime-gap-values';

export * from './production-support-runtime-gap-values';

type RuntimeGapProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly item: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicRuntimeClaim: string;
  readonly supportPublicationExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountBackendRuntimeClaim: string;
  readonly billingProviderRuntimeClaim: string;
  readonly legalExportDeleteRuntimeClaim: string;
  readonly remoteSupportSessionClaim: string;
  readonly productionSlaClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionSupportRuntimeGapRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportRuntimeGapSchemaVersionSchema,
    item: ProductionSupportRuntimeGapItemSchema,
    sourceProof: ProductionSupportRuntimeGapSourceProofSchema,
    sourceContractState: ProductionSupportRuntimeGapStateSchema,
    runtimeExecutionState: ProductionSupportRuntimeGapStateSchema,
    backendRuntimeState: ProductionSupportRuntimeGapStateSchema,
    providerRuntimeState: ProductionSupportRuntimeGapStateSchema,
    publicationState: ProductionSupportRuntimeGapStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportRuntimeGapDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportRuntimeGapDataClassSchema),
    sourceReference: RuntimeGapReferenceSchema,
    manualRequirement: RuntimeGapRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        (row.runtimeExecutionState !== 'implemented' && row.runtimeExecutionState !== 'executed') ||
        'Expected runtime gap rows to avoid implemented/executed runtime claims'
    ),
    Schema.filter(
      (row) =>
        (row.backendRuntimeState !== 'implemented' && row.backendRuntimeState !== 'executed') ||
        'Expected runtime gap rows to keep backend runtime manual-required or not implemented'
    ),
    Schema.filter(
      (row) =>
        (row.providerRuntimeState !== 'implemented' && row.providerRuntimeState !== 'executed') ||
        'Expected runtime gap rows to keep provider runtime manual-required or not implemented'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every((dataClass) => !ForbiddenRuntimeGapDataClasses.includes(dataClass as never)) ||
        'Expected runtime gap rows to exclude child activity, raw support bundles, provider, backend payload, and private data'
    ),
    Schema.filter(
      (row) =>
        ForbiddenRuntimeGapDataClasses.every((dataClass) => row.forbiddenDataClasses.includes(dataClass)) ||
        'Expected runtime gap rows to enumerate forbidden runtime gap data classes'
    )
  )
);

export const ProductionSupportRuntimeGapProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportRuntimeGapSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportRuntimeGapRowSchema),
    nonClaims: Schema.Array(ProductionSupportRuntimeGapNonClaimSchema),
    publicRuntimeClaim: ProductionSupportRuntimeGapStateSchema,
    supportPublicationExecutionClaim: ProductionSupportRuntimeGapStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportRuntimeGapStateSchema,
    accountBackendRuntimeClaim: ProductionSupportRuntimeGapStateSchema,
    billingProviderRuntimeClaim: ProductionSupportRuntimeGapStateSchema,
    legalExportDeleteRuntimeClaim: ProductionSupportRuntimeGapStateSchema,
    remoteSupportSessionClaim: ProductionSupportRuntimeGapStateSchema,
    productionSlaClaim: ProductionSupportRuntimeGapStateSchema,
    childActivityCustodyClaim: ProductionSupportRuntimeGapStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportRuntimeGapProofIsHonest(proof) ||
        'Expected runtime gap proof to cover all required rows while preserving runtime non-claims'
    )
  )
);

export type ProductionSupportRuntimeGapRow = Infer<typeof ProductionSupportRuntimeGapRowSchema>;
export type ProductionSupportRuntimeGapProof = Infer<typeof ProductionSupportRuntimeGapProofSchema>;
export type ProductionSupportRuntimeGapItem = Infer<typeof ProductionSupportRuntimeGapItemSchema>;

export const decodeProductionSupportRuntimeGapProof = Schema.decodeUnknownSync(ProductionSupportRuntimeGapProofSchema);

export function summarizeProductionSupportRuntimeGapRows(
  rows: ReadonlyArray<ProductionSupportRuntimeGapRow>
): Record<ProductionSupportRuntimeGapItem, number> {
  return RequiredRuntimeGapItems.reduce(
    (summary, item) => ({
      ...summary,
      [item]: rows.filter((row) => row.item === item).length,
    }),
    {} as Record<ProductionSupportRuntimeGapItem, number>
  );
}

function productionSupportRuntimeGapProofIsHonest(proof: RuntimeGapProofCandidate): boolean {
  return (
    RequiredRuntimeGapItems.every((item) => proof.rows.some((row) => row.item === item)) &&
    RequiredRuntimeGapNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicRuntimeClaim === 'not-implemented' &&
    proof.supportPublicationExecutionClaim === 'manual-required' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountBackendRuntimeClaim === 'backend-required' &&
    proof.billingProviderRuntimeClaim === 'provider-required' &&
    proof.legalExportDeleteRuntimeClaim === 'manual-required' &&
    proof.remoteSupportSessionClaim === 'not-implemented' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
