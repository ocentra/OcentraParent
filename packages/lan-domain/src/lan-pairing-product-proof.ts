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

export const LanHouseholdCloudRelayImplementationStateSchema = withParser(Schema.Literal('not-implemented'));

export const LanHouseholdCloudRelayDecisionStateSchema = withParser(Schema.Literal('manual-decision-required'));

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

export const LanHouseholdRouteCheckSchema = withParser(
  Schema.Literal(
    'paired-route-accepted',
    'failed-unpaired-rejected',
    'wrong-origin-rejected',
    'wrong-device-rejected',
    'replay-rejected',
    'revocation-rejected',
    'stale-selected-device-rejected',
    'offline-selected-device-rejected',
    'unavailable-route-rejected',
    'manual-required-physical-household-lan'
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

export const LanHouseholdRouteCheckOutcomeSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    routeId: LanPairingRouteIdSchema,
    check: LanHouseholdRouteCheckSchema,
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

export const LanHouseholdCloudRelayDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    implementationState: LanHouseholdCloudRelayImplementationStateSchema,
    decisionState: LanHouseholdCloudRelayDecisionStateSchema,
    requiredDecisionSummary: NonEmptyLanProductProofText,
    proofBoundary: NonEmptyLanProductProofText,
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
    cloudRelayDecision: LanHouseholdCloudRelayDecisionSchema,
    routeCheckOutcomes: Schema.Array(LanHouseholdRouteCheckOutcomeSchema),
    selectedRouteEvidence: Schema.Array(LanHouseholdSelectedRouteEvidenceSchema),
    selectedProviderPolicyEvidence: Schema.Array(LanHouseholdSelectedProviderPolicyEvidenceSchema),
    manualProofGates: Schema.Array(LanHouseholdManualProofGateRequirementSchema),
  })
);

export type LanHouseholdProductProofState = Infer<typeof LanHouseholdProductProofStateSchema>;
export type LanHouseholdProductReadinessDecision = Infer<typeof LanHouseholdProductReadinessDecisionSchema>;
export type LanHouseholdCloudRelayImplementationState = Infer<typeof LanHouseholdCloudRelayImplementationStateSchema>;
export type LanHouseholdCloudRelayDecisionState = Infer<typeof LanHouseholdCloudRelayDecisionStateSchema>;
export type LanHouseholdManualProofGate = Infer<typeof LanHouseholdManualProofGateSchema>;
export type LanHouseholdRouteCheck = Infer<typeof LanHouseholdRouteCheckSchema>;
export type LanHouseholdSelectedRouteEvidence = Infer<typeof LanHouseholdSelectedRouteEvidenceSchema>;
export type LanHouseholdRouteCheckOutcome = Infer<typeof LanHouseholdRouteCheckOutcomeSchema>;
export type LanHouseholdSelectedProviderPolicyEvidence = Infer<typeof LanHouseholdSelectedProviderPolicyEvidenceSchema>;
export type LanHouseholdManualProofGateRequirement = Infer<typeof LanHouseholdManualProofGateRequirementSchema>;
export type LanHouseholdCloudRelayDecision = Infer<typeof LanHouseholdCloudRelayDecisionSchema>;
export type LanHouseholdProductProofReadModel = Infer<typeof LanHouseholdProductProofReadModelSchema>;
