import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { AppGameLauncherAppIdSchema, AppGameLauncherManifestIdSchema } from './app-game-identity-primitives';
import {
  AppGameCapabilityStatusSchema,
  AppGameCatalogReadyStateSchema,
  AppGameCatalogRefSchema,
  AppGameClassificationStateSchema,
  AppGameConfidenceSchema,
  AppGameEvidenceClaimIdSchema,
  AppGameForegroundStateSchema,
  AppGameInventoryEntryIdSchema,
  AppGameLauncherKindSchema,
  AppGameLauncherRefSchema,
  AppGameNonNegativeCountSchema,
  AppGameObservationModeSchema,
  AppGameProcessIdentitySchema,
  AppGameProcessNameSchema,
  AppGameRuntimeStateSchema,
  AppGameSchemaVersion,
} from './app-game-primitives';

const AppGameLauncherEvidenceIdSchema = brandedNonEmptyStringSchema('AppGameLauncherEvidenceId');

export const AppGameLauncherGameProofStateSchema = withParser(
  Schema.Literal(
    'launcherOnly',
    'launcherManifestCandidate',
    'childProcessCandidate',
    'deterministicChildGame',
    'classifierBackedChildGame',
    'permissionLimited',
    'adapterError',
    'notClaimed'
  )
);

const AppGameLauncherEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  launcherEvidenceId: AppGameLauncherEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  launcherKind: AppGameLauncherKindSchema,
  launcherRef: AppGameLauncherRefSchema,
  launcherInventoryEntryId: Schema.Union(AppGameInventoryEntryIdSchema, Schema.Null),
  launcherManifestId: Schema.Union(AppGameLauncherManifestIdSchema, Schema.Null),
  launcherAppId: Schema.Union(AppGameLauncherAppIdSchema, Schema.Null),
  launcherProcessIdentity: Schema.Union(AppGameProcessIdentitySchema, Schema.Null),
  launcherProcessId: Schema.Union(AppGameNonNegativeCountSchema, Schema.Null),
  launcherProcessName: Schema.Union(AppGameProcessNameSchema, Schema.Null),
  childProcessIdentity: Schema.Union(AppGameProcessIdentitySchema, Schema.Null),
  childInventoryEntryId: Schema.Union(AppGameInventoryEntryIdSchema, Schema.Null),
  childGameEvidenceClaimId: Schema.Union(AppGameEvidenceClaimIdSchema, Schema.Null),
  catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
  runtimeState: AppGameRuntimeStateSchema,
  foregroundState: AppGameForegroundStateSchema,
  observationMode: AppGameObservationModeSchema,
  classificationState: AppGameClassificationStateSchema,
  catalogReadyState: AppGameCatalogReadyStateSchema,
  capabilityStatus: AppGameCapabilityStatusSchema,
  gameProofState: AppGameLauncherGameProofStateSchema,
  confidence: AppGameConfidenceSchema,
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const AppGameLauncherEvidenceSchema = withParser(
  AppGameLauncherEvidenceBaseSchema.pipe(
    Schema.filter(
      (launcher) => launcher.launcherRef.length > 0 || 'Expected launcher evidence to cite a launcher reference'
    )
  )
    .pipe(
      Schema.filter(
        (launcher) =>
          launcher.classificationState !== 'knownGame' ||
          launcherHasChildGameProof(launcher) ||
          'Expected launcher evidence to require child-game proof before known-game classification'
      )
    )
    .pipe(
      Schema.filter(
        (launcher) =>
          !launcherIsCandidateOnly(launcher) ||
          launcher.classificationState === 'launcherGameCandidate' ||
          'Expected launcher-game candidates to stay candidate instead of known game'
      )
    )
    .pipe(
      Schema.filter(
        (launcher) =>
          launcher.gameProofState !== 'launcherOnly' ||
          launcher.classificationState === 'knownLauncher' ||
          'Expected launcher-only evidence to stay known-launcher'
      )
    )
    .pipe(
      Schema.filter(
        (launcher) =>
          !launcherIsPermissionLimited(launcher) ||
          (launcher.capabilityStatus === 'permissionLimited' &&
            launcher.classificationState === 'permissionLimited' &&
            launcher.confidence === 0) ||
          'Expected permission-limited launcher evidence to avoid confident classification'
      )
    )
);

export const AppGameLauncherGameProofState = {
  LauncherOnly: AppGameLauncherGameProofStateSchema.parse('launcherOnly'),
  LauncherManifestCandidate: AppGameLauncherGameProofStateSchema.parse('launcherManifestCandidate'),
  ChildProcessCandidate: AppGameLauncherGameProofStateSchema.parse('childProcessCandidate'),
  DeterministicChildGame: AppGameLauncherGameProofStateSchema.parse('deterministicChildGame'),
  ClassifierBackedChildGame: AppGameLauncherGameProofStateSchema.parse('classifierBackedChildGame'),
  PermissionLimited: AppGameLauncherGameProofStateSchema.parse('permissionLimited'),
  AdapterError: AppGameLauncherGameProofStateSchema.parse('adapterError'),
  NotClaimed: AppGameLauncherGameProofStateSchema.parse('notClaimed'),
} as const;

export type AppGameLauncherEvidence = Infer<typeof AppGameLauncherEvidenceSchema>;
export type AppGameLauncherGameProofState = Infer<typeof AppGameLauncherGameProofStateSchema>;

function launcherHasChildGameProof(launcher: Infer<typeof AppGameLauncherEvidenceBaseSchema>): boolean {
  return (
    (launcher.gameProofState === 'deterministicChildGame' || launcher.gameProofState === 'classifierBackedChildGame') &&
    launcher.childGameEvidenceClaimId !== null
  );
}

function launcherIsCandidateOnly(launcher: Infer<typeof AppGameLauncherEvidenceBaseSchema>): boolean {
  return launcher.gameProofState === 'launcherManifestCandidate' || launcher.gameProofState === 'childProcessCandidate';
}

function launcherIsPermissionLimited(launcher: Infer<typeof AppGameLauncherEvidenceBaseSchema>): boolean {
  return launcher.gameProofState === 'permissionLimited';
}
