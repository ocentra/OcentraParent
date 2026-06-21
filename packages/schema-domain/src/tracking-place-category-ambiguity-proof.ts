import {
  Schema,
  withParser,
  type Infer,
  NonEmptyStringSchema
} from './effect';
import {
  TrackingPoiAmbiguityState,
  TrackingPoiAmbiguityStateSchema,
  TrackingPoiCategorySchema,
  TrackingPoiProviderReadModelSchema,
  type TrackingPoiCandidateSchema,
  type TrackingPoiProviderReadModel,
} from './tracking-poi-provider-adapter';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';
const TrackingPlaceCategoryReviewStateSchema = withParser(
  Schema.Literal('category-policy-input-only', 'parent-zone-override-review', 'manual-required')
);

export const TrackingPlaceCategorySafeCopySchema = withParser(
  Schema.Struct({
    headline: NonEmptyStringSchema,
    body: NonEmptyStringSchema,
    accusationFree: Schema.Literal(true),
    allowsAutomaticAction: Schema.Literal(false),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
  }).pipe(
    Schema.filter(
      (copy) =>
        trackingPlaceCategoryCopyIsAccusationFree(copy.headline, copy.body) ||
        'Tracking place category copy must stay accusation-free and avoid exact-presence claims'
    )
  )
);

export const TrackingPlaceCategoryAmbiguityReviewSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    providerPlaceId: NonEmptyStringSchema,
    displayName: NonEmptyStringSchema,
    category: TrackingPoiCategorySchema,
    ambiguityState: TrackingPoiAmbiguityStateSchema,
    confidence: Schema.Number.pipe(Schema.between(0, 1)),
    safeCopy: TrackingPlaceCategorySafeCopySchema,
    reviewState: TrackingPlaceCategoryReviewStateSchema,
    categoryCanTriggerActionDirectly: Schema.Literal(false),
    parentDefinedZoneOverrideCanTriggerActionDirectly: Schema.Literal(false),
    requiresPolicyDecision: Schema.Literal(true),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  })
);

export type TrackingPlaceCategorySafeCopy = Infer<typeof TrackingPlaceCategorySafeCopySchema>;
export type TrackingPlaceCategoryAmbiguityReview = Infer<typeof TrackingPlaceCategoryAmbiguityReviewSchema>;

const ForbiddenAccusationFragments = [
  'is at',
  'was at',
  'visited',
  'caught',
  'confirmed at',
  'definitely at',
  'went to',
] as const;

export function buildTrackingPlaceCategoryAmbiguityReviews(
  readModel: TrackingPoiProviderReadModel
): readonly TrackingPlaceCategoryAmbiguityReview[] {
  const parsed = TrackingPoiProviderReadModelSchema.parse(readModel);

  return parsed.candidates.map((candidate) =>
    TrackingPlaceCategoryAmbiguityReviewSchema.parse({
      schemaVersion: TrackingPolicySchemaVersion,
      providerPlaceId: candidate.providerPlaceId,
      displayName: candidate.displayName,
      category: candidate.category,
      ambiguityState: candidate.ambiguityState,
      confidence: candidate.confidence,
      safeCopy: safeCopyFor(candidate),
      reviewState:
        candidate.ambiguityState === TrackingPoiAmbiguityState.SingleCandidate
          ? 'category-policy-input-only'
          : 'manual-required',
      categoryCanTriggerActionDirectly: false,
      parentDefinedZoneOverrideCanTriggerActionDirectly: false,
      requiresPolicyDecision: true,
      auditRefs: parsed.auditRefs,
    })
  );
}

export function trackingPlaceCategoryCopyIsAccusationFree(...parts: readonly string[]): boolean {
  const normalized = parts.join(' ').toLowerCase();
  return ForbiddenAccusationFragments.every((fragment) => !normalized.includes(fragment));
}

function safeCopyFor(candidate: Infer<typeof TrackingPoiCandidateSchema>): TrackingPlaceCategorySafeCopy {
  const ambiguityReason =
    candidate.ambiguityState === TrackingPoiAmbiguityState.LowAccuracy
      ? 'low-accuracy-location'
      : candidate.ambiguityState === TrackingPoiAmbiguityState.MultipleCandidates
        ? 'multiple-nearby-candidates'
        : candidate.ambiguityState === TrackingPoiAmbiguityState.NoCandidates
          ? 'nearby-place-no-candidates'
          : 'nearby-place-category-context';

  return TrackingPlaceCategorySafeCopySchema.parse({
    headline: 'Nearby place context needs review',
    body: `Possible nearby ${candidate.category} context: ${candidate.displayName}. Review location evidence and parent policy before action.`,
    accusationFree: true,
    allowsAutomaticAction: false,
    reasonCodes: [ambiguityReason, 'category-is-policy-input-only'],
  });
}


