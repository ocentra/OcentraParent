import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { PolicyActionSchema, PolicyReasonCodeSchema, PolicyRuleIdSchema } from './policy-contracts';
import { ParentEvidenceReferenceSchema } from './family-references';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from './family-reference-primitives';
import { LocalAiSafetyResultSchema, type LocalAiSafetyResult } from './local-ai';
import {
  LocalAiEvaluationRequestIdSchema,
  LocalAiModelIdSchema,
  LocalAiPromptVersionSchema,
  LocalAiProviderIdSchema,
  LocalAiResultIdSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
} from './ai-primitives';
const LocalAiJournalCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiResultJournalEntryIdSchema = brandedNonEmptyStringSchema('LocalAiResultJournalEntryId');
export const LocalAiResultSqliteRowIdSchema = brandedNonEmptyStringSchema('LocalAiResultSqliteRowId');
export const LocalAiResultReadModelIdSchema = brandedNonEmptyStringSchema('LocalAiResultReadModelId');
export const LocalAiResultProofRefSchema = brandedNonEmptyStringSchema('LocalAiResultProofRef');
export const LocalAiResultNonClaimSchema = brandedNonEmptyStringSchema('LocalAiResultNonClaim');

export const LocalAiResultJournalStateSchema = withParser(
  Schema.Literal('journaled', 'manual-required', 'unavailable')
);
export const LocalAiResultSqliteIngestStateSchema = withParser(
  Schema.Literal('ingested', 'manual-required', 'unavailable')
);

const LocalAiResultJournalEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  journalEntryId: LocalAiResultJournalEntryIdSchema,
  sourceResultId: LocalAiResultIdSchema,
  requestId: LocalAiEvaluationRequestIdSchema,
  action: PolicyActionSchema,
  confidence: Schema.Number.pipe(Schema.between(0, 1)),
  reasonCodes: Schema.Array(PolicyReasonCodeSchema),
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  parentRuleReferences: Schema.Array(PolicyRuleIdSchema),
  runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
  providerId: LocalAiProviderIdSchema,
  modelId: LocalAiModelIdSchema,
  promptVersion: LocalAiPromptVersionSchema,
  journalState: LocalAiResultJournalStateSchema,
  journaledAt: LocalAiTimestampSchema,
  rawPromptRetained: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
});

type LocalAiResultJournalEntryCandidate = Infer<typeof LocalAiResultJournalEntryBaseSchema>;

export const LocalAiResultJournalEntrySchema = withParser(
  LocalAiResultJournalEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        localAiResultJournalEntryIsHonest(entry) ||
        'Expected local AI result journal entries to preserve typed refs without raw retention or authority claims'
    )
  )
);

const LocalAiResultSqliteRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  sqliteRowId: LocalAiResultSqliteRowIdSchema,
  journalEntryId: LocalAiResultJournalEntryIdSchema,
  sourceResultId: LocalAiResultIdSchema,
  requestId: LocalAiEvaluationRequestIdSchema,
  action: PolicyActionSchema,
  confidence: Schema.Number.pipe(Schema.between(0, 1)),
  evidenceReferenceCount: LocalAiJournalCountSchema,
  parentRuleReferenceCount: LocalAiJournalCountSchema,
  runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
  providerId: LocalAiProviderIdSchema,
  modelId: LocalAiModelIdSchema,
  promptVersion: LocalAiPromptVersionSchema,
  ingestState: LocalAiResultSqliteIngestStateSchema,
  ingestedAt: LocalAiTimestampSchema,
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
  rawPromptRetained: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
  remoteApiClaimed: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type LocalAiResultSqliteRowCandidate = Infer<typeof LocalAiResultSqliteRowBaseSchema>;

export const LocalAiResultSqliteRowSchema = withParser(
  LocalAiResultSqliteRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        localAiResultSqliteRowIsHonest(row) ||
        'Expected local AI result SQLite rows to preserve journal refs without raw retention or authority claims'
    )
  )
);

const LocalAiResultReadModelSnapshotBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelId: LocalAiResultReadModelIdSchema,
  generatedAt: LocalAiTimestampSchema,
  journalEntries: Schema.Array(LocalAiResultJournalEntrySchema),
  sqliteRows: Schema.Array(LocalAiResultSqliteRowSchema),
  readyResultCount: LocalAiJournalCountSchema,
  degradedResultCount: LocalAiJournalCountSchema,
  unavailableResultCount: LocalAiJournalCountSchema,
  sourceProofRefs: Schema.Array(LocalAiResultProofRefSchema),
  nonClaims: Schema.Array(LocalAiResultNonClaimSchema),
});

type LocalAiResultReadModelSnapshotCandidate = Infer<typeof LocalAiResultReadModelSnapshotBaseSchema>;

export const LocalAiResultReadModelSnapshotSchema = withParser(
  LocalAiResultReadModelSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        localAiResultReadModelSnapshotIsComplete(snapshot) ||
        'Expected local AI result read model snapshot counts and journal/SQLite refs to match'
    )
  )
);

export type LocalAiResultJournalEntry = Infer<typeof LocalAiResultJournalEntrySchema>;
export type LocalAiResultSqliteRow = Infer<typeof LocalAiResultSqliteRowSchema>;
export type LocalAiResultReadModelSnapshot = Infer<typeof LocalAiResultReadModelSnapshotSchema>;

const decodeProofRef = Schema.decodeUnknownSync(LocalAiResultProofRefSchema);
const decodeNonClaim = Schema.decodeUnknownSync(LocalAiResultNonClaimSchema);

export const LocalAiResultJournalSqliteNonClaims = [
  decodeNonClaim(
    'This proof validates local AI result journal and SQLite ingest contracts without production storage.'
  ),
  decodeNonClaim(
    'This proof does not execute a model, prove model quality, use remote/API AI, or grant policy authority.'
  ),
  decodeNonClaim(
    'Raw prompts and raw model output are not retained; only typed refs and summarized result fields persist.'
  ),
] as const;

export function buildLocalAiResultReadModelSnapshot(input: {
  readonly generatedAt: string;
  readonly readModelId: string;
  readonly sourceProofRefs: readonly string[];
  readonly results: readonly unknown[];
}): LocalAiResultReadModelSnapshot {
  const sourceProofRefs = input.sourceProofRefs.map((proofRef) => decodeProofRef(proofRef));
  const journalEntries = input.results.map((result, index) =>
    journalEntryFromResult(LocalAiSafetyResultSchema.parse(result), index, input.generatedAt, sourceProofRefs)
  );
  const sqliteRows = journalEntries.map((entry, index) => sqliteRowFromJournalEntry(entry, index, input.generatedAt));

  return LocalAiResultReadModelSnapshotSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: input.readModelId,
    generatedAt: input.generatedAt,
    journalEntries,
    sqliteRows,
    readyResultCount: countSqliteRows(sqliteRows, 'ingested'),
    degradedResultCount: journalEntries.filter((entry) => entry.journalState === 'manual-required').length,
    unavailableResultCount: journalEntries.filter((entry) => entry.journalState === 'unavailable').length,
    sourceProofRefs,
    nonClaims: LocalAiResultJournalSqliteNonClaims,
  });
}

function journalEntryFromResult(
  result: LocalAiSafetyResult,
  index: number,
  journaledAt: string,
  sourceProofRefs: readonly ReturnType<typeof decodeProofRef>[]
): LocalAiResultJournalEntry {
  return LocalAiResultJournalEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    journalEntryId: `local-ai-result-journal:${index}:${result.resultId}`,
    sourceResultId: result.resultId,
    requestId: result.requestId,
    action: result.action,
    confidence: result.confidence,
    reasonCodes: result.reasonCodes,
    evidenceReferences: result.evidenceReferences,
    parentRuleReferences: result.parentRuleReferences,
    runtimeReferenceId: result.modelRuntime.runtimeReferenceId,
    providerId: result.modelRuntime.providerId,
    modelId: result.modelRuntime.modelId,
    promptVersion: result.promptVersion,
    journalState: journalStateFor(result),
    journaledAt,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    sourceProofRefs,
  });
}

