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
import {
  TrackingLocationPolicyReadModelSchema,
  type TrackingLocationPolicyReadModel,
  type TrackingMissingDeviceCase,
} from './tracking-location-policy';
const TrackingMissingDeviceNonNegativeInteger = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const RequiredTrackingMissingDeviceModeProofNonClaims = [
  'no-current-location-for-stale-or-offline-device',
  'no-live-tracking-runtime',
  'no-powered-off-device-tracking',
  'no-remote-sync-runtime',
  'no-provider-delivery',
  'no-physical-device-proof',
  'no-portal-runtime-ui',
  'no-os-lost-mode-api',
] as const;

export const TrackingMissingDeviceModeProofNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingMissingDeviceModeProofNonClaims)
);

export const TrackingMissingDeviceModeProofIdSchema = brandedNonEmptyStringSchema('TrackingMissingDeviceModeProofId');
export const TrackingMissingDeviceModeProofReferenceSchema = brandedNonEmptyStringSchema('TrackingMissingDeviceModeProofReference');

export const TrackingMissingDeviceContactStateSchema = withParser(
  Schema.Literal('online', 'offline', 'powered-off', 'battery-throttled', 'unknown')
);
export const TrackingMissingDeviceUiBadgeSchema = withParser(
  Schema.Literal('last-known', 'offline', 'battery-throttled', 'manual-required', 'contact-requested')
);
export const TrackingMissingDeviceActionKindSchema = withParser(
  Schema.Literal('review-last-known', 'ask-child-check-in', 'call-child', 'mark-found', 'manual-platform-proof')
);

const TrackingMissingDeviceStatusSnapshotBaseSchema = Schema.Struct({
  statusSnapshotId: TrackingMissingDeviceModeProofReferenceSchema,
  contactState: TrackingMissingDeviceContactStateSchema,
  lastContactEvidenceRef: TrackingMissingDeviceModeProofReferenceSchema,
  batteryEvidenceRef: TrackingMissingDeviceModeProofReferenceSchema,
  connectivityEvidenceRef: TrackingMissingDeviceModeProofReferenceSchema,
  pendingUploadEvidenceRef: TrackingMissingDeviceModeProofReferenceSchema,
  lastContactAt: ParentTimestampSchema,
  batteryPercent: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.between(0, 100)), Schema.Null),
  pendingUploadCount: TrackingMissingDeviceNonNegativeInteger,
  degraded: Schema.Boolean,
});

export const TrackingMissingDeviceStatusSnapshotSchema = withParser(TrackingMissingDeviceStatusSnapshotBaseSchema);

const TrackingMissingDeviceUiStateBaseSchema = Schema.Struct({
  uiStateId: TrackingMissingDeviceModeProofReferenceSchema,
  primaryBadge: TrackingMissingDeviceUiBadgeSchema,
  secondaryBadges: Schema.Array(TrackingMissingDeviceUiBadgeSchema),
  headlineToken: TrackingMissingDeviceModeProofReferenceSchema,
  detailToken: TrackingMissingDeviceModeProofReferenceSchema,
  evidenceDrawerRefs: Schema.Array(TrackingMissingDeviceModeProofReferenceSchema),
  actionKinds: Schema.Array(TrackingMissingDeviceActionKindSchema),
  accessibilityStateToken: TrackingMissingDeviceModeProofReferenceSchema,
  currentLocationCopyAllowed: Schema.Literal(false),
});

export const TrackingMissingDeviceUiStateSchema = withParser(TrackingMissingDeviceUiStateBaseSchema);

const TrackingMissingDeviceModeProofRowBaseSchema = Schema.Struct({
  rowId: TrackingMissingDeviceModeProofReferenceSchema,
  caseId: TrackingMissingDeviceModeProofReferenceSchema,
  state: TrackingMissingDeviceModeProofReferenceSchema,
  lastKnownEvidenceRef: Schema.Union(TrackingMissingDeviceModeProofReferenceSchema, Schema.Null),
  deviceStatusEvidenceRef: Schema.Union(TrackingMissingDeviceModeProofReferenceSchema, Schema.Null),
  reasonCodeRefs: Schema.Array(TrackingMissingDeviceModeProofReferenceSchema),
  contactActionRefs: Schema.Array(TrackingMissingDeviceModeProofReferenceSchema),
  statusSnapshot: TrackingMissingDeviceStatusSnapshotSchema,
  uiState: TrackingMissingDeviceUiStateSchema,
  currentLocationClaimed: Schema.Literal(false),
  poweredOffTrackingClaimed: Schema.Literal(false),
  remoteSyncRequired: Schema.Literal(false),
});

