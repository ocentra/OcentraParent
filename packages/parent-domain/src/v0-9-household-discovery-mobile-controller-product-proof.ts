import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LanPairingDeviceReachabilitySchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingRouteIdSchema,
  LanPairingTrustStateSchema,
} from './lan-pairing-values';
import {
  ParentMobileCommandAuthorityStateSchema,
  ParentMobileControllerStateSchema,
  ParentMobilePlatformSchema,
  ParentMobileServiceAvailabilityStateSchema,
} from './parent-mobile-runtime';
import { ParentTimestampSchema } from './reference-primitives';
import {
  V09MobileControllerObserverOperationSchema,
  V09MobileControllerObserverOperationStateSchema,
} from './v0-9-mobile-controller-observer-runtime';
import {
  V09MobileControllerTransitionSchema,
  V09MobileControllerTransitionStateSchema,
  V09RuntimeProofStateSchema,
} from './v0-9-mobile-controller-discovery-runtime';

const NonEmptyHouseholdMobileProductProofText = Schema.String.pipe(Schema.minLength(1));

export const V09HouseholdDiscoveryMobileControllerProductProofIdSchema = withParser(
  Schema.Literal('v0-9-household-discovery-mobile-controller-product-proof')
);

export const V09HouseholdDiscoveryMobileControllerSourceProofSchema = withParser(
  Schema.Literal(
    'v0-9-production-discovery-household-proof',
    'v0-9-production-lan-mobile-controller-proof',
    'v0-9-mobile-controller-discovery-runtime-proof',
    'v0-9-mobile-controller-observer-runtime-proof',
    'parent-mobile-controller-observer-handoff-proof'
  )
);

export const V09HouseholdDiscoveryMobileControllerRouteCheckSchema = withParser(
  Schema.Literal(
    'paired-route-accepted',
    'failed-unpaired-rejected',
    'wrong-origin-rejected',
    'wrong-device-rejected',
    'replay-rejected',
    'revoked-pairing-rejected',
    'stale-source-rejected',
    'offline-device-rejected',
    'unavailable-route-rejected'
  )
);

export const V09HouseholdDiscoveryMobileControllerCloudRelayDecisionSchema = withParser(
  Schema.Literal('manual-decision-required')
);

export const V09HouseholdDiscoveryMobileControllerProofPathSchema = NonEmptyHouseholdMobileProductProofText.pipe(
  Schema.brand('V09HouseholdDiscoveryMobileControllerProofPath')
);
export const V09HouseholdDiscoveryMobileControllerProofCommandSchema = NonEmptyHouseholdMobileProductProofText.pipe(
  Schema.brand('V09HouseholdDiscoveryMobileControllerProofCommand')
);
export const V09HouseholdDiscoveryMobileControllerProofLabelSchema = NonEmptyHouseholdMobileProductProofText.pipe(
  Schema.brand('V09HouseholdDiscoveryMobileControllerProofLabel')
);
export const V09HouseholdDiscoveryMobileControllerClaimBoundarySchema = NonEmptyHouseholdMobileProductProofText.pipe(
  Schema.brand('V09HouseholdDiscoveryMobileControllerClaimBoundary')
);

export const V09HouseholdDiscoveryMobileControllerSourceProofInputSchema = withParser(
  Schema.Struct({
    source: V09HouseholdDiscoveryMobileControllerSourceProofSchema,
    path: V09HouseholdDiscoveryMobileControllerProofPathSchema,
    command: V09HouseholdDiscoveryMobileControllerProofCommandSchema,
  })
);

export const V09HouseholdDiscoveryMobileControllerRouteEvidenceSchema = withParser(
  Schema.Struct({
    check: V09HouseholdDiscoveryMobileControllerRouteCheckSchema,
    routeId: LanPairingRouteIdSchema,
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    trustState: LanPairingTrustStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: V09RuntimeProofStateSchema,
    proofLabel: V09HouseholdDiscoveryMobileControllerProofLabelSchema,
  })
);

export const V09HouseholdDiscoveryMobileRouteEvidenceSchema = withParser(
  Schema.Struct({
    platform: ParentMobilePlatformSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    controllerState: ParentMobileControllerStateSchema,
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    serviceState: ParentMobileServiceAvailabilityStateSchema,
    proofState: V09RuntimeProofStateSchema,
    proofLabel: V09HouseholdDiscoveryMobileControllerProofLabelSchema,
  })
);

export const V09HouseholdDiscoveryMobileControllerOperationEvidenceSchema = withParser(
  Schema.Struct({
    operation: V09MobileControllerObserverOperationSchema,
    operationState: V09MobileControllerObserverOperationStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: V09RuntimeProofStateSchema,
    proofLabel: V09HouseholdDiscoveryMobileControllerProofLabelSchema,
  })
);

