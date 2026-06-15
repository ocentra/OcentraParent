import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  TrackingChildRuntimeSnapshotRequirementsReadModelSchema,
  type TrackingChildRuntimeSnapshotRequirementsRow,
} from './tracking-child-runtime-snapshot-requirements-proof';
import { TrackingChildRuntimeAndroidEmulatorBridgeProofSchema } from './tracking-child-runtime-android-emulator-readiness-bridge-proof';
import { TrackingParentChildLocalRuntimeBridgeProofSchema } from './tracking-parent-child-local-runtime-bridge-proof';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import { TrackingRetentionSettingsProofRefSchema } from './tracking-retention-settings-read-model-proof';

const TrackingChildRuntimeProductReadinessTextSchema = Schema.String.pipe(Schema.minLength(1));
const TrackingChildRuntimeProductReadinessCounterSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingChildRuntimeProductReadinessBlockerProofIdSchema =
  TrackingChildRuntimeProductReadinessTextSchema.pipe(
    Schema.brand('TrackingChildRuntimeProductReadinessBlockerProofId')
  );

export const TrackingChildRuntimeProductReadinessBlockerSchema = Schema.Literal(
  'child-device-delivery-runtime-proof-required',
  'child-device-execution-result-proof-required',
  'rendered-child-device-ui-snapshot-proof-required',
  'parent-receipt-runtime-proof-required',
  'runtime-observation-proof-required',
  'physical-device-proof-required',
  'authority-proof-required'
);

export const RequiredTrackingChildRuntimeProductReadinessBlockers = [
  'child-device-delivery-runtime-proof-required',
  'child-device-execution-result-proof-required',
  'rendered-child-device-ui-snapshot-proof-required',
  'parent-receipt-runtime-proof-required',
  'runtime-observation-proof-required',
  'physical-device-proof-required',
  'authority-proof-required',
] as const;

const TrackingChildRuntimeProductReadinessBlockerRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  blockerProofId: TrackingChildRuntimeProductReadinessBlockerProofIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceSnapshotRequirementsProofRef: TrackingRetentionSettingsProofRefSchema,
  sourceAndroidEmulatorBridgeProofRef: TrackingRetentionSettingsProofRefSchema,
  sourceParentChildLocalRuntimeBridgeProofRef: TrackingRetentionSettingsProofRefSchema,
  sourceCheckInId: TrackingChildRuntimeProductReadinessTextSchema,
  sourceSnapshotRequirementRowId: TrackingChildRuntimeProductReadinessTextSchema,
  sourceAndroidEmulatorBridgeRowId: TrackingChildRuntimeProductReadinessTextSchema,
  sourceParentChildLocalRuntimeBridgeRowId: TrackingChildRuntimeProductReadinessTextSchema,
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  deliveryEnvelopeRef: TrackingChildRuntimeProductReadinessTextSchema,
  executionResultRequirementRefCount: TrackingChildRuntimeProductReadinessCounterSchema,
  visibleSnapshotRequirementRefCount: TrackingChildRuntimeProductReadinessCounterSchema,
  parentReceiptRequirementRefCount: TrackingChildRuntimeProductReadinessCounterSchema,
  runtimeObservationRequirementRefCount: TrackingChildRuntimeProductReadinessCounterSchema,
  androidEmulatorPrerequisitesObserved: Schema.Literal(true),
  androidPackageLaunchObserved: Schema.Literal(true),
  androidForegroundServiceObserved: Schema.Literal(true),
  androidLocalGeofenceTransitionCount: TrackingChildRuntimeProductReadinessCounterSchema,
  localParentChildRuntimeObserved: Schema.Literal(true),
  typedLocalServiceTransportObserved: Schema.Literal(true),
  parentReadModelProjectionObserved: Schema.Literal(true),
  parentChildLocalRuntimeStoredEventCount: TrackingChildRuntimeProductReadinessCounterSchema,
  parentChildLocalRuntimeDeadLetterCount: Schema.Literal(0),
  parentChildLocalRuntimeChildAgentPhaseCount: TrackingChildRuntimeProductReadinessCounterSchema,
  childRuntimeRequiredArtifacts: Schema.Array(TrackingChildRuntimeProductReadinessTextSchema),
  childRuntimePresentArtifacts: Schema.Array(TrackingChildRuntimeProductReadinessTextSchema),
  childRuntimeMissingArtifacts: Schema.Array(TrackingChildRuntimeProductReadinessTextSchema),
  childRuntimeRequiredArtifactCount: TrackingChildRuntimeProductReadinessCounterSchema,
  childRuntimePresentArtifactCount: TrackingChildRuntimeProductReadinessCounterSchema,
  childRuntimeMissingArtifactCount: TrackingChildRuntimeProductReadinessCounterSchema,
  androidEmulatorChildRuntimeMissingArtifactCount: TrackingChildRuntimeProductReadinessCounterSchema,
  blockerRefs: Schema.Array(TrackingChildRuntimeProductReadinessBlockerSchema),
  deliveryEnvelopeRequirementClaimed: Schema.Literal(true),
  executionResultRequirementClaimed: Schema.Literal(true),
  visibleSnapshotRequirementClaimed: Schema.Literal(true),
  parentReceiptRequirementClaimed: Schema.Literal(true),
  runtimeObservationRequirementClaimed: Schema.Literal(true),
  childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
  childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
  renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  notificationReceiptIngestionClaimed: Schema.Literal(false),
  liveLocationRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionWorkerClaimed: Schema.Literal(false),
  productReadyClaimed: Schema.Literal(false),
});

