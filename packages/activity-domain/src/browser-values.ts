import {
  BrowserActiveProofSourceSchema,
  BrowserActiveTabStateSchema,
  BrowserBridgeKindSchema,
  BrowserCapabilityStatusSchema,
  BrowserChannelSchema,
  BrowserCustodyLabelSchema,
  BrowserFamilySchema,
  BrowserManagedStateSchema,
  BrowserQueryVisibilityLabelSchema,
} from './browser-schemas';
import {
  BrowserUnmanagedDetectionConfidenceSchema,
  BrowserUnmanagedDetectionReasonSchema,
  BrowserUnmanagedProcessKindSchema,
} from './browser-unmanaged-process-schemas';
import {
  BrowserActiveTabCapabilitySchema,
  BrowserExactUrlCapabilitySchema,
  BrowserInventoryInstallStateSchema,
  BrowserInventoryRunningStateSchema,
  BrowserManagedProfileStateSchema,
  BrowserManagementTierSchema,
  BrowserSupportTierSchema,
  BrowserUnmanagedFallbackCapabilitySchema,
} from './browser-inventory-schemas';
import {
  BrowserInventoryPlatform as BrowserInventoryPlatformValue,
  BrowserInventoryPlatformMatrix as BrowserInventoryPlatformMatrixValue,
  BrowserInventoryPlatformProofState as BrowserInventoryPlatformProofStateValue,
} from './browser-platform-inventory-matrix';

export const BrowserFamily = {
  Edge: BrowserFamilySchema.parse('edge'),
  Chrome: BrowserFamilySchema.parse('chrome'),
  Brave: BrowserFamilySchema.parse('brave'),
  Firefox: BrowserFamilySchema.parse('firefox'),
  Opera: BrowserFamilySchema.parse('opera'),
  UnknownChromium: BrowserFamilySchema.parse('unknown-chromium'),
  Unknown: BrowserFamilySchema.parse('unknown'),
} as const;

export const BrowserChannel = {
  Stable: BrowserChannelSchema.parse('stable'),
  Beta: BrowserChannelSchema.parse('beta'),
  Dev: BrowserChannelSchema.parse('dev'),
  Canary: BrowserChannelSchema.parse('canary'),
  Unknown: BrowserChannelSchema.parse('unknown'),
} as const;

export const BrowserCapabilityStatus = {
  Available: BrowserCapabilityStatusSchema.parse('available'),
  TabListOnly: BrowserCapabilityStatusSchema.parse('tab-list-only'),
  UnsupportedBrowser: BrowserCapabilityStatusSchema.parse('unsupported-browser'),
  UnmanagedBrowser: BrowserCapabilityStatusSchema.parse('unmanaged-browser'),
  ManagedProfileMissing: BrowserCapabilityStatusSchema.parse('managed-profile-missing'),
  BridgeMissing: BrowserCapabilityStatusSchema.parse('bridge-missing'),
  PermissionLimited: BrowserCapabilityStatusSchema.parse('permission-limited'),
  Stale: BrowserCapabilityStatusSchema.parse('stale'),
  AdapterError: BrowserCapabilityStatusSchema.parse('adapter-error'),
  DisabledByParent: BrowserCapabilityStatusSchema.parse('disabled-by-parent'),
} as const;

export const BrowserManagedState = {
  NotInstalled: BrowserManagedStateSchema.parse('not-installed'),
  InstalledUnsupported: BrowserManagedStateSchema.parse('installed-unsupported'),
  InstalledSupported: BrowserManagedStateSchema.parse('installed-supported'),
  ManagedProfileReady: BrowserManagedStateSchema.parse('managed-profile-ready'),
  LaunchPending: BrowserManagedStateSchema.parse('launch-pending'),
  RunningManaged: BrowserManagedStateSchema.parse('running-managed'),
  BridgeConnected: BrowserManagedStateSchema.parse('bridge-connected'),
  BridgeDisconnected: BrowserManagedStateSchema.parse('bridge-disconnected'),
  PermissionRequired: BrowserManagedStateSchema.parse('permission-required'),
  Stopped: BrowserManagedStateSchema.parse('stopped'),
  Error: BrowserManagedStateSchema.parse('error'),
} as const;

export const BrowserBridgeKind = {
  ChromiumDevtoolsProtocol: BrowserBridgeKindSchema.parse('chromium-devtools-protocol'),
} as const;

export const BrowserActiveTabState = {
  KnownActive: BrowserActiveTabStateSchema.parse('known-active'),
  KnownInactive: BrowserActiveTabStateSchema.parse('known-inactive'),
  Unknown: BrowserActiveTabStateSchema.parse('unknown'),
} as const;

