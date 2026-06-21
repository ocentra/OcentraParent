import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import { FamilyReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from './family-references';
const TrackingAndroidStatusNonNegativeIntegerSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());
const TrackingAndroidBatteryPercentSchema = Schema.Number.pipe(Schema.int(), Schema.between(0, 100));

export const TrackingAndroidStatusProofIdSchema = brandedNonEmptyStringSchema('TrackingAndroidStatusProofId');
export const TrackingAndroidStatusProofReferenceSchema = brandedNonEmptyStringSchema('TrackingAndroidStatusProofReference');

export const TrackingAndroidStatusCaseSchema = withParser(
  Schema.Literal(
    'low-power-degraded',
    'app-killed-restarted',
    'pending-upload-auditable',
    'physical-status-observed',
    'manual-required'
  )
);
export const TrackingAndroidStatusSourceSchema = withParser(
  Schema.Literal(
    'emulator-battery-dump',
    'emulator-activity-manager',
    'query-store-pending-upload',
    'physical-device-battery-connectivity-dump',
    'manual-platform-plan'
  )
);
export const TrackingAndroidStatusClaimStateSchema = withParser(
  Schema.Literal('scaffold-observed', 'degraded', 'physical-status-observed', 'manual-required')
);

export const RequiredTrackingAndroidStatusProofNonClaims = [
  'no-foreground-location-sample',
  'no-background-location-runtime',
  'no-geofence-transition-runtime',
  'no-physical-device-behavior-proof',
  'no-offline-radio-behavior-proof',
  'no-notification-delivery',
  'no-device-owner-authority',
  'no-production-upload-worker',
  'no-product-ready-android-tracking',
] as const;

export const TrackingAndroidStatusProofNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingAndroidStatusProofNonClaims)
);

const TrackingAndroidStatusInputRowBaseSchema = Schema.Struct({
  rowId: TrackingAndroidStatusProofReferenceSchema,
  caseKind: TrackingAndroidStatusCaseSchema,
  source: TrackingAndroidStatusSourceSchema,
  observedAt: ParentTimestampSchema,
  batteryPercent: Schema.Union(TrackingAndroidBatteryPercentSchema, Schema.Null),
  charging: Schema.Boolean,
  lowPowerMode: Schema.Boolean,
  appProcessRunning: Schema.Boolean,
  appRestartObserved: Schema.Boolean,
  pendingUploadCount: TrackingAndroidStatusNonNegativeIntegerSchema,
  evidenceRefs: Schema.Array(TrackingAndroidStatusProofReferenceSchema).pipe(Schema.minItems(1)),
  auditRefs: Schema.Array(TrackingAndroidStatusProofReferenceSchema).pipe(Schema.minItems(1)),
});

export const TrackingAndroidStatusInputRowSchema = withParser(TrackingAndroidStatusInputRowBaseSchema);

const TrackingAndroidStatusProofRowBaseSchema = Schema.Struct({
  rowId: TrackingAndroidStatusProofReferenceSchema,
  caseKind: TrackingAndroidStatusCaseSchema,
  source: TrackingAndroidStatusSourceSchema,
  claimState: TrackingAndroidStatusClaimStateSchema,
  observedAt: ParentTimestampSchema,
  batteryPercent: Schema.Union(TrackingAndroidBatteryPercentSchema, Schema.Null),
  charging: Schema.Boolean,
  lowPowerMode: Schema.Boolean,
  appProcessRunning: Schema.Boolean,
  appRestartObserved: Schema.Boolean,
  pendingUploadCount: TrackingAndroidStatusNonNegativeIntegerSchema,
  evidenceRefs: Schema.Array(TrackingAndroidStatusProofReferenceSchema).pipe(Schema.minItems(1)),
  auditRefs: Schema.Array(TrackingAndroidStatusProofReferenceSchema).pipe(Schema.minItems(1)),
  parentVisibleStatusToken: TrackingAndroidStatusProofReferenceSchema,
  manualRequiredReasonRefs: Schema.Array(TrackingAndroidStatusProofReferenceSchema),
  foregroundLocationClaimed: Schema.Literal(false),
  backgroundLocationRuntimeClaimed: Schema.Literal(false),
  geofenceRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productionUploadWorkerClaimed: Schema.Literal(false),
});

