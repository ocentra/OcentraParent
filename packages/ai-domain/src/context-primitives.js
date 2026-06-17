import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
export const LocalAiContextNonNegativeCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
export const LocalAiEvidenceContextIdSchema = brandedNonEmptyStringSchema('LocalAiEvidenceContextId');
export const LocalAiEvidenceContextRefIdSchema = brandedNonEmptyStringSchema('LocalAiEvidenceContextRefId');
export const LocalAiParentRuleContextRefIdSchema = brandedNonEmptyStringSchema('LocalAiParentRuleContextRefId');
export const LocalAiEvidenceSourceIdSchema = brandedNonEmptyStringSchema('LocalAiEvidenceSourceId');
export const LocalAiEvidenceAdapterIdSchema = brandedNonEmptyStringSchema('LocalAiEvidenceAdapterId');
export const LocalAiEvidenceContextSummarySchema = brandedNonEmptyStringSchema('LocalAiEvidenceContextSummary');
export const LocalAiRejectedFieldSchema = brandedNonEmptyStringSchema('LocalAiRejectedField');
export const LocalAiRequestedEvaluationKindSchema = withParser(Schema.Literal('page', 'url', 'video', 'app', 'game', 'domain', 'network-digest', 'screen-summary', 'recent-activity', 'mixed-context'));
export const LocalAiEvidenceContextKindSchema = withParser(Schema.Literal('browser', 'app-game', 'network-flow', 'screen-summary', 'policy-decision', 'parent-action', 'recent-activity'));
export const LocalAiEvidenceCustodySchema = withParser(Schema.Literal('live-local-child-agent', 'live-lan-child-agent', 'child-device-journal', 'child-device-query-store', 'parent-device-cache', 'parent-owned-export', 'ocentra-hosted-non-activity', 'unavailable'));
export const LocalAiEvidenceRetentionStateSchema = withParser(Schema.Literal('local', 'temporary', 'deleted-source', 'export-copy', 'parent-owned-copy', 'unavailable'));
export const LocalAiConfidenceKindSchema = withParser(Schema.Literal('observation', 'correlation', 'classifier', 'model', 'memory-match', 'graph-edge', 'rule-match'));
export const LocalAiContextCapabilityStatusSchema = withParser(Schema.Literal('available', 'unsupported', 'permission-limited', 'stale', 'degraded', 'adapter-error', 'disabled-by-parent', 'unavailable'));
export const LocalAiContextReasonCodeSchema = withParser(Schema.Literal('missing-evidence', 'stale-evidence', 'source-conflict', 'unsupported-source', 'permission-limited', 'adapter-error', 'capability-disabled-by-parent', 'custody-unavailable', 'forbidden-remote-source', 'invalid-confidence', 'invalid-ai-output', 'model-unavailable', 'model-overloaded', 'model-output-unparseable', 'memory-ungrounded', 'graph-ungrounded', 'parent-rule-missing', 'parent-rule-conflict', 'schedule-unresolved', 'protected-surface', 'screen-image-deleted', 'screen-deletion-unconfirmed', 'network-encrypted-content-unavailable', 'browser-active-tab-unknown', 'app-duration-incomplete'));
export const LocalAiContextBuildStateSchema = withParser(Schema.Literal('ready', 'partial', 'insufficient', 'unavailable', 'rejected'));
//# sourceMappingURL=context-primitives.js.map