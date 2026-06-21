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
const TrackingIosLocationManualRequiredNonNegativeIntegerSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.nonNegative()
);

export const TrackingIosLocationManualRequiredProofIdSchema = brandedNonEmptyStringSchema('TrackingIosLocationManualRequiredProofId');
export const TrackingIosLocationManualRequiredProofReferenceSchema = brandedNonEmptyStringSchema('TrackingIosLocationManualRequiredProofReference');
export const TrackingIosLocationRuntimeArtifactRefSchema = brandedNonEmptyStringSchema('TrackingIosLocationRuntimeArtifactRef');

export const TrackingIosLocationManualRequiredCaseSchema = withParser(
  Schema.Literal(
    'when-in-use-authorization-manual-required',
    'foreground-sample-manual-required',
    'denied-restricted-services-disabled-manual-required',
    'always-authorization-manual-required',
    'region-transition-manual-required',
    'significant-change-visit-manual-required',
    'background-terminated-relaunch-manual-required'
  )
);
export const TrackingIosLocationManualRequiredSourceSchema = withParser(
  Schema.Literal(
    'ios-simulator-package-proof',
    'ios-simulator-manual-plan',
    'physical-device-manual-plan',
    'apple-entitlement-manual-plan'
  )
);
export const TrackingIosLocationManualRequiredClaimStateSchema = withParser(
  Schema.Literal('simulator-package-observed', 'manual-required')
);

export const RequiredTrackingIosLocationManualRequiredProofNonClaims = [
  'no-when-in-use-authorization-proof',
  'no-foreground-location-sample',
  'no-denied-restricted-runtime-state',
  'no-services-disabled-runtime-state',
  'no-always-authorization-proof',
  'no-region-monitoring-runtime',
  'no-significant-change-runtime',
  'no-visit-event-runtime',
  'no-background-location-delivery',
  'no-terminated-relaunch-proof',
  'no-apple-entitlement-proof',
  'no-notification-delivery',
  'no-provider-delivery',
  'no-physical-device-proof',
  'no-authority-proof',
  'no-product-ready-ios-tracking',
] as const;

export const TrackingIosLocationManualRequiredProofNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingIosLocationManualRequiredProofNonClaims)
);

const TrackingIosLocationManualRequiredInputRowBaseSchema = Schema.Struct({
  rowId: TrackingIosLocationManualRequiredProofReferenceSchema,
  caseKind: TrackingIosLocationManualRequiredCaseSchema,
  source: TrackingIosLocationManualRequiredSourceSchema,
  observedAt: ParentTimestampSchema,
  simulatorPackageBuilt: Schema.Boolean,
  simulatorLaunchObserved: Schema.Boolean,
  whenInUseAuthorizationObserved: Schema.Boolean,
  foregroundLocationSampleCaptured: Schema.Boolean,
  deniedRestrictedStateCaptured: Schema.Boolean,
  locationServicesDisabledStateCaptured: Schema.Boolean,
  alwaysAuthorizationObserved: Schema.Boolean,
  regionTransitionCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  significantChangeEventCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  visitEventCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  backgroundDeliveryObserved: Schema.Boolean,
  terminatedRelaunchObserved: Schema.Boolean,
  entitlementProofObserved: Schema.Boolean,
  evidenceRefs: Schema.Array(TrackingIosLocationManualRequiredProofReferenceSchema).pipe(Schema.minItems(1)),
  manualProofRefs: Schema.Array(TrackingIosLocationManualRequiredProofReferenceSchema).pipe(Schema.minItems(1)),
});

export const TrackingIosLocationManualRequiredInputRowSchema = withParser(
  TrackingIosLocationManualRequiredInputRowBaseSchema
);

