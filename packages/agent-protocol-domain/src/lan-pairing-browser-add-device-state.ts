import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentDeviceIdSchema, AgentPeerIdSchema, AgentProtocolSchemaVersion, AgentTimestampSchema } from './primitives';
import {
  AgentLanPairingAddressRefSchema,
  AgentLanPairingDeviceRefSchema,
  AgentLanPairingNetworkModeSchema,
  AgentLanPairingParentAuthoritySchema,
  AgentLanPairingProductionDiscoveryStateSchema,
  AgentLanPairingRejectionReasonSchema,
  AgentLanPairingRouteIdSchema,
  AgentLanPairingRuntimeSupportStatusSchema,
  AgentLanPairingProofDigestSchema,
  AgentLanSelectedDeviceReachabilitySchema,
  AgentLanSelectedRouteTrustStateSchema,
  AgentPairingIdSchema,
} from './security';

const NonEmptyLanAddDeviceText = Schema.String.pipe(Schema.minLength(1));
export const AgentLanPairingDiscoverySourceSchema = withParser(
  Schema.Literal('local-service', 'physical-household-lan', 'cloud-relay')
);

export const AgentLanBrowserAddDeviceDiscoveryDeviceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    discoveredAt: AgentTimestampSchema,
    childDevice: AgentLanPairingDeviceRefSchema,
    agentPeerId: AgentPeerIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    networkMode: AgentLanPairingNetworkModeSchema,
    reachability: AgentLanSelectedDeviceReachabilitySchema,
    addressRef: AgentLanPairingAddressRefSchema,
    discoveryStatus: AgentLanPairingRuntimeSupportStatusSchema,
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
  })
);

export const AgentLanBrowserAddDevicePairingRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    challengeId: NonEmptyLanAddDeviceText,
    childDeviceId: AgentDeviceIdSchema,
    parentDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptyLanAddDeviceText,
    pairingState: AgentLanPairingProductionDiscoveryStateSchema,
    rejectionReason: Schema.Union(AgentLanPairingRejectionReasonSchema, Schema.Null),
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
  })
);

export const AgentLanBrowserAddDeviceScanSummarySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    sourceLabels: Schema.Array(NonEmptyLanAddDeviceText),
    scannedDeviceCount: Schema.Number,
    agentDeviceCount: Schema.Number,
    passiveDeviceCount: Schema.Number,
    infrastructureDeviceCount: Schema.Number,
    unsupportedDeviceCount: Schema.Number,
  })
);

export const AgentLanSelectedDeviceReadinessSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    selectedChildDeviceId: Schema.Union(AgentDeviceIdSchema, Schema.Null),
    routeId: Schema.Union(AgentLanPairingRouteIdSchema, Schema.Null),
    pairingId: Schema.Union(AgentPairingIdSchema, Schema.Null),
    trustState: AgentLanSelectedRouteTrustStateSchema,
    reachability: AgentLanSelectedDeviceReachabilitySchema,
    readyForControl: Schema.Boolean,
    staleAt: Schema.Union(AgentTimestampSchema, Schema.Null),
    offlineAt: Schema.Union(AgentTimestampSchema, Schema.Null),
  })
);

export const AgentLanCanonicalHouseholdDeviceRoleSchema = withParser(
  Schema.Literal('parent-controller', 'parent-observer', 'child-agent', 'portal', 'ai-provider')
);

export const AgentLanCanonicalHouseholdDeviceClassificationSchema = withParser(
  Schema.Literal('child-agent', 'network-infrastructure', 'unsupported-lan-device', 'unknown-lan-device')
);

export const AgentLanCanonicalHouseholdDeviceSourceSchema = withParser(
  Schema.Literal('local-service', 'network-neighbor', 'trusted-registry')
);

export const AgentLanCanonicalHouseholdDeviceConfidenceSchema = withParser(
  Schema.Literal('agent-confirmed', 'mac-ip-match', 'network-neighbor', 'manual-required')
);

export const AgentLanCanonicalHouseholdRouteStateSchema = withParser(
  Schema.Literal('localhost', 'local-network', 'manual-required', 'unavailable')
);

export const AgentLanCanonicalHouseholdRoleStateSchema = withParser(
  Schema.Literal('implemented', 'scaffold', 'manual-required', 'unavailable')
);

export const AgentLanCanonicalHouseholdSurfaceSchema = withParser(
  Schema.Literal('devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai')
);

export const AgentLanCanonicalHouseholdNetworkIdentitySchema = withParser(
  Schema.Struct({
    hostname: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    ipAddresses: Schema.Array(NonEmptyLanAddDeviceText),
    macAddress: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    macVendor: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    networkInterfaces: Schema.Array(NonEmptyLanAddDeviceText),
    reachability: AgentLanSelectedDeviceReachabilitySchema,
    confidence: AgentLanCanonicalHouseholdDeviceConfidenceSchema,
    staleAt: Schema.Union(AgentTimestampSchema, Schema.Null),
    offlineAt: Schema.Union(AgentTimestampSchema, Schema.Null),
  })
);

