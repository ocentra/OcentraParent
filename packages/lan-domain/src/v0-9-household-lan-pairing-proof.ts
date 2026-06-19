import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { LanBrowserAddDeviceReadModelSchema, type LanBrowserAddDeviceReadModel } from '@ocentra-parent/lan-domain/lan-pairing-device';
import {
  type LanPairingProductionDiscoveryStateSchema,
  LanPairingRejectionReasonSchema,
  LanPairingRouteIdSchema,
} from '@ocentra-parent/lan-domain/lan-pairing-values';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { V09RuntimeProofStateSchema } from './v0-9-mobile-controller-discovery-runtime';

export const V09HouseholdLanPairingProofIdSchema = withParser(Schema.Literal('v0-9-household-lan-pairing-proof'));

export const V09HouseholdLanPairingProofSourceSchema = withParser(
  Schema.Literal(
    'browser-first-lan-discovery-add-device-state',
    'lan-browser-discovery-pairing-runtime',
    'v0-9-household-lan-proof-readiness'
  )
);

export const V09HouseholdLanPairingRuntimeEventSchema = withParser(
  Schema.Literal(
    'browser-discovery-scan-reported',
    'browser-add-device-request-reported',
    'wrong-origin-add-device-rejected',
    'selected-readiness-reported'
  )
);

export const V09HouseholdLanPairingRouteSecurityCheckSchema = withParser(
  Schema.Literal(
    'allowed-origin',
    'target-device-match',
    'non-replayed-intent',
    'wrong-origin',
    'wrong-device',
    'replayed',
    'stale',
    'revoked',
    'offline'
  )
);

export const V09HouseholdLanPairingManualGateSchema = withParser(
  Schema.Literal(
    'two-physical-household-hosts',
    'household-router-reachability',
    'os-firewall-or-local-network-permission',
    'physical-origin-allowlist',
    'physical-pairing-revocation-rejection',
    'physical-stale-offline-selected-device',
    'real-mobile-controller-package',
    'cloud-relay-separate-proof'
  )
);

export const V09HouseholdLanPairingPolicyTargetSurfaceSchema = withParser(
  Schema.Literal('devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai')
);

const V09HouseholdLanPairingReadinessDecisionSchema = withParser(
  Schema.Literal('manual-physical-household-gate-required')
);

const V09HouseholdLanPairingProofPathSchema = brandedNonEmptyStringSchema('V09HouseholdLanPairingProofPath');
const V09HouseholdLanPairingProofCommandSchema = brandedNonEmptyStringSchema('V09HouseholdLanPairingProofCommand');
const V09HouseholdLanPairingProofLabelSchema = brandedNonEmptyStringSchema('V09HouseholdLanPairingProofLabel');
const V09HouseholdLanPairingClaimBoundarySchema = brandedNonEmptyStringSchema('V09HouseholdLanPairingClaimBoundary');

const V09HouseholdLanPairingSourceProofInputSchema = withParser(
  Schema.Struct({
    source: V09HouseholdLanPairingProofSourceSchema,
    path: V09HouseholdLanPairingProofPathSchema,
    command: V09HouseholdLanPairingProofCommandSchema,
  })
);

const V09HouseholdLanPairingRuntimeEventEvidenceSchema = withParser(
  Schema.Struct({
    event: V09HouseholdLanPairingRuntimeEventSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    proofState: V09RuntimeProofStateSchema,
    evidenceLabel: V09HouseholdLanPairingProofLabelSchema,
  })
);

const V09HouseholdLanPairingRouteSecurityEvidenceSchema = withParser(
  Schema.Struct({
    check: V09HouseholdLanPairingRouteSecurityCheckSchema,
    routeId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
    rejectionReason: Schema.Union(LanPairingRejectionReasonSchema, Schema.Null),
    proofState: V09RuntimeProofStateSchema,
    evidenceLabel: V09HouseholdLanPairingProofLabelSchema,
  })
);

