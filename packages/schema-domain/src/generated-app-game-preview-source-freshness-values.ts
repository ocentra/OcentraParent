/* generated from crates/schema/src/app_game_preview_source_freshness.rs */

export const AppGamePolicyPreviewTargetDomainGenerated = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
} as const;

export const AppGamePolicyPreviewStatusGenerated = {
  PreviewReady: 'preview-ready',
  ManualRequired: 'manual-required',
  Rejected: 'rejected',
} as const;

export const AppGamePolicyPreviewNoRuntimeClaimStatesGenerated = {
  policyEvaluatorRuntimeClaimState: 'not-claimed',
  timerRuntimeClaimState: 'not-claimed',
  adapterDispatchState: 'not-dispatched',
  childDeliveryClaimState: 'not-claimed',
  platformEnforcementClaimState: 'not-claimed',
} as const;

export const AppGamePolicyPreviewNoRuntimeClaimFlagsGenerated = {
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
} as const;

export const AppGameSourceFreshnessPolicyConsumptionMatrixIdGenerated =
  'app-game-source-freshness-policy-consumption' as const;

export const AppGameSourceFreshnessPolicyTargetKindGenerated = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
  AllNativeApps: 'all-native-apps',
  AllNativeGames: 'all-native-games',
} as const;

export const AppGameSourceFreshnessRequirementKindGenerated = {
  Inventory: 'inventory',
  Runtime: 'runtime',
  Foreground: 'foreground',
  Launcher: 'launcher',
} as const;

export const AppGameSourceFreshnessSourceKindGenerated = {
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

export const AppGameSourceFreshnessReadModelStateGenerated = {
  Ready: 'ready',
  Empty: 'empty',
  Unavailable: 'unavailable',
  Offline: 'offline',
  Stale: 'stale',
  PermissionRequired: 'permission-required',
  ScaffoldOnly: 'scaffold-only',
} as const;

export const AppGameSourceFreshnessCapabilityStatusGenerated = {
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

export const AppGameSourceFreshnessRequirementStateGenerated = {
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

export const AppGameSourceFreshnessPolicyReadinessStateGenerated = {
  PolicyReady: 'policy-ready',
  ManualRequired: 'manual-required',
} as const;

export const AppGameSourceFreshnessAdapterDispatchStateGenerated = {
  NotDispatched: 'not-dispatched',
} as const;

export const AppGameSourceFreshnessReasonCodeGenerated = {
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

export const AppGameSourceFreshnessRequirementSourceKindsGenerated = {
  [AppGameSourceFreshnessRequirementKindGenerated.Inventory]: [
    AppGameSourceFreshnessSourceKindGenerated.OsInstalledRecord,
    AppGameSourceFreshnessSourceKindGenerated.Shortcut,
    AppGameSourceFreshnessSourceKindGenerated.StorePackage,
    AppGameSourceFreshnessSourceKindGenerated.ParentCatalog,
    AppGameSourceFreshnessSourceKindGenerated.ManagedDevice,
    AppGameSourceFreshnessSourceKindGenerated.PortableApp,
    AppGameSourceFreshnessSourceKindGenerated.UnknownSource,
    AppGameSourceFreshnessSourceKindGenerated.InventoryScan,
  ],
  [AppGameSourceFreshnessRequirementKindGenerated.Runtime]: [
    AppGameSourceFreshnessSourceKindGenerated.ProcessSnapshot,
    AppGameSourceFreshnessSourceKindGenerated.ProcessStart,
    AppGameSourceFreshnessSourceKindGenerated.ProcessExit,
  ],
  [AppGameSourceFreshnessRequirementKindGenerated.Foreground]: [
    AppGameSourceFreshnessSourceKindGenerated.ForegroundWindow,
  ],
  [AppGameSourceFreshnessRequirementKindGenerated.Launcher]: [
    AppGameSourceFreshnessSourceKindGenerated.LauncherManifest,
  ],
} as const;

export const AppGameSourceFreshnessPreviewGateStatusGenerated = {
  PreviewReady: 'preview-ready',
  ManualRequired: 'manual-required',
} as const;

export const AppGameSourceFreshnessPreviewGateStateGenerated = {
  SourceFresh: 'source-fresh',
  SourceManualRequired: 'source-manual-required',
  CompilerManualRequired: 'compiler-manual-required',
} as const;

export const AppGameSourceFreshnessPreviewGateNoRuntimeClaimFlagsGenerated = {
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
} as const;

export const AppGameSourceGatedPolicyPreviewReadModelProjectionStateGenerated = {
  PreviewReadyVisible: 'preview-ready-visible',
  SourceManualRequiredVisible: 'source-manual-required-visible',
  CompilerManualRequiredVisible: 'compiler-manual-required-visible',
} as const;

export const AppGameSourceGatedPolicyPreviewReadModelSensitiveBoundaryGenerated = {
  RedactedEvidenceRefsOnly: 'redacted-evidence-refs-only',
} as const;

export const RequiredAppGameSourceGatedPolicyPreviewReadModelNonClaimsGenerated = [
  'no-service-runtime-event',
  'no-portal-ui-rendered',
  'no-policy-evaluator-runtime',
  'no-timer-runtime',
  'no-adapter-dispatch',
  'no-child-delivery',
  'no-platform-enforcement',
  'no-raw-private-source-rows',
] as const;

export const AppGameSourceGatedPolicyPreviewReadModelNoClaimFlagsGenerated = {
  serviceRuntimeEventClaimed: false,
  portalUiRendered: false,
  policyEvaluatorRuntimeClaimed: false,
  timerRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
} as const;
