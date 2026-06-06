import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LocalAiSafetyResultSchema } from './local-ai';
import { LocalAiTextLlmAdapterBoundaryProofSchema } from './local-ai-text-llm-adapter-boundary-proof';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from './reference-primitives';

const NonEmptyText = Schema.String.pipe(Schema.minLength(1));

export const LocalAiTextOutputParserRunIdSchema = NonEmptyText.pipe(Schema.brand('LocalAiTextOutputParserRunId'));
export const LocalAiTextOutputParserTraceRefSchema = NonEmptyText.pipe(Schema.brand('LocalAiTextOutputParserTraceRef'));
export const LocalAiTextOutputParserNonClaimSchema = NonEmptyText.pipe(Schema.brand('LocalAiTextOutputParserNonClaim'));

export const LocalAiTextOutputParserStateSchema = withParser(
  Schema.Literal('parsed-local-result', 'rejected-invalid-output', 'manual-required')
);

const LocalAiTextOutputParserInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parserRunId: LocalAiTextOutputParserRunIdSchema,
  adapterProof: LocalAiTextLlmAdapterBoundaryProofSchema,
  candidateOutput: Schema.Unknown,
  rawModelOutputRetained: Schema.Boolean,
});

type LocalAiTextOutputParserInputCandidate = Infer<typeof LocalAiTextOutputParserInputBaseSchema>;

export const LocalAiTextOutputParserInputSchema = withParser(
  LocalAiTextOutputParserInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        localAiTextOutputParserInputIsValid(input) ||
        'Expected local text output parser input to use a local adapter proof without raw model-output retention'
    )
  )
);

const LocalAiTextOutputParserProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parserRunId: LocalAiTextOutputParserRunIdSchema,
  state: LocalAiTextOutputParserStateSchema,
  adapterRequestId: NonEmptyText,
  result: Schema.Union(LocalAiSafetyResultSchema, Schema.Null),
  parserRejectedOutput: Schema.Boolean,
  resultPolicyEligible: Schema.Boolean,
  localOnly: Schema.Boolean,
  parserBoundaryOnly: Schema.Boolean,
  modelExecuted: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  productionModelQualityClaimed: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
  outputTraceRefs: Schema.Array(LocalAiTextOutputParserTraceRefSchema),
  nonClaims: Schema.Array(LocalAiTextOutputParserNonClaimSchema),
});

type LocalAiTextOutputParserProofCandidate = Infer<typeof LocalAiTextOutputParserProofBaseSchema>;

export const LocalAiTextOutputParserProofSchema = withParser(
  LocalAiTextOutputParserProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localAiTextOutputParserProofIsHonest(proof) ||
        'Expected local text output parser proof to reject invalid output before policy eligibility without raw retention'
    )
  )
);

export type LocalAiTextOutputParserInput = Infer<typeof LocalAiTextOutputParserInputSchema>;
export type LocalAiTextOutputParserProof = Infer<typeof LocalAiTextOutputParserProofSchema>;
export type LocalAiTextOutputParserState = Infer<typeof LocalAiTextOutputParserStateSchema>;

const decodeNonClaim = Schema.decodeUnknownSync(LocalAiTextOutputParserNonClaimSchema);
const decodeTraceRef = Schema.decodeUnknownSync(LocalAiTextOutputParserTraceRefSchema);

export const LocalAiTextOutputParserNonClaims = [
  decodeNonClaim('This proof validates the local text output parser boundary without executing a model.'),
  decodeNonClaim('Malformed, remote, or metadata-mismatched output is rejected before policy eligibility.'),
  decodeNonClaim(
    'Raw model output, remote/API AI, policy authority, enforcement, portal UI, and model quality are unclaimed.'
  ),
] as const;

export function parseLocalAiTextOutput(input: unknown): LocalAiTextOutputParserProof {
  const parsed = LocalAiTextOutputParserInputSchema.parse(input);
  const candidate = LocalAiSafetyResultSchema.safeParse(parsed.candidateOutput);
  const adapterReady = parsed.adapterProof.state === 'ready-for-local-adapter';
  const accepted = adapterReady && candidate.success && candidateMatchesAdapter(candidate.data, parsed.adapterProof);

  return LocalAiTextOutputParserProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    parserRunId: parsed.parserRunId,
    state: stateFor(parsed, accepted),
    adapterRequestId: parsed.adapterProof.adapterRequestId,
    result: accepted && candidate.success ? candidate.data : null,
    parserRejectedOutput: !accepted,
    resultPolicyEligible: accepted,
    localOnly: true,
    parserBoundaryOnly: true,
    modelExecuted: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    productionModelQualityClaimed: false,
    rawModelOutputRetained: false,
    outputTraceRefs: [decodeTraceRef(`local-ai-text-output-parser:${parsed.parserRunId}`)],
    nonClaims: LocalAiTextOutputParserNonClaims,
  });
}

function localAiTextOutputParserInputIsValid(input: LocalAiTextOutputParserInputCandidate): boolean {
  return !input.rawModelOutputRetained && input.adapterProof.localOnly && input.adapterProof.parserRequiredBeforeResult;
}

function stateFor(input: LocalAiTextOutputParserInput, accepted: boolean): LocalAiTextOutputParserState {
  if (input.adapterProof.state !== 'ready-for-local-adapter') {
    return 'manual-required';
  }
  if (!accepted) {
    return 'rejected-invalid-output';
  }
  return 'parsed-local-result';
}

function candidateMatchesAdapter(
  result: typeof LocalAiSafetyResultSchema.Type,
  adapterProof: typeof LocalAiTextLlmAdapterBoundaryProofSchema.Type
): boolean {
  return (
    result.promptVersion === adapterProof.promptVersion &&
    result.modelRuntime.privacyMode === 'local-only' &&
    result.modelRuntime.runtimeReferenceId === adapterProof.runtimeReferenceId &&
    result.modelRuntime.providerId === adapterProof.providerId &&
    result.modelRuntime.modelId === adapterProof.modelId &&
    result.evidenceReferences.length === adapterProof.evidenceReferenceCount &&
    result.parentRuleReferences.length === adapterProof.parentRuleReferenceCount
  );
}

function localAiTextOutputParserProofIsHonest(proof: LocalAiTextOutputParserProofCandidate): boolean {
  if (!proofKeepsParserBoundary(proof)) {
    return false;
  }

  if (proof.state === 'parsed-local-result') {
    return parsedResultIsPolicyEligible(proof);
  }

  return rejectedResultIsNotPolicyEligible(proof);
}

function proofKeepsParserBoundary(proof: LocalAiTextOutputParserProofCandidate): boolean {
  return (
    proof.localOnly &&
    proof.parserBoundaryOnly &&
    !proof.modelExecuted &&
    !proof.remoteApiClaimed &&
    !proof.policyAuthorityClaimed &&
    !proof.enforcementClaimed &&
    !proof.productionModelQualityClaimed &&
    !proof.rawModelOutputRetained &&
    proof.outputTraceRefs.length > 0
  );
}

function parsedResultIsPolicyEligible(proof: LocalAiTextOutputParserProofCandidate): boolean {
  return Boolean(proof.result) && !proof.parserRejectedOutput && proof.resultPolicyEligible;
}

function rejectedResultIsNotPolicyEligible(proof: LocalAiTextOutputParserProofCandidate): boolean {
  return proof.result === null && proof.parserRejectedOutput && !proof.resultPolicyEligible;
}
