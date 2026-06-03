import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
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
import { ActivityTimestampSchema } from './primitives';

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

function appGameInventoryHasNoUseClaim(row: Infer<typeof AppGameInventoryEvidenceRowBaseSchema>): boolean {
  return (
    row.runtimeState === 'notClaimed' &&
    row.foregroundState === 'notClaimed' &&
    row.runningDurationMs === 0 &&
    row.foregroundDurationMs === 0
  );
}

function appGameInventorySourceHasRequiredReference(row: Infer<typeof AppGameInventoryEvidenceRowBaseSchema>): boolean {
  switch (row.sourceKind) {
    case 'osInstalledRecord':
      return row.executablePathRef !== null || row.packageId !== null || row.appUserModelId !== null;
    case 'shortcut':
      return row.desktopEntryId !== null || row.executablePathRef !== null;
    case 'storePackage':
      return row.packageId !== null || row.bundleId !== null || row.appUserModelId !== null || row.storeId !== null;
    case 'launcherManifest':
      return row.launcherRef !== null || row.launcherAppId !== null || row.launcherManifestId !== null;
    case 'parentCatalog':
      return row.catalogRef !== null;
    case 'managedDevice':
      return row.identityId !== null || row.packageId !== null || row.bundleId !== null || row.appUserModelId !== null;
    case 'portableApp':
      return row.executablePathRef !== null;
    case 'unknownSource':
      return row.inventoryState !== 'installed' && row.confidence <= 0.3;
  }
}

function appGameInventoryHighConfidenceHasIdentityRef(
  row: Infer<typeof AppGameInventoryEvidenceRowBaseSchema>
): boolean {
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
