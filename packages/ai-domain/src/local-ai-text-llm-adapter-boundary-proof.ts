import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { LocalAiEvaluationInputSchema } from '@ocentra-parent/ai-domain/local-ai';
import { LocalAiPromptVersionSchema } from './local-ai-primitives';
import { LocalModelRuntimeStatusSchema, type LocalModelRuntimeStatus } from '@ocentra-parent/ai-domain/local-ai-runtime';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const LocalAiTextLlmAdapterRequestIdSchema = brandedNonEmptyStringSchema('LocalAiTextLlmAdapterRequestId');
export const LocalAiTextLlmAdapterTraceRefSchema = brandedNonEmptyStringSchema('LocalAiTextLlmAdapterTraceRef');
export const LocalAiTextLlmAdapterParserRefSchema = brandedNonEmptyStringSchema('LocalAiTextLlmAdapterParserRef');
export const LocalAiTextLlmAdapterNonClaimSchema = brandedNonEmptyStringSchema('LocalAiTextLlmAdapterNonClaim');

export const LocalAiTextLlmAdapterBoundaryStateSchema = withParser(
  Schema.Literal('ready-for-local-adapter', 'manual-required', 'unavailable')
);

const LocalAiTextLlmAdapterBoundaryInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  adapterRequestId: LocalAiTextLlmAdapterRequestIdSchema,
  evaluationInput: LocalAiEvaluationInputSchema,
  modelRuntime: LocalModelRuntimeStatusSchema,
  promptVersion: LocalAiPromptVersionSchema,
  localAdapterAvailable: Schema.Boolean,
  manualProofRequired: Schema.Boolean,
  rawPromptRetained: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
});

type LocalAiTextLlmAdapterBoundaryInputCandidate = Infer<typeof LocalAiTextLlmAdapterBoundaryInputBaseSchema>;

export const LocalAiTextLlmAdapterBoundaryInputSchema = withParser(
  LocalAiTextLlmAdapterBoundaryInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        localAiTextLlmAdapterBoundaryInputIsValid(input) ||
        'Expected local text LLM adapter input to match local runtime metadata without raw prompt or output retention'
    )
  )
);

const LocalAiTextLlmAdapterBoundaryProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  adapterRequestId: LocalAiTextLlmAdapterRequestIdSchema,
  state: LocalAiTextLlmAdapterBoundaryStateSchema,
  runtimeReferenceId: NonEmptyStringSchema,
  providerId: NonEmptyStringSchema,
  modelId: NonEmptyStringSchema,
  promptVersion: LocalAiPromptVersionSchema,
  evidenceReferenceCount: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  parentRuleReferenceCount: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
  localOnly: Schema.Boolean,
  adapterBoundaryOnly: Schema.Boolean,
  parserRequiredBeforeResult: Schema.Boolean,
  modelExecuted: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  productionModelQualityClaimed: Schema.Boolean,
  rawPromptRetained: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
  adapterTraceRefs: Schema.Array(LocalAiTextLlmAdapterTraceRefSchema),
  parserRefs: Schema.Array(LocalAiTextLlmAdapterParserRefSchema),
  nonClaims: Schema.Array(LocalAiTextLlmAdapterNonClaimSchema),
});

type LocalAiTextLlmAdapterBoundaryProofCandidate = Infer<typeof LocalAiTextLlmAdapterBoundaryProofBaseSchema>;

export const LocalAiTextLlmAdapterBoundaryProofSchema = withParser(
  LocalAiTextLlmAdapterBoundaryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localAiTextLlmAdapterBoundaryProofIsHonest(proof) ||
        'Expected local text LLM adapter boundary proof to stay local-only, parser-gated, non-enforcing, and non-retaining'
    )
  )
);

export type LocalAiTextLlmAdapterBoundaryInput = Infer<typeof LocalAiTextLlmAdapterBoundaryInputSchema>;
export type LocalAiTextLlmAdapterBoundaryProof = Infer<typeof LocalAiTextLlmAdapterBoundaryProofSchema>;
export type LocalAiTextLlmAdapterBoundaryState = Infer<typeof LocalAiTextLlmAdapterBoundaryStateSchema>;