function sqliteRowFromJournalEntry(
  entry: LocalAiResultJournalEntry,
  index: number,
  ingestedAt: string
): LocalAiResultSqliteRow {
  return LocalAiResultSqliteRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    sqliteRowId: `local-ai-result-sqlite:${index}:${entry.sourceResultId}`,
    journalEntryId: entry.journalEntryId,
    sourceResultId: entry.sourceResultId,
    requestId: entry.requestId,
    action: entry.action,
    confidence: entry.confidence,
    evidenceReferenceCount: entry.evidenceReferences.length,
    parentRuleReferenceCount: entry.parentRuleReferences.length,
    runtimeReferenceId: entry.runtimeReferenceId,
    providerId: entry.providerId,
    modelId: entry.modelId,
    promptVersion: entry.promptVersion,
    ingestState: sqliteStateFor(entry),
    ingestedAt,
    sourceProofRefs: entry.sourceProofRefs,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    remoteApiClaimed: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
  });
}

function journalStateFor(result: LocalAiSafetyResult): typeof LocalAiResultJournalStateSchema.Type {
  if (result.unknownState === 'model-unavailable' || result.degradedState === 'provider-unavailable') {
    return 'unavailable';
  }
  if (result.unknownState !== 'none' || result.degradedState !== 'none' || result.confidence < 0.5) {
    return 'manual-required';
  }
  return 'journaled';
}

function sqliteStateFor(entry: LocalAiResultJournalEntry): typeof LocalAiResultSqliteIngestStateSchema.Type {
  if (entry.journalState === 'unavailable') {
    return 'unavailable';
  }
  if (entry.journalState === 'manual-required') {
    return 'manual-required';
  }
  return 'ingested';
}

function localAiResultJournalEntryIsHonest(entry: LocalAiResultJournalEntryCandidate): boolean {
  return (
    entry.sourceProofRefs.length > 0 &&
    (entry.journalState !== 'journaled' || entry.evidenceReferences.length > 0) &&
    entry.parentRuleReferences.length > 0 &&
    !entry.rawPromptRetained &&
    !entry.rawModelOutputRetained &&
    !entry.remoteApiClaimed &&
    !entry.policyAuthorityClaimed &&
    !entry.enforcementClaimed
  );
}

function localAiResultSqliteRowIsHonest(row: LocalAiResultSqliteRowCandidate): boolean {
  return (
    row.sourceProofRefs.length > 0 &&
    (row.ingestState !== 'ingested' || row.evidenceReferenceCount > 0) &&
    row.parentRuleReferenceCount > 0 &&
    !row.rawPromptRetained &&
    !row.rawModelOutputRetained &&
    !row.remoteApiClaimed &&
    !row.policyAuthorityClaimed &&
    !row.enforcementClaimed
  );
}

function localAiResultReadModelSnapshotIsComplete(snapshot: LocalAiResultReadModelSnapshotCandidate): boolean {
  if (snapshot.journalEntries.length === 0 || snapshot.journalEntries.length !== snapshot.sqliteRows.length) {
    return false;
  }

  const journalIds = new Set(snapshot.journalEntries.map((entry) => entry.journalEntryId));
  return (
    snapshot.sqliteRows.every((row) => journalIds.has(row.journalEntryId)) &&
    snapshot.readyResultCount === countSqliteRows(snapshot.sqliteRows, 'ingested') &&
    snapshot.degradedResultCount === countJournalEntries(snapshot.journalEntries, 'manual-required') &&
    snapshot.unavailableResultCount === countJournalEntries(snapshot.journalEntries, 'unavailable') &&
    LocalAiResultJournalSqliteNonClaims.every((nonClaim) => snapshot.nonClaims.includes(nonClaim))
  );
}

function countJournalEntries(
  entries: readonly Pick<LocalAiResultJournalEntry, 'journalState'>[],
  journalState: typeof LocalAiResultJournalStateSchema.Type
): number {
  return entries.filter((entry) => entry.journalState === journalState).length;
}

function countSqliteRows(
  rows: readonly Pick<LocalAiResultSqliteRow, 'ingestState'>[],
  ingestState: typeof LocalAiResultSqliteIngestStateSchema.Type
): number {
  return rows.filter((row) => row.ingestState === ingestState).length;
}
