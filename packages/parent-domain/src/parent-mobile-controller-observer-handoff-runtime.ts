import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LanPairingDeviceReachabilitySchema,
  LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingRouteIdSchema,
} from './lan-pairing-values';
import {
  LanProviderSelectionLifecycleStateSchema,
  LanProviderSelectionPolicyDecisionSchema,
} from './lan-pairing-provider-selection-proof';
import {
  ParentMobileCommandAuthorityStateSchema,
  ParentMobileControllerStateSchema,
  ParentMobileLocalModelExecutionStateSchema,
  ParentMobilePlatformSchema,
  type ParentMobileCommandAuthorityState,
  type ParentMobilePlatform,
} from './parent-mobile-runtime';
import {
  ParentMobileServiceBridgeAssistantJobStateSchema,
  ParentMobileServiceBridgeProofStateSchema,
  ParentMobileServiceBridgeRoleSchema,
  ParentMobileServiceBridgeRuntimeOwnerSchema,
  type ParentMobileServiceBridgeAssistantJobState,
  type ParentMobileServiceBridgeRuntimeOwner,
} from './parent-mobile-service-bridge-runtime';
import { ParentDeviceIdSchema, ParentEvidenceReferenceIdSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyParentMobileHandoffText = Schema.String.pipe(Schema.minLength(1));

export const ParentMobileControllerObserverHandoffRuntimeSchemaVersionSchema = withParser(
  Schema.Literal('parent-mobile-controller-observer-handoff-proof')
);
export const ParentMobileControllerObserverHandoffRoleSchema = ParentMobileServiceBridgeRoleSchema;
export const ParentMobileControllerObserverHandoffPhaseSchema = withParser(
  Schema.Literal(
    'observe-controller-lease',
    'observe-selected-route',
    'request-controller-takeover',
    'deny-controller-takeover',
    'degrade-controller-session',
    'release-controller-lease',
    'handoff-lan-ai-provider',
    'disable-phone-local-model',
    'refuse-cloud-relay'
  )
);
export const ParentMobileControllerObserverHandoffStateSchema = withParser(
  Schema.Literal(
    'observed-read-only',
    'manual-required',
    'denied',
    'degraded',
    'released',
    'unavailable',
    'disabled-by-default',
    'not-implemented'
  )
);
export const ParentMobileControllerObserverHandoffResponseStateSchema = withParser(
  Schema.Literal('completed', 'rejected', 'degraded', 'unavailable', 'not-implemented')
);
export const ParentMobileControllerObserverHandoffLeaseStateSchema = withParser(
  Schema.Literal('visible-read-only', 'manual-required', 'released-by-service', 'unavailable')
);
export const ParentMobileControllerObserverHandoffRouteStateSchema = withParser(
  Schema.Literal(
    'selected-route-visible',
    'selected-route-degraded',
    'provider-unavailable',
    'manual-required',
    'cloud-relay-not-implemented'
  )
);
export const ParentMobileControllerObserverHandoffProofSourceSchema = withParser(
  Schema.Literal(
    'parent-mobile-service-bridge-proof',
    'v0-9-production-lan-mobile-controller-proof',
    'v0-9-mobile-controller-discovery-runtime-proof',
    'v0-9-prod-discovery-provider-selection-proof'
  )
);

const ParentMobileControllerObserverHandoffProofPathSchema = NonEmptyParentMobileHandoffText.pipe(
  Schema.brand('ParentMobileControllerObserverHandoffProofPath')
);
const ParentMobileControllerObserverHandoffProofCommandSchema = NonEmptyParentMobileHandoffText.pipe(
  Schema.brand('ParentMobileControllerObserverHandoffProofCommand')
);
const ParentMobileControllerObserverHandoffProofLabelSchema = NonEmptyParentMobileHandoffText.pipe(
  Schema.brand('ParentMobileControllerObserverHandoffProofLabel')
);
const ParentMobileControllerObserverHandoffRequirementSchema = NonEmptyParentMobileHandoffText.pipe(
  Schema.brand('ParentMobileControllerObserverHandoffRequirement')
);
const ParentMobileControllerObserverHandoffClaimBoundarySchema = NonEmptyParentMobileHandoffText.pipe(
  Schema.brand('ParentMobileControllerObserverHandoffClaimBoundary')
);
const ParentMobileControllerObserverHandoffUnavailableReasonSchema = NonEmptyParentMobileHandoffText.pipe(
  Schema.brand('ParentMobileControllerObserverHandoffUnavailableReason')
);
const ParentMobileControllerObserverHandoffProviderIdSchema = NonEmptyParentMobileHandoffText.pipe(
  Schema.brand('ParentMobileControllerObserverHandoffProviderId')
);

export const ParentMobileControllerObserverHandoffProofInputSchema = withParser(
  Schema.Struct({
    source: ParentMobileControllerObserverHandoffProofSourceSchema,
    path: ParentMobileControllerObserverHandoffProofPathSchema,
    command: ParentMobileControllerObserverHandoffProofCommandSchema,
  })
);

export const ParentMobileControllerObserverLeaseSnapshotSchema = withParser(
  Schema.Struct({
    leaseState: ParentMobileControllerObserverHandoffLeaseStateSchema,
    controllerState: ParentMobileControllerStateSchema,
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    controllerLeaseVisible: Schema.Boolean,
    controllerDeviceId: Schema.Union(ParentDeviceIdSchema, Schema.Null),
    handoffRequirement: ParentMobileControllerObserverHandoffRequirementSchema,
  })
);

export const ParentMobileControllerObserverRouteSnapshotSchema = withParser(
  Schema.Struct({
    routeState: ParentMobileControllerObserverHandoffRouteStateSchema,
    selectedRouteId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    discoveryState: LanPairingProductionDiscoveryStateSchema,
    reachability: LanPairingDeviceReachabilitySchema,
    providerLifecycleState: LanProviderSelectionLifecycleStateSchema,
    providerPolicyDecision: LanProviderSelectionPolicyDecisionSchema,
    providerId: Schema.Union(ParentMobileControllerObserverHandoffProviderIdSchema, Schema.Null),
    localServiceState: ParentMobileServiceBridgeProofStateSchema,
    lanServiceState: ParentMobileServiceBridgeProofStateSchema,
    cloudRelayState: ParentMobileServiceBridgeProofStateSchema,
    parentCacheState: ParentMobileServiceBridgeProofStateSchema,
    parentOwnedStorageState: ParentMobileServiceBridgeProofStateSchema,
    routeRequirement: ParentMobileControllerObserverHandoffRequirementSchema,
  })
);

export const ParentMobileControllerObserverLanAiHandoffSchema = withParser(
  Schema.Struct({
    jobState: ParentMobileServiceBridgeAssistantJobStateSchema,
    routeState: ParentMobileControllerObserverHandoffRouteStateSchema,
    providerId: Schema.Union(ParentMobileControllerObserverHandoffProviderIdSchema, Schema.Null),
    unavailableReason: Schema.Union(ParentMobileControllerObserverHandoffUnavailableReasonSchema, Schema.Null),
    localModelExecutionState: ParentMobileLocalModelExecutionStateSchema,
    localModelExecutionAllowed: Schema.Literal(false),
    evidenceReferenceIds: Schema.Array(ParentEvidenceReferenceIdSchema),
  })
);

export const ParentMobileControllerObserverHandoffStepSchema = withParser(
  Schema.Struct({
    phase: ParentMobileControllerObserverHandoffPhaseSchema,
    responseState: ParentMobileControllerObserverHandoffResponseStateSchema,
    handoffState: ParentMobileControllerObserverHandoffStateSchema,
    runtimeOwner: ParentMobileServiceBridgeRuntimeOwnerSchema,
    commandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    unavailableReason: Schema.Union(ParentMobileControllerObserverHandoffUnavailableReasonSchema, Schema.Null),
    proofLabel: ParentMobileControllerObserverHandoffProofLabelSchema,
    proofRequirement: ParentMobileControllerObserverHandoffRequirementSchema,
  })
);

export const ParentMobileControllerObserverHandoffReadModelSchema = withParser(
  Schema.Struct({
    platform: ParentMobilePlatformSchema,
    parentDeviceId: ParentDeviceIdSchema,
    role: ParentMobileControllerObserverHandoffRoleSchema,
    leaseSnapshot: ParentMobileControllerObserverLeaseSnapshotSchema,
    routeSnapshot: ParentMobileControllerObserverRouteSnapshotSchema,
    lanAiHandoff: ParentMobileControllerObserverLanAiHandoffSchema,
    handoffSteps: Schema.Array(ParentMobileControllerObserverHandoffStepSchema),
  })
);

export const ParentMobileControllerObserverHandoffProofHarnessSchema = withParser(
  Schema.Struct({
    sourceProofs: Schema.Array(ParentMobileControllerObserverHandoffProofInputSchema),
    outputProofPath: ParentMobileControllerObserverHandoffProofPathSchema,
    checkpointPath: ParentMobileControllerObserverHandoffProofPathSchema,
  })
);

export const ParentMobileControllerObserverHandoffClaimBoundariesSchema = withParser(
  Schema.Struct({
    parentMobileWriteAuthority: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    mobileParity: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    childMobileAgentBehavior: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    androidChildAgentBehavior: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    iosChildAgentBehavior: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    androidDeviceOwner: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    iosFamilyControls: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    signingStoresEntitlements: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    cloudRelay: ParentMobileControllerObserverHandoffClaimBoundarySchema,
    cUiOwnership: ParentMobileControllerObserverHandoffClaimBoundarySchema,
  })
);

const ParentMobileControllerObserverHandoffRuntimeReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentMobileControllerObserverHandoffRuntimeSchemaVersionSchema,
  proofHarness: ParentMobileControllerObserverHandoffProofHarnessSchema,
  handoffReadModels: Schema.Array(ParentMobileControllerObserverHandoffReadModelSchema),
  claimBoundaries: ParentMobileControllerObserverHandoffClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ParentMobileControllerObserverHandoffRuntimeReadModelCandidate = Infer<
  typeof ParentMobileControllerObserverHandoffRuntimeReadModelBaseSchema
>;

export const ParentMobileControllerObserverHandoffRuntimeReadModelSchema = withParser(
  ParentMobileControllerObserverHandoffRuntimeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        parentMobileControllerObserverHandoffRuntimeIsHonest(readModel) ||
        'Expected parent mobile controller-observer handoff proof to keep observer authority read-only, takeover manual/denied, LAN AI degraded or unavailable, cloud relay not implemented, and phone-local model execution disabled'
    )
  )
);

