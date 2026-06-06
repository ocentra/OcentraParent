import { Schema, withParser, type Infer } from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema, ParentTimestampSchema } from './reference-primitives';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const ProviderText = Schema.String.pipe(Schema.minLength(1));
const LatitudeSchema = Schema.Number.pipe(Schema.between(-90, 90));
const LongitudeSchema = Schema.Number.pipe(Schema.between(-180, 180));
const RadiusMetersSchema = Schema.Number.pipe(Schema.int(), Schema.between(25, 1000));
const ResultCountSchema = Schema.Number.pipe(Schema.int(), Schema.between(1, 20));
const ConfidenceSchema = Schema.Number.pipe(Schema.between(0, 1));
const DistanceMetersSchema = Schema.Number.pipe(Schema.nonNegative());
const IncludedTypesSchema = Schema.Array(ProviderText).pipe(Schema.minItems(1), Schema.maxItems(50));
const TrackingPolicyAuditRefParsedSchema = withParser(TrackingPolicyAuditRefSchema);

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
    requestId: ProviderText.pipe(Schema.brand('TrackingPoiProviderRequestId')),
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
    requestId: ProviderText,
    method: Schema.Literal('POST'),
    endpointRef: Schema.Literal('places.googleapis.com/v1/places:searchNearby'),
    fieldMaskHeader: ProviderText,
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
  text: ProviderText,
});

const GoogleLocationSchema = Schema.Struct({
  latitude: LatitudeSchema,
  longitude: LongitudeSchema,
});

const GooglePlaceSchema = Schema.Struct({
  id: ProviderText,
  name: ProviderText,
  displayName: GoogleDisplayNameSchema,
  location: GoogleLocationSchema,
  primaryType: ProviderText,
  types: Schema.Array(ProviderText),
});

export const GooglePlacesNearbySearchResponseSchema = withParser(
  Schema.Struct({
    places: Schema.Array(GooglePlaceSchema),
  })
);

export const TrackingPoiCandidateSchema = withParser(
  Schema.Struct({
    providerPlaceId: ProviderText,
    providerResourceName: ProviderText,
    displayName: ProviderText,
    primaryType: ProviderText,
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
type TrackingPoiAmbiguityStateValue = Infer<typeof TrackingPoiAmbiguityStateSchema>;
type TrackingPoiCategoryValue = Infer<typeof TrackingPoiCategorySchema>;

export function buildGooglePlacesNearbySearchRequest(
  input: TrackingGooglePlacesNearbySearchInput
): TrackingGooglePlacesNearbyRequest {
  const parsed = TrackingGooglePlacesNearbySearchInputSchema.parse(input);

  return TrackingGooglePlacesNearbyRequestSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    provider: parsed.provider,
    status: TrackingPoiProviderStatus.RequestReady,
    requestId: parsed.requestId,
    method: 'POST',
    endpointRef: 'places.googleapis.com/v1/places:searchNearby',
    fieldMaskHeader: parsed.fieldMask.join(','),
    body: {
      includedTypes: parsed.includedTypes,
      maxResultCount: parsed.maxResultCount,
      locationRestriction: {
        circle: {
          center: {
            latitude: parsed.center.latitude,
            longitude: parsed.center.longitude,
          },
          radius: parsed.radiusMeters,
        },
      },
    },
    credentialsStored: false,
    liveProviderRequestClaimed: false,
    reasonCodes: ['google-places-nearby-request-ready'],
    auditRefs: parsed.auditRefs,
  });
}

export function buildGooglePlacesNearbyReadModel(
  input: TrackingGooglePlacesNearbySearchInput,
  response: GooglePlacesNearbySearchResponse
): TrackingPoiProviderReadModel {
  const parsedInput = TrackingGooglePlacesNearbySearchInputSchema.parse(input);
  const parsedResponse = GooglePlacesNearbySearchResponseSchema.parse(response);
  const request = buildGooglePlacesNearbySearchRequest(parsedInput);
  const ambiguityState = ambiguityFor(parsedInput.center.accuracyMeters, parsedResponse.places.length);
  const candidates = parsedResponse.places.map((place) => candidateFor(parsedInput, place, ambiguityState));

  return TrackingPoiProviderReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    provider: parsedInput.provider,
    status: candidates.length > 0 ? TrackingPoiProviderStatus.ResponseMapped : TrackingPoiProviderStatus.ManualRequired,
    generatedAt: parsedInput.requestedAt,
    request,
    candidates,
    radiusMeters: parsedInput.radiusMeters,
    fieldMask: parsedInput.fieldMask,
    providerFailureReason: candidates.length > 0 ? null : 'nearby-place-no-candidates',
    locationRestrictionApplied: true,
    wildcardFieldMaskRejected: true,
    credentialsStored: false,
    liveProviderRequestClaimed: false,
    exactPlaceClaimed: false,
    physicalDeviceProofClaimed: false,
    reasonCodes:
      candidates.length > 0
        ? ['google-places-response-mapped', 'nearby-place-ambiguity-preserved']
        : ['nearby-place-no-candidates'],
    auditRefs: parsedInput.auditRefs,
  });
}

