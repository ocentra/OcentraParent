import type { Infer } from './effect';
import type {
  BrowserGameReadModelReasonSchema,
  BrowserGameReadModelSourceKindSchema,
  BrowserGameReadModelStorageStateSchema,
  BrowserGameReadModelSourceKind,
} from './browser-game-journal-sqlite-read-model-values';

export type BrowserGameJournalSqliteReadModelRowGuardInput = {
  sourceKind: Infer<typeof BrowserGameReadModelSourceKindSchema>;
  rowState: 'partial-proof' | 'manual-required' | 'unavailable';
  journalState: Infer<typeof BrowserGameReadModelStorageStateSchema>;
  sqliteState: Infer<typeof BrowserGameReadModelStorageStateSchema>;
  browserEvidenceReadModelRef: unknown | null;
  appGameSessionReportRef: unknown | null;
  adapterPlanAuditRef: unknown | null;
  policyCandidateRef: unknown | null;
  journalEntryRefs: ReadonlyArray<unknown>;
  sqliteRowRefs: ReadonlyArray<unknown>;
  proofRefs: ReadonlyArray<unknown>;
  eventCount: number;
  rowCount: number;
  reasonCodes: ReadonlyArray<Infer<typeof BrowserGameReadModelReasonSchema>>;
  rawUrlIncluded: boolean;
  rawPageBodyIncluded: boolean;
  rawGamePayloadIncluded: boolean;
  rawGameTitleIncluded: boolean;
  rawAccountOrPurchaseIncluded: boolean;
  childCookieSessionReused: boolean;
  cloudTitleCertaintyClaimed: boolean;
  browserMutationClaimed: boolean;
  renderedUiClaimed: boolean;
  finalPolicyDecisionClaimed: boolean;
  enforcementClaimed: boolean;
};

export type BrowserGameJournalSqliteReadModelSnapshotGuardInput = {
  rows: ReadonlyArray<BrowserGameJournalSqliteReadModelRowGuardInput>;
};

const RequiredBrowserGameReadModelSources = [
  'managed-browser-evidence',
  'app-game-session-report',
  'adapter-plan-audit',
  'manual-required',
  'unavailable',
] as const satisfies ReadonlyArray<BrowserGameReadModelSourceKind>;

export function browserGameJournalSqliteReadModelSnapshotIsComplete(
  snapshot: BrowserGameJournalSqliteReadModelSnapshotGuardInput
): boolean {
  const sourceKinds = new Set(snapshot.rows.map((row) => row.sourceKind));
  return RequiredBrowserGameReadModelSources.every((sourceKind) => sourceKinds.has(sourceKind));
}

export function browserGameJournalSqliteReadModelRowIsHonest(
  row: BrowserGameJournalSqliteReadModelRowGuardInput
): boolean {
  if (browserGameJournalSqliteReadModelRowClaimsRuntime(row)) {
    return false;
  }
  if (row.sourceKind === 'managed-browser-evidence') {
    return provedBrowserEvidenceRow(row);
  }
  if (row.sourceKind === 'app-game-session-report') {
    return provedAppGameSessionRow(row);
  }
  if (row.sourceKind === 'adapter-plan-audit') {
    return provedAdapterAuditRow(row);
  }
  if (row.sourceKind === 'manual-required') {
    return manualRequiredRow(row);
  }
  return unavailableRow(row);
}

function provedBrowserEvidenceRow(row: BrowserGameJournalSqliteReadModelRowGuardInput): boolean {
  return (
    provedStorage(row) &&
    row.browserEvidenceReadModelRef !== null &&
    row.reasonCodes.includes('browser-journal-replay-proof-present') &&
    row.reasonCodes.includes('sqlite-read-model-proof-present')
  );
}

function provedAppGameSessionRow(row: BrowserGameJournalSqliteReadModelRowGuardInput): boolean {
  return (
    provedStorage(row) &&
    row.appGameSessionReportRef !== null &&
    row.reasonCodes.includes('app-game-session-read-model-present')
  );
}

function provedAdapterAuditRow(row: BrowserGameJournalSqliteReadModelRowGuardInput): boolean {
  return (
    provedStorage(row) && row.adapterPlanAuditRef !== null && row.reasonCodes.includes('adapter-audit-ref-present')
  );
}

function provedStorage(row: BrowserGameJournalSqliteReadModelRowGuardInput): boolean {
  return (
    row.rowState === 'partial-proof' &&
    row.journalState === 'journal-replayed' &&
    row.sqliteState === 'read-model-present' &&
    row.journalEntryRefs.length > 0 &&
    row.sqliteRowRefs.length > 0 &&
    row.proofRefs.length > 0 &&
    row.eventCount > 0 &&
    row.rowCount > 0
  );
}

function manualRequiredRow(row: BrowserGameJournalSqliteReadModelRowGuardInput): boolean {
  return (
    row.rowState === 'manual-required' &&
    row.journalState === 'manual-required' &&
    row.sqliteState === 'manual-required' &&
    row.reasonCodes.some((reason) => reason.endsWith('manual-required'))
  );
}

function unavailableRow(row: BrowserGameJournalSqliteReadModelRowGuardInput): boolean {
  return (
    row.rowState === 'unavailable' &&
    row.journalState === 'unavailable' &&
    row.sqliteState === 'unavailable' &&
    (row.reasonCodes.includes('native-game-control-unavailable') ||
      row.reasonCodes.includes('unmanaged-browser-exact-url-unavailable'))
  );
}

function browserGameJournalSqliteReadModelRowClaimsRuntime(
  row: BrowserGameJournalSqliteReadModelRowGuardInput
): boolean {
  return BrowserGameReadModelRuntimeClaimFields.some((field) => row[field] === true);
}

const BrowserGameReadModelRuntimeClaimFields = [
  'rawUrlIncluded',
  'rawPageBodyIncluded',
  'rawGamePayloadIncluded',
  'rawGameTitleIncluded',
  'rawAccountOrPurchaseIncluded',
  'childCookieSessionReused',
  'cloudTitleCertaintyClaimed',
  'browserMutationClaimed',
  'renderedUiClaimed',
  'finalPolicyDecisionClaimed',
  'enforcementClaimed',
] as const satisfies ReadonlyArray<keyof BrowserGameJournalSqliteReadModelRowGuardInput>;