const RequiredSources = [
  'parent-mobile-service-bridge-proof',
  'v0-9-production-lan-mobile-controller-proof',
  'v0-9-mobile-controller-discovery-runtime-proof',
  'v0-9-prod-discovery-provider-selection-proof',
] as const satisfies ReadonlyArray<ParentMobileControllerObserverHandoffProofSource>;

const RequiredHandoffPhases = [
  'observe-controller-lease',
  'observe-selected-route',
  'request-controller-takeover',
  'deny-controller-takeover',
  'degrade-controller-session',
  'release-controller-lease',
  'handoff-lan-ai-provider',
  'disable-phone-local-model',
  'refuse-cloud-relay',
] as const satisfies ReadonlyArray<ParentMobileControllerObserverHandoffPhase>;

type HandoffStepExpectation = Readonly<{
  responseState: ParentMobileControllerObserverHandoffResponseState;
  handoffState: ParentMobileControllerObserverHandoffState;
  runtimeOwner: ParentMobileServiceBridgeRuntimeOwner;
  commandAuthorityState: ParentMobileCommandAuthorityState;
  rejectionReason: Infer<typeof LanPairingRejectionReasonSchema> | null;
}>;

const HandoffStepExpectations: ReadonlyMap<ParentMobileControllerObserverHandoffPhase, HandoffStepExpectation> =
  new Map([
    stepExpectation('observe-controller-lease', 'completed', 'observed-read-only', 'parent-mobile-shell', null),
    stepExpectation('observe-selected-route', 'completed', 'observed-read-only', 'parent-mobile-shell', null),
    stepExpectation(
      'request-controller-takeover',
      'rejected',
      'manual-required',
      'manual-proof',
      'takeover-denied',
      'controller-takeover-manual-required'
    ),
    stepExpectation(
      'deny-controller-takeover',
      'rejected',
      'denied',
      'agent-service',
      'takeover-denied',
      'observer-read-only'
    ),
    stepExpectation(
      'degrade-controller-session',
      'degraded',
      'degraded',
      'agent-service',
      'lan-ai-provider-unavailable'
    ),
    stepExpectation('release-controller-lease', 'completed', 'released', 'agent-service', null),
    stepExpectation('disable-phone-local-model', 'rejected', 'disabled-by-default', 'parent-mobile-shell', null),
    stepExpectation('refuse-cloud-relay', 'not-implemented', 'not-implemented', 'cloud-relay-not-implemented', null),
  ]);