export function buildGooglePlacesProviderFailureReadModel(
  input: TrackingGooglePlacesNearbySearchInput,
  reason: string
): TrackingPoiProviderReadModel {
  const parsed = TrackingGooglePlacesNearbySearchInputSchema.parse(input);

  return TrackingPoiProviderReadModelSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    provider: parsed.provider,
    status: TrackingPoiProviderStatus.ProviderUnavailable,
    generatedAt: parsed.requestedAt,
    request: buildGooglePlacesNearbySearchRequest(parsed),
    candidates: [],
    radiusMeters: parsed.radiusMeters,
    fieldMask: parsed.fieldMask,
    providerFailureReason: reason,
    locationRestrictionApplied: true,
    wildcardFieldMaskRejected: true,
    credentialsStored: false,
    liveProviderRequestClaimed: false,
    exactPlaceClaimed: false,
    physicalDeviceProofClaimed: false,
    reasonCodes: ['google-places-provider-unavailable', reason],
    auditRefs: parsed.auditRefs,
  });
}

export function buildTrackingPoiProviderParityRows(
  readModel: TrackingPoiProviderReadModel
): readonly TrackingPoiProviderParityRow[] {
  const parsed = TrackingPoiProviderReadModelSchema.parse(readModel);
  const sourceProofRef = parsed.auditRefs[0] ?? auditRef('nearby-place-provider-proof');

  return [
    providerParityRow({
      schemaVersion: TrackingPolicySchemaVersion,
      provider: TrackingPoiProviderId.GooglePlacesNearby,
      status: TrackingPoiProviderParityStatus.RequestMapped,
      generatedAt: parsed.generatedAt,
      sourceProofRef,
      providerTermsReviewRequired: false,
      providerCredentialsRequired: true,
      boundedLocationRestrictionRequired: true,
      ambiguityPreservedRequired: true,
      exactPlaceClaimed: false,
      liveProviderRequestClaimed: false,
      credentialsStored: false,
      physicalDeviceProofClaimed: false,
      reasonCodes: [
        reasonCode('google-places-provider-contract-ready'),
        reasonCode('nearby-place-ambiguity-preserved'),
      ],
      auditRefs: parsed.auditRefs,
    }),
    providerParityRow({
      schemaVersion: TrackingPolicySchemaVersion,
      provider: TrackingPoiProviderId.AppleMapKitSearch,
      status: TrackingPoiProviderParityStatus.ManualRequired,
      generatedAt: parsed.generatedAt,
      sourceProofRef,
      providerTermsReviewRequired: true,
      providerCredentialsRequired: true,
      boundedLocationRestrictionRequired: true,
      ambiguityPreservedRequired: true,
      exactPlaceClaimed: false,
      liveProviderRequestClaimed: false,
      credentialsStored: false,
      physicalDeviceProofClaimed: false,
      reasonCodes: [
        reasonCode('apple-mapkit-provider-parity-manual-required'),
        reasonCode('provider-runtime-not-proved'),
      ],
      auditRefs: [auditRef('apple-mapkit-provider-parity-required'), ...parsed.auditRefs],
    }),
    providerParityRow({
      schemaVersion: TrackingPolicySchemaVersion,
      provider: TrackingPoiProviderId.OpenStreetMapNominatim,
      status: TrackingPoiProviderParityStatus.ManualRequired,
      generatedAt: parsed.generatedAt,
      sourceProofRef,
      providerTermsReviewRequired: true,
      providerCredentialsRequired: false,
      boundedLocationRestrictionRequired: true,
      ambiguityPreservedRequired: true,
      exactPlaceClaimed: false,
      liveProviderRequestClaimed: false,
      credentialsStored: false,
      physicalDeviceProofClaimed: false,
      reasonCodes: [reasonCode('osm-provider-parity-manual-required'), reasonCode('provider-runtime-not-proved')],
      auditRefs: [auditRef('osm-provider-parity-required'), ...parsed.auditRefs],
    }),
  ];
}

export function googlePlacesFieldMaskIsProductionSafe(fieldMask: readonly string[]): boolean {
  return (
    fieldMask.length === GooglePlacesNearbyFieldMask.length &&
    GooglePlacesNearbyFieldMask.every((field) => fieldMask.includes(field)) &&
    fieldMask.every((field) =>
      GooglePlacesNearbyFieldMask.includes(field as (typeof GooglePlacesNearbyFieldMask)[number])
    )
  );
}

