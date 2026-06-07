import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  RequiredTrackingFullProductUiRuntimeArtifactRefs,
  TrackingFullProductUiReadinessBlockerReferenceSchema,
} from './tracking-full-product-ui-readiness-blocker-proof';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingFullProductUiRuntimeArtifactGateTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingFullProductUiRuntimeArtifactGateStatusSchema = Schema.Literal(
  'manual-required',
  'artifact-set-present'
);

export const TrackingFullProductUiRuntimeArtifactGatePathSchema =
  TrackingFullProductUiRuntimeArtifactGateTextSchema.pipe(Schema.brand('TrackingFullProductUiRuntimeArtifactGatePath'));

export const TrackingFullProductUiRuntimeArtifactGateRowIdSchema =
  TrackingFullProductUiRuntimeArtifactGateTextSchema.pipe(
    Schema.brand('TrackingFullProductUiRuntimeArtifactGateRowId')
  );

export const TrackingFullProductUiRuntimeArtifactGateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingFullProductUiRuntimeArtifactGateRowIdSchema,
    generatedAt: ParentTimestampSchema,
    proofRoot: TrackingFullProductUiRuntimeArtifactGatePathSchema,
    requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
    currentProofTier: Schema.Literal('P2_HOSTED_CI'),
    status: TrackingFullProductUiRuntimeArtifactGateStatusSchema,
    requiredArtifacts: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
    presentArtifacts: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
    missingArtifacts: Schema.Array(TrackingFullProductUiReadinessBlockerReferenceSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    fullProductUiArtifactSetComplete: Schema.Boolean,
    parentOverviewRuntimeUiClaimed: Schema.Literal(false),
    parentDeviceDetailRuntimeUiClaimed: Schema.Literal(false),
    parentNotificationHistoryPreferencesRuntimeClaimed: Schema.Literal(false),
    retentionSettingsProductionRuntimeUiClaimed: Schema.Literal(false),
    renderedChildDeviceRuntimeUiClaimed: Schema.Literal(false),
    childDeviceSafeHelpRuntimeUiClaimed: Schema.Literal(false),
    crossSurfaceAccessibilityRuntimeClaimed: Schema.Literal(false),
    productUiEndToEndRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    productionProductUiClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Full product UI rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Full product UI rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.fullProductUiArtifactSetComplete ||
          'Full product UI artifact set status must match completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.fullProductUiArtifactSetComplete
            ? row.missingArtifacts.length === 0
            : row.missingArtifacts.length > 0) ||
          'Full product UI artifact completeness must match missing artifact count'
      )
    )
);

export const TrackingFullProductUiRuntimeArtifactGateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-full-product-ui-runtime-artifact-gate-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingFullProductUiRuntimeArtifactGateRowSchema),
    proofClaims: Schema.Struct({
      fullProductUiArtifactGateChecked: Schema.Literal(true),
      noParentOverviewRuntimeUiClaim: Schema.Literal(true),
      noParentDeviceDetailRuntimeUiClaim: Schema.Literal(true),
      noNotificationHistoryPreferencesRuntimeClaim: Schema.Literal(true),
      noRetentionSettingsProductionRuntimeUiClaim: Schema.Literal(true),
      noRenderedChildDeviceRuntimeUiClaim: Schema.Literal(true),
      noChildDeviceSafeHelpRuntimeUiClaim: Schema.Literal(true),
      noCrossSurfaceAccessibilityRuntimeClaim: Schema.Literal(true),
      noProductUiEndToEndRuntimeClaim: Schema.Literal(true),
      noChildDeviceDeliveryRuntimeClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryRuntimeClaim: Schema.Literal(true),
      noProductionProductUiClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      parentOverviewRuntimeUiClaimed: Schema.Literal(false),
      parentDeviceDetailRuntimeUiClaimed: Schema.Literal(false),
      parentNotificationHistoryPreferencesRuntimeClaimed: Schema.Literal(false),
      retentionSettingsProductionRuntimeUiClaimed: Schema.Literal(false),
      renderedChildDeviceRuntimeUiClaimed: Schema.Literal(false),
      childDeviceSafeHelpRuntimeUiClaimed: Schema.Literal(false),
      crossSurfaceAccessibilityRuntimeClaimed: Schema.Literal(false),
      productUiEndToEndRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryRuntimeClaimed: Schema.Literal(false),
      productionProductUiClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === 1 &&
          proof.rows.some((row) => row.proofRoot === RequiredTrackingFullProductUiRuntimeArtifactPlan.proofRoot)) ||
        'Full product UI runtime artifact gate must cover the product parent/child UI proof root'
    )
  )
);

