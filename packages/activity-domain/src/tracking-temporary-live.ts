import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivityTimestampSchema,
  type ActivityEvidenceId,
  type ActivityTimestamp,
  decodeActivityTimestamp,
} from './primitives';
import { TrackingDeviceStatusEvidenceSchema, type TrackingDeviceStatusEvidence } from './tracking-evidence';
import {
  TrackingAuditRefSchema,
  TrackingCapabilityStatusSchema,
  TrackingEvidenceSchemaVersion,
  TrackingNonNegativeIntegerSchema,
  TrackingPermissionStateSchema,
  TrackingReasonCodeSchema,
  TrackingRetentionModeSchema,
} from './tracking-primitives';

const TrackingTemporaryLiveDurationMinutesSchema = TrackingNonNegativeIntegerSchema.pipe(Schema.between(1, 240));
const TrackingTemporaryLiveCadenceSecondsSchema = TrackingNonNegativeIntegerSchema.pipe(Schema.between(30, 900));

export const TrackingTemporaryLiveSessionStateSchema = withParser(
  Schema.Literal('active', 'degraded', 'expired', 'stopped', 'manual-required')
);

export const TrackingTemporaryLiveStopReasonSchema = withParser(
  Schema.Literal(
    'none',
    'duration-expired',
    'parent-stopped',
    'permission-degraded',
    'battery-throttled',
    'device-offline',
    'service-stopped'
  )
);

const TrackingTemporaryLiveAuthorizationBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
  sessionId: ActivityEvidenceIdSchema,
  childDeviceId: ActivityDeviceIdSchema,
  requestedAt: ActivityTimestampSchema,
  startsAt: ActivityTimestampSchema,
  expiresAt: ActivityTimestampSchema,
  parentAuthorizationRef: TrackingAuditRefSchema,
  durationMinutes: TrackingTemporaryLiveDurationMinutesSchema,
  maxDurationMinutes: TrackingTemporaryLiveDurationMinutesSchema,
  cadenceSeconds: TrackingTemporaryLiveCadenceSecondsSchema,
  retentionMode: TrackingRetentionModeSchema,
  disclosureRequired: Schema.Boolean,
  remoteSyncDefault: Schema.Literal('disabled', 'parent-approved-only'),
  auditRefs: Schema.Array(TrackingAuditRefSchema),
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const TrackingTemporaryLiveAuthorizationSchema = withParser(
  TrackingTemporaryLiveAuthorizationBaseSchema.pipe(
    Schema.filter(
      (value) =>
        authorizationWindowIsValid(value) ||
        'temporary live tracking authorization requires bounded duration within the approved maximum'
    )
  )
);

export const TrackingTemporaryLiveRuntimeStateSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    sessionId: ActivityEvidenceIdSchema,
    childDeviceId: ActivityDeviceIdSchema,
    evaluatedAt: ActivityTimestampSchema,
    expiresAt: ActivityTimestampSchema,
    parentAuthorizationRef: TrackingAuditRefSchema,
    parentStopRequestedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    state: TrackingTemporaryLiveSessionStateSchema,
    capabilityStatus: TrackingCapabilityStatusSchema,
    permissionState: TrackingPermissionStateSchema,
    stopReason: TrackingTemporaryLiveStopReasonSchema,
    cadenceSeconds: TrackingTemporaryLiveCadenceSecondsSchema,
    nextSampleDueAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    locationEvidenceId: Schema.Union(ActivityEvidenceIdSchema, Schema.Null),
    deviceStatusEvidenceId: ActivityEvidenceIdSchema,
    retentionMode: TrackingRetentionModeSchema,
    disclosureRequired: Schema.Boolean,
    productClaimReady: Schema.Literal(false),
    reasonCodes: Schema.Array(TrackingReasonCodeSchema),
    auditRefs: Schema.Array(TrackingAuditRefSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export type TrackingTemporaryLiveAuthorization = Infer<typeof TrackingTemporaryLiveAuthorizationSchema>;
export type TrackingTemporaryLiveRuntimeState = Infer<typeof TrackingTemporaryLiveRuntimeStateSchema>;
export type TrackingTemporaryLiveSessionState = Infer<typeof TrackingTemporaryLiveSessionStateSchema>;
export type TrackingTemporaryLiveStopReason = Infer<typeof TrackingTemporaryLiveStopReasonSchema>;

export interface TrackingTemporaryLiveRuntimeInput {
  readonly authorization: TrackingTemporaryLiveAuthorization;
  readonly evaluatedAt: ActivityTimestamp;
  readonly locationCapabilityStatus: TrackingTemporaryLiveRuntimeState['capabilityStatus'];
  readonly permissionState: TrackingTemporaryLiveRuntimeState['permissionState'];
  readonly deviceStatus: TrackingDeviceStatusEvidence;
  readonly locationEvidenceId: ActivityEvidenceId | null;
  readonly parentStopRequestedAt?: ActivityTimestamp | null;
}

interface DegradedTemporaryLiveState {
  readonly capabilityStatus: TrackingTemporaryLiveRuntimeState['capabilityStatus'];
  readonly stopReason: TrackingTemporaryLiveStopReason;
  readonly reasonCodes: readonly TrackingTemporaryLiveRuntimeState['reasonCodes'][number][];
}

export function evaluateTrackingTemporaryLiveRuntime(
  input: TrackingTemporaryLiveRuntimeInput
): TrackingTemporaryLiveRuntimeState {
  const authorization = TrackingTemporaryLiveAuthorizationSchema.parse(input.authorization);
  const deviceStatus = TrackingDeviceStatusEvidenceSchema.parse(input.deviceStatus);
  const evaluatedAt = decodeActivityTimestamp(input.evaluatedAt);
  const parentStopRequestedAt =
    input.parentStopRequestedAt === undefined ? null : parseNullableTimestamp(input.parentStopRequestedAt);
  const degraded = degradedRuntimeState(input, deviceStatus);
  const expired = Date.parse(evaluatedAt) >= Date.parse(authorization.expiresAt);
  const stopped = parentStopRequestedAt !== null;
  const state = temporaryLiveState({ degraded, expired, stopped });
  const stopReason = temporaryLiveStopReason({ degraded, expired, stopped });
  const capabilityStatus = temporaryLiveCapabilityStatus({ degraded, expired, stopped });
  const nextSampleDueAt = state === 'active' ? addSeconds(evaluatedAt, authorization.cadenceSeconds) : null;
  const reasonCodes = temporaryLiveReasonCodes({ authorization, degraded, expired, stopped });

  return TrackingTemporaryLiveRuntimeStateSchema.parse({
    schemaVersion: authorization.schemaVersion,
    sessionId: authorization.sessionId,
    childDeviceId: authorization.childDeviceId,
    evaluatedAt,
    expiresAt: authorization.expiresAt,
    parentAuthorizationRef: authorization.parentAuthorizationRef,
    parentStopRequestedAt,
    state,
    capabilityStatus,
    permissionState: TrackingPermissionStateSchema.parse(input.permissionState),
    stopReason,
    cadenceSeconds: authorization.cadenceSeconds,
    nextSampleDueAt,
    locationEvidenceId: input.locationEvidenceId,
    deviceStatusEvidenceId: deviceStatus.evidenceId,
    retentionMode: authorization.retentionMode,
    disclosureRequired: authorization.disclosureRequired,
    productClaimReady: false,
    reasonCodes,
    auditRefs: authorization.auditRefs,
    evidence: [...authorization.evidence, ...deviceStatus.evidence],
  });
}

function authorizationWindowIsValid(value: Infer<typeof TrackingTemporaryLiveAuthorizationBaseSchema>) {
  const startsAt = Date.parse(value.startsAt);
  const expiresAt = Date.parse(value.expiresAt);
  const durationMillis = value.durationMinutes * 60_000;
  return (
    Number.isFinite(startsAt) &&
    Number.isFinite(expiresAt) &&
    expiresAt > startsAt &&
    value.durationMinutes <= value.maxDurationMinutes &&
    expiresAt - startsAt <= durationMillis
  );
}

function parseNullableTimestamp(value: ActivityTimestamp | null) {
  return value === null ? null : decodeActivityTimestamp(value);
}

function degradedRuntimeState(
  input: TrackingTemporaryLiveRuntimeInput,
  deviceStatus: TrackingDeviceStatusEvidence
): DegradedTemporaryLiveState | null {
  if (input.permissionState !== 'granted-foreground' && input.permissionState !== 'granted-background') {
    return degradedState(input.locationCapabilityStatus, 'permission-degraded', [
      reasonCode('temporary-live-location-permission-required'),
    ]);
  }

  if (input.locationCapabilityStatus !== 'live' && input.locationCapabilityStatus !== 'recent') {
    return degradedState(input.locationCapabilityStatus, 'permission-degraded', [
      reasonCode('temporary-live-location-capability-degraded'),
    ]);
  }

  if (deviceStatus.battery.lowPowerMode === 'enabled' || deviceStatus.capabilityStatus === 'battery-throttled') {
    return degradedState('battery-throttled', 'battery-throttled', [reasonCode('temporary-live-battery-throttled')]);
  }

  if (deviceStatus.connectivityStatus === 'offline') {
    return degradedState('offline-last-known-only', 'device-offline', [reasonCode('temporary-live-device-offline')]);
  }

  if (deviceStatus.heartbeatStatus === 'missing' || deviceStatus.heartbeatStatus === 'service-stopped') {
    return degradedState('service-disabled', 'service-stopped', [reasonCode('temporary-live-service-not-running')]);
  }

  return null;
}

function degradedState(
  capabilityStatus: TrackingTemporaryLiveRuntimeState['capabilityStatus'],
  stopReason: TrackingTemporaryLiveStopReason,
  reasonCodes: DegradedTemporaryLiveState['reasonCodes']
): DegradedTemporaryLiveState {
  return { capabilityStatus, stopReason, reasonCodes };
}

function temporaryLiveState(input: {
  readonly degraded: DegradedTemporaryLiveState | null;
  readonly expired: boolean;
  readonly stopped: boolean;
}): TrackingTemporaryLiveSessionState {
  if (input.stopped) {
    return 'stopped';
  }
  if (input.expired) {
    return 'expired';
  }
  return input.degraded === null ? 'active' : 'degraded';
}

function temporaryLiveStopReason(input: {
  readonly degraded: DegradedTemporaryLiveState | null;
  readonly expired: boolean;
  readonly stopped: boolean;
}): TrackingTemporaryLiveStopReason {
  if (input.stopped) {
    return 'parent-stopped';
  }
  if (input.expired) {
    return 'duration-expired';
  }
  return input.degraded?.stopReason ?? 'none';
}

function temporaryLiveCapabilityStatus(input: {
  readonly degraded: DegradedTemporaryLiveState | null;
  readonly expired: boolean;
  readonly stopped: boolean;
}): TrackingTemporaryLiveRuntimeState['capabilityStatus'] {
  if (input.stopped) {
    return 'disabled-by-parent';
  }
  if (input.expired) {
    return 'stale';
  }
  return input.degraded?.capabilityStatus ?? 'live';
}

function temporaryLiveReasonCodes(input: {
  readonly authorization: TrackingTemporaryLiveAuthorization;
  readonly degraded: DegradedTemporaryLiveState | null;
  readonly expired: boolean;
  readonly stopped: boolean;
}) {
  if (input.stopped) {
    return [reasonCode('temporary-live-parent-stopped')];
  }
  if (input.expired) {
    return [reasonCode('temporary-live-duration-expired')];
  }
  return input.degraded?.reasonCodes ?? [reasonCode('temporary-live-active-parent-authorized')];
}

function addSeconds(timestamp: ActivityTimestamp, seconds: number) {
  return decodeActivityTimestamp(new Date(Date.parse(timestamp) + seconds * 1_000).toISOString());
}

function reasonCode(value: unknown) {
  return TrackingReasonCodeSchema.parse(value);
}
