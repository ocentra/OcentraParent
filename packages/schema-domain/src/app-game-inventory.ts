import { type Infer, Schema, withParser } from './effect';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import {
  AppGameAppUserModelIdSchema,
  AppGameBundleIdSchema,
  AppGameDesktopEntryIdSchema,
  AppGameDisplayLabelSchema,
  AppGameExecutablePathRefSchema,
  AppGameIdentityIdSchema,
  AppGameLauncherAppIdSchema,
  AppGameLauncherManifestIdSchema,
  AppGamePackageIdSchema,
  AppGameProductKindSchema,
  AppGameStoreIdSchema,
} from './app-game-identity-primitives';
import {
  AppGameInventoryCategoryKindSchema,
  AppGameInventoryCustodyStateSchema,
  AppGameInventoryDetectionStateSchema,
  AppGameInventorySourceKindSchema,
  AppGameInventorySourceRefSchema,
} from './app-game-inventory-primitives';
import {
  AppGameCapabilityStatusSchema,
  AppGameCatalogReadyStateSchema,
  AppGameCatalogRefSchema,
  AppGameClassificationStateSchema,
  AppGameConfidenceSchema,
  AppGameForegroundStateSchema,
  AppGameInventoryEntryIdSchema,
  AppGameLauncherRefSchema,
  AppGameNonNegativeDurationSchema,
  AppGameRuntimeStateSchema,
  AppGameSchemaVersion,
} from './app-game-primitives';
import { ActivityTimestampSchema } from './evidence-primitives';

export const AppGameInventoryCategoryCandidateSchema = withParser(
  Schema.Struct({
    categoryKind: AppGameInventoryCategoryKindSchema,
    confidence: AppGameConfidenceSchema,
    catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

const AppGameInventoryEvidenceRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  inventoryEntryId: AppGameInventoryEntryIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceKind: AppGameInventorySourceKindSchema,
  sourceRef: AppGameInventorySourceRefSchema,
  custodyState: AppGameInventoryCustodyStateSchema,
  productKind: AppGameProductKindSchema,
  displayLabel: AppGameDisplayLabelSchema,
  identityId: Schema.Union(AppGameIdentityIdSchema, Schema.Null),
  packageId: Schema.Union(AppGamePackageIdSchema, Schema.Null),
  bundleId: Schema.Union(AppGameBundleIdSchema, Schema.Null),
  appUserModelId: Schema.Union(AppGameAppUserModelIdSchema, Schema.Null),
  desktopEntryId: Schema.Union(AppGameDesktopEntryIdSchema, Schema.Null),
  executablePathRef: Schema.Union(AppGameExecutablePathRefSchema, Schema.Null),
  launcherRef: Schema.Union(AppGameLauncherRefSchema, Schema.Null),
  launcherAppId: Schema.Union(AppGameLauncherAppIdSchema, Schema.Null),
  launcherManifestId: Schema.Union(AppGameLauncherManifestIdSchema, Schema.Null),
  storeId: Schema.Union(AppGameStoreIdSchema, Schema.Null),
  catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
  inventoryState: AppGameInventoryDetectionStateSchema,
  classificationState: AppGameClassificationStateSchema,
  catalogReadyState: AppGameCatalogReadyStateSchema,
  capabilityStatus: AppGameCapabilityStatusSchema,
  confidence: AppGameConfidenceSchema,
  categoryCandidates: Schema.Array(AppGameInventoryCategoryCandidateSchema),
  runtimeState: AppGameRuntimeStateSchema,
  foregroundState: AppGameForegroundStateSchema,
  runningDurationMs: AppGameNonNegativeDurationSchema,
  foregroundDurationMs: AppGameNonNegativeDurationSchema,
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

type AppGameInventoryEvidenceRowBase = Infer<typeof AppGameInventoryEvidenceRowBaseSchema>;
type AppGameInventorySourceKindValue = Infer<typeof AppGameInventorySourceKindSchema>;

export const AppGameInventoryEvidenceRowSchema = withParser(
  AppGameInventoryEvidenceRowBaseSchema.pipe(
    Schema.filter(
      (row) => appGameInventoryHasNoUseClaim(row) || 'Expected inventory evidence to avoid app/game use claims'
    )
  )
    .pipe(
      Schema.filter(
        (row) =>
          appGameInventorySourceHasRequiredReference(row) ||
          'Expected inventory source to cite a matching identity or source reference'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          appGameInventoryHighConfidenceHasIdentityRef(row) ||
          'Expected high-confidence inventory rows to include identity references'
      )
    )
);

export type AppGameInventoryCategoryCandidate = Infer<typeof AppGameInventoryCategoryCandidateSchema>;
export type AppGameInventoryEvidenceRow = Infer<typeof AppGameInventoryEvidenceRowSchema>;

const AppGameInventorySourceReferenceValidators = {
  osInstalledRecord: (row: AppGameInventoryEvidenceRowBase) =>
    row.executablePathRef !== null || row.packageId !== null || row.appUserModelId !== null,
  shortcut: (row: AppGameInventoryEvidenceRowBase) => row.desktopEntryId !== null || row.executablePathRef !== null,
  storePackage: (row: AppGameInventoryEvidenceRowBase) =>
    row.packageId !== null || row.bundleId !== null || row.appUserModelId !== null || row.storeId !== null,
  launcherManifest: (row: AppGameInventoryEvidenceRowBase) =>
    row.launcherRef !== null || row.launcherAppId !== null || row.launcherManifestId !== null,
  parentCatalog: (row: AppGameInventoryEvidenceRowBase) => row.catalogRef !== null,
  managedDevice: (row: AppGameInventoryEvidenceRowBase) =>
    row.identityId !== null || row.packageId !== null || row.bundleId !== null || row.appUserModelId !== null,
  portableApp: (row: AppGameInventoryEvidenceRowBase) => row.executablePathRef !== null,
  unknownSource: (row: AppGameInventoryEvidenceRowBase) => row.inventoryState !== 'installed' && row.confidence <= 0.3,
} satisfies Record<AppGameInventorySourceKindValue, (row: AppGameInventoryEvidenceRowBase) => boolean>;

function appGameInventoryHasNoUseClaim(row: AppGameInventoryEvidenceRowBase): boolean {
  return (
    row.runtimeState === 'notClaimed' &&
    row.foregroundState === 'notClaimed' &&
    row.runningDurationMs === 0 &&
    row.foregroundDurationMs === 0
  );
}

function appGameInventorySourceHasRequiredReference(row: AppGameInventoryEvidenceRowBase): boolean {
  return AppGameInventorySourceReferenceValidators[row.sourceKind](row);
}

function appGameInventoryHighConfidenceHasIdentityRef(row: AppGameInventoryEvidenceRowBase): boolean {
  return (
    row.confidence <= 0.3 ||
    row.identityId !== null ||
    row.packageId !== null ||
    row.bundleId !== null ||
    row.appUserModelId !== null ||
    row.desktopEntryId !== null ||
    row.executablePathRef !== null ||
    row.launcherAppId !== null ||
    row.launcherManifestId !== null ||
    row.storeId !== null ||
    row.catalogRef !== null
  );
}
