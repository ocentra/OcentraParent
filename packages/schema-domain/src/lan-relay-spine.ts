import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';
import { LanHouseholdProductProofStateSchema } from './lan-product-proof';
import {
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingResponseStateSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
} from './lan-pairing-values';
import { LanProductionHouseholdProofRuntimeOwnerSchema } from './lan-production-household-proof';

export const LanRelayAdapterKindSchema = withParser(
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
export const LanRelaySourceConfidenceSchema = withParser(
  Schema.Literal('confirmed', 'strong', 'weak', 'manual-required', 'unavailable', 'rejected')
);
export const LanRelayCustodyLabelSchema = withParser(
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
export const LanRelaySignedProofCheckSchema = withParser(
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
export const LanRelayRouteSafetyCheckSchema = withParser(
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
export const LanRelayCacheCheckSchema = withParser(
  Schema.Literal(
    'relay-route-unavailable',
    'relay-route-queued-not-configured',
    'cache-route-unavailable',
    'parent-owned-storage-unavailable',
    'ocentra-child-data-custody-not-claimed'
  )
);
export const LanRelayDecisionStateSchema = withParser(
  Schema.Literal('local-first', 'unavailable', 'queued-not-configured', 'not-implemented')
);
export const LanRelayProofStateSchema = LanHouseholdProductProofStateSchema;

export const LanRelayAdapterRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    adapter: LanRelayAdapterKindSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    proofState: LanHouseholdProductProofStateSchema,
    sourceConfidence: LanRelaySourceConfidenceSchema,
    custodyLabel: LanRelayCustodyLabelSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    evidenceLabel: NonEmptyStringSchema,
    requiredArtifactSummary: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);
export const LanRelaySignedProofRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    check: LanRelaySignedProofCheckSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    responseState: LanPairingResponseStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    evidenceLabel: NonEmptyStringSchema,
  })
);
export const LanRelayRouteSafetyRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    check: LanRelayRouteSafetyCheckSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    responseState: LanPairingResponseStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    custodyLabel: LanRelayCustodyLabelSchema,
    evidenceLabel: NonEmptyStringSchema,
  })
);
export const LanRelayCacheRowSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    check: LanRelayCacheCheckSchema,
    decisionState: LanRelayDecisionStateSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: LanProductionHouseholdProofRuntimeOwnerSchema,
    custodyLabel: LanRelayCustodyLabelSchema,
    evidenceLabel: NonEmptyStringSchema,
  })
);

export const LanRelaySpineSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    generatedAt: LanPairingTimestampSchema,
    adapterRows: Schema.Array(LanRelayAdapterRowSchema),
    signedProofRows: Schema.Array(LanRelaySignedProofRowSchema),
    routeSafetyRows: Schema.Array(LanRelayRouteSafetyRowSchema),
    relayCacheRows: Schema.Array(LanRelayCacheRowSchema),
    manualProofRequired: Schema.Array(LanRelayAdapterKindSchema),
    notImplemented: Schema.Array(LanRelayCacheCheckSchema),
    claimsProved: Schema.Array(NonEmptyStringSchema),
    claimsNotProved: Schema.Array(NonEmptyStringSchema),
  })
);

export type LanRelayAdapterKind = Infer<typeof LanRelayAdapterKindSchema>;
export type LanRelaySourceConfidence = Infer<typeof LanRelaySourceConfidenceSchema>;
export type LanRelayCustodyLabel = Infer<typeof LanRelayCustodyLabelSchema>;
export type LanRelaySignedProofCheck = Infer<typeof LanRelaySignedProofCheckSchema>;
export type LanRelayRouteSafetyCheck = Infer<typeof LanRelayRouteSafetyCheckSchema>;
export type LanRelayCacheCheck = Infer<typeof LanRelayCacheCheckSchema>;
export type LanRelayDecisionState = Infer<typeof LanRelayDecisionStateSchema>;
export type LanRelayAdapterRow = Infer<typeof LanRelayAdapterRowSchema>;
export type LanRelaySignedProofRow = Infer<typeof LanRelaySignedProofRowSchema>;
export type LanRelayRouteSafetyRow = Infer<typeof LanRelayRouteSafetyRowSchema>;
export type LanRelayCacheRow = Infer<typeof LanRelayCacheRowSchema>;
export type LanRelaySpine = Infer<typeof LanRelaySpineSchema>;
export type LanRelayProofState = Infer<typeof LanRelayProofStateSchema>;

export const AgentLanSignedDiscoveryRelayAdapterKindSchema = LanRelayAdapterKindSchema;
export const AgentLanSignedDiscoveryRelaySourceConfidenceSchema = LanRelaySourceConfidenceSchema;
export const AgentLanSignedDiscoveryRelayCustodyLabelSchema = LanRelayCustodyLabelSchema;
export const AgentLanSignedDiscoveryRelayRuntimeOwnerSchema = LanProductionHouseholdProofRuntimeOwnerSchema;
export const AgentLanSignedDiscoveryRelayProofStateSchema = LanRelayProofStateSchema;
export const AgentLanSignedDiscoveryRelaySignedProofCheckSchema = LanRelaySignedProofCheckSchema;
export const AgentLanSignedDiscoveryRelayRouteSafetyCheckSchema = LanRelayRouteSafetyCheckSchema;
export const AgentLanSignedDiscoveryRelayCacheCheckSchema = LanRelayCacheCheckSchema;
export const AgentLanSignedDiscoveryRelayDecisionStateSchema = LanRelayDecisionStateSchema;
export const AgentLanSignedDiscoveryRelayAdapterRowSchema = LanRelayAdapterRowSchema;
export const AgentLanSignedDiscoveryRelaySignedProofRowSchema = LanRelaySignedProofRowSchema;
export const AgentLanSignedDiscoveryRelayRouteSafetyRowSchema = LanRelayRouteSafetyRowSchema;
export const AgentLanSignedDiscoveryRelayCacheRowSchema = LanRelayCacheRowSchema;
export const AgentLanSignedDiscoveryRelaySpineSchema = LanRelaySpineSchema;
