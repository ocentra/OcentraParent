import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { LanPairingRejectionReasonSchema, LanPairingRouteIdSchema } from './lan-pairing-values';
import { ParentMobileCommandAuthorityStateSchema } from './parent-mobile-runtime';
import { ParentTimestampSchema } from './family-reference-primitives';
import {
  V09HouseholdDiscoveryMobileControllerCloudRelayDecisionSchema,
  V09HouseholdDiscoveryMobileControllerRouteCheckSchema,
  type V09HouseholdDiscoveryMobileControllerRouteCheck,
} from './v0-9-household-discovery-mobile-controller-product-proof';
import {
  V09HouseholdPhysicalProofArtifactRequirementEvidenceSchema,
  V09HouseholdPhysicalProofManualEvidenceCustodySchema,
  V09HouseholdPhysicalProofManualEvidenceStatusSchema,
} from './v0-9-household-physical-proof-artifact-gate';
import { V09RuntimeProofStateSchema } from './v0-9-mobile-controller-discovery-runtime';

export const V09HouseholdMultiDeviceProofGateIdSchema = withParser(
  Schema.Literal('v0-9-household-multi-device-proof-gates')
);

export const V09HouseholdMultiDeviceProofGateSourceSchema = withParser(
  Schema.Literal(
    'v0-9-household-discovery-mobile-controller-product-proof',
    'v0-9-household-physical-proof-artifact-gate',
    'v0-9-production-lan-multidevice-hardening'
  )
);

const V09HouseholdMultiDeviceProofReadinessDecisionSchema = withParser(
  Schema.Literal('manual-gate-required-before-household-multi-device-readiness')
);

export const V09HouseholdMultiDeviceRouteCustodyGateSchema = withParser(
  Schema.Literal(
    'paired-household-route-evidence',
    'failed-unpaired-household-route-evidence',
    'allowed-origin-rejection-custody',
    'wrong-device-rejection-custody',
    'replay-rejection-custody',
    'revocation-rejection-custody',
    'stale-offline-rejection-custody',
    'unsupported-route-custody'
  )
);

const V09HouseholdMultiDeviceProofPathSchema = brandedNonEmptyStringSchema('V09HouseholdMultiDeviceProofPath');
const V09HouseholdMultiDeviceProofCommandSchema = brandedNonEmptyStringSchema('V09HouseholdMultiDeviceProofCommand');
const V09HouseholdMultiDeviceProofLabelSchema = brandedNonEmptyStringSchema('V09HouseholdMultiDeviceProofLabel');
const V09HouseholdMultiDeviceClaimBoundarySchema = brandedNonEmptyStringSchema('V09HouseholdMultiDeviceClaimBoundary');

const V09HouseholdMultiDeviceSourceProofInputSchema = withParser(
  Schema.Struct({
    source: V09HouseholdMultiDeviceProofGateSourceSchema,
    path: V09HouseholdMultiDeviceProofPathSchema,
    command: V09HouseholdMultiDeviceProofCommandSchema,
  })
);