export const TrackingChildRuntimeProductReadinessBlockerRowSchema = withParser(
  TrackingChildRuntimeProductReadinessBlockerRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        (row.auditRefs.length > 0 &&
          row.blockerRefs.length === RequiredTrackingChildRuntimeProductReadinessBlockers.length &&
          trackingChildRuntimeProductReadinessBlockerRowIsHonest(row)) ||
        'Expected child runtime product-readiness blocker rows to preserve requirement refs and avoid runtime/product claims'
    )
  )
);

export const TrackingChildRuntimeProductReadinessBlockerProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-child-runtime-product-readiness-blocker-proof'),
    generatedAt: ParentTimestampSchema,
    sourceSnapshotRequirementsProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceAndroidEmulatorBridgeProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceParentChildLocalRuntimeBridgeProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceSnapshotRequirementsStatus: TrackingChildRuntimeProductReadinessTextSchema,
    sourceAndroidEmulatorBridgeStatus: TrackingChildRuntimeProductReadinessTextSchema,
    sourceParentChildLocalRuntimeBridgeStatus: TrackingChildRuntimeProductReadinessTextSchema,
    childRuntimeRequiredArtifactCount: TrackingChildRuntimeProductReadinessCounterSchema,
    childRuntimePresentArtifactCount: TrackingChildRuntimeProductReadinessCounterSchema,
    childRuntimeMissingArtifactCount: TrackingChildRuntimeProductReadinessCounterSchema,
    parentChildLocalRuntimeStoredEventCount: TrackingChildRuntimeProductReadinessCounterSchema,
    parentChildLocalRuntimeDeadLetterCount: Schema.Literal(0),
    parentChildLocalRuntimeChildAgentPhaseCount: TrackingChildRuntimeProductReadinessCounterSchema,
    rows: Schema.Array(TrackingChildRuntimeProductReadinessBlockerRowSchema),
    proofClaims: Schema.Struct({
      snapshotRequirementRowsObserved: Schema.Literal(true),
      androidEmulatorBridgeObserved: Schema.Literal(true),
      deliveryEnvelopeRequirementsObserved: Schema.Literal(true),
      executionResultRequirementsObserved: Schema.Literal(true),
      visibleSnapshotRequirementsObserved: Schema.Literal(true),
      parentReceiptRequirementsObserved: Schema.Literal(true),
      runtimeObservationRequirementsObserved: Schema.Literal(true),
      localParentChildRuntimeObserved: Schema.Literal(true),
      typedLocalServiceTransportObserved: Schema.Literal(true),
      parentReadModelProjectionObserved: Schema.Literal(true),
      productReadinessBlocked: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      childRuntimeRequirementCoverageClaimed: Schema.Literal(true),
      androidEmulatorPrerequisitesObserved: Schema.Literal(true),
      localParentChildRuntimeObserved: Schema.Literal(true),
      childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
      childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
      renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptIngestionClaimed: Schema.Literal(false),
      liveLocationRuntimeClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productReadyClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length > 0 &&
          proof.rows.every(
            (row) =>
              row.sourceSnapshotRequirementsProofRef === proof.sourceSnapshotRequirementsProofRef &&
              row.sourceAndroidEmulatorBridgeProofRef === proof.sourceAndroidEmulatorBridgeProofRef &&
              row.sourceParentChildLocalRuntimeBridgeProofRef === proof.sourceParentChildLocalRuntimeBridgeProofRef
          )) ||
        'Expected child runtime product-readiness blocker proof rows to cite snapshot, Android emulator, and parent-child local runtime proofs'
    )
  )
);

