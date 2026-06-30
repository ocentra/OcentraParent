import { type Infer, Schema, withParser, NonEmptyStringSchema } from './effect';
import {
  LanHouseholdManualProofGateSchema,
  LanHouseholdProductProofStateSchema,
  LanHouseholdProductReadinessDecisionSchema,
} from './lan-pairing-product-proof';
import {
  LanPairingDeviceReachabilitySchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
  LanPairingTrustStateSchema,
} from './lan-pairing-values';

export const V09ProductionDiscoveryHouseholdProofBoundarySchema = withParser(
  Schema.Literal('local-real-service-not-physical-household-lan')
);

export const V09ProductionDiscoveryHouseholdRuntimeOwnerSchema = withParser(
  Schema.Literal('rust-parent-runtime-contract', 'agent-protocol', 'rust-service-read-model', 'proof-harness', 'manual-proof')
);

export const V09ProductionDiscoveryHouseholdCheckSchema = withParser(
  Schema.Literal(
    'production-discovery-states',
    'paired-route-accepted',
    'failed-unpaired-rejected',
    'replay-rejected',
    'restart-selected-route-recovered',
    'restart-registry-state-recovered',
    'stale-source-rejected',
    'offline-device-rejected',
    'revoked-pairing-rejected',
    'unavailable-route-rejected',
    'wrong-origin-rejected',
    'wrong-device-rejected',
    'manual-physical-household-checklist'
  )
);

export const V09ProductionDiscoveryHouseholdSourceStateSchema = withParser(
  Schema.Literal(
    'discovered',
    'pending',
    'paired',
    'failed-unpaired',
    'restart-recovered',
    'stale',
    'offline',
    'revoked',
    'unavailable',
    'wrong-origin',
    'wrong-device',
    'manual-required'
  )
);

export const V09ProductionDiscoveryHouseholdRouteRecoveryStateSchema = withParser(
  Schema.Literal(
    'selected-route-persisted',
    'registry-restored-after-restart',
    'fail-closed-unpaired',
    'manual-required-physical-route-recovery'
  )
);

export const V09ProductionDiscoveryHouseholdStateEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    check: V09ProductionDiscoveryHouseholdCheckSchema,
    sourceState: V09ProductionDiscoveryHouseholdSourceStateSchema,
    routeId: LanPairingRouteIdSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    trustState: LanPairingTrustStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    routeRecoveryState: V09ProductionDiscoveryHouseholdRouteRecoveryStateSchema,
    proofState: LanHouseholdProductProofStateSchema,
    runtimeOwner: V09ProductionDiscoveryHouseholdRuntimeOwnerSchema,
    evidenceLabel: NonEmptyStringSchema,
  })
);

export const V09ProductionDiscoveryHouseholdManualChecklistItemSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    gate: LanHouseholdManualProofGateSchema,
    state: LanHouseholdProductProofStateSchema,
    requiredArtifactSummary: NonEmptyStringSchema,
    runtimeOwner: V09ProductionDiscoveryHouseholdRuntimeOwnerSchema,
  })
);

export const V09ProductionDiscoveryHouseholdProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    checkedAt: LanPairingTimestampSchema,
    proofBoundary: V09ProductionDiscoveryHouseholdProofBoundarySchema,
    productReadinessDecision: LanHouseholdProductReadinessDecisionSchema,
    productionDiscoveryStates: Schema.Array(V09ProductionDiscoveryHouseholdStateEvidenceSchema),
    routeChecks: Schema.Array(V09ProductionDiscoveryHouseholdStateEvidenceSchema),
    restartRecovery: Schema.Array(V09ProductionDiscoveryHouseholdStateEvidenceSchema),
    sourceDeviceStates: Schema.Array(V09ProductionDiscoveryHouseholdStateEvidenceSchema),
    manualHouseholdProofChecklist: Schema.Array(V09ProductionDiscoveryHouseholdManualChecklistItemSchema),
    claimsProved: Schema.Array(NonEmptyStringSchema),
    claimsNotProved: Schema.Array(NonEmptyStringSchema),
  })
);

export type V09ProductionDiscoveryHouseholdProofBoundary = Infer<
  typeof V09ProductionDiscoveryHouseholdProofBoundarySchema
>;
export type V09ProductionDiscoveryHouseholdRuntimeOwner = Infer<
  typeof V09ProductionDiscoveryHouseholdRuntimeOwnerSchema
>;
export type V09ProductionDiscoveryHouseholdCheck = Infer<typeof V09ProductionDiscoveryHouseholdCheckSchema>;
export type V09ProductionDiscoveryHouseholdSourceState = Infer<typeof V09ProductionDiscoveryHouseholdSourceStateSchema>;
export type V09ProductionDiscoveryHouseholdRouteRecoveryState = Infer<
  typeof V09ProductionDiscoveryHouseholdRouteRecoveryStateSchema
>;
export type V09ProductionDiscoveryHouseholdStateEvidence = Infer<
  typeof V09ProductionDiscoveryHouseholdStateEvidenceSchema
>;
export type V09ProductionDiscoveryHouseholdManualChecklistItem = Infer<
  typeof V09ProductionDiscoveryHouseholdManualChecklistItemSchema
>;
export type V09ProductionDiscoveryHouseholdProofReadModel = Infer<
  typeof V09ProductionDiscoveryHouseholdProofReadModelSchema
>;