const V09HouseholdLanPairingManualGateEvidenceSchema = withParser(
  Schema.Struct({
    gate: V09HouseholdLanPairingManualGateSchema,
    state: V09RuntimeProofStateSchema,
    requiredArtifactSummary: V09HouseholdLanPairingClaimBoundarySchema,
  })
);

const V09HouseholdLanPairingBoundarySummarySchema = withParser(
  Schema.Struct({
    localServiceDiscoveryState: V09RuntimeProofStateSchema,
    browserPairingRuntimeState: V09RuntimeProofStateSchema,
    physicalHouseholdLanState: V09RuntimeProofStateSchema,
    parentMobileControllerState: V09RuntimeProofStateSchema,
    cloudRelayState: V09RuntimeProofStateSchema,
    remoteControlState: V09RuntimeProofStateSchema,
    evidenceLabel: V09HouseholdLanPairingProofLabelSchema,
  })
);

const V09HouseholdLanPairingProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: V09HouseholdLanPairingProofIdSchema,
  checkedAt: ParentTimestampSchema,
  readinessDecision: V09HouseholdLanPairingReadinessDecisionSchema,
  sourceProofs: Schema.Array(V09HouseholdLanPairingSourceProofInputSchema),
  addDeviceReadModel: LanBrowserAddDeviceReadModelSchema,
  runtimeEvents: Schema.Array(V09HouseholdLanPairingRuntimeEventEvidenceSchema),
  routeSecurityChecks: Schema.Array(V09HouseholdLanPairingRouteSecurityEvidenceSchema),
  manualProofGates: Schema.Array(V09HouseholdLanPairingManualGateEvidenceSchema),
  boundarySummary: V09HouseholdLanPairingBoundarySummarySchema,
  claimsProved: Schema.Array(V09HouseholdLanPairingProofLabelSchema),
  claimsNotProved: Schema.Array(V09HouseholdLanPairingClaimBoundarySchema),
});

type V09HouseholdLanPairingProofReadModelCandidate = Infer<typeof V09HouseholdLanPairingProofReadModelBaseSchema>;

export const V09HouseholdLanPairingProofReadModelSchema = withParser(
  V09HouseholdLanPairingProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        householdLanPairingProofIsHonest(readModel) ||
        'Expected V0.9 household LAN pairing proof to preserve local-service browser pairing, manual physical household LAN gates, and not-implemented cloud or remote-control boundaries'
    )
  )
);

const RequiredSourceProofs = [
  'browser-first-lan-discovery-add-device-state',
  'lan-browser-discovery-pairing-runtime',
  'v0-9-household-lan-proof-readiness',
] as const satisfies ReadonlyArray<V09HouseholdLanPairingProofSource>;

const RequiredPairingStates = [
  'discovered',
  'pending',
  'paired',
  'rejected',
  'expired',
  'revoked',
  'stale',
  'offline',
] as const satisfies ReadonlyArray<Infer<typeof LanPairingProductionDiscoveryStateSchema>>;

const RequiredRuntimeEvents = [
  'browser-discovery-scan-reported',
  'browser-add-device-request-reported',
  'wrong-origin-add-device-rejected',
  'selected-readiness-reported',
] as const satisfies ReadonlyArray<V09HouseholdLanPairingRuntimeEvent>;

const RequiredRouteSecurityChecks = [
  'allowed-origin',
  'target-device-match',
  'non-replayed-intent',
  'wrong-origin',
  'wrong-device',
  'replayed',
  'stale',
  'revoked',
  'offline',
] as const satisfies ReadonlyArray<V09HouseholdLanPairingRouteSecurityCheck>;

const RequiredManualGates = [
  'two-physical-household-hosts',
  'household-router-reachability',
  'os-firewall-or-local-network-permission',
  'physical-origin-allowlist',
  'physical-pairing-revocation-rejection',
  'physical-stale-offline-selected-device',
  'real-mobile-controller-package',
  'cloud-relay-separate-proof',
] as const satisfies ReadonlyArray<V09HouseholdLanPairingManualGate>;

