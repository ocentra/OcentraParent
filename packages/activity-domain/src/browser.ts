import {
  BrowserActiveTabStateSchema,
  BrowserBridgeKindSchema,
  BrowserCapabilityStatusSchema,
  BrowserChannelSchema,
  BrowserCustodyLabelSchema,
  BrowserFamilySchema,
  BrowserManagedStateSchema,
  BrowserQueryVisibilityLabelSchema,
} from './browser-schemas';
import type {
  BrowserActiveTabState as BrowserActiveTabStateType,
  BrowserBridgeKind as BrowserBridgeKindType,
  BrowserCapabilityStatus as BrowserCapabilityStatusType,
  BrowserChannel as BrowserChannelType,
  BrowserCustodyLabel as BrowserCustodyLabelType,
  BrowserEvidenceRecentSummary,
  BrowserFamily as BrowserFamilyType,
  BrowserManagedState as BrowserManagedStateType,
  BrowserQueryVisibilityLabel as BrowserQueryVisibilityLabelType,
  BrowserTabEvidence,
} from './browser-schemas';

export {
  BrowserActiveTabStateSchema,
  BrowserAdapterIdSchema,
  BrowserBridgeKindSchema,
  BrowserCapabilityStatusSchema,
  BrowserChannelSchema,
  BrowserCustodyLabelSchema,
  BrowserDegradedReasonSchema,
  BrowserDomainSchema,
  BrowserEvidenceRecentSummarySchema,
  BrowserEvidenceSchemaVersion,
  BrowserFamilySchema,
  BrowserManagedSessionIdSchema,
  BrowserManagedStateSchema,
  BrowserOriginSchema,
  BrowserPageTitleSchema,
  BrowserProfileIdSchema,
  BrowserQueryVisibilityLabelSchema,
  BrowserTabEvidenceSchema,
  BrowserTabIdSchema,
  BrowserTargetIdSchema,
  BrowserUrlSchema,
  BrowserWindowIdSchema,
  decodeBrowserUrl,
} from './browser-schemas';

export type BrowserActiveTabState = BrowserActiveTabStateType;
export type BrowserBridgeKind = BrowserBridgeKindType;
export type BrowserCapabilityStatus = BrowserCapabilityStatusType;
export type BrowserChannel = BrowserChannelType;
export type BrowserCustodyLabel = BrowserCustodyLabelType;
export type { BrowserEvidenceRecentSummary, BrowserTabEvidence };
export type BrowserFamily = BrowserFamilyType;
export type BrowserManagedState = BrowserManagedStateType;
export type BrowserQueryVisibilityLabel = BrowserQueryVisibilityLabelType;

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
