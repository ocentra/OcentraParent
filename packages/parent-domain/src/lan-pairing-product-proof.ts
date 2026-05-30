import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LanAiProviderRoutingStateSchema,
  LanPairingDeviceReachabilitySchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
  LanPairingTrustStateSchema,
} from './lan-pairing-values';

const NonEmptyLanProductProofText = Schema.String.pipe(Schema.minLength(1));

export const LanHouseholdProductProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'not-implemented')
);

export const LanHouseholdProductReadinessDecisionSchema = withParser(
  Schema.Literal('not-ready-for-product-ready-household-lan-claim')
);

export const LanHouseholdManualProofGateSchema = withParser(
  Schema.Literal(
    'two-physical-hosts',
    'household-router-reachability',
    'os-firewall-or-local-network-permission',
    'allowed-origin-on-physical-controller',
    'physical-route-selection-and-takeover',
    'physical-revocation-and-rejection',
    'physical-stale-offline-selected-device',
    'real-mobile-controller-package',
    'real-mobile-observer-package',
    'real-lan-ai-provider-host',
    'cloud-relay-separate-proof'
  )
);

export const LanHouseholdSelectedRouteEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    routeId: LanPairingRouteIdSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    trustState: LanPairingTrustStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: LanHouseholdProductProofStateSchema,
    evidenceLabel: NonEmptyLanProductProofText,
  })
);

export const LanHouseholdSelectedProviderPolicyEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    routeId: LanPairingRouteIdSchema,
    routingState: LanAiProviderRoutingStateSchema,
    selectedRouteTrustState: LanPairingTrustStateSchema,
    selectedDeviceReachability: LanPairingDeviceReachabilitySchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: LanHouseholdProductProofStateSchema,
    evidenceLabel: NonEmptyLanProductProofText,
  })
);

export const LanHouseholdManualProofGateRequirementSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    gate: LanHouseholdManualProofGateSchema,
    state: LanHouseholdProductProofStateSchema,
    requiredArtifactSummary: NonEmptyLanProductProofText,
  })
);

export const LanHouseholdProductProofReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    checkedAt: LanPairingTimestampSchema,
    productReadinessDecision: LanHouseholdProductReadinessDecisionSchema,
    localMultiServiceProofState: LanHouseholdProductProofStateSchema,
    physicalHouseholdLanProofState: LanHouseholdProductProofStateSchema,
    parentMobileControllerProofState: LanHouseholdProductProofStateSchema,
    cloudRelayState: LanHouseholdProductProofStateSchema,
    selectedRouteEvidence: Schema.Array(LanHouseholdSelectedRouteEvidenceSchema),
    selectedProviderPolicyEvidence: Schema.Array(LanHouseholdSelectedProviderPolicyEvidenceSchema),
    manualProofGates: Schema.Array(LanHouseholdManualProofGateRequirementSchema),
  })
);

export type LanHouseholdProductProofState = Infer<typeof LanHouseholdProductProofStateSchema>;
export type LanHouseholdProductReadinessDecision = Infer<typeof LanHouseholdProductReadinessDecisionSchema>;
export type LanHouseholdManualProofGate = Infer<typeof LanHouseholdManualProofGateSchema>;
export type LanHouseholdSelectedRouteEvidence = Infer<typeof LanHouseholdSelectedRouteEvidenceSchema>;
export type LanHouseholdSelectedProviderPolicyEvidence = Infer<typeof LanHouseholdSelectedProviderPolicyEvidenceSchema>;
export type LanHouseholdManualProofGateRequirement = Infer<typeof LanHouseholdManualProofGateRequirementSchema>;
export type LanHouseholdProductProofReadModel = Infer<typeof LanHouseholdProductProofReadModelSchema>;
