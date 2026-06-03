import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { BrowserPageTitleSchema, BrowserUrlSchema } from './browser-schemas';
import { BrowserUrlShapeClassificationIdSchema } from './browser-url-intelligence-schemas';

export const BrowserUrlMetadataSchemaVersion = 1;

const NonEmptyBrowserUrlMetadataText = Schema.String.pipe(Schema.minLength(1));
const OptionalMetadataTextSchema = Schema.Union(NonEmptyBrowserUrlMetadataText, Schema.Null);

export const BrowserUrlMetadataEvidenceIdSchema = withParser(
  NonEmptyBrowserUrlMetadataText.pipe(Schema.brand('BrowserUrlMetadataEvidenceId'))
);

export const BrowserUrlMetadataSourceKindSchema = withParser(
  Schema.Literal(
    'managed-browser-title',
    'open-graph',
    'schema-org-article',
    'schema-org-video-object',
    'platform-page-metadata',
    'platform-api',
    'hidden-managed-analysis'
  )
);

export const BrowserUrlMetadataStateSchema = withParser(
  Schema.Literal('available', 'partial', 'degraded', 'manual-required', 'unavailable')
);

export const BrowserUrlMetadataDegradedReasonSchema = withParser(
  Schema.Literal(
    'source-missing',
    'source-stale',
    'metadata-empty',
    'metadata-conflicting',
    'hidden-analysis-not-proved',
    'platform-restricted',
    'permission-limited',
    'network-error',
    'manual-required'
  )
);

const OptionalMetadataUrlSchema = Schema.Union(BrowserUrlSchema, Schema.Null);
const OptionalMetadataDurationSecondsSchema = Schema.Union(
  Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  Schema.Null
);
const OptionalMetadataTimestampSchema = Schema.Union(ActivityTimestampSchema, Schema.Null);
const OptionalMetadataBooleanSchema = Schema.Union(Schema.Boolean, Schema.Null);

export const BrowserUrlMetadataSourceSchema = Schema.Struct({
  sourceKind: BrowserUrlMetadataSourceKindSchema,
  sourceRef: OptionalMetadataTextSchema,
  hiddenAnalysisProofRef: OptionalMetadataTextSchema,
});

export const BrowserUrlMetadataFieldsSchema = Schema.Struct({
  browserTitle: Schema.Union(BrowserPageTitleSchema, Schema.Null),
  openGraphTitle: OptionalMetadataTextSchema,
  openGraphDescription: OptionalMetadataTextSchema,
  schemaOrgType: Schema.Union(Schema.Literal('article', 'video-object'), Schema.Null),
  platformVideoId: OptionalMetadataTextSchema,
  platformChannelId: OptionalMetadataTextSchema,
  channelName: OptionalMetadataTextSchema,
  thumbnailUrl: OptionalMetadataUrlSchema,
  thumbnailHashRef: OptionalMetadataTextSchema,
  durationSeconds: OptionalMetadataDurationSecondsSchema,
  publishDate: OptionalMetadataTimestampSchema,
  captionsAvailable: OptionalMetadataBooleanSchema,
  transcriptAvailable: OptionalMetadataBooleanSchema,
  platformCategory: OptionalMetadataTextSchema,
  platformRating: OptionalMetadataTextSchema,
  restrictedSignal: OptionalMetadataTextSchema,
});

const BrowserUrlMetadataEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserUrlMetadataSchemaVersion),
  metadataEvidenceId: BrowserUrlMetadataEvidenceIdSchema,
  collectedAt: ActivityTimestampSchema,
  sourceEvidenceIds: Schema.Array(ActivityEvidenceIdSchema),
  urlShapeClassificationId: BrowserUrlShapeClassificationIdSchema,
  source: BrowserUrlMetadataSourceSchema,
  metadataState: BrowserUrlMetadataStateSchema,
  fields: BrowserUrlMetadataFieldsSchema,
  degradedReasons: Schema.Array(BrowserUrlMetadataDegradedReasonSchema),
  pageBodyCaptured: Schema.Boolean,
  transcriptTextCaptured: Schema.Boolean,
  contentSemanticsClaimed: Schema.Boolean,
  aiDecisionClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  canDriveAiInput: Schema.Boolean,
});

export const BrowserUrlMetadataEvidenceSchema = withParser(
  BrowserUrlMetadataEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserUrlMetadataEvidenceIsConsistent(value) ||
        'Expected browser URL metadata evidence to preserve source refs and no-authority boundaries'
    )
  )
);

export const decodeBrowserUrlMetadataEvidence = Schema.decodeUnknownSync(BrowserUrlMetadataEvidenceSchema);

export type BrowserUrlMetadataEvidenceId = Infer<typeof BrowserUrlMetadataEvidenceIdSchema>;
export type BrowserUrlMetadataSourceKind = Infer<typeof BrowserUrlMetadataSourceKindSchema>;
export type BrowserUrlMetadataState = Infer<typeof BrowserUrlMetadataStateSchema>;
export type BrowserUrlMetadataDegradedReason = Infer<typeof BrowserUrlMetadataDegradedReasonSchema>;
export type BrowserUrlMetadataSource = Infer<typeof BrowserUrlMetadataSourceSchema>;
export type BrowserUrlMetadataFields = Infer<typeof BrowserUrlMetadataFieldsSchema>;
export type BrowserUrlMetadataEvidence = Infer<typeof BrowserUrlMetadataEvidenceSchema>;

function browserUrlMetadataEvidenceIsConsistent(value: Infer<typeof BrowserUrlMetadataEvidenceBaseSchema>) {
  if (browserUrlMetadataEvidenceClaimsAuthority(value)) {
    return false;
  }
  if (value.sourceEvidenceIds.length === 0) {
    return false;
  }
  if (value.source.sourceKind === 'hidden-managed-analysis' && value.source.hiddenAnalysisProofRef === null) {
    return false;
  }
  if (value.metadataState === 'available') {
    return hasMetadataField(value.fields) && value.degradedReasons.length === 0 && value.canDriveAiInput;
  }
  if (value.metadataState === 'partial') {
    return hasMetadataField(value.fields) && value.degradedReasons.length > 0 && value.canDriveAiInput;
  }
  return value.degradedReasons.length > 0 && value.canDriveAiInput === false;
}

function browserUrlMetadataEvidenceClaimsAuthority(value: Infer<typeof BrowserUrlMetadataEvidenceBaseSchema>) {
  return (
    value.pageBodyCaptured ||
    value.transcriptTextCaptured ||
    value.contentSemanticsClaimed ||
    value.aiDecisionClaimed ||
    value.policyDecisionClaimed ||
    value.policyAuthorityClaimed
  );
}

function hasMetadataField(value: Infer<typeof BrowserUrlMetadataFieldsSchema>) {
  return Object.values(value).some((fieldValue) => fieldValue !== null);
}
