import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  TrackingAuthorityEnrollmentManualRequiredProofSchema,
  type TrackingAuthorityEnrollmentManualRequiredProof,
} from './tracking-authority-enrollment-manual-required-proof';

export const TrackingAuthorityRuntimeReadinessBlockerIdSchema = withParser(
  Schema.Literal(
    'android-device-owner-enrollment',
    'android-managed-profile-enrollment',
    'ios-family-controls-entitlement',
    'ios-app-review-approval',
    'desktop-managed-policy-enrollment',
    'hard-control-runtime',
    'authority-visible-parent-status',
    'physical-device-authority-proof',
    'production-authority-worker',
    'product-ready-authority'
  )
);

export const TrackingAuthorityRuntimeReadinessBlockerReferenceSchema =
  brandedNonEmptyStringSchema('TrackingAuthorityRuntimeReadinessBlockerReference');
export const TrackingAuthorityRuntimeReadinessBlockerProofIdSchema = brandedNonEmptyStringSchema('TrackingAuthorityRuntimeReadinessBlockerProofId');

export const TrackingAuthorityRuntimeReadinessBlockerStatusSchema = withParser(
  Schema.Literal('authority-required', 'manual-required')
);

const TrackingAuthorityRuntimeReadinessBlockerRowBaseSchema = Schema.Struct({
  blockerId: TrackingAuthorityRuntimeReadinessBlockerIdSchema,
  status: TrackingAuthorityRuntimeReadinessBlockerStatusSchema,
  sourceAuthorityRows: Schema.Array(TrackingAuthorityRuntimeReadinessBlockerReferenceSchema),
  blockingEvidenceRefs: Schema.Array(TrackingAuthorityRuntimeReadinessBlockerReferenceSchema),
  requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
  currentProofTier: Schema.Literal('P0_CONTRACT'),
  authorityEnrollmentClaimed: Schema.Literal(false),
  hardControlRuntimeClaimed: Schema.Literal(false),
  physicalDeviceClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingAuthorityRuntimeReadinessBlockerRowSchema = withParser(
  TrackingAuthorityRuntimeReadinessBlockerRowBaseSchema.pipe(
    Schema.filter((row) => row.sourceAuthorityRows.length > 0 && row.blockingEvidenceRefs.length > 0)
  )
);

const TrackingAuthorityRuntimeReadinessBlockerProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingAuthorityRuntimeReadinessBlockerProofIdSchema,
  generatedAt: ParentTimestampSchema,
  proofMode: Schema.Literal('tracking-authority-runtime-readiness-blocker-proof'),
  sourceProofRefs: Schema.Array(TrackingAuthorityRuntimeReadinessBlockerReferenceSchema),
  authorityEnrollmentRows: Schema.Number.pipe(Schema.int(), Schema.positive()),
  authorityRequiredRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  missingAuthorityRuntimeEvidenceCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
  blockers: Schema.Array(TrackingAuthorityRuntimeReadinessBlockerRowSchema),
  productClaims: Schema.Struct({
    authorityEnrollmentClaimed: Schema.Literal(false),
    hardControlRuntimeClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  }),
});

export const TrackingAuthorityRuntimeReadinessBlockerProofSchema = withParser(
  TrackingAuthorityRuntimeReadinessBlockerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        trackingAuthorityRuntimeReadinessProofIsHonest(proof) ||
        'Expected authority runtime blocker proof to consume authority enrollment rows and keep all product claims false'
    )
  )
);

export type TrackingAuthorityRuntimeReadinessBlockerId = Infer<typeof TrackingAuthorityRuntimeReadinessBlockerIdSchema>;
export type TrackingAuthorityRuntimeReadinessBlockerProof = Infer<
  typeof TrackingAuthorityRuntimeReadinessBlockerProofSchema
>;
export type TrackingAuthorityRuntimeReadinessBlockerRow = Infer<
  typeof TrackingAuthorityRuntimeReadinessBlockerRowSchema
>;

export type TrackingAuthorityRuntimeReadinessBlockerProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceProofRefs: readonly string[];
};

type TrackingAuthorityRuntimeReadinessBlockerProofInput = Infer<
  typeof TrackingAuthorityRuntimeReadinessBlockerProofBaseSchema
>;

export const RequiredTrackingAuthorityRuntimeReadinessBlockers = [
  'android-device-owner-enrollment',
  'android-managed-profile-enrollment',
  'ios-family-controls-entitlement',
  'ios-app-review-approval',
  'desktop-managed-policy-enrollment',
  'hard-control-runtime',
  'authority-visible-parent-status',
  'physical-device-authority-proof',
  'production-authority-worker',
  'product-ready-authority',
] as const;

export function buildTrackingAuthorityRuntimeReadinessBlockerProof(
  options: TrackingAuthorityRuntimeReadinessBlockerProofOptions,
  authorityProof: TrackingAuthorityEnrollmentManualRequiredProof
): TrackingAuthorityRuntimeReadinessBlockerProof {
  const parsedAuthorityProof = TrackingAuthorityEnrollmentManualRequiredProofSchema.parse(authorityProof);
  const authorityRowRefs = parsedAuthorityProof.rows.map((row) => row.rowId);
  const missingEvidenceRefs = uniqueRefs(parsedAuthorityProof.rows.flatMap((row) => row.requiredEvidenceRefs));

  return TrackingAuthorityRuntimeReadinessBlockerProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    proofMode: 'tracking-authority-runtime-readiness-blocker-proof',
    sourceProofRefs: uniqueRefs(options.sourceProofRefs),
    authorityEnrollmentRows: parsedAuthorityProof.rows.length,
    authorityRequiredRows: parsedAuthorityProof.rows.filter((row) => row.state === 'authority-required').length,
    manualRequiredRows: parsedAuthorityProof.rows.filter((row) => row.state === 'manual-required').length,
    missingAuthorityRuntimeEvidenceCount: missingEvidenceRefs.length,
    blockers: RequiredTrackingAuthorityRuntimeReadinessBlockers.map((blockerId) =>
      buildBlockerRow(blockerId, authorityRowRefs, missingEvidenceRefs)
    ),
    productClaims: {
      authorityEnrollmentClaimed: false,
      hardControlRuntimeClaimed: false,
      physicalDeviceClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function buildBlockerRow(
  blockerId: TrackingAuthorityRuntimeReadinessBlockerId,
  sourceAuthorityRows: readonly string[],
  blockingEvidenceRefs: readonly string[]
): TrackingAuthorityRuntimeReadinessBlockerRow {
  return TrackingAuthorityRuntimeReadinessBlockerRowSchema.parse({
    blockerId,
    status: blockerId === 'desktop-managed-policy-enrollment' ? 'manual-required' : 'authority-required',
    sourceAuthorityRows,
    blockingEvidenceRefs,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P0_CONTRACT',
    authorityEnrollmentClaimed: false,
    hardControlRuntimeClaimed: false,
    physicalDeviceClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function trackingAuthorityRuntimeReadinessProofIsHonest(
  proof: TrackingAuthorityRuntimeReadinessBlockerProofInput
): boolean {
  return (
    proof.sourceProofRefs.length > 0 &&
    proof.authorityEnrollmentRows === 5 &&
    proof.authorityRequiredRows === 4 &&
    proof.manualRequiredRows === 1 &&
    proof.missingAuthorityRuntimeEvidenceCount > 0 &&
    proof.blockers.length === RequiredTrackingAuthorityRuntimeReadinessBlockers.length &&
    proof.blockers.every((row) => row.sourceAuthorityRows.length === 5) &&
    Object.values(proof.productClaims).every((claim) => claim === false)
  );
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}