export type TrackingChildRuntimeProductReadinessBlockerProof = Infer<
  typeof TrackingChildRuntimeProductReadinessBlockerProofSchema
>;
type TrackingChildRuntimeProductReadinessBlockerRowInput = Infer<
  typeof TrackingChildRuntimeProductReadinessBlockerRowBaseSchema
>;
type TrackingChildRuntimeAndroidEmulatorBridgeRow = Infer<
  typeof TrackingChildRuntimeAndroidEmulatorBridgeProofSchema
>['rows'][number];
type TrackingParentChildLocalRuntimeBridgeRow = Infer<
  typeof TrackingParentChildLocalRuntimeBridgeProofSchema
>['rows'][number];

export function buildTrackingChildRuntimeProductReadinessBlockerProof(
  generatedAt: string,
  sourceSnapshotRequirementsProofRef: string,
  sourceSnapshotRequirementsProof: unknown,
  sourceAndroidEmulatorBridgeProofRef: string,
  sourceAndroidEmulatorBridgeProof: unknown,
  sourceParentChildLocalRuntimeBridgeProofRef: string,
  sourceParentChildLocalRuntimeBridgeProof: unknown
): TrackingChildRuntimeProductReadinessBlockerProof {
  const snapshotRequirements = TrackingChildRuntimeSnapshotRequirementsReadModelSchema.parse(
    snapshotRequirementsReadModelFrom(sourceSnapshotRequirementsProof)
  );
  const androidEmulatorBridge = TrackingChildRuntimeAndroidEmulatorBridgeProofSchema.parse(
    androidEmulatorBridgeReadModelFrom(sourceAndroidEmulatorBridgeProof)
  );
  const parentChildLocalRuntimeBridge = TrackingParentChildLocalRuntimeBridgeProofSchema.parse(
    parentChildLocalRuntimeBridgeReadModelFrom(sourceParentChildLocalRuntimeBridgeProof)
  );
  const [androidEmulatorBridgeRow] = androidEmulatorBridge.rows;
  const [parentChildLocalRuntimeBridgeRow] = parentChildLocalRuntimeBridge.rows;

  if (androidEmulatorBridgeRow === undefined) {
    throw new Error('Child runtime Android emulator bridge proof has no rows');
  }
  if (parentChildLocalRuntimeBridgeRow === undefined) {
    throw new Error('Parent-child local runtime bridge proof has no rows');
  }

  const rows = snapshotRequirements.rows.map((row) =>
    blockerRow(
      generatedAt,
      sourceSnapshotRequirementsProofRef,
      sourceAndroidEmulatorBridgeProofRef,
      sourceParentChildLocalRuntimeBridgeProofRef,
      androidEmulatorBridgeRow,
      parentChildLocalRuntimeBridgeRow,
      row
    )
  );

  return blockerProof(
    generatedAt,
    sourceSnapshotRequirementsProofRef,
    sourceAndroidEmulatorBridgeProofRef,
    sourceParentChildLocalRuntimeBridgeProofRef,
    statusFrom(sourceSnapshotRequirementsProof),
    androidEmulatorBridgeRow,
    parentChildLocalRuntimeBridgeRow,
    rows
  );
}

