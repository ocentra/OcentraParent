import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LocalAiEvaluationInputSchema, LocalAiSafetyResultSchema } from './local-ai';
import { LocalAiEvidenceContextBuildRequestSchema } from './local-ai-context';
import {
  LocalAiCapabilityFlagSchema,
  LocalAiModelIdSchema,
  LocalAiPromptVersionSchema,
  LocalAiProviderIdSchema,
} from './local-ai-primitives';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';

const LocalAiPromptTemplateProofTextSchema = Schema.String.pipe(Schema.minLength(1));
const LocalAiPromptTemplateProofCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiPromptTemplateRefSchema = LocalAiPromptTemplateProofTextSchema.pipe(
  Schema.brand('LocalAiPromptTemplateRef')
);

export const LocalAiPromptTemplateInputBindingSchema = withParser(
  Schema.Literal('context-builder', 'evaluation-input', 'safety-result')
);

export const LocalAiPromptTemplateVersionRowSchema = withParser(
  Schema.Struct({
    templateRef: LocalAiPromptTemplateRefSchema,
    promptVersion: LocalAiPromptVersionSchema,
    providerId: LocalAiProviderIdSchema,
    modelId: LocalAiModelIdSchema,
    task: LocalAiCapabilityFlagSchema,
    inputBinding: LocalAiPromptTemplateInputBindingSchema,
    outputSchemaRef: LocalAiPromptTemplateProofTextSchema,
    active: Schema.Boolean,
    rawPromptRetained: Schema.Literal(false),
    rawModelOutputRetained: Schema.Literal(false),
  })
);

export const LocalAiPromptTemplateVersionClaimBoundariesSchema = withParser(
  Schema.Struct({
    modelExecutionClaimed: Schema.Literal(false),
    modelQualityClaimed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    portalUiClaimed: Schema.Literal(false),
    remoteApiAiUsed: Schema.Literal(false),
    rawPromptRetained: Schema.Literal(false),
    rawModelOutputRetained: Schema.Literal(false),
  })
);

export const LocalAiPromptTemplateVersionProofInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    contextRequest: LocalAiEvidenceContextBuildRequestSchema,
    evaluationInput: LocalAiEvaluationInputSchema,
    safetyResult: LocalAiSafetyResultSchema,
    templateRows: Schema.Array(LocalAiPromptTemplateVersionRowSchema),
    claimBoundaries: LocalAiPromptTemplateVersionClaimBoundariesSchema,
  })
);

export const LocalAiPromptTemplateVersionSummarySchema = withParser(
  Schema.Struct({
    templateRowCount: LocalAiPromptTemplateProofCountSchema,
    activeTemplateRowCount: LocalAiPromptTemplateProofCountSchema,
    inputBindingCount: LocalAiPromptTemplateProofCountSchema,
    promptVersionMatchCount: LocalAiPromptTemplateProofCountSchema,
    nonRetainingTemplateRowCount: LocalAiPromptTemplateProofCountSchema,
  })
);

const LocalAiPromptTemplateVersionProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofKind: Schema.Literal('local-ai-prompt-template-version-proof'),
  contextRequest: LocalAiEvidenceContextBuildRequestSchema,
  evaluationInput: LocalAiEvaluationInputSchema,
  safetyResult: LocalAiSafetyResultSchema,
  selectedTemplateRows: Schema.Array(LocalAiPromptTemplateVersionRowSchema),
  summary: LocalAiPromptTemplateVersionSummarySchema,
  claimBoundaries: LocalAiPromptTemplateVersionClaimBoundariesSchema,
});

type LocalAiPromptTemplateVersionProofCandidate = Infer<typeof LocalAiPromptTemplateVersionProofBaseSchema>;

export const LocalAiPromptTemplateVersionProofSchema = withParser(
  LocalAiPromptTemplateVersionProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localAiPromptTemplateVersionProofIsReady(proof) ||
        'Expected local AI prompt/template proof to reconcile context, input, result, provider, model, and non-retention refs'
    )
  )
);

