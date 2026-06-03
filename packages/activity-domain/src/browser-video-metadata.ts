import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { BrowserPageTitleSchema, BrowserUrlSchema } from './browser-schemas';
import {
  type BrowserUrlShapeClassificationResult,
  BrowserUrlShapeClassificationResultSchema,
} from './browser-url-intelligence-schemas';
import {
  BrowserUrlMetadataDegradedReasonSchema,
  type BrowserUrlMetadataEvidence,
  BrowserUrlMetadataEvidenceIdSchema,
  BrowserUrlMetadataEvidenceSchema,
  BrowserUrlMetadataSchemaVersion,
  BrowserUrlMetadataSourceKindSchema,
} from './browser-url-metadata-schemas';

const NonEmptyVideoMetadataText = Schema.String.pipe(Schema.minLength(1));
const OptionalVideoMetadataTextSchema = Schema.Union(NonEmptyVideoMetadataText, Schema.Null);
const OptionalVideoMetadataUrlSchema = Schema.Union(BrowserUrlSchema, Schema.Null);
const OptionalVideoMetadataDurationSecondsSchema = Schema.Union(
  Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  Schema.Null
);
const OptionalVideoMetadataTimestampSchema = Schema.Union(ActivityTimestampSchema, Schema.Null);
const OptionalVideoMetadataBooleanSchema = Schema.Union(Schema.Boolean, Schema.Null);

const VideoSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected video metadata source evidence ids')
);
const VideoMetadataDegradedReasonsSchema = Schema.Array(BrowserUrlMetadataDegradedReasonSchema);

const VideoMetadataAdapterInputBaseSchema = Schema.Struct({
  metadataEvidenceId: BrowserUrlMetadataEvidenceIdSchema,
  collectedAt: ActivityTimestampSchema,
  sourceEvidenceIds: VideoSourceEvidenceIdsSchema,
  classification: BrowserUrlShapeClassificationResultSchema,
  sourceKind: BrowserUrlMetadataSourceKindSchema,
  sourceRef: OptionalVideoMetadataTextSchema,
  browserTitle: Schema.Union(BrowserPageTitleSchema, Schema.Null),
  openGraphTitle: OptionalVideoMetadataTextSchema,
  openGraphDescription: OptionalVideoMetadataTextSchema,
  platformVideoIdOverride: OptionalVideoMetadataTextSchema,
  channelName: OptionalVideoMetadataTextSchema,
  thumbnailUrl: OptionalVideoMetadataUrlSchema,
  thumbnailHashRef: OptionalVideoMetadataTextSchema,
  durationSeconds: OptionalVideoMetadataDurationSecondsSchema,
  publishDate: OptionalVideoMetadataTimestampSchema,
  captionsAvailable: OptionalVideoMetadataBooleanSchema,
  transcriptAvailable: OptionalVideoMetadataBooleanSchema,
  platformCategory: OptionalVideoMetadataTextSchema,
  platformRating: OptionalVideoMetadataTextSchema,
  restrictedSignal: OptionalVideoMetadataTextSchema,
  degradedReasons: VideoMetadataDegradedReasonsSchema,
});

const VideoMetadataAdapterInputSchema = withParser(
  VideoMetadataAdapterInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        videoMetadataInputIsConsistent(value) ||
        'Expected exact managed Vimeo or schema.org generic video metadata evidence'
    )
  )
);

export type VideoMetadataAdapterInput = Infer<typeof VideoMetadataAdapterInputSchema>;

export function buildVideoMetadataEvidence(input: VideoMetadataAdapterInput): BrowserUrlMetadataEvidence {
  const parsed = VideoMetadataAdapterInputSchema.parse(input);

  return BrowserUrlMetadataEvidenceSchema.parse({
    schemaVersion: BrowserUrlMetadataSchemaVersion,
    metadataEvidenceId: parsed.metadataEvidenceId,
    collectedAt: parsed.collectedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    urlShapeClassificationId: parsed.classification.classificationId,
    source: {
      sourceKind: parsed.sourceKind,
      sourceRef: parsed.sourceRef,
      hiddenAnalysisProofRef: null,
    },
    metadataState: parsed.degradedReasons.length === 0 ? 'available' : 'partial',
    fields: {
      browserTitle: parsed.browserTitle,
      openGraphTitle: parsed.openGraphTitle,
      openGraphDescription: parsed.openGraphDescription,
      schemaOrgType: 'video-object',
      platformVideoId: parsed.classification.platformIds.videoId ?? parsed.platformVideoIdOverride,
      platformChannelId: parsed.classification.platformIds.channelId,
      channelName: parsed.channelName,
      thumbnailUrl: parsed.thumbnailUrl,
      thumbnailHashRef: parsed.thumbnailHashRef,
      durationSeconds: parsed.durationSeconds,
      publishDate: parsed.publishDate,
      captionsAvailable: parsed.captionsAvailable,
      transcriptAvailable: parsed.transcriptAvailable,
      platformCategory: parsed.platformCategory,
      platformRating: parsed.platformRating,
      restrictedSignal: parsed.restrictedSignal,
    },
    degradedReasons: parsed.degradedReasons,
    pageBodyCaptured: false,
    transcriptTextCaptured: false,
    contentSemanticsClaimed: false,
    aiDecisionClaimed: false,
    policyDecisionClaimed: false,
    policyAuthorityClaimed: false,
    canDriveAiInput: true,
  });
}

function videoMetadataInputIsConsistent(value: Infer<typeof VideoMetadataAdapterInputBaseSchema>) {
  if (value.classification.sourceKind !== 'managed-browser-exact-url' || !value.classification.exactUrlEvidence) {
    return false;
  }
  return vimeoClassificationIsSupported(value.classification) || genericVideoObjectIsSupported(value);
}

function vimeoClassificationIsSupported(value: BrowserUrlShapeClassificationResult) {
  return value.platform === 'vimeo' && value.targetKind === 'video' && value.platformIds.videoId !== null;
}

function genericVideoObjectIsSupported(value: Infer<typeof VideoMetadataAdapterInputBaseSchema>) {
  return (
    value.classification.platform === 'generic-web' &&
    value.sourceKind === 'schema-org-video-object' &&
    hasGenericVideoMetadata(value)
  );
}

function hasGenericVideoMetadata(value: Infer<typeof VideoMetadataAdapterInputBaseSchema>) {
  return (
    value.openGraphTitle !== null ||
    value.openGraphDescription !== null ||
    value.thumbnailUrl !== null ||
    value.thumbnailHashRef !== null ||
    value.durationSeconds !== null ||
    value.platformVideoIdOverride !== null
  );
}