function stepExpectation(
  phase: ParentMobileControllerObserverHandoffPhase,
  responseState: ParentMobileControllerObserverHandoffResponseState,
  handoffState: ParentMobileControllerObserverHandoffState,
  runtimeOwner: ParentMobileServiceBridgeRuntimeOwner,
  rejectionReason: Infer<typeof LanPairingRejectionReasonSchema> | null,
  commandAuthorityState: ParentMobileCommandAuthorityState = 'observer-read-only'
): readonly [ParentMobileControllerObserverHandoffPhase, HandoffStepExpectation] {
  return [phase, { responseState, handoffState, runtimeOwner, commandAuthorityState, rejectionReason }];
}

function parentMobileControllerObserverHandoffRuntimeIsHonest(
  readModel: ParentMobileControllerObserverHandoffRuntimeReadModelCandidate
): boolean {
  return proofHarnessIsComplete(readModel.proofHarness) && handoffReadModelsAreComplete(readModel.handoffReadModels);
}

function proofHarnessIsComplete(proofHarness: ParentMobileControllerObserverHandoffProofHarness): boolean {
  const sources = new Set(proofHarness.sourceProofs.map((proof) => proof.source));
  return (
    RequiredSources.every((source) => sources.has(source)) &&
    proofHarness.outputProofPath.includes('parent-mobile-controller-observer-handoff-proof/proof.json') &&
    proofHarness.checkpointPath.includes('parent-mobile-controller-observer-handoff-proof-2026-05-30.md')
  );
}