export const AgentLanChildAgentInventoryPacketSchema = withParser(
  Schema.Struct({
    deviceName: NonEmptyLanAddDeviceText,
    platform: NonEmptyLanAddDeviceText,
    os: NonEmptyLanAddDeviceText,
    cpuModel: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    cpuCores: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    memoryTotal: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    gpuModel: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    gpuDriver: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    gpuMemory: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    nvidiaSmi: Schema.Union(NonEmptyLanAddDeviceText, Schema.Null),
    networkInterfaces: Schema.Array(NonEmptyLanAddDeviceText),
    capabilities: Schema.Array(NonEmptyLanAddDeviceText),
    roleState: AgentLanCanonicalHouseholdRoleStateSchema,
    routeState: AgentLanCanonicalHouseholdRouteStateSchema,
    pairingTrustState: AgentLanSelectedRouteTrustStateSchema,
  })
);

export const AgentLanCanonicalHouseholdDeviceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    canonicalDeviceId: AgentDeviceIdSchema,
    displayName: NonEmptyLanAddDeviceText,
    classification: AgentLanCanonicalHouseholdDeviceClassificationSchema,
    roleBadges: Schema.Array(AgentLanCanonicalHouseholdDeviceRoleSchema),
    enrollable: Schema.Boolean,
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
    trustState: AgentLanSelectedRouteTrustStateSchema,
    routeId: Schema.Union(AgentLanPairingRouteIdSchema, Schema.Null),
    routeState: AgentLanCanonicalHouseholdRouteStateSchema,
    networkMode: AgentLanPairingNetworkModeSchema,
    sourceLabels: Schema.Array(AgentLanCanonicalHouseholdDeviceSourceSchema),
    networkIdentity: AgentLanCanonicalHouseholdNetworkIdentitySchema,
    childAgentInventory: Schema.Union(AgentLanChildAgentInventoryPacketSchema, Schema.Null),
    policyTargetSurfaces: Schema.Array(AgentLanCanonicalHouseholdSurfaceSchema),
  })
);

export const AgentLanTrustedDeviceRegistryEntrySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    pairingId: AgentPairingIdSchema,
    childDevice: AgentLanPairingDeviceRefSchema,
    parentDevice: AgentLanPairingDeviceRefSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptyLanAddDeviceText,
    proofDigest: AgentLanPairingProofDigestSchema,
    trustState: AgentLanSelectedRouteTrustStateSchema,
    trustedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
    revokedAt: Schema.Union(AgentTimestampSchema, Schema.Null),
  })
);

export const AgentLanBrowserAddDeviceReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    generatedAt: AgentTimestampSchema,
    discoverySource: AgentLanPairingDiscoverySourceSchema,
    addDeviceState: AgentLanPairingProductionDiscoveryStateSchema,
    localServiceDiscoveryState: AgentLanPairingProductionDiscoveryStateSchema,
    physicalHouseholdLanState: AgentLanPairingProductionDiscoveryStateSchema,
    cloudRelayState: AgentLanPairingProductionDiscoveryStateSchema,
    scanSummary: AgentLanBrowserAddDeviceScanSummarySchema,
    discoveredDevices: Schema.Array(AgentLanBrowserAddDeviceDiscoveryDeviceSchema),
    canonicalHouseholdDevices: Schema.Array(AgentLanCanonicalHouseholdDeviceSchema).pipe(
      Schema.filter(
        (devices) =>
          new Set(devices.map((device) => device.canonicalDeviceId)).size === devices.length ||
          'Expected one canonical row per physical household/LAN device in the agent add-device read model'
      )
    ),
    pairingRequests: Schema.Array(AgentLanBrowserAddDevicePairingRequestSchema),
    trustedDeviceRegistry: Schema.Array(AgentLanTrustedDeviceRegistryEntrySchema),
    trustedDeviceIds: Schema.Array(AgentDeviceIdSchema),
    revokedDeviceIds: Schema.Array(AgentDeviceIdSchema),
    selectedDeviceReadiness: AgentLanSelectedDeviceReadinessSchema,
    controllerAuthority: AgentLanPairingParentAuthoritySchema,
    observerAuthority: AgentLanPairingParentAuthoritySchema,
    routeRequirementLabels: Schema.Array(NonEmptyLanAddDeviceText),
    auditCheckLabels: Schema.Array(NonEmptyLanAddDeviceText),
    honestNonClaims: Schema.Array(NonEmptyLanAddDeviceText),
  })
);

export type AgentLanPairingDiscoverySource = Infer<typeof AgentLanPairingDiscoverySourceSchema>;
export type AgentLanBrowserAddDeviceDiscoveryDevice = Infer<typeof AgentLanBrowserAddDeviceDiscoveryDeviceSchema>;
export type AgentLanBrowserAddDevicePairingRequest = Infer<typeof AgentLanBrowserAddDevicePairingRequestSchema>;
export type AgentLanBrowserAddDeviceScanSummary = Infer<typeof AgentLanBrowserAddDeviceScanSummarySchema>;
export type AgentLanSelectedDeviceReadiness = Infer<typeof AgentLanSelectedDeviceReadinessSchema>;
export type AgentLanCanonicalHouseholdDevice = Infer<typeof AgentLanCanonicalHouseholdDeviceSchema>;
export type AgentLanTrustedDeviceRegistryEntry = Infer<typeof AgentLanTrustedDeviceRegistryEntrySchema>;
export type AgentLanBrowserAddDeviceReadModel = Infer<typeof AgentLanBrowserAddDeviceReadModelSchema>;
