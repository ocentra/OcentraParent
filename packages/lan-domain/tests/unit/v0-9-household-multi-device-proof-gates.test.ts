import { describe, expect, it } from 'vitest';
import {
  V09HouseholdMultiDeviceProofGateReadModelSchema,
  V09HouseholdMultiDeviceProofGateSourceSchema,
  V09HouseholdMultiDeviceRouteCustodyGateSchema,
} from '@ocentra-parent/schema-domain/v0-9-household-multi-device-proof-gates';

const checkedAt = '2026-06-01T12:20:00.000Z';
const routeId = 'route-v0-9-household-multi-device-gates';

const readModel = {
  schemaVersion: 'v0-9-household-multi-device-proof-gates',
  checkedAt,
  readinessDecision: 'manual-gate-required-before-household-multi-device-readiness',
  householdMultiDeviceReadinessState: 'manual-required',
  localMultiServiceProofState: 'ci-mechanical-proof',
  sourceProofs: [
    sourceProof('v0-9-household-discovery-mobile-controller-product-proof'),
    sourceProof('v0-9-household-physical-proof-artifact-gate'),
    sourceProof('v0-9-production-lan-multidevice-hardening'),
  ],
  physicalArtifactRequirements: [
    physicalRequirement('two-physical-household-hosts'),
    physicalRequirement('same-router-or-subnet-evidence'),
    physicalRequirement('child-service-router-reachability'),
    physicalRequirement('os-firewall-or-local-network-permission'),
    physicalRequirement('controller-origin-allowlist-artifact'),
    physicalRequirement('selected-device-route-recovery'),
    physicalRequirement('controller-observer-route-health'),
    physicalRequirement('revoked-route-rejection'),
    physicalRequirement('stale-offline-device-rejection'),
    physicalRequirement('real-mobile-controller-package'),
    physicalRequirement('manual-evidence-custody-record'),
  ],
  manualEvidenceStatus: {
    custodyState: 'not-collected',
    requiredArtifactCount: 11,
    collectedArtifactCount: 0,
    missingArtifactCount: 11,
    reviewerSummary: 'manual household multi-device evidence is not collected',
  },
  routeCustody: [
    routeCustody('paired-route-accepted', 'paired-household-route-evidence', null, 'ci-mechanical-proof'),
    routeCustody(
      'failed-unpaired-rejected',
      'failed-unpaired-household-route-evidence',
      'anonymous',
      'ci-mechanical-proof'
    ),
    routeCustody('wrong-origin-rejected', 'allowed-origin-rejection-custody', 'wrong-origin', 'ci-mechanical-proof'),
    routeCustody('wrong-device-rejected', 'wrong-device-rejection-custody', 'wrong-device', 'ci-mechanical-proof'),
    routeCustody('replay-rejected', 'replay-rejection-custody', 'replayed', 'ci-mechanical-proof'),
    routeCustody('revoked-pairing-rejected', 'revocation-rejection-custody', 'revoked', 'ci-mechanical-proof'),
    routeCustody('stale-source-rejected', 'stale-offline-rejection-custody', 'stale', 'ci-mechanical-proof'),
    routeCustody('offline-device-rejected', 'stale-offline-rejection-custody', 'offline', 'ci-mechanical-proof'),
    routeCustody('unavailable-route-rejected', 'unsupported-route-custody', 'unsupported-route', 'unavailable'),
  ],
  selectedTrustedDeviceStorage: {
    storageState: 'ci-mechanical-proof',
    securityState: 'ci-mechanical-proof',
    selectedRouteRecoveryLabelCount: 2,
    trustedRegistryLabelCount: 2,
    selectedRouteTrustLabelCount: 3,
    selectedDeviceRejectionLabelCount: 8,
    wrongDeviceRejectionLabel: 'wrong-agent-port-rejected-as-wrong-device',
    manualArtifactStatus: 'manual-required',
    evidenceLabel: 'selected trusted-device storage follows through to route security',
  },
  cloudRelayBoundary: {
    implementationState: 'not-implemented',
    remoteControlState: 'not-implemented',
    decision: 'manual-decision-required',
    manualDecisionLabel: 'cloud relay remains a product decision boundary without implementation',
  },
  portalDeviceSpine: {
    lanDiscoveryBoundary: {
      sourceState: 'local-service-discovery-proof',
      discoverableDeviceState: 'ci-mechanical-proof',
      physicalLanDiscoveryState: 'manual-required',
      evidenceLabel: 'browser LAN adapter consumes local service discovery while physical LAN scan stays manual',
    },
    householdDeviceRegistry: {
      registryProofState: 'ci-mechanical-proof',
      devices: [
        visibleDevice('paired household child route', routeId, 'paired', 'ci-mechanical-proof'),
        visibleDevice('offline household child route', routeId, 'offline', 'ci-mechanical-proof'),
        visibleDevice('stale household child route', routeId, 'stale', 'ci-mechanical-proof'),
        visibleDevice('manual household mobile route', null, 'manual-required', 'manual-required'),
      ],
      evidenceLabel: 'household device registry exposes paired offline stale and manual-required states',
    },
    addDevicePairingRequests: [
      pairingRequest('discovered', routeId, null, 'ci-mechanical-proof'),
      pairingRequest('pending', routeId, null, 'ci-mechanical-proof'),
      pairingRequest('paired', routeId, null, 'ci-mechanical-proof'),
      pairingRequest('rejected', routeId, 'anonymous', 'ci-mechanical-proof'),
      pairingRequest('expired', routeId, 'stale', 'ci-mechanical-proof'),
      pairingRequest('revoked', routeId, 'revoked', 'ci-mechanical-proof'),
      pairingRequest('stale', routeId, 'stale', 'ci-mechanical-proof'),
      pairingRequest('offline', routeId, 'offline', 'ci-mechanical-proof'),
    ],
    trustedDeviceRegistry: {
      registryProofState: 'ci-mechanical-proof',
      entries: [
        trustedRegistryEntry(routeId, 'paired', 'paired'),
        trustedRegistryEntry(routeId, 'stale', 'stale'),
        trustedRegistryEntry(routeId, 'offline', 'offline'),
      ],
      selectedRouteRecoveryLabelCount: 2,
      trustedRegistryLabelCount: 2,
      evidenceLabel: 'trusted-device registry read model is available to portal adapter',
    },
    selectedDeviceReadiness: {
      selectedRouteId: routeId,
      selectedDeviceState: 'paired',
      routeProofState: 'ci-mechanical-proof',
      physicalArtifactStatus: 'manual-required',
      manualRequiredLabel: 'selected device physical readiness remains manual-required',
    },
    routeState: {
      currentControllerRouteId: routeId,
      currentObserverRouteId: routeId,
      controllerCommandAuthorityState: 'active-controller-backend-proof',
      observerCommandAuthorityState: 'observer-read-only',
      manualControllerTakeoverState: 'controller-takeover-manual-required',
      evidenceLabel: 'current controller and observer route state is adapter-consumable',
    },
    lanAiProviderReadiness: {
      readinessState: 'mobile-provider-degraded',
      localProviderState: 'ci-mechanical-proof',
      mobileProviderState: 'degraded',
      physicalProviderArtifactStatus: 'manual-required',
      evidenceLabels: [
        'desktop LAN AI provider has local mechanical proof',
        'mobile observer provider remains unavailable',
        'mobile controller job is degraded when provider is unavailable',
      ],
    },
    artifactReadinessGates: {
      requiredArtifactCount: 11,
      collectedArtifactCount: 0,
      missingArtifactCount: 11,
      physicalReadinessState: 'manual-required',
      cloudRelayState: 'not-implemented',
      evidenceLabel: 'artifact readiness gates remain manual or not implemented',
    },
    adapterBoundaryLabel: 'non-visual portal adapter may consume this spine without claiming UI parity',
  },
  claimsProved: [
    'local multi-service proof is mapped into typed household multi-device gates',
    'route custody keeps paired and rejected household route evidence explicit',
    'browser LAN adapter spine exposes discovery pairing registry route provider and artifact gates',
  ],
  claimsNotProved: [
    'remote desktop or remote control',
    'physical household LAN readiness',
    'real mobile controller product UX',
    'cloud relay routing or authentication',
  ],
} as const;

