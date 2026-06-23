import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import { FamilyReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from './family-references';
const TrackingAndroidPermissionNonNegativeIntegerSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingAndroidPermissionBackgroundProofIdSchema = brandedNonEmptyStringSchema(
  'TrackingAndroidPermissionBackgroundProofId'
);
export const TrackingAndroidPermissionBackgroundProofReferenceSchema = brandedNonEmptyStringSchema(
  'TrackingAndroidPermissionBackgroundProofReference'
);

export const TrackingAndroidPermissionBackgroundCaseSchema = withParser(
  Schema.Literal(
    'foreground-permission-manual-required',
    'foreground-sample-manual-required',
    'background-permission-manual-required',
    'geofence-transition-manual-required'
  )
);
export const TrackingAndroidPermissionBackgroundSourceSchema = withParser(
  Schema.Literal(
    'android-emulator-foreground-proof',
    'android-emulator-background-proof',
    'android-studio-manual-plan',
    'physical-device-manual-plan'
  )
);
export const TrackingAndroidPermissionBackgroundClaimStateSchema = withParser(
  Schema.Literal('scaffold-observed', 'manual-required')
);

export const RequiredTrackingAndroidPermissionBackgroundProofNonClaims = [
  'no-foreground-location-permission-grant',
  'no-foreground-location-sample',
  'no-background-location-permission-grant',
  'no-geofence-transition-runtime',
  'no-background-location-runtime',
  'no-physical-device-proof',
  'no-device-owner-authority',
  'no-notification-delivery',
  'no-provider-delivery',
  'no-product-ready-android-tracking',
] as const;

export const TrackingAndroidPermissionBackgroundProofNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingAndroidPermissionBackgroundProofNonClaims)
);

const TrackingAndroidPermissionBackgroundInputRowBaseSchema = Schema.Struct({
  rowId: TrackingAndroidPermissionBackgroundProofReferenceSchema,
  caseKind: TrackingAndroidPermissionBackgroundCaseSchema,
  source: TrackingAndroidPermissionBackgroundSourceSchema,
  observedAt: ParentTimestampSchema,
  packageLaunchObserved: Schema.Boolean,
  foregroundServiceObserved: Schema.Boolean,
  foregroundPermissionRequested: Schema.Boolean,
  foregroundLocationSampleCaptured: Schema.Boolean,
  backgroundPermissionRequested: Schema.Boolean,
  geofenceTransitionCount: TrackingAndroidPermissionNonNegativeIntegerSchema,
  evidenceRefs: Schema.Array(TrackingAndroidPermissionBackgroundProofReferenceSchema).pipe(Schema.minItems(1)),
  manualProofRefs: Schema.Array(TrackingAndroidPermissionBackgroundProofReferenceSchema).pipe(Schema.minItems(1)),
});

export const TrackingAndroidPermissionBackgroundInputRowSchema = withParser(
  TrackingAndroidPermissionBackgroundInputRowBaseSchema
);

const TrackingAndroidPermissionBackgroundProofRowBaseSchema = Schema.Struct({
  rowId: TrackingAndroidPermissionBackgroundProofReferenceSchema,
  caseKind: TrackingAndroidPermissionBackgroundCaseSchema,
  source: TrackingAndroidPermissionBackgroundSourceSchema,
  claimState: TrackingAndroidPermissionBackgroundClaimStateSchema,
  observedAt: ParentTimestampSchema,
  packageLaunchObserved: Schema.Boolean,
  foregroundServiceObserved: Schema.Boolean,
  foregroundPermissionRequested: Schema.Boolean,
  foregroundLocationSampleCaptured: Schema.Boolean,
  backgroundPermissionRequested: Schema.Boolean,
  geofenceTransitionCount: TrackingAndroidPermissionNonNegativeIntegerSchema,
  evidenceRefs: Schema.Array(TrackingAndroidPermissionBackgroundProofReferenceSchema).pipe(Schema.minItems(1)),
  manualProofRefs: Schema.Array(TrackingAndroidPermissionBackgroundProofReferenceSchema).pipe(Schema.minItems(1)),
  parentVisibleStatusToken: TrackingAndroidPermissionBackgroundProofReferenceSchema,
  missingProofReasonRefs: Schema.Array(TrackingAndroidPermissionBackgroundProofReferenceSchema).pipe(
    Schema.minItems(1)
  ),
  foregroundPermissionClaimed: Schema.Literal(false),
  foregroundLocationSampleClaimed: Schema.Literal(false),
  backgroundPermissionClaimed: Schema.Literal(false),
  backgroundLocationRuntimeClaimed: Schema.Literal(false),
  geofenceRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productReadyAndroidTrackingClaimed: Schema.Literal(false),
});

