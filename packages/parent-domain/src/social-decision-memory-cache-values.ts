import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptySocialDecisionMemoryText = Schema.String.pipe(Schema.minLength(1));

export const SocialDecisionMemoryCacheSchemaVersionSchema = withParser(Schema.Literal('social-decision-memory-cache'));

export const SocialDecisionMemoryCacheEntryIdSchema = withParser(
  NonEmptySocialDecisionMemoryText.pipe(Schema.brand('SocialDecisionMemoryCacheEntryId'))
);

export const SocialDecisionMemoryCacheSnapshotIdSchema = withParser(
  NonEmptySocialDecisionMemoryText.pipe(Schema.brand('SocialDecisionMemoryCacheSnapshotId'))
);

export const SocialDecisionMemorySubjectKindSchema = withParser(
  Schema.Literal('account-ref', 'video-ref', 'channel-ref')
);

export const SocialDecisionMemoryStateSchema = withParser(
  Schema.Literal('fresh-hit', 'stale-hit', 'miss', 'manual-required')
);

export const SocialDecisionMemorySourceSchema = withParser(
  Schema.Literal('parent-decision-candidate', 'parent-approval-decision', 'manual-review', 'unavailable')
);

export const SocialDecisionMemoryCacheKeyKindSchema = withParser(
  Schema.Literal(
    'social-account-ref',
    'platform-video-ref',
    'platform-channel-ref',
    'policy-version',
    'child-profile',
    'parent-rule-set',
    'decision-candidate-ref'
  )
);

export const SocialDecisionMemoryTtlClassSchema = withParser(
  Schema.Literal('account-decision', 'stable-video-decision', 'channel-decision', 'dynamic-feed-decision')
);

export const SocialDecisionMemoryInvalidationReasonSchema = withParser(
  Schema.Literal(
    'parent-policy-changed',
    'parent-rule-changed',
    'parent-override-changed',
    'account-ref-changed',
    'video-metadata-changed',
    'channel-ref-changed',
    'confidence-too-low',
    'connector-revoked',
    'ttl-expired'
  )
);

export const SocialDecisionMemoryCacheKeyValueSchema = NonEmptySocialDecisionMemoryText.pipe(
  Schema.brand('SocialDecisionMemoryCacheKeyValue')
);

export type SocialDecisionMemorySubjectKind = Infer<typeof SocialDecisionMemorySubjectKindSchema>;
export type SocialDecisionMemoryState = Infer<typeof SocialDecisionMemoryStateSchema>;
export type SocialDecisionMemoryTtlClass = Infer<typeof SocialDecisionMemoryTtlClassSchema>;
export type SocialDecisionMemoryCacheKeyKind = Infer<typeof SocialDecisionMemoryCacheKeyKindSchema>;