export const TrackingAndroidStatusProofRowSchema = withParser(
  TrackingAndroidStatusProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingAndroidStatusProofRowIsHonest(row) ||
        'Android status proof rows need evidence/audit refs, correct degraded/manual state, pending-upload auditability, and no runtime/device overclaims'
    )
  )
);

const TrackingAndroidStatusProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingAndroidStatusProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  sourceProofRefs: Schema.Array(TrackingAndroidStatusProofReferenceSchema).pipe(Schema.minItems(1)),
  runtimeEvidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  rows: Schema.Array(TrackingAndroidStatusProofRowSchema).pipe(Schema.minItems(1)),
  lowPowerDegradedCount: TrackingAndroidStatusNonNegativeIntegerSchema,
  appRestartObservedCount: TrackingAndroidStatusNonNegativeIntegerSchema,
  pendingUploadAuditableCount: TrackingAndroidStatusNonNegativeIntegerSchema,
  physicalStatusObservedCount: TrackingAndroidStatusNonNegativeIntegerSchema,
  manualRequiredCount: TrackingAndroidStatusNonNegativeIntegerSchema,
  proofNonClaims: Schema.Array(TrackingAndroidStatusProofNonClaimSchema),
  physicalDeviceStatusEvidenceObserved: Schema.Boolean,
  foregroundLocationClaimed: Schema.Literal(false),
  backgroundLocationRuntimeClaimed: Schema.Literal(false),
  geofenceRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  notificationDeliveryClaimed: Schema.Literal(false),
  deviceOwnerAuthorityClaimed: Schema.Literal(false),
  productionUploadWorkerClaimed: Schema.Literal(false),
  productReadyAndroidTrackingClaimed: Schema.Literal(false),
});

type TrackingAndroidStatusProofRowCandidate = Infer<typeof TrackingAndroidStatusProofRowBaseSchema>;
type TrackingAndroidStatusProofReadModelCandidate = Infer<typeof TrackingAndroidStatusProofReadModelBaseSchema>;

export const TrackingAndroidStatusProofReadModelSchema = withParser(
  TrackingAndroidStatusProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingAndroidStatusProofReadModelIsHonest(readModel) ||
        'Android status proof read model counts and non-claims must match rows without claiming runtime Android tracking'
    )
  )
);

export type TrackingAndroidStatusInputRow = Infer<typeof TrackingAndroidStatusInputRowSchema>;
export type TrackingAndroidStatusProofRow = Infer<typeof TrackingAndroidStatusProofRowSchema>;
export type TrackingAndroidStatusProofReadModel = Infer<typeof TrackingAndroidStatusProofReadModelSchema>;

export type TrackingAndroidStatusProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly deviceId: string;
  readonly childProfileId: string;
  readonly deviceLabel: string;
  readonly sourceProofRefs: readonly string[];
};

