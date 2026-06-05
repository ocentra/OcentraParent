import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { TrackingLocationPolicyReadModelSchema, TrackingPolicySchemaVersion } from './tracking-location-policy';
import {
  TrackingLiveTrackingGrantIdSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
} from './tracking-location-policy-primitives';
import type {
  TrackingLocationPolicyReadModel,
  TrackingPlatformProofRoute,
  TrackingTemporaryLiveTrackingGrant,
} from './tracking-location-policy-types';

const TrackingTemporaryLiveTextSchema = Schema.String.pipe(Schema.minLength(1));
const TrackingTemporaryLiveProofIdSchema = TrackingTemporaryLiveTextSchema.pipe(
  Schema.brand('TrackingTemporaryLiveProofId')
);
const TrackingTemporaryLiveSourceRefSchema = TrackingTemporaryLiveTextSchema.pipe(
  Schema.brand('TrackingTemporaryLiveSourceRef')
);
const TrackingTemporaryLiveCadenceSecondsSchema = Schema.Number.pipe(Schema.int(), Schema.positive());
const TrackingTemporaryLiveDurationSecondsSchema = Schema.Number.pipe(Schema.int(), Schema.positive());

export const TrackingTemporaryLiveReadinessStateSchema = withParser(
  Schema.Literal(
    'ready-to-start',
    'waiting-for-parent-authorization',
    'child-disclosure-required',
    'active-time-boxed',
    'expired-auto-stop-required',
    'revoked-by-parent',
    'denied-by-parent',
    'platform-unavailable-manual-required',
    'duration-policy-manual-required'
  )
);

export const TrackingTemporaryLiveAutoStopReasonSchema = withParser(
  Schema.Literal('expires-at', 'expired', 'parent-revoked', 'parent-denied', 'platform-unavailable', 'manual-required')
);

export const TrackingTemporaryLiveRetentionActionSchema = withParser(
  Schema.Literal('retain-until-expiry', 'delete-after-retention-window', 'manual-review-required')
);

export const TrackingTemporaryLivePlatformRuntimeProofStateSchema = withParser(
  Schema.Literal('manual-required', 'contract-proved', 'real-device-required', 'platform-unavailable')
);

export const RequiredTrackingTemporaryLiveReadinessNonClaims = {
  liveLocationRuntimeClaimed: false,
  physicalDeviceProofClaimed: false,
  backgroundLocationClaimed: false,
  batteryRuntimeClaimed: false,
  childDisclosureUiClaimed: false,
  parentLiveUiClaimed: false,
  remoteSyncClaimed: false,
  providerDeliveryClaimed: false,
  productClaimReady: false,
} as const;

export const TrackingTemporaryLiveReadinessRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    grantId: TrackingLiveTrackingGrantIdSchema,
    readinessState: TrackingTemporaryLiveReadinessStateSchema,
    requestedAt: TrackingTemporaryLiveTextSchema,
    expiresAt: TrackingTemporaryLiveTextSchema,
    durationSeconds: TrackingTemporaryLiveDurationSecondsSchema,
    maximumDurationSeconds: TrackingTemporaryLiveDurationSecondsSchema,
    cadenceSeconds: TrackingTemporaryLiveCadenceSecondsSchema,
    parentAuthorized: Schema.Boolean,
    childDisclosureRequired: Schema.Boolean,
    autoStopReason: TrackingTemporaryLiveAutoStopReasonSchema,
    retentionAction: TrackingTemporaryLiveRetentionActionSchema,
    platformRuntimeProofState: TrackingTemporaryLivePlatformRuntimeProofStateSchema,
    batteryStatusProofState: TrackingTemporaryLivePlatformRuntimeProofStateSchema,
    manualProofRequirements: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    sourceContractRefs: Schema.Array(TrackingTemporaryLiveSourceRefSchema),
    liveLocationRuntimeClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    backgroundLocationClaimed: Schema.Literal(false),
    batteryRuntimeClaimed: Schema.Literal(false),
    childDisclosureUiClaimed: Schema.Literal(false),
    parentLiveUiClaimed: Schema.Literal(false),
    remoteSyncClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (row) =>
        row.readinessState !== 'active-time-boxed' ||
        (row.parentAuthorized &&
          row.childDisclosureRequired &&
          row.autoStopReason === 'expires-at' &&
          row.retentionAction === 'retain-until-expiry') ||
        'Active temporary live tracking readiness rows require parent authorization, child disclosure, expiry, and retention boundaries'
    )
  )
);

export const TrackingTemporaryLiveReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    generatedAt: TrackingTemporaryLiveTextSchema,
    proofId: TrackingTemporaryLiveProofIdSchema,
    sourceTrackingReadModelRef: TrackingTemporaryLiveSourceRefSchema,
    sourceContractRefs: Schema.Array(TrackingTemporaryLiveSourceRefSchema),
    rows: Schema.Array(TrackingTemporaryLiveReadinessRowSchema),
    readyToStartCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    activeCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    expiredAutoStopCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    manualRequiredCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    revokedOrDeniedCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    liveLocationRuntimeClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    backgroundLocationClaimed: Schema.Literal(false),
    batteryRuntimeClaimed: Schema.Literal(false),
    childDisclosureUiClaimed: Schema.Literal(false),
    parentLiveUiClaimed: Schema.Literal(false),
    remoteSyncClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
);

export type TrackingTemporaryLiveReadinessRow = Infer<typeof TrackingTemporaryLiveReadinessRowSchema>;
export type TrackingTemporaryLiveReadinessReadModel = Infer<typeof TrackingTemporaryLiveReadinessReadModelSchema>;

const MaximumDurationSeconds = 3600;
const DefaultCadenceSeconds = 60;

export function buildTrackingTemporaryLiveReadinessReadModel(
  metadata: {
    generatedAt: string;
    proofId: string;
    sourceTrackingReadModelRef: string;
    sourceContractRefs: ReadonlyArray<string>;
  },
  sourceReadModel: TrackingLocationPolicyReadModel
) {
  const parsedSourceReadModel = TrackingLocationPolicyReadModelSchema.parse(sourceReadModel);
  const generatedAtMs = Date.parse(metadata.generatedAt);
  const platformProof = summarizePlatformProof(parsedSourceReadModel.platformProofRoutes);
  const rows = parsedSourceReadModel.temporaryLiveGrants.map((grant) =>
    buildTemporaryLiveReadinessRow({
      grant,
      generatedAtMs,
      metadata,
      platformProof,
    })
  );

  return TrackingTemporaryLiveReadinessReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: metadata.generatedAt,
    proofId: metadata.proofId,
    sourceTrackingReadModelRef: metadata.sourceTrackingReadModelRef,
    sourceContractRefs: metadata.sourceContractRefs,
    rows,
    readyToStartCount: rows.filter((row) => row.readinessState === 'ready-to-start').length,
    activeCount: rows.filter((row) => row.readinessState === 'active-time-boxed').length,
    expiredAutoStopCount: rows.filter((row) => row.readinessState === 'expired-auto-stop-required').length,
    manualRequiredCount: rows.filter(
      (row) =>
        row.readinessState === 'platform-unavailable-manual-required' ||
        row.readinessState === 'duration-policy-manual-required' ||
        row.manualProofRequirements.length > 0
    ).length,
    revokedOrDeniedCount: rows.filter(
      (row) => row.readinessState === 'revoked-by-parent' || row.readinessState === 'denied-by-parent'
    ).length,
    ...RequiredTrackingTemporaryLiveReadinessNonClaims,
  });
}

function buildTemporaryLiveReadinessRow(input: {
  grant: TrackingTemporaryLiveTrackingGrant;
  generatedAtMs: number;
  metadata: {
    sourceContractRefs: ReadonlyArray<string>;
  };
  platformProof: {
    location: TrackingTemporaryLiveReadinessRow['platformRuntimeProofState'];
    battery: TrackingTemporaryLiveReadinessRow['batteryStatusProofState'];
  };
}) {
  const readinessState = resolveReadinessState(input.grant, input.generatedAtMs, input.platformProof.location);
  const manualProofRequirements = resolveManualProofRequirements(input.grant, input.platformProof);

  return TrackingTemporaryLiveReadinessRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    grantId: input.grant.grantId,
    readinessState,
    requestedAt: input.grant.requestedAt,
    expiresAt: input.grant.expiresAt,
    durationSeconds: input.grant.durationSeconds,
    maximumDurationSeconds: MaximumDurationSeconds,
    cadenceSeconds: DefaultCadenceSeconds,
    parentAuthorized: input.grant.parentApproved,
    childDisclosureRequired: input.grant.childDisclosureRequired,
    autoStopReason: resolveAutoStopReason(input.grant, readinessState),
    retentionAction: resolveRetentionAction(readinessState),
    platformRuntimeProofState: input.platformProof.location,
    batteryStatusProofState: input.platformProof.battery,
    manualProofRequirements,
    auditRefs: input.grant.auditRefs,
    sourceContractRefs: input.metadata.sourceContractRefs,
    liveLocationRuntimeClaimed: false,
    physicalDeviceProofClaimed: false,
    backgroundLocationClaimed: false,
    batteryRuntimeClaimed: false,
    childDisclosureUiClaimed: false,
    parentLiveUiClaimed: false,
    remoteSyncClaimed: false,
    providerDeliveryClaimed: false,
  });
}

