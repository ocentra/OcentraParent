import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';
import {
  AppGameAiDigestReferenceSchema,
  AppGameCatalogReadyState,
  AppGameClassificationState,
  AppGameForegroundState,
  AppGameInventoryEntrySchema,
  AppGameObservationMode,
  AppGameProcessObservationSchema,
  AppGameSchemaVersion,
  AppGameSessionDailyRollupSchema,
  AppGameSessionEndReason,
  AppGameSessionQueryResultSchema,
  AppGameSessionReportSchema,
  AppGameSessionSummarySchema,
} from '../../src/app-game';

const JournalEvidence = {
  evidenceId: 'journal-entry-app-game-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-game-journal-digest',
  uri: null,
} as const;

const KnownGameSession = {
  schemaVersion: AppGameSchemaVersion,
  sessionId: 'session-elden-ring-1',
  primaryProcessIdentity: 'process-4242',
  displayName: 'Elden Ring',
  classificationState: AppGameClassificationState.KnownGame,
  catalogReadyState: AppGameCatalogReadyState.CatalogReady,
  inventoryEntryId: 'inventory-elden-ring',
  launcherRef: 'launcher-steam',
  catalogRef: 'catalog-steam-1245620',
  startedAt: '2026-05-21T02:00:00Z',
  lastObservedAt: '2026-05-21T02:15:00Z',
  endedAt: null,
  endReason: null,
  runningDurationMs: 900000,
  foregroundDurationMs: 600000,
  backgroundDurationMs: 300000,
  lastForegroundAt: '2026-05-21T02:12:00Z',
  lastBackgroundAt: '2026-05-21T02:15:00Z',
  observationGapMs: 300000,
  observationCount: 3,
  evidenceCount: 1,
  evidence: [JournalEvidence],
  aiDigestRef: 'ai-digest-session-1',
  confidence: 0.94,
} as const;

const KnownGameInventoryEntry = {
  schemaVersion: AppGameSchemaVersion,
  inventoryEntryId: 'inventory-elden-ring',
  observedAt: '2026-05-21T02:00:00Z',
  displayName: 'Elden Ring',
  executablePath: 'C:\\Program Files\\Steam\\steamapps\\common\\ELDEN RING\\Game\\eldenring.exe',
  launcherKind: 'steam',
  launcherRef: 'launcher-steam',
  catalogRef: 'catalog-steam-1245620',
  catalogReadyState: AppGameCatalogReadyState.CatalogReady,
  classificationState: AppGameClassificationState.KnownGame,
  capabilityStatus: 'available',
  confidence: 0.98,
  evidence: [JournalEvidence],
} as const;

const PermissionLimitedObservation = {
  schemaVersion: AppGameSchemaVersion,
  observedAt: '2026-05-21T02:10:00Z',
  processIdentity: 'process-5150',
  processId: 5150,
  processName: 'private-process.exe',
  executablePath: null,
  foregroundState: AppGameForegroundState.PermissionLimited,
  observationMode: AppGameObservationMode.ProcessSnapshot,
  classificationState: AppGameClassificationState.PermissionLimited,
  inventoryEntryId: null,
  launcherRef: null,
  catalogRef: null,
  confidence: 0,
  evidence: [JournalEvidence],
} as const;

const assertKnownGameInventoryEntry = () => {
  const parsed = AppGameInventoryEntrySchema.safeParse(KnownGameInventoryEntry);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.classificationState).toBe('knownGame');
    expect(parsed.data.catalogReadyState).toBe('catalogReady');
  }
};

const assertPermissionLimitedObservation = () => {
  const parsed = AppGameProcessObservationSchema.safeParse(PermissionLimitedObservation);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.classificationState).toBe('permissionLimited');
    expect(parsed.data.inventoryEntryId).toBeNull();
  }
};

const assertSessionQueryResult = () => {
  const parsed = AppGameSessionQueryResultSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    limit: 10,
    returned: 1,
    catalogReadyState: AppGameCatalogReadyState.CatalogReady,
    firstObservedAt: '2026-05-21T02:00:00Z',
    lastObservedAt: '2026-05-21T02:15:00Z',
    sessions: [KnownGameSession],
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.sessions[0]?.aiDigestRef).toBe('ai-digest-session-1');
    expect(parsed.data.sessions[0]?.foregroundDurationMs).toBe(600000);
  }
};

const assertSessionReport = () => {
  const parsed = AppGameSessionReportSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    limit: 10,
    returned: 1,
    catalogReadyState: AppGameCatalogReadyState.CatalogReady,
    firstObservedAt: '2026-05-21T02:00:00Z',
    lastObservedAt: '2026-05-21T02:15:00Z',
    mostRecentSessionId: 'session-elden-ring-1',
    mostRecentClassificationState: AppGameClassificationState.KnownGame,
    mostRecentProcessIdentity: 'process-4242',
    mostRecentDisplayName: 'Elden Ring',
    mostRecentRunningDurationMs: 900000,
    mostRecentForegroundDurationMs: 600000,
    mostRecentEvidenceCount: 1,
  });

  expect(parsed.success).toBe(true);
};