describe('V0.9 household multi-device proof gate contracts', () => {
  it('accepts a complete gate while physical household readiness and cloud relay stay unclaimed', acceptsCompleteGate);

  it(
    'rejects physical readiness, cloud relay, route custody, selected-device, and portal-spine overclaims',
    rejectsOverclaims
  );

  it('keeps source proof and route custody vocabularies explicit', () => {
    expect(V09HouseholdMultiDeviceProofGateSourceSchema.parse('v0-9-production-lan-multidevice-hardening')).toBe(
      'v0-9-production-lan-multidevice-hardening'
    );
    expect(V09HouseholdMultiDeviceRouteCustodyGateSchema.parse('allowed-origin-rejection-custody')).toBe(
      'allowed-origin-rejection-custody'
    );
    expect(V09HouseholdMultiDeviceRouteCustodyGateSchema.safeParse('cloud-relay-ready').success).toBe(false);
  });
});

function acceptsCompleteGate() {
  const parsed = V09HouseholdMultiDeviceProofGateReadModelSchema.parse(readModel);

  expect(parsed.householdMultiDeviceReadinessState).toBe('manual-required');
  expect(parsed.localMultiServiceProofState).toBe('ci-mechanical-proof');
  expect(parsed.routeCustody.map((entry) => entry.custodyGate)).toEqual([
    'paired-household-route-evidence',
    'failed-unpaired-household-route-evidence',
    'allowed-origin-rejection-custody',
    'wrong-device-rejection-custody',
    'replay-rejection-custody',
    'revocation-rejection-custody',
    'stale-offline-rejection-custody',
    'stale-offline-rejection-custody',
    'unsupported-route-custody',
  ]);
  expect(parsed.selectedTrustedDeviceStorage.selectedDeviceRejectionLabelCount).toBe(8);
  expect(parsed.cloudRelayBoundary.implementationState).toBe('not-implemented');
  expect(parsed.portalDeviceSpine.householdDeviceRegistry.devices.map((device) => device.deviceState)).toEqual([
    'paired',
    'offline',
    'stale',
    'manual-required',
  ]);
  expect(parsed.portalDeviceSpine.addDevicePairingRequests.map((request) => request.requestState)).toEqual([
    'discovered',
    'pending',
    'paired',
    'rejected',
    'expired',
    'revoked',
    'stale',
    'offline',
  ]);
  expect(parsed.portalDeviceSpine.trustedDeviceRegistry.entries.map((entry) => entry.deviceState)).toEqual([
    'paired',
    'stale',
    'offline',
  ]);
  expect(parsed.portalDeviceSpine.selectedDeviceReadiness.physicalArtifactStatus).toBe('manual-required');
  expect(parsed.portalDeviceSpine.routeState.observerCommandAuthorityState).toBe('observer-read-only');
  expect(parsed.portalDeviceSpine.lanAiProviderReadiness.mobileProviderState).toBe('degraded');
}

