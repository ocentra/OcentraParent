import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { LocalAiEvaluationInputSchema, LocalAiSafetyResultSchema, type LocalAiSafetyResult } from './local-ai';
import { LocalAiContextKindSchema, LocalAiPromptVersionSchema } from './ai-primitives';
import { LocalModelRuntimeStatusSchema } from './ai-runtime';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from './family-reference-primitives';
import {
  LocalAiDeterministicClassifierNonClaims,
  LocalAiDeterministicClassifierNonClaimSchema,
  LocalAiDeterministicClassifierTraceRefSchema,
  decodeTraceRef,
  deterministicClassifierInputIsReady,
  deterministicClassifierResultIsHonest,
  resultFor,
  stateFor,
} from './local-ai-deterministic-classifier-proof-logic';

export const LocalAiDeterministicClassifierRunIdSchema = brandedNonEmptyStringSchema(
  'LocalAiDeterministicClassifierRunId'
);

export const LocalAiDeterministicClassifierStateSchema = withParser(
  Schema.Literal('classified', 'low-confidence', 'missing-evidence', 'runtime-unavailable')
);

const LocalAiDeterministicClassifierInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  classifierRunId: LocalAiDeterministicClassifierRunIdSchema,
  evaluationInput: LocalAiEvaluationInputSchema,
  modelRuntime: LocalModelRuntimeStatusSchema,
  rawEvidenceRetained: Schema.Boolean,
});

export const LocalAiDeterministicClassifierInputSchema = withParser(
  LocalAiDeterministicClassifierInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        deterministicClassifierInputIsReady(input) ||
        'Expected deterministic classifier input to use matching local runtime metadata without raw evidence retention'
    )
  )
);

const LocalAiDeterministicClassifierResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  classifierRunId: LocalAiDeterministicClassifierRunIdSchema,
  state: LocalAiDeterministicClassifierStateSchema,
  result: LocalAiSafetyResultSchema,
  contextKind: LocalAiContextKindSchema,
  modelRuntime: LocalModelRuntimeStatusSchema,
  promptVersion: LocalAiPromptVersionSchema,
  evidenceReferenceCount: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  parentRuleReferenceCount: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  dryRun: Schema.Boolean,
  deterministicOnly: Schema.Boolean,
  localOnly: Schema.Boolean,
  modelExecuted: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  productionModelQualityClaimed: Schema.Boolean,
  rawEvidenceRetained: Schema.Boolean,
  classifierTraceRefs: Schema.Array(LocalAiDeterministicClassifierTraceRefSchema),
  nonClaims: Schema.Array(LocalAiDeterministicClassifierNonClaimSchema),
});

export const LocalAiDeterministicClassifierResultSchema = withParser(
  LocalAiDeterministicClassifierResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        deterministicClassifierResultIsHonest(result) ||
        'Expected deterministic classifier result to stay local-only, deterministic-only, non-enforcing, and non-retaining'
    )
  )
);

export type LocalAiDeterministicClassifierInput = Infer<typeof LocalAiDeterministicClassifierInputSchema>;
export type LocalAiDeterministicClassifierResult = Infer<typeof LocalAiDeterministicClassifierResultSchema>;
export type LocalAiDeterministicClassifierState = Infer<typeof LocalAiDeterministicClassifierStateSchema>;

export function runLocalAiDeterministicClassifier(input: unknown): LocalAiDeterministicClassifierResult {
  const parsed = LocalAiDeterministicClassifierInputSchema.parse(input);
  const result = resultFor(parsed);

  return LocalAiDeterministicClassifierResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    classifierRunId: parsed.classifierRunId,
    state: stateFor(parsed, result),
    result,
    contextKind: parsed.evaluationInput.currentObservation.contextKind,
    modelRuntime: parsed.modelRuntime,
    promptVersion: parsed.evaluationInput.modelRequest.promptVersion,
    evidenceReferenceCount: parsed.evaluationInput.evidenceReferences.length,
    parentRuleReferenceCount: parsed.evaluationInput.parentRuleReferences.length,
    dryRun: true,
    deterministicOnly: true,
    localOnly: true,
    modelExecuted: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    productionModelQualityClaimed: false,
    rawEvidenceRetained: false,
    classifierTraceRefs: [decodeTraceRef(`local-ai-deterministic-classifier:${parsed.evaluationInput.requestId}`)],
    nonClaims: LocalAiDeterministicClassifierNonClaims,
  });
}