const TrackingIosLocationManualRequiredProofRowBaseSchema = Schema.Struct({
  rowId: TrackingIosLocationManualRequiredProofReferenceSchema,
  caseKind: TrackingIosLocationManualRequiredCaseSchema,
  source: TrackingIosLocationManualRequiredSourceSchema,
  claimState: TrackingIosLocationManualRequiredClaimStateSchema,
  observedAt: ParentTimestampSchema,
  simulatorPackageBuilt: Schema.Boolean,
  simulatorLaunchObserved: Schema.Boolean,
  whenInUseAuthorizationObserved: Schema.Boolean,
  foregroundLocationSampleCaptured: Schema.Boolean,
  deniedRestrictedStateCaptured: Schema.Boolean,
  locationServicesDisabledStateCaptured: Schema.Boolean,
  alwaysAuthorizationObserved: Schema.Boolean,
  regionTransitionCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  significantChangeEventCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  visitEventCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  backgroundDeliveryObserved: Schema.Boolean,
  terminatedRelaunchObserved: Schema.Boolean,
  entitlementProofObserved: Schema.Boolean,
  evidenceRefs: Schema.Array(TrackingIosLocationManualRequiredProofReferenceSchema).pipe(Schema.minItems(1)),
  manualProofRefs: Schema.Array(TrackingIosLocationManualRequiredProofReferenceSchema).pipe(Schema.minItems(1)),
  parentVisibleStatusToken: TrackingIosLocationManualRequiredProofReferenceSchema,
  missingProofReasonRefs: Schema.Array(TrackingIosLocationManualRequiredProofReferenceSchema).pipe(Schema.minItems(1)),
  whenInUseAuthorizationClaimed: Schema.Literal(false),
  foregroundLocationSampleClaimed: Schema.Literal(false),
  deniedRestrictedStateClaimed: Schema.Literal(false),
  servicesDisabledStateClaimed: Schema.Literal(false),
  alwaysAuthorizationClaimed: Schema.Literal(false),
  regionMonitoringClaimed: Schema.Literal(false),
  significantChangeClaimed: Schema.Literal(false),
  visitEventClaimed: Schema.Literal(false),
  backgroundLocationDeliveryClaimed: Schema.Literal(false),
  terminatedRelaunchClaimed: Schema.Literal(false),
  entitlementProofClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productReadyIosTrackingClaimed: Schema.Literal(false),
});

export const TrackingIosLocationManualRequiredProofRowSchema = withParser(
  TrackingIosLocationManualRequiredProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingIosLocationManualRequiredProofRowIsHonest(row) ||
        'iOS location manual-required rows must keep manual proof refs and must not claim missing Core Location runtime behavior'
    )
  )
);

const TrackingIosLocationManualRequiredProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingIosLocationManualRequiredProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  sourceProofRefs: Schema.Array(TrackingIosLocationManualRequiredProofReferenceSchema).pipe(Schema.minItems(1)),
  localEvidenceArtifactRefs: Schema.Array(TrackingIosLocationRuntimeArtifactRefSchema).pipe(Schema.minItems(1)),
  requiredRuntimeArtifactRefs: Schema.Array(TrackingIosLocationRuntimeArtifactRefSchema).pipe(Schema.minItems(1)),
  presentRuntimeArtifactRefs: Schema.Array(TrackingIosLocationRuntimeArtifactRefSchema),
  missingRuntimeArtifactRefs: Schema.Array(TrackingIosLocationRuntimeArtifactRefSchema).pipe(Schema.minItems(1)),
  runtimeArtifactSetComplete: Schema.Literal(false),
  runtimeEvidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  rows: Schema.Array(TrackingIosLocationManualRequiredProofRowSchema).pipe(Schema.minItems(1)),
  whenInUseAuthorizationManualRequiredCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  foregroundSampleManualRequiredCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  degradedStateManualRequiredCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  alwaysAuthorizationManualRequiredCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  regionTransitionManualRequiredCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  significantChangeVisitManualRequiredCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  backgroundTerminatedRelaunchManualRequiredCount: TrackingIosLocationManualRequiredNonNegativeIntegerSchema,
  proofNonClaims: Schema.Array(TrackingIosLocationManualRequiredProofNonClaimSchema),
  whenInUseAuthorizationClaimed: Schema.Literal(false),
  foregroundLocationSampleClaimed: Schema.Literal(false),
  deniedRestrictedStateClaimed: Schema.Literal(false),
  servicesDisabledStateClaimed: Schema.Literal(false),
  alwaysAuthorizationClaimed: Schema.Literal(false),
  regionMonitoringClaimed: Schema.Literal(false),
  significantChangeClaimed: Schema.Literal(false),
  visitEventClaimed: Schema.Literal(false),
  backgroundLocationDeliveryClaimed: Schema.Literal(false),
  terminatedRelaunchClaimed: Schema.Literal(false),
  entitlementProofClaimed: Schema.Literal(false),
  notificationDeliveryClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productReadyIosTrackingClaimed: Schema.Literal(false),
});

