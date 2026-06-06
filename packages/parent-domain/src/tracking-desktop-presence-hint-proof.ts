import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import { FamilyReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from './references';

const TrackingDesktopPresenceTextSchema = Schema.String.pipe(Schema.minLength(1));
const TrackingDesktopPresenceCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingDesktopPresenceHintProofIdSchema = TrackingDesktopPresenceTextSchema.pipe(
  Schema.brand('TrackingDesktopPresenceHintProofId')
);
export const TrackingDesktopPresenceHintProofReferenceSchema = TrackingDesktopPresenceTextSchema.pipe(
  Schema.brand('TrackingDesktopPresenceHintProofReference')
);

export const TrackingDesktopPresenceHintCaseSchema = withParser(
  Schema.Literal(
    'lan-presence-hint',
    'wifi-presence-hint',
    'ip-coarse-hint',
    'manual-check-in',
    'stale-offline-last-known',
    'missing-device',
    'desktop-os-location-manual-required'
  )
);
export const TrackingDesktopPresenceHintSourceSchema = withParser(
  Schema.Literal(
    'lan-pairing-presence',
    'wifi-network-hint',
    'ip-coarse-hint',
    'child-manual-check-in',
    'query-store-last-known',
    'query-store-missing-device',
    'manual-platform-plan'
  )
);
export const TrackingDesktopPresenceHintClaimStateSchema = withParser(
  Schema.Literal('hint-only', 'manual-check-in', 'stale-offline', 'missing-device', 'manual-required')
);
export const TrackingDesktopPresenceFreshnessStateSchema = withParser(
  Schema.Literal('fresh-hint', 'manual-reported', 'stale', 'offline', 'missing', 'manual-required')
);

export const RequiredTrackingDesktopPresenceHintProofNonClaims = [
  'no-gps-from-lan-wifi-ip',
  'no-precise-location-from-network-hints',
  'no-lan-pairing-physical-presence-proof',
  'no-wifi-physical-presence-proof',
  'no-ip-physical-presence-proof',
  'no-desktop-os-location-runtime',
  'no-physical-device-proof',
  'no-product-ready-desktop-tracking',
] as const;

export const TrackingDesktopPresenceHintProofNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingDesktopPresenceHintProofNonClaims)
);

const TrackingDesktopPresenceHintInputRowBaseSchema = Schema.Struct({
  rowId: TrackingDesktopPresenceHintProofReferenceSchema,
  caseKind: TrackingDesktopPresenceHintCaseSchema,
  source: TrackingDesktopPresenceHintSourceSchema,
  observedAt: ParentTimestampSchema,
  platform: ParentPlatformSchema,
  freshnessState: TrackingDesktopPresenceFreshnessStateSchema,
  evidenceRefs: Schema.Array(TrackingDesktopPresenceHintProofReferenceSchema).pipe(Schema.minItems(1)),
  auditRefs: Schema.Array(TrackingDesktopPresenceHintProofReferenceSchema).pipe(Schema.minItems(1)),
  lastKnownEvidenceRef: Schema.Union(TrackingDesktopPresenceHintProofReferenceSchema, Schema.Null),
  manualCheckInRef: Schema.Union(TrackingDesktopPresenceHintProofReferenceSchema, Schema.Null),
  stale: Schema.Boolean,
  offline: Schema.Boolean,
  missingDevice: Schema.Boolean,
});

export const TrackingDesktopPresenceHintInputRowSchema = withParser(TrackingDesktopPresenceHintInputRowBaseSchema);

