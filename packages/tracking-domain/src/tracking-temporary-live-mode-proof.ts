import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from './tracking-location-policy';
import {
  TrackingLiveTrackingGrantStateSchema,
  TrackingPolicyAuditRefSchema,
} from './tracking-location-policy-primitives';
import type { TrackingTemporaryLiveTrackingGrant } from './tracking-location-policy-types';
const TrackingTemporaryLiveDurationSeconds = Schema.Number.pipe(Schema.int(), Schema.positive());
const TrackingTemporaryLiveNonNegativeSeconds = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const RequiredTrackingTemporaryLiveModeNonClaims = [
  'no-live-location-runtime',
  'no-current-location-runtime',
  'no-background-location-runtime',
  'no-provider-delivery',
  'no-remote-relay-runtime',
  'no-parent-portal-live-map-runtime',
  'no-child-device-delivery',
  'no-physical-device-proof',
  'no-production-session-worker',
] as const;

export const TrackingTemporaryLiveModeNonClaimSchema = withParser(
  Schema.Literal(...RequiredTrackingTemporaryLiveModeNonClaims)
);

export const TrackingTemporaryLiveModeProofIdSchema = brandedNonEmptyStringSchema('TrackingTemporaryLiveModeProofId');
export const TrackingTemporaryLiveModeReferenceSchema = brandedNonEmptyStringSchema('TrackingTemporaryLiveModeReference');
export const TrackingTemporaryLiveModeSessionStateSchema = withParser(
  Schema.Literal(
    'active-authorized',
    'battery-degraded',
    'permission-degraded',
    'expired-auto-stopped',
    'retention-delete-ready',
    'manual-required'
  )
);
export const TrackingTemporaryLiveModeCadenceSchema = withParser(
  Schema.Literal('one-shot', 'on-change', 'interval', 'high-accuracy-burst')
);
export const TrackingTemporaryLiveModePermissionStateSchema = withParser(
  Schema.Literal('foreground-only', 'background', 'background-permission-required', 'permission-required')
);
export const TrackingTemporaryLiveModeBatteryStateSchema = withParser(
  Schema.Literal('normal', 'battery-throttled', 'low-power-mode')
);
export const TrackingTemporaryLiveModeDeliveryPathSchema = withParser(
  Schema.Literal('local-lan', 'parent-cache', 'parent-owned-storage', 'manual-required')
);

const TrackingTemporaryLiveModeContextSchema = withParser(
  Schema.Struct({
    grantId: TrackingTemporaryLiveModeReferenceSchema,
    requestedCadence: TrackingTemporaryLiveModeCadenceSchema,
    requestedCadenceSeconds: TrackingTemporaryLiveNonNegativeSeconds,
    maxDurationSeconds: TrackingTemporaryLiveDurationSeconds,
    permissionState: TrackingTemporaryLiveModePermissionStateSchema,
    batteryState: TrackingTemporaryLiveModeBatteryStateSchema,
    deliveryPath: TrackingTemporaryLiveModeDeliveryPathSchema,
    locationEvidenceRefs: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
    policyDecisionRefs: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
    retentionRefs: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
    manualProofRequirements: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
    autoStopReason: Schema.Union(TrackingTemporaryLiveModeReferenceSchema, Schema.Null),
  })
);

const TrackingTemporaryLiveModeRowBaseSchema = Schema.Struct({
  rowId: TrackingTemporaryLiveModeReferenceSchema,
  grantId: TrackingTemporaryLiveModeReferenceSchema,
  grantState: TrackingLiveTrackingGrantStateSchema,
  sessionState: TrackingTemporaryLiveModeSessionStateSchema,
  requestedAt: TrackingTemporaryLiveModeReferenceSchema,
  expiresAt: TrackingTemporaryLiveModeReferenceSchema,
  durationSeconds: TrackingTemporaryLiveDurationSeconds,
  requestedCadence: TrackingTemporaryLiveModeCadenceSchema,
  requestedCadenceSeconds: TrackingTemporaryLiveNonNegativeSeconds,
  maxDurationSeconds: TrackingTemporaryLiveDurationSeconds,
  permissionState: TrackingTemporaryLiveModePermissionStateSchema,
  batteryState: TrackingTemporaryLiveModeBatteryStateSchema,
  deliveryPath: TrackingTemporaryLiveModeDeliveryPathSchema,
  parentApproved: Schema.Boolean,
  childDisclosureRequired: Schema.Boolean,
  locationEvidenceRefs: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
  policyDecisionRefs: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  retentionRefs: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
  manualProofRequirements: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
  autoStopReason: Schema.Union(TrackingTemporaryLiveModeReferenceSchema, Schema.Null),
  liveLocationRuntimeClaimed: Schema.Literal(false),
  currentLocationRuntimeClaimed: Schema.Literal(false),
  backgroundLocationRuntimeClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  remoteRelayRuntimeClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
});

