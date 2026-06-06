import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { PolicyRuleIdSchema } from './policy';
import { ParentEvidenceReferenceSchema } from './references';
import { ParentContractSchemaVersionSchema } from './reference-primitives';
import { LocalAiEvaluationInputSchema, LocalAiSafetyResultSchema } from './local-ai';
import {
  LocalAiCapabilityFlagSchema,
  LocalAiModelIdSchema,
  LocalAiPromptVersionSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
} from './local-ai-primitives';
import { LocalModelRuntimeStatusSchema } from './local-ai-runtime';

const NonEmptyLocalAiPromptTemplateVersionsSchema = <A, I, R>(item: Schema.Schema<A, I, R>) =>
  Schema.Array(item).pipe(Schema.filter((value) => value.length > 0 || 'Expected at least one local AI prompt record'));

export const LocalAiPromptTemplateLifecycleStateSchema = withParser(Schema.Literal('active', 'superseded', 'invalid'));

export const LocalAiPromptTemplateVersionClaimBoundariesSchema = withParser(
  Schema.Struct({
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    rawPromptRetained: Schema.Literal(false),
    rawTemplateTextRetained: Schema.Literal(false),
    modelQualityClaimed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    uiClaimed: Schema.Literal(false),
  })
);

const LocalAiPromptTemplateVersionRecordBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  promptVersion: LocalAiPromptVersionSchema,
  lifecycleState: LocalAiPromptTemplateLifecycleStateSchema,
  compatibleModelIds: NonEmptyLocalAiPromptTemplateVersionsSchema(LocalAiModelIdSchema),
  compatibleRuntimeRefs: NonEmptyLocalAiPromptTemplateVersionsSchema(LocalAiRuntimeReferenceIdSchema),
  taskRequirements: NonEmptyLocalAiPromptTemplateVersionsSchema(LocalAiCapabilityFlagSchema),
  evidenceReferences: NonEmptyLocalAiPromptTemplateVersionsSchema(ParentEvidenceReferenceSchema),
  parentRuleReferences: NonEmptyLocalAiPromptTemplateVersionsSchema(PolicyRuleIdSchema),
  generatedAt: LocalAiTimestampSchema,
  validFrom: LocalAiTimestampSchema,
  validUntil: Schema.Union(LocalAiTimestampSchema, Schema.Null),
  supersededByPromptVersion: Schema.Union(LocalAiPromptVersionSchema, Schema.Null),
  rawPromptRetained: Schema.Literal(false),
  rawTemplateTextRetained: Schema.Literal(false),
  remoteAiRequired: Schema.Literal(false),
});

export const LocalAiPromptTemplateVersionRecordSchema = withParser(
  LocalAiPromptTemplateVersionRecordBaseSchema.pipe(
    Schema.filter(
      (record) =>
        localAiPromptTemplateVersionRecordIsConsistent(record) ||
        'Expected local AI prompt template record to be active, local-only, cited, and lifecycle-consistent'
    )
  )
);

export const LocalAiPromptTemplateVersionProofInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    evaluationInput: LocalAiEvaluationInputSchema,
    safetyResult: LocalAiSafetyResultSchema,
    runtimeStatus: LocalModelRuntimeStatusSchema,
    promptRecords: NonEmptyLocalAiPromptTemplateVersionsSchema(LocalAiPromptTemplateVersionRecordSchema),
    claimBoundaries: LocalAiPromptTemplateVersionClaimBoundariesSchema,
  })
);

export const LocalAiPromptTemplateVersionProofSummarySchema = withParser(
  Schema.Struct({
    inputPromptRecordCount: Schema.Number,
    selectedPromptRecordCount: Schema.Number,
    evidenceReferenceCount: Schema.Number,
    parentRuleReferenceCount: Schema.Number,
    compatibleModelCount: Schema.Number,
    compatibleRuntimeCount: Schema.Number,
  })
);

export const LocalAiPromptTemplateVersionProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofKind: Schema.Literal('local-ai-prompt-template-version-proof'),
    promptRecord: LocalAiPromptTemplateVersionRecordSchema,
    evaluationInput: LocalAiEvaluationInputSchema,
    safetyResult: LocalAiSafetyResultSchema,
    runtimeStatus: LocalModelRuntimeStatusSchema,
    summary: LocalAiPromptTemplateVersionProofSummarySchema,
    claimBoundaries: LocalAiPromptTemplateVersionClaimBoundariesSchema,
  }).pipe(
    Schema.filter(
      (proof) =>
        localAiPromptTemplateVersionProofIsReady(proof) ||
        'Expected prompt template version proof to align request, result, runtime, citations, and local-only boundaries'
    )
  )
);

