import {
  GooglePlacesNearbyFieldMask,
  GooglePlacesNearbyFieldMaskSchema,
  GooglePlacesNearbySearchResponseSchema,
  TrackingGooglePlacesNearbyRequestSchema,
  TrackingGooglePlacesNearbySearchInputSchema,
  TrackingPoiAmbiguityState,
  TrackingPoiAmbiguityStateSchema,
  TrackingPoiCandidateSchema,
  TrackingPoiCategory,
  TrackingPoiCategorySchema,
  TrackingPoiProviderId,
  TrackingPoiProviderIdSchema,
  TrackingPoiProviderParityRowSchema,
  TrackingPoiProviderParityStatus,
  TrackingPoiProviderParityStatusSchema,
  TrackingPoiProviderReadModelSchema,
  TrackingPoiProviderStatus,
  TrackingPoiProviderStatusSchema,
  TrackingPoiSearchCenterSchema,
  type GooglePlacesNearbySearchResponse,
  type TrackingGooglePlacesNearbyRequest,
  type TrackingGooglePlacesNearbySearchInput,
  type TrackingPoiProviderParityRow,
  type TrackingPoiProviderReadModel,
  googlePlacesFieldMaskIsProductionSafe,
} from '@ocentra-parent/schema-domain/tracking-poi-provider-adapter';
import { withParser } from '@ocentra-parent/schema-domain/effect';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from '@ocentra-parent/schema-domain/tracking-location-policy-primitives';
type GooglePlace = GooglePlacesNearbySearchResponse['places'][number];
type TrackingPoiAmbiguityStateValue = TrackingPoiProviderReadModel['candidates'][number]['ambiguityState'];
type TrackingPoiCategoryValue = TrackingPoiProviderReadModel['candidates'][number]['category'];
const TrackingPolicyAuditRefParsedSchema = withParser(TrackingPolicyAuditRefSchema);

export {
  GooglePlacesNearbyFieldMask,
  GooglePlacesNearbyFieldMaskSchema,
  GooglePlacesNearbySearchResponseSchema,
  TrackingGooglePlacesNearbyRequestSchema,
  TrackingGooglePlacesNearbySearchInputSchema,
  TrackingPoiAmbiguityState,
  TrackingPoiAmbiguityStateSchema,
  TrackingPoiCandidateSchema,
  TrackingPoiCategory,
  TrackingPoiCategorySchema,
  TrackingPoiProviderId,
  TrackingPoiProviderIdSchema,
  TrackingPoiProviderParityRowSchema,
  TrackingPoiProviderParityStatus,
  TrackingPoiProviderParityStatusSchema,
  TrackingPoiProviderReadModelSchema,
  TrackingPoiProviderStatus,
  TrackingPoiProviderStatusSchema,
  TrackingPoiSearchCenterSchema,
  googlePlacesFieldMaskIsProductionSafe,
};

export type {
  GooglePlacesNearbySearchResponse,
  TrackingGooglePlacesNearbyRequest,
  TrackingGooglePlacesNearbySearchInput,
  TrackingPoiProviderParityRow,
  TrackingPoiProviderReadModel,
};

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

function providerParityRow(input: TrackingPoiProviderParityRow): TrackingPoiProviderParityRow {
  return TrackingPoiProviderParityRowSchema.parse(input);
}

function reasonCode(value: string) {
  return TrackingPolicyReasonCodeSchema.parse(value);
}

function auditRef(value: string) {
  return TrackingPolicyAuditRefParsedSchema.parse(value);
}

function candidateFor(
  input: TrackingGooglePlacesNearbySearchInput,
  place: GooglePlace,
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

