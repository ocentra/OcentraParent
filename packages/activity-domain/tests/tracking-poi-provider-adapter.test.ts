import { describe, expect, it } from 'vitest';
import {
  TrackingGooglePlacesNearbyRequestSchema,
  buildTrackingGooglePlacesNearbyEvidence,
  buildTrackingGooglePlacesNearbyFieldMaskHeader,
  buildTrackingGooglePlacesNearbyUnavailableEvidence,
} from '../src/tracking';

describe('tracking POI provider adapter contracts', () => {
  registersBoundedRequestProof();
  registersNegativeRequestProof();
  registersCandidateMappingProof();
  registersAmbiguityProof();
  registersUnavailableProviderProof();
});

function registersBoundedRequestProof() {
  it('builds a bounded Google Places nearby request with a minimal production field mask', () => {
    const request = TrackingGooglePlacesNearbyRequestSchema.parse(validRequest());

    expect(request.radiusMeters).toBe(1000);
    expect(request.maxResultCount).toBe(5);
    expect(buildTrackingGooglePlacesNearbyFieldMaskHeader(request)).toBe(
      'places.id,places.location,places.primaryType,places.types'
    );
  });
}

function registersNegativeRequestProof() {
  it('rejects wildcard, broad field, unbounded radius, and conflicting place type requests', () => {
    expect(
      TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        fieldMask: ['*'],
      }).success
    ).toBe(false);
    expect(
      TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        fieldMask: ['places.id', 'places.reviews'],
      }).success
    ).toBe(false);
    expect(
      TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        radiusMeters: 0,
      }).success
    ).toBe(false);
    expect(
      TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        radiusMeters: 50_001,
      }).success
    ).toBe(false);
    expect(
      TrackingGooglePlacesNearbyRequestSchema.safeParse({
        ...validRequest(),
        includedTypes: ['school', 'movie_theater'],
        excludedTypes: ['school'],
      }).success
    ).toBe(false);
  });
}

function registersCandidateMappingProof() {
  it('maps a Google Places candidate into evidence without treating nearby POI as exact location', () => {
    const evidence = buildTrackingGooglePlacesNearbyEvidence({
      request: TrackingGooglePlacesNearbyRequestSchema.parse(validRequest()),
      evidenceId: 'google-places-nearby-cinema-proof',
      observedAt: '2026-06-04T10:00:00.000Z',
      response: {
        places: [
          {
            providerRef: 'places/provider-cinema',
            location: { latitude: 43.6538, longitude: -79.3838 },
            primaryType: 'movie_theater',
            types: ['movie_theater', 'point_of_interest'],
            distanceMeters: 118,
          },
        ],
      },
      evidence: [evidenceRef()],
    });

    expect(evidence.providerKind).toBe('google-places');
    expect(evidence.category).toBe('cinema');
    expect(evidence.queryRadiusMeters).toBe(1000);
    expect(evidence.distanceMeters).toBe(118);
    expect(evidence.confidence).toBe(0.82);
    expect(evidence.ambiguityState).toBe('clear');
    expect(evidence.reasonCodes).toContain('google-places-nearby-mapped');
  });
}

function registersAmbiguityProof() {
  it('marks multiple nearby candidates as ambiguous instead of exact-place proof', () => {
    const evidence = buildTrackingGooglePlacesNearbyEvidence({
      request: TrackingGooglePlacesNearbyRequestSchema.parse(validRequest()),
      evidenceId: 'google-places-nearby-ambiguous-proof',
      observedAt: '2026-06-04T10:02:00.000Z',
      response: {
        places: [
          {
            providerRef: 'places/provider-school',
            location: { latitude: 43.654, longitude: -79.384 },
            primaryType: 'school',
            types: ['school'],
            distanceMeters: 142,
          },
          {
            providerRef: 'places/provider-park',
            location: { latitude: 43.6542, longitude: -79.3842 },
            primaryType: 'park',
            types: ['park'],
            distanceMeters: 166,
          },
        ],
      },
      evidence: [evidenceRef()],
    });

    expect(evidence.providerKind).toBe('google-places');
    expect(evidence.category).toBe('school');
    expect(evidence.ambiguityState).toBe('multiple-candidates');
    expect(evidence.confidence).toBe(0.56);
    expect(evidence.reasonCodes).toContain('nearby-place-multiple-candidates');
  });
}

function registersUnavailableProviderProof() {
  it('degrades provider failures to unavailable nearby-place evidence', () => {
    const evidence = buildTrackingGooglePlacesNearbyUnavailableEvidence({
      request: TrackingGooglePlacesNearbyRequestSchema.parse(validRequest()),
      evidenceId: 'google-places-nearby-unavailable-proof',
      observedAt: '2026-06-04T10:03:00.000Z',
      failureReason: 'quota-exhausted',
      evidence: [evidenceRef()],
    });

    expect(evidence.providerKind).toBe('unavailable');
    expect(evidence.providerRef).toBe('google-places-nearby-search');
    expect(evidence.category).toBe('unknown');
    expect(evidence.confidence).toBe(0);
    expect(evidence.ambiguityState).toBe('provider-unavailable');
    expect(evidence.reasonCodes).toEqual(['quota-exhausted', 'provider-degraded']);
  });
}

function validRequest() {
  return {
    schemaVersion: 1,
    requestId: 'google-places-nearby-request-proof',
    requestedAt: '2026-06-04T09:59:00.000Z',
    providerKind: 'google-places',
    providerRef: 'google-places-nearby-search',
    locationEvidenceId: 'location-evidence-1',
    center: {
      latitude: 43.6532,
      longitude: -79.3832,
    },
    radiusMeters: 1000,
    includedTypes: ['school', 'movie_theater', 'park'],
    excludedTypes: ['bar', 'night_club'],
    fieldMask: ['places.id', 'places.location', 'places.primaryType', 'places.types'],
    maxResultCount: 5,
    rankPreference: 'DISTANCE',
    productionUse: true,
    auditRefs: ['google-places-nearby-request-proof'],
  };
}

function evidenceRef() {
  return {
    evidenceId: 'tracking-location-row-1',
    kind: 'journal-entry',
    digest: 'sha256:tracking-poi-provider-proof',
    uri: null,
  };
}
