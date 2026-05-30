import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentDeviceIdSchema } from './reference-primitives';
import { ChildProfileReferenceSchema, ParentDeviceReferenceSchema } from './references';
import {
  LanPairingAddressRefSchema,
  LanPairingAgentPeerIdSchema,
  LanPairingChallengeIdSchema,
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
} from './lan-pairing-values';
import { LanPairingRuntimeSupportStatusSchema } from './lan-pairing-support';

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
    childDevice: ParentDeviceReferenceSchema,
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

export type LanPairingEnablement = Infer<typeof LanPairingEnablementSchema>;
export type LanPairingDiscoveryDevice = Infer<typeof LanPairingDiscoveryDeviceSchema>;
export type LanPairingChallenge = Infer<typeof LanPairingChallengeSchema>;
export type LanPairingChallengeRequest = Infer<typeof LanPairingChallengeRequestSchema>;
export type LanPairingProofPreview = Infer<typeof LanPairingProofPreviewSchema>;
export type LanPairingProof = Infer<typeof LanPairingProofSchema>;
export type LanTrustedDeviceRegistryEntry = Infer<typeof LanTrustedDeviceRegistryEntrySchema>;
export type LanSelectedRouteTarget = Infer<typeof LanSelectedRouteTargetSchema>;