export const BrowserActiveProofSource = {
  TargetListOnly: BrowserActiveProofSourceSchema.parse('target-list-only'),
  CdpFocusActivation: BrowserActiveProofSourceSchema.parse('cdp-focus-activation'),
  ManagedExtensionEvent: BrowserActiveProofSourceSchema.parse('managed-extension-event'),
  ForegroundCorrelation: BrowserActiveProofSourceSchema.parse('foreground-correlation'),
  OwnedShellEvent: BrowserActiveProofSourceSchema.parse('owned-shell-event'),
} as const;

export const BrowserCustodyLabel = {
  ChildDeviceLocal: BrowserCustodyLabelSchema.parse('child-device-local'),
  LocalNetworkChildAgent: BrowserCustodyLabelSchema.parse('local-network-child-agent'),
  ParentCache: BrowserCustodyLabelSchema.parse('parent-cache'),
  ParentOwnedExport: BrowserCustodyLabelSchema.parse('parent-owned-export'),
  Unavailable: BrowserCustodyLabelSchema.parse('unavailable'),
} as const;

export const BrowserQueryVisibilityLabel = {
  LiveLocal: BrowserQueryVisibilityLabelSchema.parse('live-local'),
  LiveLan: BrowserQueryVisibilityLabelSchema.parse('live-lan'),
  ParentCache: BrowserQueryVisibilityLabelSchema.parse('parent-cache'),
  ParentOwnedExport: BrowserQueryVisibilityLabelSchema.parse('parent-owned-export'),
  Unavailable: BrowserQueryVisibilityLabelSchema.parse('unavailable'),
} as const;

export const BrowserUnmanagedDetectionConfidence = {
  High: BrowserUnmanagedDetectionConfidenceSchema.parse('high'),
  Medium: BrowserUnmanagedDetectionConfidenceSchema.parse('medium'),
  Low: BrowserUnmanagedDetectionConfidenceSchema.parse('low'),
} as const;

export const BrowserUnmanagedProcessKind = {
  SupportedBrowser: BrowserUnmanagedProcessKindSchema.parse('supported-browser'),
  UnsupportedBrowser: BrowserUnmanagedProcessKindSchema.parse('unsupported-browser'),
  PortableBrowser: BrowserUnmanagedProcessKindSchema.parse('portable-browser'),
  TorPrivacyBrowser: BrowserUnmanagedProcessKindSchema.parse('tor-privacy-browser'),
  PackagedBrowser: BrowserUnmanagedProcessKindSchema.parse('packaged-browser'),
  EmbeddedBrowserLike: BrowserUnmanagedProcessKindSchema.parse('embedded-browser-like'),
  UnknownBrowserLike: BrowserUnmanagedProcessKindSchema.parse('unknown-browser-like'),
  PossibleSocialBypass: BrowserUnmanagedProcessKindSchema.parse('possible-social-bypass'),
  PossibleBrowserGameBypass: BrowserUnmanagedProcessKindSchema.parse('possible-browser-game-bypass'),
  PossibleCloudGamingBypass: BrowserUnmanagedProcessKindSchema.parse('possible-cloud-gaming-bypass'),
} as const;

export const BrowserUnmanagedDetectionReason = {
  SupportedBrowserOutsideManagedSession: BrowserUnmanagedDetectionReasonSchema.parse(
    'supported-browser-outside-managed-session'
  ),
  UnsupportedBrowserProcess: BrowserUnmanagedDetectionReasonSchema.parse('unsupported-browser-process'),
  PortableBrowserProcess: BrowserUnmanagedDetectionReasonSchema.parse('portable-browser-process'),
  TorPrivacyBrowserProcess: BrowserUnmanagedDetectionReasonSchema.parse('tor-privacy-browser-process'),
  PackagedBrowserProcess: BrowserUnmanagedDetectionReasonSchema.parse('packaged-browser-process'),
  BrowserLikeProcess: BrowserUnmanagedDetectionReasonSchema.parse('browser-like-process'),
  PossibleSocialBypass: BrowserUnmanagedDetectionReasonSchema.parse('possible-social-bypass'),
  PossibleBrowserGameBypass: BrowserUnmanagedDetectionReasonSchema.parse('possible-browser-game-bypass'),
  PossibleCloudGamingBypass: BrowserUnmanagedDetectionReasonSchema.parse('possible-cloud-gaming-bypass'),
} as const;

export const BrowserInventoryInstallState = {
  Installed: BrowserInventoryInstallStateSchema.parse('installed'),
  NotInstalled: BrowserInventoryInstallStateSchema.parse('not-installed'),
  CandidateRunning: BrowserInventoryInstallStateSchema.parse('candidate-running'),
  Packaged: BrowserInventoryInstallStateSchema.parse('packaged'),
  Portable: BrowserInventoryInstallStateSchema.parse('portable'),
  Unknown: BrowserInventoryInstallStateSchema.parse('unknown'),
} as const;

