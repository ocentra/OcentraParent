import { type Infer, NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentDeviceIdSchema, AgentPeerIdSchema, AgentProtocolSchemaVersion, AgentTimestampSchema } from '@ocentra-parent/event-domain/primitives';
import { AgentLanDiscoverySourceMatrixSchema } from './lan-discovery-source-matrix';
import { AgentLanSignedDiscoveryRelaySpineSchema } from './lan-signed-discovery-relay-spine';
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

const AgentLanCanonicalDeviceIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanCanonicalDeviceId'));
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
    challengeId: NonEmptyStringSchema,
    childDeviceId: AgentDeviceIdSchema,
    parentDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptyStringSchema,
    pairingState: AgentLanPairingProductionDiscoveryStateSchema,
    rejectionReason: Schema.Union(AgentLanPairingRejectionReasonSchema, Schema.Null),
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
  })
);

export const AgentLanBrowserAddDeviceScanSummarySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    sourceLabels: Schema.Array(NonEmptyStringSchema),
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

export const AgentLanDiscoveryEvidenceSourceSchema = withParser(
  Schema.Literal(
    'local-service',
    'windows-neighbor-table',
    'dns-cache',
    'netbios',
    'trusted-registry',
    'parent-assignment',
    'child-agent-hello',
    'child-agent-heartbeat'
  )
);

export const AgentLanDiscoveryEvidenceKindSchema = withParser(
  Schema.Literal(
    'interface',
    'ip-address',
    'mac-address',
    'hostname',
    'vendor',
    'router-classification',
    'child-agent-presence',
    'trusted-registry',
    'parent-decision',
    'route'
  )
);

export const AgentLanDiscoveryEvidenceConfidenceSchema = withParser(
  Schema.Literal('confirmed', 'strong', 'weak', 'manual-required', 'rejected')
);

export const AgentLanHouseholdDeviceActionKindSchema = withParser(
  Schema.Literal('assign', 'rename', 'ignore', 'restore', 'trust')
);

const AgentLanHouseholdDeviceKindSchema = withParser(
  Schema.Literal('mobile', 'desktop', 'laptop', 'tablet', 'router', 'unknown')
);

export const AgentLanHouseholdDeviceDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    actionId: NonEmptyStringSchema,
    actionKind: AgentLanHouseholdDeviceActionKindSchema,
    canonicalDeviceId: AgentLanCanonicalDeviceIdSchema,
    childProfileId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    displayName: Schema.Union(NonEmptyStringSchema, Schema.Null),
    deviceKind: Schema.optionalWith(Schema.Union(AgentLanHouseholdDeviceKindSchema, Schema.Null), {
      default: () => null,
    }),
    parentActorId: NonEmptyStringSchema,
    decidedAt: AgentTimestampSchema,
    revokedAt: Schema.Union(AgentTimestampSchema, Schema.Null),
  })
);

export const AgentLanProductionHouseholdProofCapabilitySchema = withParser(
  Schema.Literal(
    'signed-lan-hello',
    'signed-lan-heartbeat',
    'passive-neighbor-discovery',
    'router-neighbor-discovery',
    'mdns-name-discovery',
    'ssdp-name-discovery',
    'router-dhcp-name-discovery',
    'trusted-registry',
    'parent-assignment',
    'parent-rename',
    'parent-ignore',
    'parent-revocation',
    'route-custody',
    'stale-selected-device',
    'offline-selected-device',
    'relay-route',
    'cache-route',
    'second-physical-child-agent',
    'android-child-agent-parity',
    'ios-child-agent-parity',
    'store-signing'
  )
);

export const AgentLanProductionHouseholdProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'not-implemented')
);

export const AgentLanProductionHouseholdProofRuntimeOwnerSchema = withParser(
  Schema.Literal('parent-domain-contract', 'agent-protocol', 'rust-service-read-model', 'proof-harness', 'manual-proof')
);

export const AgentLanProductionHouseholdProofStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    capability: AgentLanProductionHouseholdProofCapabilitySchema,
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
    proofState: AgentLanProductionHouseholdProofStateSchema,
    runtimeOwner: AgentLanProductionHouseholdProofRuntimeOwnerSchema,
    evidenceLabel: NonEmptyStringSchema,
    requiredArtifactSummary: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);

export const AgentLanProductionHouseholdProofSummarySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    generatedAt: AgentTimestampSchema,
    statusRows: Schema.Array(AgentLanProductionHouseholdProofStatusSchema),
    manualProofRequired: Schema.Array(AgentLanProductionHouseholdProofCapabilitySchema),
    notImplemented: Schema.Array(AgentLanProductionHouseholdProofCapabilitySchema),
    claimsProved: Schema.Array(NonEmptyStringSchema),
    claimsNotProved: Schema.Array(NonEmptyStringSchema),
  })
);

export const AgentLanDiscoveryEvidenceRecordSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    evidenceId: NonEmptyStringSchema,
    source: AgentLanDiscoveryEvidenceSourceSchema,
    evidenceKind: AgentLanDiscoveryEvidenceKindSchema,
    deviceId: AgentDeviceIdSchema,
    value: NonEmptyStringSchema,
    normalizedValue: NonEmptyStringSchema,
    firstSeenAt: AgentTimestampSchema,
    lastSeenAt: AgentTimestampSchema,
    expiresAt: Schema.Union(AgentTimestampSchema, Schema.Null),
    confidence: AgentLanDiscoveryEvidenceConfidenceSchema,
    mergeKey: NonEmptyStringSchema,
    note: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
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
    hostname: Schema.Union(NonEmptyStringSchema, Schema.Null),
    ipAddresses: Schema.Array(NonEmptyStringSchema),
    macAddress: Schema.Union(NonEmptyStringSchema, Schema.Null),
    macVendor: Schema.Union(NonEmptyStringSchema, Schema.Null),
    networkInterfaces: Schema.Array(NonEmptyStringSchema),
    reachability: AgentLanSelectedDeviceReachabilitySchema,
    confidence: AgentLanCanonicalHouseholdDeviceConfidenceSchema,
    staleAt: Schema.Union(AgentTimestampSchema, Schema.Null),
    offlineAt: Schema.Union(AgentTimestampSchema, Schema.Null),
    evidenceRecords: Schema.Array(AgentLanDiscoveryEvidenceRecordSchema).pipe(
      Schema.filter(
        (records) => records.length > 0 || 'Expected agent canonical LAN devices to include evidence records'
      )
    ),
  })
);

