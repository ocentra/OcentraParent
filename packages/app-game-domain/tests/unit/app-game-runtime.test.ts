import { describe, expect, it } from 'vitest';
import {
  AppGameCatalogReadyState,
  AppGameCapabilityStatus,
  AppGameClassificationState,
  AppGameForegroundState,
  AppGameObservationMode,
  AppGameRuntimeEvidenceSchema,
  AppGameRuntimeState,
  AppGameSchemaVersion,
} from '../../src/app-game';
import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';

const RuntimeEvidenceRef = {
  evidenceId: 'journal-entry-app-game-runtime-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-game-runtime-digest',
  uri: null,
} as const;

const RunningAppRuntimeEvidence = {
  schemaVersion: AppGameSchemaVersion,
  runtimeEvidenceId: 'runtime-evidence-process-4242',
  observedAt: '2026-05-21T02:05:00Z',
  processIdentity: 'process-4242',
  processId: 4242,
  parentProcessId: 1000,
  processName: 'ocentra-fixture.exe',
  executablePathRef: 'path-ref-ocentra-fixture',
  publisherSignatureRef: 'signature-ref-ocentra-fixture',
  fileHashRef: 'hash-ref-ocentra-fixture',
  inventoryEntryId: 'inventory-ocentra-fixture',
  launcherRef: null,
  catalogRef: 'catalog-ocentra-fixture',
  startedAt: '2026-05-21T02:00:00Z',
  exitedAt: null,
  runningDurationMs: 300000,
  runtimeState: AppGameRuntimeState.Running,
  foregroundState: AppGameForegroundState.NotClaimed,
  observationMode: AppGameObservationMode.ProcessSnapshot,
  classificationState: AppGameClassificationState.KnownApp,
  catalogReadyState: AppGameCatalogReadyState.CatalogReady,
  capabilityStatus: AppGameCapabilityStatus.Available,
  confidence: 0.82,
  evidence: [RuntimeEvidenceRef],
} as const;

const LauncherRuntimeEvidence = {
  ...RunningAppRuntimeEvidence,
  runtimeEvidenceId: 'runtime-evidence-launcher-5150',
  processIdentity: 'process-5150',
  processId: 5150,
  parentProcessId: null,
  processName: 'steam.exe',
  inventoryEntryId: 'inventory-steam-launcher',
  launcherRef: 'launcher-steam',
  catalogRef: null,
  classificationState: AppGameClassificationState.KnownLauncher,
  catalogReadyState: AppGameCatalogReadyState.CatalogUnavailable,
  confidence: 0.73,
} as const;

const PermissionLimitedRuntimeEvidence = {
  ...RunningAppRuntimeEvidence,
  runtimeEvidenceId: 'runtime-evidence-private-6161',
  processIdentity: 'process-6161',
  processId: 6161,
  parentProcessId: null,
  processName: 'private-process.exe',
  executablePathRef: null,
  publisherSignatureRef: null,
  fileHashRef: null,
  inventoryEntryId: null,
  catalogRef: null,
  startedAt: null,
  runningDurationMs: 0,
  runtimeState: AppGameRuntimeState.PermissionLimited,
  classificationState: AppGameClassificationState.PermissionLimited,
  catalogReadyState: AppGameCatalogReadyState.PermissionLimited,
  capabilityStatus: AppGameCapabilityStatus.PermissionLimited,
  confidence: 0,
} as const;

const assertRunningProcessRuntimeEvidence = () => {
  const parsed = AppGameRuntimeEvidenceSchema.safeParse(RunningAppRuntimeEvidence);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.runtimeState).toBe(AppGameRuntimeState.Running);
    expect(parsed.data.foregroundState).toBe(AppGameForegroundState.NotClaimed);
    expect(parsed.data.parentProcessId).toBe(1000);
    expect(parsed.data.publisherSignatureRef).toBe('signature-ref-ocentra-fixture');
    expect(parsed.data.fileHashRef).toBe('hash-ref-ocentra-fixture');
  }
};

const assertLauncherRuntimeStaysLauncher = () => {
  const parsed = AppGameRuntimeEvidenceSchema.safeParse(LauncherRuntimeEvidence);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.classificationState).toBe(AppGameClassificationState.KnownLauncher);
    expect(parsed.data.launcherRef).toBe('launcher-steam');
    expect(parsed.data.catalogRef).toBeNull();
  }
};

const assertPermissionLimitedRuntimeKeepsMissingMetadataExplicit = () => {
  const parsed = AppGameRuntimeEvidenceSchema.safeParse(PermissionLimitedRuntimeEvidence);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.capabilityStatus).toBe(AppGameCapabilityStatus.PermissionLimited);
    expect(parsed.data.executablePathRef).toBeNull();
    expect(parsed.data.publisherSignatureRef).toBeNull();
    expect(parsed.data.fileHashRef).toBeNull();
  }
};

const assertProcessExitClosesRuntime = () => {
  const closed = AppGameRuntimeEvidenceSchema.safeParse({
    ...RunningAppRuntimeEvidence,
    observationMode: AppGameObservationMode.ProcessExit,
    runtimeState: AppGameRuntimeState.NotRunning,
    exitedAt: '2026-05-21T02:20:00Z',
    runningDurationMs: 1200000,
  });
  const stillRunning = AppGameRuntimeEvidenceSchema.safeParse({
    ...RunningAppRuntimeEvidence,
    observationMode: AppGameObservationMode.ProcessExit,
    runtimeState: AppGameRuntimeState.Running,
    exitedAt: null,
  });

  expect(closed.success).toBe(true);
  expect(stillRunning.success).toBe(false);
};

const assertRuntimeEvidenceCannotClaimForeground = () => {
  const parsed = AppGameRuntimeEvidenceSchema.safeParse({
    ...RunningAppRuntimeEvidence,
    foregroundState: AppGameForegroundState.Foreground,
  });

  expect(parsed.success).toBe(false);
};

describe('app game runtime evidence contracts', () => {
  it(
    'AppGameRuntimeEvidenceSchema: accepts running process evidence with metadata refs',
    assertRunningProcessRuntimeEvidence
  );
  it(
    'AppGameRuntimeEvidenceSchema: keeps launcher process evidence as launcher state',
    assertLauncherRuntimeStaysLauncher
  );
  it(
    'AppGameRuntimeEvidenceSchema: preserves permission-limited process metadata gaps',
    assertPermissionLimitedRuntimeKeepsMissingMetadataExplicit
  );
  it('AppGameRuntimeEvidenceSchema: requires process-exit rows to close runtime state', assertProcessExitClosesRuntime);
  it(
    'AppGameRuntimeEvidenceSchema: rejects runtime evidence that claims foreground',
    assertRuntimeEvidenceCannotClaimForeground
  );
});