type TrackingIosLocationManualRequiredProofRowCandidate = Infer<
  typeof TrackingIosLocationManualRequiredProofRowBaseSchema
>;
type TrackingIosLocationManualRequiredProofReadModelCandidate = Infer<
  typeof TrackingIosLocationManualRequiredProofReadModelBaseSchema
>;

export const TrackingIosLocationManualRequiredProofReadModelSchema = withParser(
  TrackingIosLocationManualRequiredProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingIosLocationManualRequiredProofReadModelIsHonest(readModel) ||
        'iOS location manual-required proof read model counts and non-claims must match manual-required rows'
    )
  )
);

export type TrackingIosLocationManualRequiredInputRow = Infer<typeof TrackingIosLocationManualRequiredInputRowSchema>;
export type TrackingIosLocationManualRequiredProofRow = Infer<typeof TrackingIosLocationManualRequiredProofRowSchema>;
export type TrackingIosLocationManualRequiredProofReadModel = Infer<
  typeof TrackingIosLocationManualRequiredProofReadModelSchema
>;

export type TrackingIosLocationManualRequiredProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly deviceId: string;
  readonly childProfileId: string;
  readonly deviceLabel: string;
  readonly sourceProofRefs: readonly string[];
};

export const LocalTrackingIosLocationEvidenceArtifactRefs = [
  'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/18-ios-simulator-proof.json',
  'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/18-ios-simulator-proof.json',
] as const;

export const RequiredTrackingIosLocationRuntimeArtifactRefs = [
  'output/tracking-plan-proof/ios-core-location/when-in-use-authorization-state.json',
  'output/tracking-plan-proof/ios-core-location/foreground-location-events.ndjson',
  'output/tracking-plan-proof/ios-core-location/degraded-location-state.json',
  'output/tracking-plan-proof/ios-region-monitoring/02-authorization-state.json',
  'output/tracking-plan-proof/ios-region-monitoring/05-region-transitions.ndjson',
  'output/tracking-plan-proof/ios-region-monitoring/significant-change-events.ndjson',
  'output/tracking-plan-proof/ios-region-monitoring/visit-events.ndjson',
  'output/tracking-plan-proof/ios-region-monitoring/background-terminated-relaunch-result.json',
  'output/tracking-plan-proof/ios-region-monitoring/authority-entitlement-approval.json',
] as const;

