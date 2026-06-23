import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  LanPairingDeviceReachabilitySchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingRouteIdSchema,
  LanPairingSchemaVersionSchema,
} from './lan-pairing-values';
import {
  ParentMobileCommandAuthorityStateSchema,
  ParentMobileControllerStateSchema,
  ParentMobilePackageStateSchema,
  ParentMobilePlatformSchema,
  ParentMobileServiceAvailabilityStateSchema,
} from './parent-mobile-runtime';
import {
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';

export const V09MobileControllerDiscoveryRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('v0.9-mobile-controller-discovery-runtime')
);
export const V09RuntimeProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'manual-required', 'not-implemented', 'degraded', 'unavailable')
);
export const V09MobileRouteSourceSchema = withParser(
  Schema.Literal('local-real-service-proof', 'manual-mobile-package-required')
);
export const V09MobileControllerTransitionSchema = withParser(
  Schema.Literal('takeover', 'release', 'renew', 'degraded-provider', 'failed-unpaired')
);
export const V09MobileControllerTransitionStateSchema = withParser(
  Schema.Literal('proved-local-service', 'manual-required-mobile-package', 'degraded', 'rejected')
);
export const V09ProofLabelSchema = brandedNonEmptyStringSchema('V09ProofLabel');
export const V09ClaimBoundarySchema = brandedNonEmptyStringSchema('V09ClaimBoundary');

export const V09HouseholdDiscoveryRuntimeProofSchema = withParser(
  Schema.Struct({
    localServiceState: V09RuntimeProofStateSchema,
    physicalHouseholdLanState: V09RuntimeProofStateSchema,
    cloudRelayState: V09RuntimeProofStateSchema,
    discoveryStatesCovered: Schema.Array(LanPairingProductionDiscoveryStateSchema),
    evidenceReferenceIds: Schema.Array(ParentEvidenceReferenceIdSchema),
  })
);

export const V09MobileRouteReadModelSchema = withParser(
  Schema.Struct({
    platform: ParentMobilePlatformSchema,
    parentDeviceId: ParentDeviceIdSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    routeSource: V09MobileRouteSourceSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    controllerState: ParentMobileControllerStateSchema,
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    serviceState: ParentMobileServiceAvailabilityStateSchema,
    packageState: ParentMobilePackageStateSchema,
    proofLabels: Schema.Array(V09ProofLabelSchema),
  })
);

export const V09MobileControllerTransitionProofSchema = withParser(
  Schema.Struct({
    transition: V09MobileControllerTransitionSchema,
    state: V09MobileControllerTransitionStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofLabel: V09ProofLabelSchema,
  })
);

export const V09RejectedRuntimeBehaviorSchema = withParser(
  Schema.Struct({
    reason: LanPairingRejectionReasonSchema,
    proofLabel: V09ProofLabelSchema,
  })
);

export const V09RuntimeClaimBoundariesSchema = withParser(
  Schema.Struct({
    physicalHouseholdLan: V09ClaimBoundarySchema,
    parentMobileWriteAuthority: V09ClaimBoundarySchema,
    cloudRelay: V09ClaimBoundarySchema,
    mobileChildAgentBehavior: V09ClaimBoundarySchema,
    storesSigningEntitlements: V09ClaimBoundarySchema,
  })
);

const V09MobileControllerDiscoveryRuntimeReadModelBaseSchema = Schema.Struct({
  schemaVersion: V09MobileControllerDiscoveryRuntimeSchemaVersionSchema,
  lanSchemaVersion: LanPairingSchemaVersionSchema,
  householdDiscovery: V09HouseholdDiscoveryRuntimeProofSchema,
  mobileRouteReadModels: Schema.Array(V09MobileRouteReadModelSchema),
  controllerTransitions: Schema.Array(V09MobileControllerTransitionProofSchema),
  failedUnpairedBehavior: V09RejectedRuntimeBehaviorSchema,
  staleOfflineBehavior: Schema.Array(V09RejectedRuntimeBehaviorSchema),
  claimBoundaries: V09RuntimeClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type V09MobileControllerDiscoveryRuntimeReadModelCandidate = Infer<
  typeof V09MobileControllerDiscoveryRuntimeReadModelBaseSchema
>;

export const V09MobileControllerDiscoveryRuntimeReadModelSchema = withParser(
  V09MobileControllerDiscoveryRuntimeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        v09MobileControllerDiscoveryRuntimeIsHonest(readModel) ||
        'Expected V0.9 mobile controller discovery runtime proof to keep physical LAN, mobile authority, cloud relay, and mobile child-agent claims manual or unavailable'
    )
  )
);

