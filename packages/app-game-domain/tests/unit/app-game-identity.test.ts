import { describe, expect, it } from 'vitest';
import {
  AppGameClassificationState,
  AppGameIdentityConfidence,
  AppGameIdentityDeterministicRefKind,
  AppGameIdentityMergeProofSchema,
  AppGameIdentitySchema,
  AppGameProductKind,
  AppGameSchemaVersion,
} from '../../src/app-game';
import { ActivityEvidenceKind } from '@ocentra-parent/evidence-domain/kinds';

const JournalEvidence = {
  evidenceId: 'journal-entry-app-game-identity-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:app-game-identity-digest',
  uri: null,
} as const;

const DeterministicGameIdentity = {
  schemaVersion: AppGameSchemaVersion,
  identityId: 'identity-elden-ring-game',
  productKind: AppGameProductKind.NativeGame,
  displayLabel: 'Elden Ring',
  parentLabel: null,
  confidence: AppGameIdentityConfidence.Deterministic,
  classificationState: AppGameClassificationState.KnownGame,
  packageId: null,
  bundleId: null,
  appUserModelId: null,
  desktopEntryId: null,
  applicationTokenRef: null,
  executablePathRef: 'path-ref-elden-ring',
  publisherSignatureRef: 'publisher-ref-fromsoftware',
  fileHashRef: 'hash-ref-elden-ring',
  launcherRef: 'launcher-steam',
  launcherAppId: 'steam-app-1245620',
  launcherManifestId: 'steam-manifest-1245620',
  storeId: 'steam-store-1245620',
  catalogRef: 'catalog-steam-1245620',
  childGameEvidenceClaimId: 'claim-child-game-proof-1',
  evidence: [JournalEvidence],
} as const;

const LauncherIdentity = {
  schemaVersion: AppGameSchemaVersion,
  identityId: 'identity-steam-launcher',
  productKind: AppGameProductKind.Launcher,
  displayLabel: 'Steam',
  parentLabel: null,
  confidence: AppGameIdentityConfidence.Deterministic,
  classificationState: AppGameClassificationState.KnownLauncher,
  packageId: null,
  bundleId: null,
  appUserModelId: null,
  desktopEntryId: null,
  applicationTokenRef: null,
  executablePathRef: null,
  publisherSignatureRef: null,
  fileHashRef: null,
  launcherRef: 'launcher-steam',
  launcherAppId: 'steam-client',
  launcherManifestId: 'steam-library-root',
  storeId: null,
  catalogRef: null,
  childGameEvidenceClaimId: null,
  evidence: [JournalEvidence],
} as const;

const DisplayOnlyIdentity = {
  ...DeterministicGameIdentity,
  identityId: 'identity-display-only',
  productKind: AppGameProductKind.UnknownExecutable,
  displayLabel: 'Mystery Game',
  confidence: AppGameIdentityConfidence.Weak,
  classificationState: AppGameClassificationState.UnknownProcess,
  executablePathRef: null,
  publisherSignatureRef: null,
  fileHashRef: null,
  launcherRef: null,
  launcherAppId: null,
  launcherManifestId: null,
  storeId: null,
  catalogRef: null,
  childGameEvidenceClaimId: null,
} as const;

const assertAcceptsDeterministicGameIdentity = () => {
  const parsed = AppGameIdentitySchema.safeParse(DeterministicGameIdentity);

  expect(parsed.success).toBe(true);
  if (parsed.success) {
    expect(parsed.data.classificationState).toBe('knownGame');
    expect(parsed.data.childGameEvidenceClaimId).toBe('claim-child-game-proof-1');
  }
};

const assertKeepsDisplayOnlyIdentityWeak = () => {
  const parsed = AppGameIdentitySchema.safeParse(DisplayOnlyIdentity);
  const promoted = AppGameIdentitySchema.safeParse({
    ...DisplayOnlyIdentity,
    confidence: AppGameIdentityConfidence.Deterministic,
    classificationState: AppGameClassificationState.KnownGame,
  });

  expect(parsed.success).toBe(true);
  expect(promoted.success).toBe(false);
};

