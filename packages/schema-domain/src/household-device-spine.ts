import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { DeviceTrustState, type DeviceTrustState as FamilyDeviceTrustState } from './family-household-authority';
import { ChildProfileIdSchema, ParentDeviceIdSchema, ParentDeviceLabelSchema } from './family-reference-primitives';
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
import { LanDiscoveryEvidenceRecordSchema } from './lan-discovery-evidence';

export const HouseholdCanonicalDeviceIdSchema = brandedNonEmptyStringSchema('HouseholdCanonicalDeviceId');

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
    ipAddress: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    macAddress: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    hostname: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    networkInterface: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
      default: () => null,
    }),
    agentStatus: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    hardwareProfile: Schema.optionalWith(
      Schema.Union(
        Schema.Struct({
          manufacturer: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
            default: () => null,
          }),
          model: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          cpuModel: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
            default: () => null,
          }),
          cpuCores: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
            default: () => null,
          }),
          memoryTotal: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
            default: () => null,
          }),
          gpuModel: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
            default: () => null,
          }),
          gpuDriver: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
            default: () => null,
          }),
          gpuMemory: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
            default: () => null,
          }),
          nvidiaSmi: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), {
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
    hostname: Schema.Union(NonEmptyStringSchema, Schema.Null),
    ipAddresses: Schema.Array(NonEmptyStringSchema),
    macAddress: Schema.Union(NonEmptyStringSchema, Schema.Null),
    macVendor: Schema.Union(NonEmptyStringSchema, Schema.Null),
    networkInterfaces: Schema.Array(NonEmptyStringSchema),
    reachability: LanPairingDeviceReachabilitySchema,
    confidence: HouseholdDeviceInventoryConfidenceSchema,
    staleAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
    offlineAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
    evidenceRecords: Schema.Array(LanDiscoveryEvidenceRecordSchema).pipe(
      Schema.filter(
        (records) => records.length > 0 || 'Expected canonical LAN devices to include at least one evidence record'
      )
    ),
  })
);

export const ChildAgentInventoryPacketSchema = withParser(
  Schema.Struct({
    deviceName: NonEmptyStringSchema,
    platform: HouseholdDevicePlatformSchema,
    os: HouseholdDevicePlatformSchema,
    cpuModel: Schema.Union(NonEmptyStringSchema, Schema.Null),
    cpuCores: Schema.Union(NonEmptyStringSchema, Schema.Null),
    memoryTotal: Schema.Union(NonEmptyStringSchema, Schema.Null),
    gpuModel: Schema.Union(NonEmptyStringSchema, Schema.Null),
    gpuDriver: Schema.Union(NonEmptyStringSchema, Schema.Null),
    gpuMemory: Schema.Union(NonEmptyStringSchema, Schema.Null),
    nvidiaSmi: Schema.Union(NonEmptyStringSchema, Schema.Null),
    networkInterfaces: Schema.Array(NonEmptyStringSchema),
    capabilities: Schema.Array(NonEmptyStringSchema),
    roleState: DeviceRuntimeRoleStateSchema,
    routeState: DeviceRuntimeRouteStateSchema,
    pairingTrustState: LanPairingTrustStateSchema,
  })
);

const HouseholdDeviceBaseSchema = Schema.Struct({
  schemaVersion: LanPairingSchemaVersionSchema,
  canonicalDeviceId: HouseholdCanonicalDeviceIdSchema,
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
        'Expected routers and unsupported devices to be non-enrollable, child agents to expose stable target surfaces, child-agent trust states to stay aligned, and role badges to be unique'
    )
  )
);

export function deviceTrustStateFromLanPairingTrustState(
  trustState: Infer<typeof LanPairingTrustStateSchema>
): FamilyDeviceTrustState {
  const parsedTrustState = LanPairingTrustStateSchema.parse(trustState);

  switch (parsedTrustState) {
    case 'paired':
      return DeviceTrustState.Trusted;
    case 'revoked':
      return DeviceTrustState.Revoked;
    case 'expired':
      return DeviceTrustState.ResetRequired;
    case 'pairing':
    case 'unpaired':
      return DeviceTrustState.Pending;
  }
}

function householdDeviceSpineEntryIsConsistent(device: HouseholdDeviceCandidate): boolean {
  if (new Set(device.roleBadges).size !== device.roleBadges.length) {
    return false;
  }

  if (device.classification !== 'child-agent') {
    return !device.enrollable && device.childAgentInventory === null;
  }

  return (
    device.childAgentInventory !== null &&
    device.roleBadges.includes('child-agent') &&
    device.enrollable &&
    device.trustState === device.childAgentInventory.pairingTrustState &&
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
export type HouseholdCanonicalDeviceId = typeof HouseholdCanonicalDeviceIdSchema.Type;
export type HouseholdLanDeviceRef = Infer<typeof HouseholdLanDeviceRefSchema>;
export type HouseholdDeviceNetworkIdentity = Infer<typeof HouseholdDeviceNetworkIdentitySchema>;
export type ChildAgentInventoryPacket = Infer<typeof ChildAgentInventoryPacketSchema>;
export type HouseholdDeviceSpineEntry = Infer<typeof HouseholdDeviceSpineEntrySchema>;