export const TrackingAndroidPermissionBackgroundProofRowSchema = withParser(
  TrackingAndroidPermissionBackgroundProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingAndroidPermissionBackgroundProofRowIsHonest(row) ||
        'Android permission/background proof rows must keep manual-required source refs and must not claim missing location/geofence runtime behavior'
    )
  )
);

const TrackingAndroidPermissionBackgroundProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingAndroidPermissionBackgroundProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  sourceProofRefs: Schema.Array(TrackingAndroidPermissionBackgroundProofReferenceSchema).pipe(Schema.minItems(1)),
  runtimeEvidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  rows: Schema.Array(TrackingAndroidPermissionBackgroundProofRowSchema).pipe(Schema.minItems(1)),
  foregroundPermissionManualRequiredCount: TrackingAndroidPermissionNonNegativeIntegerSchema,
  foregroundSampleManualRequiredCount: TrackingAndroidPermissionNonNegativeIntegerSchema,
  backgroundPermissionManualRequiredCount: TrackingAndroidPermissionNonNegativeIntegerSchema,
  geofenceTransitionManualRequiredCount: TrackingAndroidPermissionNonNegativeIntegerSchema,
  proofNonClaims: Schema.Array(TrackingAndroidPermissionBackgroundProofNonClaimSchema),
  foregroundPermissionClaimed: Schema.Literal(false),
  foregroundLocationSampleClaimed: Schema.Literal(false),
  backgroundPermissionClaimed: Schema.Literal(false),
  backgroundLocationRuntimeClaimed: Schema.Literal(false),
  geofenceRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  deviceOwnerAuthorityClaimed: Schema.Literal(false),
  notificationDeliveryClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  productReadyAndroidTrackingClaimed: Schema.Literal(false),
});

type TrackingAndroidPermissionBackgroundProofRowCandidate = Infer<
  typeof TrackingAndroidPermissionBackgroundProofRowBaseSchema
>;
type TrackingAndroidPermissionBackgroundProofReadModelCandidate = Infer<
  typeof TrackingAndroidPermissionBackgroundProofReadModelBaseSchema
>;

export const TrackingAndroidPermissionBackgroundProofReadModelSchema = withParser(
  TrackingAndroidPermissionBackgroundProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingAndroidPermissionBackgroundProofReadModelIsHonest(readModel) ||
        'Android permission/background proof read model counts and non-claims must match manual-required rows'
    )
  )
);

export type TrackingAndroidPermissionBackgroundInputRow = Infer<
  typeof TrackingAndroidPermissionBackgroundInputRowSchema
>;
export type TrackingAndroidPermissionBackgroundProofRow = Infer<
  typeof TrackingAndroidPermissionBackgroundProofRowSchema
>;
export type TrackingAndroidPermissionBackgroundProofReadModel = Infer<
  typeof TrackingAndroidPermissionBackgroundProofReadModelSchema
>;

export type TrackingAndroidPermissionBackgroundProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly deviceId: string;
  readonly childProfileId: string;
  readonly deviceLabel: string;
  readonly sourceProofRefs: readonly string[];
};