function blockerProof(
  generatedAt: string,
  sourceSnapshotRequirementsProofRef: string,
  sourceAndroidEmulatorBridgeProofRef: string,
  sourceParentChildLocalRuntimeBridgeProofRef: string,
  sourceSnapshotRequirementsStatus: string,
  androidEmulatorBridgeRow: TrackingChildRuntimeAndroidEmulatorBridgeRow,
  parentChildLocalRuntimeBridgeRow: TrackingParentChildLocalRuntimeBridgeRow,
  rows: readonly Infer<typeof TrackingChildRuntimeProductReadinessBlockerRowSchema>[]
): TrackingChildRuntimeProductReadinessBlockerProof {
  return TrackingChildRuntimeProductReadinessBlockerProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-child-runtime-product-readiness-blocker-proof',
    generatedAt,
    sourceSnapshotRequirementsProofRef,
    sourceAndroidEmulatorBridgeProofRef,
    sourceParentChildLocalRuntimeBridgeProofRef,
    sourceSnapshotRequirementsStatus,
    sourceAndroidEmulatorBridgeStatus: androidEmulatorBridgeRow.status,
    sourceParentChildLocalRuntimeBridgeStatus: parentChildLocalRuntimeBridgeRow.status,
    childRuntimeRequiredArtifactCount: androidEmulatorBridgeRow.childRuntimeRequiredArtifacts.length,
    childRuntimePresentArtifactCount: androidEmulatorBridgeRow.childRuntimePresentArtifacts.length,
    childRuntimeMissingArtifactCount: androidEmulatorBridgeRow.childRuntimeMissingArtifacts.length,
    parentChildLocalRuntimeStoredEventCount: parentChildLocalRuntimeBridgeRow.storedEventCount,
    parentChildLocalRuntimeDeadLetterCount: parentChildLocalRuntimeBridgeRow.deadLetterCount,
    parentChildLocalRuntimeChildAgentPhaseCount: parentChildLocalRuntimeBridgeRow.childAgentPhaseCount,
    rows,
    proofClaims: {
      snapshotRequirementRowsObserved: true,
      androidEmulatorBridgeObserved: true,
      deliveryEnvelopeRequirementsObserved: true,
      executionResultRequirementsObserved: true,
      visibleSnapshotRequirementsObserved: true,
      parentReceiptRequirementsObserved: true,
      runtimeObservationRequirementsObserved: true,
      localParentChildRuntimeObserved: true,
      typedLocalServiceTransportObserved: true,
      parentReadModelProjectionObserved: true,
      productReadinessBlocked: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      childRuntimeRequirementCoverageClaimed: true,
      androidEmulatorPrerequisitesObserved: true,
      localParentChildRuntimeObserved: true,
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptIngestionClaimed: false,
      liveLocationRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productReadyClaimed: false,
    },
  });
}

