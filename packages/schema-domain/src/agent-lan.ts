import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import {
  AgentDeviceIdSchema,
  AgentEventIdSchema,
  AgentPeerIdSchema,
  AgentPlatformSchema,
  AgentProtocolSchemaVersion,
  AgentRouteSchema,
  AgentTimestampSchema,
} from './event-primitives';
import { ParentEvidenceReferenceIdSchema, ParentEvidenceReferenceKindSchema } from './family-reference-primitives';
import { LanPairingAuditEventTypeSchema as AgentLanPairingAuditEventTypeSchema } from './lan-pairing-values';

export const AgentPairingIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentPairingId'));
export const AgentPairingTokenHashSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentPairingTokenHash'));
export const AgentLanPairingAddressRefSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingAddressRef'));
export const AgentLanPairingChallengeIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingChallengeId'));
export const AgentLanPairingIntentIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingIntentId'));
export const AgentLanPairingProofDigestSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingProofDigest'));
export const AgentLanPairingRouteIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingRouteId'));
const AgentLanPairingControllerLeaseIdSchema = NonEmptyStringSchema.pipe(Schema.brand('AgentLanPairingControllerLeaseId'));
const AgentLanPairingEvidenceReferenceIdSchema = ParentEvidenceReferenceIdSchema;
const AgentLanPairingEvidenceReferenceKindSchema = ParentEvidenceReferenceKindSchema;

export const AgentPairingStateSchema = withParser(
  Schema.Literal('unauthenticated', 'unpaired', 'pairing', 'paired', 'revoked')
);

export const AgentLanSelectedDeviceReachabilitySchema = withParser(Schema.Literal('online', 'offline', 'stale'));
export const AgentLanSelectedRouteTrustStateSchema = withParser(
  Schema.Literal('unpaired', 'pairing', 'paired', 'revoked', 'expired')
);
export const AgentLanPairingNetworkModeSchema = withParser(Schema.Literal('loopback', 'local-network'));
export const AgentLanPairingParentAuthoritySchema = withParser(Schema.Literal('active-controller', 'observer'));
export const AgentLanPairingProductionDiscoveryStateSchema = withParser(
  Schema.Literal('discovered', 'pending', 'paired', 'rejected', 'expired', 'revoked', 'stale', 'offline', 'manual-required', 'unavailable')
);
export const AgentLanPairingRuntimeSupportStatusSchema = withParser(
  Schema.Literal('planned-unsupported', 'websocket-direct', 'network-neighbor')
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
export const AgentLanAiProviderRoutingStateSchema = withParser(
  Schema.Literal('authorized-result', 'busy', 'degraded', 'unavailable', 'unsupported-capability')
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
    childProfileId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    label: NonEmptyStringSchema,
    platform: AgentPlatformSchema,
    ipAddress: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    macAddress: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    hostname: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    networkInterface: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    agentStatus: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
    hardwareProfile: Schema.optionalWith(
      Schema.Union(
        Schema.Struct({
          manufacturer: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          model: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          cpuModel: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          cpuCores: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          memoryTotal: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          gpuModel: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          gpuDriver: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          gpuMemory: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
          nvidiaSmi: Schema.optionalWith(Schema.Union(NonEmptyStringSchema, Schema.Null), { default: () => null }),
        }),
        Schema.Null
      ),
      { default: () => null }
    ),
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

export const AgentRouteSecurityPolicySchema = withParser(
  Schema.Struct({
    route: AgentRouteSchema,
    requiresPairing: Schema.Boolean,
    allowsAnonymousControl: Schema.Boolean,
  })
);

export type AgentPairingId = typeof AgentPairingIdSchema.Type;
export type AgentPairingTokenHash = typeof AgentPairingTokenHashSchema.Type;
export type AgentLanPairingAddressRef = typeof AgentLanPairingAddressRefSchema.Type;
export type AgentLanPairingChallengeId = typeof AgentLanPairingChallengeIdSchema.Type;
export type AgentLanPairingIntentId = typeof AgentLanPairingIntentIdSchema.Type;
export type AgentLanPairingDeviceRef = Infer<typeof AgentLanPairingDeviceRefSchema>;
export type AgentLanPairingDiscoveryDevice = Infer<typeof AgentLanPairingDiscoveryDeviceSchema>;
export type AgentLanPairingChallenge = Infer<typeof AgentLanPairingChallengeSchema>;
export type AgentLanPairingIntentKind = Infer<typeof AgentLanPairingIntentKindSchema>;
export type AgentLanPairingNetworkMode = Infer<typeof AgentLanPairingNetworkModeSchema>;
export type AgentLanPairingParentAuthority = Infer<typeof AgentLanPairingParentAuthoritySchema>;
export type AgentLanPairingProductionDiscoveryState = Infer<typeof AgentLanPairingProductionDiscoveryStateSchema>;
export type AgentLanPairingProofDigest = typeof AgentLanPairingProofDigestSchema.Type;
export type AgentLanPairingProofPreview = Infer<typeof AgentLanPairingProofPreviewSchema>;
export type AgentLanPairingRejectionReason = Infer<typeof AgentLanPairingRejectionReasonSchema>;
export type AgentLanPairingResponseState = Infer<typeof AgentLanPairingResponseStateSchema>;
export type AgentLanPairingRouteId = typeof AgentLanPairingRouteIdSchema.Type;
export type AgentLanPairingRuntimeSupportStatus = Infer<typeof AgentLanPairingRuntimeSupportStatusSchema>;
export type AgentLanAiProviderRoutingState = Infer<typeof AgentLanAiProviderRoutingStateSchema>;
export type AgentLanChildAgentResponse = Infer<typeof AgentLanChildAgentResponseSchema>;
export type AgentLanParentIntentEnvelope = Infer<typeof AgentLanParentIntentEnvelopeSchema>;
export type AgentLanSelectedDeviceReachability = Infer<typeof AgentLanSelectedDeviceReachabilitySchema>;
export type AgentLanSelectedRouteTrustState = Infer<typeof AgentLanSelectedRouteTrustStateSchema>;
export type AgentPairingState = Infer<typeof AgentPairingStateSchema>;
export type AgentPairingProof = Infer<typeof AgentPairingProofSchema>;
export type AgentRouteSecurityPolicy = Infer<typeof AgentRouteSecurityPolicySchema>;