const RequiredPolicyTargetSurfaces = [
  'devices',
  'policy',
  'browser',
  'app',
  'screen',
  'network',
  'activity',
  'tracking',
  'ai',
] as const satisfies ReadonlyArray<V09HouseholdLanPairingPolicyTargetSurface>;

function householdLanPairingProofIsHonest(readModel: V09HouseholdLanPairingProofReadModelCandidate): boolean {
  return (
    sourceProofsAreComplete(readModel.sourceProofs) &&
    addDeviceReadModelIsBrowserFirst(readModel.addDeviceReadModel) &&
    runtimeEventsAreComplete(readModel.runtimeEvents) &&
    routeSecurityChecksAreComplete(readModel.routeSecurityChecks) &&
    manualGatesRemainUnclaimed(readModel.manualProofGates) &&
    boundarySummaryIsHonest(readModel.boundarySummary) &&
    readModel.claimsNotProved.some((claim) => claim.includes('physical household LAN')) &&
    readModel.claimsNotProved.some((claim) => claim.includes('cloud relay')) &&
    readModel.claimsNotProved.some((claim) => claim.includes('remote desktop'))
  );
}

function sourceProofsAreComplete(proofs: ReadonlyArray<V09HouseholdLanPairingSourceProofInput>): boolean {
  const sources = new Set(proofs.map((proof) => proof.source));
  return RequiredSourceProofs.every((source) => sources.has(source));
}

function addDeviceReadModelIsBrowserFirst(readModel: LanBrowserAddDeviceReadModel): boolean {
  return (
    readModel.discoverySource === 'local-service' &&
    readModel.localServiceDiscoveryState === 'pending' &&
    readModel.physicalHouseholdLanState === 'manual-required' &&
    readModel.cloudRelayState === 'unavailable' &&
    readModel.trustedDeviceRegistry.length > 0 &&
    readModel.trustedDeviceIds.length > 0 &&
    readModel.honestNonClaims.some((claim) => claim.includes('physical-household-lan')) &&
    readModel.honestNonClaims.some((claim) => claim.includes('cloud-relay')) &&
    canonicalHouseholdDeviceSpineIsRenderable(readModel) &&
    pairingStatesAreComplete(readModel.pairingRequests.map((request) => request.pairingState))
  );
}

function canonicalHouseholdDeviceSpineIsRenderable(readModel: LanBrowserAddDeviceReadModel): boolean {
  const canonicalDeviceIds = readModel.canonicalHouseholdDevices.map((device) => device.canonicalDeviceId);
  return (
    readModel.canonicalHouseholdDevices.length > 0 &&
    new Set(canonicalDeviceIds).size === canonicalDeviceIds.length &&
    readModel.canonicalHouseholdDevices.some((device) => canonicalSpineDeviceIsChildAgentTarget(device)) &&
    readModel.canonicalHouseholdDevices
      .filter((device) => device.classification !== 'child-agent')
      .every((device) => !device.enrollable && device.childAgentInventory === null)
  );
}

function canonicalSpineDeviceIsChildAgentTarget(
  device: LanBrowserAddDeviceReadModel['canonicalHouseholdDevices'][number]
): boolean {
  const surfaces = new Set(device.policyTargetSurfaces);
  return (
    device.classification === 'child-agent' &&
    device.enrollable &&
    device.roleBadges.includes('child-agent') &&
    RequiredPolicyTargetSurfaces.every((surface) => surfaces.has(surface))
  );
}

function pairingStatesAreComplete(
  states: ReadonlyArray<Infer<typeof LanPairingProductionDiscoveryStateSchema>>
): boolean {
  const covered = new Set(states);
  return RequiredPairingStates.every((state) => covered.has(state));
}

