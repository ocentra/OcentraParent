import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  TrackingAuditRefSchema,
  TrackingCoordinateSchema,
  TrackingEvidenceSchemaVersion,
  TrackingNonNegativeIntegerSchema,
  TrackingNonNegativeNumberSchema,
  TrackingProviderRefSchema,
  TrackingReasonCodeSchema,
} from './tracking-primitives';
import {
  TrackingAmbiguityStateSchema,
  TrackingNearbyPlaceEvidenceSchema,
  type TrackingAmbiguityState,
  type TrackingNearbyPlaceEvidence,
} from './tracking-geofence';
import { categoryForGooglePlace } from './tracking-poi-provider-category';

export const TrackingGooglePlacesNearbyMaxRadiusMeters = 50_000;
export const TrackingGooglePlacesNearbyMaxResultCount = 10;

export const TrackingGooglePlacesNearbyFieldSchema = withParser(
  Schema.Literal('places.id', 'places.location', 'places.primaryType', 'places.types')
);

export const TrackingGooglePlacesNearbyRankPreferenceSchema = withParser(Schema.Literal('DISTANCE', 'POPULARITY'));
export const TrackingPoiProviderFailureReasonSchema = withParser(
  Schema.Literal('provider-unavailable', 'request-denied', 'quota-exhausted', 'timeout', 'invalid-provider-response')
);

const TrackingGooglePlacesTypeSchema = withParser(Schema.String.pipe(Schema.minLength(1)));
const TrackingGooglePlacesNearbyFieldMaskSchema = Schema.Array(TrackingGooglePlacesNearbyFieldSchema).pipe(
  Schema.filter((fields) => fields.length > 0 || 'Expected at least one Google Places field mask field'),
  Schema.filter((fields) => trackingValuesAreUnique(fields) || 'Expected Google Places field mask fields to be unique')
);
const TrackingGooglePlacesIncludedTypeSchema = Schema.Array(TrackingGooglePlacesTypeSchema).pipe(
  Schema.filter((types) => types.length > 0 || 'Expected at least one Google Places included type'),
  Schema.filter((types) => trackingValuesAreUnique(types) || 'Expected Google Places included types to be unique')
);
const TrackingGooglePlacesExcludedTypeSchema = Schema.Array(TrackingGooglePlacesTypeSchema).pipe(
  Schema.filter((types) => trackingValuesAreUnique(types) || 'Expected Google Places excluded types to be unique')
);
const TrackingGooglePlacesNearbyRadiusSchema = TrackingNonNegativeNumberSchema.pipe(
  Schema.filter(
    (radiusMeters) =>
      (radiusMeters > 0 && radiusMeters <= TrackingGooglePlacesNearbyMaxRadiusMeters) ||
      'Expected Google Places nearby radius between 1 and 50000 meters'
  )
);
const TrackingGooglePlacesNearbyMaxResultCountSchema = TrackingNonNegativeIntegerSchema.pipe(
  Schema.filter(
    (maxResultCount) =>
      (maxResultCount > 0 && maxResultCount <= TrackingGooglePlacesNearbyMaxResultCount) ||
      'Expected Google Places nearby maxResultCount between 1 and 10'
  )
);

const TrackingGooglePlacesNearbyRequestBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingEvidenceSchemaVersion),
  requestId: ActivityEvidenceIdSchema,
  requestedAt: ActivityTimestampSchema,
  providerKind: Schema.Literal('google-places'),
  providerRef: TrackingProviderRefSchema,
  locationEvidenceId: ActivityEvidenceIdSchema,
  center: TrackingCoordinateSchema,
  radiusMeters: TrackingGooglePlacesNearbyRadiusSchema,
  includedTypes: TrackingGooglePlacesIncludedTypeSchema,
  excludedTypes: TrackingGooglePlacesExcludedTypeSchema,
  fieldMask: TrackingGooglePlacesNearbyFieldMaskSchema,
  maxResultCount: TrackingGooglePlacesNearbyMaxResultCountSchema,
  rankPreference: TrackingGooglePlacesNearbyRankPreferenceSchema,
  productionUse: Schema.Boolean,
  auditRefs: Schema.Array(TrackingAuditRefSchema),
});

export const TrackingGooglePlacesNearbyRequestSchema = withParser(
  TrackingGooglePlacesNearbyRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        trackingTypeRestrictionsDoNotConflict(request) ||
        'Expected Google Places includedTypes and excludedTypes to be disjoint'
    )
  )
);

export const TrackingGooglePlacesNearbyPlaceSchema = withParser(
  Schema.Struct({
    providerRef: TrackingProviderRefSchema,
    location: TrackingCoordinateSchema,
    primaryType: Schema.Union(TrackingGooglePlacesTypeSchema, Schema.Null),
    types: Schema.Array(TrackingGooglePlacesTypeSchema),
    distanceMeters: TrackingNonNegativeNumberSchema,
  })
);

export const TrackingGooglePlacesNearbyResponseSchema = withParser(
  Schema.Struct({
    places: Schema.Array(TrackingGooglePlacesNearbyPlaceSchema),
  })
);

export type TrackingPoiProviderFailureReason = Infer<typeof TrackingPoiProviderFailureReasonSchema>;
export type TrackingGooglePlacesNearbyRequest = Infer<typeof TrackingGooglePlacesNearbyRequestSchema>;
export type TrackingGooglePlacesNearbyPlace = Infer<typeof TrackingGooglePlacesNearbyPlaceSchema>;
export type TrackingGooglePlacesNearbyResponse = Infer<typeof TrackingGooglePlacesNearbyResponseSchema>;

