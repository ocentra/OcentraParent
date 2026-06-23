import { NonEmptyStringSchema, Schema, brandedNonEmptyStringSchema, withParser } from './effect';

export const TrackingNonNegativeNumberSchema = Schema.Number.pipe(Schema.nonNegative());
export const TrackingNonNegativeIntegerSchema = TrackingNonNegativeNumberSchema.pipe(Schema.int());
export const TrackingConfidenceSchema = Schema.Number.pipe(Schema.between(0, 1));
export const TrackingLatitudeSchema = Schema.Number.pipe(Schema.between(-90, 90));
export const TrackingLongitudeSchema = Schema.Number.pipe(Schema.between(-180, 180));
export const TrackingCoordinateCountSchema = Schema.Number.pipe(Schema.int(), Schema.between(3, 64));

export const TrackingEvidenceSchemaVersion = 1;

export const TrackingAdapterIdSchema = withParser(brandedNonEmptyStringSchema('TrackingAdapterId'));
export const TrackingReasonCodeSchema = withParser(brandedNonEmptyStringSchema('TrackingReasonCode'));
export const TrackingRuleIdSchema = withParser(brandedNonEmptyStringSchema('TrackingRuleId'));
export const TrackingPlaceIdSchema = withParser(brandedNonEmptyStringSchema('TrackingPlaceId'));
export const TrackingGeofenceIdSchema = withParser(brandedNonEmptyStringSchema('TrackingGeofenceId'));
export const TrackingScheduleIdSchema = withParser(brandedNonEmptyStringSchema('TrackingScheduleId'));
export const TrackingProviderRefSchema = withParser(brandedNonEmptyStringSchema('TrackingProviderRef'));
export const TrackingAuditRefSchema = withParser(brandedNonEmptyStringSchema('TrackingAuditRef'));
export const TrackingLabelSchema = withParser(brandedNonEmptyStringSchema('TrackingLabel'));
export const TrackingTimezoneSchema = withParser(brandedNonEmptyStringSchema('TrackingTimezone'));

export const TrackingSourceKindSchema = withParser(
  Schema.Literal(
    'android-fused-location',
    'android-geofence',
    'android-device-status',
    'ios-core-location',
    'ios-region-monitoring',
    'desktop-os-location',
    'desktop-presence-hint',
    'manual-child-check-in',
    'parent-defined-place',
    'nearby-place-provider',
    'journal-replay'
  )
);

export const TrackingCapabilityStatusSchema = withParser(
  Schema.Literal(
    'live',
    'recent',
    'stale',
    'last-known',
    'foreground-only',
    'background-ready',
    'approximate-only',
    'permission-required',
    'offline-last-known-only',
    'background-permission-required',
    'platform-unsupported',
    'permission-denied',
    'service-disabled',
    'battery-throttled',
    'unavailable',
    'manual-required',
    'adapter-error',
    'disabled-by-parent'
  )
);

export const TrackingPermissionStateSchema = withParser(
  Schema.Literal(
    'granted-foreground',
    'granted-background',
    'approximate-only',
    'denied',
    'restricted',
    'not-requested',
    'service-disabled',
    'unavailable',
    'manual-required'
  )
);

export const TrackingCustodyLabelSchema = withParser(
  Schema.Literal(
    'child-device-local',
    'live-lan-child-agent',
    'parent-device-cache',
    'parent-owned-export',
    'parent-approved-cloud',
    'unavailable'
  )
);

export const TrackingRetentionModeSchema = withParser(
  Schema.Literal('last-known-only', '24h', '7d', '30d', 'custom', 'delete-on-resolution', 'export-only')
);

export const TrackingHintQualitySchema = withParser(
  Schema.Literal(
    'gps',
    'os-location',
    'geofence-region',
    'wifi-lan-hint',
    'ip-coarse-hint',
    'manual-check-in',
    'unknown'
  )
);

export const TrackingPlatformSchema = withParser(
  Schema.Literal('android', 'ios', 'windows', 'macos', 'linux', 'web', 'unknown')
);

export const TrackingPlatformProofStateSchema = withParser(
  Schema.Literal('proved', 'manual-required', 'simulator-only', 'blocked', 'not-implemented', 'unavailable')
);

export const TrackingCoordinateSchema = withParser(
  Schema.Struct({
    latitude: TrackingLatitudeSchema,
    longitude: TrackingLongitudeSchema,
  })
);

export const TrackingTimeWindowSchema = withParser(
  Schema.Struct({
    startsAt: NonEmptyStringSchema,
    endsAt: NonEmptyStringSchema,
    timezone: TrackingTimezoneSchema,
  })
);
