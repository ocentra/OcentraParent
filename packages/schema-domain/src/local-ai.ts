import { type Infer, Schema, withParser } from './effect';
import { PolicyActionSchema, PolicyReasonCodeSchema, PolicyRuleIdSchema } from './policy-contracts';
import { ChildProfileReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from './family-references';
import { ParentContractSchemaVersionSchema } from './family-reference-primitives';
import {
  LocalAiDegradedStateSchema,
  LocalAiConfidenceSchema,
  LocalAiEvaluationRequestIdSchema,
  LocalAiExplanationReferenceSchema,
  LocalAiPromptVersionSchema,
  LocalAiResultIdSchema,
  LocalAiTimestampSchema,
  LocalAiUnknownStateSchema,
} from './ai-primitives';
import {
  LocalAiGraphReferenceSchema,
  LocalAiMemoryReferenceSchema,
  LocalAiObservationReferenceSchema,
} from './ai-references';
import { LocalAiModelRequestMetadataSchema, LocalModelRuntimeStatusSchema } from './ai-runtime';

export const LocalAiEvaluationInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: LocalAiEvaluationRequestIdSchema,
    childProfile: ChildProfileReferenceSchema,
    device: ParentDeviceReferenceSchema,
    currentObservation: LocalAiObservationReferenceSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    parentRuleReferences: Schema.Array(PolicyRuleIdSchema),
    recentActivityWindow: Schema.Array(ParentEvidenceReferenceSchema),
    memoryReferences: Schema.Array(LocalAiMemoryReferenceSchema),
    graphReferences: Schema.Array(LocalAiGraphReferenceSchema),
    modelRequest: LocalAiModelRequestMetadataSchema,
  })
);

export const LocalAiSafetyResultSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    resultId: LocalAiResultIdSchema,
    requestId: LocalAiEvaluationRequestIdSchema,
    action: PolicyActionSchema,
    confidence: LocalAiConfidenceSchema,
    unknownState: LocalAiUnknownStateSchema,
    degradedState: LocalAiDegradedStateSchema,
    reasonCodes: Schema.Array(PolicyReasonCodeSchema),
    explanationReference: Schema.Union(LocalAiExplanationReferenceSchema, Schema.Null),
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    parentRuleReferences: Schema.Array(PolicyRuleIdSchema),
    memoryReferences: Schema.Array(LocalAiMemoryReferenceSchema),
    graphReferences: Schema.Array(LocalAiGraphReferenceSchema),
    modelRuntime: LocalModelRuntimeStatusSchema,
    promptVersion: LocalAiPromptVersionSchema,
    expiresAt: Schema.Union(LocalAiTimestampSchema, Schema.Null),
  })
);

export type LocalAiEvaluationInput = Infer<typeof LocalAiEvaluationInputSchema>;
export type LocalAiSafetyResult = Infer<typeof LocalAiSafetyResultSchema>;
