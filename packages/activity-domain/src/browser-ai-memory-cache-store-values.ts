import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyMemoryCacheValueText = Schema.String.pipe(Schema.minLength(1));

export const BrowserAiMemoryCacheStoreIdSchema = withParser(
  NonEmptyMemoryCacheValueText.pipe(Schema.brand('BrowserAiMemoryCacheStoreId'))
);
export const BrowserAiMemoryCacheEntryIdSchema = withParser(
  NonEmptyMemoryCacheValueText.pipe(Schema.brand('BrowserAiMemoryCacheEntryId'))
);

export const BrowserAiMemoryCacheKeyKindSchema = withParser(
  Schema.Literal(
    'canonical-url',
    'normalized-origin-path',
    'platform-video-id',
    'platform-channel-id',
    'metadata-hash',
    'transcript-hash',
    'thumbnail-perceptual-hash',
    'model-prompt-version',
    'policy-version',
    'child-profile'
  )
);
export const BrowserAiMemoryCacheTtlClassSchema = withParser(
  Schema.Literal(
    'stable-video',
    'dynamic-feed',
    'search-results',
    'homepage',
    'social-feed',
    'livestream',
    'parent-approved-educational'
  )
);
export const BrowserAiMemoryCacheInvalidationReasonSchema = withParser(
  Schema.Literal(
    'parent-policy-changed',
    'model-changed',
    'prompt-changed',
    'metadata-changed',
    'transcript-changed',
    'platform-id-changed',
    'parent-override-changed',
    'confidence-too-low',
    'ttl-expired'
  )
);

export type BrowserAiMemoryCacheKeyKind = Infer<typeof BrowserAiMemoryCacheKeyKindSchema>;
export type BrowserAiMemoryCacheTtlClass = Infer<typeof BrowserAiMemoryCacheTtlClassSchema>;
