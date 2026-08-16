/* generated from crates/browser-core/src/browser_generated_social_ts.rs */

import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserSocialPlatformSchema,
  type BrowserSocialRouteEvidence,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRouteEvidenceSchema,
  BrowserSocialRouteKindSchema,
} from './generated-browser-social-platform-route-schemas';
const OptionalSocialVideoMetadataTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const OptionalSocialVideoMetadataTimestampSchema = Schema.Union(ActivityTimestampSchema, Schema.Null);
const OptionalSocialVideoMetadataDurationSchema = Schema.Union(
  Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  Schema.Null
);
const SocialVideoMetadataSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social video metadata source evidence ids')
);

export const BrowserSocialVideoMetadataSchemaVersion = 1;

export const BrowserSocialVideoMetadataEvidenceIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialVideoMetadataEvidenceId')
);

export const BrowserSocialVideoMetadataSourceKindSchema = withParser(
  Schema.Literal('platform-page-metadata', 'open-graph', 'schema-org-video-object', 'manual-required')
);

export const BrowserSocialVideoMetadataStateSchema = withParser(
  Schema.Literal('available', 'partial', 'manual-required')
);

const BrowserSocialVideoMetadataExtractorInputBaseSchema = Schema.Struct({
  metadataEvidenceId: BrowserSocialVideoMetadataEvidenceIdSchema,
  collectedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialVideoMetadataSourceEvidenceIdsSchema,
  routeEvidence: BrowserSocialRouteEvidenceSchema,
  sourceKind: BrowserSocialVideoMetadataSourceKindSchema,
  titleRef: OptionalSocialVideoMetadataTextSchema,
  descriptionRef: OptionalSocialVideoMetadataTextSchema,
  authorHashRef: OptionalSocialVideoMetadataTextSchema,
  thumbnailHashRef: OptionalSocialVideoMetadataTextSchema,
  durationSeconds: OptionalSocialVideoMetadataDurationSchema,
  publishedAt: OptionalSocialVideoMetadataTimestampSchema,
  categoryRef: OptionalSocialVideoMetadataTextSchema,
  restrictionSignalRef: OptionalSocialVideoMetadataTextSchema,
});

const BrowserSocialVideoMetadataExtractorInputSchema = withParser(
  BrowserSocialVideoMetadataExtractorInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialVideoMetadataExtractorInputIsConsistent(value) ||
        'Expected managed social video/post/feed route evidence and bounded metadata refs'
    )
  )
);

const BrowserSocialVideoMetadataEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialVideoMetadataSchemaVersion),
  metadataEvidenceId: BrowserSocialVideoMetadataEvidenceIdSchema,
  collectedAt: ActivityTimestampSchema,
  sourceEvidenceIds: SocialVideoMetadataSourceEvidenceIdsSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  platform: BrowserSocialPlatformSchema,
  routeKind: BrowserSocialRouteKindSchema,
  sourceKind: BrowserSocialVideoMetadataSourceKindSchema,
  metadataState: BrowserSocialVideoMetadataStateSchema,
  titleRef: OptionalSocialVideoMetadataTextSchema,
  descriptionRef: OptionalSocialVideoMetadataTextSchema,
  authorHashRef: OptionalSocialVideoMetadataTextSchema,
  thumbnailHashRef: OptionalSocialVideoMetadataTextSchema,
  durationSeconds: OptionalSocialVideoMetadataDurationSchema,
  publishedAt: OptionalSocialVideoMetadataTimestampSchema,
  categoryRef: OptionalSocialVideoMetadataTextSchema,
  restrictionSignalRef: OptionalSocialVideoMetadataTextSchema,
  pageBodyCaptured: Schema.Boolean,
  transcriptTextCaptured: Schema.Boolean,
  messageContentCaptured: Schema.Boolean,
  feedContentSemanticsClaimed: Schema.Boolean,
  contentSemanticsClaimed: Schema.Boolean,
  aiDecisionClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
});

export const BrowserSocialVideoMetadataEvidenceSchema = withParser(
  BrowserSocialVideoMetadataEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserSocialVideoMetadataEvidenceIsConsistent(value) ||
        'Expected social video metadata evidence to preserve bounded metadata boundaries'
    )
  )
);

export const decodeBrowserSocialVideoMetadataEvidence = Schema.decodeUnknownSync(
  BrowserSocialVideoMetadataEvidenceSchema
);

