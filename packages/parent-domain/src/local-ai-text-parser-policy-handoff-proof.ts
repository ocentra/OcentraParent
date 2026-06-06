import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiTextParserManualReasonSchema,
  LocalAiTextParserReadModelRowIdSchema,
  LocalAiTextParserReadModelRowSchema,
  type LocalAiTextParserReadModelRow,
} from './local-ai-text-parser-read-model-proof';
import { LocalAiResultNonClaimSchema, LocalAiResultProofRefSchema } from './local-ai-result-journal-sqlite-proof';
import { LocalAiTimestampSchema } from './local-ai-primitives';
import {
  PolicyActionSchema,
  PolicyDecisionHandoffStateSchema,
  PolicyDecisionSchema,
  type PolicyDecision,
} from './policy';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from './reference-primitives';

const NonEmptyTextParserPolicyText = Schema.String.pipe(Schema.minLength(1));
const TextParserPolicyCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiTextParserPolicyHandoffIdSchema = NonEmptyTextParserPolicyText.pipe(
  Schema.brand('LocalAiTextParserPolicyHandoffId')
);
export const LocalAiTextParserPolicyHandoffRowIdSchema = NonEmptyTextParserPolicyText.pipe(
  Schema.brand('LocalAiTextParserPolicyHandoffRowId')
);

export const LocalAiTextParserPolicyHandoffStateSchema = withParser(
  Schema.Literal('policy-dry-run-ready', 'manual-required')
);

const LocalAiTextParserPolicyHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  policyHandoffRowId: LocalAiTextParserPolicyHandoffRowIdSchema,
  sourceReadModelRowId: LocalAiTextParserReadModelRowIdSchema,
  parserRunId: NonEmptyTextParserPolicyText,
  handoffState: LocalAiTextParserPolicyHandoffStateSchema,
  action: PolicyActionSchema,
  policyDecision: Schema.Union(PolicyDecisionSchema, Schema.Null),
  policyDecisionHandoffState: PolicyDecisionHandoffStateSchema,
  manualRequiredReasons: Schema.Array(LocalAiTextParserManualReasonSchema),
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
  resultPolicyEligible: Schema.Boolean,
  dryRunOnly: Schema.Boolean,
  reportOnly: Schema.Boolean,
  modelExecuted: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  productionModelQualityClaimed: Schema.Boolean,
});

type LocalAiTextParserPolicyHandoffRowCandidate = Infer<typeof LocalAiTextParserPolicyHandoffRowBaseSchema>;

export const LocalAiTextParserPolicyHandoffRowSchema = withParser(
  LocalAiTextParserPolicyHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        localAiTextParserPolicyHandoffRowIsHonest(row) ||
        'Expected local text parser policy handoff rows to keep dry-run decisions separate from enforcement'
    )
  )
);

const LocalAiTextParserPolicyHandoffProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: LocalAiTextParserPolicyHandoffIdSchema,
  generatedAt: LocalAiTimestampSchema,
  rows: Schema.Array(LocalAiTextParserPolicyHandoffRowSchema),
  policyReadyRowCount: TextParserPolicyCountSchema,
  manualRequiredRowCount: TextParserPolicyCountSchema,
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
  nonClaims: Schema.Array(LocalAiResultNonClaimSchema),
});

type LocalAiTextParserPolicyHandoffProofCandidate = Infer<typeof LocalAiTextParserPolicyHandoffProofBaseSchema>;

export const LocalAiTextParserPolicyHandoffProofSchema = withParser(
  LocalAiTextParserPolicyHandoffProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        localAiTextParserPolicyHandoffProofIsComplete(proof) ||
        'Expected local text parser policy handoff proof counts and source refs to match rows'
    )
  )
);

export type LocalAiTextParserPolicyHandoffRow = Infer<typeof LocalAiTextParserPolicyHandoffRowSchema>;
export type LocalAiTextParserPolicyHandoffProof = Infer<typeof LocalAiTextParserPolicyHandoffProofSchema>;

const decodeProofRef = Schema.decodeUnknownSync(LocalAiResultProofRefSchema);
const decodeNonClaim = Schema.decodeUnknownSync(LocalAiResultNonClaimSchema);

export const LocalAiTextParserPolicyHandoffNonClaims = [
  decodeNonClaim('This proof feeds ready local text parser rows into dry-run policy decisions.'),
  decodeNonClaim('Rejected and manual parser rows remain manual-required and cannot create policy decisions.'),
  decodeNonClaim(
    'This proof does not execute a model, prove model quality, retain raw model output, render portal UI, dispatch enforcement, or use remote/API AI.'
  ),
] as const;

export function buildLocalAiTextParserPolicyHandoffProof(input: {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceProofRefs: readonly string[];
  readonly readModelRows: readonly unknown[];
}): LocalAiTextParserPolicyHandoffProof {
  const sourceProofRefs = input.sourceProofRefs.map((proofRef) => decodeProofRef(proofRef));
  const rows = input.readModelRows.map((row, index) =>
    policyHandoffRowFromReadModelRow(LocalAiTextParserReadModelRowSchema.parse(row), index, sourceProofRefs)
  );

  return LocalAiTextParserPolicyHandoffProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: input.proofId,
    generatedAt: input.generatedAt,
    rows,
    policyReadyRowCount: rows.filter((row) => row.handoffState === 'policy-dry-run-ready').length,
    manualRequiredRowCount: rows.filter((row) => row.handoffState === 'manual-required').length,
    sourceProofRefs,
    nonClaims: LocalAiTextParserPolicyHandoffNonClaims,
  });
}