export const TrackingMissingDeviceModeProofRowSchema = withParser(
  TrackingMissingDeviceModeProofRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingMissingDeviceModeProofRowIsHonest(row) ||
        'Missing-device proof rows need last-known/status evidence, contact actions, degraded state when offline, and no current-location/powered-off/remote-sync claims'
    )
  )
);

const TrackingMissingDeviceModeProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingMissingDeviceModeProofIdSchema,
  generatedAt: ParentTimestampSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  sourceTrackingReadModelRef: TrackingMissingDeviceModeProofReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingMissingDeviceModeProofReferenceSchema),
  runtimeEvidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  rows: Schema.Array(TrackingMissingDeviceModeProofRowSchema),
  lastKnownOnlyCount: TrackingMissingDeviceNonNegativeInteger,
  offlineCount: TrackingMissingDeviceNonNegativeInteger,
  contactRequestedCount: TrackingMissingDeviceNonNegativeInteger,
  manualRequiredCount: TrackingMissingDeviceNonNegativeInteger,
  proofNonClaims: Schema.Array(TrackingMissingDeviceModeProofNonClaimSchema),
  currentLocationRuntimeClaimed: Schema.Literal(false),
  liveTrackingRuntimeClaimed: Schema.Literal(false),
  poweredOffDeviceTrackingClaimed: Schema.Literal(false),
  remoteSyncRuntimeClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  portalRuntimeUiClaimed: Schema.Literal(false),
  osLostModeApiClaimed: Schema.Literal(false),
});

export const TrackingMissingDeviceModeProofReadModelSchema = withParser(
  TrackingMissingDeviceModeProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingMissingDeviceModeProofReadModelIsHonest(readModel) ||
        'Missing-device proof read model counts and non-claims must match rows and preserve runtime evidence'
    )
  )
);

export type TrackingMissingDeviceContactState = Infer<typeof TrackingMissingDeviceContactStateSchema>;
export type TrackingMissingDeviceModeProofRow = Infer<typeof TrackingMissingDeviceModeProofRowSchema>;
export type TrackingMissingDeviceModeProofReadModel = Infer<typeof TrackingMissingDeviceModeProofReadModelSchema>;

export type TrackingMissingDeviceModeProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly familyId: string;
  readonly deviceId: string;
  readonly childProfileId: string;
  readonly deviceLabel: string;
  readonly platform: 'windows' | 'linux' | 'macos' | 'android' | 'ios';
  readonly sourceTrackingReadModelRef: string;
  readonly sourceContractRefs: readonly string[];
};

type TrackingMissingDeviceStatusSnapshotInput = Infer<typeof TrackingMissingDeviceStatusSnapshotBaseSchema>;
type TrackingMissingDeviceUiStateInput = Infer<typeof TrackingMissingDeviceUiStateBaseSchema>;
type TrackingMissingDeviceModeProofRowInput = Infer<typeof TrackingMissingDeviceModeProofRowBaseSchema>;
type TrackingMissingDeviceModeProofReadModelInput = Infer<typeof TrackingMissingDeviceModeProofReadModelBaseSchema>;

export function buildTrackingMissingDeviceModeProofReadModel(
  options: TrackingMissingDeviceModeProofOptions,
  sourceReadModel: TrackingLocationPolicyReadModel
): TrackingMissingDeviceModeProofReadModel {
  const parsedSource = TrackingLocationPolicyReadModelSchema.parse(sourceReadModel);
  const rows = parsedSource.missingDeviceCases.map((missingCase) => missingDeviceProofRowForCase(missingCase));

  return TrackingMissingDeviceModeProofReadModelSchema.parse({
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
    sourceTrackingReadModelRef: options.sourceTrackingReadModelRef,
    sourceContractRefs: options.sourceContractRefs,
    runtimeEvidenceRefs: rows.flatMap(runtimeEvidenceRefsForRow),
    rows,
    lastKnownOnlyCount: countRows(rows, 'last-known-only'),
    offlineCount: countRows(rows, 'offline'),
    contactRequestedCount: countRows(rows, 'contact-requested'),
    manualRequiredCount: countRows(rows, 'manual-required'),
    proofNonClaims: RequiredTrackingMissingDeviceModeProofNonClaims,
    currentLocationRuntimeClaimed: false,
    liveTrackingRuntimeClaimed: false,
    poweredOffDeviceTrackingClaimed: false,
    remoteSyncRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    physicalDeviceProofClaimed: false,
    portalRuntimeUiClaimed: false,
    osLostModeApiClaimed: false,
  });
}

