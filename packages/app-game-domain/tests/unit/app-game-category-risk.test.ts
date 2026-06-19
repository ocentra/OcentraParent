import { describe, expect, it } from 'vitest';
import {
  AppGameCategoryCandidateSchema,
  AppGameCategoryCandidateSetSchema,
} from '../../src/app-game-category-risk';
import {
  AppGameCategoryCandidateState,
  AppGameCategoryEnforcementState,
  AppGameCategoryFamily,
  AppGameCategoryPolicyCandidateAction,
  AppGameCategorySourceKind,
  AppGameContextSignalKind,
  AppGameNativeAppCategory,
  AppGameNativeGameCategory,
  AppGameRiskSignalKind,
} from '../../src/app-game-category-risk-primitives';
import { AppGameProductKind } from '../../src/app-game-identity-primitives';
import { AppGameSchemaVersion } from '../../src/app-game-primitives';
import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';

const CatalogEvidence = {
  evidenceId: 'evidence-app-category-catalog-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-category-catalog',
  uri: null,
} as const;

const LocalAiEvidence = {
  evidenceId: 'evidence-app-category-ai-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-category-ai',
  uri: null,
} as const;

const KnownCatalogCategoryCandidate = {
  schemaVersion: AppGameSchemaVersion,
  candidateId: 'category-candidate-known-school',
  inventoryEntryId: 'inventory-school-app',
  identityId: 'identity-school-app',
  productKind: AppGameProductKind.NativeApp,
  categoryFamily: AppGameCategoryFamily.NativeApp,
  nativeAppCategory: AppGameNativeAppCategory.School,
  nativeGameCategory: null,
  riskSignal: null,
  gameContextSignal: null,
  sourceKind: AppGameCategorySourceKind.Catalog,
  sourceRef: 'source-parent-catalog-v1',
  candidateState: AppGameCategoryCandidateState.CatalogCandidate,
  confidence: 0.97,
  reasonCode: 'catalog-school-category',
  evidence: [CatalogEvidence],
  catalogRef: 'catalog-school-app',
  aiDigestRef: null,
  parentOverride: null,
  policyCandidateAction: AppGameCategoryPolicyCandidateAction.Observe,
  enforcementState: AppGameCategoryEnforcementState.NotEnforcement,
} as const;

const UnknownVpnNamedCandidate = {
  ...KnownCatalogCategoryCandidate,
  candidateId: 'category-candidate-vpn-name',
  inventoryEntryId: 'inventory-unknown-vpn-name',
  identityId: null,
  productKind: AppGameProductKind.UnknownExecutable,
  categoryFamily: AppGameCategoryFamily.RiskCandidate,
  nativeAppCategory: null,
  riskSignal: AppGameRiskSignalKind.VpnProxy,
  sourceKind: AppGameCategorySourceKind.ExecutableName,
  sourceRef: 'source-process-name-vpn-like',
  candidateState: AppGameCategoryCandidateState.NameHeuristicCandidate,
  confidence: 0.42,
  reasonCode: 'name-looks-like-vpn',
  catalogRef: null,
  policyCandidateAction: AppGameCategoryPolicyCandidateAction.ManualReview,
} as const;

const assertKnownCatalogCategoryKeepsSourceConfidenceAndEvidence = () => {
  const parsed = AppGameCategoryCandidateSchema.safeParse(KnownCatalogCategoryCandidate);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.sourceKind).toBe('catalog');
    expect(parsed.data.confidence).toBe(0.97);
    expect(parsed.data.evidence[0]?.evidenceId).toBe('evidence-app-category-catalog-1');
    expect(parsed.data.enforcementState).toBe('notEnforcement');
  }
};

const assertRejectsMissingEvidenceAndInvalidConfidence = () => {
  const missingEvidence = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    evidence: [],
  });
  const invalidConfidence = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    confidence: 1.1,
  });

  expect(missingEvidence.success).toBe(false);
  expect(invalidConfidence.success).toBe(false);
};