function blockerRow(
  generatedAt: string,
  sourceSnapshotRequirementsProofRef: string,
  sourceAndroidEmulatorBridgeProofRef: string,
  sourceParentChildLocalRuntimeBridgeProofRef: string,
  androidEmulatorBridgeRow: TrackingChildRuntimeAndroidEmulatorBridgeRow,
  parentChildLocalRuntimeBridgeRow: TrackingParentChildLocalRuntimeBridgeRow,
  sourceRow: TrackingChildRuntimeSnapshotRequirementsRow
) {
  return TrackingChildRuntimeProductReadinessBlockerRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    blockerProofId: `tracking-child-runtime-product-readiness-blocked-${sourceRow.sourceCheckInId}`,
    generatedAt,
    sourceSnapshotRequirementsProofRef,
    sourceAndroidEmulatorBridgeProofRef,
    sourceParentChildLocalRuntimeBridgeProofRef,
    sourceCheckInId: sourceRow.sourceCheckInId,
    sourceSnapshotRequirementRowId: sourceRow.rowId,
    sourceAndroidEmulatorBridgeRowId: androidEmulatorBridgeRow.rowId,
    sourceParentChildLocalRuntimeBridgeRowId: parentChildLocalRuntimeBridgeRow.rowId,
    auditRefs: [`tracking-child-runtime-product-readiness-blocker-audit-${sourceRow.sourceCheckInId}`],
    deliveryEnvelopeRef: sourceRow.deliveryEnvelopeRef,
    executionResultRequirementRefCount: sourceRow.executionResultRequirementRefs.length,
    visibleSnapshotRequirementRefCount: sourceRow.visibleSnapshotRequirementRefs.length,
    parentReceiptRequirementRefCount: sourceRow.parentReceiptRequirementRefs.length,
    runtimeObservationRequirementRefCount: sourceRow.runtimeObservationRequirementRefs.length,
    androidEmulatorPrerequisitesObserved: true,
    androidPackageLaunchObserved: androidEmulatorBridgeRow.packageLaunchObserved,
    androidForegroundServiceObserved: androidEmulatorBridgeRow.foregroundServiceObserved,
    androidLocalGeofenceTransitionCount: androidEmulatorBridgeRow.localGeofenceTransitionCount,
    localParentChildRuntimeObserved: parentChildLocalRuntimeBridgeRow.localParentChildRuntimeObserved,
    typedLocalServiceTransportObserved: parentChildLocalRuntimeBridgeRow.typedLocalServiceTransportObserved,
    parentReadModelProjectionObserved: parentChildLocalRuntimeBridgeRow.parentReadModelProjectionObserved,
    parentChildLocalRuntimeStoredEventCount: parentChildLocalRuntimeBridgeRow.storedEventCount,
    parentChildLocalRuntimeDeadLetterCount: parentChildLocalRuntimeBridgeRow.deadLetterCount,
    parentChildLocalRuntimeChildAgentPhaseCount: parentChildLocalRuntimeBridgeRow.childAgentPhaseCount,
    childRuntimeRequiredArtifacts: [...androidEmulatorBridgeRow.childRuntimeRequiredArtifacts],
    childRuntimePresentArtifacts: [...androidEmulatorBridgeRow.childRuntimePresentArtifacts],
    childRuntimeMissingArtifacts: [...androidEmulatorBridgeRow.childRuntimeMissingArtifacts],
    childRuntimeRequiredArtifactCount: androidEmulatorBridgeRow.childRuntimeRequiredArtifacts.length,
    childRuntimePresentArtifactCount: androidEmulatorBridgeRow.childRuntimePresentArtifacts.length,
    childRuntimeMissingArtifactCount: androidEmulatorBridgeRow.childRuntimeMissingArtifacts.length,
    androidEmulatorChildRuntimeMissingArtifactCount: androidEmulatorBridgeRow.childRuntimeMissingArtifacts.length,
    blockerRefs: [...RequiredTrackingChildRuntimeProductReadinessBlockers],
    deliveryEnvelopeRequirementClaimed: true,
    executionResultRequirementClaimed: true,
    visibleSnapshotRequirementClaimed: true,
    parentReceiptRequirementClaimed: true,
    runtimeObservationRequirementClaimed: true,
    childDeviceDeliveryRuntimeClaimed: false,
    childDeviceExecutionRuntimeClaimed: false,
    renderedChildDeviceUiRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptIngestionClaimed: false,
    liveLocationRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    productReadyClaimed: false,
  });
}

function snapshotRequirementsReadModelFrom(sourceSnapshotRequirementsProof: unknown): unknown {
  const candidate = sourceSnapshotRequirementsProof as { readonly readModel?: unknown };
  if (candidate.readModel === undefined) {
    throw new Error('Child runtime snapshot-requirements proof is missing readModel');
  }
  return candidate.readModel;
}

function androidEmulatorBridgeReadModelFrom(sourceAndroidEmulatorBridgeProof: unknown): unknown {
  const candidate = sourceAndroidEmulatorBridgeProof as { readonly readModel?: unknown };
  return candidate.readModel ?? sourceAndroidEmulatorBridgeProof;
}

