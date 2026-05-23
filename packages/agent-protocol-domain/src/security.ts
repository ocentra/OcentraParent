import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentDeviceIdSchema, AgentPeerIdSchema, AgentRouteSchema, AgentTimestampSchema } from './primitives';

const NonEmptySecurityText = Schema.String.pipe(Schema.minLength(1));

export const AgentPairingIdSchema = NonEmptySecurityText.pipe(Schema.brand('AgentPairingId'));
export const AgentPairingTokenHashSchema = NonEmptySecurityText.pipe(Schema.brand('AgentPairingTokenHash'));

export const AgentPairingStateSchema = withParser(
  Schema.Literal('unauthenticated', 'unpaired', 'pairing', 'paired', 'revoked')
);

export const AgentLanSelectedDeviceReachabilitySchema = withParser(Schema.Literal('online', 'offline', 'stale'));

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
export type AgentPairingProof = Infer<typeof AgentPairingProofSchema>;
export type AgentRouteSecurityPolicy = Infer<typeof AgentRouteSecurityPolicySchema>;
