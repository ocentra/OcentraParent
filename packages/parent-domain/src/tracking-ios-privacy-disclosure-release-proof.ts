import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from './references';

const TrackingIosPrivacyDisclosureTextSchema = Schema.String.pipe(Schema.minLength(1));
const TrackingIosPrivacyDisclosureNonNegativeIntegerSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingIosPrivacyDisclosureProofIdSchema = TrackingIosPrivacyDisclosureTextSchema.pipe(
  Schema.brand('TrackingIosPrivacyDisclosureProofId')
);
export const TrackingIosPrivacyDisclosureProofReferenceSchema = TrackingIosPrivacyDisclosureTextSchema.pipe(
  Schema.brand('TrackingIosPrivacyDisclosureProofReference')
);

export const TrackingIosPrivacyDisclosureReleaseGateSchema = withParser(
  Schema.Literal(
    'location-purpose-disclosure',
    'background-location-disclosure',
    'region-monitoring-disclosure',
    'notification-disclosure',
    'data-custody-disclosure',
    'app-store-review-evidence'
  )
);
export const TrackingIosPrivacyDisclosureGateStateSchema = withParser(
  Schema.Literal('release-blocked-before-disclosure', 'manual-review-required')
);

export const RequiredTrackingIosPrivacyDisclosureReleaseProofNonClaims = [
  'no-apple-app-store-review-proof',
  'no-apple-privacy-nutrition-label-proof',
  'no-core-location-runtime-proof',
  'no-background-location-delivery-proof',
  'no-region-monitoring-runtime-proof',
  'no-notification-delivery-proof',
  'no-apple-entitlement-proof',
  'no-testflight-device-install-proof',
  'no-physical-device-proof',
  'no-authority-proof',
  'no-product-ready-ios-tracking',
] as const;

export const TrackingIosPrivacyDisclosureReleaseProofNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingIosPrivacyDisclosureReleaseProofNonClaims)
);

const TrackingIosPrivacyDisclosureInputRowBaseSchema = Schema.Struct({
  rowId: TrackingIosPrivacyDisclosureProofReferenceSchema,
  releaseGate: TrackingIosPrivacyDisclosureReleaseGateSchema,
  observedAt: ParentTimestampSchema,
  disclosureEvidenceRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema),
  manualProofRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema).pipe(Schema.minItems(1)),
  appStoreReviewArtifactRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema),
  privacyNutritionArtifactRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema),
  runtimeEvidenceRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema),
  requiredBeforeReleaseClaimRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema).pipe(
    Schema.minItems(1)
  ),
});

export const TrackingIosPrivacyDisclosureInputRowSchema = withParser(TrackingIosPrivacyDisclosureInputRowBaseSchema);

const TrackingIosPrivacyDisclosureProofRowBaseSchema = Schema.Struct({
  rowId: TrackingIosPrivacyDisclosureProofReferenceSchema,
  releaseGate: TrackingIosPrivacyDisclosureReleaseGateSchema,
  gateState: TrackingIosPrivacyDisclosureGateStateSchema,
  observedAt: ParentTimestampSchema,
  disclosureEvidenceRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema),
  manualProofRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema).pipe(Schema.minItems(1)),
  appStoreReviewArtifactRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema),
  privacyNutritionArtifactRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema),
  runtimeEvidenceRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema),
  requiredBeforeReleaseClaimRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema).pipe(
    Schema.minItems(1)
  ),
  parentVisibleStatusToken: TrackingIosPrivacyDisclosureProofReferenceSchema,
  releaseClaimAllowed: Schema.Literal(false),
  appStoreReviewClaimed: Schema.Literal(false),
  privacyNutritionLabelClaimed: Schema.Literal(false),
  coreLocationRuntimeClaimed: Schema.Literal(false),
  backgroundLocationDeliveryClaimed: Schema.Literal(false),
  regionMonitoringClaimed: Schema.Literal(false),
  notificationDeliveryClaimed: Schema.Literal(false),
  entitlementProofClaimed: Schema.Literal(false),
  testflightDeviceInstallClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productReadyIosTrackingClaimed: Schema.Literal(false),
});

export const TrackingIosPrivacyDisclosureProofRowSchema = withParser(
  TrackingIosPrivacyDisclosureProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingIosPrivacyDisclosureProofRowIsHonest(row) ||
        'iOS privacy disclosure release proof rows must keep release claims blocked until disclosure, review, and runtime proof artifacts exist'
    )
  )
);

const TrackingIosPrivacyDisclosureProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingIosPrivacyDisclosureProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  sourceProofRefs: Schema.Array(TrackingIosPrivacyDisclosureProofReferenceSchema).pipe(Schema.minItems(1)),
  releaseGateRows: Schema.Array(TrackingIosPrivacyDisclosureProofRowSchema).pipe(Schema.minItems(1)),
  runtimeEvidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  releaseBlockedCount: TrackingIosPrivacyDisclosureNonNegativeIntegerSchema,
  manualReviewRequiredCount: TrackingIosPrivacyDisclosureNonNegativeIntegerSchema,
  proofNonClaims: Schema.Array(TrackingIosPrivacyDisclosureReleaseProofNonClaimSchema),
  releaseClaimAllowed: Schema.Literal(false),
  appStoreReviewClaimed: Schema.Literal(false),
  privacyNutritionLabelClaimed: Schema.Literal(false),
  coreLocationRuntimeClaimed: Schema.Literal(false),
  backgroundLocationDeliveryClaimed: Schema.Literal(false),
  regionMonitoringClaimed: Schema.Literal(false),
  notificationDeliveryClaimed: Schema.Literal(false),
  entitlementProofClaimed: Schema.Literal(false),
  testflightDeviceInstallClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productReadyIosTrackingClaimed: Schema.Literal(false),
});

type TrackingIosPrivacyDisclosureProofRowCandidate = Infer<typeof TrackingIosPrivacyDisclosureProofRowBaseSchema>;
type TrackingIosPrivacyDisclosureProofReadModelCandidate = Infer<
  typeof TrackingIosPrivacyDisclosureProofReadModelBaseSchema
>;

export const TrackingIosPrivacyDisclosureProofReadModelSchema = withParser(
  TrackingIosPrivacyDisclosureProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingIosPrivacyDisclosureProofReadModelIsHonest(readModel) ||
        'iOS privacy disclosure release proof read model must preserve disclosure blockers and release non-claims'
    )
  )
);

export type TrackingIosPrivacyDisclosureInputRow = Infer<typeof TrackingIosPrivacyDisclosureInputRowSchema>;
export type TrackingIosPrivacyDisclosureProofRow = Infer<typeof TrackingIosPrivacyDisclosureProofRowSchema>;
export type TrackingIosPrivacyDisclosureProofReadModel = Infer<typeof TrackingIosPrivacyDisclosureProofReadModelSchema>;

export type TrackingIosPrivacyDisclosureProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly deviceId: string;
  readonly childProfileId: string;
  readonly deviceLabel: string;
  readonly sourceProofRefs: readonly string[];
};

export function buildTrackingIosPrivacyDisclosureProofReadModel(
  options: TrackingIosPrivacyDisclosureProofOptions,
  inputRows: readonly TrackingIosPrivacyDisclosureInputRow[]
): TrackingIosPrivacyDisclosureProofReadModel {
  const releaseGateRows = inputRows.map((row) => privacyDisclosureProofRowForInput(row));

  return TrackingIosPrivacyDisclosureProofReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: { familyId: options.familyId },
    device: {
      deviceId: options.deviceId,
      childProfileId: options.childProfileId,
      label: options.deviceLabel,
      platform: 'ios',
    },
    sourceProofRefs: options.sourceProofRefs,
    releaseGateRows,
    runtimeEvidenceRefs: releaseGateRows.flatMap(runtimeEvidenceRefsForRow),
    releaseBlockedCount: countRows(releaseGateRows, (row) => row.gateState === 'release-blocked-before-disclosure'),
    manualReviewRequiredCount: countRows(releaseGateRows, (row) => row.gateState === 'manual-review-required'),
    proofNonClaims: RequiredTrackingIosPrivacyDisclosureReleaseProofNonClaims,
    releaseClaimAllowed: false,
    appStoreReviewClaimed: false,
    privacyNutritionLabelClaimed: false,
    coreLocationRuntimeClaimed: false,
    backgroundLocationDeliveryClaimed: false,
    regionMonitoringClaimed: false,
    notificationDeliveryClaimed: false,
    entitlementProofClaimed: false,
    testflightDeviceInstallClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productReadyIosTrackingClaimed: false,
  });
}

function privacyDisclosureProofRowForInput(
  input: TrackingIosPrivacyDisclosureInputRow
): TrackingIosPrivacyDisclosureProofRow {
  return TrackingIosPrivacyDisclosureProofRowSchema.parse({
    ...input,
    gateState: gateStateFor(input),
    parentVisibleStatusToken: parentVisibleStatusTokenFor(input),
    releaseClaimAllowed: false,
    appStoreReviewClaimed: false,
    privacyNutritionLabelClaimed: false,
    coreLocationRuntimeClaimed: false,
    backgroundLocationDeliveryClaimed: false,
    regionMonitoringClaimed: false,
    notificationDeliveryClaimed: false,
    entitlementProofClaimed: false,
    testflightDeviceInstallClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productReadyIosTrackingClaimed: false,
  });
}

