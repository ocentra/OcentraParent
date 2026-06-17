import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
export const LocalAiEvaluationRequestIdSchema = brandedNonEmptyStringSchema('LocalAiEvaluationRequestId');
export const LocalAiResultIdSchema = brandedNonEmptyStringSchema('LocalAiResultId');
export const LocalAiPromptVersionSchema = brandedNonEmptyStringSchema('LocalAiPromptVersion');
export const LocalAiModelIdSchema = brandedNonEmptyStringSchema('LocalAiModelId');
export const LocalAiProviderIdSchema = brandedNonEmptyStringSchema('LocalAiProviderId');
export const LocalAiModelReferenceSchema = brandedNonEmptyStringSchema('LocalAiModelReference');
export const LocalAiExplanationReferenceSchema = brandedNonEmptyStringSchema('LocalAiExplanationReference');
export const LocalAiMemoryReferenceIdSchema = brandedNonEmptyStringSchema('LocalAiMemoryReferenceId');
export const LocalAiGraphReferenceIdSchema = brandedNonEmptyStringSchema('LocalAiGraphReferenceId');
export const LocalAiRuntimeReferenceIdSchema = brandedNonEmptyStringSchema('LocalAiRuntimeReferenceId');
export const LocalAiTimestampSchema = brandedNonEmptyStringSchema('AiTimestamp');
export const LocalAiDerivedIndexVersionSchema = brandedNonEmptyStringSchema('LocalAiDerivedIndexVersion');
export const LocalAiUnavailableReasonSchema = brandedNonEmptyStringSchema('LocalAiUnavailableReason');
export const LocalAiConfidenceSchema = Schema.Number.pipe(Schema.between(0, 1));
export const LocalAiContextKindSchema = withParser(Schema.Literal('app', 'process', 'window', 'url', 'page', 'video', 'domain', 'network', 'recent-activity'));
export const LocalAiModelLoadStateSchema = withParser(Schema.Literal('unavailable', 'loading', 'loaded', 'degraded', 'failed'));
export const LocalAiCapabilityFlagSchema = withParser(Schema.Literal('classification', 'summarization', 'embedding', 'safety-decision', 'chat-completion'));
export const LocalAiResourceClassSchema = withParser(Schema.Literal('cpu', 'gpu', 'npu', 'remote-unavailable'));
export const LocalAiDegradedStateSchema = withParser(Schema.Literal('none', 'provider-unavailable', 'model-load-failed', 'overloaded', 'invalid-output'));
export const LocalAiUnknownStateSchema = withParser(Schema.Literal('none', 'missing-evidence', 'low-confidence', 'model-unavailable', 'policy-conflict'));
export const LocalAiMemoryReferenceKindSchema = withParser(Schema.Literal('evidence-memory', 'recent-activity', 'policy-memory', 'semantic-memory'));
export const LocalAiGraphReferenceKindSchema = withParser(Schema.Literal('graph-entity', 'graph-edge'));
export const LocalAiDegradedState = {
    None: LocalAiDegradedStateSchema.parse('none'),
    ProviderUnavailable: LocalAiDegradedStateSchema.parse('provider-unavailable'),
    ModelLoadFailed: LocalAiDegradedStateSchema.parse('model-load-failed'),
    Overloaded: LocalAiDegradedStateSchema.parse('overloaded'),
    InvalidOutput: LocalAiDegradedStateSchema.parse('invalid-output'),
};
export const LocalAiUnknownState = {
    None: LocalAiUnknownStateSchema.parse('none'),
    MissingEvidence: LocalAiUnknownStateSchema.parse('missing-evidence'),
    LowConfidence: LocalAiUnknownStateSchema.parse('low-confidence'),
    ModelUnavailable: LocalAiUnknownStateSchema.parse('model-unavailable'),
    PolicyConflict: LocalAiUnknownStateSchema.parse('policy-conflict'),
};
//# sourceMappingURL=primitives.js.map