export const TrackingTemporaryLiveModeRowSchema = withParser(
  TrackingTemporaryLiveModeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        trackingTemporaryLiveModeRowIsHonest(row) ||
        'Expected temporary live tracking proof rows to require authorization, bounded duration, audit refs, degraded/manual states, and no runtime/device/provider overclaims'
    )
  )
);

const TrackingTemporaryLiveModeReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  proofId: TrackingTemporaryLiveModeProofIdSchema,
  generatedAt: TrackingTemporaryLiveModeReferenceSchema,
  sourceReadModelGeneratedAt: TrackingTemporaryLiveModeReferenceSchema,
  sourceContractRefs: Schema.Array(TrackingTemporaryLiveModeReferenceSchema),
  rows: Schema.Array(TrackingTemporaryLiveModeRowSchema),
  activeAuthorizedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  degradedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  autoStoppedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  retentionDeleteReadyCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  proofNonClaims: Schema.Array(TrackingTemporaryLiveModeNonClaimSchema),
  liveLocationRuntimeClaimed: Schema.Literal(false),
  currentLocationRuntimeClaimed: Schema.Literal(false),
  backgroundLocationRuntimeClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  remoteRelayRuntimeClaimed: Schema.Literal(false),
  parentPortalLiveMapRuntimeClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  productionSessionWorkerClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingTemporaryLiveModeReadModelSchema = withParser(
  TrackingTemporaryLiveModeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingTemporaryLiveModeReadModelIsHonest(readModel) ||
        'Expected temporary live tracking proof read model to include all required session states, matching counts, non-claims, and no product-ready runtime claims'
    )
  )
);

export type TrackingTemporaryLiveModeContext = Infer<typeof TrackingTemporaryLiveModeContextSchema>;
export type TrackingTemporaryLiveModeRow = Infer<typeof TrackingTemporaryLiveModeRowSchema>;
export type TrackingTemporaryLiveModeReadModel = Infer<typeof TrackingTemporaryLiveModeReadModelSchema>;

export type TrackingTemporaryLiveModeProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceContractRefs: readonly string[];
  readonly contexts: readonly unknown[];
};

type TrackingTemporaryLiveModeRowInput = Infer<typeof TrackingTemporaryLiveModeRowBaseSchema>;
type TrackingTemporaryLiveModeReadModelInput = Infer<typeof TrackingTemporaryLiveModeReadModelBaseSchema>;
type TrackingTemporaryLiveModeSessionState = Infer<typeof TrackingTemporaryLiveModeSessionStateSchema>;

