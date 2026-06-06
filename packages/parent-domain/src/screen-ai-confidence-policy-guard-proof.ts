import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { LocalAiSafetyResultSchema } from './local-ai';
import { LocalAiConfidenceSchema, LocalAiTimestampSchema, LocalAiUnknownState } from './local-ai-primitives';
import { PolicyAction, PolicyDecisionSchema } from './policy';
import { ParentContractSchemaVersionSchema } from './reference-primitives';

const NonEmptyConfidenceGuardTextSchema = Schema.String.pipe(Schema.minLength(1));
const ScreenAiConfidencePolicyGuardProofIdSchema = NonEmptyConfidenceGuardTextSchema.pipe(
  Schema.brand('ScreenAiConfidencePolicyGuardProofId')
);
const ScreenAiConfidencePolicyGuardRowIdSchema = NonEmptyConfidenceGuardTextSchema.pipe(
  Schema.brand('ScreenAiConfidencePolicyGuardRowId')
);
const ScreenAiConfidencePolicyGuardEvidenceRefSchema = NonEmptyConfidenceGuardTextSchema.pipe(
  Schema.brand('ScreenAiConfidencePolicyGuardEvidenceRef')
);

export const ScreenAiConfidenceBandSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const ScreenAiConfidencePolicyGuardOutcomeSchema = withParser(
  Schema.Literal('policy-ready', 'parent-review-required', 'manual-required')
);

const ScreenAiConfidencePolicyGuardRowBaseSchema = Schema.Struct({
  rowId: ScreenAiConfidencePolicyGuardRowIdSchema,
  sourceEvidenceRef: ScreenAiConfidencePolicyGuardEvidenceRefSchema,
  localAiResult: LocalAiSafetyResultSchema,
  policyDecision: PolicyDecisionSchema,
  confidenceThreshold: LocalAiConfidenceSchema,
  confidenceBand: ScreenAiConfidenceBandSchema,
  guardOutcome: ScreenAiConfidencePolicyGuardOutcomeSchema,
  policyEligible: Schema.Boolean,
  enforcementAllowed: Schema.Boolean,
  remoteProviderUsed: Schema.Literal(false),
  rawImageRetained: Schema.Literal(false),
});

type ScreenAiConfidencePolicyGuardRowCandidate = Infer<typeof ScreenAiConfidencePolicyGuardRowBaseSchema>;

export const ScreenAiConfidencePolicyGuardRowSchema = withParser(
  ScreenAiConfidencePolicyGuardRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenAiConfidencePolicyGuardRowIsSafe(row) ||
        'Expected screen AI confidence guard row to keep low/unknown confidence out of allow/block enforcement, preserve local-only custody, and require policy decisions to cite the same local AI result'
    )
  )
);

const ScreenAiConfidencePolicyGuardProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: ScreenAiConfidencePolicyGuardProofIdSchema,
  generatedAt: LocalAiTimestampSchema,
  rows: Schema.Array(ScreenAiConfidencePolicyGuardRowSchema),
});

type ScreenAiConfidencePolicyGuardProofCandidate = Infer<typeof ScreenAiConfidencePolicyGuardProofBaseSchema>;

export const ScreenAiConfidencePolicyGuardProofSchema = withParser(
  ScreenAiConfidencePolicyGuardProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        screenAiConfidencePolicyGuardProofIsComplete(proof) ||
        'Expected screen AI confidence guard proof to include high, medium, low, and unknown confidence policy outcomes'
    )
  )
);

export const ScreenAiConfidencePolicyGuardSummarySchema = withParser(
  Schema.Struct({
    totalRows: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
    policyReadyRows: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
    parentReviewRows: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
    manualRequiredRows: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
    lowConfidenceRows: Schema.Number.pipe(Schema.nonNegative(), Schema.int()),
    unsafeAllowOrBlockRows: Schema.Literal(0),
    enforcementAllowedRows: Schema.Literal(0),
    remoteProviderRows: Schema.Literal(0),
    rawRetainedRows: Schema.Literal(0),
  })
);

export type ScreenAiConfidenceBand = Infer<typeof ScreenAiConfidenceBandSchema>;
export type ScreenAiConfidencePolicyGuardOutcome = Infer<typeof ScreenAiConfidencePolicyGuardOutcomeSchema>;
export type ScreenAiConfidencePolicyGuardRow = Infer<typeof ScreenAiConfidencePolicyGuardRowSchema>;
export type ScreenAiConfidencePolicyGuardProof = Infer<typeof ScreenAiConfidencePolicyGuardProofSchema>;
export type ScreenAiConfidencePolicyGuardSummary = Infer<typeof ScreenAiConfidencePolicyGuardSummarySchema>;

export function buildScreenAiConfidencePolicyGuardProof(input: unknown): ScreenAiConfidencePolicyGuardProof {
  return ScreenAiConfidencePolicyGuardProofSchema.parse(input);
}

