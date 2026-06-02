import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentProtocolSchemaVersion, AgentTimestampSchema } from './primitives';
import {
  AgentLanPairingProductionDiscoveryStateSchema,
  AgentLanPairingRejectionReasonSchema,
  AgentLanPairingResponseStateSchema,
  AgentLanPairingRouteIdSchema,
} from './security';

const NonEmptySignedDiscoveryRelayText = Schema.String.pipe(Schema.minLength(1));

export const AgentLanSignedDiscoveryRelayAdapterKindSchema = withParser(
  Schema.Literal(
    'passive-lan-neighbor',
    'router-infrastructure',
    'mdns-name',
    'ssdp-name',
    'router-dhcp-name',
    'manual-direct-address',
    'signed-child-agent-hello',
    'signed-child-agent-heartbeat'
  )
);

export const AgentLanSignedDiscoveryRelaySourceConfidenceSchema = withParser(
  Schema.Literal('confirmed', 'strong', 'weak', 'manual-required', 'unavailable', 'rejected')
);

export const AgentLanSignedDiscoveryRelayCustodyLabelSchema = withParser(
  Schema.Literal(
    'parent-local-service',
    'passive-lan-observation',
    'router-infrastructure-observation',
    'manual-parent-entry',
    'signed-child-agent-artifact',
    'no-ocentra-child-data-custody',
    'parent-owned-storage-unavailable'
  )
);

export const AgentLanSignedDiscoveryRelayRuntimeOwnerSchema = withParser(
  Schema.Literal('parent-domain-contract', 'agent-protocol', 'rust-service-read-model', 'proof-harness', 'manual-proof')
);

export const AgentLanSignedDiscoveryRelayProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'not-implemented')
);

export const AgentLanSignedDiscoveryRelaySignedProofCheckSchema = withParser(
  Schema.Literal(
    'signed-hello-manual-required',
    'signed-heartbeat-manual-required',
    'accepted-signed-child-agent-manual-required',
    'unauthenticated-caller-rejected',
    'expired-signed-proof-rejected',
    'replayed-signed-proof-rejected',
    'wrong-origin-signed-proof-rejected',
    'wrong-device-signed-proof-rejected',
    'revoked-signed-proof-rejected',
    'stale-signed-proof-rejected'
  )
);

export const AgentLanSignedDiscoveryRelayRouteSafetyCheckSchema = withParser(
  Schema.Literal(
    'trusted-registry-restart-recovery',
    'selected-route-custody',
    'stale-selected-device-rejected',
    'offline-selected-device-rejected',
    'wrong-route-rejected',
    'revoked-route-rejected',
    'parent-assign-decision-audited',
    'parent-rename-decision-audited',
    'parent-ignore-decision-audited',
    'parent-restore-decision-audited',
    'parent-trust-decision-audited',
    'parent-revoke-decision-audited'
  )
);

export const AgentLanSignedDiscoveryRelayCacheCheckSchema = withParser(
  Schema.Literal(
    'relay-route-unavailable',
    'relay-route-queued-not-configured',
    'cache-route-unavailable',
    'parent-owned-storage-unavailable',
    'ocentra-child-data-custody-not-claimed'
  )
);

export const AgentLanSignedDiscoveryRelayDecisionStateSchema = withParser(
  Schema.Literal('local-first', 'unavailable', 'queued-not-configured', 'not-implemented')
);

export const AgentLanSignedDiscoveryRelayAdapterRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    adapter: AgentLanSignedDiscoveryRelayAdapterKindSchema,
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
    proofState: AgentLanSignedDiscoveryRelayProofStateSchema,
    sourceConfidence: AgentLanSignedDiscoveryRelaySourceConfidenceSchema,
    custodyLabel: AgentLanSignedDiscoveryRelayCustodyLabelSchema,
    runtimeOwner: AgentLanSignedDiscoveryRelayRuntimeOwnerSchema,
    evidenceLabel: NonEmptySignedDiscoveryRelayText,
    requiredArtifactSummary: Schema.Union(NonEmptySignedDiscoveryRelayText, Schema.Null),
  })
);

export const AgentLanSignedDiscoveryRelaySignedProofRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    check: AgentLanSignedDiscoveryRelaySignedProofCheckSchema,
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
    responseState: AgentLanPairingResponseStateSchema,
    rejectionReason: Schema.Union(AgentLanPairingRejectionReasonSchema, Schema.Null),
    proofState: AgentLanSignedDiscoveryRelayProofStateSchema,
    runtimeOwner: AgentLanSignedDiscoveryRelayRuntimeOwnerSchema,
    evidenceLabel: NonEmptySignedDiscoveryRelayText,
  })
);