function rejectsOverclaims() {
  for (const candidate of [
    { ...readModel, householdMultiDeviceReadinessState: 'ci-mechanical-proof' },
    {
      ...readModel,
      cloudRelayBoundary: { ...readModel.cloudRelayBoundary, implementationState: 'ci-mechanical-proof' },
    },
    { ...readModel, routeCustody: readModel.routeCustody.filter((entry) => entry.check !== 'wrong-device-rejected') },
    {
      ...readModel,
      selectedTrustedDeviceStorage: { ...readModel.selectedTrustedDeviceStorage, storageState: 'manual-required' },
    },
    withoutOfflineVisibleDevice(),
    withPhysicalLanOverclaim(),
    withoutExpiredPairingRequest(),
    withWeakTrustedRegistry(),
    withObserverCommandAuthorityOverclaim(),
    withPhysicalProviderOverclaim(),
  ]) {
    expectRejected(candidate);
  }
}

function withoutOfflineVisibleDevice() {
  return {
    ...readModel,
    portalDeviceSpine: {
      ...readModel.portalDeviceSpine,
      householdDeviceRegistry: {
        ...readModel.portalDeviceSpine.householdDeviceRegistry,
        devices: readModel.portalDeviceSpine.householdDeviceRegistry.devices.filter(
          (device) => device.deviceState !== 'offline'
        ),
      },
    },
  };
}

