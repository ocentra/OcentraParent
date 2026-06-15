import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import {
  TrackingAuditRefSchema,
  TrackingCapabilityStatusSchema,
  TrackingConfidenceSchema,
  TrackingCoordinateSchema,
  TrackingEvidenceSchemaVersion,
  TrackingGeofenceIdSchema,
  TrackingLabelSchema,
  TrackingNonNegativeIntegerSchema,
  TrackingNonNegativeNumberSchema,
  TrackingPlaceIdSchema,
  TrackingProviderRefSchema,
  TrackingReasonCodeSchema,
  TrackingRetentionModeSchema,
  TrackingRuleIdSchema,
  TrackingScheduleIdSchema,
  TrackingTimeWindowSchema,
} from './tracking-primitives';

export const TrackingGeofencePlaceKindSchema = withParser(
  Schema.Literal('home', 'school', 'activity', 'safe-zone', 'restricted-zone', 'temporary-trip', 'custom')
);

export const TrackingGeofenceShapeKindSchema = withParser(Schema.Literal('circle', 'polygon'));
export const TrackingGeofenceTransitionKindSchema = withParser(
  Schema.Literal('enter', 'exit', 'dwell', 'missed-arrival', 'stale-at-place', 'ambiguous')
);
export const TrackingExpectedPlaceOutcomeSchema = withParser(
  Schema.Literal('where-expected', 'left-expected-place', 'late-arrival', 'early-exit', 'unknown', 'manual-required')
);
export const TrackingExpectedPlaceExceptionStateSchema = withParser(Schema.Literal('holiday-mode', 'trip-exception'));
export const TrackingNearbyProviderKindSchema = withParser(
  Schema.Literal('google-places', 'apple-mapkit', 'openstreetmap', 'parent-defined', 'local-cache', 'unavailable')
);
export const TrackingPlaceRiskCategorySchema = withParser(
  Schema.Literal(
    'home',
    'school',
    'hospital',
    'cinema',
    'mall',
    'bar',
    'nightclub',
    'liquor',
    'casino',
    'hotel',
    'transit',
    'park',
    'friend-area',
    'out-of-town',
    'remote-area',
    'unknown'
  )
);
export const TrackingAmbiguityStateSchema = withParser(
  Schema.Literal('clear', 'multiple-candidates', 'low-accuracy', 'provider-unavailable', 'unknown')
);

const TrackingGeofenceShapeBaseSchema = Schema.Struct({
  kind: TrackingGeofenceShapeKindSchema,
  center: Schema.Union(TrackingCoordinateSchema, Schema.Null),
  radiusMeters: Schema.Union(TrackingNonNegativeNumberSchema, Schema.Null),
  polygon: Schema.Array(TrackingCoordinateSchema),
});

export const TrackingGeofenceShapeSchema = withParser(
  TrackingGeofenceShapeBaseSchema.pipe(
    Schema.filter(
      (shape) =>
        trackingGeofenceShapeIsValid(shape) ||
        'Circle geofences need center and radius; polygon geofences need between 3 and 64 coordinates'
    )
  )
);

export const TrackingGeofenceRuleSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    ruleId: TrackingRuleIdSchema,
    geofenceId: TrackingGeofenceIdSchema,
    placeId: TrackingPlaceIdSchema,
    label: TrackingLabelSchema,
    placeKind: TrackingGeofencePlaceKindSchema,
    shape: TrackingGeofenceShapeSchema,
    minAccuracyMeters: TrackingNonNegativeNumberSchema,
    enterGraceSeconds: TrackingNonNegativeIntegerSchema,
    exitGraceSeconds: TrackingNonNegativeIntegerSchema,
    dwellSeconds: TrackingNonNegativeIntegerSchema,
    scheduleId: Schema.Union(TrackingScheduleIdSchema, Schema.Null),
    enabled: Schema.Boolean,
    retentionMode: TrackingRetentionModeSchema,
    auditRefs: Schema.Array(TrackingAuditRefSchema),
  })
);

const TrackingGeofenceTransitionBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
  transitionId: ActivityEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  ruleId: TrackingRuleIdSchema,
  geofenceId: TrackingGeofenceIdSchema,
  locationEvidenceId: ActivityEvidenceIdSchema,
  transition: TrackingGeofenceTransitionKindSchema,
  capabilityStatus: TrackingCapabilityStatusSchema,
  distanceMeters: Schema.Union(TrackingNonNegativeNumberSchema, Schema.Null),
  reasonCodes: Schema.Array(TrackingReasonCodeSchema),
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const TrackingGeofenceTransitionSchema = withParser(
  TrackingGeofenceTransitionBaseSchema.pipe(
    Schema.filter(
      (transition) =>
        trackingGeofenceTransitionCapabilityIsHonest(transition) ||
        'Enter, exit, and dwell geofence transitions require live or recent location capability status'
    )
  )
);

export const TrackingExpectedPlaceActiveExceptionSchema = withParser(
  Schema.Struct({
    state: TrackingExpectedPlaceExceptionStateSchema,
    auditRef: TrackingAuditRefSchema,
  })
);

export const TrackingExpectedPlaceScheduleSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    scheduleId: TrackingScheduleIdSchema,
    ruleId: Schema.optionalWith(Schema.Union(TrackingRuleIdSchema, Schema.Null), {
      default: () => null,
    }),
    placeId: TrackingPlaceIdSchema,
    label: TrackingLabelSchema,
    windows: Schema.Array(TrackingTimeWindowSchema),
    distanceToleranceMeters: Schema.optionalWith(
      Schema.Union(TrackingNonNegativeNumberSchema, Schema.Null),
      {
        default: () => null,
      }
    ),
    lateGraceSeconds: TrackingNonNegativeIntegerSchema,
    earlyExitGraceSeconds: TrackingNonNegativeIntegerSchema,
    activeException: Schema.optionalWith(Schema.Union(TrackingExpectedPlaceActiveExceptionSchema, Schema.Null), {
      default: () => null,
    }),
    enabled: Schema.Boolean,
    auditRefs: Schema.Array(TrackingAuditRefSchema),
  })
);

export const TrackingExpectedPlaceDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    decisionId: ActivityEvidenceIdSchema,
    observedAt: ActivityTimestampSchema,
    scheduleId: TrackingScheduleIdSchema,
    ruleId: Schema.optionalWith(Schema.Union(TrackingRuleIdSchema, Schema.Null), {
      default: () => null,
    }),
    locationEvidenceId: ActivityEvidenceIdSchema,
    outcome: TrackingExpectedPlaceOutcomeSchema,
    distanceToleranceMeters: Schema.optionalWith(
      Schema.Union(TrackingNonNegativeNumberSchema, Schema.Null),
      {
        default: () => null,
      }
    ),
    lateGraceSeconds: Schema.optionalWith(TrackingNonNegativeIntegerSchema, {
      default: () => 0,
    }),
    earlyExitGraceSeconds: Schema.optionalWith(TrackingNonNegativeIntegerSchema, {
      default: () => 0,
    }),
    exceptionState: Schema.optionalWith(Schema.Union(TrackingExpectedPlaceExceptionStateSchema, Schema.Null), {
      default: () => null,
    }),
    exceptionAuditRef: Schema.optionalWith(Schema.Union(TrackingAuditRefSchema, Schema.Null), {
      default: () => null,
    }),
    reasonCodes: Schema.Array(TrackingReasonCodeSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const TrackingNearbyPlaceEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    evidenceId: ActivityEvidenceIdSchema,
    observedAt: ActivityTimestampSchema,
    locationEvidenceId: ActivityEvidenceIdSchema,
    providerKind: TrackingNearbyProviderKindSchema,
    providerRef: Schema.Union(TrackingProviderRefSchema, Schema.Null),
    queryRadiusMeters: TrackingNonNegativeNumberSchema,
    distanceMeters: Schema.Union(TrackingNonNegativeNumberSchema, Schema.Null),
    category: TrackingPlaceRiskCategorySchema,
    confidence: TrackingConfidenceSchema,
    ambiguityState: TrackingAmbiguityStateSchema,
    reasonCodes: Schema.Array(TrackingReasonCodeSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const TrackingParentDefinedPlaceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
    placeId: TrackingPlaceIdSchema,
    label: TrackingLabelSchema,
    placeKind: TrackingGeofencePlaceKindSchema,
    shape: TrackingGeofenceShapeSchema,
    createdAt: ActivityTimestampSchema,
    updatedAt: ActivityTimestampSchema,
    auditRefs: Schema.Array(TrackingAuditRefSchema),
  })
);

export type TrackingGeofencePlaceKind = Infer<typeof TrackingGeofencePlaceKindSchema>;
export type TrackingGeofenceShapeKind = Infer<typeof TrackingGeofenceShapeKindSchema>;
export type TrackingGeofenceTransitionKind = Infer<typeof TrackingGeofenceTransitionKindSchema>;
export type TrackingExpectedPlaceOutcome = Infer<typeof TrackingExpectedPlaceOutcomeSchema>;
export type TrackingExpectedPlaceExceptionState = Infer<typeof TrackingExpectedPlaceExceptionStateSchema>;
export type TrackingNearbyProviderKind = Infer<typeof TrackingNearbyProviderKindSchema>;
export type TrackingPlaceRiskCategory = Infer<typeof TrackingPlaceRiskCategorySchema>;
export type TrackingAmbiguityState = Infer<typeof TrackingAmbiguityStateSchema>;
type TrackingGeofenceShapeBase = Infer<typeof TrackingGeofenceShapeBaseSchema>;
type TrackingGeofenceTransitionBase = Infer<typeof TrackingGeofenceTransitionBaseSchema>;
export type TrackingGeofenceShape = Infer<typeof TrackingGeofenceShapeSchema>;
export type TrackingGeofenceRule = Infer<typeof TrackingGeofenceRuleSchema>;
export type TrackingGeofenceTransition = Infer<typeof TrackingGeofenceTransitionSchema>;
export type TrackingExpectedPlaceActiveException = Infer<typeof TrackingExpectedPlaceActiveExceptionSchema>;
export type TrackingExpectedPlaceSchedule = Infer<typeof TrackingExpectedPlaceScheduleSchema>;
export type TrackingExpectedPlaceDecision = Infer<typeof TrackingExpectedPlaceDecisionSchema>;
export type TrackingNearbyPlaceEvidence = Infer<typeof TrackingNearbyPlaceEvidenceSchema>;
export type TrackingParentDefinedPlace = Infer<typeof TrackingParentDefinedPlaceSchema>;

function trackingGeofenceShapeIsValid(shape: TrackingGeofenceShapeBase) {
  if (shape.kind === 'circle') {
    return shape.center !== null && shape.radiusMeters !== null && shape.polygon.length === 0;
  }

  return (
    shape.center === null && shape.radiusMeters === null && shape.polygon.length >= 3 && shape.polygon.length <= 64
  );
}

function trackingGeofenceTransitionCapabilityIsHonest(transition: TrackingGeofenceTransitionBase) {
  if (transition.transition === 'ambiguous' || transition.transition === 'missed-arrival') {
    return true;
  }

  if (transition.transition === 'stale-at-place') {
    return transition.capabilityStatus === 'stale' || transition.capabilityStatus === 'offline-last-known-only';
  }

  return transition.capabilityStatus === 'live' || transition.capabilityStatus === 'recent';
}