export function screenAiConfidencePolicyGuardSummary(
  proof: ScreenAiConfidencePolicyGuardProof
): ScreenAiConfidencePolicyGuardSummary {
  return ScreenAiConfidencePolicyGuardSummarySchema.parse({
    totalRows: proof.rows.length,
    policyReadyRows: proof.rows.filter((row) => row.guardOutcome === 'policy-ready').length,
    parentReviewRows: proof.rows.filter((row) => row.guardOutcome === 'parent-review-required').length,
    manualRequiredRows: proof.rows.filter((row) => row.guardOutcome === 'manual-required').length,
    lowConfidenceRows: proof.rows.filter((row) => row.confidenceBand === 'low').length,
    unsafeAllowOrBlockRows: proof.rows.filter((row) => lowConfidenceRowAllowsOrBlocks(row)).length,
    enforcementAllowedRows: proof.rows.filter((row) => row.enforcementAllowed).length,
    remoteProviderRows: proof.rows.filter((row) => row.remoteProviderUsed).length,
    rawRetainedRows: proof.rows.filter((row) => row.rawImageRetained).length,
  });
}

function screenAiConfidencePolicyGuardRowIsSafe(row: ScreenAiConfidencePolicyGuardRowCandidate): boolean {
  return (
    localAiResultReferenceMatches(row) &&
    row.policyDecision.dryRun &&
    row.localAiResult.modelRuntime.privacyMode === 'local-only' &&
    row.remoteProviderUsed === false &&
    row.rawImageRetained === false &&
    confidenceBandMatchesValue(row) &&
    guardOutcomeMatchesConfidence(row)
  );
}

function localAiResultReferenceMatches(row: ScreenAiConfidencePolicyGuardRowCandidate): boolean {
  return (
    row.policyDecision.localAiResultId !== null &&
    String(row.policyDecision.localAiResultId) === String(row.localAiResult.resultId)
  );
}

function confidenceBandMatchesValue(row: ScreenAiConfidencePolicyGuardRowCandidate): boolean {
  if (row.localAiResult.unknownState === LocalAiUnknownState.LowConfidence) {
    return row.confidenceBand === 'low' && row.localAiResult.confidence < row.confidenceThreshold;
  }

  if (row.confidenceBand === 'high') {
    return row.localAiResult.confidence >= row.confidenceThreshold;
  }

  if (row.confidenceBand === 'medium') {
    return row.localAiResult.confidence >= 0.5 && row.localAiResult.confidence < row.confidenceThreshold;
  }

  return row.confidenceBand === 'unknown' && row.localAiResult.confidence === 0;
}

function guardOutcomeMatchesConfidence(row: ScreenAiConfidencePolicyGuardRowCandidate): boolean {
  if (row.confidenceBand === 'high') {
    return row.guardOutcome === 'policy-ready' && row.policyEligible && row.enforcementAllowed === false;
  }

  if (row.confidenceBand === 'medium') {
    return row.guardOutcome === 'parent-review-required' && parentReviewDecisionIsSafe(row);
  }

  return uncertainDecisionIsSafe(row);
}

function parentReviewDecisionIsSafe(row: ScreenAiConfidencePolicyGuardRowCandidate): boolean {
  return (
    row.policyDecision.action === PolicyAction.AskParent &&
    row.localAiResult.action === PolicyAction.AskParent &&
    row.policyEligible &&
    row.enforcementAllowed === false &&
    row.policyDecision.enforcementHandoffState === 'disabled'
  );
}

function uncertainDecisionIsSafe(row: ScreenAiConfidencePolicyGuardRowCandidate): boolean {
  return (
    !lowConfidenceRowAllowsOrBlocks(row) &&
    row.guardOutcome === 'manual-required' &&
    row.policyEligible === false &&
    row.enforcementAllowed === false &&
    row.policyDecision.enforcementHandoffState === 'not-requested'
  );
}

function lowConfidenceRowAllowsOrBlocks(row: ScreenAiConfidencePolicyGuardRowCandidate): boolean {
  if (row.confidenceBand !== 'low' && row.confidenceBand !== 'unknown') {
    return false;
  }

  return (
    row.localAiResult.action === PolicyAction.Allow ||
    row.localAiResult.action === PolicyAction.Block ||
    row.policyDecision.action === PolicyAction.Allow ||
    row.policyDecision.action === PolicyAction.Block
  );
}

function screenAiConfidencePolicyGuardProofIsComplete(proof: ScreenAiConfidencePolicyGuardProofCandidate): boolean {
  const bands = new Set(proof.rows.map((row) => row.confidenceBand));
  const unsafeAllowOrBlockRows = proof.rows.filter((row) => lowConfidenceRowAllowsOrBlocks(row)).length;

  return (
    bands.has('high') && bands.has('medium') && bands.has('low') && bands.has('unknown') && unsafeAllowOrBlockRows === 0
  );
}
