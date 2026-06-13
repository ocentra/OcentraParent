import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import {
  ActivityJournalLineSchema,
  ActivityJournalSchemaVersion,
  ActivityJournalCipher,
  type ActivityJournalLine,
} from './journal';
import {
  ActivityReadModelStateSchema,
  ActivityScreenReadModelSchema,
  ActivitySurfaceSchemaVersion,
  type ActivityScreenReadModel,
  type ActivitySurfaceRequest,
} from './activity-surface';
import {
  ScreenVlmExecutionReadinessNonClaimsSchema,
  ScreenVlmExecutionStatusRowSchema,
  type ScreenVlmExecutionStatusRow,
} from '@ocentra-parent/screen-domain/screen-vlm-execution-readiness';

export const ScreenVlmJournalReadModelSchemaVersion = 1;
export const ScreenVlmJournalReadModelProofTier = 'P3_CONTRACT_VLM_JOURNAL_READ_MODEL';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const NonNegativeDurationSchema = withParser(Schema.Number.pipe(Schema.nonNegative(), Schema.int()));

const ScreenVlmJournalProjectionStateSchema = withParser(ActivityReadModelStateSchema);

export const ScreenVlmJournalReadModelProjectionSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenVlmJournalReadModelSchemaVersion),
    projectionId: NonEmptyStringSchema,
    statusRow: ScreenVlmExecutionStatusRowSchema,
    journalLine: ActivityJournalLineSchema,
    readModel: ActivityScreenReadModelSchema,
    nonClaims: ScreenVlmExecutionReadinessNonClaimsSchema,
    localOnly: RequiredTrue,
    remoteAiUsed: RequiredFalse,
    rawImageRetained: RequiredFalse,
    portalRuntimeClaimed: RequiredFalse,
    enforcementClaimed: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) =>
        value.statusRow.status === 'completed' ||
        'Expected VLM journal/read-model projections to start from completed status rows'
    ),
    Schema.filter(
      (value) =>
        completedStatusHasDeletedCustody(value) ||
        'Expected completed VLM projection rows to require deleted query-store custody'
    ),
    Schema.filter(
      (value) =>
        journalLineMatchesStatus(value) ||
        'Expected encrypted journal line to cite the completed VLM status id and image digest'
    ),
    Schema.filter(
      (value) =>
        readModelContainsStatusProjection(value) ||
        'Expected Activity Screen read model to expose the completed VLM row through a journal evidence ref'
    )
  )
);

export const ScreenVlmJournalReadModelProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenVlmJournalReadModelSchemaVersion),
    proofId: NonEmptyStringSchema,
    proofTier: Schema.Literal(ScreenVlmJournalReadModelProofTier),
    projections: Schema.Array(ScreenVlmJournalReadModelProjectionSchema),
    localOnly: RequiredTrue,
    remoteAiUsed: RequiredFalse,
    rawImageRetained: RequiredFalse,
    portalRuntimeClaimed: RequiredFalse,
    enforcementClaimed: RequiredFalse,
  }).pipe(
    Schema.filter(
      (value) => value.projections.length > 0 || 'Expected VLM journal/read-model proof to include projections'
    ),
    Schema.filter(
      (value) =>
        value.projections.every(
          (projection) =>
            projection.readModel.rows.every((row) => row.rawImageRetained === false) &&
            !projection.nonClaims.rawImageRetained &&
            !projection.nonClaims.remoteAiUsed
        ) || 'Expected all VLM journal/read-model projections to reject raw retention and remote AI'
    )
  )
);

export function screenVlmJournalLineFromCompletedStatus(input: {
  readonly statusRow: ScreenVlmExecutionStatusRow;
  readonly entryId: unknown;
  readonly segmentId: unknown;
  readonly writtenAt: unknown;
  readonly nonce: unknown;
  readonly ciphertext: unknown;
}) {
  if (input.statusRow.result === null) {
    throw new Error('Expected completed VLM status row before journal projection');
  }

  return ActivityJournalLineSchema.parse({
    schemaVersion: ActivityJournalSchemaVersion,
    entryId: input.entryId,
    segmentId: input.segmentId,
    writtenAt: input.writtenAt,
    eventId: input.statusRow.statusId,
    cipher: ActivityJournalCipher.XChaCha20Poly1305,
    nonce: input.nonce,
    ciphertext: input.ciphertext,
    activityDigest: input.statusRow.result.imageDigest,
  });
}