function v09MobileControllerDiscoveryRuntimeIsHonest(
  readModel: V09MobileControllerDiscoveryRuntimeReadModelCandidate
): boolean {
  return (
    householdDiscoveryStatesAreComplete(readModel.householdDiscovery) &&
    mobileRouteReadModelsAreHonest(readModel.mobileRouteReadModels) &&
    controllerTransitionsAreCovered(readModel.controllerTransitions) &&
    staleOfflineBehaviorIsCovered(readModel.staleOfflineBehavior) &&
    readModel.failedUnpairedBehavior.reason === 'anonymous' &&
    readModel.householdDiscovery.physicalHouseholdLanState === 'manual-required' &&
    readModel.householdDiscovery.cloudRelayState === 'not-implemented'
  );
}

function householdDiscoveryStatesAreComplete(discovery: V09HouseholdDiscoveryRuntimeProof): boolean {
  const states = new Set(discovery.discoveryStatesCovered);
  const requiredStates: ReadonlyArray<V09HouseholdDiscoveryRuntimeProof['discoveryStatesCovered'][number]> = [
    'discovered',
    'pending',
    'paired',
    'revoked',
    'stale',
    'offline',
    'unavailable',
  ];
  return discovery.localServiceState === 'ci-mechanical-proof' && requiredStates.every((state) => states.has(state));
}

function mobileRouteReadModelsAreHonest(routeReadModels: ReadonlyArray<V09MobileRouteReadModel>): boolean {
  const platforms = new Set(routeReadModels.map((route) => route.platform));
  return (
    platforms.size === 2 &&
    platforms.has('android') &&
    platforms.has('ios') &&
    routeReadModels.every((route) => route.commandAuthorityState !== 'active-controller-backend-proof')
  );
}

function controllerTransitionsAreCovered(transitions: ReadonlyArray<V09MobileControllerTransitionProof>): boolean {
  const covered = new Set(transitions.map((transition) => transition.transition));
  const requiredTransitions: ReadonlyArray<V09MobileControllerTransitionProof['transition']> = [
    'takeover',
    'release',
    'renew',
    'degraded-provider',
    'failed-unpaired',
  ];
  return requiredTransitions.every((transition) => covered.has(transition));
}

function staleOfflineBehaviorIsCovered(behaviors: ReadonlyArray<V09RejectedRuntimeBehavior>): boolean {
  const reasons = new Set(behaviors.map((behavior) => behavior.reason));
  return reasons.has('stale') && reasons.has('offline');
}

export type V09RuntimeProofState = Infer<typeof V09RuntimeProofStateSchema>;
export type V09MobileRouteSource = Infer<typeof V09MobileRouteSourceSchema>;
export type V09MobileControllerTransition = Infer<typeof V09MobileControllerTransitionSchema>;
export type V09MobileControllerTransitionState = Infer<typeof V09MobileControllerTransitionStateSchema>;
export type V09HouseholdDiscoveryRuntimeProof = Infer<typeof V09HouseholdDiscoveryRuntimeProofSchema>;
export type V09MobileRouteReadModel = Infer<typeof V09MobileRouteReadModelSchema>;
export type V09MobileControllerTransitionProof = Infer<typeof V09MobileControllerTransitionProofSchema>;
export type V09RejectedRuntimeBehavior = Infer<typeof V09RejectedRuntimeBehaviorSchema>;
export type V09RuntimeClaimBoundaries = Infer<typeof V09RuntimeClaimBoundariesSchema>;
export type V09MobileControllerDiscoveryRuntimeReadModel = Infer<
  typeof V09MobileControllerDiscoveryRuntimeReadModelSchema
>;