function resolveReadinessState(
  grant: TrackingTemporaryLiveTrackingGrant,
  generatedAtMs: number,
  platformRuntimeProofState: TrackingTemporaryLiveReadinessRow['platformRuntimeProofState']
): TrackingTemporaryLiveReadinessRow['readinessState'] {
  if (grant.durationSeconds > MaximumDurationSeconds) {
    return 'duration-policy-manual-required';
  }
  if (!grant.parentApproved) {
    return 'waiting-for-parent-authorization';
  }
  if (!grant.childDisclosureRequired) {
    return 'child-disclosure-required';
  }
  if (grant.state === 'unavailable' || platformRuntimeProofState === 'platform-unavailable') {
    return 'platform-unavailable-manual-required';
  }
  if (grant.state === 'denied') {
    return 'denied-by-parent';
  }
  if (grant.state === 'revoked') {
    return 'revoked-by-parent';
  }
  if (grant.state === 'expired' || Date.parse(grant.expiresAt) <= generatedAtMs) {
    return 'expired-auto-stop-required';
  }
  if (grant.state === 'active') {
    return 'active-time-boxed';
  }
  return 'ready-to-start';
}

function resolveAutoStopReason(
  grant: TrackingTemporaryLiveTrackingGrant,
  readinessState: TrackingTemporaryLiveReadinessRow['readinessState']
): TrackingTemporaryLiveReadinessRow['autoStopReason'] {
  if (readinessState === 'expired-auto-stop-required') {
    return 'expired';
  }
  if (grant.state === 'revoked') {
    return 'parent-revoked';
  }
  if (grant.state === 'denied') {
    return 'parent-denied';
  }
  if (readinessState === 'platform-unavailable-manual-required') {
    return 'platform-unavailable';
  }
  if (
    readinessState === 'duration-policy-manual-required' ||
    readinessState === 'waiting-for-parent-authorization' ||
    readinessState === 'child-disclosure-required'
  ) {
    return 'manual-required';
  }
  return 'expires-at';
}

function resolveRetentionAction(
  readinessState: TrackingTemporaryLiveReadinessRow['readinessState']
): TrackingTemporaryLiveReadinessRow['retentionAction'] {
  if (readinessState === 'expired-auto-stop-required' || readinessState === 'revoked-by-parent') {
    return 'delete-after-retention-window';
  }
  if (
    readinessState === 'duration-policy-manual-required' ||
    readinessState === 'platform-unavailable-manual-required' ||
    readinessState === 'waiting-for-parent-authorization' ||
    readinessState === 'child-disclosure-required'
  ) {
    return 'manual-review-required';
  }
  return 'retain-until-expiry';
}

function resolveManualProofRequirements(
  grant: TrackingTemporaryLiveTrackingGrant,
  platformProof: {
    location: TrackingTemporaryLiveReadinessRow['platformRuntimeProofState'];
    battery: TrackingTemporaryLiveReadinessRow['batteryStatusProofState'];
  }
) {
  const requirements = new Set<string>();
  if (grant.durationSeconds > MaximumDurationSeconds) {
    requirements.add('temporary-live-duration-policy-review-required');
  }
  if (!grant.parentApproved) {
    requirements.add('parent-authorization-required');
  }
  if (!grant.childDisclosureRequired) {
    requirements.add('child-disclosure-proof-required');
  }
  if (platformProof.location !== 'contract-proved') {
    requirements.add('live-location-runtime-proof-required');
  }
  if (platformProof.battery !== 'contract-proved') {
    requirements.add('battery-status-runtime-proof-required');
  }
  return Array.from(requirements);
}

function summarizePlatformProof(routes: ReadonlyArray<TrackingPlatformProofRoute>) {
  return {
    location: summarizeLocationProof(routes),
    battery: summarizeBatteryProof(routes),
  };
}

function summarizeLocationProof(
  routes: ReadonlyArray<TrackingPlatformProofRoute>
): TrackingTemporaryLiveReadinessRow['platformRuntimeProofState'] {
  if (routes.some((route) => route.foregroundLocation === 'contract-proved')) {
    return 'contract-proved';
  }
  if (routes.some((route) => route.foregroundLocation === 'platform-unsupported')) {
    return 'platform-unavailable';
  }
  if (routes.some((route) => route.foregroundLocation === 'real-device-required')) {
    return 'real-device-required';
  }
  return 'manual-required';
}

function summarizeBatteryProof(
  routes: ReadonlyArray<TrackingPlatformProofRoute>
): TrackingTemporaryLiveReadinessRow['batteryStatusProofState'] {
  if (routes.some((route) => route.deviceStatus === 'contract-proved')) {
    return 'contract-proved';
  }
  if (routes.some((route) => route.deviceStatus === 'platform-unsupported')) {
    return 'platform-unavailable';
  }
  if (routes.some((route) => route.deviceStatus === 'real-device-required')) {
    return 'real-device-required';
  }
  return 'manual-required';
}