function handoffReadModelsAreComplete(
  readModels: ReadonlyArray<ParentMobileControllerObserverHandoffReadModel>
): boolean {
  const platforms = new Set(readModels.map((readModel) => readModel.platform));
  return (
    readModels.length === 2 &&
    platforms.has('android') &&
    platforms.has('ios') &&
    readModels.every((readModel) => handoffReadModelIsHonest(readModel))
  );
}

function handoffReadModelIsHonest(readModel: ParentMobileControllerObserverHandoffReadModel): boolean {
  if (
    readModel.leaseSnapshot.controllerState === 'active-controller' ||
    readModel.leaseSnapshot.commandAuthorityState === 'active-controller-backend-proof'
  ) {
    return false;
  }

  return (
    leaseSnapshotIsHonest(readModel.leaseSnapshot, readModel.platform) &&
    routeSnapshotIsHonest(readModel.routeSnapshot) &&
    lanAiHandoffIsHonest(readModel.lanAiHandoff) &&
    handoffStepsAreHonest(readModel.handoffSteps, readModel.lanAiHandoff.jobState)
  );
}

function leaseSnapshotIsHonest(
  leaseSnapshot: ParentMobileControllerObserverLeaseSnapshot,
  platform: ParentMobilePlatform
): boolean {
  if (platform === 'android') {
    return (
      leaseSnapshot.leaseState === 'visible-read-only' &&
      leaseSnapshot.controllerState === 'observer' &&
      leaseSnapshot.commandAuthorityState === 'observer-read-only' &&
      leaseSnapshot.controllerLeaseVisible === true
    );
  }

  return (
    leaseSnapshot.leaseState === 'manual-required' &&
    leaseSnapshot.controllerState === 'manual-required' &&
    leaseSnapshot.commandAuthorityState === 'controller-takeover-manual-required' &&
    leaseSnapshot.controllerLeaseVisible === false &&
    leaseSnapshot.controllerDeviceId === null
  );
}

function routeSnapshotIsHonest(routeSnapshot: ParentMobileControllerObserverRouteSnapshot): boolean {
  return (
    routeSnapshot.cloudRelayState === 'not-implemented' &&
    routeSnapshot.parentCacheState === 'stale' &&
    routeSnapshot.parentOwnedStorageState === 'offline' &&
    routeSnapshot.localServiceState !== 'available' &&
    routeSnapshot.providerLifecycleState !== 'candidate-selected' &&
    routeSnapshot.routeState !== 'cloud-relay-not-implemented' &&
    routeSnapshot.providerPolicyDecision !== 'select-authorized-provider'
  );
}

