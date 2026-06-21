import { type Infer, Schema, withParser } from './effect';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './evidence-primitives';
import {
  TrackingCapabilityStatusMatrixSchema,
  TrackingDeviceStatusEvidenceSchema,
  TrackingLocationEvidenceSchema,
  TrackingRetentionPolicySchema,
} from './tracking-evidence';
import {
  TrackingExpectedPlaceDecisionSchema,
  TrackingGeofenceTransitionSchema,
  TrackingNearbyPlaceEvidenceSchema,
} from './tracking-geofence';
import {
  TrackingCapabilityStatusSchema,
  TrackingCustodyLabelSchema,
  TrackingEvidenceSchemaVersion,
  TrackingNonNegativeIntegerSchema,
  TrackingReasonCodeSchema,
} from './tracking-primitives';

export const TrackingTimelineRowKindSchema = withParser(
  Schema.Literal('location', 'device-status', 'geofence-transition', 'expected-place', 'nearby-place', 'retention')
);

export const TrackingTimelineRowSchema = withParser(
  Schema.Struct({
    rowId: ActivityEvidenceIdSchema,
    kind: TrackingTimelineRowKindSchema,
    observedAt: ActivityTimestampSchema,
    capabilityStatus: TrackingCapabilityStatusSchema,
    reasonCodes: Schema.Array(TrackingReasonCodeSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const TrackingReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    custodyLabel: TrackingCustodyLabelSchema,
    capabilityStatus: TrackingCapabilityStatusSchema,
    limit: TrackingNonNegativeIntegerSchema,
    returned: TrackingNonNegativeIntegerSchema,
    locationRows: Schema.Array(TrackingLocationEvidenceSchema),
    deviceStatusRows: Schema.Array(TrackingDeviceStatusEvidenceSchema),
    capabilityRows: Schema.Array(TrackingCapabilityStatusMatrixSchema),
    geofenceTransitions: Schema.Array(TrackingGeofenceTransitionSchema),
    expectedPlaceDecisions: Schema.Array(TrackingExpectedPlaceDecisionSchema),
    nearbyPlaceRows: Schema.Array(TrackingNearbyPlaceEvidenceSchema),
    retentionPolicies: Schema.Array(TrackingRetentionPolicySchema),
    timeline: Schema.Array(TrackingTimelineRowSchema),
  })
);

export type TrackingTimelineRowKind = Infer<typeof TrackingTimelineRowKindSchema>;
export type TrackingTimelineRow = Infer<typeof TrackingTimelineRowSchema>;
export type TrackingReadModel = Infer<typeof TrackingReadModelSchema>;