export function buildTrackingAndroidPermissionBackgroundProofReadModel(
  options: TrackingAndroidPermissionBackgroundProofOptions,
  inputRows: readonly TrackingAndroidPermissionBackgroundInputRow[]
): TrackingAndroidPermissionBackgroundProofReadModel {
  const rows = inputRows.map((row) => androidPermissionBackgroundProofRowForInput(row));

  return TrackingAndroidPermissionBackgroundProofReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: { familyId: options.familyId },
    device: {
      deviceId: options.deviceId,
      childProfileId: options.childProfileId,
      label: options.deviceLabel,
      platform: 'android',
    },
    sourceProofRefs: options.sourceProofRefs,
    runtimeEvidenceRefs: rows.flatMap(runtimeEvidenceRefsForRow),
    rows,
    foregroundPermissionManualRequiredCount: countRows(
      rows,
      (row) => row.caseKind === 'foreground-permission-manual-required'
    ),
    foregroundSampleManualRequiredCount: countRows(rows, (row) => row.caseKind === 'foreground-sample-manual-required'),
    backgroundPermissionManualRequiredCount: countRows(
      rows,
      (row) => row.caseKind === 'background-permission-manual-required'
    ),
    geofenceTransitionManualRequiredCount: countRows(
      rows,
      (row) => row.caseKind === 'geofence-transition-manual-required'
    ),
    proofNonClaims: RequiredTrackingAndroidPermissionBackgroundProofNonClaims,
    foregroundPermissionClaimed: false,
    foregroundLocationSampleClaimed: false,
    backgroundPermissionClaimed: false,
    backgroundLocationRuntimeClaimed: false,
    geofenceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    deviceOwnerAuthorityClaimed: false,
    notificationDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    productReadyAndroidTrackingClaimed: false,
  });
}

function androidPermissionBackgroundProofRowForInput(
  input: TrackingAndroidPermissionBackgroundInputRow
): TrackingAndroidPermissionBackgroundProofRow {
  return TrackingAndroidPermissionBackgroundProofRowSchema.parse({
    ...input,
    claimState: claimStateFor(input),
    parentVisibleStatusToken: parentVisibleStatusTokenFor(input),
    missingProofReasonRefs: missingProofReasonRefsFor(input),
    foregroundPermissionClaimed: false,
    foregroundLocationSampleClaimed: false,
    backgroundPermissionClaimed: false,
    backgroundLocationRuntimeClaimed: false,
    geofenceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    productReadyAndroidTrackingClaimed: false,
  });
}

function runtimeEvidenceRefsForRow(row: TrackingAndroidPermissionBackgroundProofRow) {
  return row.evidenceRefs.map((evidenceReferenceId) => ({
    evidenceReferenceId,
    kind: 'policy-decision',
    observedAt: row.observedAt,
  }));
}

function claimStateFor(input: TrackingAndroidPermissionBackgroundInputRow) {
  if (input.packageLaunchObserved && input.foregroundServiceObserved) {
    return 'scaffold-observed';
  }
  return 'manual-required';
}

function parentVisibleStatusTokenFor(input: TrackingAndroidPermissionBackgroundInputRow): string {
  if (input.caseKind === 'foreground-permission-manual-required') {
    return 'tracking-android-foreground-permission-manual-required';
  }
  if (input.caseKind === 'foreground-sample-manual-required') {
    return 'tracking-android-foreground-sample-manual-required';
  }
  if (input.caseKind === 'background-permission-manual-required') {
    return 'tracking-android-background-permission-manual-required';
  }
  return 'tracking-android-geofence-transition-manual-required';
}

function missingProofReasonRefsFor(input: TrackingAndroidPermissionBackgroundInputRow): readonly string[] {
  if (input.caseKind === 'foreground-permission-manual-required') {
    return ['tracking-android-foreground-permission-grant-not-captured'];
  }
  if (input.caseKind === 'foreground-sample-manual-required') {
    return ['tracking-android-foreground-location-sample-not-captured'];
  }
  if (input.caseKind === 'background-permission-manual-required') {
    return ['tracking-android-background-permission-grant-not-captured'];
  }
  return ['tracking-android-geofence-transition-not-captured'];
}

function trackingAndroidPermissionBackgroundProofReadModelIsHonest(
  readModel: TrackingAndroidPermissionBackgroundProofReadModelCandidate
): boolean {
  return (
    readModelCountsAreHonest(readModel) &&
    readModelNonClaimsAreHonest(readModel) &&
    readModel.rows.every((row) => row.claimState === 'scaffold-observed' || row.claimState === 'manual-required')
  );
}

