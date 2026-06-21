export const AppGameSourceFreshnessPolicyConsumptionMatrixId = 'app-game-source-freshness-policy-consumption' as const;

export const AppGameSourceFreshnessPolicyTargetKind = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
  AllNativeApps: 'all-native-apps',
  AllNativeGames: 'all-native-games',
} as const;

export const AppGameSourceFreshnessRequirementKind = {
  Inventory: 'inventory',
  Runtime: 'runtime',
  Foreground: 'foreground',
  Launcher: 'launcher',
} as const;

export const AppGameSourceFreshnessSourceKind = {
  OsInstalledRecord: 'osInstalledRecord',
  Shortcut: 'shortcut',
  StorePackage: 'storePackage',
  LauncherManifest: 'launcherManifest',
  ParentCatalog: 'parentCatalog',
  ManagedDevice: 'managedDevice',
  PortableApp: 'portableApp',
  UnknownSource: 'unknownSource',
  ProcessSnapshot: 'processSnapshot',
  ForegroundWindow: 'foregroundWindow',
  ProcessStart: 'processStart',
  ProcessExit: 'processExit',
  InventoryScan: 'inventoryScan',
} as const;

export const AppGameSourceFreshnessReadModelState = {
  Ready: 'ready',
  Empty: 'empty',
  Unavailable: 'unavailable',
  Offline: 'offline',
  Stale: 'stale',
  PermissionRequired: 'permission-required',
  ScaffoldOnly: 'scaffold-only',
} as const;

export const AppGameSourceFreshnessCapabilityStatus = {
  Available: 'available',
  Unavailable: 'unavailable',
  PermissionLimited: 'permissionLimited',
  UnsupportedPlatform: 'unsupportedPlatform',
  AdapterError: 'adapterError',
  Stale: 'stale',
  Degraded: 'degraded',
  ManualRequired: 'manualRequired',
  NotClaimed: 'notClaimed',
} as const;

export const AppGameSourceFreshnessRequirementState = {
  Satisfied: 'satisfied',
  Missing: 'missing',
  Empty: 'empty',
  MissingObservedAt: 'missing-observed-at',
  Stale: 'stale',
  PermissionLimited: 'permission-limited',
  Unavailable: 'unavailable',
  AdapterError: 'adapter-error',
  ManualRequired: 'manual-required',
  NotClaimed: 'not-claimed',
  MissingEvidence: 'missing-evidence',
} as const;

export const AppGameSourceFreshnessPolicyReadinessState = {
  PolicyReady: 'policy-ready',
  ManualRequired: 'manual-required',
} as const;

export const AppGameSourceFreshnessAdapterDispatchState = {
  NotDispatched: 'not-dispatched',
} as const;

export const AppGameSourceFreshnessReasonCode = {
  MissingSourceStatusRow: 'missing-source-status-row',
  EmptySourceStatusRow: 'empty-source-status-row',
  MissingObservedAt: 'missing-observed-at',
  StaleSourceStatusRow: 'stale-source-status-row',
  PermissionLimitedSourceStatus: 'permission-limited-source-status',
  UnavailableSourceStatus: 'unavailable-source-status',
  AdapterErrorSourceStatus: 'adapter-error-source-status',
  ManualRequiredSourceStatus: 'manual-required-source-status',
  NotClaimedSourceStatus: 'not-claimed-source-status',
  MissingSourceEvidence: 'missing-source-evidence',
} as const;

export const AppGameSourceFreshnessRequirementSourceKinds = {
  [AppGameSourceFreshnessRequirementKind.Inventory]: [
    AppGameSourceFreshnessSourceKind.OsInstalledRecord,
    AppGameSourceFreshnessSourceKind.Shortcut,
    AppGameSourceFreshnessSourceKind.StorePackage,
    AppGameSourceFreshnessSourceKind.ParentCatalog,
    AppGameSourceFreshnessSourceKind.ManagedDevice,
    AppGameSourceFreshnessSourceKind.PortableApp,
    AppGameSourceFreshnessSourceKind.UnknownSource,
    AppGameSourceFreshnessSourceKind.InventoryScan,
  ],
  [AppGameSourceFreshnessRequirementKind.Runtime]: [
    AppGameSourceFreshnessSourceKind.ProcessSnapshot,
    AppGameSourceFreshnessSourceKind.ProcessStart,
    AppGameSourceFreshnessSourceKind.ProcessExit,
  ],
  [AppGameSourceFreshnessRequirementKind.Foreground]: [AppGameSourceFreshnessSourceKind.ForegroundWindow],
  [AppGameSourceFreshnessRequirementKind.Launcher]: [AppGameSourceFreshnessSourceKind.LauncherManifest],
} as const;

export type AppGameSourceFreshnessPolicyTargetKind =
  (typeof AppGameSourceFreshnessPolicyTargetKind)[keyof typeof AppGameSourceFreshnessPolicyTargetKind];
export type AppGameSourceFreshnessRequirementKind =
  (typeof AppGameSourceFreshnessRequirementKind)[keyof typeof AppGameSourceFreshnessRequirementKind];
export type AppGameSourceFreshnessSourceKind =
  (typeof AppGameSourceFreshnessSourceKind)[keyof typeof AppGameSourceFreshnessSourceKind];
export type AppGameSourceFreshnessCapabilityStatus =
  (typeof AppGameSourceFreshnessCapabilityStatus)[keyof typeof AppGameSourceFreshnessCapabilityStatus];
export type AppGameSourceFreshnessReadModelState =
  (typeof AppGameSourceFreshnessReadModelState)[keyof typeof AppGameSourceFreshnessReadModelState];
export type AppGameSourceFreshnessRequirementState =
  (typeof AppGameSourceFreshnessRequirementState)[keyof typeof AppGameSourceFreshnessRequirementState];
export type AppGameSourceFreshnessReasonCode =
  (typeof AppGameSourceFreshnessReasonCode)[keyof typeof AppGameSourceFreshnessReasonCode];
