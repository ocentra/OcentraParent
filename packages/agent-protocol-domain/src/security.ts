import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AgentDeviceIdSchema,
  AgentPeerIdSchema,
  AgentPlatformSchema,
  AgentProtocolSchemaVersion,
  AgentRouteSchema,
  AgentTimestampSchema,
} from './primitives';

const NonEmptySecurityText = Schema.String.pipe(Schema.minLength(1));

export const AgentPairingIdSchema = NonEmptySecurityText.pipe(Schema.brand('AgentPairingId'));
export const AgentPairingTokenHashSchema = NonEmptySecurityText.pipe(Schema.brand('AgentPairingTokenHash'));
export const AgentLanPairingAddressRefSchema = NonEmptySecurityText.pipe(Schema.brand('AgentLanPairingAddressRef'));
export const AgentLanPairingChallengeIdSchema = NonEmptySecurityText.pipe(Schema.brand('AgentLanPairingChallengeId'));
export const AgentLanPairingProofDigestSchema = NonEmptySecurityText.pipe(Schema.brand('AgentLanPairingProofDigest'));
export const AgentLanPairingRouteIdSchema = NonEmptySecurityText.pipe(Schema.brand('AgentLanPairingRouteId'));

export const AgentPairingStateSchema = withParser(
  Schema.Literal('unauthenticated', 'unpaired', 'pairing', 'paired', 'revoked')
);

export const AgentLanSelectedDeviceReachabilitySchema = withParser(Schema.Literal('online', 'offline', 'stale'));
export const AgentLanPairingNetworkModeSchema = withParser(Schema.Literal('loopback', 'local-network'));
export const AgentLanPairingRuntimeSupportStatusSchema = withParser(Schema.Literal('planned-unsupported'));

export const AgentLanPairingDeviceRefSchema = withParser(
  Schema.Struct({
    deviceId: AgentDeviceIdSchema,
    childProfileId: Schema.Union(NonEmptySecurityText, Schema.Null),
    label: NonEmptySecurityText,
    platform: AgentPlatformSchema,
  })
);

export const AgentLanPairingDiscoveryDeviceSchema = withParser(
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
  })
);

export const AgentLanPairingChallengeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    challengeId: AgentLanPairingChallengeIdSchema,
    childDevice: AgentLanPairingDeviceRefSchema,
    parentDevice: AgentLanPairingDeviceRefSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptySecurityText,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
    challengeStatus: AgentLanPairingRuntimeSupportStatusSchema,
  })
);

export const AgentLanPairingProofPreviewSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    challengeId: AgentLanPairingChallengeIdSchema,
    childDeviceId: AgentDeviceIdSchema,
    parentDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptySecurityText,
    proofDigest: AgentLanPairingProofDigestSchema,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
    proofPreviewStatus: AgentLanPairingRuntimeSupportStatusSchema,
  })
);

export const AgentPairingProofSchema = withParser(
  Schema.Struct({
    pairingId: AgentPairingIdSchema,
    deviceId: AgentDeviceIdSchema,
    parentPeerId: AgentPeerIdSchema,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
    tokenHash: AgentPairingTokenHashSchema,
  })
);

export const AgentRouteSecurityPolicySchema = withParser(
  Schema.Struct({
    route: AgentRouteSchema,
    requiresPairing: Schema.Boolean,
    allowsAnonymousControl: Schema.Boolean,
  })
);

export type AgentPairingId = typeof AgentPairingIdSchema.Type;
export type AgentPairingTokenHash = typeof AgentPairingTokenHashSchema.Type;
export type AgentPairingState = Infer<typeof AgentPairingStateSchema>;
export type AgentLanSelectedDeviceReachability = Infer<typeof AgentLanSelectedDeviceReachabilitySchema>;
export type AgentLanPairingAddressRef = typeof AgentLanPairingAddressRefSchema.Type;
export type AgentLanPairingChallengeId = typeof AgentLanPairingChallengeIdSchema.Type;
export type AgentLanPairingDeviceRef = Infer<typeof AgentLanPairingDeviceRefSchema>;
export type AgentLanPairingDiscoveryDevice = Infer<typeof AgentLanPairingDiscoveryDeviceSchema>;
export type AgentLanPairingChallenge = Infer<typeof AgentLanPairingChallengeSchema>;
export type AgentLanPairingNetworkMode = Infer<typeof AgentLanPairingNetworkModeSchema>;
export type AgentLanPairingProofDigest = typeof AgentLanPairingProofDigestSchema.Type;
export type AgentLanPairingProofPreview = Infer<typeof AgentLanPairingProofPreviewSchema>;
export type AgentLanPairingRouteId = typeof AgentLanPairingRouteIdSchema.Type;
export type AgentLanPairingRuntimeSupportStatus = Infer<typeof AgentLanPairingRuntimeSupportStatusSchema>;
export type AgentPairingProof = Infer<typeof AgentPairingProofSchema>;
export type AgentRouteSecurityPolicy = Infer<typeof AgentRouteSecurityPolicySchema>;