const assertConfidenceBoundaries = () => {
  const processResult = AppGameProcessObservationSchema.safeParse({
    ...PermissionLimitedObservation,
    processName: 'foreground-game.exe',
    foregroundState: AppGameForegroundState.Foreground,
    observationMode: AppGameObservationMode.ForegroundWindow,
    classificationState: AppGameClassificationState.PossiblyGame,
    confidence: 1.1,
  });
  const digestResult = AppGameAiDigestReferenceSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    digestRef: 'ai-digest-session-1',
    digest: 'sha256:app-game-ai-digest',
    generatedAt: '2026-05-21T02:16:00Z',
    confidence: -0.1,
    sourceEvidenceIds: ['journal-entry-app-game-1'],
    sourceSessionIds: ['session-elden-ring-1'],
    unavailableReason: null,
  });

  expect(processResult.success).toBe(false);
  expect(digestResult.success).toBe(false);
};

const assertDurationConsistency = () => {
  const parsed = AppGameSessionSummarySchema.safeParse({
    ...KnownGameSession,
    foregroundDurationMs: 800000,
    backgroundDurationMs: 300000,
  });

  expect(parsed.success).toBe(false);
};

const assertClosedSessionsRequireReason = () => {
  const missingReason = AppGameSessionSummarySchema.safeParse({
    ...KnownGameSession,
    endedAt: '2026-05-21T02:15:00Z',
    endReason: null,
  });
  const missingEnd = AppGameSessionSummarySchema.safeParse({
    ...KnownGameSession,
    endedAt: null,
    endReason: AppGameSessionEndReason.ProcessExit,
  });
  const closed = AppGameSessionSummarySchema.safeParse({
    ...KnownGameSession,
    endedAt: '2026-05-21T02:15:00Z',
    endReason: AppGameSessionEndReason.ProcessExit,
  });

  expect(missingReason.success).toBe(false);
  expect(missingEnd.success).toBe(false);
  expect(closed.success).toBe(true);
};

const assertDurationEvidenceTimesAreRequired = () => {
  const foregroundWithoutEvidenceTime = AppGameSessionSummarySchema.safeParse({
    ...KnownGameSession,
    lastForegroundAt: null,
  });
  const backgroundWithoutEvidenceTime = AppGameSessionSummarySchema.safeParse({
    ...KnownGameSession,
    lastBackgroundAt: null,
  });

  expect(foregroundWithoutEvidenceTime.success).toBe(false);
  expect(backgroundWithoutEvidenceTime.success).toBe(false);
};

const assertDailyRollupDurationConsistency = () => {
  const validRollup = AppGameSessionDailyRollupSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    rollupDate: '2026-05-21',
    classificationState: AppGameClassificationState.KnownGame,
    sessionCount: 1,
    runningDurationMs: 900000,
    foregroundDurationMs: 600000,
    backgroundDurationMs: 300000,
    evidenceCount: 1,
    sessionIds: ['session-elden-ring-1'],
    evidence: [JournalEvidence],
  });
  const invalidRollup = AppGameSessionDailyRollupSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    rollupDate: '2026-05-21',
    classificationState: AppGameClassificationState.KnownGame,
    sessionCount: 1,
    runningDurationMs: 900000,
    foregroundDurationMs: 800000,
    backgroundDurationMs: 300000,
    evidenceCount: 1,
    sessionIds: ['session-elden-ring-1'],
    evidence: [JournalEvidence],
  });

  expect(validRollup.success).toBe(true);
  expect(invalidRollup.success).toBe(false);
};

describe('app game activity contracts', () => {
  it('AppGameInventoryEntrySchema: accepts deterministic known-game inventory state', assertKnownGameInventoryEntry);
  it('AppGameProcessObservationSchema: preserves permission-limited unknowns', assertPermissionLimitedObservation);
  it('AppGameSessionQueryResultSchema: accepts app game session read-model output', assertSessionQueryResult);
  it('AppGameSessionReportSchema: accepts flattened service and portal visibility state', assertSessionReport);
  it('app game confidence contracts: reject values outside the 0..1 boundary', assertConfidenceBoundaries);
  it('AppGameSessionSummarySchema: rejects durations beyond running duration', assertDurationConsistency);
  it('AppGameSessionSummarySchema: pairs closed sessions with an end reason', assertClosedSessionsRequireReason);
  it('AppGameSessionSummarySchema: requires duration evidence timestamps', assertDurationEvidenceTimesAreRequired);
  it('AppGameSessionDailyRollupSchema: requires exact duration totals', assertDailyRollupDurationConsistency);
});