export function buildTrackingIosLocationManualRequiredProofReadModel(
  options: TrackingIosLocationManualRequiredProofOptions,
  inputRows: readonly TrackingIosLocationManualRequiredInputRow[]
): TrackingIosLocationManualRequiredProofReadModel {
  const rows = inputRows.map((row) => iosLocationManualRequiredProofRowForInput(row));

  return TrackingIosLocationManualRequiredProofReadModelSchema.parse({
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
    localEvidenceArtifactRefs: [...LocalTrackingIosLocationEvidenceArtifactRefs],
    requiredRuntimeArtifactRefs: [...RequiredTrackingIosLocationRuntimeArtifactRefs],
    presentRuntimeArtifactRefs: [],
    missingRuntimeArtifactRefs: [...RequiredTrackingIosLocationRuntimeArtifactRefs],
    runtimeArtifactSetComplete: false,
    runtimeEvidenceRefs: rows.flatMap(runtimeEvidenceRefsForRow),
    rows,
    whenInUseAuthorizationManualRequiredCount: countRows(
      rows,
      (row) => row.caseKind === 'when-in-use-authorization-manual-required'
    ),
    foregroundSampleManualRequiredCount: countRows(rows, (row) => row.caseKind === 'foreground-sample-manual-required'),
    degradedStateManualRequiredCount: countRows(
      rows,
      (row) => row.caseKind === 'denied-restricted-services-disabled-manual-required'
    ),
    alwaysAuthorizationManualRequiredCount: countRows(
      rows,
      (row) => row.caseKind === 'always-authorization-manual-required'
    ),
    regionTransitionManualRequiredCount: countRows(rows, (row) => row.caseKind === 'region-transition-manual-required'),
    significantChangeVisitManualRequiredCount: countRows(
      rows,
      (row) => row.caseKind === 'significant-change-visit-manual-required'
    ),
    backgroundTerminatedRelaunchManualRequiredCount: countRows(
      rows,
      (row) => row.caseKind === 'background-terminated-relaunch-manual-required'
    ),
    proofNonClaims: RequiredTrackingIosLocationManualRequiredProofNonClaims,
    whenInUseAuthorizationClaimed: false,
    foregroundLocationSampleClaimed: false,
    deniedRestrictedStateClaimed: false,
    servicesDisabledStateClaimed: false,
    alwaysAuthorizationClaimed: false,
    regionMonitoringClaimed: false,
    significantChangeClaimed: false,
    visitEventClaimed: false,
    backgroundLocationDeliveryClaimed: false,
    terminatedRelaunchClaimed: false,
    entitlementProofClaimed: false,
    notificationDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productReadyIosTrackingClaimed: false,
  });
}

function iosLocationManualRequiredProofRowForInput(
  input: TrackingIosLocationManualRequiredInputRow
): TrackingIosLocationManualRequiredProofRow {
  return TrackingIosLocationManualRequiredProofRowSchema.parse({
    ...input,
    claimState: claimStateFor(input),
    parentVisibleStatusToken: parentVisibleStatusTokenFor(input),
    missingProofReasonRefs: missingProofReasonRefsFor(input),
    whenInUseAuthorizationClaimed: false,
    foregroundLocationSampleClaimed: false,
    deniedRestrictedStateClaimed: false,
    servicesDisabledStateClaimed: false,
    alwaysAuthorizationClaimed: false,
    regionMonitoringClaimed: false,
    significantChangeClaimed: false,
    visitEventClaimed: false,
    backgroundLocationDeliveryClaimed: false,
    terminatedRelaunchClaimed: false,
    entitlementProofClaimed: false,
    physicalDeviceProofClaimed: false,
    productReadyIosTrackingClaimed: false,
  });
}

function runtimeEvidenceRefsForRow(row: TrackingIosLocationManualRequiredProofRow) {
  return row.evidenceRefs.map((evidenceReferenceId) => ({
    evidenceReferenceId,
    kind: 'policy-decision',
    observedAt: row.observedAt,
  }));
}

function claimStateFor(input: TrackingIosLocationManualRequiredInputRow) {
  if (input.simulatorPackageBuilt && input.simulatorLaunchObserved) {
    return 'simulator-package-observed';
  }
  return 'manual-required';
}

function parentVisibleStatusTokenFor(input: TrackingIosLocationManualRequiredInputRow): string {
  if (input.caseKind === 'when-in-use-authorization-manual-required') {
    return 'tracking-ios-when-in-use-authorization-manual-required';
  }
  if (input.caseKind === 'foreground-sample-manual-required') {
    return 'tracking-ios-foreground-sample-manual-required';
  }
  if (input.caseKind === 'denied-restricted-services-disabled-manual-required') {
    return 'tracking-ios-degraded-location-state-manual-required';
  }
  if (input.caseKind === 'always-authorization-manual-required') {
    return 'tracking-ios-always-authorization-manual-required';
  }
  if (input.caseKind === 'region-transition-manual-required') {
    return 'tracking-ios-region-transition-manual-required';
  }
  if (input.caseKind === 'significant-change-visit-manual-required') {
    return 'tracking-ios-significant-change-visit-manual-required';
  }
  return 'tracking-ios-background-terminated-relaunch-manual-required';
}