function missingDeviceProofRowForCase(missingCase: TrackingMissingDeviceCase): TrackingMissingDeviceModeProofRow {
  const lastKnownEvidenceRef = missingCase.lastKnownEvidence?.evidenceReferenceId ?? null;
  const deviceStatusEvidenceRef = missingCase.deviceStatusEvidence?.evidenceReferenceId ?? null;
  const statusSnapshot = statusSnapshotForCase(missingCase);

  return TrackingMissingDeviceModeProofRowSchema.parse({
    rowId: `tracking-missing-device-${missingCase.caseId}`,
    caseId: missingCase.caseId,
    state: missingCase.state,
    lastKnownEvidenceRef,
    deviceStatusEvidenceRef,
    reasonCodeRefs: missingCase.reasonCodes,
    contactActionRefs: missingCase.contactActionRefs,
    statusSnapshot,
    uiState: uiStateForCase(missingCase, statusSnapshot, lastKnownEvidenceRef, deviceStatusEvidenceRef),
    currentLocationClaimed: false,
    poweredOffTrackingClaimed: false,
    remoteSyncRequired: false,
  });
}

function statusSnapshotForCase(missingCase: TrackingMissingDeviceCase) {
  const caseId = missingCase.caseId;
  const statusRef = missingCase.deviceStatusEvidence?.evidenceReferenceId ?? `tracking-device-status-missing-${caseId}`;
  const observedAt = missingCase.deviceStatusEvidence?.observedAt ?? missingCase.openedAt;
  const state = missingCase.state;

  return TrackingMissingDeviceStatusSnapshotSchema.parse({
    statusSnapshotId: `tracking-missing-device-status-${caseId}`,
    contactState: contactStateFor(state),
    lastContactEvidenceRef: `tracking-device-last-contact-${caseId}`,
    batteryEvidenceRef: `tracking-device-battery-${caseId}`,
    connectivityEvidenceRef: statusRef,
    pendingUploadEvidenceRef: `tracking-device-pending-upload-${caseId}`,
    lastContactAt: observedAt,
    batteryPercent: state === 'offline' ? 9 : state === 'manual-required' ? null : 38,
    pendingUploadCount: state === 'contact-requested' || state === 'offline' ? 1 : 0,
    degraded: state !== 'open',
  });
}

function uiStateForCase(
  missingCase: TrackingMissingDeviceCase,
  statusSnapshot: TrackingMissingDeviceStatusSnapshotInput,
  lastKnownEvidenceRef: string | null,
  deviceStatusEvidenceRef: string | null
): TrackingMissingDeviceUiStateInput {
  return TrackingMissingDeviceUiStateSchema.parse({
    uiStateId: `tracking-missing-device-ui-${missingCase.caseId}`,
    primaryBadge: primaryBadgeFor(missingCase.state),
    secondaryBadges: secondaryBadgesFor(missingCase.state),
    headlineToken: headlineTokenFor(missingCase.state),
    detailToken: detailTokenFor(missingCase.state),
    evidenceDrawerRefs: [
      lastKnownEvidenceRef ?? `tracking-last-known-unavailable-${missingCase.caseId}`,
      deviceStatusEvidenceRef ?? statusSnapshot.statusSnapshotId,
      statusSnapshot.batteryEvidenceRef,
      statusSnapshot.connectivityEvidenceRef,
      statusSnapshot.pendingUploadEvidenceRef,
    ],
    actionKinds: actionKindsFor(missingCase.state),
    accessibilityStateToken: `tracking-missing-device-a11y-${missingCase.state}`,
    currentLocationCopyAllowed: false,
  });
}

function contactStateFor(state: TrackingMissingDeviceCase['state']): TrackingMissingDeviceContactState {
  if (state === 'offline') {
    return 'powered-off';
  }
  if (state === 'last-known-only') {
    return 'offline';
  }
  if (state === 'manual-required') {
    return 'unknown';
  }
  return 'online';
}

function primaryBadgeFor(state: TrackingMissingDeviceCase['state']): TrackingMissingDeviceUiStateInput['primaryBadge'] {
  if (state === 'offline') {
    return 'offline';
  }
  if (state === 'manual-required') {
    return 'manual-required';
  }
  if (state === 'contact-requested') {
    return 'contact-requested';
  }
  return 'last-known';
}

function secondaryBadgesFor(
  state: TrackingMissingDeviceCase['state']
): TrackingMissingDeviceUiStateInput['secondaryBadges'] {
  if (state === 'offline') {
    return ['last-known', 'battery-throttled'];
  }
  if (state === 'manual-required') {
    return ['last-known'];
  }
  if (state === 'contact-requested') {
    return ['last-known', 'offline'];
  }
  return ['offline'];
}

function headlineTokenFor(state: TrackingMissingDeviceCase['state']): string {
  if (state === 'offline') {
    return 'tracking-missing-device-headline-powered-off-last-known';
  }
  if (state === 'manual-required') {
    return 'tracking-missing-device-headline-platform-manual-required';
  }
  if (state === 'contact-requested') {
    return 'tracking-missing-device-headline-contact-requested';
  }
  return 'tracking-missing-device-headline-last-known-only';
}

