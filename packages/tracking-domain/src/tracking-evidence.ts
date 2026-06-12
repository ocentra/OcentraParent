import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivitySourceIdSchema,
  ActivityTimestampSchema,
} from './primitives';
import {
  TrackingAdapterIdSchema,
  TrackingAuditRefSchema,
  TrackingCapabilityStatusSchema,
  TrackingConfidenceSchema,
  TrackingCoordinateSchema,
  TrackingCustodyLabelSchema,
  TrackingEvidenceSchemaVersion,
  TrackingHintQualitySchema,
  TrackingNonNegativeIntegerSchema,
  TrackingNonNegativeNumberSchema,
  TrackingPermissionStateSchema,
  TrackingPlatformProofStateSchema,
  TrackingPlatformSchema,
  TrackingReasonCodeSchema,
  TrackingRetentionModeSchema,
  TrackingSourceKindSchema,
} from './tracking-primitives';

export const TrackingLocationHintSchema = withParser(
  Schema.Struct({
    quality: TrackingHintQualitySchema,
    coarseRadiusMeters: Schema.Union(TrackingNonNegativeNumberSchema, Schema.Null),
    label: Schema.Union(Schema.String.pipe(Schema.minLength(1)), Schema.Null),
  })
);

const TrackingLocationEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
  evidenceId: ActivityEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  freshUntil: ActivityTimestampSchema,
  staleAt: ActivityTimestampSchema,
  sourceId: ActivitySourceIdSchema,
  adapterId: TrackingAdapterIdSchema,
  deviceId: ActivityDeviceIdSchema,
  sourceKind: TrackingSourceKindSchema,
  capabilityStatus: TrackingCapabilityStatusSchema,
  permissionState: TrackingPermissionStateSchema,
  coordinate: Schema.Union(TrackingCoordinateSchema, Schema.Null),
  accuracyMeters: Schema.Union(TrackingNonNegativeNumberSchema, Schema.Null),
  hint: TrackingLocationHintSchema,
  confidence: TrackingConfidenceSchema,
  custodyLabel: TrackingCustodyLabelSchema,
  retentionMode: TrackingRetentionModeSchema,
  reasonCodes: Schema.Array(TrackingReasonCodeSchema),
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const TrackingLocationEvidenceSchema = withParser(
  TrackingLocationEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        trackingLocationPrecisionIsHonest(value) ||
        'LAN, Wi-Fi, IP, manual, and unknown tracking hints must not carry precise coordinates or GPS accuracy'
    )
  )
);

export const TrackingBatteryStatusSchema = withParser(
  Schema.Struct({
    percent: Schema.Union(Schema.Number.pipe(Schema.between(0, 100)), Schema.Null),
    chargingState: Schema.Literal('charging', 'discharging', 'full', 'unknown'),
    lowPowerMode: Schema.Literal('enabled', 'disabled', 'unknown'),
  })
);

export const TrackingConnectivityStatusSchema = withParser(
  Schema.Literal('online', 'offline', 'captive-network', 'metered', 'unknown')
);

export const TrackingHeartbeatStatusSchema = withParser(
  Schema.Literal('healthy', 'late', 'missing', 'service-stopped', 'unknown')
);

export const TrackingDeviceStatusEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    evidenceId: ActivityEvidenceIdSchema,
    observedAt: ActivityTimestampSchema,
    freshUntil: ActivityTimestampSchema,
    staleAt: ActivityTimestampSchema,
    sourceId: ActivitySourceIdSchema,
    adapterId: TrackingAdapterIdSchema,
    deviceId: ActivityDeviceIdSchema,
    sourceKind: TrackingSourceKindSchema,
    capabilityStatus: TrackingCapabilityStatusSchema,
    lastLocationEvidenceId: Schema.Union(ActivityEvidenceIdSchema, Schema.Null),
    heartbeatStatus: TrackingHeartbeatStatusSchema,
    battery: TrackingBatteryStatusSchema,
    connectivityStatus: TrackingConnectivityStatusSchema,
    pendingUploadCount: TrackingNonNegativeIntegerSchema,
    custodyLabel: TrackingCustodyLabelSchema,
    retentionMode: TrackingRetentionModeSchema,
    degradedReasons: Schema.Array(TrackingReasonCodeSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const TrackingCapabilityStatusMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    checkedAt: ActivityTimestampSchema,
    platform: TrackingPlatformSchema,
    foregroundLocation: TrackingPlatformProofStateSchema,
    backgroundLocation: TrackingPlatformProofStateSchema,
    geofenceTransitions: TrackingPlatformProofStateSchema,
    deviceStatus: TrackingPlatformProofStateSchema,
    permissionState: TrackingPermissionStateSchema,
    manualActionRequired: Schema.Boolean,
    sourceId: ActivitySourceIdSchema,
    adapterId: TrackingAdapterIdSchema,
    reasonCodes: Schema.Array(TrackingReasonCodeSchema),
    auditRefs: Schema.Array(TrackingAuditRefSchema),
  })
);

export const TrackingRetentionPolicySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    policyId: TrackingReasonCodeSchema,
    mode: TrackingRetentionModeSchema,
    custodyLabel: TrackingCustodyLabelSchema,
    customRetentionHours: Schema.Union(TrackingNonNegativeIntegerSchema, Schema.Null),
    deleteOnResolution: Schema.Boolean,
    exportAllowed: Schema.Boolean,
    remoteSyncDefault: Schema.Literal('disabled', 'parent-approved-only'),
    auditRefs: Schema.Array(TrackingAuditRefSchema),
  })
);

export type TrackingLocationHint = Infer<typeof TrackingLocationHintSchema>;
type TrackingLocationEvidenceBase = Infer<typeof TrackingLocationEvidenceBaseSchema>;
export type TrackingLocationEvidence = Infer<typeof TrackingLocationEvidenceSchema>;
export type TrackingBatteryStatus = Infer<typeof TrackingBatteryStatusSchema>;
export type TrackingConnectivityStatus = Infer<typeof TrackingConnectivityStatusSchema>;
export type TrackingHeartbeatStatus = Infer<typeof TrackingHeartbeatStatusSchema>;
export type TrackingDeviceStatusEvidence = Infer<typeof TrackingDeviceStatusEvidenceSchema>;
export type TrackingCapabilityStatusMatrix = Infer<typeof TrackingCapabilityStatusMatrixSchema>;
export type TrackingRetentionPolicy = Infer<typeof TrackingRetentionPolicySchema>;

function trackingLocationPrecisionIsHonest(value: TrackingLocationEvidenceBase) {
  const carriesPreciseLocation = value.coordinate !== null || value.accuracyMeters !== null;
  if (!carriesPreciseLocation) {
    return true;
  }

  const preciseSourceKind =
    value.sourceKind === 'android-fused-location' ||
    value.sourceKind === 'android-geofence' ||
    value.sourceKind === 'ios-core-location' ||
    value.sourceKind === 'ios-region-monitoring' ||
    value.sourceKind === 'desktop-os-location';
  const preciseHintQuality =
    value.hint.quality === 'gps' || value.hint.quality === 'os-location' || value.hint.quality === 'geofence-region';

  return preciseSourceKind && preciseHintQuality && value.coordinate !== null && value.accuracyMeters !== null;
}