function lanAiHandoffIsHonest(handoff: ParentMobileControllerObserverLanAiHandoff): boolean {
  if (handoff.localModelExecutionAllowed !== false || handoff.localModelExecutionState !== 'disabled-by-default') {
    return false;
  }

  return (
    (handoff.jobState === 'degraded' || handoff.jobState === 'unavailable') &&
    handoff.providerId === null &&
    handoff.unavailableReason !== null
  );
}

function handoffStepsAreHonest(
  steps: ReadonlyArray<ParentMobileControllerObserverHandoffStep>,
  aiState: ParentMobileServiceBridgeAssistantJobState
): boolean {
  const byPhase = new Map(steps.map((step) => [step.phase, step] as const));
  return (
    byPhase.size === steps.length &&
    RequiredHandoffPhases.every((phase) => byPhase.has(phase)) &&
    steps.every((step) => handoffStepIsHonest(step, aiState))
  );
}

function handoffStepIsHonest(
  step: ParentMobileControllerObserverHandoffStep,
  aiState: ParentMobileServiceBridgeAssistantJobState
): boolean {
  if (step.phase === 'handoff-lan-ai-provider') {
    return lanAiHandoffStepIsHonest(step, aiState);
  }

  const expected = HandoffStepExpectations.get(step.phase);
  return (
    expected !== undefined &&
    step.responseState === expected.responseState &&
    step.handoffState === expected.handoffState &&
    step.runtimeOwner === expected.runtimeOwner &&
    step.commandAuthorityState === expected.commandAuthorityState &&
    step.rejectionReason === expected.rejectionReason
  );
}

function lanAiHandoffStepIsHonest(
  step: ParentMobileControllerObserverHandoffStep,
  aiState: ParentMobileServiceBridgeAssistantJobState
): boolean {
  return (
    (aiState === 'degraded' || aiState === 'unavailable') &&
    step.responseState === aiState &&
    step.handoffState === aiState &&
    step.runtimeOwner === 'lan-ai-provider' &&
    step.commandAuthorityState === 'observer-read-only' &&
    step.rejectionReason === 'lan-ai-provider-unavailable' &&
    step.unavailableReason !== null
  );
}

export type ParentMobileControllerObserverHandoffPhase = Infer<typeof ParentMobileControllerObserverHandoffPhaseSchema>;
export type ParentMobileControllerObserverHandoffState = Infer<typeof ParentMobileControllerObserverHandoffStateSchema>;
export type ParentMobileControllerObserverHandoffResponseState = Infer<
  typeof ParentMobileControllerObserverHandoffResponseStateSchema
>;
export type ParentMobileControllerObserverHandoffLeaseState = Infer<
  typeof ParentMobileControllerObserverHandoffLeaseStateSchema
>;
export type ParentMobileControllerObserverHandoffRouteState = Infer<
  typeof ParentMobileControllerObserverHandoffRouteStateSchema
>;
export type ParentMobileControllerObserverHandoffProofSource = Infer<
  typeof ParentMobileControllerObserverHandoffProofSourceSchema
>;
export type ParentMobileControllerObserverHandoffProofInput = Infer<
  typeof ParentMobileControllerObserverHandoffProofInputSchema
>;
export type ParentMobileControllerObserverLeaseSnapshot = Infer<
  typeof ParentMobileControllerObserverLeaseSnapshotSchema
>;
export type ParentMobileControllerObserverRouteSnapshot = Infer<
  typeof ParentMobileControllerObserverRouteSnapshotSchema
>;
export type ParentMobileControllerObserverLanAiHandoff = Infer<typeof ParentMobileControllerObserverLanAiHandoffSchema>;
export type ParentMobileControllerObserverHandoffStep = Infer<typeof ParentMobileControllerObserverHandoffStepSchema>;
export type ParentMobileControllerObserverHandoffReadModel = Infer<
  typeof ParentMobileControllerObserverHandoffReadModelSchema
>;
export type ParentMobileControllerObserverHandoffProofHarness = Infer<
  typeof ParentMobileControllerObserverHandoffProofHarnessSchema
>;
export type ParentMobileControllerObserverHandoffClaimBoundaries = Infer<
  typeof ParentMobileControllerObserverHandoffClaimBoundariesSchema
>;
export type ParentMobileControllerObserverHandoffRuntimeReadModel = Infer<
  typeof ParentMobileControllerObserverHandoffRuntimeReadModelSchema
>;