export const AgentLanSignedDiscoveryRelayRouteSafetyRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    check: AgentLanSignedDiscoveryRelayRouteSafetyCheckSchema,
    routeId: Schema.Union(AgentLanPairingRouteIdSchema, Schema.Null),
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
    responseState: AgentLanPairingResponseStateSchema,
    rejectionReason: Schema.Union(AgentLanPairingRejectionReasonSchema, Schema.Null),
    proofState: AgentLanSignedDiscoveryRelayProofStateSchema,
    runtimeOwner: AgentLanSignedDiscoveryRelayRuntimeOwnerSchema,
    custodyLabel: AgentLanSignedDiscoveryRelayCustodyLabelSchema,
    evidenceLabel: NonEmptySignedDiscoveryRelayText,
  })
);

export const AgentLanSignedDiscoveryRelayCacheRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    check: AgentLanSignedDiscoveryRelayCacheCheckSchema,
    decisionState: AgentLanSignedDiscoveryRelayDecisionStateSchema,
    discoveryState: AgentLanPairingProductionDiscoveryStateSchema,
    proofState: AgentLanSignedDiscoveryRelayProofStateSchema,
    runtimeOwner: AgentLanSignedDiscoveryRelayRuntimeOwnerSchema,
    custodyLabel: AgentLanSignedDiscoveryRelayCustodyLabelSchema,
    evidenceLabel: NonEmptySignedDiscoveryRelayText,
  })
);

export const AgentLanSignedDiscoveryRelaySpineSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    generatedAt: AgentTimestampSchema,
    adapterRows: Schema.Array(AgentLanSignedDiscoveryRelayAdapterRowSchema),
    signedProofRows: Schema.Array(AgentLanSignedDiscoveryRelaySignedProofRowSchema),
    routeSafetyRows: Schema.Array(AgentLanSignedDiscoveryRelayRouteSafetyRowSchema),
    relayCacheRows: Schema.Array(AgentLanSignedDiscoveryRelayCacheRowSchema),
    manualProofRequired: Schema.Array(AgentLanSignedDiscoveryRelayAdapterKindSchema),
    notImplemented: Schema.Array(AgentLanSignedDiscoveryRelayCacheCheckSchema),
    claimsProved: Schema.Array(NonEmptySignedDiscoveryRelayText),
    claimsNotProved: Schema.Array(NonEmptySignedDiscoveryRelayText),
  })
);

export type AgentLanSignedDiscoveryRelayAdapterKind = Infer<typeof AgentLanSignedDiscoveryRelayAdapterKindSchema>;
export type AgentLanSignedDiscoveryRelaySourceConfidence = Infer<
  typeof AgentLanSignedDiscoveryRelaySourceConfidenceSchema
>;
export type AgentLanSignedDiscoveryRelayCustodyLabel = Infer<typeof AgentLanSignedDiscoveryRelayCustodyLabelSchema>;
export type AgentLanSignedDiscoveryRelayRuntimeOwner = Infer<typeof AgentLanSignedDiscoveryRelayRuntimeOwnerSchema>;
export type AgentLanSignedDiscoveryRelayProofState = Infer<typeof AgentLanSignedDiscoveryRelayProofStateSchema>;
export type AgentLanSignedDiscoveryRelaySignedProofCheck = Infer<
  typeof AgentLanSignedDiscoveryRelaySignedProofCheckSchema
>;
export type AgentLanSignedDiscoveryRelayRouteSafetyCheck = Infer<
  typeof AgentLanSignedDiscoveryRelayRouteSafetyCheckSchema
>;
export type AgentLanSignedDiscoveryRelayCacheCheck = Infer<typeof AgentLanSignedDiscoveryRelayCacheCheckSchema>;
export type AgentLanSignedDiscoveryRelayDecisionState = Infer<typeof AgentLanSignedDiscoveryRelayDecisionStateSchema>;
export type AgentLanSignedDiscoveryRelayAdapterRow = Infer<typeof AgentLanSignedDiscoveryRelayAdapterRowSchema>;
export type AgentLanSignedDiscoveryRelaySignedProofRow = Infer<typeof AgentLanSignedDiscoveryRelaySignedProofRowSchema>;
export type AgentLanSignedDiscoveryRelayRouteSafetyRow = Infer<typeof AgentLanSignedDiscoveryRelayRouteSafetyRowSchema>;
export type AgentLanSignedDiscoveryRelayCacheRow = Infer<typeof AgentLanSignedDiscoveryRelayCacheRowSchema>;
export type AgentLanSignedDiscoveryRelaySpine = Infer<typeof AgentLanSignedDiscoveryRelaySpineSchema>;
