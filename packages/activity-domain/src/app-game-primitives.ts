import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const AppGameSchemaVersion = 1;

const NonEmptyAppGameText = Schema.String.pipe(Schema.minLength(1));

export const AppGameNonNegativeDurationSchema = Schema.Number.pipe(
  Schema.filter((value) => value >= 0 || 'Expected a non-negative duration')
);

export const AppGameNonNegativeCountSchema = Schema.Number.pipe(
  Schema.filter((value) => value >= 0 || 'Expected a non-negative count')
);

export const AppGameInventoryEntryIdSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameInventoryEntryId'));
export const AppGameProcessIdentitySchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameProcessIdentity'));
export const AppGameProcessNameSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameProcessName'));
export const AppGameDisplayNameSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameDisplayName'));
export const AppGameExecutablePathSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameExecutablePath'));
export const AppGameSessionIdSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameSessionId'));
export const AppGameCatalogRefSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameCatalogRef'));
export const AppGameLauncherRefSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameLauncherRef'));
export const AppGameAiDigestRefSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameAiDigestRef'));
export const AppGameUnavailableReasonSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameUnavailableReason'));

export const AppGameConfidenceSchema = withParser(Schema.Number.pipe(Schema.between(0, 1)));

export const AppGameClassificationStateSchema = withParser(
  Schema.Literal(
    'knownApp',
    'knownGame',
    'knownLauncher',
    'launcherGameCandidate',
    'possiblyGame',
    'unknownProcess',
    'permissionLimited',
    'unsupportedPlatform',
    'stale',
    'adapterError'
  )
);

export const AppGameCatalogReadyStateSchema = withParser(
  Schema.Literal('catalogReady', 'catalogUnavailable', 'catalogNotLoaded', 'catalogStale', 'permissionLimited')
);

export const AppGameCapabilityStatusSchema = withParser(
  Schema.Literal('available', 'unavailable', 'permissionLimited', 'unsupportedPlatform', 'adapterError', 'stale')
);

export const AppGameForegroundStateSchema = withParser(
  Schema.Literal(
    'foreground',
    'background',
    'notWindowed',
    'unknown',
    'unsupported',
    'permissionLimited',
    'adapterError'
  )
);

export const AppGameObservationModeSchema = withParser(
  Schema.Literal(
    'processSnapshot',
    'foregroundWindow',
    'processStart',
    'processExit',
    'inventoryScan',
    'launcherManifest'
  )
);

export const AppGameLauncherKindSchema = withParser(
  Schema.Literal(
    'steam',
    'epicGames',
    'xbox',
    'riotClient',
    'battleNet',
    'eaApp',
    'ubisoftConnect',
    'gogGalaxy',
    'roblox',
    'minecraft',
    'unknownLauncher'
  )
);

export const AppGameClassificationState = {
  KnownApp: AppGameClassificationStateSchema.parse('knownApp'),
  KnownGame: AppGameClassificationStateSchema.parse('knownGame'),
  KnownLauncher: AppGameClassificationStateSchema.parse('knownLauncher'),
  LauncherGameCandidate: AppGameClassificationStateSchema.parse('launcherGameCandidate'),
  PossiblyGame: AppGameClassificationStateSchema.parse('possiblyGame'),
  UnknownProcess: AppGameClassificationStateSchema.parse('unknownProcess'),
  PermissionLimited: AppGameClassificationStateSchema.parse('permissionLimited'),
  UnsupportedPlatform: AppGameClassificationStateSchema.parse('unsupportedPlatform'),
  Stale: AppGameClassificationStateSchema.parse('stale'),
  AdapterError: AppGameClassificationStateSchema.parse('adapterError'),
} as const;

export const AppGameCatalogReadyState = {
  CatalogReady: AppGameCatalogReadyStateSchema.parse('catalogReady'),
  CatalogUnavailable: AppGameCatalogReadyStateSchema.parse('catalogUnavailable'),
  CatalogNotLoaded: AppGameCatalogReadyStateSchema.parse('catalogNotLoaded'),
  CatalogStale: AppGameCatalogReadyStateSchema.parse('catalogStale'),
  PermissionLimited: AppGameCatalogReadyStateSchema.parse('permissionLimited'),
} as const;

export const AppGameForegroundState = {
  Foreground: AppGameForegroundStateSchema.parse('foreground'),
  Background: AppGameForegroundStateSchema.parse('background'),
  NotWindowed: AppGameForegroundStateSchema.parse('notWindowed'),
  Unknown: AppGameForegroundStateSchema.parse('unknown'),
  Unsupported: AppGameForegroundStateSchema.parse('unsupported'),
  PermissionLimited: AppGameForegroundStateSchema.parse('permissionLimited'),
  AdapterError: AppGameForegroundStateSchema.parse('adapterError'),
} as const;

export const AppGameObservationMode = {
  ProcessSnapshot: AppGameObservationModeSchema.parse('processSnapshot'),
  ForegroundWindow: AppGameObservationModeSchema.parse('foregroundWindow'),
  ProcessStart: AppGameObservationModeSchema.parse('processStart'),
  ProcessExit: AppGameObservationModeSchema.parse('processExit'),
  InventoryScan: AppGameObservationModeSchema.parse('inventoryScan'),
  LauncherManifest: AppGameObservationModeSchema.parse('launcherManifest'),
} as const;

export type AppGameClassificationState = Infer<typeof AppGameClassificationStateSchema>;
export type AppGameCatalogReadyState = Infer<typeof AppGameCatalogReadyStateSchema>;
export type AppGameCapabilityStatus = Infer<typeof AppGameCapabilityStatusSchema>;
export type AppGameForegroundState = Infer<typeof AppGameForegroundStateSchema>;
export type AppGameObservationMode = Infer<typeof AppGameObservationModeSchema>;
export type AppGameLauncherKind = Infer<typeof AppGameLauncherKindSchema>;
