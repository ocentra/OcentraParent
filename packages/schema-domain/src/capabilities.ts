/* generated from crates/schema/src/parent_control_capabilities_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentControlPlatformCapabilityInputs } from './capability-data';

export const ParentControlPlatformSchema = withParser(Schema.Literal('windows', 'linux', 'macos', 'android', 'ios'));
export const ParentControlCapabilityStatusSchema = withParser(
  Schema.Literal(
    'supported',
    'implemented',
    'preview-scaffold',
    'scaffold',
    'manual-required',
    'unavailable',
    'planned',
    'not-implemented'
  )
);

export const ParentControlCapabilityNameSchema = withParser(
  Schema.Literal(
    'headless-agent-service',
    'local-websocket-control',
    'lan-websocket-control',
    'owned-process-terminate',
    'app-time-limit',
    'app-blocking',
    'network-domain-blocking',
    'managed-browser-control',
    'unmanaged-browser-detection',
    'parent-mobile-controller',
    'parent-mobile-observer',
    'foreground-mobile-service',
    'local-storage',
    'typed-protocol-bridge',
    'usage-stats',
    'accessibility-service',
    'vpn-dns-filtering',
    'device-owner-policy',
    'managed-profile',
    'package-lifecycle',
    'family-controls-entitlement',
    'device-activity',
    'screen-time-api',
    'network-extension',
    'notifications',
    'background-execution',
    'signing-entitlements',
    'testflight-distribution',
    'signed-auto-update',
    'store-distribution'
  )
);

export const ParentControlCapabilitySchema = withParser(
  Schema.Struct({
    capability: ParentControlCapabilityNameSchema,
    status: ParentControlCapabilityStatusSchema,
    note: brandedNonEmptyStringSchema('ParentControlCapabilityNote'),
  })
);

export const ParentControlPlatformCapabilitySchema = withParser(
  Schema.Struct({
    platform: ParentControlPlatformSchema,
    capabilities: Schema.Array(ParentControlCapabilitySchema),
  })
);

export type ParentControlPlatform = Infer<typeof ParentControlPlatformSchema>;
export type ParentControlCapabilityStatus = Infer<typeof ParentControlCapabilityStatusSchema>;
export type ParentControlCapabilityName = Infer<typeof ParentControlCapabilityNameSchema>;
export type ParentControlCapability = Infer<typeof ParentControlCapabilitySchema>;
export type ParentControlPlatformCapability = Infer<typeof ParentControlPlatformCapabilitySchema>;

export const ParentControlCapabilityStatus = {
  Supported: ParentControlCapabilityStatusSchema.parse('supported'),
  Implemented: ParentControlCapabilityStatusSchema.parse('implemented'),
  PreviewScaffold: ParentControlCapabilityStatusSchema.parse('preview-scaffold'),
  Scaffold: ParentControlCapabilityStatusSchema.parse('scaffold'),
  ManualRequired: ParentControlCapabilityStatusSchema.parse('manual-required'),
  Unavailable: ParentControlCapabilityStatusSchema.parse('unavailable'),
  Planned: ParentControlCapabilityStatusSchema.parse('planned'),
  NotImplemented: ParentControlCapabilityStatusSchema.parse('not-implemented'),
} as const;

export const ParentControlCapabilityName = {
  HeadlessAgentService: ParentControlCapabilityNameSchema.parse('headless-agent-service'),
  LocalWebSocketControl: ParentControlCapabilityNameSchema.parse('local-websocket-control'),
  LanWebSocketControl: ParentControlCapabilityNameSchema.parse('lan-websocket-control'),
  OwnedProcessTerminate: ParentControlCapabilityNameSchema.parse('owned-process-terminate'),
  AppTimeLimit: ParentControlCapabilityNameSchema.parse('app-time-limit'),
  AppBlocking: ParentControlCapabilityNameSchema.parse('app-blocking'),
  NetworkDomainBlocking: ParentControlCapabilityNameSchema.parse('network-domain-blocking'),
  ManagedBrowserControl: ParentControlCapabilityNameSchema.parse('managed-browser-control'),
  UnmanagedBrowserDetection: ParentControlCapabilityNameSchema.parse('unmanaged-browser-detection'),
  ParentMobileController: ParentControlCapabilityNameSchema.parse('parent-mobile-controller'),
  ParentMobileObserver: ParentControlCapabilityNameSchema.parse('parent-mobile-observer'),
  ForegroundMobileService: ParentControlCapabilityNameSchema.parse('foreground-mobile-service'),
  LocalStorage: ParentControlCapabilityNameSchema.parse('local-storage'),
  TypedProtocolBridge: ParentControlCapabilityNameSchema.parse('typed-protocol-bridge'),
  UsageStats: ParentControlCapabilityNameSchema.parse('usage-stats'),
  AccessibilityService: ParentControlCapabilityNameSchema.parse('accessibility-service'),
  VpnDnsFiltering: ParentControlCapabilityNameSchema.parse('vpn-dns-filtering'),
  DeviceOwnerPolicy: ParentControlCapabilityNameSchema.parse('device-owner-policy'),
  ManagedProfile: ParentControlCapabilityNameSchema.parse('managed-profile'),
  PackageLifecycle: ParentControlCapabilityNameSchema.parse('package-lifecycle'),
  FamilyControlsEntitlement: ParentControlCapabilityNameSchema.parse('family-controls-entitlement'),
  DeviceActivity: ParentControlCapabilityNameSchema.parse('device-activity'),
  ScreenTimeApi: ParentControlCapabilityNameSchema.parse('screen-time-api'),
  NetworkExtension: ParentControlCapabilityNameSchema.parse('network-extension'),
  Notifications: ParentControlCapabilityNameSchema.parse('notifications'),
  BackgroundExecution: ParentControlCapabilityNameSchema.parse('background-execution'),
  SigningEntitlements: ParentControlCapabilityNameSchema.parse('signing-entitlements'),
  TestflightDistribution: ParentControlCapabilityNameSchema.parse('testflight-distribution'),
  SignedAutoUpdate: ParentControlCapabilityNameSchema.parse('signed-auto-update'),
  StoreDistribution: ParentControlCapabilityNameSchema.parse('store-distribution'),
} as const;

export const ParentControlPlatformCapabilities = ParentControlPlatformCapabilityInputs.map((entry) =>
  ParentControlPlatformCapabilitySchema.parse(entry)
);