export function screenVlmReadModelFromCompletedStatus(input: {
  readonly statusRow: ScreenVlmExecutionStatusRow;
  readonly request: ActivitySurfaceRequest;
  readonly generatedAt: unknown;
  readonly rowId: unknown;
  readonly label: unknown;
  readonly deviceId: unknown;
  readonly journalEntryId: unknown;
  readonly state?: unknown;
  readonly totalMs?: unknown;
  readonly foregroundMs?: unknown;
  readonly backgroundMs?: unknown;
  readonly policyDecisionRef?: unknown;
  readonly policyAction?: unknown;
  readonly policyReasonCodes?: readonly unknown[];
  readonly parentRuleRefs?: readonly unknown[];
  readonly parentExplanationRefs?: readonly unknown[];
  readonly explanationReasons?: readonly unknown[];
  readonly deletionReasons?: readonly unknown[];
}) {
  if (input.statusRow.result === null) {
    throw new Error('Expected completed VLM status row before read-model projection');
  }

  const evidenceRef = ActivityEvidenceRefSchema.parse({
    evidenceId: input.journalEntryId,
    kind: ActivityEvidenceKind.JournalEntry,
    digest: input.statusRow.result.imageDigest,
    uri: null,
  });
  const state = input.state ?? 'ready';

  return ActivityScreenReadModelSchema.parse({
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: input.request,
    state,
    generatedAt: input.generatedAt,
    summary: input.statusRow.statusReason,
    rows: [screenVlmReadModelRowFromCompletedStatus(input, evidenceRef, state)],
  });
}

export function screenVlmJournalReadModelProjection(input: {
  readonly projectionId: unknown;
  readonly statusRow: ScreenVlmExecutionStatusRow;
  readonly journalLine: unknown;
  readonly readModel: ActivityScreenReadModel;
}) {
  return ScreenVlmJournalReadModelProjectionSchema.parse({
    schemaVersion: ScreenVlmJournalReadModelSchemaVersion,
    projectionId: input.projectionId,
    statusRow: input.statusRow,
    journalLine: input.journalLine,
    readModel: input.readModel,
    nonClaims: input.statusRow.nonClaims,
    localOnly: true,
    remoteAiUsed: false,
    rawImageRetained: false,
    portalRuntimeClaimed: false,
    enforcementClaimed: false,
  });
}

export type ScreenVlmJournalReadModelProjection = Infer<typeof ScreenVlmJournalReadModelProjectionSchema>;
export type ScreenVlmJournalReadModelProof = Infer<typeof ScreenVlmJournalReadModelProofSchema>;

function completedStatusHasDeletedCustody(input: { readonly statusRow: ScreenVlmExecutionStatusRow }) {
  const result = input.statusRow.result;
  return (
    result !== null &&
    result.imageDeletionState === 'deleted' &&
    input.statusRow.custodyState === 'child-device-query-store' &&
    result.custodyState === 'child-device-query-store' &&
    !result.rawImageRetained &&
    !result.remoteAiUsed
  );
}

function journalLineMatchesStatus(input: {
  readonly statusRow: ScreenVlmExecutionStatusRow;
  readonly journalLine: ActivityJournalLine;
}) {
  const result = input.statusRow.result;
  return (
    result !== null &&
    input.journalLine.schemaVersion === ActivityJournalSchemaVersion &&
    String(input.journalLine.eventId) === String(input.statusRow.statusId) &&
    String(input.journalLine.activityDigest) === String(result.imageDigest) &&
    input.journalLine.cipher === ActivityJournalCipher.XChaCha20Poly1305
  );
}

function readModelContainsStatusProjection(input: {
  readonly statusRow: ScreenVlmExecutionStatusRow;
  readonly journalLine: ActivityJournalLine;
  readonly readModel: ActivityScreenReadModel;
}) {
  return input.readModel.rows.some((row) => readModelRowMatchesStatus(row, input.statusRow, input.journalLine));
}