export const V09HouseholdDiscoveryMobileControllerTransitionEvidenceSchema = withParser(
  Schema.Struct({
    transition: V09MobileControllerTransitionSchema,
    state: V09MobileControllerTransitionStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofLabel: V09HouseholdDiscoveryMobileControllerProofLabelSchema,
  })
);

export const V09HouseholdDiscoveryMobileControllerManualBoundarySchema = withParser(
  Schema.Struct({
    physicalHouseholdLan: V09RuntimeProofStateSchema,
    parentMobileWriteAuthority: V09RuntimeProofStateSchema,
    cloudRelayImplementation: V09RuntimeProofStateSchema,
    cloudRelayDecision: V09HouseholdDiscoveryMobileControllerCloudRelayDecisionSchema,
    mobileBackgroundBehavior: V09RuntimeProofStateSchema,
    physicalDeviceChecklist: Schema.Array(V09HouseholdDiscoveryMobileControllerClaimBoundarySchema),
  })
);

const V09HouseholdDiscoveryMobileControllerProductProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: V09HouseholdDiscoveryMobileControllerProductProofIdSchema,
  checkedAt: ParentTimestampSchema,
  sourceProofs: Schema.Array(V09HouseholdDiscoveryMobileControllerSourceProofInputSchema),
  productionDiscoveryStates: Schema.Array(LanPairingProductionDiscoveryStateSchema),
  routeChecks: Schema.Array(V09HouseholdDiscoveryMobileControllerRouteEvidenceSchema),
  mobileRoutes: Schema.Array(V09HouseholdDiscoveryMobileRouteEvidenceSchema),
  observerOperations: Schema.Array(V09HouseholdDiscoveryMobileControllerOperationEvidenceSchema),
  controllerTransitions: Schema.Array(V09HouseholdDiscoveryMobileControllerTransitionEvidenceSchema),
  manualProofBoundary: V09HouseholdDiscoveryMobileControllerManualBoundarySchema,
  claimsProved: Schema.Array(V09HouseholdDiscoveryMobileControllerProofLabelSchema),
  claimsNotProved: Schema.Array(V09HouseholdDiscoveryMobileControllerClaimBoundarySchema),
});

type V09HouseholdDiscoveryMobileControllerProductProofReadModelCandidate = Infer<
  typeof V09HouseholdDiscoveryMobileControllerProductProofReadModelBaseSchema
>;

export const V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema = withParser(
  V09HouseholdDiscoveryMobileControllerProductProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        householdDiscoveryMobileControllerProofIsHonest(readModel) ||
        'Expected V0.9 household discovery/mobile controller proof to keep physical household LAN, mobile write authority, cloud relay, and mobile background behavior manual or not implemented'
    )
  )
);

const RequiredSourceProofs = [
  'v0-9-production-discovery-household-proof',
  'v0-9-production-lan-mobile-controller-proof',
  'v0-9-mobile-controller-discovery-runtime-proof',
  'v0-9-mobile-controller-observer-runtime-proof',
  'parent-mobile-controller-observer-handoff-proof',
] as const satisfies ReadonlyArray<V09HouseholdDiscoveryMobileControllerSourceProof>;

const RequiredDiscoveryStates = [
  'discovered',
  'pending',
  'paired',
  'revoked',
  'stale',
  'offline',
  'unavailable',
] as const satisfies ReadonlyArray<Infer<typeof LanPairingProductionDiscoveryStateSchema>>;

const RequiredRouteChecks = [
  'paired-route-accepted',
  'failed-unpaired-rejected',
  'wrong-origin-rejected',
  'wrong-device-rejected',
  'replay-rejected',
  'revoked-pairing-rejected',
  'stale-source-rejected',
  'offline-device-rejected',
  'unavailable-route-rejected',
] as const satisfies ReadonlyArray<V09HouseholdDiscoveryMobileControllerRouteCheck>;

function householdDiscoveryMobileControllerProofIsHonest(
  readModel: V09HouseholdDiscoveryMobileControllerProductProofReadModelCandidate
): boolean {
  return (
    sourceProofsAreComplete(readModel.sourceProofs) &&
    discoveryStatesAreComplete(readModel.productionDiscoveryStates) &&
    routeChecksAreComplete(readModel.routeChecks) &&
    mobileRoutesAreHonest(readModel.mobileRoutes) &&
    observerOperationsAreHonest(readModel.observerOperations) &&
    controllerTransitionsAreComplete(readModel.controllerTransitions) &&
    manualBoundaryIsHonest(readModel.manualProofBoundary)
  );
}

function sourceProofsAreComplete(
  proofs: ReadonlyArray<V09HouseholdDiscoveryMobileControllerSourceProofInput>
): boolean {
  const sources = new Set(proofs.map((proof) => proof.source));
  return RequiredSourceProofs.every((source) => sources.has(source));
}