const assertUnknownVpnNameStaysCandidate = () => {
  const parsed = AppGameCategoryCandidateSchema.safeParse(UnknownVpnNamedCandidate);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.identityId).toBe(null);
    expect(parsed.data.candidateState).toBe('nameHeuristicCandidate');
    expect(parsed.data.categoryFamily).toBe('riskCandidate');
    expect(parsed.data.enforcementState).toBe('notEnforcement');
  }
};

const assertRiskLabelCannotBecomeEnforcementDecision = () => {
  const directBlock = AppGameCategoryCandidateSchema.safeParse({
    ...UnknownVpnNamedCandidate,
    policyCandidateAction: 'blockLaunch',
  });
  const executableState = AppGameCategoryCandidateSchema.safeParse({
    ...UnknownVpnNamedCandidate,
    enforcementState: 'terminateProcess',
  });

  expect(directBlock.success).toBe(false);
  expect(executableState.success).toBe(false);
};

const assertParentOverrideChangesDisplayOnly = () => {
  const parsed = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    candidateId: 'category-candidate-parent-label',
    sourceKind: AppGameCategorySourceKind.ParentLabel,
    sourceRef: 'source-parent-label',
    candidateState: AppGameCategoryCandidateState.ParentDisplayOverride,
    nativeAppCategory: AppGameNativeAppCategory.Productivity,
    reasonCode: 'parent-labels-homework-app',
    parentOverride: {
      parentLabel: 'Homework',
      displayFamily: AppGameCategoryFamily.NativeApp,
      displayNativeAppCategory: AppGameNativeAppCategory.Productivity,
      displayNativeGameCategory: null,
      displayRiskSignal: null,
      displayContextSignal: null,
      policyCandidateAction: AppGameCategoryPolicyCandidateAction.AskParent,
      rawIdentityChanged: false,
    },
    policyCandidateAction: AppGameCategoryPolicyCandidateAction.AskParent,
  });
  const rawIdentityMutation = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    candidateId: 'category-candidate-bad-parent-label',
    sourceKind: AppGameCategorySourceKind.ParentLabel,
    sourceRef: 'source-parent-label-bad',
    candidateState: AppGameCategoryCandidateState.ParentDisplayOverride,
    parentOverride: {
      parentLabel: 'Homework',
      displayFamily: AppGameCategoryFamily.NativeApp,
      displayNativeAppCategory: AppGameNativeAppCategory.Productivity,
      displayNativeGameCategory: null,
      displayRiskSignal: null,
      displayContextSignal: null,
      policyCandidateAction: AppGameCategoryPolicyCandidateAction.AskParent,
      rawIdentityChanged: true,
    },
  });

  expect(parsed.success).toBe(true);
  expect(rawIdentityMutation.success).toBe(false);
  if (parsed.success) {
    expect(parsed.data.parentOverride?.rawIdentityChanged).toBe(false);
    expect(parsed.data.identityId).toBe('identity-school-app');
  }
};

const assertAiCategoryCannotDirectlyAct = () => {
  const parsed = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    candidateId: 'category-candidate-local-ai-social',
    categoryFamily: AppGameCategoryFamily.NativeApp,
    nativeAppCategory: AppGameNativeAppCategory.Social,
    sourceKind: AppGameCategorySourceKind.LocalAi,
    sourceRef: 'source-local-ai-taxonomy',
    candidateState: AppGameCategoryCandidateState.AiCandidate,
    confidence: 0.76,
    reasonCode: 'local-ai-social-candidate',
    evidence: [LocalAiEvidence],
    catalogRef: null,
    aiDigestRef: 'ai-digest-social-candidate',
    policyCandidateAction: AppGameCategoryPolicyCandidateAction.Warn,
  });
  const missingDigest = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    candidateId: 'category-candidate-local-ai-missing-digest',
    sourceKind: AppGameCategorySourceKind.LocalAi,
    sourceRef: 'source-local-ai-taxonomy-missing-digest',
    candidateState: AppGameCategoryCandidateState.AiCandidate,
    reasonCode: 'local-ai-without-digest',
    aiDigestRef: null,
  });
  const directBlock = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    candidateId: 'category-candidate-local-ai-block',
    sourceKind: AppGameCategorySourceKind.LocalAi,
    sourceRef: 'source-local-ai-taxonomy-block',
    candidateState: AppGameCategoryCandidateState.AiCandidate,
    reasonCode: 'local-ai-direct-block',
    aiDigestRef: 'ai-digest-block-candidate',
    policyCandidateAction: 'shieldApp',
  });

  expect(parsed.success).toBe(true);
  expect(missingDigest.success).toBe(false);
  expect(directBlock.success).toBe(false);
};