export function buildTrackingTemporaryLiveModeReadModel(
  options: TrackingTemporaryLiveModeProofOptions,
  sourceReadModel: unknown
): TrackingTemporaryLiveModeReadModel {
  const parsedSource = TrackingLocationPolicyReadModelSchema.parse(sourceReadModel);
  const contexts = options.contexts.map((context) => TrackingTemporaryLiveModeContextSchema.parse(context));
  const rows = parsedSource.temporaryLiveGrants.map((grant) =>
    trackingTemporaryLiveModeRowForGrant(grant, contextForGrant(grant, contexts))
  );

  return TrackingTemporaryLiveModeReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    sourceReadModelGeneratedAt: parsedSource.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    activeAuthorizedCount: countRows(rows, ['active-authorized']),
    degradedCount: countRows(rows, ['battery-degraded', 'permission-degraded']),
    autoStoppedCount: countRows(rows, ['expired-auto-stopped']),
    retentionDeleteReadyCount: countRows(rows, ['retention-delete-ready']),
    manualRequiredCount: countRows(rows, ['manual-required']),
    proofNonClaims: RequiredTrackingTemporaryLiveModeNonClaims,
    liveLocationRuntimeClaimed: false,
    currentLocationRuntimeClaimed: false,
    backgroundLocationRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    remoteRelayRuntimeClaimed: false,
    parentPortalLiveMapRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    physicalDeviceProofClaimed: false,
    productionSessionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function trackingTemporaryLiveModeRowForGrant(
  grant: TrackingTemporaryLiveTrackingGrant,
  context: TrackingTemporaryLiveModeContext
): TrackingTemporaryLiveModeRow {
  const sessionState = sessionStateFor(grant, context);

  return TrackingTemporaryLiveModeRowSchema.parse({
    rowId: `tracking-temporary-live-${grant.grantId}`,
    grantId: grant.grantId,
    grantState: grant.state,
    sessionState,
    requestedAt: grant.requestedAt,
    expiresAt: grant.expiresAt,
    durationSeconds: grant.durationSeconds,
    requestedCadence: context.requestedCadence,
    requestedCadenceSeconds: context.requestedCadenceSeconds,
    maxDurationSeconds: context.maxDurationSeconds,
    permissionState: context.permissionState,
    batteryState: context.batteryState,
    deliveryPath: context.deliveryPath,
    parentApproved: grant.parentApproved,
    childDisclosureRequired: grant.childDisclosureRequired,
    locationEvidenceRefs: context.locationEvidenceRefs,
    policyDecisionRefs: context.policyDecisionRefs,
    auditRefs: grant.auditRefs,
    retentionRefs: context.retentionRefs,
    manualProofRequirements: context.manualProofRequirements,
    autoStopReason: context.autoStopReason,
    liveLocationRuntimeClaimed: false,
    currentLocationRuntimeClaimed: false,
    backgroundLocationRuntimeClaimed: false,
    providerDeliveryClaimed: false,
    remoteRelayRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
  });
}

function sessionStateFor(
  grant: TrackingTemporaryLiveTrackingGrant,
  context: TrackingTemporaryLiveModeContext
): TrackingTemporaryLiveModeSessionState {
  if (grant.state === 'expired') {
    return context.retentionRefs.length > 0 ? 'retention-delete-ready' : 'expired-auto-stopped';
  }
  if (grant.state === 'unavailable' || grant.state === 'denied') {
    return 'manual-required';
  }
  if (
    context.permissionState === 'permission-required' ||
    context.permissionState === 'background-permission-required'
  ) {
    return 'permission-degraded';
  }
  if (context.batteryState !== 'normal') {
    return 'battery-degraded';
  }
  return 'active-authorized';
}

function contextForGrant(
  grant: TrackingTemporaryLiveTrackingGrant,
  contexts: readonly TrackingTemporaryLiveModeContext[]
): TrackingTemporaryLiveModeContext {
  const context = contexts.find((candidate) => String(candidate.grantId) === String(grant.grantId));
  if (context === undefined) {
    throw new Error(`Missing temporary live proof context for grant ${grant.grantId}`);
  }
  return context;
}

function trackingTemporaryLiveModeRowIsHonest(row: TrackingTemporaryLiveModeRowInput) {
  return (
    row.durationSeconds <= row.maxDurationSeconds &&
    row.auditRefs.length > 0 &&
    row.policyDecisionRefs.length > 0 &&
    row.locationEvidenceRefs.length > 0 &&
    rowHasNoRuntimeOrProviderClaims(row) &&
    activeRowHasAuthorization(row) &&
    degradedRowsHaveProof(row) &&
    stoppedRowsHaveStopReason(row)
  );
}

function rowHasNoRuntimeOrProviderClaims(row: TrackingTemporaryLiveModeRowInput) {
  return (
    row.liveLocationRuntimeClaimed === false &&
    row.currentLocationRuntimeClaimed === false &&
    row.backgroundLocationRuntimeClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.remoteRelayRuntimeClaimed === false &&
    row.physicalDeviceProofClaimed === false
  );
}

function activeRowHasAuthorization(row: TrackingTemporaryLiveModeRowInput) {
  return (
    row.sessionState !== 'active-authorized' ||
    (row.grantState === 'active' && row.parentApproved && row.childDisclosureRequired)
  );
}

function degradedRowsHaveProof(row: TrackingTemporaryLiveModeRowInput) {
  return (
    (row.sessionState !== 'battery-degraded' && row.sessionState !== 'permission-degraded') ||
    row.manualProofRequirements.length > 0
  );
}

function stoppedRowsHaveStopReason(row: TrackingTemporaryLiveModeRowInput) {
  return (
    (row.sessionState !== 'expired-auto-stopped' && row.sessionState !== 'retention-delete-ready') ||
    row.autoStopReason !== null
  );
}

function trackingTemporaryLiveModeReadModelIsHonest(readModel: TrackingTemporaryLiveModeReadModelInput) {
  const states = readModel.rows.map((row) => row.sessionState);
  return (
    RequiredTrackingTemporaryLiveModeNonClaims.every((nonClaim) => readModel.proofNonClaims.includes(nonClaim)) &&
    readModel.activeAuthorizedCount === countRows(readModel.rows, ['active-authorized']) &&
    readModel.degradedCount === countRows(readModel.rows, ['battery-degraded', 'permission-degraded']) &&
    readModel.autoStoppedCount === countRows(readModel.rows, ['expired-auto-stopped']) &&
    readModel.retentionDeleteReadyCount === countRows(readModel.rows, ['retention-delete-ready']) &&
    readModel.manualRequiredCount === countRows(readModel.rows, ['manual-required']) &&
    states.includes('active-authorized') &&
    states.includes('battery-degraded') &&
    states.includes('permission-degraded') &&
    states.includes('expired-auto-stopped') &&
    states.includes('retention-delete-ready') &&
    readModel.productClaimReady === false
  );
}

function countRows(
  rows: ReadonlyArray<{ readonly sessionState: TrackingTemporaryLiveModeSessionState }>,
  states: readonly TrackingTemporaryLiveModeSessionState[]
) {
  return rows.filter((row) => states.includes(row.sessionState)).length;
}