export interface TrackingGooglePlacesNearbyEvidenceInput {
  readonly request: TrackingGooglePlacesNearbyRequest;
  readonly response: TrackingGooglePlacesNearbyResponse;
  readonly evidenceId: TrackingNearbyPlaceEvidence['evidenceId'];
  readonly observedAt: TrackingNearbyPlaceEvidence['observedAt'];
  readonly evidence: readonly TrackingNearbyPlaceEvidence['evidence'][number][];
}

export interface TrackingGooglePlacesNearbyUnavailableEvidenceInput {
  readonly request: TrackingGooglePlacesNearbyRequest;
  readonly evidenceId: TrackingNearbyPlaceEvidence['evidenceId'];
  readonly observedAt: TrackingNearbyPlaceEvidence['observedAt'];
  readonly failureReason: TrackingPoiProviderFailureReason;
  readonly evidence: readonly TrackingNearbyPlaceEvidence['evidence'][number][];
}

export function buildTrackingGooglePlacesNearbyFieldMaskHeader(request: TrackingGooglePlacesNearbyRequest) {
  return TrackingGooglePlacesNearbyRequestSchema.parse(request).fieldMask.join(',');
}

export function buildTrackingGooglePlacesNearbyEvidence(
  input: TrackingGooglePlacesNearbyEvidenceInput
): TrackingNearbyPlaceEvidence {
  const request = TrackingGooglePlacesNearbyRequestSchema.parse(input.request);
  const response = TrackingGooglePlacesNearbyResponseSchema.parse(input.response);
  const candidates = response.places
    .filter((place) => place.distanceMeters <= request.radiusMeters)
    .sort((left, right) => left.distanceMeters - right.distanceMeters);

  if (candidates.length === 0) {
    return buildTrackingGooglePlacesNearbyUnavailableEvidence({
      request,
      evidenceId: input.evidenceId,
      observedAt: input.observedAt,
      failureReason: 'invalid-provider-response',
      evidence: input.evidence,
    });
  }

  const selected = candidates[0];
  const ambiguityState = candidates.length === 1 ? 'clear' : 'multiple-candidates';

  return TrackingNearbyPlaceEvidenceSchema.parse({
    schemaVersion: TrackingEvidenceSchemaVersion,
    evidenceId: input.evidenceId,
    observedAt: input.observedAt,
    locationEvidenceId: request.locationEvidenceId,
    providerKind: 'google-places',
    providerRef: selected?.providerRef ?? request.providerRef,
    queryRadiusMeters: request.radiusMeters,
    distanceMeters: selected?.distanceMeters ?? null,
    category: categoryForGooglePlace(selected),
    confidence: confidenceForProviderCandidate(ambiguityState),
    ambiguityState,
    reasonCodes: reasonCodesForProviderCandidate(ambiguityState, selected),
    evidence: input.evidence.map((ref) => ActivityEvidenceRefSchema.parse(ref)),
  });
}

export function buildTrackingGooglePlacesNearbyUnavailableEvidence(
  input: TrackingGooglePlacesNearbyUnavailableEvidenceInput
): TrackingNearbyPlaceEvidence {
  const request = TrackingGooglePlacesNearbyRequestSchema.parse(input.request);
  const failureReason = TrackingPoiProviderFailureReasonSchema.parse(input.failureReason);

  return TrackingNearbyPlaceEvidenceSchema.parse({
    schemaVersion: TrackingEvidenceSchemaVersion,
    evidenceId: input.evidenceId,
    observedAt: input.observedAt,
    locationEvidenceId: request.locationEvidenceId,
    providerKind: 'unavailable',
    providerRef: request.providerRef,
    queryRadiusMeters: request.radiusMeters,
    distanceMeters: null,
    category: 'unknown',
    confidence: 0,
    ambiguityState: 'provider-unavailable',
    reasonCodes: [TrackingReasonCodeSchema.parse(failureReason), TrackingReasonCodeSchema.parse('provider-degraded')],
    evidence: input.evidence.map((ref) => ActivityEvidenceRefSchema.parse(ref)),
  });
}

function confidenceForProviderCandidate(ambiguityState: TrackingAmbiguityState) {
  const parsed = TrackingAmbiguityStateSchema.parse(ambiguityState);
  if (parsed === 'clear') return 0.82;
  if (parsed === 'multiple-candidates') return 0.56;
  return 0.2;
}

function reasonCodesForProviderCandidate(
  ambiguityState: TrackingAmbiguityState,
  place: TrackingGooglePlacesNearbyPlace | undefined
) {
  const reasonCodes = [TrackingReasonCodeSchema.parse('google-places-nearby-mapped')];

  if (!place?.primaryType && place?.types.length === 0) {
    reasonCodes.push(TrackingReasonCodeSchema.parse('provider-place-type-missing'));
  }

  if (ambiguityState === 'multiple-candidates') {
    reasonCodes.push(TrackingReasonCodeSchema.parse('nearby-place-multiple-candidates'));
  }

  return reasonCodes;
}

function trackingValuesAreUnique(values: readonly unknown[]) {
  return values.length === new Set(values).size;
}

function trackingTypeRestrictionsDoNotConflict(request: Infer<typeof TrackingGooglePlacesNearbyRequestBaseSchema>) {
  const excluded = new Set(request.excludedTypes);
  return request.includedTypes.every((type) => !excluded.has(type));
}
