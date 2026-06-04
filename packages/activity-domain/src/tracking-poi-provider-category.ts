import { TrackingPlaceRiskCategorySchema, type TrackingPlaceRiskCategory } from './tracking-geofence';

interface TrackingGooglePlaceCategoryCandidate {
  readonly primaryType: string | null;
  readonly types: readonly string[];
}

const GooglePlaceCategoryRules: readonly (readonly [readonly string[], TrackingPlaceRiskCategory])[] = [
  [['school', 'primary_school', 'secondary_school'], 'school'],
  [['hospital'], 'hospital'],
  [['movie_theater'], 'cinema'],
  [['shopping_mall'], 'mall'],
  [['bar'], 'bar'],
  [['night_club'], 'nightclub'],
  [['liquor_store'], 'liquor'],
  [['casino'], 'casino'],
  [['lodging', 'hotel'], 'hotel'],
  [['bus_station', 'train_station', 'subway_station', 'transit_station'], 'transit'],
  [['park'], 'park'],
];

export function categoryForGooglePlace(
  place: TrackingGooglePlaceCategoryCandidate | undefined
): TrackingPlaceRiskCategory {
  const types = googlePlaceTypesFor(place);
  const rule = GooglePlaceCategoryRules.find(([googleTypes]) =>
    googleTypes.some((googleType) => types.includes(googleType))
  );

  return rule?.[1] ?? TrackingPlaceRiskCategorySchema.parse('unknown');
}

function googlePlaceTypesFor(place: TrackingGooglePlaceCategoryCandidate | undefined) {
  const types = [...(place?.types ?? [])];
  if (place?.primaryType !== null && place?.primaryType !== undefined) {
    types.unshift(place.primaryType);
  }
  return types;
}
