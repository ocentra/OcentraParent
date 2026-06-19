import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentDeviceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { ChildProfileReferenceSchema, ParentDeviceReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  LanPairingAddressRefSchema,
  LanPairingAgentPeerIdSchema,
  LanPairingChallengeIdSchema,
  LanPairingDiscoverySourceSchema,
  LanPairingDeviceReachabilitySchema,
  LanPairingEnablementStateSchema,
  LanPairingIdSchema,
  LanPairingNetworkModeSchema,
  LanPairingOriginSchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingProofDigestSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
  LanPairingTrustStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingParentAuthoritySchema,
} from './lan-pairing-values';
import { LanPairingRuntimeSupportStatusSchema } from './lan-pairing-support';
import { HouseholdDeviceSpineEntrySchema, HouseholdLanDeviceRefSchema } from './household-device-spine';
import { LanHouseholdDeviceDecisionSchema } from './lan-device-parent-actions';
import { LanDiscoverySourceMatrixSchema } from './lan-discovery-source-matrix';
import { LanProductionHouseholdProofSummarySchema } from './lan-production-household-proof';
import { LanSignedDiscoveryRelaySpineSchema } from './lan-signed-discovery-relay-spine';

export const LanPairingEnablementSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    state: LanPairingEnablementStateSchema,
    networkMode: LanPairingNetworkModeSchema,
    allowedOrigins: Schema.Array(LanPairingOriginSchema),
    updatedAt: LanPairingTimestampSchema,
  })
);

export const LanPairingDiscoveryDeviceSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    discoveredAt: LanPairingTimestampSchema,
    childProfile: ChildProfileReferenceSchema,
    childDevice: HouseholdLanDeviceRefSchema,
    agentPeerId: LanPairingAgentPeerIdSchema,
    routeId: LanPairingRouteIdSchema,
    networkMode: LanPairingNetworkModeSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    addressRef: LanPairingAddressRefSchema,
    discoveryStatus: LanPairingRuntimeSupportStatusSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
  })
);

export const LanPairingChallengeSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    challengeId: LanPairingChallengeIdSchema,
    childDevice: ParentDeviceReferenceSchema,
    parentDevice: ParentDeviceReferenceSchema,
    routeId: LanPairingRouteIdSchema,
    origin: LanPairingOriginSchema,
    issuedAt: LanPairingTimestampSchema,
    expiresAt: LanPairingTimestampSchema,
    challengeStatus: LanPairingRuntimeSupportStatusSchema,
  })
);

export const LanPairingChallengeRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    childDeviceId: ParentDeviceIdSchema,
    parentDeviceId: ParentDeviceIdSchema,
    routeId: LanPairingRouteIdSchema,
    origin: LanPairingOriginSchema,
    issuedAt: LanPairingTimestampSchema,
    expiresAt: LanPairingTimestampSchema,
  })
);

export const LanPairingProofPreviewSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    challengeId: LanPairingChallengeIdSchema,
    childDeviceId: ParentDeviceIdSchema,
    parentDeviceId: ParentDeviceIdSchema,
    routeId: LanPairingRouteIdSchema,
    origin: LanPairingOriginSchema,
    proofDigest: LanPairingProofDigestSchema,
    issuedAt: LanPairingTimestampSchema,
    expiresAt: LanPairingTimestampSchema,
    proofPreviewStatus: LanPairingRuntimeSupportStatusSchema,
  })
);

export const LanPairingProofSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    pairingId: LanPairingIdSchema,
    challengeId: LanPairingChallengeIdSchema,
    childDeviceId: ParentDeviceIdSchema,
    parentDeviceId: ParentDeviceIdSchema,
    routeId: LanPairingRouteIdSchema,
    origin: LanPairingOriginSchema,
    proofDigest: LanPairingProofDigestSchema,
    issuedAt: LanPairingTimestampSchema,
    expiresAt: LanPairingTimestampSchema,
  })
);

export const LanTrustedDeviceRegistryEntrySchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    pairingId: LanPairingIdSchema,
    childDevice: ParentDeviceReferenceSchema,
    parentDevice: ParentDeviceReferenceSchema,
    routeId: LanPairingRouteIdSchema,
    origin: LanPairingOriginSchema,
    proofDigest: LanPairingProofDigestSchema,
    trustState: LanPairingTrustStateSchema,
    trustedAt: LanPairingTimestampSchema,
    expiresAt: LanPairingTimestampSchema,
    revokedAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
  })
);

export const LanSelectedRouteTargetSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    selectedChildDeviceId: ParentDeviceIdSchema,
    routeId: LanPairingRouteIdSchema,
    pairingId: Schema.Union(LanPairingIdSchema, Schema.Null),
    trustState: LanPairingTrustStateSchema,
    networkMode: LanPairingNetworkModeSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    staleAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
    offlineAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
  })
);

export const LanBrowserAddDevicePairingRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    challengeId: LanPairingChallengeIdSchema,
    childDeviceId: ParentDeviceIdSchema,
    parentDeviceId: ParentDeviceIdSchema,
    routeId: LanPairingRouteIdSchema,
    origin: LanPairingOriginSchema,
    pairingState: LanPairingProductionDiscoveryStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    issuedAt: LanPairingTimestampSchema,
    expiresAt: LanPairingTimestampSchema,
  })
);

export const LanBrowserAddDeviceScanSummarySchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    sourceLabels: Schema.Array(NonEmptyStringSchema),
    scannedDeviceCount: Schema.Number,
    agentDeviceCount: Schema.Number,
    passiveDeviceCount: Schema.Number,
    infrastructureDeviceCount: Schema.Number,
    unsupportedDeviceCount: Schema.Number,
  })
);

export const LanSelectedDeviceReadinessSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    selectedChildDeviceId: Schema.Union(ParentDeviceIdSchema, Schema.Null),
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    pairingId: Schema.Union(LanPairingIdSchema, Schema.Null),
    trustState: LanPairingTrustStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    readyForControl: Schema.Boolean,
    staleAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
    offlineAt: Schema.Union(LanPairingTimestampSchema, Schema.Null),
  })
);

export const LanBrowserAddDeviceReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    generatedAt: LanPairingTimestampSchema,
    discoverySource: LanPairingDiscoverySourceSchema,
    addDeviceState: LanPairingProductionDiscoveryStateSchema,
    localServiceDiscoveryState: LanPairingProductionDiscoveryStateSchema,
    physicalHouseholdLanState: LanPairingProductionDiscoveryStateSchema,
    cloudRelayState: LanPairingProductionDiscoveryStateSchema,
    scanSummary: LanBrowserAddDeviceScanSummarySchema,
    discoveredDevices: Schema.Array(LanPairingDiscoveryDeviceSchema),
    canonicalHouseholdDevices: Schema.Array(HouseholdDeviceSpineEntrySchema).pipe(
      Schema.filter(
        (devices) =>
          new Set(devices.map((device) => device.canonicalDeviceId)).size === devices.length ||
          'Expected one canonical row per physical household/LAN device in the LAN add-device read model'
      )
    ),
    pairingRequests: Schema.Array(LanBrowserAddDevicePairingRequestSchema),
    trustedDeviceRegistry: Schema.Array(LanTrustedDeviceRegistryEntrySchema),
    householdDeviceDecisions: Schema.Array(LanHouseholdDeviceDecisionSchema),
    productionHouseholdProof: Schema.optionalWith(Schema.Union(LanProductionHouseholdProofSummarySchema, Schema.Null), {
      default: () => null,
    }),
    signedDiscoveryRelaySpine: Schema.optionalWith(Schema.Union(LanSignedDiscoveryRelaySpineSchema, Schema.Null), {
      default: () => null,
    }),
    lanDiscoverySourceMatrix: Schema.optionalWith(Schema.Union(LanDiscoverySourceMatrixSchema, Schema.Null), {
      default: () => null,
    }),
    trustedDeviceIds: Schema.Array(ParentDeviceIdSchema),
    revokedDeviceIds: Schema.Array(ParentDeviceIdSchema),
    selectedDeviceReadiness: LanSelectedDeviceReadinessSchema,
    controllerAuthority: LanPairingParentAuthoritySchema,
    observerAuthority: LanPairingParentAuthoritySchema,
    routeRequirementLabels: Schema.Array(NonEmptyStringSchema),
    auditCheckLabels: Schema.Array(NonEmptyStringSchema),
    honestNonClaims: Schema.Array(NonEmptyStringSchema),
  })
);

export type LanPairingEnablement = Infer<typeof LanPairingEnablementSchema>;
export type LanPairingDiscoveryDevice = Infer<typeof LanPairingDiscoveryDeviceSchema>;
export type LanPairingChallenge = Infer<typeof LanPairingChallengeSchema>;
export type LanPairingChallengeRequest = Infer<typeof LanPairingChallengeRequestSchema>;
export type LanPairingProofPreview = Infer<typeof LanPairingProofPreviewSchema>;
export type LanPairingProof = Infer<typeof LanPairingProofSchema>;
export type LanTrustedDeviceRegistryEntry = Infer<typeof LanTrustedDeviceRegistryEntrySchema>;
export type LanSelectedRouteTarget = Infer<typeof LanSelectedRouteTargetSchema>;
export type LanBrowserAddDevicePairingRequest = Infer<typeof LanBrowserAddDevicePairingRequestSchema>;
export type LanBrowserAddDeviceScanSummary = Infer<typeof LanBrowserAddDeviceScanSummarySchema>;
export type LanSelectedDeviceReadiness = Infer<typeof LanSelectedDeviceReadinessSchema>;
export type LanBrowserAddDeviceReadModel = Infer<typeof LanBrowserAddDeviceReadModelSchema>;

