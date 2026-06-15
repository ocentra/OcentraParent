import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LanAiProviderRoutingStateSchema,
  LanPairingAgentPeerIdSchema,
  LanPairingDeviceReachabilitySchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
  LanPairingTimestampSchema,
  LanPairingTrustStateSchema,
} from './lan-pairing-values';
import {
  LanHouseholdCloudRelayDecisionStateSchema,
  LanHouseholdCloudRelayImplementationStateSchema,
  LanHouseholdProductProofStateSchema,
} from './lan-pairing-product-proof';

const NonEmptyProviderSelectionText = Schema.String.pipe(Schema.minLength(1));

export const LanProviderSelectionLifecycleStateSchema = withParser(
  Schema.Literal(
    'candidate-discovered',
    'candidate-eligible',
    'candidate-selected',
    'candidate-rejected',
    'candidate-degraded',
    'candidate-unavailable',
    'manual-required',
    'not-implemented'
  )
);

export const LanProviderSelectionPolicyDecisionSchema = withParser(
  Schema.Literal(
    'select-authorized-provider',
    'refuse-unpaired-provider',
    'refuse-route-blocked-provider',
    'refuse-unsupported-capability',
    'degrade-busy-provider',
    'degrade-provider-unavailable',
    'require-physical-household-proof',
    'require-cloud-relay-decision'
  )
);

export const LanProviderSelectionManualRequirementSchema = withParser(
  Schema.Literal(
    'physical-household-provider-host',
    'provider-route-origin-allowlist',
    'provider-route-stale-offline-artifact',
    'provider-revocation-artifact',
    'cloud-relay-provider-decision'
  )
);

export const LanProviderSelectionCandidateEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    providerPeerId: LanPairingAgentPeerIdSchema,
    routeId: LanPairingRouteIdSchema,
    lifecycleState: LanProviderSelectionLifecycleStateSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    trustState: LanPairingTrustStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    routingState: LanAiProviderRoutingStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    policyDecision: LanProviderSelectionPolicyDecisionSchema,
    proofState: LanHouseholdProductProofStateSchema,
    evidenceLabel: NonEmptyProviderSelectionText,
  })
);

export const LanProviderSelectionManualRequirementEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    requirement: LanProviderSelectionManualRequirementSchema,
    state: LanHouseholdProductProofStateSchema,
    requiredArtifactSummary: NonEmptyProviderSelectionText,
  })
);

export const LanProviderSelectionReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: LanPairingSchemaVersionSchema,
    checkedAt: LanPairingTimestampSchema,
    selectedProviderRouteId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    authorizedProviderSelectionState: LanHouseholdProductProofStateSchema,
    physicalHouseholdProviderProofState: LanHouseholdProductProofStateSchema,
    cloudRelayImplementationState: LanHouseholdCloudRelayImplementationStateSchema,
    cloudRelayDecisionState: LanHouseholdCloudRelayDecisionStateSchema,
    candidates: Schema.Array(LanProviderSelectionCandidateEvidenceSchema),
    manualRequirements: Schema.Array(LanProviderSelectionManualRequirementEvidenceSchema),
  })
);

export type LanProviderSelectionLifecycleState = Infer<typeof LanProviderSelectionLifecycleStateSchema>;
export type LanProviderSelectionPolicyDecision = Infer<typeof LanProviderSelectionPolicyDecisionSchema>;
export type LanProviderSelectionManualRequirement = Infer<typeof LanProviderSelectionManualRequirementSchema>;
export type LanProviderSelectionCandidateEvidence = Infer<typeof LanProviderSelectionCandidateEvidenceSchema>;
export type LanProviderSelectionManualRequirementEvidence = Infer<
  typeof LanProviderSelectionManualRequirementEvidenceSchema
>;
export type LanProviderSelectionReadModel = Infer<typeof LanProviderSelectionReadModelSchema>;