const assertGameContextSignalsRemainPolicyInputs = () => {
  const parsed = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    candidateId: 'category-candidate-game-multiplayer',
    productKind: AppGameProductKind.NativeGame,
    categoryFamily: AppGameCategoryFamily.GameContext,
    nativeAppCategory: null,
    nativeGameCategory: null,
    riskSignal: null,
    gameContextSignal: AppGameContextSignalKind.Multiplayer,
    sourceKind: AppGameCategorySourceKind.LauncherManifest,
    sourceRef: 'source-launcher-context',
    candidateState: AppGameCategoryCandidateState.CatalogCandidate,
    confidence: 0.88,
    reasonCode: 'launcher-multiplayer-context',
    policyCandidateAction: AppGameCategoryPolicyCandidateAction.AskParent,
  });
  const gameCategory = AppGameCategoryCandidateSchema.safeParse({
    ...KnownCatalogCategoryCandidate,
    candidateId: 'category-candidate-game-puzzle',
    productKind: AppGameProductKind.NativeGame,
    categoryFamily: AppGameCategoryFamily.NativeGame,
    nativeAppCategory: null,
    nativeGameCategory: AppGameNativeGameCategory.Puzzle,
    sourceKind: AppGameCategorySourceKind.Catalog,
    sourceRef: 'source-game-catalog',
    candidateState: AppGameCategoryCandidateState.CatalogCandidate,
    confidence: 0.91,
    reasonCode: 'catalog-game-puzzle',
    catalogRef: 'catalog-puzzle-game',
    policyCandidateAction: AppGameCategoryPolicyCandidateAction.Observe,
  });

  expect(parsed.success).toBe(true);
  expect(gameCategory.success).toBe(true);
  if (parsed.success && gameCategory.success) {
    expect(parsed.data.gameContextSignal).toBe('multiplayer');
    expect(gameCategory.data.nativeGameCategory).toBe('puzzle');
    expect(parsed.data.enforcementState).toBe('notEnforcement');
  }
};

const assertCandidateSetRejectsDuplicateIds = () => {
  const parsed = AppGameCategoryCandidateSetSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    candidates: [KnownCatalogCategoryCandidate, KnownCatalogCategoryCandidate],
  });

  expect(parsed.success).toBe(false);
};

describe('app game category risk taxonomy contracts', () => {
  it(
    'AppGameCategoryCandidateSchema: keeps known catalog category source confidence and evidence',
    assertKnownCatalogCategoryKeepsSourceConfidenceAndEvidence
  );
  it(
    'AppGameCategoryCandidateSchema: rejects missing evidence and invalid confidence',
    assertRejectsMissingEvidenceAndInvalidConfidence
  );
  it('AppGameCategoryCandidateSchema: keeps unknown vpn-like names as candidates', assertUnknownVpnNameStaysCandidate);
  it(
    'AppGameCategoryCandidateSchema: rejects risk labels as enforcement decisions',
    assertRiskLabelCannotBecomeEnforcementDecision
  );
  it('AppGameCategoryCandidateSchema: keeps parent overrides display-only', assertParentOverrideChangesDisplayOnly);
  it('AppGameCategoryCandidateSchema: keeps AI category output review-only', assertAiCategoryCannotDirectlyAct);
  it(
    'AppGameCategoryCandidateSchema: accepts game category and context signals as policy inputs',
    assertGameContextSignalsRemainPolicyInputs
  );
  it('AppGameCategoryCandidateSetSchema: rejects duplicate candidate ids', assertCandidateSetRejectsDuplicateIds);
});