export const BrowserInventoryRunningState = {
  NotRunning: BrowserInventoryRunningStateSchema.parse('not-running'),
  RunningManaged: BrowserInventoryRunningStateSchema.parse('running-managed'),
  RunningUnmanaged: BrowserInventoryRunningStateSchema.parse('running-unmanaged'),
  RunningUnknown: BrowserInventoryRunningStateSchema.parse('running-unknown'),
  Unknown: BrowserInventoryRunningStateSchema.parse('unknown'),
} as const;

export const BrowserManagementTier = {
  Managed: BrowserManagementTierSchema.parse('managed'),
  OwnedShell: BrowserManagementTierSchema.parse('owned-shell'),
  ManagedProfileExtension: BrowserManagementTierSchema.parse('managed-profile-extension'),
  Unmanaged: BrowserManagementTierSchema.parse('unmanaged'),
  Unsupported: BrowserManagementTierSchema.parse('unsupported'),
  ManualRequired: BrowserManagementTierSchema.parse('manual-required'),
  Unknown: BrowserManagementTierSchema.parse('unknown'),
} as const;

export const BrowserSupportTier = {
  ManagedUrlTab: BrowserSupportTierSchema.parse('managed-url-tab'),
  ManagedTargetList: BrowserSupportTierSchema.parse('managed-target-list'),
  Candidate: BrowserSupportTierSchema.parse('candidate'),
  UnmanagedProcessOnly: BrowserSupportTierSchema.parse('unmanaged-process-only'),
  Unsupported: BrowserSupportTierSchema.parse('unsupported'),
  ManualRequired: BrowserSupportTierSchema.parse('manual-required'),
  Unknown: BrowserSupportTierSchema.parse('unknown'),
} as const;

export const BrowserExactUrlCapability = {
  ManagedExactUrlAvailable: BrowserExactUrlCapabilitySchema.parse('managed-exact-url-available'),
  ManagedTargetListOnly: BrowserExactUrlCapabilitySchema.parse('managed-target-list-only'),
  ManualRequired: BrowserExactUrlCapabilitySchema.parse('manual-required'),
  NotClaimed: BrowserExactUrlCapabilitySchema.parse('not-claimed'),
  Unsupported: BrowserExactUrlCapabilitySchema.parse('unsupported'),
  Unavailable: BrowserExactUrlCapabilitySchema.parse('unavailable'),
} as const;

export const BrowserActiveTabCapability = {
  KnownActiveSupported: BrowserActiveTabCapabilitySchema.parse('known-active-supported'),
  TargetListOnly: BrowserActiveTabCapabilitySchema.parse('target-list-only'),
  ManualRequired: BrowserActiveTabCapabilitySchema.parse('manual-required'),
  NotClaimed: BrowserActiveTabCapabilitySchema.parse('not-claimed'),
  Unsupported: BrowserActiveTabCapabilitySchema.parse('unsupported'),
  Unavailable: BrowserActiveTabCapabilitySchema.parse('unavailable'),
} as const;

export const BrowserManagedProfileState = {
  Ready: BrowserManagedProfileStateSchema.parse('ready'),
  Missing: BrowserManagedProfileStateSchema.parse('missing'),
  RepairRequired: BrowserManagedProfileStateSchema.parse('repair-required'),
  NotApplicable: BrowserManagedProfileStateSchema.parse('not-applicable'),
  ManualRequired: BrowserManagedProfileStateSchema.parse('manual-required'),
  Unavailable: BrowserManagedProfileStateSchema.parse('unavailable'),
} as const;

export const BrowserUnmanagedFallbackCapability = {
  ReportOnly: BrowserUnmanagedFallbackCapabilitySchema.parse('report-only'),
  WarnChild: BrowserUnmanagedFallbackCapabilitySchema.parse('warn-child'),
  TerminateProcess: BrowserUnmanagedFallbackCapabilitySchema.parse('terminate-process'),
  RelaunchManaged: BrowserUnmanagedFallbackCapabilitySchema.parse('relaunch-managed'),
  OsBlockManualRequired: BrowserUnmanagedFallbackCapabilitySchema.parse('os-block-manual-required'),
  Unsupported: BrowserUnmanagedFallbackCapabilitySchema.parse('unsupported'),
  Unavailable: BrowserUnmanagedFallbackCapabilitySchema.parse('unavailable'),
} as const;

export const BrowserInventoryPlatform = BrowserInventoryPlatformValue;
export const BrowserInventoryPlatformProofState = BrowserInventoryPlatformProofStateValue;
export const BrowserInventoryPlatformMatrix = BrowserInventoryPlatformMatrixValue;