function runtimeEventsAreComplete(events: ReadonlyArray<V09HouseholdLanPairingRuntimeEventEvidence>): boolean {
  const covered = new Set(events.map((event) => event.event));
  return (
    RequiredRuntimeEvents.every((event) => covered.has(event)) &&
    events.every((event) => event.proofState === 'ci-mechanical-proof')
  );
}

function routeSecurityChecksAreComplete(checks: ReadonlyArray<V09HouseholdLanPairingRouteSecurityEvidence>): boolean {
  const covered = new Set(checks.map((check) => check.check));
  return (
    RequiredRouteSecurityChecks.every((check) => covered.has(check)) &&
    checks.some((check) => check.check === 'wrong-origin' && check.rejectionReason === 'wrong-origin') &&
    checks.some((check) => check.check === 'wrong-device' && check.rejectionReason === 'wrong-device') &&
    checks.some((check) => check.check === 'revoked' && check.rejectionReason === 'revoked') &&
    checks.every((check) => check.proofState === 'ci-mechanical-proof')
  );
}

function manualGatesRemainUnclaimed(gates: ReadonlyArray<V09HouseholdLanPairingManualGateEvidence>): boolean {
  const byGate = new Map(gates.map((gate) => [gate.gate, gate] as const));
  return RequiredManualGates.every((gate) => manualGateRemainsUnclaimed(byGate.get(gate), gate));
}

function manualGateRemainsUnclaimed(
  gate: V09HouseholdLanPairingManualGateEvidence | undefined,
  expectedGate: V09HouseholdLanPairingManualGate
): boolean {
  if (gate === undefined) {
    return false;
  }
  if (expectedGate === 'cloud-relay-separate-proof') {
    return gate.state === 'not-implemented';
  }
  return gate.state === 'manual-required';
}

function boundarySummaryIsHonest(summary: V09HouseholdLanPairingBoundarySummary): boolean {
  return (
    summary.localServiceDiscoveryState === 'ci-mechanical-proof' &&
    summary.browserPairingRuntimeState === 'ci-mechanical-proof' &&
    summary.physicalHouseholdLanState === 'manual-required' &&
    summary.parentMobileControllerState === 'manual-required' &&
    summary.cloudRelayState === 'not-implemented' &&
    summary.remoteControlState === 'not-implemented'
  );
}

export type V09HouseholdLanPairingProofId = Infer<typeof V09HouseholdLanPairingProofIdSchema>;
export type V09HouseholdLanPairingProofSource = Infer<typeof V09HouseholdLanPairingProofSourceSchema>;
export type V09HouseholdLanPairingRuntimeEvent = Infer<typeof V09HouseholdLanPairingRuntimeEventSchema>;
export type V09HouseholdLanPairingRouteSecurityCheck = Infer<typeof V09HouseholdLanPairingRouteSecurityCheckSchema>;
export type V09HouseholdLanPairingManualGate = Infer<typeof V09HouseholdLanPairingManualGateSchema>;
export type V09HouseholdLanPairingPolicyTargetSurface = Infer<typeof V09HouseholdLanPairingPolicyTargetSurfaceSchema>;
export type V09HouseholdLanPairingSourceProofInput = Infer<typeof V09HouseholdLanPairingSourceProofInputSchema>;
export type V09HouseholdLanPairingRuntimeEventEvidence = Infer<typeof V09HouseholdLanPairingRuntimeEventEvidenceSchema>;
export type V09HouseholdLanPairingRouteSecurityEvidence = Infer<
  typeof V09HouseholdLanPairingRouteSecurityEvidenceSchema
>;
export type V09HouseholdLanPairingManualGateEvidence = Infer<typeof V09HouseholdLanPairingManualGateEvidenceSchema>;
export type V09HouseholdLanPairingBoundarySummary = Infer<typeof V09HouseholdLanPairingBoundarySummarySchema>;
export type V09HouseholdLanPairingProofReadModel = Infer<typeof V09HouseholdLanPairingProofReadModelSchema>;