export type LocalAiPromptTemplateRef = typeof LocalAiPromptTemplateRefSchema.Type;
export type LocalAiPromptTemplateInputBinding = Infer<typeof LocalAiPromptTemplateInputBindingSchema>;
export type LocalAiPromptTemplateVersionRow = Infer<typeof LocalAiPromptTemplateVersionRowSchema>;
export type LocalAiPromptTemplateVersionClaimBoundaries = Infer<
  typeof LocalAiPromptTemplateVersionClaimBoundariesSchema
>;
export type LocalAiPromptTemplateVersionProofInput = Infer<typeof LocalAiPromptTemplateVersionProofInputSchema>;
export type LocalAiPromptTemplateVersionProof = Infer<typeof LocalAiPromptTemplateVersionProofSchema>;

function selectedTemplateRows(input: LocalAiPromptTemplateVersionProofInput): LocalAiPromptTemplateVersionRow[] {
  return input.templateRows.filter((row) => {
    return (
      row.active &&
      row.promptVersion === input.evaluationInput.modelRequest.promptVersion &&
      row.providerId === input.evaluationInput.modelRequest.providerId &&
      row.modelId === input.evaluationInput.modelRequest.modelId
    );
  });
}

function summaryFor(
  input: LocalAiPromptTemplateVersionProofInput,
  rows: readonly LocalAiPromptTemplateVersionRow[]
): Infer<typeof LocalAiPromptTemplateVersionSummarySchema> {
  const bindings = new Set(rows.map((row) => row.inputBinding));
  return {
    templateRowCount: input.templateRows.length,
    activeTemplateRowCount: input.templateRows.filter((row) => row.active).length,
    inputBindingCount: bindings.size,
    promptVersionMatchCount: rows.length,
    nonRetainingTemplateRowCount: rows.filter((row) => !row.rawPromptRetained && !row.rawModelOutputRetained).length,
  };
}

function localAiPromptTemplateVersionProofIsReady(proof: LocalAiPromptTemplateVersionProofCandidate): boolean {
  return [
    promptVersionRefsMatch(proof),
    selectedRowsCoverBindings(proof),
    selectedRowsAreNonRetaining(proof),
    selectedRowsMatchProviderAndModel(proof),
  ].every(Boolean);
}

function promptVersionRefsMatch(proof: LocalAiPromptTemplateVersionProofCandidate): boolean {
  return (
    proof.contextRequest.promptVersion === proof.evaluationInput.modelRequest.promptVersion &&
    proof.safetyResult.promptVersion === proof.evaluationInput.modelRequest.promptVersion
  );
}

function selectedRowsCoverBindings(proof: LocalAiPromptTemplateVersionProofCandidate): boolean {
  const bindings = new Set(proof.selectedTemplateRows.map((row) => row.inputBinding));
  return (
    proof.selectedTemplateRows.length >= 3 &&
    bindings.has('context-builder') &&
    bindings.has('evaluation-input') &&
    bindings.has('safety-result') &&
    proof.summary.inputBindingCount === 3 &&
    proof.summary.promptVersionMatchCount === proof.selectedTemplateRows.length
  );
}

function selectedRowsAreNonRetaining(proof: LocalAiPromptTemplateVersionProofCandidate): boolean {
  return proof.summary.nonRetainingTemplateRowCount === proof.selectedTemplateRows.length;
}

function selectedRowsMatchProviderAndModel(proof: LocalAiPromptTemplateVersionProofCandidate): boolean {
  return proof.selectedTemplateRows.every(
    (row) =>
      row.providerId === proof.evaluationInput.modelRequest.providerId &&
      row.modelId === proof.evaluationInput.modelRequest.modelId
  );
}

export function buildLocalAiPromptTemplateVersionProof(input: unknown): LocalAiPromptTemplateVersionProof {
  const parsed = LocalAiPromptTemplateVersionProofInputSchema.parse(input);
  const rows = selectedTemplateRows(parsed);
  return LocalAiPromptTemplateVersionProofSchema.parse({
    schemaVersion: parsed.schemaVersion,
    proofKind: 'local-ai-prompt-template-version-proof',
    contextRequest: parsed.contextRequest,
    evaluationInput: parsed.evaluationInput,
    safetyResult: parsed.safetyResult,
    selectedTemplateRows: rows,
    summary: summaryFor(parsed, rows),
    claimBoundaries: parsed.claimBoundaries,
  });
}