function readModelRowMatchesStatus(
  row: ActivityScreenReadModel['rows'][number],
  statusRow: ScreenVlmExecutionStatusRow,
  journalLine: ActivityJournalLine
) {
  const result = statusRow.result;
  return (
    result !== null &&
    readModelRowMatchesStatusRefs(row, statusRow) &&
    readModelRowMatchesResult(row, result) &&
    readModelRowCitesJournal(row, journalLine)
  );
}

function readModelRowMatchesStatusRefs(
  row: ActivityScreenReadModel['rows'][number],
  statusRow: ScreenVlmExecutionStatusRow
) {
  return (
    row.queueJobId === statusRow.queueJobId &&
    row.modelRuntimeRef === statusRow.modelRuntimeRef &&
    row.modelId === statusRow.modelId &&
    row.promptOrTemplateVersion === statusRow.promptOrTemplateVersion
  );
}

function readModelRowMatchesResult(
  row: ActivityScreenReadModel['rows'][number],
  result: NonNullable<ScreenVlmExecutionStatusRow['result']>
) {
  return (
    row.providerKind === result.providerKind &&
    row.primaryCategory === result.primaryCategory &&
    row.confidence === result.confidence &&
    row.imageDigest === result.imageDigest &&
    row.imageDeletionState === 'deleted' &&
    row.custodyState === 'child-device-query-store' &&
    row.rawImageRetained === false &&
    row.policyEligible === result.policyEligible
  );
}

function readModelRowCitesJournal(row: ActivityScreenReadModel['rows'][number], journalLine: ActivityJournalLine) {
  return row.evidence.some(
    (ref) => String(ref.evidenceId) === String(journalLine.entryId) && ref.kind === 'journal-entry'
  );
}

function screenVlmReadModelRowFromCompletedStatus(
  input: Parameters<typeof screenVlmReadModelFromCompletedStatus>[0],
  evidenceRef: Infer<typeof ActivityEvidenceRefSchema>,
  state: unknown
) {
  const result = input.statusRow.result;
  if (result === null) {
    throw new Error('Expected completed VLM result before read-model row projection');
  }

  return {
    rowId: input.rowId,
    label: input.label,
    deviceId: input.deviceId,
    state: ScreenVlmJournalProjectionStateSchema.parse(state),
    totalMs: NonNegativeDurationSchema.parse(input.totalMs ?? 0),
    foregroundMs: NonNegativeDurationSchema.parse(input.foregroundMs ?? 0),
    backgroundMs: NonNegativeDurationSchema.parse(input.backgroundMs ?? 0),
    captureReason: result.captureReason,
    captureScope: result.captureScope,
    capabilityStatus: result.capabilityStatus,
    queueJobId: input.statusRow.queueJobId,
    modelRuntimeRef: input.statusRow.modelRuntimeRef,
    modelId: input.statusRow.modelId,
    providerKind: result.providerKind,
    promptOrTemplateVersion: input.statusRow.promptOrTemplateVersion,
    primaryCategory: result.primaryCategory,
    confidence: result.confidence,
    imageDeletionState: result.imageDeletionState,
    rawImageRetained: false,
    policyEligible: result.policyEligible,
    imageDigest: result.imageDigest,
    custodyState: result.custodyState,
    evidence: [evidenceRef],
    policyDecisionRef: nullableRef(input.policyDecisionRef),
    policyAction: nullableRef(input.policyAction),
    policyReasonCodes: optionalList(input.policyReasonCodes),
    parentRuleRefs: optionalList(input.parentRuleRefs),
    localModelRuntimeRefs: [input.statusRow.modelRuntimeRef],
    parentExplanationRefs: optionalList(input.parentExplanationRefs),
    explanationReasons: optionalList(input.explanationReasons),
    deletionReasons: optionalList(input.deletionReasons, ['raw-image-deleted-before-read-model']),
  } as const;
}

function nullableRef(value: unknown) {
  return value ?? null;
}

function optionalList(values: readonly unknown[] | undefined, fallback: readonly unknown[] = []) {
  return values ?? fallback;
}