export function buildTrackingAndroidStatusProofReadModel(
  options: TrackingAndroidStatusProofOptions,
  inputRows: readonly TrackingAndroidStatusInputRow[]
): TrackingAndroidStatusProofReadModel {
  const rows = inputRows.map((row) => androidStatusProofRowForInput(row));

  return TrackingAndroidStatusProofReadModelSchema.parse({
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
    lowPowerDegradedCount: rows.filter((row) => row.caseKind === 'low-power-degraded').length,
    appRestartObservedCount: rows.filter((row) => row.appRestartObserved).length,
    pendingUploadAuditableCount: rows.filter((row) => row.pendingUploadCount > 0).length,
    physicalStatusObservedCount: rows.filter((row) => row.caseKind === 'physical-status-observed').length,
    manualRequiredCount: rows.filter((row) => row.claimState === 'manual-required').length,
    proofNonClaims: RequiredTrackingAndroidStatusProofNonClaims,
    physicalDeviceStatusEvidenceObserved: rows.some((row) => row.caseKind === 'physical-status-observed'),
    foregroundLocationClaimed: false,
    backgroundLocationRuntimeClaimed: false,
    geofenceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    notificationDeliveryClaimed: false,
    deviceOwnerAuthorityClaimed: false,
    productionUploadWorkerClaimed: false,
    productReadyAndroidTrackingClaimed: false,
  });
}

function runtimeEvidenceRefsForRow(row: TrackingAndroidStatusProofRow) {
  return row.evidenceRefs.map((evidenceReferenceId) => ({
    evidenceReferenceId,
    kind: evidenceKindFor(row),
    observedAt: row.observedAt,
  }));
}

function evidenceKindFor(row: TrackingAndroidStatusProofRow) {
  if (row.source === 'query-store-pending-upload') {
    return 'query-store-summary';
  }
  if (row.source === 'manual-platform-plan') {
    return 'policy-decision';
  }
  return 'activity-event';
}

function androidStatusProofRowForInput(input: TrackingAndroidStatusInputRow): TrackingAndroidStatusProofRow {
  return TrackingAndroidStatusProofRowSchema.parse({
    ...input,
    claimState: claimStateFor(input),
    parentVisibleStatusToken: parentVisibleStatusTokenFor(input),
    manualRequiredReasonRefs: manualRequiredReasonRefsFor(input),
    foregroundLocationClaimed: false,
    backgroundLocationRuntimeClaimed: false,
    geofenceRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    productionUploadWorkerClaimed: false,
  });
}

function claimStateFor(input: TrackingAndroidStatusInputRow) {
  if (input.caseKind === 'manual-required') {
    return 'manual-required';
  }
  if (input.caseKind === 'physical-status-observed') {
    return 'physical-status-observed';
  }
  if (input.lowPowerMode || input.pendingUploadCount > 0 || input.appRestartObserved) {
    return 'degraded';
  }
  return 'scaffold-observed';
}

function parentVisibleStatusTokenFor(input: TrackingAndroidStatusInputRow): string {
  if (input.caseKind === 'low-power-degraded') {
    return 'tracking-android-status-low-power-degraded';
  }
  if (input.caseKind === 'app-killed-restarted') {
    return 'tracking-android-status-app-restarted-audit';
  }
  if (input.caseKind === 'pending-upload-auditable') {
    return 'tracking-android-status-pending-upload-audit';
  }
  if (input.caseKind === 'physical-status-observed') {
    return 'tracking-android-status-physical-battery-connectivity-observed';
  }
  return 'tracking-android-status-manual-platform-proof-required';
}

function manualRequiredReasonRefsFor(input: TrackingAndroidStatusInputRow): readonly string[] {
  if (input.caseKind !== 'manual-required') {
    return [];
  }
  return [
    'tracking-android-background-location-not-proved',
    'tracking-android-physical-device-not-proved',
    'tracking-android-device-owner-not-proved',
  ];
}

function trackingAndroidStatusProofReadModelIsHonest(readModel: TrackingAndroidStatusProofReadModelCandidate): boolean {
  return readModelCountsAreHonest(readModel) && readModelNonClaimsAreHonest(readModel);
}

function trackingAndroidStatusProofRowIsHonest(row: TrackingAndroidStatusProofRowCandidate): boolean {
  return (
    rowRefsArePresent(row) &&
    rowDerivedStateMatches(row) &&
    rowCaseEvidenceMatches(row) &&
    rowRuntimeClaimsAreFalse(row)
  );
}

