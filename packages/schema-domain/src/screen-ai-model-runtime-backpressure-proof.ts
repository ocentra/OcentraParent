import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  LocalAiDegradedStateSchema,
  LocalAiModelIdSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
  LocalAiUnavailableReasonSchema,
} from './ai-primitives';
import { LocalAiPhysicalDeviceIdSchema, LocalAiProviderSchedulerDecisionSchema } from './local-ai-provider-scheduler';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
const ScreenAiModelRuntimeBackpressureProofIdSchema = brandedNonEmptyStringSchema(
  'ScreenAiModelRuntimeBackpressureProofId'
);
const ScreenAiModelRuntimeBackpressureJobIdSchema = brandedNonEmptyStringSchema(
  'ScreenAiModelRuntimeBackpressureJobId'
);
const ScreenAiModelRuntimeEvidenceRefSchema = brandedNonEmptyStringSchema('ScreenAiModelRuntimeEvidenceRef');
const BackpressureCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const ScreenAiModelRuntimeBackpressureJobStateSchema = withParser(
  Schema.Literal('running', 'queued', 'overflow-degraded', 'unavailable')
);

export const ScreenAiModelRuntimeBackpressurePrioritySchema = withParser(
  Schema.Literal('policy-blocking', 'foreground', 'cadence', 'background-summary')
);

export const ScreenAiModelRuntimeBackpressureActionSchema = withParser(
  Schema.Literal('run-now', 'enqueue', 'reject-overload', 'reject-unavailable')
);

const ScreenAiModelRuntimeBackpressureRowBaseSchema = Schema.Struct({
  jobId: ScreenAiModelRuntimeBackpressureJobIdSchema,
  physicalDeviceId: LocalAiPhysicalDeviceIdSchema,
  sourceEncryptedQueueRef: ScreenAiModelRuntimeEvidenceRefSchema,
  captureDigestRef: ScreenAiModelRuntimeEvidenceRefSchema,
  priority: ScreenAiModelRuntimeBackpressurePrioritySchema,
  requestedAt: LocalAiTimestampSchema,
  modelId: LocalAiModelIdSchema,
  runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
  providerDecision: LocalAiProviderSchedulerDecisionSchema,
  jobState: ScreenAiModelRuntimeBackpressureJobStateSchema,
  queuePosition: Schema.Union(BackpressureCountSchema, Schema.Null),
  maxQueueDepth: BackpressureCountSchema,
  activeHeavyRuntimeCount: BackpressureCountSchema,
  queuedHeavyJobCount: BackpressureCountSchema,
  backpressureAction: ScreenAiModelRuntimeBackpressureActionSchema,
  degradedState: LocalAiDegradedStateSchema,
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  policyEligible: Schema.Literal(false),
  remoteProviderUsed: Schema.Literal(false),
  rawImageRetained: Schema.Literal(false),
});

type ScreenAiModelRuntimeBackpressureRowCandidate = Infer<typeof ScreenAiModelRuntimeBackpressureRowBaseSchema>;

export const ScreenAiModelRuntimeBackpressureRowSchema = withParser(
  ScreenAiModelRuntimeBackpressureRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenAiModelRuntimeBackpressureRowIsSafe(row) ||
        'Expected screen AI model runtime backpressure row to enforce one active heavy runtime, bounded queue depth, overload degradation, no policy eligibility, no remote provider, and no raw image retention'
    )
  )
);

const ScreenAiModelRuntimeBackpressureProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: ScreenAiModelRuntimeBackpressureProofIdSchema,
  generatedAt: ParentTimestampSchema,
  maxQueueDepth: BackpressureCountSchema,
  rows: Schema.Array(ScreenAiModelRuntimeBackpressureRowSchema),
});

type ScreenAiModelRuntimeBackpressureProofCandidate = Infer<typeof ScreenAiModelRuntimeBackpressureProofBaseSchema>;

export const ScreenAiModelRuntimeBackpressureProofSchema = withParser(
  ScreenAiModelRuntimeBackpressureProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        screenAiModelRuntimeBackpressureProofIsSafe(proof) ||
        'Expected screen AI model runtime backpressure proof to include running, queued, and overloaded rows for one bounded local model lane'
    )
  )
);

export const ScreenAiModelRuntimeBackpressureSummarySchema = withParser(
  Schema.Struct({
    totalJobs: BackpressureCountSchema,
    maxQueueDepth: BackpressureCountSchema,
    activeHeavyRuntimeCount: BackpressureCountSchema,
    queuedHeavyJobCount: BackpressureCountSchema,
    overflowRejectedCount: BackpressureCountSchema,
    policyEligibleRows: Schema.Literal(0),
    remoteProviderRows: Schema.Literal(0),
    rawRetainedRows: Schema.Literal(0),
    singleActiveHeavyRuntime: Schema.Literal(true),
    boundedQueueDepth: Schema.Literal(true),
    overflowRowsPolicyIneligible: Schema.Literal(true),
  })
);

