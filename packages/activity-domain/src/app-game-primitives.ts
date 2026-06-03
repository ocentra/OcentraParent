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
export const AppGameEvidenceClaimIdSchema = NonEmptyAppGameText.pipe(Schema.brand('AppGameEvidenceClaimId'));
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
  Schema.Literal(
    'available',
    'unavailable',
    'permissionLimited',
    'unsupportedPlatform',
    'adapterError',
    'stale',
    'degraded',
    'manualRequired',
    'notClaimed'
  )
);

export const AppGameForegroundStateSchema = withParser(
  Schema.Literal(
    'foreground',
    'background',
    'notWindowed',
    'notClaimed',
    'unknown',
    'unsupported',
    'permissionLimited',
    'degraded',
    'adapterError'
  )
);

export const AppGameRuntimeStateSchema = withParser(
  Schema.Literal(
    'running',
    'notRunning',
    'notClaimed',
    'unknown',
    'permissionLimited',
    'unavailable',
    'degraded',
    'stale',
    'adapterError'
  )
);

export const AppGameEvidenceClaimKindSchema = withParser(
  Schema.Literal('inventory', 'runtime', 'foreground', 'launcher', 'session', 'catalog', 'aiDigest')
);

export const AppGameIdentityStrengthSchema = withParser(
  Schema.Literal(
    'displayNameOnly',
    'weak',
    'observedProcess',
    'catalogMatched',
    'launcherClaimed',
    'platformManaged',
    'childGameProof'
  )
);

export const AppGameAiActionHintSchema = withParser(
  Schema.Literal(
    'classifyOnly',
    'summarizeEvidence',
    'parentReview',
    'policyDraftPreview',
    'askParentPreview',
    'markUnavailable'
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

export const AppGameCapabilityStatus = {
  Available: AppGameCapabilityStatusSchema.parse('available'),
  Unavailable: AppGameCapabilityStatusSchema.parse('unavailable'),
  PermissionLimited: AppGameCapabilityStatusSchema.parse('permissionLimited'),
  UnsupportedPlatform: AppGameCapabilityStatusSchema.parse('unsupportedPlatform'),
  AdapterError: AppGameCapabilityStatusSchema.parse('adapterError'),
  Stale: AppGameCapabilityStatusSchema.parse('stale'),
  Degraded: AppGameCapabilityStatusSchema.parse('degraded'),
  ManualRequired: AppGameCapabilityStatusSchema.parse('manualRequired'),
  NotClaimed: AppGameCapabilityStatusSchema.parse('notClaimed'),
} as const;

export const AppGameForegroundState = {
  Foreground: AppGameForegroundStateSchema.parse('foreground'),
  Background: AppGameForegroundStateSchema.parse('background'),
  NotWindowed: AppGameForegroundStateSchema.parse('notWindowed'),
  NotClaimed: AppGameForegroundStateSchema.parse('notClaimed'),
  Unknown: AppGameForegroundStateSchema.parse('unknown'),
  Unsupported: AppGameForegroundStateSchema.parse('unsupported'),
  PermissionLimited: AppGameForegroundStateSchema.parse('permissionLimited'),
  Degraded: AppGameForegroundStateSchema.parse('degraded'),
  AdapterError: AppGameForegroundStateSchema.parse('adapterError'),
} as const;

export const AppGameRuntimeState = {
  Running: AppGameRuntimeStateSchema.parse('running'),
  NotRunning: AppGameRuntimeStateSchema.parse('notRunning'),
  NotClaimed: AppGameRuntimeStateSchema.parse('notClaimed'),
  Unknown: AppGameRuntimeStateSchema.parse('unknown'),
  PermissionLimited: AppGameRuntimeStateSchema.parse('permissionLimited'),
  Unavailable: AppGameRuntimeStateSchema.parse('unavailable'),
  Degraded: AppGameRuntimeStateSchema.parse('degraded'),
  Stale: AppGameRuntimeStateSchema.parse('stale'),
  AdapterError: AppGameRuntimeStateSchema.parse('adapterError'),
} as const;

export const AppGameEvidenceClaimKind = {
  Inventory: AppGameEvidenceClaimKindSchema.parse('inventory'),
  Runtime: AppGameEvidenceClaimKindSchema.parse('runtime'),
  Foreground: AppGameEvidenceClaimKindSchema.parse('foreground'),
  Launcher: AppGameEvidenceClaimKindSchema.parse('launcher'),
  Session: AppGameEvidenceClaimKindSchema.parse('session'),
  Catalog: AppGameEvidenceClaimKindSchema.parse('catalog'),
  AiDigest: AppGameEvidenceClaimKindSchema.parse('aiDigest'),
} as const;

export const AppGameIdentityStrength = {
  DisplayNameOnly: AppGameIdentityStrengthSchema.parse('displayNameOnly'),
  Weak: AppGameIdentityStrengthSchema.parse('weak'),
  ObservedProcess: AppGameIdentityStrengthSchema.parse('observedProcess'),
  CatalogMatched: AppGameIdentityStrengthSchema.parse('catalogMatched'),
  LauncherClaimed: AppGameIdentityStrengthSchema.parse('launcherClaimed'),
  PlatformManaged: AppGameIdentityStrengthSchema.parse('platformManaged'),
  ChildGameProof: AppGameIdentityStrengthSchema.parse('childGameProof'),
} as const;

export const AppGameAiActionHint = {
  ClassifyOnly: AppGameAiActionHintSchema.parse('classifyOnly'),
  SummarizeEvidence: AppGameAiActionHintSchema.parse('summarizeEvidence'),
  ParentReview: AppGameAiActionHintSchema.parse('parentReview'),
  PolicyDraftPreview: AppGameAiActionHintSchema.parse('policyDraftPreview'),
  AskParentPreview: AppGameAiActionHintSchema.parse('askParentPreview'),
  MarkUnavailable: AppGameAiActionHintSchema.parse('markUnavailable'),
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
export type AppGameRuntimeState = Infer<typeof AppGameRuntimeStateSchema>;
export type AppGameEvidenceClaimKind = Infer<typeof AppGameEvidenceClaimKindSchema>;
export type AppGameIdentityStrength = Infer<typeof AppGameIdentityStrengthSchema>;
export type AppGameAiActionHint = Infer<typeof AppGameAiActionHintSchema>;
export type AppGameObservationMode = Infer<typeof AppGameObservationModeSchema>;
export type AppGameLauncherKind = Infer<typeof AppGameLauncherKindSchema>;