const decodeNonClaim = Schema.decodeUnknownSync(LocalAiTextLlmAdapterNonClaimSchema);
const decodeTraceRef = Schema.decodeUnknownSync(LocalAiTextLlmAdapterTraceRefSchema);
const decodeParserRef = Schema.decodeUnknownSync(LocalAiTextLlmAdapterParserRefSchema);

export const LocalAiTextLlmAdapterBoundaryNonClaims = [
  decodeNonClaim('This proof validates only the local text LLM adapter boundary and does not execute a model.'),
  decodeNonClaim('Raw prompt text and raw model output are not retained across the adapter boundary.'),
  decodeNonClaim(
    'Remote/API AI, policy authority, enforcement, portal UI, and production model quality are unclaimed.'
  ),
] as const;

export function proveLocalAiTextLlmAdapterBoundary(input: unknown): LocalAiTextLlmAdapterBoundaryProof {
  const parsed = LocalAiTextLlmAdapterBoundaryInputSchema.parse(input);

  return LocalAiTextLlmAdapterBoundaryProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    adapterRequestId: parsed.adapterRequestId,
    state: stateFor(parsed),
    runtimeReferenceId: parsed.modelRuntime.runtimeReferenceId,
    providerId: parsed.modelRuntime.providerId,
    modelId: parsed.modelRuntime.modelId,
    promptVersion: parsed.promptVersion,
    evidenceReferenceCount: parsed.evaluationInput.evidenceReferences.length,
    parentRuleReferenceCount: parsed.evaluationInput.parentRuleReferences.length,
    localOnly: true,
    adapterBoundaryOnly: true,
    parserRequiredBeforeResult: true,
    modelExecuted: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    productionModelQualityClaimed: false,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    adapterTraceRefs: [decodeTraceRef(`local-ai-text-adapter:${parsed.evaluationInput.requestId}`)],
    parserRefs: [decodeParserRef(`local-ai-text-parser:${parsed.promptVersion}`)],
    nonClaims: LocalAiTextLlmAdapterBoundaryNonClaims,
  });
}

function stateFor(input: LocalAiTextLlmAdapterBoundaryInput): LocalAiTextLlmAdapterBoundaryState {
  if (runtimeUnavailable(input.modelRuntime)) {
    return 'unavailable';
  }
  if (!input.localAdapterAvailable || input.manualProofRequired) {
    return 'manual-required';
  }
  return 'ready-for-local-adapter';
}

function runtimeUnavailable(runtime: LocalModelRuntimeStatus): boolean {
  return runtime.loadState === 'unavailable' || runtime.executionState === 'disabled';
}

function localAiTextLlmAdapterBoundaryInputIsValid(input: LocalAiTextLlmAdapterBoundaryInputCandidate): boolean {
  return (
    !input.rawPromptRetained &&
    !input.rawModelOutputRetained &&
    input.promptVersion === input.evaluationInput.modelRequest.promptVersion &&
    input.modelRuntime.privacyMode === 'local-only' &&
    input.modelRuntime.providerId === input.evaluationInput.modelRequest.providerId &&
    input.modelRuntime.modelId === input.evaluationInput.modelRequest.modelId
  );
}

function localAiTextLlmAdapterBoundaryProofIsHonest(proof: LocalAiTextLlmAdapterBoundaryProofCandidate): boolean {
  return (
    proof.localOnly &&
    proof.adapterBoundaryOnly &&
    proof.parserRequiredBeforeResult &&
    !proof.modelExecuted &&
    !proof.remoteApiClaimed &&
    !proof.policyAuthorityClaimed &&
    !proof.enforcementClaimed &&
    !proof.productionModelQualityClaimed &&
    !proof.rawPromptRetained &&
    !proof.rawModelOutputRetained &&
    proof.adapterTraceRefs.length > 0 &&
    proof.parserRefs.length > 0
  );
}

