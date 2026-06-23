import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { BrowserKnowledgeGraphRefSchema } from './browser-ai-analysis-values';

export { BrowserKnowledgeGraphRefSchema };

export const BrowserAiKnowledgeGraphIdSchema = withParser(brandedNonEmptyStringSchema('BrowserAiKnowledgeGraphId'));
export const BrowserAiKnowledgeGraphSnapshotIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiKnowledgeGraphSnapshotId')
);
export const BrowserAiKnowledgeGraphVersionRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiKnowledgeGraphVersionRef')
);
export const BrowserAiKnowledgeGraphNodeRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiKnowledgeGraphNodeRef')
);
export const BrowserAiKnowledgeGraphEdgeRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAiKnowledgeGraphEdgeRef')
);

export const BrowserAiKnowledgeGraphSourceKindSchema = withParser(
  Schema.Literal(
    'browser-evidence',
    'metadata-evidence',
    'memory-cache',
    'ai-analysis',
    'parent-rule',
    'parent-approved-source',
    'external-taxonomy',
    'platform-label'
  )
);
export const BrowserAiKnowledgeGraphNodeKindSchema = withParser(
  Schema.Literal(
    'url',
    'origin',
    'domain',
    'platform-video',
    'platform-channel',
    'content-category',
    'riskSignal',
    'benefit-signal',
    'parent-rule',
    'schedule-window',
    'trusted-source',
    'unknown'
  )
);
export const BrowserAiKnowledgeGraphEdgeKindSchema = withParser(
  Schema.Literal(
    'canonicalizes',
    'belongs-to-channel',
    'hosted-on-domain',
    'has-category-signal',
    'has-riskSignal',
    'has-benefit-signal',
    'parent-rule-applies',
    'schedule-applies',
    'memory-supports',
    'source-derived-from',
    'related-topic',
    'unknown'
  )
);
export const BrowserAiKnowledgeGraphUseSchema = withParser(
  Schema.Literal('ai-input-context', 'policy-candidate-support', 'parent-explanation', 'memory-key-support')
);

export type BrowserKnowledgeGraphRef = Infer<typeof BrowserKnowledgeGraphRefSchema>;
export type BrowserAiKnowledgeGraphNodeRef = Infer<typeof BrowserAiKnowledgeGraphNodeRefSchema>;
export type BrowserAiKnowledgeGraphSourceKind = Infer<typeof BrowserAiKnowledgeGraphSourceKindSchema>;
export type BrowserAiKnowledgeGraphUse = Infer<typeof BrowserAiKnowledgeGraphUseSchema>;
