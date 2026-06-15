import { describe, expect, it } from 'vitest';
import {
  AppGameCatalogReadyState,
  AppGameClassificationState,
  AppGameForegroundState,
  AppGameInventoryCategoryKind,
  AppGameInventoryCustodyState,
  AppGameInventoryDetectionState,
  AppGameInventoryEvidenceRowSchema,
  AppGameInventorySourceKind,
  AppGameProductKind,
  AppGameRuntimeState,
  AppGameSchemaVersion,
} from '../../src/app-game';
import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';

const JournalEvidence = {
  evidenceId: 'journal-entry-app-game-inventory-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-game-inventory-digest',
  uri: null,
} as const;

const SteamGameInventoryRow = {
  schemaVersion: AppGameSchemaVersion,
  inventoryEntryId: 'inventory-elden-ring-install',
  observedAt: '2026-05-22T02:00:00Z',
  sourceKind: AppGameInventorySourceKind.LauncherManifest,
  sourceRef: 'source-steam-library-1',
  custodyState: AppGameInventoryCustodyState.LauncherManifest,
  productKind: AppGameProductKind.NativeGame,
  displayLabel: 'Elden Ring',
  identityId: 'identity-elden-ring-game',
  packageId: null,
  bundleId: null,
  appUserModelId: null,
  desktopEntryId: null,
  executablePathRef: 'path-ref-elden-ring',
  launcherRef: 'launcher-steam',
  launcherAppId: 'steam-app-1245620',
  launcherManifestId: 'steam-manifest-1245620',
  storeId: 'steam-store-1245620',
  catalogRef: 'catalog-steam-1245620',
  inventoryState: AppGameInventoryDetectionState.Installed,
  classificationState: AppGameClassificationState.KnownGame,
  catalogReadyState: AppGameCatalogReadyState.CatalogReady,
  capabilityStatus: 'available',
  confidence: 0.96,
  categoryCandidates: [
    {
      categoryKind: AppGameInventoryCategoryKind.Game,
      confidence: 0.98,
      catalogRef: 'catalog-steam-1245620',
      evidence: [JournalEvidence],
    },
  ],
  runtimeState: AppGameRuntimeState.NotClaimed,
  foregroundState: AppGameForegroundState.NotClaimed,
  runningDurationMs: 0,
  foregroundDurationMs: 0,
  evidence: [JournalEvidence],
} as const;

const assertAcceptsLauncherInventoryWithoutUseClaims = () => {
  const parsed = AppGameInventoryEvidenceRowSchema.safeParse(SteamGameInventoryRow);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.sourceKind).toBe('launcherManifest');
    expect(parsed.data.productKind).toBe('nativeGame');
    expect(parsed.data.runtimeState).toBe('notClaimed');
    expect(parsed.data.categoryCandidates[0]?.categoryKind).toBe('game');
  }
};

const assertRejectsInventoryRowsThatClaimUse = () => {
  const running = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    runtimeState: AppGameRuntimeState.Running,
  });
  const foreground = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    foregroundState: AppGameForegroundState.Foreground,
  });
  const duration = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    runningDurationMs: 1,
  });

  expect(running.success).toBe(false);
  expect(foreground.success).toBe(false);
  expect(duration.success).toBe(false);
};

const assertLauncherInstallDoesNotBecomeGameplay = () => {
  const installedGame = AppGameInventoryEvidenceRowSchema.safeParse(SteamGameInventoryRow);
  const gameplayClaim = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    foregroundState: AppGameForegroundState.Foreground,
    foregroundDurationMs: 30000,
  });

  expect(installedGame.success).toBe(true);
  expect(gameplayClaim.success).toBe(false);
};