function discoveryStatesAreComplete(
  states: ReadonlyArray<Infer<typeof LanPairingProductionDiscoveryStateSchema>>
): boolean {
  const covered = new Set(states);
  return RequiredDiscoveryStates.every((state) => covered.has(state));
}

function routeChecksAreComplete(
  routeChecks: ReadonlyArray<V09HouseholdDiscoveryMobileControllerRouteEvidence>
): boolean {
  const byCheck = new Map(routeChecks.map((entry) => [entry.check, entry] as const));
  return (
    RequiredRouteChecks.every((check) => byCheck.has(check)) &&
    byCheck.get('paired-route-accepted')?.rejectionReason === null &&
    byCheck.get('failed-unpaired-rejected')?.rejectionReason === 'anonymous' &&
    byCheck.get('wrong-origin-rejected')?.rejectionReason === 'wrong-origin' &&
    byCheck.get('wrong-device-rejected')?.rejectionReason === 'wrong-device' &&
    byCheck.get('replay-rejected')?.rejectionReason === 'replayed'
  );
}

function mobileRoutesAreHonest(routes: ReadonlyArray<V09HouseholdDiscoveryMobileRouteEvidence>): boolean {
  const byPlatform = new Map(routes.map((route) => [route.platform, route] as const));
  return (
    byPlatform.get('android')?.commandAuthorityState === 'observer-read-only' &&
    byPlatform.get('ios')?.commandAuthorityState === 'controller-takeover-manual-required' &&
    routes.every((route) => route.controllerState !== 'active-controller')
  );
}

function observerOperationsAreHonest(
  operations: ReadonlyArray<V09HouseholdDiscoveryMobileControllerOperationEvidence>
): boolean {
  const byOperation = new Map(operations.map((operation) => [operation.operation, operation] as const));
  return (
    byOperation.get('observe-status')?.operationState === 'allowed-read-only' &&
    byOperation.get('request-controller-takeover')?.operationState === 'manual-required-mobile-package' &&
    byOperation.get('write-policy')?.operationState === 'rejected-observer-read-only' &&
    byOperation.get('pair-device')?.operationState === 'rejected-observer-read-only' &&
    byOperation.get('revoke-device')?.operationState === 'rejected-observer-read-only'
  );
}

function controllerTransitionsAreComplete(
  transitions: ReadonlyArray<V09HouseholdDiscoveryMobileControllerTransitionEvidence>
): boolean {
  const covered = new Set(transitions.map((transition) => transition.transition));
  return ['takeover', 'release', 'renew', 'degraded-provider', 'failed-unpaired'].every((transition) =>
    covered.has(transition as V09HouseholdDiscoveryMobileControllerTransitionEvidence['transition'])
  );
}

function manualBoundaryIsHonest(boundary: V09HouseholdDiscoveryMobileControllerManualBoundary): boolean {
  return (
    boundary.physicalHouseholdLan === 'manual-required' &&
    boundary.parentMobileWriteAuthority === 'manual-required' &&
    boundary.cloudRelayImplementation === 'not-implemented' &&
    boundary.cloudRelayDecision === 'manual-decision-required' &&
    boundary.mobileBackgroundBehavior === 'manual-required' &&
    boundary.physicalDeviceChecklist.length >= 5
  );
}

export type V09HouseholdDiscoveryMobileControllerProductProofId = Infer<
  typeof V09HouseholdDiscoveryMobileControllerProductProofIdSchema
>;
export type V09HouseholdDiscoveryMobileControllerSourceProof = Infer<
  typeof V09HouseholdDiscoveryMobileControllerSourceProofSchema
>;
export type V09HouseholdDiscoveryMobileControllerRouteCheck = Infer<
  typeof V09HouseholdDiscoveryMobileControllerRouteCheckSchema
>;
export type V09HouseholdDiscoveryMobileControllerSourceProofInput = Infer<
  typeof V09HouseholdDiscoveryMobileControllerSourceProofInputSchema
>;
export type V09HouseholdDiscoveryMobileControllerRouteEvidence = Infer<
  typeof V09HouseholdDiscoveryMobileControllerRouteEvidenceSchema
>;
export type V09HouseholdDiscoveryMobileRouteEvidence = Infer<typeof V09HouseholdDiscoveryMobileRouteEvidenceSchema>;
export type V09HouseholdDiscoveryMobileControllerOperationEvidence = Infer<
  typeof V09HouseholdDiscoveryMobileControllerOperationEvidenceSchema
>;
export type V09HouseholdDiscoveryMobileControllerTransitionEvidence = Infer<
  typeof V09HouseholdDiscoveryMobileControllerTransitionEvidenceSchema
>;
export type V09HouseholdDiscoveryMobileControllerManualBoundary = Infer<
  typeof V09HouseholdDiscoveryMobileControllerManualBoundarySchema
>;
export type V09HouseholdDiscoveryMobileControllerProductProofReadModel = Infer<
  typeof V09HouseholdDiscoveryMobileControllerProductProofReadModelSchema
>;
