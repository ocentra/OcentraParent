import {
  Schema,
  withParser,
  type Infer,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from './effect';
import {
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const LatitudeSchema = Schema.Number.pipe(Schema.between(-90, 90));
const LongitudeSchema = Schema.Number.pipe(Schema.between(-180, 180));
const RadiusMetersSchema = Schema.Number.pipe(Schema.int(), Schema.between(25, 1000));
const ResultCountSchema = Schema.Number.pipe(Schema.int(), Schema.between(1, 20));
const ConfidenceSchema = Schema.Number.pipe(Schema.between(0, 1));
const DistanceMetersSchema = Schema.Number.pipe(Schema.nonNegative());
const IncludedTypesSchema = Schema.Array(NonEmptyStringSchema).pipe(Schema.minItems(1), Schema.maxItems(50));

export const TrackingPoiProviderId = {
  GooglePlacesNearby: 'google-places-nearby',
  AppleMapKitSearch: 'apple-mapkit-search',
  OpenStreetMapNominatim: 'openstreetmap-nominatim',
} as const;

export const TrackingPoiProviderStatus = {
  RequestReady: 'request-ready',
  ResponseMapped: 'response-mapped',
  ProviderUnavailable: 'provider-unavailable',
  ManualRequired: 'manual-required',
} as const;

export const TrackingPoiProviderParityStatus = {
  RequestMapped: 'request-mapped',
  ManualRequired: 'manual-required',
  ProviderUnavailable: 'provider-unavailable',
} as const;

export const TrackingPoiCategory = {
  School: 'school',
  Food: 'food',
  Store: 'store',
  Transit: 'transit',
  Sensitive: 'sensitive',
  Healthcare: 'healthcare',
  Unknown: 'unknown',
} as const;

export const TrackingPoiAmbiguityState = {
  SingleCandidate: 'single-candidate',
  MultipleCandidates: 'multiple-candidates',
  LowAccuracy: 'low-accuracy',
  NoCandidates: 'no-candidates',
} as const;

export const GooglePlacesNearbyFieldMask = [
  'places.id',
  'places.name',
  'places.displayName.text',
  'places.location',
  'places.primaryType',
  'places.types',
] as const;

export const TrackingPoiProviderIdSchema = withParser(Schema.Literal(...Object.values(TrackingPoiProviderId)));
export const TrackingPoiProviderStatusSchema = withParser(Schema.Literal(...Object.values(TrackingPoiProviderStatus)));
export const TrackingPoiProviderParityStatusSchema = withParser(
  Schema.Literal(...Object.values(TrackingPoiProviderParityStatus))
);
export const TrackingPoiCategorySchema = withParser(Schema.Literal(...Object.values(TrackingPoiCategory)));
export const TrackingPoiAmbiguityStateSchema = withParser(Schema.Literal(...Object.values(TrackingPoiAmbiguityState)));
export const GooglePlacesNearbyFieldMaskSchema = withParser(Schema.Literal(...GooglePlacesNearbyFieldMask));

export const TrackingPoiSearchCenterSchema = withParser(
  Schema.Struct({
    latitude: LatitudeSchema,
    longitude: LongitudeSchema,
    accuracyMeters: DistanceMetersSchema,
    evidenceReferenceId: ParentEvidenceReferenceIdSchema,
  })
);

export const TrackingGooglePlacesNearbySearchInputSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    provider: Schema.Literal(TrackingPoiProviderId.GooglePlacesNearby),
    requestId: brandedNonEmptyStringSchema('TrackingPoiProviderRequestId'),
    requestedAt: ParentTimestampSchema,
    center: TrackingPoiSearchCenterSchema,
    radiusMeters: RadiusMetersSchema,
    maxResultCount: ResultCountSchema,
    includedTypes: IncludedTypesSchema,
    fieldMask: Schema.Array(GooglePlacesNearbyFieldMaskSchema).pipe(Schema.minItems(1)),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  }).pipe(
    Schema.filter(
      (input) =>
        googlePlacesFieldMaskIsProductionSafe(input.fieldMask) ||
        'Google Places nearby provider requests must use the minimal production field mask and must not use wildcard fields'
    )
  )
);

export const TrackingGooglePlacesNearbyRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    provider: Schema.Literal(TrackingPoiProviderId.GooglePlacesNearby),
    status: Schema.Literal(TrackingPoiProviderStatus.RequestReady),
    requestId: NonEmptyStringSchema,
    method: Schema.Literal('POST'),
    endpointRef: Schema.Literal('places.googleapis.com/v1/places:searchNearby'),
    fieldMaskHeader: NonEmptyStringSchema,
    body: Schema.Struct({
      includedTypes: IncludedTypesSchema,
      maxResultCount: ResultCountSchema,
      locationRestriction: Schema.Struct({
        circle: Schema.Struct({
          center: Schema.Struct({
            latitude: LatitudeSchema,
            longitude: LongitudeSchema,
          }),
          radius: RadiusMetersSchema,
        }),
      }),
    }),
    credentialsStored: Schema.Literal(false),
    liveProviderRequestClaimed: Schema.Literal(false),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
);

const GoogleDisplayNameSchema = Schema.Struct({
  text: NonEmptyStringSchema,
});

const GoogleLocationSchema = Schema.Struct({
  latitude: LatitudeSchema,
  longitude: LongitudeSchema,
});

const GooglePlaceSchema = Schema.Struct({
  id: NonEmptyStringSchema,
  name: NonEmptyStringSchema,
  displayName: GoogleDisplayNameSchema,
  location: GoogleLocationSchema,
  primaryType: NonEmptyStringSchema,
  types: Schema.Array(NonEmptyStringSchema),
});

export const GooglePlacesNearbySearchResponseSchema = withParser(
  Schema.Struct({
    places: Schema.Array(GooglePlaceSchema),
  })
);

export const TrackingPoiCandidateSchema = withParser(
  Schema.Struct({
    providerPlaceId: NonEmptyStringSchema,
    providerResourceName: NonEmptyStringSchema,
    displayName: NonEmptyStringSchema,
    primaryType: NonEmptyStringSchema,
    category: TrackingPoiCategorySchema,
    distanceMeters: DistanceMetersSchema,
    confidence: ConfidenceSchema,
    ambiguityState: TrackingPoiAmbiguityStateSchema,
    evidenceReferenceId: ParentEvidenceReferenceIdSchema,
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
  })
);

export const TrackingPoiProviderReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    provider: TrackingPoiProviderIdSchema,
    status: TrackingPoiProviderStatusSchema,
    generatedAt: ParentTimestampSchema,
    request: TrackingGooglePlacesNearbyRequestSchema,
    candidates: Schema.Array(TrackingPoiCandidateSchema),
    radiusMeters: RadiusMetersSchema,
    fieldMask: Schema.Array(GooglePlacesNearbyFieldMaskSchema),
    providerFailureReason: Schema.Union(TrackingPolicyReasonCodeSchema, Schema.Null),
    locationRestrictionApplied: Schema.Literal(true),
    wildcardFieldMaskRejected: Schema.Literal(true),
    credentialsStored: Schema.Literal(false),
    liveProviderRequestClaimed: Schema.Literal(false),
    exactPlaceClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  }).pipe(
    Schema.filter(
      (model) =>
        model.status !== TrackingPoiProviderStatus.ResponseMapped ||
        model.candidates.length > 0 ||
        'Mapped provider responses need at least one candidate; empty responses must remain no-candidates/manual-required'
    )
  )
);

export const TrackingPoiProviderParityRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    provider: TrackingPoiProviderIdSchema,
    status: TrackingPoiProviderParityStatusSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofRef: TrackingPolicyAuditRefSchema,
    providerTermsReviewRequired: Schema.Boolean,
    providerCredentialsRequired: Schema.Boolean,
    boundedLocationRestrictionRequired: Schema.Literal(true),
    ambiguityPreservedRequired: Schema.Literal(true),
    exactPlaceClaimed: Schema.Literal(false),
    liveProviderRequestClaimed: Schema.Literal(false),
    credentialsStored: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
);

export type TrackingGooglePlacesNearbySearchInput = Infer<typeof TrackingGooglePlacesNearbySearchInputSchema>;
export type TrackingGooglePlacesNearbyRequest = Infer<typeof TrackingGooglePlacesNearbyRequestSchema>;
export type GooglePlacesNearbySearchResponse = Infer<typeof GooglePlacesNearbySearchResponseSchema>;
export type TrackingPoiProviderReadModel = Infer<typeof TrackingPoiProviderReadModelSchema>;
export type TrackingPoiProviderParityRow = Infer<typeof TrackingPoiProviderParityRowSchema>;

export function googlePlacesFieldMaskIsProductionSafe(fieldMask: readonly string[]): boolean {
  return (
    fieldMask.length === GooglePlacesNearbyFieldMask.length &&
    GooglePlacesNearbyFieldMask.every((field) => fieldMask.includes(field)) &&
    fieldMask.every((field) =>
      GooglePlacesNearbyFieldMask.includes(field as (typeof GooglePlacesNearbyFieldMask)[number])
    )
  );
}