function gateStateFor(input: TrackingIosPrivacyDisclosureInputRow) {
  if (input.disclosureEvidenceRefs.length === 0) {
    return 'release-blocked-before-disclosure';
  }
  return 'manual-review-required';
}

function parentVisibleStatusTokenFor(input: TrackingIosPrivacyDisclosureInputRow): string {
  if (input.releaseGate === 'location-purpose-disclosure') {
    return 'tracking-ios-location-purpose-disclosure-required';
  }
  if (input.releaseGate === 'background-location-disclosure') {
    return 'tracking-ios-background-location-disclosure-required';
  }
  if (input.releaseGate === 'region-monitoring-disclosure') {
    return 'tracking-ios-region-monitoring-disclosure-required';
  }
  if (input.releaseGate === 'notification-disclosure') {
    return 'tracking-ios-notification-disclosure-required';
  }
  if (input.releaseGate === 'data-custody-disclosure') {
    return 'tracking-ios-data-custody-disclosure-required';
  }
  return 'tracking-ios-app-store-review-evidence-required';
}

function runtimeEvidenceRefsForRow(row: TrackingIosPrivacyDisclosureProofRow) {
  return row.runtimeEvidenceRefs.map((evidenceReferenceId) => ({
    evidenceReferenceId,
    kind: 'policy-decision',
    observedAt: row.observedAt,
  }));
}

function trackingIosPrivacyDisclosureProofReadModelIsHonest(
  readModel: TrackingIosPrivacyDisclosureProofReadModelCandidate
): boolean {
  const runtimeEvidenceRefCount = readModel.releaseGateRows.reduce(
    (total, row) => total + row.runtimeEvidenceRefs.length,
    0
  );

  return (
    readModel.releaseBlockedCount ===
      countRows(readModel.releaseGateRows, (row) => row.gateState === 'release-blocked-before-disclosure') &&
    readModel.manualReviewRequiredCount ===
      countRows(readModel.releaseGateRows, (row) => row.gateState === 'manual-review-required') &&
    readModel.runtimeEvidenceRefs.length === runtimeEvidenceRefCount &&
    readModelNonClaimsAreHonest(readModel)
  );
}

function trackingIosPrivacyDisclosureProofRowIsHonest(row: TrackingIosPrivacyDisclosureProofRowCandidate): boolean {
  return (
    row.requiredBeforeReleaseClaimRefs.length > 0 &&
    row.manualProofRefs.length > 0 &&
    row.gateState === gateStateFor(row) &&
    row.parentVisibleStatusToken === parentVisibleStatusTokenFor(row) &&
    rowRuntimeClaimsAreFalse(row)
  );
}

function readModelNonClaimsAreHonest(readModel: TrackingIosPrivacyDisclosureProofReadModelCandidate): boolean {
  return (
    readModel.proofNonClaims.length === RequiredTrackingIosPrivacyDisclosureReleaseProofNonClaims.length &&
    RequiredTrackingIosPrivacyDisclosureReleaseProofNonClaims.every((nonClaim) =>
      readModel.proofNonClaims.includes(nonClaim)
    ) &&
    [
      readModel.releaseClaimAllowed,
      readModel.appStoreReviewClaimed,
      readModel.privacyNutritionLabelClaimed,
      readModel.coreLocationRuntimeClaimed,
      readModel.backgroundLocationDeliveryClaimed,
      readModel.regionMonitoringClaimed,
      readModel.notificationDeliveryClaimed,
      readModel.entitlementProofClaimed,
      readModel.testflightDeviceInstallClaimed,
      readModel.physicalDeviceProofClaimed,
      readModel.authorityProofClaimed,
      readModel.productReadyIosTrackingClaimed,
    ].every((claim) => claim === false)
  );
}

function rowRuntimeClaimsAreFalse(row: TrackingIosPrivacyDisclosureProofRowCandidate): boolean {
  return [
    row.releaseClaimAllowed,
    row.appStoreReviewClaimed,
    row.privacyNutritionLabelClaimed,
    row.coreLocationRuntimeClaimed,
    row.backgroundLocationDeliveryClaimed,
    row.regionMonitoringClaimed,
    row.notificationDeliveryClaimed,
    row.entitlementProofClaimed,
    row.testflightDeviceInstallClaimed,
    row.physicalDeviceProofClaimed,
    row.authorityProofClaimed,
    row.productReadyIosTrackingClaimed,
  ].every((claim) => claim === false);
}

function countRows(
  rows: ReadonlyArray<TrackingIosPrivacyDisclosureProofRowCandidate>,
  predicate: (row: TrackingIosPrivacyDisclosureProofRowCandidate) => boolean
): number {
  return rows.filter(predicate).length;
}
