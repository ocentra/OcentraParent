import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  browserGameJournalSqliteReadModelRowIsHonest,
  browserGameJournalSqliteReadModelSnapshotIsComplete,
} from './browser-game-journal-sqlite-read-model-guards';
import {
  BrowserGameJournalSqliteReadModelIdSchema,
  BrowserGameJournalSqliteReadModelRowIdSchema,
  BrowserGameJournalSqliteReadModelSchemaVersionSchema,
  BrowserGameReadModelReasonSchema,
  BrowserGameReadModelRowStateSchema,
  BrowserGameReadModelSourceKindSchema,
  BrowserGameReadModelStorageStateSchema,
} from './browser-game-journal-sqlite-read-model-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const BrowserGameReadModelRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema);
const BrowserGameReadModelNonEmptyRefsSchema = BrowserGameReadModelRefsSchema.pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game read-model refs')
);
const BrowserGameReadModelReasonsSchema = Schema.Array(BrowserGameReadModelReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game read-model reason codes')
);
const BrowserGameReadModelCountSchema = Schema.Number.pipe(
  Schema.filter((value) => value >= 0 || 'Expected non-negative browser-game read-model count')
);

const BrowserGameJournalSqliteReadModelRowBaseSchema = Schema.Struct({
  rowId: BrowserGameJournalSqliteReadModelRowIdSchema,
  sourceKind: BrowserGameReadModelSourceKindSchema,
  rowState: BrowserGameReadModelRowStateSchema,
  journalState: BrowserGameReadModelStorageStateSchema,
  sqliteState: BrowserGameReadModelStorageStateSchema,
  browserEvidenceReadModelRef: OptionalParentEvidenceRefSchema,
  appGameSessionReportRef: OptionalParentEvidenceRefSchema,
  adapterPlanAuditRef: OptionalParentEvidenceRefSchema,
  policyCandidateRef: OptionalParentEvidenceRefSchema,
  journalEntryRefs: BrowserGameReadModelRefsSchema,
  sqliteRowRefs: BrowserGameReadModelRefsSchema,
  proofRefs: BrowserGameReadModelNonEmptyRefsSchema,
  eventCount: BrowserGameReadModelCountSchema,
  rowCount: BrowserGameReadModelCountSchema,
  reasonCodes: BrowserGameReadModelReasonsSchema,
  rawUrlIncluded: Schema.Boolean,
  rawPageBodyIncluded: Schema.Boolean,
  rawGamePayloadIncluded: Schema.Boolean,
  rawGameTitleIncluded: Schema.Boolean,
  rawAccountOrPurchaseIncluded: Schema.Boolean,
  childCookieSessionReused: Schema.Boolean,
  cloudTitleCertaintyClaimed: Schema.Boolean,
  browserMutationClaimed: Schema.Boolean,
  renderedUiClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

export const BrowserGameJournalSqliteReadModelRowSchema = withParser(
  BrowserGameJournalSqliteReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        browserGameJournalSqliteReadModelRowIsHonest(row) ||
        'Expected browser-game journal/SQLite read-model row to stay proof-linked without raw payload or enforcement claims'
    )
  )
);

export const BrowserGameJournalSqliteClaimBoundariesSchema = withParser(
  Schema.Struct({
    rawUrlStorage: Schema.Literal('not-claimed'),
    rawPageBodyStorage: Schema.Literal('not-claimed'),
    rawGamePayloadStorage: Schema.Literal('not-claimed'),
    rawGameTitleStorage: Schema.Literal('not-claimed'),
    rawAccountOrPurchaseStorage: Schema.Literal('not-claimed'),
    childCookieSessionReuse: Schema.Literal('not-claimed'),
    cloudTitleCertainty: Schema.Literal('not-claimed'),
    browserMutation: Schema.Literal('not-claimed'),
    renderedUi: Schema.Literal('not-claimed'),
    finalPolicyDecision: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
  })
);

const BrowserGameJournalSqliteReadModelSnapshotBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameJournalSqliteReadModelSchemaVersionSchema,
  readModelId: BrowserGameJournalSqliteReadModelIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceProofRefs: BrowserGameReadModelNonEmptyRefsSchema,
  rows: Schema.Array(BrowserGameJournalSqliteReadModelRowSchema),
  claimBoundaries: BrowserGameJournalSqliteClaimBoundariesSchema,
});

export const BrowserGameJournalSqliteReadModelSnapshotSchema = withParser(
  BrowserGameJournalSqliteReadModelSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        browserGameJournalSqliteReadModelSnapshotIsComplete(snapshot) ||
        'Expected browser-game journal/SQLite read-model snapshot to cover browser evidence, app-game session, adapter audit, manual, and unavailable rows'
    )
  )
);

export const decodeBrowserGameJournalSqliteReadModelSnapshot = Schema.decodeUnknownSync(
  BrowserGameJournalSqliteReadModelSnapshotSchema
);

export type BrowserGameJournalSqliteReadModelRow = Infer<typeof BrowserGameJournalSqliteReadModelRowSchema>;
export type BrowserGameJournalSqliteReadModelSnapshot = Infer<typeof BrowserGameJournalSqliteReadModelSnapshotSchema>;