export type TrackingFullProductUiRuntimeArtifactGateProof = Infer<
  typeof TrackingFullProductUiRuntimeArtifactGateProofSchema
>;
export type TrackingFullProductUiRuntimeArtifactGateRow = Infer<
  typeof TrackingFullProductUiRuntimeArtifactGateRowSchema
>;

export interface TrackingFullProductUiRuntimeArtifactInventory {
  readonly presentArtifacts: readonly string[];
}

export const RequiredTrackingFullProductUiRuntimeArtifactPlan = {
  proofRoot: 'output/tracking-plan-proof',
  requiredArtifacts: RequiredTrackingFullProductUiRuntimeArtifactRefs,
} as const;

export function buildTrackingFullProductUiRuntimeArtifactGateProof(
  generatedAt: string,
  inventory: TrackingFullProductUiRuntimeArtifactInventory
): TrackingFullProductUiRuntimeArtifactGateProof {
  return TrackingFullProductUiRuntimeArtifactGateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-full-product-ui-runtime-artifact-gate-proof',
    generatedAt,
    rows: [fullProductUiArtifactRow(generatedAt, inventory)],
    proofClaims: {
      fullProductUiArtifactGateChecked: true,
      noParentOverviewRuntimeUiClaim: true,
      noParentDeviceDetailRuntimeUiClaim: true,
      noNotificationHistoryPreferencesRuntimeClaim: true,
      noRetentionSettingsProductionRuntimeUiClaim: true,
      noRenderedChildDeviceRuntimeUiClaim: true,
      noChildDeviceSafeHelpRuntimeUiClaim: true,
      noCrossSurfaceAccessibilityRuntimeClaim: true,
      noProductUiEndToEndRuntimeClaim: true,
      noChildDeviceDeliveryRuntimeClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryRuntimeClaim: true,
      noProductionProductUiClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      parentOverviewRuntimeUiClaimed: false,
      parentDeviceDetailRuntimeUiClaimed: false,
      parentNotificationHistoryPreferencesRuntimeClaimed: false,
      retentionSettingsProductionRuntimeUiClaimed: false,
      renderedChildDeviceRuntimeUiClaimed: false,
      childDeviceSafeHelpRuntimeUiClaimed: false,
      crossSurfaceAccessibilityRuntimeClaimed: false,
      productUiEndToEndRuntimeClaimed: false,
      childDeviceDeliveryRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionProductUiClaimed: false,
      productClaimReady: false,
    },
  });
}

function fullProductUiArtifactRow(
  generatedAt: string,
  inventory: TrackingFullProductUiRuntimeArtifactInventory
): TrackingFullProductUiRuntimeArtifactGateRow {
  const presentArtifactSet = new Set(inventory.presentArtifacts);
  const requiredArtifacts = RequiredTrackingFullProductUiRuntimeArtifactPlan.requiredArtifacts;
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const fullProductUiArtifactSetComplete = missingArtifacts.length === 0;

  return TrackingFullProductUiRuntimeArtifactGateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-full-product-ui-runtime-artifacts',
    generatedAt,
    proofRoot: RequiredTrackingFullProductUiRuntimeArtifactPlan.proofRoot,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P2_HOSTED_CI',
    status: fullProductUiArtifactSetComplete ? 'artifact-set-present' : 'manual-required',
    requiredArtifacts: [...requiredArtifacts],
    presentArtifacts,
    missingArtifacts,
    auditRefs: ['tracking-full-product-ui-runtime-artifacts-audit'],
    fullProductUiArtifactSetComplete,
    parentOverviewRuntimeUiClaimed: false,
    parentDeviceDetailRuntimeUiClaimed: false,
    parentNotificationHistoryPreferencesRuntimeClaimed: false,
    retentionSettingsProductionRuntimeUiClaimed: false,
    renderedChildDeviceRuntimeUiClaimed: false,
    childDeviceSafeHelpRuntimeUiClaimed: false,
    crossSurfaceAccessibilityRuntimeClaimed: false,
    productUiEndToEndRuntimeClaimed: false,
    childDeviceDeliveryRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    productionProductUiClaimed: false,
    productClaimReady: false,
  });
}
