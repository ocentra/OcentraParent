import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ChildProfileIdSchema, ParentDeviceIdSchema, ParentDeviceLabelSchema } from './reference-primitives';
import {
  LanPairingDeviceReachabilitySchema,
  LanPairingNetworkModeSchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
  LanPairingTrustStateSchema,
} from './lan-pairing-values';
import { DeviceRuntimeRoleSchema, DeviceRuntimeRoleStateSchema, DeviceRuntimeRouteStateSchema } from './device-roles';

const NonEmptyHouseholdDeviceText = Schema.String.pipe(Schema.minLength(1));

export const HouseholdDevicePlatformSchema = withParser(
  Schema.Literal('windows', 'linux', 'macos', 'android', 'ios', 'router', 'unknown')
);

export const HouseholdDeviceClassificationSchema = withParser(
  Schema.Literal('child-agent', 'network-infrastructure', 'unsupported-lan-device', 'unknown-lan-device')
);

export const HouseholdDeviceInventorySourceSchema = withParser(
  Schema.Literal('local-service', 'network-neighbor', 'trusted-registry')
);

export const HouseholdDeviceInventoryConfidenceSchema = withParser(
  Schema.Literal('agent-confirmed', 'mac-ip-match', 'network-neighbor', 'manual-required')
);

export const HouseholdDevicePolicyTargetSurfaceSchema = withParser(
  Schema.Literal('devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai')
);

export const HouseholdLanDeviceRefSchema = withParser(
  Schema.Struct({
    deviceId: ParentDeviceIdSchema,
    childProfileId: Schema.Union(ChildProfileIdSchema, Schema.Null),
    label: ParentDeviceLabelSchema,
    platform: HouseholdDevicePlatformSchema,
    ipAddress: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), { default: () => null }),
    macAddress: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), { default: () => null }),
    hostname: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), { default: () => null }),
    networkInterface: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
      default: () => null,
    }),
    agentStatus: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), { default: () => null }),
    hardwareProfile: Schema.optionalWith(
      Schema.Union(
        Schema.Struct({
          manufacturer: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
            default: () => null,
          }),
          model: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), { default: () => null }),
          cpuModel: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
            default: () => null,
          }),
          cpuCores: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
            default: () => null,
          }),
          memoryTotal: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
            default: () => null,
          }),
          gpuModel: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
            default: () => null,
          }),
          gpuDriver: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
            default: () => null,
          }),
          gpuMemory: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
            default: () => null,
          }),
          nvidiaSmi: Schema.optionalWith(Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null), {
            default: () => null,
          }),
        }),
        Schema.Null
      ),
      { default: () => null }
    ),
  })
);

export const HouseholdDeviceNetworkIdentitySchema = withParser(
  Schema.Struct({
    hostname: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    ipAddresses: Schema.Array(NonEmptyHouseholdDeviceText),
    macAddress: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    macVendor: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    networkInterfaces: Schema.Array(NonEmptyHouseholdDeviceText),
    reachability: LanPairingDeviceReachabilitySchema,
    confidence: HouseholdDeviceInventoryConfidenceSchema,
    staleAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
    offlineAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
  })
);

export const ChildAgentInventoryPacketSchema = withParser(
  Schema.Struct({
    deviceName: NonEmptyHouseholdDeviceText,
    platform: HouseholdDevicePlatformSchema,
    os: HouseholdDevicePlatformSchema,
    cpuModel: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    cpuCores: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    memoryTotal: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    gpuModel: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    gpuDriver: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    gpuMemory: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    nvidiaSmi: Schema.Union(NonEmptyHouseholdDeviceText, Schema.Null),
    networkInterfaces: Schema.Array(NonEmptyHouseholdDeviceText),
    capabilities: Schema.Array(NonEmptyHouseholdDeviceText),
    roleState: DeviceRuntimeRoleStateSchema,
    routeState: DeviceRuntimeRouteStateSchema,
    pairingTrustState: LanPairingTrustStateSchema,
  })
);

const HouseholdDeviceBaseSchema = Schema.Struct({
  schemaVersion: LanPairingSchemaVersionSchema,
  canonicalDeviceId: ParentDeviceIdSchema,
  displayName: ParentDeviceLabelSchema,
  classification: HouseholdDeviceClassificationSchema,
  roleBadges: Schema.Array(DeviceRuntimeRoleSchema),
  enrollable: Schema.Boolean,
  discoveryState: LanPairingProductionDiscoveryStateSchema,
  trustState: LanPairingTrustStateSchema,
  routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
  routeState: DeviceRuntimeRouteStateSchema,
  networkMode: LanPairingNetworkModeSchema,
  sourceLabels: Schema.Array(HouseholdDeviceInventorySourceSchema),
  networkIdentity: HouseholdDeviceNetworkIdentitySchema,
  childAgentInventory: Schema.Union(ChildAgentInventoryPacketSchema, Schema.Null),
  policyTargetSurfaces: Schema.Array(HouseholdDevicePolicyTargetSurfaceSchema),
});

type HouseholdDeviceCandidate = Infer<typeof HouseholdDeviceBaseSchema>;

export const HouseholdDeviceSpineEntrySchema = withParser(
  HouseholdDeviceBaseSchema.pipe(
    Schema.filter(
      (device) =>
        householdDeviceSpineEntryIsConsistent(device) ||
        'Expected routers and unsupported devices to be non-enrollable, child agents to expose stable target surfaces, and role badges to be unique'
    )
  )
);

function householdDeviceSpineEntryIsConsistent(device: HouseholdDeviceCandidate): boolean {
  if (new Set(device.roleBadges).size !== device.roleBadges.length) {
    return false;
  }

  if (device.classification !== 'child-agent') {
    return !device.enrollable && device.childAgentInventory === null;
  }

  return (
    device.roleBadges.includes('child-agent') &&
    device.enrollable &&
    device.policyTargetSurfaces.includes('policy') &&
    device.policyTargetSurfaces.includes('activity') &&
    device.policyTargetSurfaces.includes('network') &&
    device.policyTargetSurfaces.includes('tracking') &&
    device.policyTargetSurfaces.includes('ai')
  );
}

export type HouseholdDevicePlatform = Infer<typeof HouseholdDevicePlatformSchema>;
export type HouseholdDeviceClassification = Infer<typeof HouseholdDeviceClassificationSchema>;
export type HouseholdDeviceInventorySource = Infer<typeof HouseholdDeviceInventorySourceSchema>;
export type HouseholdDeviceInventoryConfidence = Infer<typeof HouseholdDeviceInventoryConfidenceSchema>;
export type HouseholdDevicePolicyTargetSurface = Infer<typeof HouseholdDevicePolicyTargetSurfaceSchema>;
export type HouseholdLanDeviceRef = Infer<typeof HouseholdLanDeviceRefSchema>;
export type HouseholdDeviceNetworkIdentity = Infer<typeof HouseholdDeviceNetworkIdentitySchema>;
export type ChildAgentInventoryPacket = Infer<typeof ChildAgentInventoryPacketSchema>;
export type HouseholdDeviceSpineEntry = Infer<typeof HouseholdDeviceSpineEntrySchema>;