export const AgentLanChildAgentInventoryPacketSchema = withParser(
  Schema.Struct({
    deviceName: NonEmptyStringSchema,
    platform: NonEmptyStringSchema,
    os: NonEmptyStringSchema,
    cpuModel: Schema.Union(NonEmptyStringSchema, Schema.Null),
    cpuCores: Schema.Union(NonEmptyStringSchema, Schema.Null),
    memoryTotal: Schema.Union(NonEmptyStringSchema, Schema.Null),
    gpuModel: Schema.Union(NonEmptyStringSchema, Schema.Null),
    gpuDriver: Schema.Union(NonEmptyStringSchema, Schema.Null),
    gpuMemory: Schema.Union(NonEmptyStringSchema, Schema.Null),
    nvidiaSmi: Schema.Union(NonEmptyStringSchema, Schema.Null),
    networkInterfaces: Schema.Array(NonEmptyStringSchema),
    capabilities: Schema.Array(NonEmptyStringSchema),
    roleState: AgentLanCanonicalHouseholdRoleStateSchema,
    routeState: AgentLanCanonicalHouseholdRouteStateSchema,
    pairingTrustState: AgentLanSelectedRouteTrustStateSchema,
  })
);

export const AgentLanCanonicalHouseholdDeviceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    canonicalDeviceId: AgentLanCanonicalDeviceIdSchema,
    displayName: NonEmptyStringSchema,
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
    origin: NonEmptyStringSchema,
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
    householdDeviceDecisions: Schema.Array(AgentLanHouseholdDeviceDecisionSchema),
    productionHouseholdProof: Schema.optionalWith(
      Schema.Union(AgentLanProductionHouseholdProofSummarySchema, Schema.Null),
      { default: () => null }
    ),
    signedDiscoveryRelaySpine: Schema.optionalWith(Schema.Union(AgentLanSignedDiscoveryRelaySpineSchema, Schema.Null), {
      default: () => null,
    }),
    lanDiscoverySourceMatrix: Schema.optionalWith(Schema.Union(AgentLanDiscoverySourceMatrixSchema, Schema.Null), {
      default: () => null,
    }),
    trustedDeviceIds: Schema.Array(AgentDeviceIdSchema),
    revokedDeviceIds: Schema.Array(AgentDeviceIdSchema),
    selectedDeviceReadiness: AgentLanSelectedDeviceReadinessSchema,
    controllerAuthority: AgentLanPairingParentAuthoritySchema,
    observerAuthority: AgentLanPairingParentAuthoritySchema,
    routeRequirementLabels: Schema.Array(NonEmptyStringSchema),
    auditCheckLabels: Schema.Array(NonEmptyStringSchema),
    honestNonClaims: Schema.Array(NonEmptyStringSchema),
  })
);

export type AgentLanPairingDiscoverySource = Infer<typeof AgentLanPairingDiscoverySourceSchema>;
export type AgentLanHouseholdDeviceActionKind = Infer<typeof AgentLanHouseholdDeviceActionKindSchema>;
export type AgentLanBrowserAddDeviceDiscoveryDevice = Infer<typeof AgentLanBrowserAddDeviceDiscoveryDeviceSchema>;
export type AgentLanBrowserAddDevicePairingRequest = Infer<typeof AgentLanBrowserAddDevicePairingRequestSchema>;
export type AgentLanBrowserAddDeviceScanSummary = Infer<typeof AgentLanBrowserAddDeviceScanSummarySchema>;
export type AgentLanSelectedDeviceReadiness = Infer<typeof AgentLanSelectedDeviceReadinessSchema>;
export type AgentLanProductionHouseholdProofCapability = Infer<typeof AgentLanProductionHouseholdProofCapabilitySchema>;
export type AgentLanProductionHouseholdProofState = Infer<typeof AgentLanProductionHouseholdProofStateSchema>;
export type AgentLanProductionHouseholdProofRuntimeOwner = Infer<
  typeof AgentLanProductionHouseholdProofRuntimeOwnerSchema
>;
export type AgentLanProductionHouseholdProofStatus = Infer<typeof AgentLanProductionHouseholdProofStatusSchema>;
export type AgentLanProductionHouseholdProofSummary = Infer<typeof AgentLanProductionHouseholdProofSummarySchema>;
export type AgentLanDiscoveryEvidenceRecord = Infer<typeof AgentLanDiscoveryEvidenceRecordSchema>;
export type AgentLanHouseholdDeviceDecision = Infer<typeof AgentLanHouseholdDeviceDecisionSchema>;
export type AgentLanCanonicalHouseholdDevice = Infer<typeof AgentLanCanonicalHouseholdDeviceSchema>;
export type AgentLanTrustedDeviceRegistryEntry = Infer<typeof AgentLanTrustedDeviceRegistryEntrySchema>;
export type AgentLanBrowserAddDeviceReadModel = Infer<typeof AgentLanBrowserAddDeviceReadModelSchema>;
