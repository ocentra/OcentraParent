import { describe, expect, it } from 'vitest';
import {
  AppGameAiActionHint,
  AppGameAiClassificationDigestSchema,
  AppGameCatalogReadyState,
  AppGameClassificationState,
  AppGameEvidenceClaimKind,
  AppGameEvidenceClaimSchema,
  AppGameForegroundState,
  AppGameIdentityStrength,
  AppGameObservationMode,
  AppGameRuntimeState,
  AppGameSchemaVersion,
} from '../../src/app-game';
import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';

const JournalEvidence = {
  evidenceId: 'journal-entry-app-game-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-game-journal-digest',
  uri: null,
} as const;

const DisplayNameOnlyEvidenceClaim = {
  schemaVersion: AppGameSchemaVersion,
  claimId: 'claim-display-name-only-1',
  observedAt: '2026-05-21T02:00:00Z',
  claimKind: AppGameEvidenceClaimKind.Inventory,
  observationMode: AppGameObservationMode.InventoryScan,
  displayName: 'Unverified Game Name',
  identityStrength: AppGameIdentityStrength.DisplayNameOnly,
  classificationState: AppGameClassificationState.UnknownProcess,
  catalogReadyState: AppGameCatalogReadyState.CatalogNotLoaded,
  runtimeState: AppGameRuntimeState.NotClaimed,
  foregroundState: AppGameForegroundState.NotClaimed,
  inventoryEntryId: null,
  processIdentity: null,
  launcherRef: null,
  catalogRef: null,
  confidence: 0.2,
  evidence: [JournalEvidence],
} as const;

const InventoryEvidenceClaim = {
  schemaVersion: AppGameSchemaVersion,
  claimId: 'claim-inventory-only-1',
  observedAt: '2026-05-21T02:00:00Z',
  claimKind: AppGameEvidenceClaimKind.Inventory,
  observationMode: AppGameObservationMode.InventoryScan,
  displayName: 'Elden Ring',
  identityStrength: AppGameIdentityStrength.CatalogMatched,
  classificationState: AppGameClassificationState.KnownGame,
  catalogReadyState: AppGameCatalogReadyState.CatalogReady,
  runtimeState: AppGameRuntimeState.NotClaimed,
  foregroundState: AppGameForegroundState.NotClaimed,
  inventoryEntryId: 'inventory-elden-ring',
  processIdentity: null,
  launcherRef: 'launcher-steam',
  catalogRef: 'catalog-steam-1245620',
  confidence: 0.92,
  evidence: [JournalEvidence],
} as const;

const LauncherEvidenceClaim = {
  schemaVersion: AppGameSchemaVersion,
  claimId: 'claim-launcher-candidate-1',
  observedAt: '2026-05-21T02:00:00Z',
  claimKind: AppGameEvidenceClaimKind.Launcher,
  observationMode: AppGameObservationMode.LauncherManifest,
  displayName: 'Elden Ring',
  identityStrength: AppGameIdentityStrength.LauncherClaimed,
  classificationState: AppGameClassificationState.LauncherGameCandidate,
  catalogReadyState: AppGameCatalogReadyState.CatalogReady,
  runtimeState: AppGameRuntimeState.NotClaimed,
  foregroundState: AppGameForegroundState.NotClaimed,
  inventoryEntryId: 'inventory-elden-ring',
  processIdentity: null,
  launcherRef: 'launcher-steam',
  catalogRef: 'catalog-steam-1245620',
  confidence: 0.84,
  evidence: [JournalEvidence],
} as const;

const assertDisplayNameOnlyStaysWeak = () => {
  const displayNameOnly = AppGameEvidenceClaimSchema.safeParse(DisplayNameOnlyEvidenceClaim);
  const strongDisplayNameOnly = AppGameEvidenceClaimSchema.safeParse({
    ...DisplayNameOnlyEvidenceClaim,
    confidence: 0.8,
  });

  expect(displayNameOnly.success).toBe(true);
  expect(strongDisplayNameOnly.success).toBe(false);
};

const assertInventoryCannotClaimRuntimeUse = () => {
  const inventoryClaim = AppGameEvidenceClaimSchema.safeParse(InventoryEvidenceClaim);
  const inventoryAsRuntime = AppGameEvidenceClaimSchema.safeParse({
    ...InventoryEvidenceClaim,
    runtimeState: AppGameRuntimeState.Running,
    foregroundState: AppGameForegroundState.Foreground,
  });

  expect(inventoryClaim.success).toBe(true);
  expect(inventoryAsRuntime.success).toBe(false);
};

const assertLauncherNeedsChildGameProof = () => {
  const launcherCandidate = AppGameEvidenceClaimSchema.safeParse(LauncherEvidenceClaim);
  const launcherAsKnownGame = AppGameEvidenceClaimSchema.safeParse({
    ...LauncherEvidenceClaim,
    classificationState: AppGameClassificationState.KnownGame,
  });
  const childGameProof = AppGameEvidenceClaimSchema.safeParse({
    ...LauncherEvidenceClaim,
    identityStrength: AppGameIdentityStrength.ChildGameProof,
    classificationState: AppGameClassificationState.KnownGame,
  });

  expect(launcherCandidate.success).toBe(true);
  expect(launcherAsKnownGame.success).toBe(false);
  expect(childGameProof.success).toBe(true);
};

const assertAiDigestCannotAuthorizeActions = () => {
  const parsed = AppGameAiClassificationDigestSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    digestRef: 'ai-digest-session-1',
    digest: 'sha256:app-game-ai-digest',
    generatedAt: '2026-05-21T02:16:00Z',
    classificationState: AppGameClassificationState.PossiblyGame,
    confidence: 0.67,
    actionHints: [AppGameAiActionHint.ClassifyOnly, AppGameAiActionHint.ParentReview],
    sourceEvidenceIds: ['journal-entry-app-game-1'],
    sourceSessionIds: ['session-elden-ring-1'],
    unavailableReason: null,
  });
  const enforcementAttempt = AppGameAiClassificationDigestSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    digestRef: 'ai-digest-session-1',
    digest: 'sha256:app-game-ai-digest',
    generatedAt: '2026-05-21T02:16:00Z',
    classificationState: AppGameClassificationState.PossiblyGame,
    confidence: 0.67,
    actionHints: [AppGameAiActionHint.ClassifyOnly, 'terminate'],
    sourceEvidenceIds: ['journal-entry-app-game-1'],
    sourceSessionIds: ['session-elden-ring-1'],
    unavailableReason: null,
  });

  expect(parsed.success).toBe(true);
  expect(enforcementAttempt.success).toBe(false);
};

describe('app game evidence claim contracts', () => {
  it('keeps display-name-only identity weak and unlinked', assertDisplayNameOnlyStaysWeak);
  it('keeps inventory evidence separate from running and foreground use', assertInventoryCannotClaimRuntimeUse);
  it('keeps launcher evidence from becoming a known game without child-game proof', assertLauncherNeedsChildGameProof);
  it('keeps AI classification digests out of enforcement authority', assertAiDigestCannotAuthorizeActions);
});