function missingProofReasonRefsFor(input: TrackingIosLocationManualRequiredInputRow): readonly string[] {
  if (input.caseKind === 'when-in-use-authorization-manual-required') {
    return ['tracking-ios-when-in-use-authorization-not-captured'];
  }
  if (input.caseKind === 'foreground-sample-manual-required') {
    return ['tracking-ios-foreground-location-sample-not-captured'];
  }
  if (input.caseKind === 'denied-restricted-services-disabled-manual-required') {
    return ['tracking-ios-denied-restricted-state-not-captured', 'tracking-ios-services-disabled-state-not-captured'];
  }
  if (input.caseKind === 'always-authorization-manual-required') {
    return ['tracking-ios-always-authorization-not-captured'];
  }
  if (input.caseKind === 'region-transition-manual-required') {
    return ['tracking-ios-region-transition-not-captured'];
  }
  if (input.caseKind === 'significant-change-visit-manual-required') {
    return ['tracking-ios-significant-change-not-captured', 'tracking-ios-visit-event-not-captured'];
  }
  return ['tracking-ios-background-delivery-not-captured', 'tracking-ios-terminated-relaunch-not-captured'];
}

function trackingIosLocationManualRequiredProofReadModelIsHonest(
  readModel: TrackingIosLocationManualRequiredProofReadModelCandidate
): boolean {
  return (
    readModelCountsAreHonest(readModel) &&
    readModelArtifactRefsAreHonest(readModel) &&
    readModelNonClaimsAreHonest(readModel) &&
    readModel.rows.every(
      (row) => row.claimState === 'simulator-package-observed' || row.claimState === 'manual-required'
    )
  );
}

function trackingIosLocationManualRequiredProofRowIsHonest(
  row: TrackingIosLocationManualRequiredProofRowCandidate
): boolean {
  return (
    rowRefsArePresent(row) &&
    rowDerivedStateMatches(row) &&
    rowCaseDoesNotOverclaim(row) &&
    rowRuntimeClaimsAreFalse(row)
  );
}

function readModelCountsAreHonest(readModel: TrackingIosLocationManualRequiredProofReadModelCandidate): boolean {
  const runtimeEvidenceRefCount = readModel.rows.reduce((total, row) => total + row.evidenceRefs.length, 0);

  return (
    readModel.whenInUseAuthorizationManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'when-in-use-authorization-manual-required') &&
    readModel.foregroundSampleManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'foreground-sample-manual-required') &&
    readModel.degradedStateManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'denied-restricted-services-disabled-manual-required') &&
    readModel.alwaysAuthorizationManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'always-authorization-manual-required') &&
    readModel.regionTransitionManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'region-transition-manual-required') &&
    readModel.significantChangeVisitManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'significant-change-visit-manual-required') &&
    readModel.backgroundTerminatedRelaunchManualRequiredCount ===
      countRows(readModel.rows, (row) => row.caseKind === 'background-terminated-relaunch-manual-required') &&
    readModel.runtimeEvidenceRefs.length === runtimeEvidenceRefCount
  );
}

function readModelArtifactRefsAreHonest(readModel: TrackingIosLocationManualRequiredProofReadModelCandidate): boolean {
  return (
    readModel.localEvidenceArtifactRefs.length > 0 &&
    readModel.requiredRuntimeArtifactRefs.length ===
      readModel.presentRuntimeArtifactRefs.length + readModel.missingRuntimeArtifactRefs.length &&
    readModel.presentRuntimeArtifactRefs.length === 0 &&
    readModel.missingRuntimeArtifactRefs.length === RequiredTrackingIosLocationRuntimeArtifactRefs.length &&
    readModel.runtimeArtifactSetComplete === false
  );
}

