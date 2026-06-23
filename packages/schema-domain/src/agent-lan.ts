import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import {
  AgentDeviceIdSchema,
  AgentEventIdSchema,
  AgentPeerIdSchema,
  AgentProtocolSchemaVersion,
  AgentTimestampSchema,
} from './event-primitives';
import {
  AgentLanPairingAddressRefSchema,
  AgentLanPairingDeviceRefSchema,
  AgentLanPairingIntentKindSchema,
  AgentLanPairingNetworkModeSchema,
  AgentLanPairingParentAuthoritySchema,
  AgentLanPairingProductionDiscoveryStateSchema,
  AgentLanPairingProofDigestSchema,
  AgentLanPairingResponseStateSchema,
  AgentLanPairingRouteIdSchema,
  AgentLanPairingRuntimeSupportStatusSchema,
  AgentLanSelectedDeviceReachabilitySchema,
  AgentPairingIdSchema,
  AgentPairingTokenHashSchema,
} from './agent-lan-primitives';
import { ParentEvidenceReferenceIdSchema, ParentEvidenceReferenceKindSchema } from './family-reference-primitives';
import { LanPairingAuditEventTypeSchema as AgentLanPairingAuditEventTypeSchema } from './lan-pairing-values';

export const AgentLanPairingChallengeIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingChallengeId'));
export const AgentLanPairingIntentIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingIntentId'));
const AgentLanPairingControllerLeaseIdSchema = NonEmptyStringSchema.pipe(
  Schema.brand('AgentLanPairingControllerLeaseId')
);
const AgentLanPairingEvidenceReferenceIdSchema = ParentEvidenceReferenceIdSchema;
const AgentLanPairingEvidenceReferenceKindSchema = ParentEvidenceReferenceKindSchema;

export const AgentPairingStateSchema = withParser(
  Schema.Literal('unauthenticated', 'unpaired', 'pairing', 'paired', 'revoked')
);

export const AgentLanSelectedRouteTrustStateSchema = withParser(
  Schema.Literal('unpaired', 'pairing', 'paired', 'revoked', 'expired')
);
export const AgentLanPairingRejectionReasonSchema = withParser(
  Schema.Literal(
    'anonymous',
    'wrong-origin',
    'wrong-device',
    'expired',
    'replayed',
    'malformed',
    'stale',
    'offline',
    'revoked',
    'local-network-disabled',
    'unsupported-route',
    'unselected-device',
    'controller-lease-missing',
    'controller-lease-expired',
    'wrong-controller',
    'observer-read-only',
    'takeover-denied',
    'lan-ai-provider-unavailable',
    'lan-ai-job-unauthorized'
  )
);

const AgentLanPairingEvidenceReferenceSchema = withParser(
  Schema.Struct({
    evidenceReferenceId: AgentLanPairingEvidenceReferenceIdSchema,
    kind: AgentLanPairingEvidenceReferenceKindSchema,
    observedAt: AgentTimestampSchema,
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
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
  })
);

export const AgentLanPairingChallengeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    challengeId: AgentLanPairingChallengeIdSchema,
    childDevice: AgentLanPairingDeviceRefSchema,
    parentDevice: AgentLanPairingDeviceRefSchema,
    routeId: AgentLanPairingRouteIdSchema,
    origin: NonEmptyStringSchema,
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
    origin: NonEmptyStringSchema,
    proofDigest: AgentLanPairingProofDigestSchema,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
    proofPreviewStatus: AgentLanPairingRuntimeSupportStatusSchema,
  })
);

export const AgentLanParentIntentEnvelopeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    intentId: AgentLanPairingIntentIdSchema,
    intentKind: AgentLanPairingIntentKindSchema,
    targetChildDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    pairingId: AgentPairingIdSchema,
    proofDigest: AgentLanPairingProofDigestSchema,
    origin: NonEmptyStringSchema,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
    controllerLeaseId: AgentLanPairingControllerLeaseIdSchema,
    controllerDeviceId: AgentDeviceIdSchema,
    parentActorId: NonEmptyStringSchema,
    parentAuthority: AgentLanPairingParentAuthoritySchema,
    controllerLeaseIssuedAt: AgentTimestampSchema,
    controllerLeaseExpiresAt: AgentTimestampSchema,
    evidenceReferences: Schema.Array(AgentLanPairingEvidenceReferenceSchema),
  })
);

export const AgentLanPairingAuditEventSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    auditEventId: AgentEventIdSchema,
    eventType: AgentLanPairingAuditEventTypeSchema,
    pairingId: Schema.Union(AgentPairingIdSchema, Schema.Null),
    intentId: Schema.Union(AgentLanPairingIntentIdSchema, Schema.Null),
    childDeviceId: Schema.Union(AgentDeviceIdSchema, Schema.Null),
    parentDeviceId: Schema.Union(AgentDeviceIdSchema, Schema.Null),
    controllerLeaseId: Schema.Union(AgentLanPairingControllerLeaseIdSchema, Schema.Null),
    controllerDeviceId: Schema.Union(AgentDeviceIdSchema, Schema.Null),
    parentActorId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    routeId: AgentLanPairingRouteIdSchema,
    origin: Schema.Union(NonEmptyStringSchema, Schema.Null),
    rejectionReason: Schema.Union(AgentLanPairingRejectionReasonSchema, Schema.Null),
    observedAt: AgentTimestampSchema,
    evidenceReferences: Schema.Array(AgentLanPairingEvidenceReferenceSchema),
  })
);

export const AgentLanChildAgentResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    intentId: AgentLanPairingIntentIdSchema,
    targetChildDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    state: AgentLanPairingResponseStateSchema,
    rejectionReason: Schema.Union(AgentLanPairingRejectionReasonSchema, Schema.Null),
    auditEventId: AgentEventIdSchema,
    respondedAt: AgentTimestampSchema,
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

export type AgentLanPairingChallengeId = typeof AgentLanPairingChallengeIdSchema.Type;
export type AgentLanPairingIntentId = typeof AgentLanPairingIntentIdSchema.Type;
export type AgentLanPairingDiscoveryDevice = Infer<typeof AgentLanPairingDiscoveryDeviceSchema>;
export type AgentLanPairingChallenge = Infer<typeof AgentLanPairingChallengeSchema>;
export type AgentLanPairingProofPreview = Infer<typeof AgentLanPairingProofPreviewSchema>;
export type AgentLanPairingRejectionReason = Infer<typeof AgentLanPairingRejectionReasonSchema>;
export type AgentLanChildAgentResponse = Infer<typeof AgentLanChildAgentResponseSchema>;
export type AgentLanParentIntentEnvelope = Infer<typeof AgentLanParentIntentEnvelopeSchema>;
export type AgentLanSelectedRouteTrustState = Infer<typeof AgentLanSelectedRouteTrustStateSchema>;
export type AgentPairingState = Infer<typeof AgentPairingStateSchema>;
export type AgentPairingProof = Infer<typeof AgentPairingProofSchema>;
