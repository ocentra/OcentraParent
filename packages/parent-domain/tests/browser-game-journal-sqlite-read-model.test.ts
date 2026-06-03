import { describe, expect, it } from 'vitest';
import {
  BrowserGameJournalSqliteReadModelRowSchema,
  BrowserGameJournalSqliteReadModelSnapshotSchema,
} from '../src/browser-game-journal-sqlite-read-model';
import {
  browserGameReadModelRow,
  browserGameReadModelSnapshot,
  manualRequiredRow,
  unavailableRow,
} from './browser-game-journal-sqlite-read-model-fixtures';

describe('browser game journal sqlite read model contracts', () => {
  it(
    'accepts a snapshot covering browser evidence, app-game, adapter audit, manual, and unavailable rows',
    acceptsSnapshot
  );
  it('accepts manual-required and unavailable rows without stored events', acceptsFallbackRows);
  it('rejects raw payload, child session, UI, browser mutation, policy, and enforcement claims', rejectsClaims);
  it('rejects proof-backed rows without journal, SQLite, or source refs', rejectsMissingProofRefs);
});

function acceptsSnapshot() {
  const snapshot = BrowserGameJournalSqliteReadModelSnapshotSchema.parse(browserGameReadModelSnapshot());
  const sourceKinds = snapshot.rows.map((row) => row.sourceKind);

  expect(sourceKinds).toEqual([
    'managed-browser-evidence',
    'app-game-session-report',
    'adapter-plan-audit',
    'manual-required',
    'unavailable',
  ]);
  expect(snapshot.rows[0].journalState).toBe('journal-replayed');
  expect(snapshot.rows[0].sqliteState).toBe('read-model-present');
  expect(snapshot.rows[0].rawUrlIncluded).toBe(false);
  expect(snapshot.rows[0].enforcementClaimed).toBe(false);
}

function acceptsFallbackRows() {
  expect(BrowserGameJournalSqliteReadModelRowSchema.safeParse(manualRequiredRow()).success).toBe(true);
  expect(BrowserGameJournalSqliteReadModelRowSchema.safeParse(unavailableRow()).success).toBe(true);
}

function rejectsClaims() {
  const valid = browserGameReadModelRow();
  const invalidRows = [
    { ...valid, rawUrlIncluded: true },
    { ...valid, rawPageBodyIncluded: true },
    { ...valid, rawGamePayloadIncluded: true },
    { ...valid, rawGameTitleIncluded: true },
    { ...valid, rawAccountOrPurchaseIncluded: true },
    { ...valid, childCookieSessionReused: true },
    { ...valid, cloudTitleCertaintyClaimed: true },
    { ...valid, browserMutationClaimed: true },
    { ...valid, renderedUiClaimed: true },
    { ...valid, finalPolicyDecisionClaimed: true },
    { ...valid, enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameJournalSqliteReadModelRowSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsMissingProofRefs() {
  const valid = browserGameReadModelRow();
  const invalidRows = [
    { ...valid, browserEvidenceReadModelRef: null },
    { ...valid, journalState: 'manual-required' },
    { ...valid, sqliteState: 'manual-required' },
    { ...valid, journalEntryRefs: [] },
    { ...valid, sqliteRowRefs: [] },
    { ...valid, proofRefs: [] },
    { ...valid, eventCount: 0 },
    { ...valid, rowCount: 0 },
    { ...valid, reasonCodes: ['browser-journal-replay-proof-present'] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameJournalSqliteReadModelRowSchema.safeParse(invalid).success).toBe(false);
  }
}
