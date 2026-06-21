import { describe, expect, it } from 'vitest';
import { GooglePlacesNearbyFieldMask, buildGooglePlacesNearbyReadModel } from '../../src/tracking-poi-provider-adapter';
import {
  TrackingPlaceCategoryAmbiguityReviewSchema,
  TrackingPlaceCategorySafeCopySchema,
  buildTrackingPlaceCategoryAmbiguityReviews,
  trackingPlaceCategoryCopyIsAccusationFree,
} from '../../src/tracking-place-category-ambiguity-proof';
import { TrackingPolicySchemaVersion } from '@ocentra-parent/schema-domain/tracking-location-policy';

const Timestamp = '2026-06-06T18:14:00.000Z';
const SearchInput = {
  schemaVersion: TrackingPolicySchemaVersion,
  provider: 'google-places-nearby',
  requestId: 'tracking-google-places-category-proof-1',
  requestedAt: Timestamp,
  center: {
    latitude: 43.6532,
    longitude: -79.3832,
    accuracyMeters: 22,
    evidenceReferenceId: 'location-evidence-category-query-1',
  },
  radiusMeters: 250,
  maxResultCount: 5,
  includedTypes: ['school', 'bar', 'restaurant'],
  fieldMask: GooglePlacesNearbyFieldMask,
  auditRefs: ['place-category-ambiguity-proof'],
} as const;

const MultipleCandidateResponse = {
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
      id: 'google-place-bar-1',
      name: 'places/google-place-bar-1',
      displayName: { text: 'Market Bar' },
      location: { latitude: 43.654, longitude: -79.3829 },
      primaryType: 'bar',
      types: ['bar', 'point_of_interest', 'establishment'],
    },
  ],
} as const;

const SingleCandidateResponse = {
  places: [MultipleCandidateResponse.places[0]],
} as const;

describe('tracking place category ambiguity proof', () => {
  it('turns multiple nearby places into accusation-free review rows only', () => {
    const reviews = buildTrackingPlaceCategoryAmbiguityReviews(
      buildGooglePlacesNearbyReadModel(SearchInput, MultipleCandidateResponse)
    );

    expect(reviews).toHaveLength(2);
    expect(reviews.map((review) => review.category)).toEqual(['school', 'sensitive']);
    expect(reviews.map((review) => review.ambiguityState)).toEqual(['multiple-candidates', 'multiple-candidates']);
    expect(reviews.map((review) => review.reviewState)).toEqual(['manual-required', 'manual-required']);
    expect(reviews.every((review) => review.categoryCanTriggerActionDirectly === false)).toBe(true);
    expect(reviews.every((review) => review.parentDefinedZoneOverrideCanTriggerActionDirectly === false)).toBe(true);
    expect(reviews.every((review) => review.safeCopy.accusationFree)).toBe(true);
    expect(reviews.every((review) => trackingPlaceCategoryCopyIsAccusationFree(review.safeCopy.body))).toBe(true);
  });

  it('keeps low accuracy category matches as policy input requiring review', () => {
    const lowAccuracyInput = {
      ...SearchInput,
      center: {
        ...SearchInput.center,
        accuracyMeters: 130,
      },
    } as const;
    const reviews = buildTrackingPlaceCategoryAmbiguityReviews(
      buildGooglePlacesNearbyReadModel(lowAccuracyInput, SingleCandidateResponse)
    );

    expect(reviews).toHaveLength(1);
    expect(reviews[0]?.ambiguityState).toBe('low-accuracy');
    expect(reviews[0]?.reviewState).toBe('manual-required');
    expect(reviews[0]?.safeCopy.reasonCodes).toContain('low-accuracy-location');
    expect(reviews[0]?.requiresPolicyDecision).toBe(true);
  });

  it('rejects accusation copy and upgraded direct-action claims', () => {
    expect(
      TrackingPlaceCategorySafeCopySchema.safeParse({
        headline: 'Child is at a restricted place',
        body: 'The child definitely at Market Bar.',
        accusationFree: true,
        allowsAutomaticAction: false,
        reasonCodes: ['unsafe-copy-negative-case'],
      }).success
    ).toBe(false);
    expect(
      TrackingPlaceCategoryAmbiguityReviewSchema.safeParse({
        ...buildTrackingPlaceCategoryAmbiguityReviews(
          buildGooglePlacesNearbyReadModel(SearchInput, MultipleCandidateResponse)
        )[0],
        categoryCanTriggerActionDirectly: true,
      }).success
    ).toBe(false);
  });
});