export type ScreenAiModelRuntimeBackpressureRow = Infer<typeof ScreenAiModelRuntimeBackpressureRowSchema>;
export type ScreenAiModelRuntimeBackpressureProof = Infer<typeof ScreenAiModelRuntimeBackpressureProofSchema>;
export type ScreenAiModelRuntimeBackpressureSummary = Infer<typeof ScreenAiModelRuntimeBackpressureSummarySchema>;

export function buildScreenAiModelRuntimeBackpressureProof(input: unknown): ScreenAiModelRuntimeBackpressureProof {
  return ScreenAiModelRuntimeBackpressureProofSchema.parse(input);
}

export function screenAiModelRuntimeBackpressureSummary(
  proof: ScreenAiModelRuntimeBackpressureProof
): ScreenAiModelRuntimeBackpressureSummary {
  const activeHeavyRuntimeCount = Math.max(...proof.rows.map((row) => row.activeHeavyRuntimeCount), 0);
  const queuedHeavyJobCount = Math.max(...proof.rows.map((row) => row.queuedHeavyJobCount), 0);
  const overflowRejectedCount = proof.rows.filter((row) => row.backpressureAction === 'reject-overload').length;

  return ScreenAiModelRuntimeBackpressureSummarySchema.parse({
    totalJobs: proof.rows.length,
    maxQueueDepth: proof.maxQueueDepth,
    activeHeavyRuntimeCount,
    queuedHeavyJobCount,
    overflowRejectedCount,
    policyEligibleRows: proof.rows.filter((row) => row.policyEligible).length,
    remoteProviderRows: proof.rows.filter((row) => row.remoteProviderUsed).length,
    rawRetainedRows: proof.rows.filter((row) => row.rawImageRetained).length,
    singleActiveHeavyRuntime: activeHeavyRuntimeCount === 1,
    boundedQueueDepth: queuedHeavyJobCount <= proof.maxQueueDepth,
    overflowRowsPolicyIneligible: proof.rows
      .filter((row) => row.jobState === 'overflow-degraded')
      .every((row) => row.policyEligible === false),
  });
}

function screenAiModelRuntimeBackpressureRowIsSafe(row: ScreenAiModelRuntimeBackpressureRowCandidate): boolean {
  return (
    row.activeHeavyRuntimeCount <= 1 &&
    row.queuedHeavyJobCount <= row.maxQueueDepth &&
    row.providerDecision.physicalDeviceId === row.physicalDeviceId &&
    row.providerDecision.selectedRuntimeReferenceId === row.runtimeReferenceId &&
    row.providerDecision.queuePosition === row.queuePosition &&
    row.providerDecision.duplicateRuntimeBlocked &&
    row.remoteProviderUsed === false &&
    row.rawImageRetained === false &&
    stateMatchesBackpressureAction(row)
  );
}

function stateMatchesBackpressureAction(row: ScreenAiModelRuntimeBackpressureRowCandidate): boolean {
  switch (row.jobState) {
    case 'running':
      return runningRowMatchesBackpressureAction(row);
    case 'queued':
      return queuedRowMatchesBackpressureAction(row);
    case 'overflow-degraded':
      return overflowRowMatchesBackpressureAction(row);
    case 'unavailable':
      return unavailableRowMatchesBackpressureAction(row);
  }

  return false;
}

function runningRowMatchesBackpressureAction(row: ScreenAiModelRuntimeBackpressureRowCandidate): boolean {
  return row.backpressureAction === 'run-now' && row.queuePosition === null && row.degradedState === 'none';
}

function queuedRowMatchesBackpressureAction(row: ScreenAiModelRuntimeBackpressureRowCandidate): boolean {
  return row.backpressureAction === 'enqueue' && row.queuePosition !== null && row.degradedState === 'none';
}

function overflowRowMatchesBackpressureAction(row: ScreenAiModelRuntimeBackpressureRowCandidate): boolean {
  return (
    row.backpressureAction === 'reject-overload' &&
    row.queuePosition === null &&
    row.degradedState === 'overloaded' &&
    row.unavailableReason === null
  );
}

function unavailableRowMatchesBackpressureAction(row: ScreenAiModelRuntimeBackpressureRowCandidate): boolean {
  return (
    row.backpressureAction === 'reject-unavailable' &&
    row.queuePosition === null &&
    row.degradedState === 'provider-unavailable' &&
    row.unavailableReason !== null
  );
}

function screenAiModelRuntimeBackpressureProofIsSafe(proof: ScreenAiModelRuntimeBackpressureProofCandidate): boolean {
  const activeHeavyRuntimeCount = Math.max(...proof.rows.map((row) => row.activeHeavyRuntimeCount), 0);

  return (
    proof.maxQueueDepth > 0 &&
    proof.rows.length >= 3 &&
    proof.rows.every((row) => row.maxQueueDepth === proof.maxQueueDepth) &&
    proof.rows.some((row) => row.jobState === 'running') &&
    proof.rows.some((row) => row.jobState === 'queued') &&
    proof.rows.some((row) => row.jobState === 'overflow-degraded') &&
    activeHeavyRuntimeCount === 1
  );
}
