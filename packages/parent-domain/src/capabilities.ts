import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyCapabilityText = Schema.String.pipe(Schema.minLength(1));

export const ParentControlPlatformSchema = withParser(Schema.Literal('windows', 'linux', 'macos', 'android', 'ios'));
export const ParentControlCapabilityStatusSchema = withParser(
  Schema.Literal('supported', 'preview-scaffold', 'planned')
);

export const ParentControlCapabilityNameSchema = withParser(
  Schema.Literal(
    'headless-agent-service',
    'local-websocket-control',
    'lan-websocket-control',
    'foreground-mobile-service',
    'device-owner-policy',
    'family-controls-entitlement',
    'signed-auto-update',
    'store-distribution'
  )
);

export const ParentControlCapabilitySchema = withParser(
  Schema.Struct({
    capability: ParentControlCapabilityNameSchema,
    status: ParentControlCapabilityStatusSchema,
    note: NonEmptyCapabilityText.pipe(Schema.brand('ParentControlCapabilityNote')),
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
  PreviewScaffold: ParentControlCapabilityStatusSchema.parse('preview-scaffold'),
  Planned: ParentControlCapabilityStatusSchema.parse('planned'),
} as const;

export const ParentControlCapabilityName = {
  HeadlessAgentService: ParentControlCapabilityNameSchema.parse('headless-agent-service'),
  LocalWebSocketControl: ParentControlCapabilityNameSchema.parse('local-websocket-control'),
  LanWebSocketControl: ParentControlCapabilityNameSchema.parse('lan-websocket-control'),
  ForegroundMobileService: ParentControlCapabilityNameSchema.parse('foreground-mobile-service'),
  DeviceOwnerPolicy: ParentControlCapabilityNameSchema.parse('device-owner-policy'),
  FamilyControlsEntitlement: ParentControlCapabilityNameSchema.parse('family-controls-entitlement'),
  SignedAutoUpdate: ParentControlCapabilityNameSchema.parse('signed-auto-update'),
  StoreDistribution: ParentControlCapabilityNameSchema.parse('store-distribution'),
} as const;

export const ParentControlPlatformCapabilities = [
  ParentControlPlatformCapabilitySchema.parse({
    platform: 'windows',
    capabilities: [
      {
        capability: 'headless-agent-service',
        status: 'supported',
        note: 'Windows service package is the first supported agent target.',
      },
      {
        capability: 'local-websocket-control',
        status: 'supported',
        note: 'Local portal can connect to the Windows agent over localhost.',
      },
      {
        capability: 'lan-websocket-control',
        status: 'preview-scaffold',
        note: 'LAN transport exists for development and must require pairing before child activity control.',
      },
      {
        capability: 'signed-auto-update',
        status: 'supported',
        note: 'Windows update manifest signing and MSI upgrade scaffold are wired.',
      },
    ],
  }),
  ParentControlPlatformCapabilitySchema.parse({
    platform: 'linux',
    capabilities: [
      {
        capability: 'headless-agent-service',
        status: 'preview-scaffold',
        note: 'Linux deb and systemd package preview builds in CI.',
      },
    ],
  }),
  ParentControlPlatformCapabilitySchema.parse({
    platform: 'macos',
    capabilities: [
      {
        capability: 'headless-agent-service',
        status: 'preview-scaffold',
        note: 'macOS pkg and launchd package preview builds in CI.',
      },
    ],
  }),
  ParentControlPlatformCapabilitySchema.parse({
    platform: 'android',
    capabilities: [
      {
        capability: 'foreground-mobile-service',
        status: 'preview-scaffold',
        note: 'Android debug APK foreground service preview builds in CI.',
      },
      {
        capability: 'device-owner-policy',
        status: 'planned',
        note: 'Device-owner policy is not claimed until enrollment and policy tests exist.',
      },
      {
        capability: 'store-distribution',
        status: 'planned',
        note: 'Google Play signing and release tracks are not wired yet.',
      },
    ],
  }),
  ParentControlPlatformCapabilitySchema.parse({
    platform: 'ios',
    capabilities: [
      {
        capability: 'foreground-mobile-service',
        status: 'preview-scaffold',
        note: 'iOS simulator app preview builds in CI.',
      },
      {
        capability: 'family-controls-entitlement',
        status: 'planned',
        note: 'Apple Family Controls entitlement is not claimed until entitlement and device tests exist.',
      },
      {
        capability: 'store-distribution',
        status: 'planned',
        note: 'Apple signing, notarization, and App Store workflows are not wired yet.',
      },
    ],
  }),
] as const;
