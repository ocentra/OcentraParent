/* generated from crates/browser-core/src/browser_generated_social_ts.rs */

import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserDomainSchema, BrowserPageTitleSchema, BrowserUrlSchema } from './generated-browser-schemas';
import {
  browserUrlIntelligenceMemoryHitIsConsistent,
  browserUrlShapeClassificationResultIsConsistent,
} from './generated-browser-url-intelligence-rules';

export const BrowserUrlShapeSchemaVersion = 1;
export const BrowserUrlIntelligenceMemorySchemaVersion = 1;

export const BrowserUrlShapeClassificationIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserUrlShapeClassificationId')
);

export const BrowserUrlIntelligenceMemoryHitIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserUrlIntelligenceMemoryHitId')
);

export const BrowserUrlShapeSourceKindSchema = withParser(
  Schema.Literal(
    'managed-browser-exact-url',
    'managed-browser-target-list',
    'unmanaged-browser-process',
    'network-domain'
  )
);

export const BrowserUrlShapeTargetKindSchema = withParser(
  Schema.Literal(
    'video',
    'short-video',
    'channel',
    'playlist',
    'search',
    'article',
    'forum',
    'social-feed',
    'social-post',
    'social-messaging',
    'social-upload-post',
    'social-livestream',
    'game',
    'cloud-gaming',
    'download',
    'browser-internal',
    'file',
    'unknown'
  )
);

export const BrowserUrlShapePlatformSchema = withParser(
  Schema.Literal(
    'youtube',
    'youtube-shorts',
    'vimeo',
    'tiktok',
    'instagram',
    'facebook',
    'twitch',
    'x-twitter',
    'reddit',
    'discord',
    'generic-web',
    'unknown'
  )
);

export const BrowserUrlShapeConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserUrlShapeReasonCodeSchema = withParser(
  Schema.Literal(
    'parsed-url',
    'parsed-youtube-video-id',
    'parsed-youtube-shorts-id',
    'parsed-channel-id',
    'parsed-playlist-id',
    'parsed-search-query',
    'parsed-post-id',
    'parsed-social-route',
    'title-domain-only',
    'unsupported-scheme',
    'unmanaged-process-only',
    'network-domain-only',
    'no-exact-evidence',
    'dynamic-feed',
    'content-not-inferred',
    'manual-required'
  )
);

export const BrowserUrlIntelligenceMemoryKeyKindSchema = withParser(
  Schema.Literal(
    'normalized-url',
    'canonical-video-id',
    'platform-video-id',
    'platform-channel-id',
    'domain-path-hash',
    'content-metadata-hash',
    'parent-approved-exception',
    'previous-ai-analysis',
    'previous-policy-decision'
  )
);

export const BrowserUrlIntelligenceMemoryHitStateSchema = withParser(
  Schema.Literal('hit', 'miss', 'stale', 'manual-required')
);

export const BrowserUrlIntelligenceMemoryDecisionKindSchema = withParser(
  Schema.Literal(
    'known-allowed',
    'known-blocked',
    'previously-approved',
    'previously-denied',
    'known-unknown',
    'manual-required',
    'no-hit'
  )
);

export const BrowserUrlIntelligenceMemoryStaleReasonSchema = withParser(
  Schema.Literal(
    'expired',
    'policy-changed',
    'model-changed',
    'prompt-changed',
    'metadata-changed',
    'parent-override-changed',
    'confidence-too-low',
    'dynamic-feed-ttl',
    'source-missing'
  )
);

const OptionalShapeIdSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const BrowserUrlShapePlatformIdsSchema = Schema.Struct({
  videoId: OptionalShapeIdSchema,
  channelId: OptionalShapeIdSchema,
  playlistId: OptionalShapeIdSchema,
  postId: OptionalShapeIdSchema,
  query: OptionalShapeIdSchema,
});

const BrowserUrlIntelligenceMemoryRefSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);

export const BrowserUrlIntelligenceMemoryKeySchema = Schema.Struct({
  keyKind: BrowserUrlIntelligenceMemoryKeyKindSchema,
  keyValue: NonEmptyStringSchema,
});

const BrowserUrlShapeClassificationResultBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserUrlShapeSchemaVersion),
  classificationId: BrowserUrlShapeClassificationIdSchema,
  classifiedAt: ActivityTimestampSchema,
  sourceEvidenceIds: Schema.Array(ActivityEvidenceIdSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected at least one source evidence id')
  ),
  sourceKind: BrowserUrlShapeSourceKindSchema,
  url: Schema.Union(BrowserUrlSchema, Schema.Null),
  domain: Schema.Union(BrowserDomainSchema, Schema.Null),
  title: Schema.Union(BrowserPageTitleSchema, Schema.Null),
  targetKind: BrowserUrlShapeTargetKindSchema,
  platform: BrowserUrlShapePlatformSchema,
  platformIds: BrowserUrlShapePlatformIdsSchema,
  confidence: BrowserUrlShapeConfidenceSchema,
  reasonCodes: Schema.Array(BrowserUrlShapeReasonCodeSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected at least one URL shape reason code')
  ),
  exactUrlEvidence: Schema.Boolean,
  contentSemanticsClaimed: Schema.Boolean,
  aiDecisionClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
});

export const BrowserUrlShapeClassificationResultSchema = withParser(
  BrowserUrlShapeClassificationResultBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserUrlShapeClassificationResultIsConsistent(value) ||
        'Expected URL shape result to preserve evidence and no-claim boundaries'
    )
  )
);

export const decodeBrowserUrlShapeClassificationResult = Schema.decodeUnknownSync(
  BrowserUrlShapeClassificationResultSchema
);

const BrowserUrlIntelligenceMemoryHitBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserUrlIntelligenceMemorySchemaVersion),
  memoryHitId: BrowserUrlIntelligenceMemoryHitIdSchema,
  lookedUpAt: ActivityTimestampSchema,
  key: BrowserUrlIntelligenceMemoryKeySchema,
  hitState: BrowserUrlIntelligenceMemoryHitStateSchema,
  decisionKind: BrowserUrlIntelligenceMemoryDecisionKindSchema,
  sourceEvidenceIds: Schema.Array(ActivityEvidenceIdSchema),
  analysisRef: BrowserUrlIntelligenceMemoryRefSchema,
  parentActionRef: BrowserUrlIntelligenceMemoryRefSchema,
  policyVersionRef: BrowserUrlIntelligenceMemoryRefSchema,
  expiresAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  staleReason: Schema.Union(BrowserUrlIntelligenceMemoryStaleReasonSchema, Schema.Null),
  canDrivePolicyInput: Schema.Boolean,
  canDirectlyEnforce: Schema.Boolean,
});

export const BrowserUrlIntelligenceMemoryHitSchema = withParser(
  BrowserUrlIntelligenceMemoryHitBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserUrlIntelligenceMemoryHitIsConsistent(value) ||
        'Expected browser URL intelligence memory hit to cite source refs and preserve no-enforcement boundary'
    )
  )
);

export const decodeBrowserUrlIntelligenceMemoryHit = Schema.decodeUnknownSync(BrowserUrlIntelligenceMemoryHitSchema);

export type BrowserUrlIntelligenceMemoryHitId = Infer<typeof BrowserUrlIntelligenceMemoryHitIdSchema>;
export type BrowserUrlShapeClassificationId = Infer<typeof BrowserUrlShapeClassificationIdSchema>;
export type BrowserUrlShapeSourceKind = Infer<typeof BrowserUrlShapeSourceKindSchema>;
export type BrowserUrlShapeTargetKind = Infer<typeof BrowserUrlShapeTargetKindSchema>;
export type BrowserUrlShapePlatform = Infer<typeof BrowserUrlShapePlatformSchema>;
export type BrowserUrlShapeConfidence = Infer<typeof BrowserUrlShapeConfidenceSchema>;
export type BrowserUrlShapeReasonCode = Infer<typeof BrowserUrlShapeReasonCodeSchema>;
export type BrowserUrlShapePlatformIds = Infer<typeof BrowserUrlShapePlatformIdsSchema>;
export type BrowserUrlShapeClassificationResult = Infer<typeof BrowserUrlShapeClassificationResultSchema>;
export type BrowserUrlIntelligenceMemoryKeyKind = Infer<typeof BrowserUrlIntelligenceMemoryKeyKindSchema>;
export type BrowserUrlIntelligenceMemoryHitState = Infer<typeof BrowserUrlIntelligenceMemoryHitStateSchema>;
export type BrowserUrlIntelligenceMemoryDecisionKind = Infer<typeof BrowserUrlIntelligenceMemoryDecisionKindSchema>;
export type BrowserUrlIntelligenceMemoryStaleReason = Infer<typeof BrowserUrlIntelligenceMemoryStaleReasonSchema>;
export type BrowserUrlIntelligenceMemoryKey = Infer<typeof BrowserUrlIntelligenceMemoryKeySchema>;
export type BrowserUrlIntelligenceMemoryHit = Infer<typeof BrowserUrlIntelligenceMemoryHitSchema>;
