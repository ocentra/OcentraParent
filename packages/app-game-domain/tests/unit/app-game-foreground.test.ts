import { describe, expect, it } from 'vitest';
import {
  AppGameCapabilityStatus,
  AppGameCatalogReadyState,
  AppGameClassificationState,
  AppGameForegroundEvidenceSchema,
  AppGameForegroundState,
  AppGameObservationMode,
  AppGameRuntimeState,
  AppGameSchemaVersion,
} from '../../src/app-game';
import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';

const ForegroundEvidenceRef = {
  evidenceId: 'journal-entry-app-game-foreground-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-game-foreground-digest',
  uri: null,
} as const;

const ActiveForegroundEvidence = {
  schemaVersion: AppGameSchemaVersion,
  foregroundEvidenceId: 'foreground-evidence-window-4242',
  observedAt: '2026-05-21T02:10:00Z',
  processIdentity: 'process-4242',
  processId: 4242,
  processName: 'ocentra-fixture.exe',
  inventoryEntryId: 'inventory-ocentra-fixture',
  launcherRef: null,
  catalogRef: 'catalog-ocentra-fixture',
  windowRef: 'window-ref-4242',
  windowTitleRef: 'title-ref-4242',
  titleCaptureState: 'titleRef',
  foregroundStartedAt: '2026-05-21T02:10:00Z',
  foregroundEndedAt: null,
  foregroundDurationMs: 0,
  runtimeState: AppGameRuntimeState.Running,
  foregroundState: AppGameForegroundState.Foreground,
  observationMode: AppGameObservationMode.ForegroundWindow,
  classificationState: AppGameClassificationState.KnownApp,
  catalogReadyState: AppGameCatalogReadyState.CatalogReady,
  capabilityStatus: AppGameCapabilityStatus.Available,
  contentKnowledgeState: 'notClaimed',
  confidence: 0.84,
  evidence: [ForegroundEvidenceRef],
} as const;

const assertActiveForegroundEvidence = () => {
  const parsed = AppGameForegroundEvidenceSchema.safeParse(ActiveForegroundEvidence);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.foregroundState).toBe(AppGameForegroundState.Foreground);
    expect(parsed.data.runtimeState).toBe(AppGameRuntimeState.Running);
    expect(parsed.data.contentKnowledgeState).toBe('notClaimed');
    expect(parsed.data.windowTitleRef).toBe('title-ref-4242');
  }
};

const assertBackgroundRowClosesInterval = () => {
  const closed = AppGameForegroundEvidenceSchema.safeParse({
    ...ActiveForegroundEvidence,
    foregroundEvidenceId: 'foreground-evidence-window-4242-closed',
    observedAt: '2026-05-21T02:15:00Z',
    foregroundState: AppGameForegroundState.Background,
    foregroundStartedAt: '2026-05-21T02:10:00Z',
    foregroundEndedAt: '2026-05-21T02:15:00Z',
    foregroundDurationMs: 300000,
    windowTitleRef: null,
    titleCaptureState: 'titleOmitted',
  });
  const openBackground = AppGameForegroundEvidenceSchema.safeParse({
    ...ActiveForegroundEvidence,
    foregroundState: AppGameForegroundState.Background,
    foregroundEndedAt: null,
  });

  expect(closed.success).toBe(true);
  expect(openBackground.success).toBe(false);
};

const assertBackgroundProcessDoesNotGainForegroundTime = () => {
  const parsed = AppGameForegroundEvidenceSchema.safeParse({
    ...ActiveForegroundEvidence,
    foregroundEvidenceId: 'foreground-evidence-window-background-gap',
    foregroundState: AppGameForegroundState.Background,
    foregroundStartedAt: null,
    foregroundEndedAt: '2026-05-21T02:11:00Z',
    foregroundDurationMs: 0,
    windowRef: null,
    windowTitleRef: null,
    titleCaptureState: 'titleOmitted',
  });

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.foregroundDurationMs).toBe(0);
  }
};

const assertTitleCanBeOmitted = () => {
  const parsed = AppGameForegroundEvidenceSchema.safeParse({
    ...ActiveForegroundEvidence,
    foregroundEvidenceId: 'foreground-evidence-window-title-omitted',
    windowTitleRef: null,
    titleCaptureState: 'titleOmitted',
  });
  const invalidTitle = AppGameForegroundEvidenceSchema.safeParse({
    ...ActiveForegroundEvidence,
    foregroundEvidenceId: 'foreground-evidence-window-invalid-title',
    windowTitleRef: 'title-ref-without-state',
    titleCaptureState: 'titleOmitted',
  });

  expect(parsed.success).toBe(true);
  expect(invalidTitle.success).toBe(false);
};

const assertPermissionLimitedForegroundState = () => {
  const parsed = AppGameForegroundEvidenceSchema.safeParse({
    ...ActiveForegroundEvidence,
    foregroundEvidenceId: 'foreground-evidence-window-permission-limited',
    processIdentity: 'foreground-permission-limited',
    processId: 0,
    processName: 'unknown-window-process',
    inventoryEntryId: null,
    catalogRef: null,
    windowRef: null,
    windowTitleRef: null,
    titleCaptureState: 'permissionLimited',
    foregroundStartedAt: null,
    foregroundEndedAt: null,
    foregroundDurationMs: 0,
    runtimeState: AppGameRuntimeState.Unknown,
    foregroundState: AppGameForegroundState.PermissionLimited,
    classificationState: AppGameClassificationState.PermissionLimited,
    catalogReadyState: AppGameCatalogReadyState.PermissionLimited,
    capabilityStatus: AppGameCapabilityStatus.PermissionLimited,
    confidence: 0,
  });

  expect(parsed.success).toBe(true);
};

const assertForegroundEvidenceCannotClaimContent = () => {
  const parsed = AppGameForegroundEvidenceSchema.safeParse({
    ...ActiveForegroundEvidence,
    contentKnowledgeState: 'windowTitleContent',
  });

  expect(parsed.success).toBe(false);
};

describe('app game foreground evidence contracts', () => {
  it('AppGameForegroundEvidenceSchema: accepts active foreground focus evidence', assertActiveForegroundEvidence);
  it(
    'AppGameForegroundEvidenceSchema: requires background rows to close foreground intervals',
    assertBackgroundRowClosesInterval
  );
  it(
    'AppGameForegroundEvidenceSchema: keeps background processes from gaining foreground duration',
    assertBackgroundProcessDoesNotGainForegroundTime
  );
  it('AppGameForegroundEvidenceSchema: allows omitted title refs without content claims', assertTitleCanBeOmitted);
  it(
    'AppGameForegroundEvidenceSchema: preserves permission-limited foreground state',
    assertPermissionLimitedForegroundState
  );
  it(
    'AppGameForegroundEvidenceSchema: rejects foreground evidence that claims content knowledge',
    assertForegroundEvidenceCannotClaimContent
  );
});