const TrackingDesktopPresenceHintProofRowBaseSchema = Schema.Struct({
  rowId: TrackingDesktopPresenceHintProofReferenceSchema,
  caseKind: TrackingDesktopPresenceHintCaseSchema,
  source: TrackingDesktopPresenceHintSourceSchema,
  claimState: TrackingDesktopPresenceHintClaimStateSchema,
  observedAt: ParentTimestampSchema,
  platform: ParentPlatformSchema,
  freshnessState: TrackingDesktopPresenceFreshnessStateSchema,
  evidenceRefs: Schema.Array(TrackingDesktopPresenceHintProofReferenceSchema).pipe(Schema.minItems(1)),
  auditRefs: Schema.Array(TrackingDesktopPresenceHintProofReferenceSchema).pipe(Schema.minItems(1)),
  lastKnownEvidenceRef: Schema.Union(TrackingDesktopPresenceHintProofReferenceSchema, Schema.Null),
  manualCheckInRef: Schema.Union(TrackingDesktopPresenceHintProofReferenceSchema, Schema.Null),
  stale: Schema.Boolean,
  offline: Schema.Boolean,
  missingDevice: Schema.Boolean,
  parentVisibleStatusToken: TrackingDesktopPresenceHintProofReferenceSchema,
  manualRequiredReasonRefs: Schema.Array(TrackingDesktopPresenceHintProofReferenceSchema),
  preciseLocationClaimed: Schema.Literal(false),
  gpsClaimed: Schema.Literal(false),
  physicalPresenceClaimed: Schema.Literal(false),
  lanPairingPhysicalProofClaimed: Schema.Literal(false),
  osLocationRuntimeClaimed: Schema.Literal(false),
});

export const TrackingDesktopPresenceHintProofRowSchema = withParser(
  TrackingDesktopPresenceHintProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingDesktopPresenceHintProofRowIsHonest(row) ||
        'Desktop presence rows need honest hint/manual/stale/missing state, evidence refs, manual-required reasons, and no GPS/precise/physical-presence overclaims'
    )
  )
);

const TrackingDesktopPresenceHintReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingDesktopPresenceHintProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  sourceProofRefs: Schema.Array(TrackingDesktopPresenceHintProofReferenceSchema).pipe(Schema.minItems(1)),
  runtimeEvidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  rows: Schema.Array(TrackingDesktopPresenceHintProofRowSchema).pipe(Schema.minItems(1)),
  hintOnlyCount: TrackingDesktopPresenceCountSchema,
  manualCheckInCount: TrackingDesktopPresenceCountSchema,
  staleOfflineCount: TrackingDesktopPresenceCountSchema,
  missingDeviceCount: TrackingDesktopPresenceCountSchema,
  manualRequiredCount: TrackingDesktopPresenceCountSchema,
  proofNonClaims: Schema.Array(TrackingDesktopPresenceHintProofNonClaimSchema),
  preciseLocationClaimed: Schema.Literal(false),
  gpsClaimed: Schema.Literal(false),
  physicalPresenceClaimed: Schema.Literal(false),
  lanPairingPhysicalProofClaimed: Schema.Literal(false),
  wifiPhysicalPresenceClaimed: Schema.Literal(false),
  ipPhysicalPresenceClaimed: Schema.Literal(false),
  osLocationRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productReadyDesktopTrackingClaimed: Schema.Literal(false),
});

type TrackingDesktopPresenceHintProofRowCandidate = Infer<typeof TrackingDesktopPresenceHintProofRowBaseSchema>;
type TrackingDesktopPresenceHintReadModelCandidate = Infer<typeof TrackingDesktopPresenceHintReadModelBaseSchema>;

export const TrackingDesktopPresenceHintProofReadModelSchema = withParser(
  TrackingDesktopPresenceHintReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingDesktopPresenceHintReadModelIsHonest(readModel) ||
        'Desktop presence read model counts and non-claims must match rows without claiming GPS, precise location, physical presence, or product-ready desktop tracking'
    )
  )
);

export type TrackingDesktopPresenceHintInputRow = Infer<typeof TrackingDesktopPresenceHintInputRowSchema>;
export type TrackingDesktopPresenceHintProofRow = Infer<typeof TrackingDesktopPresenceHintProofRowSchema>;
export type TrackingDesktopPresenceHintProofReadModel = Infer<typeof TrackingDesktopPresenceHintProofReadModelSchema>;

export type TrackingDesktopPresenceHintProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly deviceId: string;
  readonly childProfileId: string;
  readonly deviceLabel: string;
  readonly platform: 'windows' | 'linux' | 'macos';
  readonly sourceProofRefs: readonly string[];
};