export type LocalAiPromptTemplateLifecycleState = Infer<typeof LocalAiPromptTemplateLifecycleStateSchema>;
export type LocalAiPromptTemplateVersionClaimBoundaries = Infer<
  typeof LocalAiPromptTemplateVersionClaimBoundariesSchema
>;
export type LocalAiPromptTemplateVersionRecord = Infer<typeof LocalAiPromptTemplateVersionRecordSchema>;
export type LocalAiPromptTemplateVersionProofInput = Infer<typeof LocalAiPromptTemplateVersionProofInputSchema>;
export type LocalAiPromptTemplateVersionProof = Infer<typeof LocalAiPromptTemplateVersionProofSchema>;

function localAiPromptTemplateVersionRecordIsConsistent(
  record: Infer<typeof LocalAiPromptTemplateVersionRecordBaseSchema>
): boolean {
  if (record.lifecycleState === 'active') {
    return record.validUntil === null && record.supersededByPromptVersion === null;
  }

  if (record.lifecycleState === 'superseded') {
    return record.validUntil !== null && record.supersededByPromptVersion !== null;
  }

  return record.validUntil !== null;
}

function selectedPromptRecord(
  input: LocalAiPromptTemplateVersionProofInput
): LocalAiPromptTemplateVersionRecord | null {
  return (
    input.promptRecords.find(
      (record) =>
        record.lifecycleState === 'active' &&
        record.promptVersion === input.evaluationInput.modelRequest.promptVersion &&
        record.promptVersion === input.safetyResult.promptVersion &&
        record.compatibleModelIds.includes(input.evaluationInput.modelRequest.modelId) &&
        record.compatibleModelIds.includes(input.runtimeStatus.modelId) &&
        record.compatibleRuntimeRefs.includes(input.runtimeStatus.runtimeReferenceId) &&
        record.taskRequirements.every((task) => input.runtimeStatus.capabilityFlags.includes(task)) &&
        input.evaluationInput.evidenceReferences.every((reference) =>
          record.evidenceReferences.some(
            (recordReference) => recordReference.evidenceReferenceId === reference.evidenceReferenceId
          )
        ) &&
        input.evaluationInput.parentRuleReferences.every((ruleRef) => record.parentRuleReferences.includes(ruleRef))
    ) ?? null
  );
}

function localAiPromptTemplateVersionProofIsReady(candidate: {
  promptRecord: LocalAiPromptTemplateVersionRecord;
  evaluationInput: LocalAiPromptTemplateVersionProofInput['evaluationInput'];
  safetyResult: LocalAiPromptTemplateVersionProofInput['safetyResult'];
  runtimeStatus: LocalAiPromptTemplateVersionProofInput['runtimeStatus'];
  summary: Infer<typeof LocalAiPromptTemplateVersionProofSummarySchema>;
}): boolean {
  return (
    candidate.promptRecord.promptVersion === candidate.evaluationInput.modelRequest.promptVersion &&
    candidate.promptRecord.promptVersion === candidate.safetyResult.promptVersion &&
    candidate.evaluationInput.modelRequest.modelId === candidate.runtimeStatus.modelId &&
    candidate.summary.selectedPromptRecordCount === 1 &&
    candidate.summary.evidenceReferenceCount === candidate.promptRecord.evidenceReferences.length &&
    candidate.summary.parentRuleReferenceCount === candidate.promptRecord.parentRuleReferences.length
  );
}

export function buildLocalAiPromptTemplateVersionProof(input: unknown): LocalAiPromptTemplateVersionProof {
  const parsed = LocalAiPromptTemplateVersionProofInputSchema.parse(input);
  const promptRecord = selectedPromptRecord(parsed);
  if (promptRecord === null) {
    throw new Error('Expected one active local AI prompt template version record for the evaluation request');
  }

  return LocalAiPromptTemplateVersionProofSchema.parse({
    schemaVersion: parsed.schemaVersion,
    proofKind: 'local-ai-prompt-template-version-proof',
    promptRecord,
    evaluationInput: parsed.evaluationInput,
    safetyResult: parsed.safetyResult,
    runtimeStatus: parsed.runtimeStatus,
    summary: {
      inputPromptRecordCount: parsed.promptRecords.length,
      selectedPromptRecordCount: 1,
      evidenceReferenceCount: promptRecord.evidenceReferences.length,
      parentRuleReferenceCount: promptRecord.parentRuleReferences.length,
      compatibleModelCount: promptRecord.compatibleModelIds.length,
      compatibleRuntimeCount: promptRecord.compatibleRuntimeRefs.length,
    },
    claimBoundaries: parsed.claimBoundaries,
  });
}