const assertKeepsLauncherSeparateFromGame = () => {
  const launcherGameClaim = {
    ...LauncherIdentity,
    identityId: 'identity-launcher-as-game',
    productKind: AppGameProductKind.NativeGame,
    classificationState: AppGameClassificationState.KnownGame,
  } as const;
  const launcher = AppGameIdentitySchema.safeParse(LauncherIdentity);
  const launcherAsGame = AppGameIdentitySchema.safeParse(launcherGameClaim);
  const launcherWithChildProof = AppGameIdentitySchema.safeParse({
    ...launcherGameClaim,
    executablePathRef: 'path-ref-child-game',
    childGameEvidenceClaimId: 'claim-child-game-proof-2',
  });

  expect(launcher.success).toBe(true);
  expect(launcherAsGame.success).toBe(false);
  expect(launcherWithChildProof.success).toBe(true);
};

const assertRejectsDisplayOnlyMerge = () => {
  const parsed = AppGameIdentityMergeProofSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    mergeId: 'identity-merge-display-only',
    targetIdentity: DeterministicGameIdentity,
    sourceIdentityIds: ['identity-left', 'identity-right'],
    mergeConfidence: 0.8,
    displayLabelMatched: true,
    parentLabelChanged: false,
    conflictingFileHashRefs: false,
    sharedDeterministicRefs: [],
    evidence: [JournalEvidence],
  });

  expect(parsed.success).toBe(false);
};

const assertStoreAndProcessMergeThroughDeterministicRefs = () => {
  const parsed = AppGameIdentityMergeProofSchema.safeParse({
    schemaVersion: AppGameSchemaVersion,
    mergeId: 'identity-merge-store-process',
    targetIdentity: DeterministicGameIdentity,
    sourceIdentityIds: ['identity-store-package', 'identity-process'],
    mergeConfidence: 0.92,
    displayLabelMatched: true,
    parentLabelChanged: false,
    conflictingFileHashRefs: false,
    sharedDeterministicRefs: [
      AppGameIdentityDeterministicRefKind.StoreId,
      AppGameIdentityDeterministicRefKind.FileHashRef,
    ],
    evidence: [JournalEvidence],
  });

  expect(parsed.success).toBe(true);
};

const assertParentLabelDoesNotCreateIdentity = () => {
  const labeledMerge = {
    schemaVersion: AppGameSchemaVersion,
    mergeId: 'identity-merge-parent-label',
    targetIdentity: {
      ...DeterministicGameIdentity,
      parentLabel: 'Weekend RPG',
      displayLabel: 'Weekend RPG',
      confidence: AppGameIdentityConfidence.ParentLabeled,
    },
    sourceIdentityIds: ['identity-before-label', 'identity-after-label'],
    mergeConfidence: 0.9,
    displayLabelMatched: false,
    parentLabelChanged: true,
    conflictingFileHashRefs: false,
    sharedDeterministicRefs: [AppGameIdentityDeterministicRefKind.FileHashRef],
    evidence: [JournalEvidence],
  } as const;
  const labeled = AppGameIdentityMergeProofSchema.safeParse(labeledMerge);
  const labelOnly = AppGameIdentityMergeProofSchema.safeParse({
    ...labeledMerge,
    mergeId: 'identity-merge-label-only',
    sharedDeterministicRefs: [],
  });

  expect(labeled.success).toBe(true);
  expect(labelOnly.success).toBe(false);
};

describe('app game identity contracts', () => {
  it('accepts deterministic game identity with child-game proof', assertAcceptsDeterministicGameIdentity);
  it('keeps display-label-only identity weak and unknown', assertKeepsDisplayOnlyIdentityWeak);
  it('keeps launcher identity separate from child game identity', assertKeepsLauncherSeparateFromGame);
  it('rejects display-label-only identity merges', assertRejectsDisplayOnlyMerge);
  it(
    'requires deterministic refs for store-package and process merge',
    assertStoreAndProcessMergeThroughDeterministicRefs
  );
  it('keeps parent labels from becoming raw identity', assertParentLabelDoesNotCreateIdentity);
});
