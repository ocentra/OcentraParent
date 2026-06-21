import { describe, expect, it } from 'vitest';
import {
  GooglePlacesNearbyFieldMask,
  TrackingGooglePlacesNearbySearchInputSchema,
  TrackingPoiProviderStatus,
  TrackingPoiProviderReadModelSchema,
  buildGooglePlacesNearbyReadModel,
  buildGooglePlacesNearbySearchRequest,
  buildGooglePlacesProviderFailureReadModel,
  buildTrackingPoiProviderParityRows,
  googlePlacesFieldMaskIsProductionSafe,
} from '../../src/tracking-poi-provider-adapter';
import { TrackingPolicySchemaVersion } from '@ocentra-parent/schema-domain/tracking-location-policy';

const Timestamp = '2026-06-05T14:45:00.000Z';
const SearchInput = {
  schemaVersion: TrackingPolicySchemaVersion,
  provider: 'google-places-nearby',
  requestId: 'tracking-google-places-request-1',
  requestedAt: Timestamp,
  center: {
    latitude: 43.6532,
    longitude: -79.3832,
    accuracyMeters: 22,
    evidenceReferenceId: 'location-evidence-poi-query-1',
  },
  radiusMeters: 250,
  maxResultCount: 5,
  includedTypes: ['school', 'restaurant', 'transit_station'],
  fieldMask: GooglePlacesNearbyFieldMask,
  auditRefs: ['google-places-field-mask-reviewed', 'nearby-place-provider-proof'],
} as const;

const GoogleResponse = {
  places: [
    {
      id: 'google-place-school-1',
      name: 'places/google-place-school-1',
      displayName: { text: 'Central School' },
      location: { latitude: 43.65335, longitude: -79.383 },
      primaryType: 'school',
      types: ['school', 'point_of_interest', 'establishment'],
    },
    {
      id: 'google-place-cafe-1',
      name: 'places/google-place-cafe-1',
      displayName: { text: 'Corner Cafe' },
      location: { latitude: 43.654, longitude: -79.3829 },
      primaryType: 'cafe',
      types: ['cafe', 'food', 'point_of_interest', 'establishment'],
    },
  ],
} as const;

describe('tracking POI provider adapter', () => {
  it('builds a bounded Google Places Nearby Search request with a production field mask', () => {
    const request = buildGooglePlacesNearbySearchRequest(SearchInput);

    expect(request.method).toBe('POST');
    expect(request.endpointRef).toBe('places.googleapis.com/v1/places:searchNearby');
    expect(request.fieldMaskHeader).toBe(
      'places.id,places.name,places.displayName.text,places.location,places.primaryType,places.types'
    );
    expect(request.body.locationRestriction.circle.radius).toBe(250);
    expect(request.body.locationRestriction.circle.center).toEqual({ latitude: 43.6532, longitude: -79.3832 });
    expect(request.body.includedTypes).toEqual(['school', 'restaurant', 'transit_station']);
    expect(request.credentialsStored).toBe(false);
    expect(request.liveProviderRequestClaimed).toBe(false);
  });

  it('maps provider response rows to nearby-place candidates while preserving ambiguity', () => {
    const readModel = buildGooglePlacesNearbyReadModel(SearchInput, GoogleResponse);

    expect(readModel.status).toBe(TrackingPoiProviderStatus.ResponseMapped);
    expect(readModel.candidates).toHaveLength(2);
    expect(readModel.candidates.map((candidate) => candidate.category)).toEqual(['school', 'food']);
    expect(readModel.candidates.map((candidate) => candidate.ambiguityState)).toEqual([
      'multiple-candidates',
      'multiple-candidates',
    ]);
    expect(readModel.candidates[0]?.distanceMeters).toBeGreaterThan(0);
    expect(readModel.candidates[0]?.confidence).toBeGreaterThan(0.5);
    expect(readModel.locationRestrictionApplied).toBe(true);
    expect(readModel.exactPlaceClaimed).toBe(false);
    expect(readModel.physicalDeviceProofClaimed).toBe(false);
  });

  it('degrades provider failures without pretending live provider execution happened', () => {
    const readModel = buildGooglePlacesProviderFailureReadModel(SearchInput, 'google-places-quota-unavailable');

    expect(readModel.status).toBe(TrackingPoiProviderStatus.ProviderUnavailable);
    expect(readModel.candidates).toEqual([]);
    expect(readModel.providerFailureReason).toBe('google-places-quota-unavailable');
    expect(readModel.liveProviderRequestClaimed).toBe(false);
    expect(readModel.credentialsStored).toBe(false);
    expect(readModel.reasonCodes).toEqual(['google-places-provider-unavailable', 'google-places-quota-unavailable']);
  });

  it('rejects wildcard masks, unbounded radius, empty included types, and upgraded claims', () => {
    expect(googlePlacesFieldMaskIsProductionSafe(['*'])).toBe(false);
    expect(
      TrackingGooglePlacesNearbySearchInputSchema.safeParse({
        ...SearchInput,
        fieldMask: ['*'],
      }).success
    ).toBe(false);
    expect(
      TrackingGooglePlacesNearbySearchInputSchema.safeParse({
        ...SearchInput,
        radiusMeters: 5001,
      }).success
    ).toBe(false);
    expect(
      TrackingGooglePlacesNearbySearchInputSchema.safeParse({
        ...SearchInput,
        includedTypes: [],
      }).success
    ).toBe(false);
    expect(
      TrackingPoiProviderReadModelSchema.safeParse({
        ...buildGooglePlacesNearbyReadModel(SearchInput, GoogleResponse),
        liveProviderRequestClaimed: true,
      }).success
    ).toBe(false);
  });
});

describe('tracking POI provider parity readiness', () => {
  it('keeps provider parity rows explicit without upgrading Apple or OSM claims', () => {
    const parityRows = buildTrackingPoiProviderParityRows(
      buildGooglePlacesNearbyReadModel(SearchInput, GoogleResponse)
    );

    expect(parityRows.map((row) => row.provider)).toEqual([
      'google-places-nearby',
      'apple-mapkit-search',
      'openstreetmap-nominatim',
    ]);
    expect(parityRows.map((row) => row.status)).toEqual(['request-mapped', 'manual-required', 'manual-required']);
    expect(parityRows.map((row) => row.liveProviderRequestClaimed)).toEqual([false, false, false]);
    expect(parityRows.map((row) => row.exactPlaceClaimed)).toEqual([false, false, false]);
    expect(parityRows.map((row) => row.physicalDeviceProofClaimed)).toEqual([false, false, false]);
    expect(parityRows[1]?.reasonCodes).toContain('apple-mapkit-provider-parity-manual-required');
    expect(parityRows[2]?.reasonCodes).toContain('osm-provider-parity-manual-required');
  });
});
