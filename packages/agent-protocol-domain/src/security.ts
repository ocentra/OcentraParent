import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AgentDeviceIdSchema,
  AgentEventIdSchema,
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
const AgentLanPairingEvidenceReferenceIdSchema = NonEmptySecurityText.pipe(
  Schema.brand('AgentLanPairingEvidenceReferenceId')
);
export const AgentLanPairingIntentIdSchema = NonEmptySecurityText.pipe(Schema.brand('AgentLanPairingIntentId'));
export const AgentLanPairingProofDigestSchema = NonEmptySecurityText.pipe(Schema.brand('AgentLanPairingProofDigest'));
export const AgentLanPairingRouteIdSchema = NonEmptySecurityText.pipe(Schema.brand('AgentLanPairingRouteId'));
const AgentLanPairingControllerLeaseIdSchema = NonEmptySecurityText.pipe(
  Schema.brand('AgentLanPairingControllerLeaseId')
);

export const AgentPairingStateSchema = withParser(
  Schema.Literal('unauthenticated', 'unpaired', 'pairing', 'paired', 'revoked')
);

export const AgentLanSelectedDeviceReachabilitySchema = withParser(Schema.Literal('online', 'offline', 'stale'));
export const AgentLanPairingNetworkModeSchema = withParser(Schema.Literal('loopback', 'local-network'));
export const AgentLanPairingParentAuthoritySchema = withParser(Schema.Literal('active-controller', 'observer'));
export const AgentLanPairingRuntimeSupportStatusSchema = withParser(
  Schema.Literal('planned-unsupported', 'websocket-direct')
);
export const AgentLanPairingIntentKindSchema = withParser(
  Schema.Literal(
    'health-query',
    'rule-query',
    'rule-update',
    'approval-decision',
    'configuration-update',
    'controller-lease-renew',
    'controller-lease-release',
    'controller-lease-takeover',
    'lan-ai-provider-status',
    'lan-ai-job-submit'
  )
);
export const AgentLanPairingResponseStateSchema = withParser(
  Schema.Literal('accepted', 'rejected', 'queued', 'completed', 'degraded')
);
const AgentLanPairingEvidenceReferenceKindSchema = withParser(
  Schema.Literal('journal-event', 'query-store-summary', 'activity-event', 'policy-decision', 'local-ai-result')
);
const AgentLanPairingAuditEventTypeSchema = withParser(
  Schema.Literal(
    'discovery-advertised',
    'pairing-challenge-issued',
    'pairing-proof-accepted',
    'pairing-proof-rejected',
    'control-accepted',
    'control-rejected',
    'route-selected',
    'pairing-revoked',
    'selected-device-changed',
    'controller-lease-renewed',
    'controller-lease-released',
    'controller-lease-takeover-accepted',
    'controller-lease-takeover-rejected',
    'lan-ai-provider-advertised',
    'lan-ai-job-accepted',
    'lan-ai-job-rejected',
    'lan-ai-job-completed',
    'lan-ai-job-degraded'
  )
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

export const AgentLanPairingDeviceRefSchema = withParser(
  Schema.Struct({
    deviceId: AgentDeviceIdSchema,
    childProfileId: Schema.Union(NonEmptySecurityText, Schema.Null),
    label: NonEmptySecurityText,
    platform: AgentPlatformSchema,
  })
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

export const AgentLanParentIntentEnvelopeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    intentId: AgentLanPairingIntentIdSchema,
    intentKind: AgentLanPairingIntentKindSchema,
    targetChildDeviceId: AgentDeviceIdSchema,
    routeId: AgentLanPairingRouteIdSchema,
    pairingId: AgentPairingIdSchema,
    proofDigest: AgentLanPairingProofDigestSchema,
    origin: NonEmptySecurityText,
    issuedAt: AgentTimestampSchema,
    expiresAt: AgentTimestampSchema,
    controllerLeaseId: AgentLanPairingControllerLeaseIdSchema,
    controllerDeviceId: AgentDeviceIdSchema,
    parentActorId: NonEmptySecurityText,
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
    parentActorId: Schema.Union(NonEmptySecurityText, Schema.Null),
    routeId: AgentLanPairingRouteIdSchema,
    origin: Schema.Union(NonEmptySecurityText, Schema.Null),
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

export const AgentRouteSecurityPolicySchema = withParser(
  Schema.Struct({
    route: AgentRouteSchema,
    requiresPairing: Schema.Boolean,
    allowsAnonymousControl: Schema.Boolean,
  })
);

type AgentPairingId = typeof AgentPairingIdSchema.Type;
type AgentPairingTokenHash = typeof AgentPairingTokenHashSchema.Type;
type AgentPairingState = Infer<typeof AgentPairingStateSchema>;
type AgentLanSelectedDeviceReachability = Infer<typeof AgentLanSelectedDeviceReachabilitySchema>;
type AgentLanPairingAddressRef = typeof AgentLanPairingAddressRefSchema.Type;
type AgentLanPairingChallengeId = typeof AgentLanPairingChallengeIdSchema.Type;
type AgentLanPairingIntentId = typeof AgentLanPairingIntentIdSchema.Type;
type AgentLanPairingDeviceRef = Infer<typeof AgentLanPairingDeviceRefSchema>;
type AgentLanPairingDiscoveryDevice = Infer<typeof AgentLanPairingDiscoveryDeviceSchema>;
type AgentLanPairingChallenge = Infer<typeof AgentLanPairingChallengeSchema>;
type AgentLanPairingIntentKind = Infer<typeof AgentLanPairingIntentKindSchema>;
type AgentLanPairingNetworkMode = Infer<typeof AgentLanPairingNetworkModeSchema>;
type AgentLanPairingParentAuthority = Infer<typeof AgentLanPairingParentAuthoritySchema>;
type AgentLanPairingProofDigest = typeof AgentLanPairingProofDigestSchema.Type;
type AgentLanPairingProofPreview = Infer<typeof AgentLanPairingProofPreviewSchema>;
type AgentLanPairingRejectionReason = Infer<typeof AgentLanPairingRejectionReasonSchema>;
type AgentLanPairingResponseState = Infer<typeof AgentLanPairingResponseStateSchema>;
type AgentLanPairingRouteId = typeof AgentLanPairingRouteIdSchema.Type;
type AgentLanPairingRuntimeSupportStatus = Infer<typeof AgentLanPairingRuntimeSupportStatusSchema>;
type AgentLanChildAgentResponse = Infer<typeof AgentLanChildAgentResponseSchema>;
type AgentLanParentIntentEnvelope = Infer<typeof AgentLanParentIntentEnvelopeSchema>;
type AgentPairingProof = Infer<typeof AgentPairingProofSchema>;
type AgentRouteSecurityPolicy = Infer<typeof AgentRouteSecurityPolicySchema>;

export type {
  AgentLanChildAgentResponse,
  AgentLanPairingAddressRef,
  AgentLanPairingChallenge,
  AgentLanPairingChallengeId,
  AgentLanPairingDeviceRef,
  AgentLanPairingDiscoveryDevice,
  AgentLanPairingIntentId,
  AgentLanPairingIntentKind,
  AgentLanPairingNetworkMode,
  AgentLanPairingParentAuthority,
  AgentLanPairingProofDigest,
  AgentLanPairingProofPreview,
  AgentLanPairingRejectionReason,
  AgentLanPairingResponseState,
  AgentLanPairingRouteId,
  AgentLanPairingRuntimeSupportStatus,
  AgentLanParentIntentEnvelope,
  AgentLanSelectedDeviceReachability,
  AgentPairingId,
  AgentPairingProof,
  AgentPairingState,
  AgentPairingTokenHash,
  AgentRouteSecurityPolicy,
};