function policyHandoffRowFromReadModelRow(
  row: LocalAiTextParserReadModelRow,
  index: number,
  sourceProofRefs: readonly ReturnType<typeof decodeProofRef>[]
): LocalAiTextParserPolicyHandoffRow {
  const isReady = localAiTextParserRowCanBecomePolicyDecision(row);

  return LocalAiTextParserPolicyHandoffRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    policyHandoffRowId: `local-ai-text-parser-policy-handoff:${index}:${row.readModelRowId}`,
    sourceReadModelRowId: row.readModelRowId,
    parserRunId: row.parserRunId,
    handoffState: isReady ? 'policy-dry-run-ready' : 'manual-required',
    action: row.action ?? 'ask-parent',
    policyDecision: isReady ? policyDecisionFor(row, index) : null,
    policyDecisionHandoffState: isReady ? 'disabled' : 'not-requested',
    manualRequiredReasons: isReady ? [] : row.manualRequiredReasons,
    sourceProofRefs,
    resultPolicyEligible: isReady,
    dryRunOnly: true,
    reportOnly: true,
    modelExecuted: row.modelExecuted,
    rawModelOutputRetained: row.rawModelOutputRetained,
    remoteApiClaimed: row.remoteApiClaimed,
    policyAuthorityClaimed: row.policyAuthorityClaimed,
    enforcementClaimed: row.enforcementClaimed,
    productionModelQualityClaimed: row.productionModelQualityClaimed,
  });
}

function localAiTextParserRowCanBecomePolicyDecision(row: LocalAiTextParserReadModelRow): boolean {
  return (
    row.readModelState === 'ready' &&
    row.resultPolicyEligible &&
    row.sourceResultId !== null &&
    row.action !== null &&
    row.evidenceReferences.length > 0 &&
    row.parentRuleReferences.length > 0
  );
}

function policyDecisionFor(row: LocalAiTextParserReadModelRow, index: number): PolicyDecision {
  return PolicyDecisionSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    decisionId: `local-ai-text-parser-policy:${index}:${row.sourceResultId}`,
    action: row.action,
    reasonCodes: row.reasonCodes,
    evidenceReferences: row.evidenceReferences,
    ruleIds: row.parentRuleReferences,
    localAiResultId: row.sourceResultId,
    dryRun: true,
    enforcementHandoffState: 'disabled',
    expiresAt: null,
  });
}

function localAiTextParserPolicyHandoffRowIsHonest(row: LocalAiTextParserPolicyHandoffRowCandidate): boolean {
  return (
    row.dryRunOnly &&
    row.reportOnly &&
    row.sourceProofRefs.length > 0 &&
    localAiTextParserPolicyHandoffStateIsConsistent(row) &&
    localAiTextParserPolicyHandoffRowHasNoOverclaims(row)
  );
}

function localAiTextParserPolicyHandoffStateIsConsistent(row: LocalAiTextParserPolicyHandoffRowCandidate): boolean {
  if (row.handoffState === 'policy-dry-run-ready') {
    return localAiTextParserPolicyReadyRowIsConsistent(row);
  }

  return (
    row.policyDecision === null &&
    !row.resultPolicyEligible &&
    row.policyDecisionHandoffState === 'not-requested' &&
    row.manualRequiredReasons.length > 0
  );
}

function localAiTextParserPolicyReadyRowIsConsistent(row: LocalAiTextParserPolicyHandoffRowCandidate): boolean {
  return (
    row.policyDecision !== null &&
    row.resultPolicyEligible &&
    row.manualRequiredReasons.length === 0 &&
    row.policyDecision.dryRun &&
    row.policyDecision.enforcementHandoffState === 'disabled' &&
    row.policyDecision.localAiResultId !== null
  );
}

function localAiTextParserPolicyHandoffRowHasNoOverclaims(row: LocalAiTextParserPolicyHandoffRowCandidate): boolean {
  return (
    !row.modelExecuted &&
    !row.rawModelOutputRetained &&
    !row.remoteApiClaimed &&
    !row.policyAuthorityClaimed &&
    !row.enforcementClaimed &&
    !row.productionModelQualityClaimed
  );
}

function localAiTextParserPolicyHandoffProofIsComplete(proof: LocalAiTextParserPolicyHandoffProofCandidate): boolean {
  return (
    proof.rows.length > 0 &&
    proof.sourceProofRefs.length > 0 &&
    proof.rows.every((row) => proof.sourceProofRefs.every((proofRef) => row.sourceProofRefs.includes(proofRef))) &&
    proof.policyReadyRowCount === proof.rows.filter((row) => row.handoffState === 'policy-dry-run-ready').length &&
    proof.manualRequiredRowCount === proof.rows.filter((row) => row.handoffState === 'manual-required').length &&
    LocalAiTextParserPolicyHandoffNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim))
  );
}