function parentChildLocalRuntimeBridgeReadModelFrom(sourceParentChildLocalRuntimeBridgeProof: unknown): unknown {
  const candidate = sourceParentChildLocalRuntimeBridgeProof as { readonly readModel?: unknown };
  return candidate.readModel ?? sourceParentChildLocalRuntimeBridgeProof;
}

function statusFrom(sourceSnapshotRequirementsProof: unknown): string {
  const candidate = sourceSnapshotRequirementsProof as { readonly status?: unknown };
  if (typeof candidate.status !== 'string' || candidate.status.length === 0) {
    throw new Error('Child runtime snapshot-requirements proof is missing status');
  }
  return candidate.status;
}

function trackingChildRuntimeProductReadinessBlockerRowIsHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    trackingChildRuntimeProductReadinessRequirementCountsAreHonest(row) &&
    trackingChildRuntimeProductReadinessAndroidPrereqsAreHonest(row) &&
    trackingChildRuntimeProductReadinessLocalRuntimeIsHonest(row) &&
    trackingChildRuntimeProductReadinessArtifactCountsAreHonest(row) &&
    trackingChildRuntimeProductReadinessRequirementClaimsAreHonest(row) &&
    trackingChildRuntimeProductReadinessBlockerRowNonClaimsAreHonest(row)
  );
}

function trackingChildRuntimeProductReadinessLocalRuntimeIsHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    row.localParentChildRuntimeObserved === true &&
    row.typedLocalServiceTransportObserved === true &&
    row.parentReadModelProjectionObserved === true &&
    row.parentChildLocalRuntimeStoredEventCount >= 9 &&
    row.parentChildLocalRuntimeDeadLetterCount === 0 &&
    row.parentChildLocalRuntimeChildAgentPhaseCount >= 4
  );
}

function trackingChildRuntimeProductReadinessRequirementCountsAreHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    row.deliveryEnvelopeRef.length > 0 &&
    row.executionResultRequirementRefCount > 0 &&
    row.visibleSnapshotRequirementRefCount > 0 &&
    row.parentReceiptRequirementRefCount > 0 &&
    row.runtimeObservationRequirementRefCount > 0
  );
}

function trackingChildRuntimeProductReadinessAndroidPrereqsAreHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    row.androidEmulatorPrerequisitesObserved === true &&
    row.androidPackageLaunchObserved === true &&
    row.androidForegroundServiceObserved === true &&
    row.androidLocalGeofenceTransitionCount > 0
  );
}

function trackingChildRuntimeProductReadinessArtifactCountsAreHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    row.childRuntimeRequiredArtifacts.length > 0 &&
    row.childRuntimeRequiredArtifacts.length === row.childRuntimeRequiredArtifactCount &&
    row.childRuntimePresentArtifacts.length === row.childRuntimePresentArtifactCount &&
    row.childRuntimeMissingArtifacts.length === row.childRuntimeMissingArtifactCount &&
    row.childRuntimeRequiredArtifactCount ===
      row.childRuntimePresentArtifactCount + row.childRuntimeMissingArtifactCount &&
    row.childRuntimeMissingArtifactCount > 0 &&
    row.androidEmulatorChildRuntimeMissingArtifactCount > 0
  );
}

function trackingChildRuntimeProductReadinessRequirementClaimsAreHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    row.deliveryEnvelopeRequirementClaimed === true &&
    row.executionResultRequirementClaimed === true &&
    row.visibleSnapshotRequirementClaimed === true &&
    row.parentReceiptRequirementClaimed === true &&
    row.runtimeObservationRequirementClaimed === true
  );
}

function trackingChildRuntimeProductReadinessBlockerRowNonClaimsAreHonest(
  row: TrackingChildRuntimeProductReadinessBlockerRowInput
): boolean {
  return (
    row.childDeviceDeliveryRuntimeClaimed === false &&
    row.childDeviceExecutionRuntimeClaimed === false &&
    row.renderedChildDeviceUiRuntimeClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.notificationReceiptIngestionClaimed === false &&
    row.liveLocationRuntimeClaimed === false &&
    row.physicalDeviceProofClaimed === false &&
    row.authorityProofClaimed === false &&
    row.productionWorkerClaimed === false &&
    row.productReadyClaimed === false
  );
}