const V09HouseholdMultiDeviceRouteCustodyEvidenceSchema = withParser(
  Schema.Struct({
    check: V09HouseholdDiscoveryMobileControllerRouteCheckSchema,
    custodyGate: V09HouseholdMultiDeviceRouteCustodyGateSchema,
    routeId: LanPairingRouteIdSchema,
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: V09RuntimeProofStateSchema,
    manualArtifactStatus: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceSelectedTrustedStorageEvidenceSchema = withParser(
  Schema.Struct({
    storageState: V09RuntimeProofStateSchema,
    securityState: V09RuntimeProofStateSchema,
    selectedRouteRecoveryLabelCount: Schema.Number,
    trustedRegistryLabelCount: Schema.Number,
    selectedRouteTrustLabelCount: Schema.Number,
    selectedDeviceRejectionLabelCount: Schema.Number,
    wrongDeviceRejectionLabel: V09HouseholdMultiDeviceProofLabelSchema,
    manualArtifactStatus: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceCloudRelayBoundarySchema = withParser(
  Schema.Struct({
    implementationState: V09RuntimeProofStateSchema,
    remoteControlState: V09RuntimeProofStateSchema,
    decision: V09HouseholdDiscoveryMobileControllerCloudRelayDecisionSchema,
    manualDecisionLabel: V09HouseholdMultiDeviceClaimBoundarySchema,
  })
);

export const V09HouseholdMultiDeviceVisibleDeviceStateSchema = withParser(
  Schema.Literal('paired', 'offline', 'stale', 'manual-required')
);

export const V09HouseholdMultiDeviceLanAiProviderReadinessSchema = withParser(
  Schema.Literal('local-provider-ready', 'mobile-provider-degraded', 'manual-required')
);

export const V09HouseholdMultiDeviceLanDiscoverySourceSchema = withParser(
  Schema.Literal('local-service-discovery-proof', 'physical-lan-discovery-manual-required')
);

export const V09HouseholdMultiDevicePairingRequestStateSchema = withParser(
  Schema.Literal('discovered', 'pending', 'paired', 'rejected', 'expired', 'revoked', 'stale', 'offline')
);

const V09HouseholdMultiDeviceVisibleDeviceSchema = withParser(
  Schema.Struct({
    deviceLabel: V09HouseholdMultiDeviceProofLabelSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    deviceState: V09HouseholdMultiDeviceVisibleDeviceStateSchema,
    routeProofState: V09RuntimeProofStateSchema,
    artifactGateStatus: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceLanDiscoveryBoundarySchema = withParser(
  Schema.Struct({
    sourceState: V09HouseholdMultiDeviceLanDiscoverySourceSchema,
    discoverableDeviceState: V09RuntimeProofStateSchema,
    physicalLanDiscoveryState: V09RuntimeProofStateSchema,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceAddDevicePairingRequestSchema = withParser(
  Schema.Struct({
    requestState: V09HouseholdMultiDevicePairingRequestStateSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: V09RuntimeProofStateSchema,
    manualArtifactStatus: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceTrustedRegistryEntrySchema = withParser(
  Schema.Struct({
    routeId: LanPairingRouteIdSchema,
    pairingState: V09HouseholdMultiDevicePairingRequestStateSchema,
    deviceState: V09HouseholdMultiDeviceVisibleDeviceStateSchema,
    registryProofState: V09RuntimeProofStateSchema,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceTrustedDeviceRegistrySchema = withParser(
  Schema.Struct({
    registryProofState: V09RuntimeProofStateSchema,
    entries: Schema.Array(V09HouseholdMultiDeviceTrustedRegistryEntrySchema),
    selectedRouteRecoveryLabelCount: Schema.Number,
    trustedRegistryLabelCount: Schema.Number,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceHouseholdDeviceRegistrySchema = withParser(
  Schema.Struct({
    registryProofState: V09RuntimeProofStateSchema,
    devices: Schema.Array(V09HouseholdMultiDeviceVisibleDeviceSchema),
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceSelectedDeviceReadinessSchema = withParser(
  Schema.Struct({
    selectedRouteId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    selectedDeviceState: V09HouseholdMultiDeviceVisibleDeviceStateSchema,
    routeProofState: V09RuntimeProofStateSchema,
    physicalArtifactStatus: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    manualRequiredLabel: V09HouseholdMultiDeviceClaimBoundarySchema,
  })
);

const V09HouseholdMultiDeviceVisibleRouteStateSchema = withParser(
  Schema.Struct({
    currentControllerRouteId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    currentObserverRouteId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    controllerCommandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    observerCommandAuthorityState: ParentMobileCommandAuthorityStateSchema,
    manualControllerTakeoverState: ParentMobileCommandAuthorityStateSchema,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

const V09HouseholdMultiDeviceLanAiProviderReadinessEvidenceSchema = withParser(
  Schema.Struct({
    readinessState: V09HouseholdMultiDeviceLanAiProviderReadinessSchema,
    localProviderState: V09RuntimeProofStateSchema,
    mobileProviderState: V09RuntimeProofStateSchema,
    physicalProviderArtifactStatus: V09HouseholdPhysicalProofManualEvidenceStatusSchema,
    evidenceLabels: Schema.Array(V09HouseholdMultiDeviceProofLabelSchema),
  })
);

const V09HouseholdMultiDeviceArtifactReadinessGateSummarySchema = withParser(
  Schema.Struct({
    requiredArtifactCount: Schema.Number,
    collectedArtifactCount: Schema.Number,
    missingArtifactCount: Schema.Number,
    physicalReadinessState: V09RuntimeProofStateSchema,
    cloudRelayState: V09RuntimeProofStateSchema,
    evidenceLabel: V09HouseholdMultiDeviceProofLabelSchema,
  })
);

export const V09HouseholdMultiDevicePortalDeviceSpineSchema = withParser(
  Schema.Struct({
    lanDiscoveryBoundary: V09HouseholdMultiDeviceLanDiscoveryBoundarySchema,
    householdDeviceRegistry: V09HouseholdMultiDeviceHouseholdDeviceRegistrySchema,
    addDevicePairingRequests: Schema.Array(V09HouseholdMultiDeviceAddDevicePairingRequestSchema),
    trustedDeviceRegistry: V09HouseholdMultiDeviceTrustedDeviceRegistrySchema,
    selectedDeviceReadiness: V09HouseholdMultiDeviceSelectedDeviceReadinessSchema,
    routeState: V09HouseholdMultiDeviceVisibleRouteStateSchema,
    lanAiProviderReadiness: V09HouseholdMultiDeviceLanAiProviderReadinessEvidenceSchema,
    artifactReadinessGates: V09HouseholdMultiDeviceArtifactReadinessGateSummarySchema,
    adapterBoundaryLabel: V09HouseholdMultiDeviceClaimBoundarySchema,
  })
);

const V09HouseholdMultiDeviceProofGateReadModelBaseSchema = Schema.Struct({
  schemaVersion: V09HouseholdMultiDeviceProofGateIdSchema,
  checkedAt: ParentTimestampSchema,
  readinessDecision: V09HouseholdMultiDeviceProofReadinessDecisionSchema,
  householdMultiDeviceReadinessState: V09RuntimeProofStateSchema,
  localMultiServiceProofState: V09RuntimeProofStateSchema,
  sourceProofs: Schema.Array(V09HouseholdMultiDeviceSourceProofInputSchema),
  physicalArtifactRequirements: Schema.Array(V09HouseholdPhysicalProofArtifactRequirementEvidenceSchema),
  manualEvidenceStatus: V09HouseholdPhysicalProofManualEvidenceCustodySchema,
  routeCustody: Schema.Array(V09HouseholdMultiDeviceRouteCustodyEvidenceSchema),
  selectedTrustedDeviceStorage: V09HouseholdMultiDeviceSelectedTrustedStorageEvidenceSchema,
  cloudRelayBoundary: V09HouseholdMultiDeviceCloudRelayBoundarySchema,
  portalDeviceSpine: V09HouseholdMultiDevicePortalDeviceSpineSchema,
  claimsProved: Schema.Array(V09HouseholdMultiDeviceProofLabelSchema),
  claimsNotProved: Schema.Array(V09HouseholdMultiDeviceClaimBoundarySchema),
});

type V09HouseholdMultiDeviceProofGateReadModelCandidate = Infer<
  typeof V09HouseholdMultiDeviceProofGateReadModelBaseSchema
>;

export const V09HouseholdMultiDeviceProofGateReadModelSchema = withParser(
  V09HouseholdMultiDeviceProofGateReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        householdMultiDeviceProofGateIsHonest(readModel) ||
        'Expected V0.9 household multi-device proof gates to preserve manual-required physical readiness and not-implemented cloud relay boundaries'
    )
  )
);

const RequiredSourceProofs = [
  'v0-9-household-discovery-mobile-controller-product-proof',
  'v0-9-household-physical-proof-artifact-gate',
  'v0-9-production-lan-multidevice-hardening',
] as const satisfies ReadonlyArray<V09HouseholdMultiDeviceProofGateSource>;

const RequiredRouteCustody = new Map<
  V09HouseholdDiscoveryMobileControllerRouteCheck,
  {
    custodyGate: V09HouseholdMultiDeviceRouteCustodyGate;
    rejectionReason: V09HouseholdMultiDeviceRouteCustodyEvidence['rejectionReason'];
  }
>([
  ['paired-route-accepted', { custodyGate: 'paired-household-route-evidence', rejectionReason: null }],
  [
    'failed-unpaired-rejected',
    { custodyGate: 'failed-unpaired-household-route-evidence', rejectionReason: 'anonymous' },
  ],
  ['wrong-origin-rejected', { custodyGate: 'allowed-origin-rejection-custody', rejectionReason: 'wrong-origin' }],
  ['wrong-device-rejected', { custodyGate: 'wrong-device-rejection-custody', rejectionReason: 'wrong-device' }],
  ['replay-rejected', { custodyGate: 'replay-rejection-custody', rejectionReason: 'replayed' }],
  ['revoked-pairing-rejected', { custodyGate: 'revocation-rejection-custody', rejectionReason: 'revoked' }],
  ['stale-source-rejected', { custodyGate: 'stale-offline-rejection-custody', rejectionReason: 'stale' }],
  ['offline-device-rejected', { custodyGate: 'stale-offline-rejection-custody', rejectionReason: 'offline' }],
  ['unavailable-route-rejected', { custodyGate: 'unsupported-route-custody', rejectionReason: 'unsupported-route' }],
]);

const RequiredPairingRequestStates = [
  'discovered',
  'pending',
  'paired',
  'rejected',
  'expired',
  'revoked',
  'stale',
  'offline',
] as const satisfies ReadonlyArray<V09HouseholdMultiDevicePairingRequestState>;

function householdMultiDeviceProofGateIsHonest(readModel: V09HouseholdMultiDeviceProofGateReadModelCandidate): boolean {
  return (
    readModel.householdMultiDeviceReadinessState === 'manual-required' &&
    readModel.localMultiServiceProofState === 'ci-mechanical-proof' &&
    sourceProofsAreComplete(readModel.sourceProofs) &&
    physicalArtifactsRemainManualGated(readModel) &&
    routeCustodyIsComplete(readModel.routeCustody) &&
    selectedTrustedStorageFollowsThrough(readModel.selectedTrustedDeviceStorage) &&
    cloudRelayBoundaryIsHonest(readModel.cloudRelayBoundary) &&
    portalDeviceSpineIsComplete(readModel.portalDeviceSpine) &&
    readModel.claimsNotProved.some((claim) => claim.includes('remote desktop')) &&
    readModel.claimsNotProved.some((claim) => claim.includes('physical household LAN readiness')) &&
    readModel.claimsNotProved.some((claim) => claim.includes('cloud relay'))
  );
}

function sourceProofsAreComplete(proofs: ReadonlyArray<V09HouseholdMultiDeviceSourceProofInput>): boolean {
  const sources = new Set(proofs.map((proof) => proof.source));
  return RequiredSourceProofs.every((source) => sources.has(source));
}

function physicalArtifactsRemainManualGated(readModel: V09HouseholdMultiDeviceProofGateReadModelCandidate): boolean {
  return (
    readModel.physicalArtifactRequirements.length >= 11 &&
    readModel.physicalArtifactRequirements.every((artifact) => artifact.status === 'manual-required') &&
    readModel.manualEvidenceStatus.custodyState === 'not-collected' &&
    readModel.manualEvidenceStatus.collectedArtifactCount === 0 &&
    readModel.manualEvidenceStatus.missingArtifactCount === readModel.physicalArtifactRequirements.length
  );
}

function routeCustodyIsComplete(routeCustody: ReadonlyArray<V09HouseholdMultiDeviceRouteCustodyEvidence>): boolean {
  const byCheck = new Map(routeCustody.map((entry) => [entry.check, entry] as const));
  for (const [check, expected] of RequiredRouteCustody.entries()) {
    const entry = byCheck.get(check);
    if (
      entry === undefined ||
      entry.custodyGate !== expected.custodyGate ||
      entry.rejectionReason !== expected.rejectionReason ||
      entry.manualArtifactStatus !== 'manual-required'
    ) {
      return false;
    }
  }
  return true;
}

function selectedTrustedStorageFollowsThrough(
  evidence: V09HouseholdMultiDeviceSelectedTrustedStorageEvidence
): boolean {
  return (
    evidence.storageState === 'ci-mechanical-proof' &&
    evidence.securityState === 'ci-mechanical-proof' &&
    evidence.selectedRouteRecoveryLabelCount >= 2 &&
    evidence.trustedRegistryLabelCount >= 2 &&
    evidence.selectedRouteTrustLabelCount >= 3 &&
    evidence.selectedDeviceRejectionLabelCount >= 8 &&
    evidence.wrongDeviceRejectionLabel.includes('wrong-device') &&
    evidence.manualArtifactStatus === 'manual-required'
  );
}

function cloudRelayBoundaryIsHonest(boundary: V09HouseholdMultiDeviceCloudRelayBoundary): boolean {
  return (
    boundary.implementationState === 'not-implemented' &&
    boundary.remoteControlState === 'not-implemented' &&
    boundary.decision === 'manual-decision-required'
  );
}

function portalDeviceSpineIsComplete(spine: V09HouseholdMultiDevicePortalDeviceSpine): boolean {
  return (
    lanDiscoveryBoundaryIsRenderable(spine) &&
    householdDeviceRegistryIsRenderable(spine) &&
    pairingRequestStatesAreComplete(spine.addDevicePairingRequests) &&
    trustedDeviceRegistryIsComplete(spine.trustedDeviceRegistry) &&
    selectedDeviceReadinessIsRenderable(spine) &&
    routeStateIsRenderable(spine) &&
    lanAiProviderReadinessIsRenderable(spine) &&
    artifactReadinessGatesAreHonest(spine)
  );
}

function lanDiscoveryBoundaryIsRenderable(spine: V09HouseholdMultiDevicePortalDeviceSpine): boolean {
  return (
    spine.lanDiscoveryBoundary.sourceState === 'local-service-discovery-proof' &&
    spine.lanDiscoveryBoundary.discoverableDeviceState === 'ci-mechanical-proof' &&
    spine.lanDiscoveryBoundary.physicalLanDiscoveryState === 'manual-required'
  );
}

function householdDeviceRegistryIsRenderable(spine: V09HouseholdMultiDevicePortalDeviceSpine): boolean {
  const coveredDeviceStates = new Set(spine.householdDeviceRegistry.devices.map((device) => device.deviceState));
  return (
    spine.householdDeviceRegistry.registryProofState === 'ci-mechanical-proof' &&
    ['paired', 'offline', 'stale', 'manual-required'].every((state) =>
      coveredDeviceStates.has(state as V09HouseholdMultiDeviceVisibleDeviceState)
    ) &&
    spine.householdDeviceRegistry.devices.every((device) => device.artifactGateStatus === 'manual-required')
  );
}

function selectedDeviceReadinessIsRenderable(spine: V09HouseholdMultiDevicePortalDeviceSpine): boolean {
  return (
    spine.selectedDeviceReadiness.selectedRouteId !== null &&
    spine.selectedDeviceReadiness.selectedDeviceState === 'paired' &&
    spine.selectedDeviceReadiness.routeProofState === 'ci-mechanical-proof' &&
    spine.selectedDeviceReadiness.physicalArtifactStatus === 'manual-required'
  );
}

function routeStateIsRenderable(spine: V09HouseholdMultiDevicePortalDeviceSpine): boolean {
  return (
    spine.routeState.currentControllerRouteId !== null &&
    spine.routeState.currentObserverRouteId !== null &&
    spine.routeState.controllerCommandAuthorityState === 'active-controller-backend-proof' &&
    spine.routeState.observerCommandAuthorityState === 'observer-read-only' &&
    spine.routeState.manualControllerTakeoverState === 'controller-takeover-manual-required'
  );
}

function lanAiProviderReadinessIsRenderable(spine: V09HouseholdMultiDevicePortalDeviceSpine): boolean {
  return (
    spine.lanAiProviderReadiness.readinessState === 'mobile-provider-degraded' &&
    spine.lanAiProviderReadiness.localProviderState === 'ci-mechanical-proof' &&
    spine.lanAiProviderReadiness.mobileProviderState === 'degraded' &&
    spine.lanAiProviderReadiness.physicalProviderArtifactStatus === 'manual-required' &&
    spine.lanAiProviderReadiness.evidenceLabels.length >= 3
  );
}

function artifactReadinessGatesAreHonest(spine: V09HouseholdMultiDevicePortalDeviceSpine): boolean {
  return (
    spine.artifactReadinessGates.requiredArtifactCount >= 11 &&
    spine.artifactReadinessGates.collectedArtifactCount === 0 &&
    spine.artifactReadinessGates.missingArtifactCount === spine.artifactReadinessGates.requiredArtifactCount &&
    spine.artifactReadinessGates.physicalReadinessState === 'manual-required' &&
    spine.artifactReadinessGates.cloudRelayState === 'not-implemented'
  );
}

function pairingRequestStatesAreComplete(
  requests: ReadonlyArray<V09HouseholdMultiDeviceAddDevicePairingRequest>
): boolean {
  const byState = new Map(requests.map((request) => [request.requestState, request] as const));
  return RequiredPairingRequestStates.every((state) => {
    const request = byState.get(state);
    return request !== undefined && request.manualArtifactStatus === 'manual-required';
  });
}

function trustedDeviceRegistryIsComplete(registry: V09HouseholdMultiDeviceTrustedDeviceRegistry): boolean {
  const coveredStates = new Set(registry.entries.map((entry) => entry.deviceState));
  return (
    registry.registryProofState === 'ci-mechanical-proof' &&
    registry.entries.every((entry) => entry.registryProofState === 'ci-mechanical-proof') &&
    ['paired', 'offline', 'stale'].every((state) =>
      coveredStates.has(state as V09HouseholdMultiDeviceVisibleDeviceState)
    ) &&
    registry.selectedRouteRecoveryLabelCount >= 2 &&
    registry.trustedRegistryLabelCount >= 2
  );
}

type V09HouseholdMultiDeviceProofGateSource = Infer<typeof V09HouseholdMultiDeviceProofGateSourceSchema>;
type V09HouseholdMultiDeviceRouteCustodyGate = Infer<typeof V09HouseholdMultiDeviceRouteCustodyGateSchema>;
type V09HouseholdMultiDeviceSourceProofInput = Infer<typeof V09HouseholdMultiDeviceSourceProofInputSchema>;
type V09HouseholdMultiDeviceRouteCustodyEvidence = Infer<typeof V09HouseholdMultiDeviceRouteCustodyEvidenceSchema>;
type V09HouseholdMultiDeviceSelectedTrustedStorageEvidence = Infer<
  typeof V09HouseholdMultiDeviceSelectedTrustedStorageEvidenceSchema
>;
type V09HouseholdMultiDeviceCloudRelayBoundary = Infer<typeof V09HouseholdMultiDeviceCloudRelayBoundarySchema>;
export type V09HouseholdMultiDeviceVisibleDeviceState = Infer<typeof V09HouseholdMultiDeviceVisibleDeviceStateSchema>;
export type V09HouseholdMultiDeviceLanAiProviderReadiness = Infer<
  typeof V09HouseholdMultiDeviceLanAiProviderReadinessSchema
>;
export type V09HouseholdMultiDeviceLanDiscoverySource = Infer<typeof V09HouseholdMultiDeviceLanDiscoverySourceSchema>;
export type V09HouseholdMultiDevicePairingRequestState = Infer<typeof V09HouseholdMultiDevicePairingRequestStateSchema>;
export type V09HouseholdMultiDeviceVisibleDevice = Infer<typeof V09HouseholdMultiDeviceVisibleDeviceSchema>;
export type V09HouseholdMultiDeviceLanDiscoveryBoundary = Infer<
  typeof V09HouseholdMultiDeviceLanDiscoveryBoundarySchema
>;
export type V09HouseholdMultiDeviceAddDevicePairingRequest = Infer<
  typeof V09HouseholdMultiDeviceAddDevicePairingRequestSchema
>;
export type V09HouseholdMultiDeviceTrustedRegistryEntry = Infer<
  typeof V09HouseholdMultiDeviceTrustedRegistryEntrySchema
>;
export type V09HouseholdMultiDeviceTrustedDeviceRegistry = Infer<
  typeof V09HouseholdMultiDeviceTrustedDeviceRegistrySchema
>;
export type V09HouseholdMultiDeviceHouseholdDeviceRegistry = Infer<
  typeof V09HouseholdMultiDeviceHouseholdDeviceRegistrySchema
>;
export type V09HouseholdMultiDeviceSelectedDeviceReadiness = Infer<
  typeof V09HouseholdMultiDeviceSelectedDeviceReadinessSchema
>;
export type V09HouseholdMultiDeviceVisibleRouteState = Infer<typeof V09HouseholdMultiDeviceVisibleRouteStateSchema>;
export type V09HouseholdMultiDeviceLanAiProviderReadinessEvidence = Infer<
  typeof V09HouseholdMultiDeviceLanAiProviderReadinessEvidenceSchema
>;
export type V09HouseholdMultiDeviceArtifactReadinessGateSummary = Infer<
  typeof V09HouseholdMultiDeviceArtifactReadinessGateSummarySchema
>;
export type V09HouseholdMultiDevicePortalDeviceSpine = Infer<typeof V09HouseholdMultiDevicePortalDeviceSpineSchema>;
export type V09HouseholdMultiDeviceProofGateReadModel = Infer<typeof V09HouseholdMultiDeviceProofGateReadModelSchema>;