export type BrowserSocialVideoMetadataEvidence = Infer<typeof BrowserSocialVideoMetadataEvidenceSchema>;
export type BrowserSocialVideoMetadataEvidenceId = Infer<typeof BrowserSocialVideoMetadataEvidenceIdSchema>;
export type BrowserSocialVideoMetadataExtractorInput = Infer<typeof BrowserSocialVideoMetadataExtractorInputSchema>;
export type BrowserSocialVideoMetadataSourceKind = Infer<typeof BrowserSocialVideoMetadataSourceKindSchema>;
export type BrowserSocialVideoMetadataState = Infer<typeof BrowserSocialVideoMetadataStateSchema>;

export function extractBrowserSocialVideoMetadata(
  input: BrowserSocialVideoMetadataExtractorInput
): BrowserSocialVideoMetadataEvidence {
  const parsed = BrowserSocialVideoMetadataExtractorInputSchema.parse(input);

  return BrowserSocialVideoMetadataEvidenceSchema.parse({
    schemaVersion: BrowserSocialVideoMetadataSchemaVersion,
    metadataEvidenceId: parsed.metadataEvidenceId,
    collectedAt: parsed.collectedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    socialRouteEvidenceId: parsed.routeEvidence.socialRouteEvidenceId,
    platform: parsed.routeEvidence.platform,
    routeKind: parsed.routeEvidence.routeKind,
    sourceKind: parsed.sourceKind,
    metadataState: metadataStateForInput(parsed),
    titleRef: parsed.titleRef,
    descriptionRef: parsed.descriptionRef,
    authorHashRef: parsed.authorHashRef,
    thumbnailHashRef: parsed.thumbnailHashRef,
    durationSeconds: parsed.durationSeconds,
    publishedAt: parsed.publishedAt,
    categoryRef: parsed.categoryRef,
    restrictionSignalRef: parsed.restrictionSignalRef,
    pageBodyCaptured: false,
    transcriptTextCaptured: false,
    messageContentCaptured: false,
    feedContentSemanticsClaimed: false,
    contentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    enforcementClaimed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  });
}

function socialVideoMetadataExtractorInputIsConsistent(
  value: Infer<typeof BrowserSocialVideoMetadataExtractorInputBaseSchema>
) {
  if (!routeEvidenceCanExtractMetadata(value.routeEvidence)) {
    return false;
  }
  if (value.sourceKind === 'manual-required') {
    return !hasAnyMetadataRef(value);
  }
  return hasAnyMetadataRef(value);
}

function browserSocialVideoMetadataEvidenceIsConsistent(
  value: Infer<typeof BrowserSocialVideoMetadataEvidenceBaseSchema>
) {
  if (socialVideoMetadataEvidenceClaimsAuthority(value)) {
    return false;
  }
  if (value.sourceKind === 'manual-required') {
    return value.metadataState === 'manual-required' && !hasAnyEvidenceMetadataRef(value);
  }
  return value.metadataState !== 'manual-required' && hasAnyEvidenceMetadataRef(value);
}

function socialVideoMetadataEvidenceClaimsAuthority(value: Infer<typeof BrowserSocialVideoMetadataEvidenceBaseSchema>) {
  return (
    value.pageBodyCaptured ||
    value.transcriptTextCaptured ||
    value.messageContentCaptured ||
    value.feedContentSemanticsClaimed ||
    value.contentSemanticsClaimed ||
    value.aiDecisionClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed
  );
}

function routeEvidenceCanExtractMetadata(value: BrowserSocialRouteEvidence) {
  return (
    value.sourceKind === 'managed-browser-url-shape' &&
    value.exactManagedBrowserRouteEvidence &&
    value.proofState === 'route-evidence' &&
    (value.routeKind === 'video' || value.routeKind === 'post' || value.routeKind === 'feed')
  );
}

function metadataStateForInput(value: Infer<typeof BrowserSocialVideoMetadataExtractorInputBaseSchema>) {
  if (value.sourceKind === 'manual-required') {
    return 'manual-required' as const;
  }
  return value.titleRef !== null && value.thumbnailHashRef !== null ? 'available' : 'partial';
}

function hasAnyMetadataRef(value: Infer<typeof BrowserSocialVideoMetadataExtractorInputBaseSchema>) {
  return (
    value.titleRef !== null ||
    value.descriptionRef !== null ||
    value.authorHashRef !== null ||
    value.thumbnailHashRef !== null ||
    value.durationSeconds !== null ||
    value.publishedAt !== null ||
    value.categoryRef !== null ||
    value.restrictionSignalRef !== null
  );
}

function hasAnyEvidenceMetadataRef(value: Infer<typeof BrowserSocialVideoMetadataEvidenceBaseSchema>) {
  return (
    value.titleRef !== null ||
    value.descriptionRef !== null ||
    value.authorHashRef !== null ||
    value.thumbnailHashRef !== null ||
    value.durationSeconds !== null ||
    value.publishedAt !== null ||
    value.categoryRef !== null ||
    value.restrictionSignalRef !== null
  );
}