function readModelCountsAreHonest(readModel: TrackingAndroidStatusProofReadModelCandidate): boolean {
  const runtimeEvidenceRefCount = readModel.rows.reduce((total, row) => total + row.evidenceRefs.length, 0);

  return (
    readModel.lowPowerDegradedCount === countRows(readModel.rows, (row) => row.caseKind === 'low-power-degraded') &&
    readModel.appRestartObservedCount === countRows(readModel.rows, (row) => row.appRestartObserved) &&
    readModel.pendingUploadAuditableCount === countRows(readModel.rows, (row) => row.pendingUploadCount > 0) &&
    readModel.physicalStatusObservedCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'physical-status-observed') &&
    readModel.physicalDeviceStatusEvidenceObserved ===
      readModel.rows.some((row) => row.caseKind === 'physical-status-observed') &&
    readModel.manualRequiredCount === countRows(readModel.rows, (row) => row.claimState === 'manual-required') &&
    readModel.runtimeEvidenceRefs.length === runtimeEvidenceRefCount
  );
}

function readModelNonClaimsAreHonest(readModel: TrackingAndroidStatusProofReadModelCandidate): boolean {
  return (
    readModel.proofNonClaims.length === RequiredTrackingAndroidStatusProofNonClaims.length &&
    RequiredTrackingAndroidStatusProofNonClaims.every((nonClaim) => readModel.proofNonClaims.includes(nonClaim)) &&
    readModelClaimsAreFalse(readModel)
  );
}

function readModelClaimsAreFalse(readModel: TrackingAndroidStatusProofReadModelCandidate): boolean {
  return [
    readModel.foregroundLocationClaimed,
    readModel.backgroundLocationRuntimeClaimed,
    readModel.geofenceRuntimeClaimed,
    readModel.physicalDeviceProofClaimed,
    readModel.notificationDeliveryClaimed,
    readModel.deviceOwnerAuthorityClaimed,
    readModel.productionUploadWorkerClaimed,
    readModel.productReadyAndroidTrackingClaimed,
  ].every((claim) => claim === false);
}

function rowRefsArePresent(row: TrackingAndroidStatusProofRowCandidate): boolean {
  return row.evidenceRefs.length > 0 && row.auditRefs.length > 0;
}

function rowDerivedStateMatches(row: TrackingAndroidStatusProofRowCandidate): boolean {
  return (
    row.claimState === claimStateFor(row) &&
    row.parentVisibleStatusToken === parentVisibleStatusTokenFor(row) &&
    row.manualRequiredReasonRefs.length === manualRequiredReasonRefsFor(row).length
  );
}

function rowCaseEvidenceMatches(row: TrackingAndroidStatusProofRowCandidate): boolean {
  const lowPowerMatches = row.caseKind !== 'low-power-degraded' || (row.lowPowerMode && row.claimState === 'degraded');
  const restartMatches = row.caseKind !== 'app-killed-restarted' || row.appRestartObserved;
  const uploadMatches = row.caseKind !== 'pending-upload-auditable' || row.pendingUploadCount > 0;
  const physicalStatusMatches =
    row.caseKind !== 'physical-status-observed' ||
    (row.source === 'physical-device-battery-connectivity-dump' && row.batteryPercent !== null);
  return lowPowerMatches && restartMatches && uploadMatches && physicalStatusMatches;
}

function rowRuntimeClaimsAreFalse(row: TrackingAndroidStatusProofRowCandidate): boolean {
  return [
    row.foregroundLocationClaimed,
    row.backgroundLocationRuntimeClaimed,
    row.geofenceRuntimeClaimed,
    row.physicalDeviceProofClaimed,
    row.productionUploadWorkerClaimed,
  ].every((claim) => claim === false);
}

function countRows(
  rows: ReadonlyArray<TrackingAndroidStatusProofRowCandidate>,
  predicate: (row: TrackingAndroidStatusProofRowCandidate) => boolean
): number {
  return rows.filter(predicate).length;
}

