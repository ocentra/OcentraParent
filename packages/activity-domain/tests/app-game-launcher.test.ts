import { describe, expect, it } from 'vitest';
import {
  AppGameCapabilityStatus,
  AppGameCatalogReadyState,
  AppGameClassificationState,
  AppGameForegroundState,
  AppGameLauncherEvidenceSchema,
  AppGameLauncherGameProofState,
  AppGameObservationMode,
  AppGameRuntimeState,
  AppGameSchemaVersion,
} from '../src/app-game';
import { ActivityEvidenceKind } from '../src/kinds';

const LauncherEvidenceRef = {
  evidenceId: 'journal-entry-app-game-launcher-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-game-launcher-digest',
  uri: null,
} as const;

const SteamLauncherEvidence = {
  schemaVersion: AppGameSchemaVersion,
  launcherEvidenceId: 'launcher-evidence-steam-only',
  observedAt: '2026-05-21T02:10:00Z',
  launcherKind: 'steam',
  launcherRef: 'launcher-steam',
  launcherInventoryEntryId: 'inventory-steam-launcher',
  launcherManifestId: 'manifest-steam-library',
  launcherAppId: null,
  launcherProcessIdentity: 'process-steam',
  launcherProcessId: 5150,
  launcherProcessName: 'steam.exe',
  childProcessIdentity: null,
  childInventoryEntryId: null,
  childGameEvidenceClaimId: null,
  catalogRef: null,
  runtimeState: AppGameRuntimeState.Running,
  foregroundState: AppGameForegroundState.NotClaimed,
  observationMode: AppGameObservationMode.ProcessSnapshot,
  classificationState: AppGameClassificationState.KnownLauncher,
  catalogReadyState: AppGameCatalogReadyState.CatalogUnavailable,
  capabilityStatus: AppGameCapabilityStatus.Available,
  gameProofState: AppGameLauncherGameProofState.LauncherOnly,
  confidence: 0.74,
  evidence: [LauncherEvidenceRef],
} as const;

const assertLauncherOnlyEvidenceStaysLauncher = () => {
  const parsed = AppGameLauncherEvidenceSchema.safeParse(SteamLauncherEvidence);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.classificationState).toBe(AppGameClassificationState.KnownLauncher);
    expect(parsed.data.gameProofState).toBe(AppGameLauncherGameProofState.LauncherOnly);
    expect(parsed.data.childGameEvidenceClaimId).toBeNull();
  }
};

const assertLauncherGameCandidateIsNotKnownGame = () => {
  const parsed = AppGameLauncherEvidenceSchema.safeParse({
    ...SteamLauncherEvidence,
    launcherEvidenceId: 'launcher-evidence-steam-candidate',
    launcherAppId: 'steam-app-730',
    childProcessIdentity: 'process-cs2-candidate',
    gameProofState: AppGameLauncherGameProofState.ChildProcessCandidate,
    classificationState: AppGameClassificationState.LauncherGameCandidate,
    confidence: 0.52,
  });
  const overclaim = AppGameLauncherEvidenceSchema.safeParse({
    ...SteamLauncherEvidence,
    launcherEvidenceId: 'launcher-evidence-steam-overclaim',
    gameProofState: AppGameLauncherGameProofState.ChildProcessCandidate,
    classificationState: AppGameClassificationState.KnownGame,
  });

  expect(parsed.success).toBe(true);
  expect(overclaim.success).toBe(false);
};

const assertKnownGameRequiresChildGameEvidence = () => {
  const parsed = AppGameLauncherEvidenceSchema.safeParse({
    ...SteamLauncherEvidence,
    launcherEvidenceId: 'launcher-evidence-steam-known-game',
    launcherAppId: 'steam-app-730',
    childProcessIdentity: 'process-cs2',
    childInventoryEntryId: 'inventory-cs2',
    childGameEvidenceClaimId: 'claim-cs2-child-game',
    catalogRef: 'catalog-cs2',
    gameProofState: AppGameLauncherGameProofState.DeterministicChildGame,
    classificationState: AppGameClassificationState.KnownGame,
    catalogReadyState: AppGameCatalogReadyState.CatalogReady,
    confidence: 0.91,
  });
  const missingProof = AppGameLauncherEvidenceSchema.safeParse({
    ...SteamLauncherEvidence,
    launcherEvidenceId: 'launcher-evidence-steam-known-game-missing-proof',
    gameProofState: AppGameLauncherGameProofState.DeterministicChildGame,
    classificationState: AppGameClassificationState.KnownGame,
    childGameEvidenceClaimId: null,
  });

  expect(parsed.success).toBe(true);
  expect(missingProof.success).toBe(false);
};

const assertPermissionLimitedLauncherIsExplicit = () => {
  const parsed = AppGameLauncherEvidenceSchema.safeParse({
    ...SteamLauncherEvidence,
    launcherEvidenceId: 'launcher-evidence-permission-limited',
    launcherInventoryEntryId: null,
    launcherManifestId: null,
    launcherAppId: null,
    launcherProcessIdentity: null,
    launcherProcessId: null,
    launcherProcessName: null,
    runtimeState: AppGameRuntimeState.PermissionLimited,
    foregroundState: AppGameForegroundState.PermissionLimited,
    classificationState: AppGameClassificationState.PermissionLimited,
    catalogReadyState: AppGameCatalogReadyState.PermissionLimited,
    capabilityStatus: AppGameCapabilityStatus.PermissionLimited,
    gameProofState: AppGameLauncherGameProofState.PermissionLimited,
    confidence: 0,
  });

  expect(parsed.success).toBe(true);
};

describe('app game launcher evidence contracts', () => {
  it(
    'AppGameLauncherEvidenceSchema: keeps launcher-only evidence as launcher',
    assertLauncherOnlyEvidenceStaysLauncher
  );
  it(
    'AppGameLauncherEvidenceSchema: keeps launcher-game candidates out of known-game state',
    assertLauncherGameCandidateIsNotKnownGame
  );
  it(
    'AppGameLauncherEvidenceSchema: requires child-game evidence before known-game classification',
    assertKnownGameRequiresChildGameEvidence
  );
  it(
    'AppGameLauncherEvidenceSchema: preserves permission-limited launcher state',
    assertPermissionLimitedLauncherIsExplicit
  );
});