function readModelNonClaimsAreHonest(readModel: TrackingIosLocationManualRequiredProofReadModelCandidate): boolean {
  return (
    readModel.proofNonClaims.length === RequiredTrackingIosLocationManualRequiredProofNonClaims.length &&
    RequiredTrackingIosLocationManualRequiredProofNonClaims.every((nonClaim) =>
      readModel.proofNonClaims.includes(nonClaim)
    ) &&
    [
      readModel.whenInUseAuthorizationClaimed,
      readModel.foregroundLocationSampleClaimed,
      readModel.deniedRestrictedStateClaimed,
      readModel.servicesDisabledStateClaimed,
      readModel.alwaysAuthorizationClaimed,
      readModel.regionMonitoringClaimed,
      readModel.significantChangeClaimed,
      readModel.visitEventClaimed,
      readModel.backgroundLocationDeliveryClaimed,
      readModel.terminatedRelaunchClaimed,
      readModel.entitlementProofClaimed,
      readModel.notificationDeliveryClaimed,
      readModel.providerDeliveryClaimed,
      readModel.physicalDeviceProofClaimed,
      readModel.authorityProofClaimed,
      readModel.productReadyIosTrackingClaimed,
    ].every((claim) => claim === false)
  );
}

function rowRefsArePresent(row: TrackingIosLocationManualRequiredProofRowCandidate): boolean {
  return row.evidenceRefs.length > 0 && row.manualProofRefs.length > 0 && row.missingProofReasonRefs.length > 0;
}

function rowDerivedStateMatches(row: TrackingIosLocationManualRequiredProofRowCandidate): boolean {
  return (
    row.claimState === claimStateFor(row) &&
    row.parentVisibleStatusToken === parentVisibleStatusTokenFor(row) &&
    row.missingProofReasonRefs.length === missingProofReasonRefsFor(row).length
  );
}

function rowCaseDoesNotOverclaim(row: TrackingIosLocationManualRequiredProofRowCandidate): boolean {
  if (row.caseKind === 'when-in-use-authorization-manual-required') {
    return !row.whenInUseAuthorizationObserved;
  }
  if (row.caseKind === 'foreground-sample-manual-required') {
    return !row.foregroundLocationSampleCaptured;
  }
  if (row.caseKind === 'denied-restricted-services-disabled-manual-required') {
    return !row.deniedRestrictedStateCaptured && !row.locationServicesDisabledStateCaptured;
  }
  if (row.caseKind === 'always-authorization-manual-required') {
    return !row.alwaysAuthorizationObserved && !row.entitlementProofObserved;
  }
  if (row.caseKind === 'region-transition-manual-required') {
    return row.regionTransitionCount === 0;
  }
  if (row.caseKind === 'significant-change-visit-manual-required') {
    return row.significantChangeEventCount === 0 && row.visitEventCount === 0;
  }
  return !row.backgroundDeliveryObserved && !row.terminatedRelaunchObserved;
}

function rowRuntimeClaimsAreFalse(row: TrackingIosLocationManualRequiredProofRowCandidate): boolean {
  return [
    row.whenInUseAuthorizationClaimed,
    row.foregroundLocationSampleClaimed,
    row.deniedRestrictedStateClaimed,
    row.servicesDisabledStateClaimed,
    row.alwaysAuthorizationClaimed,
    row.regionMonitoringClaimed,
    row.significantChangeClaimed,
    row.visitEventClaimed,
    row.backgroundLocationDeliveryClaimed,
    row.terminatedRelaunchClaimed,
    row.entitlementProofClaimed,
    row.physicalDeviceProofClaimed,
    row.productReadyIosTrackingClaimed,
  ].every((claim) => claim === false);
}

function countRows(
  rows: ReadonlyArray<TrackingIosLocationManualRequiredProofRowCandidate>,
  predicate: (row: TrackingIosLocationManualRequiredProofRowCandidate) => boolean
): number {
  return rows.filter(predicate).length;
}

