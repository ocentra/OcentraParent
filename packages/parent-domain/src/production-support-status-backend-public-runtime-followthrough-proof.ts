import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  ForbiddenStatusBackendPublicRuntimeFollowthroughDataClasses,
  ProductionSupportStatusBackendPublicRuntimeFollowthroughDataClassSchema,
  ProductionSupportStatusBackendPublicRuntimeFollowthroughNonClaimSchema,
  ProductionSupportStatusBackendPublicRuntimeFollowthroughSchemaVersionSchema,
  ProductionSupportStatusBackendPublicRuntimeFollowthroughSourceProofSchema,
  ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
  ProductionSupportStatusBackendPublicRuntimeFollowthroughTargetSchema,
  RequiredStatusBackendPublicRuntimeFollowthroughNonClaims,
  RequiredStatusBackendPublicRuntimeFollowthroughStates,
  RequiredStatusBackendPublicRuntimeFollowthroughTargets,
  StatusBackendPublicRuntimeFollowthroughManualRequirementSchema,
  StatusBackendPublicRuntimeFollowthroughReferenceSchema,
} from './production-support-status-backend-public-runtime-followthrough-values';

export * from './production-support-status-backend-public-runtime-followthrough-values';

type StatusBackendPublicRuntimeFollowthroughProofCandidate = {
  readonly rows: ReadonlyArray<{ readonly target: string; readonly followthroughState: string }>;
  readonly nonClaims: ReadonlyArray<string>;
  readonly publicRuntimeExecutionClaim: string;
  readonly statusBackendExecutionClaim: string;
  readonly supportBackendUploadExecutionClaim: string;
  readonly accountLookupExecutionClaim: string;
  readonly billingProviderContactClaim: string;
  readonly productionSlaClaim: string;
  readonly legalDisclosureExecutionClaim: string;
  readonly childActivityCustodyClaim: string;
};

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendPublicRuntimeFollowthroughSchemaVersionSchema,
    target: ProductionSupportStatusBackendPublicRuntimeFollowthroughTargetSchema,
    sourceProof: ProductionSupportStatusBackendPublicRuntimeFollowthroughSourceProofSchema,
    followthroughState: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    sourceContractState: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    statusContractState: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    publicRuntimeFollowthroughState: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    statusBackendFollowthroughState: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    supportBackendUploadState: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    supportSafeDataClasses: Schema.Array(ProductionSupportStatusBackendPublicRuntimeFollowthroughDataClassSchema),
    forbiddenDataClasses: Schema.Array(ProductionSupportStatusBackendPublicRuntimeFollowthroughDataClassSchema),
    publicRuntimeReference: StatusBackendPublicRuntimeFollowthroughReferenceSchema,
    statusBackendReference: StatusBackendPublicRuntimeFollowthroughReferenceSchema,
    manualRequirement: StatusBackendPublicRuntimeFollowthroughManualRequirementSchema,
  }).pipe(
    Schema.filter(
      (row) =>
        RequiredStatusBackendPublicRuntimeFollowthroughStates.includes(row.followthroughState as never) ||
        'Expected status backend/public runtime follow-through rows to use required states'
    ),
    Schema.filter(
      (row) =>
        (row.sourceContractState === 'source-contract-ready' && row.statusContractState === 'status-contract-ready') ||
        'Expected status backend/public runtime follow-through rows to stay source and status contract backed'
    ),
    Schema.filter(
      (row) =>
        (row.publicRuntimeFollowthroughState !== 'implemented' && row.publicRuntimeFollowthroughState !== 'executed') ||
        'Expected status backend/public runtime follow-through rows to avoid public runtime execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.statusBackendFollowthroughState !== 'implemented' && row.statusBackendFollowthroughState !== 'executed') ||
        'Expected status backend/public runtime follow-through rows to avoid status backend execution claims'
    ),
    Schema.filter(
      (row) =>
        (row.supportBackendUploadState !== 'implemented' && row.supportBackendUploadState !== 'executed') ||
        'Expected status backend/public runtime follow-through rows to avoid support backend upload execution claims'
    ),
    Schema.filter(
      (row) =>
        row.supportSafeDataClasses.every(
          (dataClass) => !ForbiddenStatusBackendPublicRuntimeFollowthroughDataClasses.includes(dataClass as never)
        ) || 'Expected status backend/public runtime follow-through rows to exclude forbidden data classes'
    ),
    Schema.filter(
      (row) =>
        ForbiddenStatusBackendPublicRuntimeFollowthroughDataClasses.every((dataClass) =>
          row.forbiddenDataClasses.includes(dataClass)
        ) || 'Expected status backend/public runtime follow-through rows to enumerate every forbidden data class'
    )
  )
);