function providerParityRow(input: TrackingPoiProviderParityRow): TrackingPoiProviderParityRow {
  return TrackingPoiProviderParityRowSchema.parse(input);
}

function reasonCode(value: string): Infer<typeof TrackingPolicyReasonCodeSchema> {
  return TrackingPolicyReasonCodeSchema.parse(value);
}

function auditRef(value: string): Infer<typeof TrackingPolicyAuditRefSchema> {
  return TrackingPolicyAuditRefParsedSchema.parse(value);
}

function candidateFor(
  input: TrackingGooglePlacesNearbySearchInput,
  place: Infer<typeof GooglePlaceSchema>,
  ambiguityState: TrackingPoiAmbiguityStateValue
) {
  const distanceMeters = distanceBetweenMeters(input.center, place.location);
  return TrackingPoiCandidateSchema.parse({
    providerPlaceId: place.id,
    providerResourceName: place.name,
    displayName: place.displayName.text,
    primaryType: place.primaryType,
    category: categoryFor([place.primaryType, ...place.types]),
    distanceMeters,
    confidence: confidenceFor(distanceMeters, input.radiusMeters, ambiguityState),
    ambiguityState,
    evidenceReferenceId: input.center.evidenceReferenceId,
    reasonCodes: ['google-places-candidate-mapped', `google-primary-type-${place.primaryType}`],
  });
}

function ambiguityFor(accuracyMeters: number, candidateCount: number): TrackingPoiAmbiguityStateValue {
  if (candidateCount === 0) {
    return TrackingPoiAmbiguityState.NoCandidates;
  }
  if (accuracyMeters > 75) {
    return TrackingPoiAmbiguityState.LowAccuracy;
  }
  if (candidateCount > 1) {
    return TrackingPoiAmbiguityState.MultipleCandidates;
  }
  return TrackingPoiAmbiguityState.SingleCandidate;
}

function confidenceFor(
  distanceMeters: number,
  radiusMeters: number,
  ambiguityState: TrackingPoiAmbiguityStateValue
): number {
  const distanceScore = Math.max(0.1, 1 - distanceMeters / radiusMeters);
  const ambiguityPenalty =
    ambiguityState === TrackingPoiAmbiguityState.SingleCandidate
      ? 1
      : ambiguityState === TrackingPoiAmbiguityState.MultipleCandidates
        ? 0.72
        : ambiguityState === TrackingPoiAmbiguityState.LowAccuracy
          ? 0.5
          : 0.2;

  return Number(Math.max(0.1, Math.min(0.95, distanceScore * ambiguityPenalty)).toFixed(2));
}

function categoryFor(types: readonly string[]): TrackingPoiCategoryValue {
  if (types.some((type) => type.includes('school'))) {
    return TrackingPoiCategory.School;
  }
  if (types.some((type) => type.includes('restaurant') || type.includes('cafe') || type.includes('food'))) {
    return TrackingPoiCategory.Food;
  }
  if (types.some((type) => type.includes('store') || type.includes('shopping'))) {
    return TrackingPoiCategory.Store;
  }
  if (types.some((type) => type.includes('transit') || type.includes('bus') || type.includes('train'))) {
    return TrackingPoiCategory.Transit;
  }
  if (types.some((type) => type.includes('hospital') || type.includes('doctor') || type.includes('pharmacy'))) {
    return TrackingPoiCategory.Healthcare;
  }
  if (types.some((type) => type.includes('bar') || type.includes('casino') || type.includes('liquor'))) {
    return TrackingPoiCategory.Sensitive;
  }
  return TrackingPoiCategory.Unknown;
}

function distanceBetweenMeters(
  from: { readonly latitude: number; readonly longitude: number },
  to: { readonly latitude: number; readonly longitude: number }
): number {
  const earthRadiusMeters = 6_371_000;
  const fromLatitude = radians(from.latitude);
  const toLatitude = radians(to.latitude);
  const latitudeDelta = radians(to.latitude - from.latitude);
  const longitudeDelta = radians(to.longitude - from.longitude);
  const haversine =
    Math.sin(latitudeDelta / 2) ** 2 +
    Math.cos(fromLatitude) * Math.cos(toLatitude) * Math.sin(longitudeDelta / 2) ** 2;

  return Math.round(earthRadiusMeters * 2 * Math.atan2(Math.sqrt(haversine), Math.sqrt(1 - haversine)));
}

function radians(value: number): number {
  return (value * Math.PI) / 180;
}
