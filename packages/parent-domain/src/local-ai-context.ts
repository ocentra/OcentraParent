import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { PolicyRuleIdSchema } from './policy';
import { ChildProfileReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from './references';
import { ParentContractSchemaVersionSchema } from './reference-primitives';
import {
  LocalAiCapabilityFlagSchema,
  LocalAiConfidenceSchema,
  LocalAiEvaluationRequestIdSchema,
  LocalAiPromptVersionSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
} from './local-ai-primitives';
import { LocalAiGraphReferenceSchema, LocalAiMemoryReferenceSchema } from './local-ai-references';
import { LocalModelRuntimeStatusSchema } from './local-ai-runtime';
import {
  LocalAiConfidenceKindSchema,
  LocalAiContextBuildStateSchema,
  LocalAiContextCapabilityStatusSchema,
  LocalAiContextNonNegativeCountSchema,
  LocalAiContextReasonCodeSchema,
  LocalAiEvidenceAdapterIdSchema,
  LocalAiEvidenceContextIdSchema,
  LocalAiEvidenceContextKindSchema,
  LocalAiEvidenceContextRefIdSchema,
  LocalAiEvidenceContextSummarySchema,
  LocalAiEvidenceCustodySchema,
  LocalAiEvidenceRetentionStateSchema,
  LocalAiEvidenceSourceIdSchema,
  LocalAiRejectedFieldSchema,
  LocalAiRequestedEvaluationKindSchema,
} from './local-ai-context-primitives';

export * from './local-ai-context-primitives';

export const LocalAiEvidenceContextSourceRefSchema = withParser(
  Schema.Struct({
    evidenceRefId: LocalAiEvidenceContextRefIdSchema,
    evidence: ParentEvidenceReferenceSchema,
    evidenceKind: LocalAiEvidenceContextKindSchema,
    sourceSchemaVersion: ParentContractSchemaVersionSchema,
    observedAt: LocalAiTimestampSchema,
    ingestedAt: Schema.Union(LocalAiTimestampSchema, Schema.Null),
    freshUntil: Schema.Union(LocalAiTimestampSchema, Schema.Null),
    sourceId: LocalAiEvidenceSourceIdSchema,
    adapterId: LocalAiEvidenceAdapterIdSchema,
    device: ParentDeviceReferenceSchema,
    childProfile: ChildProfileReferenceSchema,
    custody: LocalAiEvidenceCustodySchema,
    retentionState: LocalAiEvidenceRetentionStateSchema,
    confidence: Schema.Union(LocalAiConfidenceSchema, Schema.Null),
    confidenceKind: Schema.Union(LocalAiConfidenceKindSchema, Schema.Null),
    capabilityStatus: LocalAiContextCapabilityStatusSchema,
    degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
    unknownReasons: Schema.Array(LocalAiContextReasonCodeSchema),
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  }).pipe(
    Schema.filter(
      (reference) =>
        reference.sourceEvidenceReferences.length > 0 || 'Expected local AI context evidence to cite stored evidence'
    )
  )
);

export const LocalAiEvidenceContextBuildRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: LocalAiEvaluationRequestIdSchema,
    requestedAt: LocalAiTimestampSchema,
    childProfile: ChildProfileReferenceSchema,
    device: ParentDeviceReferenceSchema,
    requestedEvaluationKind: LocalAiRequestedEvaluationKindSchema,
    requiredEvidenceKinds: Schema.Array(LocalAiEvidenceContextKindSchema),
    parentRuleReferences: Schema.Array(PolicyRuleIdSchema),
    modelTaskRequirements: Schema.Array(LocalAiCapabilityFlagSchema),
    allowedCustody: Schema.Array(LocalAiEvidenceCustodySchema),
    promptVersion: LocalAiPromptVersionSchema,
  })
);

export const LocalAiEvidenceContextValidationSummarySchema = withParser(
  Schema.Struct({
    evidenceReferenceCount: LocalAiContextNonNegativeCountSchema,
    sourceEvidenceReferenceCount: LocalAiContextNonNegativeCountSchema,
    runtimeReferenceCount: LocalAiContextNonNegativeCountSchema,
    memoryReferenceCount: LocalAiContextNonNegativeCountSchema,
    graphReferenceCount: LocalAiContextNonNegativeCountSchema,
    forbiddenCustodyReferenceCount: LocalAiContextNonNegativeCountSchema,
  })
);

export const LocalAiEvidenceContextSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    contextId: LocalAiEvidenceContextIdSchema,
    requestId: LocalAiEvaluationRequestIdSchema,
    childProfile: ChildProfileReferenceSchema,
    device: ParentDeviceReferenceSchema,
    evidenceReferences: Schema.Array(LocalAiEvidenceContextSourceRefSchema),
    browserEvidenceRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
    appGameEvidenceRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
    networkFlowEvidenceRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
    screenSummaryRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
    parentRuleReferences: Schema.Array(PolicyRuleIdSchema),
    recentActivitySummaryRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
    memoryReferences: Schema.Array(LocalAiMemoryReferenceSchema),
    graphReferences: Schema.Array(LocalAiGraphReferenceSchema),
    localModelRuntimeRefs: Schema.Array(LocalAiRuntimeReferenceIdSchema),
    promptVersion: LocalAiPromptVersionSchema,
    custodyLabels: Schema.Array(LocalAiEvidenceCustodySchema),
    degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
    unknownReasons: Schema.Array(LocalAiContextReasonCodeSchema),
    validationSummary: LocalAiEvidenceContextValidationSummarySchema,
  })
);

export const LocalAiEvidenceContextBuildResultSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: LocalAiEvaluationRequestIdSchema,
    state: LocalAiContextBuildStateSchema,
    context: Schema.Union(LocalAiEvidenceContextSchema, Schema.Null),
    rejectedFields: Schema.Array(LocalAiRejectedFieldSchema),
    missingEvidenceKinds: Schema.Array(LocalAiEvidenceContextKindSchema),
    degradedSourceRefs: Schema.Array(LocalAiEvidenceContextRefIdSchema),
    custodyBoundarySummary: LocalAiEvidenceContextSummarySchema,
    validationGateSummary: LocalAiEvidenceContextSummarySchema,
    auditEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  })
);

export const LocalAiStoredEvidenceContextBuildInputSchema = withParser(
  Schema.Struct({
    contextId: LocalAiEvidenceContextIdSchema,
    request: LocalAiEvidenceContextBuildRequestSchema,
    evidenceReferences: Schema.Array(LocalAiEvidenceContextSourceRefSchema),
    runtimeReferences: Schema.Array(LocalModelRuntimeStatusSchema),
    memoryReferences: Schema.Array(LocalAiMemoryReferenceSchema),
    graphReferences: Schema.Array(LocalAiGraphReferenceSchema),
  })
);

export type LocalAiEvidenceContextSourceRef = Infer<typeof LocalAiEvidenceContextSourceRefSchema>;
export type LocalAiEvidenceContextBuildRequest = Infer<typeof LocalAiEvidenceContextBuildRequestSchema>;
export type LocalAiEvidenceContextValidationSummary = Infer<typeof LocalAiEvidenceContextValidationSummarySchema>;
export type LocalAiEvidenceContext = Infer<typeof LocalAiEvidenceContextSchema>;
export type LocalAiEvidenceContextBuildResult = Infer<typeof LocalAiEvidenceContextBuildResultSchema>;
export type LocalAiStoredEvidenceContextBuildInput = Infer<typeof LocalAiStoredEvidenceContextBuildInputSchema>;