function trackingAndroidPermissionBackgroundProofRowIsHonest(
  row: TrackingAndroidPermissionBackgroundProofRowCandidate
): boolean {
  return (
    rowRefsArePresent(row) &&
    rowDerivedStateMatches(row) &&
    rowCaseDoesNotOverclaim(row) &&
    rowRuntimeClaimsAreFalse(row)
  );
}

function readModelCountsAreHonest(readModel: TrackingAndroidPermissionBackgroundProofReadModelCandidate): boolean {
  const runtimeEvidenceRefCount = readModel.rows.reduce((total, row) => total + row.evidenceRefs.length, 0);

  return (
    readModel.foregroundPermissionManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'foreground-permission-manual-required') &&
    readModel.foregroundSampleManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'foreground-sample-manual-required') &&
    readModel.backgroundPermissionManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'background-permission-manual-required') &&
    readModel.geofenceTransitionManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'geofence-transition-manual-required') &&
    readModel.runtimeEvidenceRefs.length === runtimeEvidenceRefCount
  );
}

function readModelNonClaimsAreHonest(readModel: TrackingAndroidPermissionBackgroundProofReadModelCandidate): boolean {
  return (
    readModel.proofNonClaims.length === RequiredTrackingAndroidPermissionBackgroundProofNonClaims.length &&
    RequiredTrackingAndroidPermissionBackgroundProofNonClaims.every((nonClaim) =>
      readModel.proofNonClaims.includes(nonClaim)
    ) &&
    [
      readModel.foregroundPermissionClaimed,
      readModel.foregroundLocationSampleClaimed,
      readModel.backgroundPermissionClaimed,
      readModel.backgroundLocationRuntimeClaimed,
      readModel.geofenceRuntimeClaimed,
      readModel.physicalDeviceProofClaimed,
      readModel.deviceOwnerAuthorityClaimed,
      readModel.notificationDeliveryClaimed,
      readModel.providerDeliveryClaimed,
      readModel.productReadyAndroidTrackingClaimed,
    ].every((claim) => claim === false)
  );
}

function rowRefsArePresent(row: TrackingAndroidPermissionBackgroundProofRowCandidate): boolean {
  return row.evidenceRefs.length > 0 && row.manualProofRefs.length > 0 && row.missingProofReasonRefs.length > 0;
}

function rowDerivedStateMatches(row: TrackingAndroidPermissionBackgroundProofRowCandidate): boolean {
  return (
    row.claimState === claimStateFor(row) &&
    row.parentVisibleStatusToken === parentVisibleStatusTokenFor(row) &&
    row.missingProofReasonRefs.length === missingProofReasonRefsFor(row).length
  );
}

function rowCaseDoesNotOverclaim(row: TrackingAndroidPermissionBackgroundProofRowCandidate): boolean {
  if (row.caseKind === 'foreground-permission-manual-required') {
    return !row.foregroundPermissionRequested;
  }
  if (row.caseKind === 'foreground-sample-manual-required') {
    return !row.foregroundLocationSampleCaptured;
  }
  if (row.caseKind === 'background-permission-manual-required') {
    return !row.backgroundPermissionRequested;
  }
  return row.geofenceTransitionCount === 0;
}

function rowRuntimeClaimsAreFalse(row: TrackingAndroidPermissionBackgroundProofRowCandidate): boolean {
  return [
    row.foregroundPermissionClaimed,
    row.foregroundLocationSampleClaimed,
    row.backgroundPermissionClaimed,
    row.backgroundLocationRuntimeClaimed,
    row.geofenceRuntimeClaimed,
    row.physicalDeviceProofClaimed,
    row.productReadyAndroidTrackingClaimed,
  ].every((claim) => claim === false);
}

function countRows(
  rows: ReadonlyArray<TrackingAndroidPermissionBackgroundProofRowCandidate>,
  predicate: (row: TrackingAndroidPermissionBackgroundProofRowCandidate) => boolean
): number {
  return rows.filter(predicate).length;
}