export function buildTrackingDesktopPresenceHintProofReadModel(
  options: TrackingDesktopPresenceHintProofOptions,
  inputRows: readonly TrackingDesktopPresenceHintInputRow[]
): TrackingDesktopPresenceHintProofReadModel {
  const rows = inputRows.map((row) => desktopPresenceProofRowForInput(row));

  return TrackingDesktopPresenceHintProofReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    family: { familyId: options.familyId },
    device: {
      deviceId: options.deviceId,
      childProfileId: options.childProfileId,
      label: options.deviceLabel,
      platform: options.platform,
    },
    sourceProofRefs: options.sourceProofRefs,
    runtimeEvidenceRefs: rows.flatMap(runtimeEvidenceRefsForRow),
    rows,
    hintOnlyCount: rows.filter((row) => row.claimState === 'hint-only').length,
    manualCheckInCount: rows.filter((row) => row.claimState === 'manual-check-in').length,
    staleOfflineCount: rows.filter((row) => row.claimState === 'stale-offline').length,
    missingDeviceCount: rows.filter((row) => row.claimState === 'missing-device').length,
    manualRequiredCount: rows.filter((row) => row.claimState === 'manual-required').length,
    proofNonClaims: RequiredTrackingDesktopPresenceHintProofNonClaims,
    preciseLocationClaimed: false,
    gpsClaimed: false,
    physicalPresenceClaimed: false,
    lanPairingPhysicalProofClaimed: false,
    wifiPhysicalPresenceClaimed: false,
    ipPhysicalPresenceClaimed: false,
    osLocationRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    productReadyDesktopTrackingClaimed: false,
  });
}

function runtimeEvidenceRefsForRow(row: TrackingDesktopPresenceHintProofRow) {
  return row.evidenceRefs.map((evidenceReferenceId) => ({
    evidenceReferenceId,
    kind: evidenceKindFor(row),
    observedAt: row.observedAt,
  }));
}

function evidenceKindFor(row: TrackingDesktopPresenceHintProofRow) {
  if (row.claimState === 'manual-check-in' || row.claimState === 'manual-required') {
    return 'policy-decision';
  }
  if (row.claimState === 'stale-offline' || row.claimState === 'missing-device') {
    return 'query-store-summary';
  }
  return 'activity-event';
}

function desktopPresenceProofRowForInput(
  input: TrackingDesktopPresenceHintInputRow
): TrackingDesktopPresenceHintProofRow {
  return TrackingDesktopPresenceHintProofRowSchema.parse({
    ...input,
    claimState: claimStateFor(input),
    parentVisibleStatusToken: parentVisibleStatusTokenFor(input),
    manualRequiredReasonRefs: manualRequiredReasonRefsFor(input),
    preciseLocationClaimed: false,
    gpsClaimed: false,
    physicalPresenceClaimed: false,
    lanPairingPhysicalProofClaimed: false,
    osLocationRuntimeClaimed: false,
  });
}

function claimStateFor(input: TrackingDesktopPresenceHintInputRow) {
  if (input.caseKind === 'manual-check-in') {
    return 'manual-check-in';
  }
  if (input.caseKind === 'stale-offline-last-known') {
    return 'stale-offline';
  }
  if (input.caseKind === 'missing-device') {
    return 'missing-device';
  }
  if (input.caseKind === 'desktop-os-location-manual-required') {
    return 'manual-required';
  }
  return 'hint-only';
}

function parentVisibleStatusTokenFor(input: TrackingDesktopPresenceHintInputRow): string {
  if (input.caseKind === 'manual-check-in') {
    return 'tracking-desktop-presence-manual-check-in';
  }
  if (input.caseKind === 'stale-offline-last-known') {
    return 'tracking-desktop-presence-stale-offline-last-known';
  }
  if (input.caseKind === 'missing-device') {
    return 'tracking-desktop-presence-missing-device';
  }
  if (input.caseKind === 'desktop-os-location-manual-required') {
    return 'tracking-desktop-presence-os-location-manual-required';
  }
  return 'tracking-desktop-presence-hint-only';
}

function manualRequiredReasonRefsFor(input: TrackingDesktopPresenceHintInputRow): readonly string[] {
  if (input.caseKind !== 'desktop-os-location-manual-required') {
    return [];
  }
  return [
    'tracking-desktop-os-location-sample-not-proved',
    'tracking-desktop-physical-device-not-proved',
    'tracking-desktop-product-runtime-not-proved',
  ];
}

