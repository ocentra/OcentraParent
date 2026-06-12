import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { PolicyActionSchema, PolicyReasonCodeSchema, PolicyRuleIdSchema } from './policy';
import { ChildProfileReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  LocalAiDegradedStateSchema,
  LocalAiConfidenceSchema,
  LocalAiEvaluationRequestIdSchema,
  LocalAiExplanationReferenceSchema,
  LocalAiPromptVersionSchema,
  LocalAiResultIdSchema,
  LocalAiTimestampSchema,
  LocalAiUnknownStateSchema,
} from './local-ai-primitives';
import {
  LocalAiGraphReferenceSchema,
  LocalAiMemoryReferenceSchema,
  LocalAiObservationReferenceSchema,
} from './local-ai-references';
import { LocalAiModelRequestMetadataSchema, LocalModelRuntimeStatusSchema } from './local-ai-runtime';

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
