export const BrowserFamily = {
  Edge: 'edge',
  Chrome: 'chrome',
  Brave: 'brave',
  Firefox: 'firefox',
  Opera: 'opera',
  UnknownChromium: 'unknown-chromium',
  Unknown: 'unknown',
} as const;

export const BrowserChannel = {
  Stable: 'stable',
  Beta: 'beta',
  Dev: 'dev',
  Canary: 'canary',
  Unknown: 'unknown',
} as const;

export const BrowserCapabilityStatus = {
  Available: 'available',
  TabListOnly: 'tab-list-only',
  UnsupportedBrowser: 'unsupported-browser',
  UnmanagedBrowser: 'unmanaged-browser',
  ManagedProfileMissing: 'managed-profile-missing',
  BridgeMissing: 'bridge-missing',
  PermissionLimited: 'permission-limited',
  Stale: 'stale',
  AdapterError: 'adapter-error',
  DisabledByParent: 'disabled-by-parent',
} as const;

export const BrowserManagedState = {
  NotInstalled: 'not-installed',
  InstalledUnsupported: 'installed-unsupported',
  InstalledSupported: 'installed-supported',
  ManagedProfileReady: 'managed-profile-ready',
  LaunchPending: 'launch-pending',
  RunningManaged: 'running-managed',
  BridgeConnected: 'bridge-connected',
  BridgeDisconnected: 'bridge-disconnected',
  PermissionRequired: 'permission-required',
  Stopped: 'stopped',
  Error: 'error',
} as const;

export const BrowserBridgeKind = {
  ChromiumDevtoolsProtocol: 'chromium-devtools-protocol',
} as const;

export const BrowserActiveTabState = {
  KnownActive: 'known-active',
  KnownInactive: 'known-inactive',
  Unknown: 'unknown',
} as const;

export const BrowserActiveProofSource = {
  TargetListOnly: 'target-list-only',
  CdpFocusActivation: 'cdp-focus-activation',
  ManagedExtensionEvent: 'managed-extension-event',
  ForegroundCorrelation: 'foreground-correlation',
  OwnedShellEvent: 'owned-shell-event',
} as const;

export const BrowserCustodyLabel = {
  ChildDeviceLocal: 'child-device-local',
  LocalNetworkChildAgent: 'local-network-child-agent',
  ParentCache: 'parent-cache',
  ParentOwnedExport: 'parent-owned-export',
  Unavailable: 'unavailable',
} as const;

export const BrowserQueryVisibilityLabel = {
  LiveLocal: 'live-local',
  LiveLan: 'live-lan',
  ParentCache: 'parent-cache',
  ParentOwnedExport: 'parent-owned-export',
  Unavailable: 'unavailable',
} as const;

export const BrowserUnmanagedDetectionConfidence = {
  High: 'high',
  Medium: 'medium',
  Low: 'low',
} as const;

export const BrowserUnmanagedProcessKind = {
  SupportedBrowser: 'supported-browser',
  UnsupportedBrowser: 'unsupported-browser',
  PortableBrowser: 'portable-browser',
  TorPrivacyBrowser: 'tor-privacy-browser',
  PackagedBrowser: 'packaged-browser',
  EmbeddedBrowserLike: 'embedded-browser-like',
  UnknownBrowserLike: 'unknown-browser-like',
  PossibleSocialBypass: 'possible-social-bypass',
  PossibleBrowserGameBypass: 'possible-browser-game-bypass',
  PossibleCloudGamingBypass: 'possible-cloud-gaming-bypass',
} as const;

export const BrowserUnmanagedDetectionReason = {
  SupportedBrowserOutsideManagedSession: 'supported-browser-outside-managed-session',
  UnsupportedBrowserProcess: 'unsupported-browser-process',
  PortableBrowserProcess: 'portable-browser-process',
  TorPrivacyBrowserProcess: 'tor-privacy-browser-process',
  PackagedBrowserProcess: 'packaged-browser-process',
  BrowserLikeProcess: 'browser-like-process',
  PossibleSocialBypass: 'possible-social-bypass',
  PossibleBrowserGameBypass: 'possible-browser-game-bypass',
  PossibleCloudGamingBypass: 'possible-cloud-gaming-bypass',
} as const;

export const BrowserInventoryInstallState = {
  Installed: 'installed',
  NotInstalled: 'not-installed',
  CandidateRunning: 'candidate-running',
  Packaged: 'packaged',
  Portable: 'portable',
  Unknown: 'unknown',
} as const;

export const BrowserInventoryRunningState = {
  NotRunning: 'not-running',
  RunningManaged: 'running-managed',
  RunningUnmanaged: 'running-unmanaged',
  RunningUnknown: 'running-unknown',
  Unknown: 'unknown',
} as const;

export const BrowserManagementTier = {
  Managed: 'managed',
  OwnedShell: 'owned-shell',
  ManagedProfileExtension: 'managed-profile-extension',
  Unmanaged: 'unmanaged',
  Unsupported: 'unsupported',
  ManualRequired: 'manual-required',
  Unknown: 'unknown',
} as const;

export const BrowserSupportTier = {
  ManagedUrlTab: 'managed-url-tab',
  ManagedTargetList: 'managed-target-list',
  Candidate: 'candidate',
  UnmanagedProcessOnly: 'unmanaged-process-only',
  Unsupported: 'unsupported',
  ManualRequired: 'manual-required',
  Unknown: 'unknown',
} as const;

export const BrowserExactUrlCapability = {
  ManagedExactUrlAvailable: 'managed-exact-url-available',
  ManagedTargetListOnly: 'managed-target-list-only',
  ManualRequired: 'manual-required',
  NotClaimed: 'not-claimed',
  Unsupported: 'unsupported',
  Unavailable: 'unavailable',
} as const;

export const BrowserActiveTabCapability = {
  KnownActiveSupported: 'known-active-supported',
  TargetListOnly: 'target-list-only',
  ManualRequired: 'manual-required',
  NotClaimed: 'not-claimed',
  Unsupported: 'unsupported',
  Unavailable: 'unavailable',
} as const;

export const BrowserManagedProfileState = {
  Ready: 'ready',
  Missing: 'missing',
  RepairRequired: 'repair-required',
  NotApplicable: 'not-applicable',
  ManualRequired: 'manual-required',
  Unavailable: 'unavailable',
} as const;

export const BrowserUnmanagedFallbackCapability = {
  ReportOnly: 'report-only',
  WarnChild: 'warn-child',
  TerminateProcess: 'terminate-process',
  RelaunchManaged: 'relaunch-managed',
  OsBlockManualRequired: 'os-block-manual-required',
  Unsupported: 'unsupported',
  Unavailable: 'unavailable',
} as const;
