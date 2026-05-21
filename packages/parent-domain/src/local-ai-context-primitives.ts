import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const LocalAiContextText = Schema.String.pipe(Schema.minLength(1));

export const LocalAiContextNonNegativeCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
export const LocalAiEvidenceContextIdSchema = LocalAiContextText.pipe(Schema.brand('LocalAiEvidenceContextId'));
export const LocalAiEvidenceContextRefIdSchema = LocalAiContextText.pipe(Schema.brand('LocalAiEvidenceContextRefId'));
export const LocalAiParentRuleContextRefIdSchema = LocalAiContextText.pipe(
  Schema.brand('LocalAiParentRuleContextRefId')
);
export const LocalAiEvidenceSourceIdSchema = LocalAiContextText.pipe(Schema.brand('LocalAiEvidenceSourceId'));
export const LocalAiEvidenceAdapterIdSchema = LocalAiContextText.pipe(Schema.brand('LocalAiEvidenceAdapterId'));
export const LocalAiEvidenceContextSummarySchema = LocalAiContextText.pipe(
  Schema.brand('LocalAiEvidenceContextSummary')
);
export const LocalAiRejectedFieldSchema = LocalAiContextText.pipe(Schema.brand('LocalAiRejectedField'));

export const LocalAiRequestedEvaluationKindSchema = withParser(
  Schema.Literal(
    'page',
    'url',
    'video',
    'app',
    'game',
    'domain',
    'network-digest',
    'screen-summary',
    'recent-activity',
    'mixed-context'
  )
);

export const LocalAiEvidenceContextKindSchema = withParser(
  Schema.Literal(
    'browser',
    'app-game',
    'network-flow',
    'screen-summary',
    'policy-decision',
    'parent-action',
    'recent-activity'
  )
);

export const LocalAiEvidenceCustodySchema = withParser(
  Schema.Literal(
    'live-local-child-agent',
    'live-lan-child-agent',
    'child-device-journal',
    'child-device-query-store',
    'parent-device-cache',
    'parent-owned-export',
    'ocentra-hosted-non-activity',
    'unavailable'
  )
);

export const LocalAiEvidenceRetentionStateSchema = withParser(
  Schema.Literal('local', 'temporary', 'deleted-source', 'export-copy', 'parent-owned-copy', 'unavailable')
);

export const LocalAiConfidenceKindSchema = withParser(
  Schema.Literal('observation', 'correlation', 'classifier', 'model', 'memory-match', 'graph-edge', 'rule-match')
);

export const LocalAiContextCapabilityStatusSchema = withParser(
  Schema.Literal(
    'available',
    'unsupported',
    'permission-limited',
    'stale',
    'degraded',
    'adapter-error',
    'disabled-by-parent',
    'unavailable'
  )
);

export const LocalAiContextReasonCodeSchema = withParser(
  Schema.Literal(
    'missing-evidence',
    'stale-evidence',
    'source-conflict',
    'unsupported-source',
    'permission-limited',
    'adapter-error',
    'capability-disabled-by-parent',
    'custody-unavailable',
    'forbidden-remote-source',
    'invalid-confidence',
    'invalid-ai-output',
    'model-unavailable',
    'model-overloaded',
    'model-output-unparseable',
    'memory-ungrounded',
    'graph-ungrounded',
    'parent-rule-missing',
    'parent-rule-conflict',
    'schedule-unresolved',
    'protected-surface',
    'screen-image-deleted',
    'screen-deletion-unconfirmed',
    'network-encrypted-content-unavailable',
    'browser-active-tab-unknown',
    'app-duration-incomplete'
  )
);

export const LocalAiContextBuildStateSchema = withParser(
  Schema.Literal('ready', 'partial', 'insufficient', 'unavailable', 'rejected')
);

export type LocalAiEvidenceContextId = typeof LocalAiEvidenceContextIdSchema.Type;
export type LocalAiEvidenceContextRefId = typeof LocalAiEvidenceContextRefIdSchema.Type;
export type LocalAiParentRuleContextRefId = typeof LocalAiParentRuleContextRefIdSchema.Type;
export type LocalAiEvidenceSourceId = typeof LocalAiEvidenceSourceIdSchema.Type;
export type LocalAiEvidenceAdapterId = typeof LocalAiEvidenceAdapterIdSchema.Type;
export type LocalAiEvidenceContextSummary = typeof LocalAiEvidenceContextSummarySchema.Type;
export type LocalAiRejectedField = typeof LocalAiRejectedFieldSchema.Type;
export type LocalAiRequestedEvaluationKind = Infer<typeof LocalAiRequestedEvaluationKindSchema>;
export type LocalAiEvidenceContextKind = Infer<typeof LocalAiEvidenceContextKindSchema>;
export type LocalAiEvidenceCustody = Infer<typeof LocalAiEvidenceCustodySchema>;
export type LocalAiEvidenceRetentionState = Infer<typeof LocalAiEvidenceRetentionStateSchema>;
export type LocalAiConfidenceKind = Infer<typeof LocalAiConfidenceKindSchema>;
export type LocalAiContextCapabilityStatus = Infer<typeof LocalAiContextCapabilityStatusSchema>;
export type LocalAiContextReasonCode = Infer<typeof LocalAiContextReasonCodeSchema>;
export type LocalAiContextBuildState = Infer<typeof LocalAiContextBuildStateSchema>;