function trackingDesktopPresenceHintReadModelIsHonest(
  readModel: TrackingDesktopPresenceHintReadModelCandidate
): boolean {
  return readModelCountsAreHonest(readModel) && readModelNonClaimsAreHonest(readModel);
}

function trackingDesktopPresenceHintProofRowIsHonest(row: TrackingDesktopPresenceHintProofRowCandidate): boolean {
  return rowRefsArePresent(row) && rowStateMatches(row) && rowCaseEvidenceMatches(row) && rowRuntimeClaimsAreFalse(row);
}

function readModelCountsAreHonest(readModel: TrackingDesktopPresenceHintReadModelCandidate): boolean {
  const runtimeEvidenceRefCount = readModel.rows.reduce((total, row) => total + row.evidenceRefs.length, 0);

  return (
    readModel.hintOnlyCount === countRows(readModel.rows, (row) => row.claimState === 'hint-only') &&
    readModel.manualCheckInCount === countRows(readModel.rows, (row) => row.claimState === 'manual-check-in') &&
    readModel.staleOfflineCount === countRows(readModel.rows, (row) => row.claimState === 'stale-offline') &&
    readModel.missingDeviceCount === countRows(readModel.rows, (row) => row.claimState === 'missing-device') &&
    readModel.manualRequiredCount === countRows(readModel.rows, (row) => row.claimState === 'manual-required') &&
    readModel.runtimeEvidenceRefs.length === runtimeEvidenceRefCount
  );
}

function readModelNonClaimsAreHonest(readModel: TrackingDesktopPresenceHintReadModelCandidate): boolean {
  return (
    readModel.proofNonClaims.length === RequiredTrackingDesktopPresenceHintProofNonClaims.length &&
    RequiredTrackingDesktopPresenceHintProofNonClaims.every((nonClaim) =>
      readModel.proofNonClaims.includes(nonClaim)
    ) &&
    [
      readModel.preciseLocationClaimed,
      readModel.gpsClaimed,
      readModel.physicalPresenceClaimed,
      readModel.lanPairingPhysicalProofClaimed,
      readModel.wifiPhysicalPresenceClaimed,
      readModel.ipPhysicalPresenceClaimed,
      readModel.osLocationRuntimeClaimed,
      readModel.physicalDeviceProofClaimed,
      readModel.productReadyDesktopTrackingClaimed,
    ].every((claim) => claim === false)
  );
}

function rowRefsArePresent(row: TrackingDesktopPresenceHintProofRowCandidate): boolean {
  return row.evidenceRefs.length > 0 && row.auditRefs.length > 0;
}

function rowStateMatches(row: TrackingDesktopPresenceHintProofRowCandidate): boolean {
  return (
    row.claimState === claimStateFor(row) &&
    row.parentVisibleStatusToken === parentVisibleStatusTokenFor(row) &&
    row.manualRequiredReasonRefs.length === manualRequiredReasonRefsFor(row).length
  );
}

function rowCaseEvidenceMatches(row: TrackingDesktopPresenceHintProofRowCandidate): boolean {
  const manualCheckInMatches = row.caseKind !== 'manual-check-in' || row.manualCheckInRef !== null;
  const staleOfflineMatches =
    row.caseKind !== 'stale-offline-last-known' || (row.lastKnownEvidenceRef !== null && row.stale && row.offline);
  const missingDeviceMatches = row.caseKind !== 'missing-device' || row.missingDevice;
  return manualCheckInMatches && staleOfflineMatches && missingDeviceMatches;
}

function rowRuntimeClaimsAreFalse(row: TrackingDesktopPresenceHintProofRowCandidate): boolean {
  return [
    row.preciseLocationClaimed,
    row.gpsClaimed,
    row.physicalPresenceClaimed,
    row.lanPairingPhysicalProofClaimed,
    row.osLocationRuntimeClaimed,
  ].every((claim) => claim === false);
}

function countRows(
  rows: ReadonlyArray<TrackingDesktopPresenceHintProofRowCandidate>,
  predicate: (row: TrackingDesktopPresenceHintProofRowCandidate) => boolean
): number {
  return rows.filter(predicate).length;
}
