import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

export const TrackingAuthorityEnrollmentRowIdSchema = brandedNonEmptyStringSchema('TrackingAuthorityEnrollmentRowId');

export const TrackingAuthorityEnrollmentProofRefSchema = brandedNonEmptyStringSchema('TrackingAuthorityEnrollmentProofRef');

export const TrackingAuthorityEnrollmentPlatformSchema = Schema.Literal('android', 'ios', 'desktop');

export const TrackingAuthorityEnrollmentModeSchema = Schema.Literal(
  'android-device-owner',
  'android-managed-profile',
  'ios-family-controls-entitlement',
  'ios-app-review-approval',
  'desktop-managed-policy'
);

export const TrackingAuthorityEnrollmentStateSchema = Schema.Literal(
  'authority-required',
  'manual-required',
  'not-product-ready'
);

export const TrackingAuthorityEnrollmentRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingAuthorityEnrollmentRowIdSchema,
    generatedAt: ParentTimestampSchema,
    platform: TrackingAuthorityEnrollmentPlatformSchema,
    enrollmentMode: TrackingAuthorityEnrollmentModeSchema,
    state: TrackingAuthorityEnrollmentStateSchema,
    requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
    currentProofTier: Schema.Literal('P0_CONTRACT'),
    requiredEvidenceRefs: Schema.Array(TrackingAuthorityEnrollmentProofRefSchema),
    manualProofCommand: NonEmptyStringSchema,
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    authorityEnrollmentClaimed: Schema.Literal(false),
    hardControlRuntimeClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.requiredEvidenceRefs.length >= 3 || 'Authority rows need evidence refs'))
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Authority rows need audit refs'))
);

export const TrackingAuthorityEnrollmentManualRequiredProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-authority-enrollment-manual-required-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingAuthorityEnrollmentRowSchema),
    proofClaims: Schema.Struct({
      authorityEvidenceRequirementsEnumerated: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noPhysicalDeviceClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      authorityEnrollmentClaimed: Schema.Literal(false),
      hardControlRuntimeClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        proof.rows.length === RequiredTrackingAuthorityEnrollmentModes.length ||
        'Authority proof must cover every required enrollment mode'
    )
  )
);

export type TrackingAuthorityEnrollmentManualRequiredProof = Infer<
  typeof TrackingAuthorityEnrollmentManualRequiredProofSchema
>;
export type TrackingAuthorityEnrollmentRow = Infer<typeof TrackingAuthorityEnrollmentRowSchema>;

export const RequiredTrackingAuthorityEnrollmentModes = [
  'android-device-owner',
  'android-managed-profile',
  'ios-family-controls-entitlement',
  'ios-app-review-approval',
  'desktop-managed-policy',
] as const;

export function buildTrackingAuthorityEnrollmentManualRequiredProof(
  generatedAt: string
): TrackingAuthorityEnrollmentManualRequiredProof {
  const rows = RequiredTrackingAuthorityEnrollmentModes.map((mode) => authorityRow(generatedAt, mode));
  return TrackingAuthorityEnrollmentManualRequiredProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-authority-enrollment-manual-required-proof',
    generatedAt,
    rows,
    proofClaims: {
      authorityEvidenceRequirementsEnumerated: true,
      noAuthorityClaim: true,
      noPhysicalDeviceClaim: true,
      noProductReadyClaim: true,
    },
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

function authorityRow(
  generatedAt: string,
  mode: (typeof RequiredTrackingAuthorityEnrollmentModes)[number]
): TrackingAuthorityEnrollmentRow {
  const platform = platformForMode(mode);
  return TrackingAuthorityEnrollmentRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `tracking-authority-${mode}`,
    generatedAt,
    platform,
    enrollmentMode: mode,
    state: mode === 'desktop-managed-policy' ? 'manual-required' : 'authority-required',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P0_CONTRACT',
    requiredEvidenceRefs: evidenceRefsForMode(mode),
    manualProofCommand: manualCommandForMode(mode),
    auditRefs: [`tracking-authority-${mode}-audit`],
    authorityEnrollmentClaimed: false,
    hardControlRuntimeClaimed: false,
    physicalDeviceClaimed: false,
    providerDeliveryClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function platformForMode(mode: (typeof RequiredTrackingAuthorityEnrollmentModes)[number]) {
  if (mode.startsWith('android')) return 'android';
  if (mode.startsWith('ios')) return 'ios';
  return 'desktop';
}

function evidenceRefsForMode(mode: (typeof RequiredTrackingAuthorityEnrollmentModes)[number]) {
  return [
    `tracking-authority-${mode}-device-identity-proof`,
    `tracking-authority-${mode}-enrollment-state-proof`,
    `tracking-authority-${mode}-approved-capability-proof`,
    `tracking-authority-${mode}-parent-visible-consent-proof`,
  ];
}

function manualCommandForMode(mode: (typeof RequiredTrackingAuthorityEnrollmentModes)[number]) {
  return `collect tracking authority evidence for ${mode} with enrolled device, capability grant, screenshot/log bundle, and parent-visible status row`;
}

