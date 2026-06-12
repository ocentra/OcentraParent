import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { BrowserPageTitleSchema, BrowserUrlSchema } from './browser-schemas';
import {
  BrowserUrlShapeClassificationIdSchema,
  type BrowserUrlShapeClassificationResult,
  BrowserUrlShapeClassificationResultSchema,
} from './browser-url-intelligence-schemas';
import {
  BrowserUrlMetadataDegradedReasonSchema,
  type BrowserUrlMetadataEvidence,
  BrowserUrlMetadataEvidenceIdSchema,
  BrowserUrlMetadataEvidenceSchema,
  BrowserUrlMetadataSchemaVersion,
} from './browser-url-metadata-schemas';

const NonEmptyYouTubeMetadataText = Schema.String.pipe(Schema.minLength(1));
const OptionalYouTubeMetadataTextSchema = Schema.Union(NonEmptyYouTubeMetadataText, Schema.Null);
const OptionalYouTubeMetadataUrlSchema = Schema.Union(BrowserUrlSchema, Schema.Null);
const OptionalYouTubeMetadataDurationSecondsSchema = Schema.Union(
  Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  Schema.Null
);
const OptionalYouTubeMetadataTimestampSchema = Schema.Union(ActivityTimestampSchema, Schema.Null);
const OptionalYouTubeMetadataBooleanSchema = Schema.Union(Schema.Boolean, Schema.Null);

const YouTubeSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected YouTube metadata source evidence ids')
);
const YouTubeMetadataDegradedReasonsSchema = Schema.Array(BrowserUrlMetadataDegradedReasonSchema);

const YouTubeMetadataAdapterInputBaseSchema = Schema.Struct({
  metadataEvidenceId: BrowserUrlMetadataEvidenceIdSchema,
  collectedAt: ActivityTimestampSchema,
  sourceEvidenceIds: YouTubeSourceEvidenceIdsSchema,
  classification: BrowserUrlShapeClassificationResultSchema,
  sourceRef: OptionalYouTubeMetadataTextSchema,
  browserTitle: Schema.Union(BrowserPageTitleSchema, Schema.Null),
  openGraphTitle: OptionalYouTubeMetadataTextSchema,
  openGraphDescription: OptionalYouTubeMetadataTextSchema,
  channelName: OptionalYouTubeMetadataTextSchema,
  thumbnailUrl: OptionalYouTubeMetadataUrlSchema,
  thumbnailHashRef: OptionalYouTubeMetadataTextSchema,
  durationSeconds: OptionalYouTubeMetadataDurationSecondsSchema,
  publishDate: OptionalYouTubeMetadataTimestampSchema,
  captionsAvailable: OptionalYouTubeMetadataBooleanSchema,
  transcriptAvailable: OptionalYouTubeMetadataBooleanSchema,
  platformCategory: OptionalYouTubeMetadataTextSchema,
  platformRating: OptionalYouTubeMetadataTextSchema,
  restrictedSignal: OptionalYouTubeMetadataTextSchema,
  degradedReasons: YouTubeMetadataDegradedReasonsSchema,
});

const YouTubeMetadataAdapterInputSchema = withParser(
  YouTubeMetadataAdapterInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        youtubeMetadataInputIsConsistent(value) ||
        'Expected exact managed YouTube URL shape before building metadata evidence'
    )
  )
);

export type YouTubeMetadataAdapterInput = Infer<typeof YouTubeMetadataAdapterInputSchema>;

export function buildYouTubeMetadataEvidence(input: YouTubeMetadataAdapterInput): BrowserUrlMetadataEvidence {
  const parsed = YouTubeMetadataAdapterInputSchema.parse(input);

  return BrowserUrlMetadataEvidenceSchema.parse({
    schemaVersion: BrowserUrlMetadataSchemaVersion,
    metadataEvidenceId: parsed.metadataEvidenceId,
    collectedAt: parsed.collectedAt,
    sourceEvidenceIds: parsed.sourceEvidenceIds,
    urlShapeClassificationId: parsed.classification.classificationId,
    source: {
      sourceKind: 'platform-page-metadata',
      sourceRef: parsed.sourceRef,
      hiddenAnalysisProofRef: null,
    },
    metadataState: parsed.degradedReasons.length === 0 ? 'available' : 'partial',
    fields: {
      browserTitle: parsed.browserTitle,
      openGraphTitle: parsed.openGraphTitle,
      openGraphDescription: parsed.openGraphDescription,
      schemaOrgType: youtubeSchemaOrgType(parsed.classification),
      platformVideoId: parsed.classification.platformIds.videoId,
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

function youtubeMetadataInputIsConsistent(value: Infer<typeof YouTubeMetadataAdapterInputBaseSchema>) {
  return (
    value.classification.exactUrlEvidence &&
    value.classification.sourceKind === 'managed-browser-exact-url' &&
    youtubeClassificationIsSupported(value.classification) &&
    value.classification.classificationId ===
      BrowserUrlShapeClassificationIdSchema.parse(value.classification.classificationId)
  );
}

function youtubeClassificationIsSupported(value: BrowserUrlShapeClassificationResult) {
  if (value.platform !== 'youtube' && value.platform !== 'youtube-shorts') {
    return false;
  }
  return (
    value.targetKind === 'video' ||
    value.targetKind === 'short-video' ||
    value.targetKind === 'channel' ||
    value.targetKind === 'playlist'
  );
}

function youtubeSchemaOrgType(value: BrowserUrlShapeClassificationResult) {
  if (value.targetKind === 'video' || value.targetKind === 'short-video') {
    return 'video-object' as const;
  }
  return null;
}