export const ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ProductionSupportStatusBackendPublicRuntimeFollowthroughSchemaVersionSchema,
    rows: Schema.Array(ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema),
    nonClaims: Schema.Array(ProductionSupportStatusBackendPublicRuntimeFollowthroughNonClaimSchema),
    publicRuntimeExecutionClaim: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    statusBackendExecutionClaim: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    supportBackendUploadExecutionClaim: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    accountLookupExecutionClaim: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    billingProviderContactClaim: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    productionSlaClaim: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    legalDisclosureExecutionClaim: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    childActivityCustodyClaim: ProductionSupportStatusBackendPublicRuntimeFollowthroughStateSchema,
    updatedAt: ParentTimestampSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        productionSupportStatusBackendPublicRuntimeFollowthroughProofIsHonest(proof) ||
        'Expected status backend/public runtime follow-through proof to cover rows and preserve non-claims'
    )
  )
);

export type ProductionSupportStatusBackendPublicRuntimeFollowthroughRow = Infer<
  typeof ProductionSupportStatusBackendPublicRuntimeFollowthroughRowSchema
>;
export type ProductionSupportStatusBackendPublicRuntimeFollowthroughProof = Infer<
  typeof ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema
>;
export type ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget = Infer<
  typeof ProductionSupportStatusBackendPublicRuntimeFollowthroughTargetSchema
>;
export type ProductionSupportStatusBackendPublicRuntimeFollowthroughState =
  (typeof RequiredStatusBackendPublicRuntimeFollowthroughStates)[number];

export const decodeProductionSupportStatusBackendPublicRuntimeFollowthroughProof = Schema.decodeUnknownSync(
  ProductionSupportStatusBackendPublicRuntimeFollowthroughProofSchema
);

export function summarizeProductionSupportStatusBackendPublicRuntimeFollowthroughRows(
  rows: ReadonlyArray<ProductionSupportStatusBackendPublicRuntimeFollowthroughRow>
): Record<
  ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget,
  Record<ProductionSupportStatusBackendPublicRuntimeFollowthroughState, number>
> {
  return RequiredStatusBackendPublicRuntimeFollowthroughTargets.reduce(
    (summary, target) => ({
      ...summary,
      [target]: RequiredStatusBackendPublicRuntimeFollowthroughStates.reduce(
        (targetSummary, followthroughState) => ({
          ...targetSummary,
          [followthroughState]: rows.filter(
            (row) => row.target === target && row.followthroughState === followthroughState
          ).length,
        }),
        {} as Record<ProductionSupportStatusBackendPublicRuntimeFollowthroughState, number>
      ),
    }),
    {} as Record<
      ProductionSupportStatusBackendPublicRuntimeFollowthroughTarget,
      Record<ProductionSupportStatusBackendPublicRuntimeFollowthroughState, number>
    >
  );
}

function productionSupportStatusBackendPublicRuntimeFollowthroughProofIsHonest(
  proof: StatusBackendPublicRuntimeFollowthroughProofCandidate
): boolean {
  return (
    RequiredStatusBackendPublicRuntimeFollowthroughTargets.every((target) =>
      RequiredStatusBackendPublicRuntimeFollowthroughStates.every((state) =>
        proof.rows.some((row) => row.target === target && row.followthroughState === state)
      )
    ) &&
    RequiredStatusBackendPublicRuntimeFollowthroughNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    proof.publicRuntimeExecutionClaim === 'not-implemented' &&
    proof.statusBackendExecutionClaim === 'manual-required' &&
    proof.supportBackendUploadExecutionClaim === 'manual-required' &&
    proof.accountLookupExecutionClaim === 'manual-required' &&
    proof.billingProviderContactClaim === 'manual-required' &&
    proof.productionSlaClaim === 'not-implemented' &&
    proof.legalDisclosureExecutionClaim === 'manual-required' &&
    proof.childActivityCustodyClaim === 'not-implemented'
  );
}