const assertPreservesPermissionLimitedAndStaleStates = () => {
  const permissionLimited = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    inventoryEntryId: 'inventory-permission-limited',
    sourceKind: AppGameInventorySourceKind.UnknownSource,
    sourceRef: 'source-permission-limited-adapter',
    custodyState: AppGameInventoryCustodyState.Unknown,
    productKind: AppGameProductKind.UnknownExecutable,
    displayLabel: 'Permission-limited inventory',
    identityId: null,
    executablePathRef: null,
    launcherRef: null,
    launcherAppId: null,
    launcherManifestId: null,
    storeId: null,
    catalogRef: null,
    inventoryState: AppGameInventoryDetectionState.PermissionLimited,
    classificationState: AppGameClassificationState.PermissionLimited,
    catalogReadyState: AppGameCatalogReadyState.PermissionLimited,
    capabilityStatus: 'permissionLimited',
    confidence: 0,
    categoryCandidates: [],
  });
  const stale = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    inventoryEntryId: 'inventory-stale-store-package',
    sourceKind: AppGameInventorySourceKind.StorePackage,
    custodyState: AppGameInventoryCustodyState.StorePackage,
    packageId: 'package-ref-stale-store',
    inventoryState: AppGameInventoryDetectionState.Stale,
    capabilityStatus: 'stale',
    confidence: 0.42,
  });

  expect(permissionLimited.success).toBe(true);
  expect(stale.success).toBe(true);
  if (permissionLimited.success && stale.success) {
    expect(permissionLimited.data.inventoryState).toBe('permissionLimited');
    expect(stale.data.inventoryState).toBe('stale');
  }
};

const assertRejectsMismatchedInventorySourceRefs = () => {
  const launcherWithoutLauncherRef = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    launcherRef: null,
    launcherAppId: null,
    launcherManifestId: null,
  });
  const storeWithoutPackageRef = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    sourceKind: AppGameInventorySourceKind.StorePackage,
    custodyState: AppGameInventoryCustodyState.StorePackage,
    packageId: null,
    bundleId: null,
    appUserModelId: null,
    storeId: null,
  });

  expect(launcherWithoutLauncherRef.success).toBe(false);
  expect(storeWithoutPackageRef.success).toBe(false);
};

const assertRejectsHighConfidenceDisplayOnlyInventory = () => {
  const parsed = AppGameInventoryEvidenceRowSchema.safeParse({
    ...SteamGameInventoryRow,
    inventoryEntryId: 'inventory-display-only-high-confidence',
    sourceKind: AppGameInventorySourceKind.UnknownSource,
    sourceRef: 'source-display-name-only',
    custodyState: AppGameInventoryCustodyState.Unknown,
    productKind: AppGameProductKind.UnknownExecutable,
    identityId: null,
    packageId: null,
    bundleId: null,
    appUserModelId: null,
    desktopEntryId: null,
    executablePathRef: null,
    launcherRef: null,
    launcherAppId: null,
    launcherManifestId: null,
    storeId: null,
    catalogRef: null,
    inventoryState: AppGameInventoryDetectionState.Detectable,
    classificationState: AppGameClassificationState.UnknownProcess,
    catalogReadyState: AppGameCatalogReadyState.CatalogUnavailable,
    capabilityStatus: 'degraded',
    confidence: 0.8,
    categoryCandidates: [],
  });

  expect(parsed.success).toBe(false);
};

describe('app game inventory evidence contracts', () => {
  it(
    'AppGameInventoryEvidenceRowSchema: accepts launcher inventory without use claims',
    assertAcceptsLauncherInventoryWithoutUseClaims
  );
  it(
    'AppGameInventoryEvidenceRowSchema: rejects running foreground or duration claims',
    assertRejectsInventoryRowsThatClaimUse
  );
  it(
    'AppGameInventoryEvidenceRowSchema: keeps launcher installs separate from gameplay',
    assertLauncherInstallDoesNotBecomeGameplay
  );
  it(
    'AppGameInventoryEvidenceRowSchema: preserves permission-limited and stale states',
    assertPreservesPermissionLimitedAndStaleStates
  );
  it(
    'AppGameInventoryEvidenceRowSchema: rejects mismatched source references',
    assertRejectsMismatchedInventorySourceRefs
  );
  it(
    'AppGameInventoryEvidenceRowSchema: rejects high-confidence display-only inventory',
    assertRejectsHighConfidenceDisplayOnlyInventory
  );
});