function withPhysicalLanOverclaim() {
  return {
    ...readModel,
    portalDeviceSpine: {
      ...readModel.portalDeviceSpine,
      lanDiscoveryBoundary: {
        ...readModel.portalDeviceSpine.lanDiscoveryBoundary,
        physicalLanDiscoveryState: 'ci-mechanical-proof',
      },
    },
  };
}

function withoutExpiredPairingRequest() {
  return {
    ...readModel,
    portalDeviceSpine: {
      ...readModel.portalDeviceSpine,
      addDevicePairingRequests: readModel.portalDeviceSpine.addDevicePairingRequests.filter(
        (request) => request.requestState !== 'expired'
      ),
    },
  };
}

function withWeakTrustedRegistry() {
  return {
    ...readModel,
    portalDeviceSpine: {
      ...readModel.portalDeviceSpine,
      trustedDeviceRegistry: {
        ...readModel.portalDeviceSpine.trustedDeviceRegistry,
        selectedRouteRecoveryLabelCount: 1,
      },
    },
  };
}

function withObserverCommandAuthorityOverclaim() {
  return {
    ...readModel,
    portalDeviceSpine: {
      ...readModel.portalDeviceSpine,
      routeState: {
        ...readModel.portalDeviceSpine.routeState,
        observerCommandAuthorityState: 'active-controller-backend-proof',
      },
    },
  };
}

function withPhysicalProviderOverclaim() {
  return {
    ...readModel,
    portalDeviceSpine: {
      ...readModel.portalDeviceSpine,
      lanAiProviderReadiness: {
        ...readModel.portalDeviceSpine.lanAiProviderReadiness,
        physicalProviderArtifactStatus: 'complete',
      },
    },
  };
}

function expectRejected(candidate: unknown) {
  expect(V09HouseholdMultiDeviceProofGateReadModelSchema.safeParse(candidate).success).toBe(false);
}

function sourceProof(source: unknown) {
  return {
    source,
    path: `test-results/${String(source)}/proof.json`,
    command: `node scripts/test/${String(source)}.mjs`,
  };
}

function physicalRequirement(requirement: unknown) {
  return {
    requirement,
    status: 'manual-required',
    requiredArtifactSummary: `${String(requirement)} manual artifact remains required`,
    evidencePath: null,
    evidenceCapturedAt: null,
  };
}

function routeCustody(custodyCheck: unknown, custodyGate: unknown, rejectionReason: unknown, proofState: unknown) {
  return {
    check: custodyCheck,
    custodyGate,
    routeId,
    rejectionReason,
    proofState,
    manualArtifactStatus: 'manual-required',
    evidenceLabel: `${String(custodyCheck)} ${String(custodyGate)}`,
  };
}

function visibleDevice(deviceLabel: unknown, deviceRouteId: unknown, deviceState: unknown, routeProofState: unknown) {
  return {
    deviceLabel,
    routeId: deviceRouteId,
    deviceState,
    routeProofState,
    artifactGateStatus: 'manual-required',
    evidenceLabel: `${String(deviceLabel)} ${String(deviceState)}`,
  };
}

function pairingRequest(requestState: unknown, requestRouteId: unknown, rejectionReason: unknown, proofState: unknown) {
  return {
    requestState,
    routeId: requestRouteId,
    rejectionReason,
    proofState,
    manualArtifactStatus: 'manual-required',
    evidenceLabel: `add-device pairing request ${String(requestState)}`,
  };
}

function trustedRegistryEntry(entryRouteId: unknown, pairingState: unknown, deviceState: unknown) {
  return {
    routeId: entryRouteId,
    pairingState,
    deviceState,
    registryProofState: 'ci-mechanical-proof',
    evidenceLabel: `trusted registry ${String(pairingState)} ${String(deviceState)}`,
  };
}