function detailTokenFor(state: TrackingMissingDeviceCase['state']): string {
  if (state === 'offline') {
    return 'tracking-missing-device-detail-last-known-contact-battery-network';
  }
  if (state === 'manual-required') {
    return 'tracking-missing-device-detail-os-lost-mode-proof-required';
  }
  if (state === 'contact-requested') {
    return 'tracking-missing-device-detail-parent-action-queued';
  }
  return 'tracking-missing-device-detail-no-current-location-claim';
}

function actionKindsFor(state: TrackingMissingDeviceCase['state']): TrackingMissingDeviceUiStateInput['actionKinds'] {
  if (state === 'manual-required') {
    return ['review-last-known', 'manual-platform-proof'];
  }
  if (state === 'contact-requested') {
    return ['review-last-known', 'call-child', 'mark-found'];
  }
  return ['review-last-known', 'ask-child-check-in', 'call-child', 'mark-found'];
}

function trackingMissingDeviceModeProofRowIsHonest(row: TrackingMissingDeviceModeProofRowInput): boolean {
  return (
    trackingMissingDeviceModeRowHasEvidence(row) &&
    trackingMissingDeviceModeRowHasUiState(row) &&
    trackingMissingDeviceModeRowKeepsRuntimeUnclaimed(row) &&
    trackingMissingDeviceOfflineRowIsDegraded(row)
  );
}

function trackingMissingDeviceModeRowHasEvidence(row: TrackingMissingDeviceModeProofRowInput): boolean {
  return (
    row.lastKnownEvidenceRef !== null &&
    row.deviceStatusEvidenceRef !== null &&
    row.reasonCodeRefs.length > 0 &&
    row.contactActionRefs.length > 0
  );
}

function trackingMissingDeviceModeRowHasUiState(row: TrackingMissingDeviceModeProofRowInput): boolean {
  return (
    row.uiState.evidenceDrawerRefs.length >= 2 &&
    row.uiState.actionKinds.length >= 2 &&
    row.uiState.currentLocationCopyAllowed === false
  );
}

function trackingMissingDeviceModeRowKeepsRuntimeUnclaimed(row: TrackingMissingDeviceModeProofRowInput): boolean {
  return (
    row.currentLocationClaimed === false && row.poweredOffTrackingClaimed === false && row.remoteSyncRequired === false
  );
}

function trackingMissingDeviceOfflineRowIsDegraded(row: TrackingMissingDeviceModeProofRowInput): boolean {
  return row.state !== 'offline' || (row.statusSnapshot.degraded && row.statusSnapshot.contactState === 'powered-off');
}

function trackingMissingDeviceModeProofReadModelIsHonest(
  readModel: TrackingMissingDeviceModeProofReadModelInput
): boolean {
  return (
    readModel.rows.length > 0 &&
    readModel.runtimeEvidenceRefs.length >= readModel.rows.length * 4 &&
    readModel.lastKnownOnlyCount === countRows(readModel.rows, 'last-known-only') &&
    readModel.offlineCount === countRows(readModel.rows, 'offline') &&
    readModel.contactRequestedCount === countRows(readModel.rows, 'contact-requested') &&
    readModel.manualRequiredCount === countRows(readModel.rows, 'manual-required') &&
    RequiredTrackingMissingDeviceModeProofNonClaims.every((claim) => readModel.proofNonClaims.includes(claim))
  );
}

function runtimeEvidenceRefsForRow(row: TrackingMissingDeviceModeProofRowInput) {
  if (row.lastKnownEvidenceRef === null || row.deviceStatusEvidenceRef === null) {
    return [];
  }

  return [
    {
      evidenceReferenceId: row.lastKnownEvidenceRef,
      kind: 'journal-event',
      observedAt: row.statusSnapshot.lastContactAt,
    },
    {
      evidenceReferenceId: row.deviceStatusEvidenceRef,
      kind: 'query-store-summary',
      observedAt: row.statusSnapshot.lastContactAt,
    },
    {
      evidenceReferenceId: row.statusSnapshot.batteryEvidenceRef,
      kind: 'activity-event',
      observedAt: row.statusSnapshot.lastContactAt,
    },
    {
      evidenceReferenceId: row.statusSnapshot.pendingUploadEvidenceRef,
      kind: 'query-store-summary',
      observedAt: row.statusSnapshot.lastContactAt,
    },
  ];
}

const countRows = (
  rows: ReadonlyArray<{ readonly state: string }>,
  state: TrackingMissingDeviceCase['state']
): number => rows.filter((row) => row.state === state).length;

export const decodeTrackingMissingDeviceModeProofReadModel = Schema.decodeUnknownSync(
  TrackingMissingDeviceModeProofReadModelSchema
);

