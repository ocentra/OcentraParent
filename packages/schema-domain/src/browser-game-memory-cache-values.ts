import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const BrowserGameMemoryCacheSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-memory-cache-contract')
);

export const BrowserGameMemoryCacheEntryIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameMemoryCacheEntryId')
);

export const BrowserGameMemoryCacheSnapshotIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameMemoryCacheSnapshotId')
);

export const BrowserGameMemorySubjectKindSchema = withParser(
  Schema.Literal('game-url-ref', 'platform-game-ref', 'cloud-game-ref', 'category-ref', 'parent-decision-ref')
);

export const BrowserGameMemoryStateSchema = withParser(
  Schema.Literal('fresh-hit', 'stale-hit', 'miss', 'manual-required')
);

export const BrowserGameMemorySourceSchema = withParser(
  Schema.Literal(
    'parent-decision-candidate',
    'parent-approval-decision',
    'ai-analysis-candidate',
    'metadata-classifier',
    'manual-review',
    'unavailable'
  )
);

export const BrowserGameMemoryCacheKeyKindSchema = withParser(
  Schema.Literal(
    'canonical-url-ref',
    'platform-game-ref',
    'domain-path-hash',
    'cloud-game-title-ref',
    'parent-decision-ref',
    'game-category-ref',
    'policy-version',
    'child-profile',
    'parent-rule-set',
    'evidence-ref'
  )
);

export const BrowserGameMemoryTtlClassSchema = withParser(
  Schema.Literal(
    'short-dynamic-game-page',
    'cloud-launcher-page',
    'ugc-game-page',
    'stable-approved-game',
    'parent-approved-account-page'
  )
);

export const BrowserGameMemoryInvalidationReasonSchema = withParser(
  Schema.Literal(
    'parent-policy-changed',
    'parent-rule-changed',
    'parent-override-changed',
    'url-evidence-changed',
    'metadata-changed',
    'cloud-title-ref-changed',
    'confidence-too-low',
    'ttl-expired',
    'manual-required'
  )
);

export const BrowserGameMemoryCacheKeyValueSchema = brandedNonEmptyStringSchema('BrowserGameMemoryCacheKeyValue');

export type BrowserGameMemoryCacheKeyKind = Infer<typeof BrowserGameMemoryCacheKeyKindSchema>;
export type BrowserGameMemoryState = Infer<typeof BrowserGameMemoryStateSchema>;
export type BrowserGameMemorySubjectKind = Infer<typeof BrowserGameMemorySubjectKindSchema>;
export type BrowserGameMemoryTtlClass = Infer<typeof BrowserGameMemoryTtlClassSchema>;

