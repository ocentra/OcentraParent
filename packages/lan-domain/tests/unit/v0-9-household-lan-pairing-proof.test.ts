import { describe, expect, it } from 'vitest';
import {
  V09HouseholdLanPairingManualGateSchema,
  V09HouseholdLanPairingPolicyTargetSurfaceSchema,
  V09HouseholdLanPairingProofReadModelSchema,
  V09HouseholdLanPairingProofSourceSchema,
  V09HouseholdLanPairingRuntimeEventSchema,
} from '@ocentra-parent/schema-domain/v0-9-household-lan-pairing-proof';

const checkedAt = '2026-06-01T19:45:00.000Z';
const routeId = 'route-v0-9-household-lan-pairing-proof';
const parentDeviceId = 'parent-device-1';
const childDeviceId = 'child-device-1';

const readModel = {
  schemaVersion: 'v0-9-household-lan-pairing-proof',
  checkedAt,
  readinessDecision: 'manual-physical-household-gate-required',
  sourceProofs: [
    sourceProof('browser-first-lan-discovery-add-device-state'),
    sourceProof('lan-browser-discovery-pairing-runtime'),
    sourceProof('v0-9-household-lan-proof-readiness'),
  ],
  addDeviceReadModel: addDeviceReadModel(),
  runtimeEvents: [
    runtimeEvent('browser-discovery-scan-reported', routeId, 'browser discovery scan reported local service state'),
    runtimeEvent('browser-add-device-request-reported', routeId, 'browser add-device request issued pairing state'),
    runtimeEvent('wrong-origin-add-device-rejected', routeId, 'wrong-origin add-device request rejected'),
    runtimeEvent('selected-readiness-reported', routeId, 'selected-device readiness reported for portal adapter'),
  ],
  routeSecurityChecks: [
    routeSecurity('allowed-origin', routeId, null, 'allowed origin checked'),
    routeSecurity('target-device-match', routeId, null, 'target device matched'),
    routeSecurity('non-replayed-intent', routeId, null, 'non-replayed intent checked'),
    routeSecurity('wrong-origin', routeId, 'wrong-origin', 'wrong origin rejected'),
    routeSecurity('wrong-device', routeId, 'wrong-device', 'wrong device rejected'),
    routeSecurity('replayed', routeId, 'replayed', 'replay rejected'),
    routeSecurity('stale', routeId, 'stale', 'stale selected device rejected'),
    routeSecurity('revoked', routeId, 'revoked', 'revoked route rejected'),
    routeSecurity('offline', routeId, 'offline', 'offline selected device rejected'),
  ],
  manualProofGates: [
    manualGate('two-physical-household-hosts', 'manual-required'),
    manualGate('household-router-reachability', 'manual-required'),
    manualGate('os-firewall-or-local-network-permission', 'manual-required'),
    manualGate('physical-origin-allowlist', 'manual-required'),
    manualGate('physical-pairing-revocation-rejection', 'manual-required'),
    manualGate('physical-stale-offline-selected-device', 'manual-required'),
    manualGate('real-mobile-controller-package', 'manual-required'),
    manualGate('cloud-relay-separate-proof', 'not-implemented'),
  ],
  boundarySummary: {
    localServiceDiscoveryState: 'ci-mechanical-proof',
    browserPairingRuntimeState: 'ci-mechanical-proof',
    physicalHouseholdLanState: 'manual-required',
    parentMobileControllerState: 'manual-required',
    cloudRelayState: 'not-implemented',
    remoteControlState: 'not-implemented',
    evidenceLabel: 'browser-first LAN pairing is local-service proof with manual physical gates',
  },
  claimsProved: [
    'browser-first local-service discovery and add-device pairing state is typed',
    'LAN browser runtime reports scan add-device rejection and selected readiness events',
    'household LAN proof keeps physical router firewall mobile and cloud gates separate',
  ],
  claimsNotProved: [
    'physical household LAN readiness',
    'cloud relay routing or authentication',
    'remote desktop or remote control',
  ],
} as const;

describe('V0.9 household LAN pairing proof contracts', () => {
  it('accepts browser-first household LAN pairing proof without physical, cloud, or remote overclaims', acceptsProof);

  it('rejects source, add-device, route, manual-gate, cloud, and remote overclaims', rejectsOverclaims);

  it('keeps proof source, runtime event, and manual gate vocabularies explicit', keepsVocabulariesExplicit);
});

function acceptsProof() {
  const parsed = V09HouseholdLanPairingProofReadModelSchema.parse(readModel);

  expect(parsed.sourceProofs.map((proof) => proof.source)).toEqual([
    'browser-first-lan-discovery-add-device-state',
    'lan-browser-discovery-pairing-runtime',
    'v0-9-household-lan-proof-readiness',
  ]);
  expect(parsed.addDeviceReadModel.discoverySource).toBe('local-service');
  expect(parsed.addDeviceReadModel.physicalHouseholdLanState).toBe('manual-required');
  expect(parsed.addDeviceReadModel.pairingRequests.map((request) => request.pairingState)).toEqual([
    'discovered',
    'pending',
    'paired',
    'rejected',
    'expired',
    'revoked',
    'stale',
    'offline',
  ]);
  expect(parsed.boundarySummary.cloudRelayState).toBe('not-implemented');
  expect(parsed.boundarySummary.remoteControlState).toBe('not-implemented');
  expect(parsed.addDeviceReadModel.householdDeviceDecisions[0]?.actionKind).toBe('rename');
  expect(parsed.addDeviceReadModel.canonicalHouseholdDevices[0]?.roleBadges).toEqual([
    'child-agent',
    'portal',
    'parent-controller',
  ]);
  expect(parsed.addDeviceReadModel.canonicalHouseholdDevices[0]?.policyTargetSurfaces).toEqual([
    'devices',
    'policy',
    'browser',
    'app',
    'screen',
    'network',
    'activity',
    'tracking',
    'ai',
  ]);
}

function rejectsOverclaims() {
  for (const candidate of rejectedReadModelCandidates()) {
    expect(V09HouseholdLanPairingProofReadModelSchema.safeParse(candidate).success).toBe(false);
  }
}

function keepsVocabulariesExplicit() {
  expect(V09HouseholdLanPairingProofSourceSchema.parse('lan-browser-discovery-pairing-runtime')).toBe(
    'lan-browser-discovery-pairing-runtime'
  );
  expect(V09HouseholdLanPairingRuntimeEventSchema.parse('wrong-origin-add-device-rejected')).toBe(
    'wrong-origin-add-device-rejected'
  );
  expect(V09HouseholdLanPairingPolicyTargetSurfaceSchema.parse('ai')).toBe('ai');
  expect(V09HouseholdLanPairingManualGateSchema.parse('cloud-relay-separate-proof')).toBe('cloud-relay-separate-proof');
  expect(V09HouseholdLanPairingManualGateSchema.safeParse('product-ready-household-lan').success).toBe(false);
}

function sourceProof(source: unknown) {
  return {
    source,
    path: `test-results/${String(source)}/proof.json`,
    command: `node scripts/test/${String(source)}.mjs`,
  };
}

function addDeviceReadModel() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: checkedAt,
    discoverySource: 'local-service',
    addDeviceState: 'pending',
    localServiceDiscoveryState: 'pending',
    physicalHouseholdLanState: 'manual-required',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: 'v0.9',
      sourceLabels: ['local-service'],
      scannedDeviceCount: 1,
      agentDeviceCount: 1,
      passiveDeviceCount: 0,
      infrastructureDeviceCount: 0,
      unsupportedDeviceCount: 0,
    },
    discoveredDevices: [discoveredDevice('stale')],
    canonicalHouseholdDevices: [canonicalHouseholdDevice()],
    pairingRequests: [
      pairingRequest('discovered', null),
      pairingRequest('pending', null),
      pairingRequest('paired', null),
      pairingRequest('rejected', 'wrong-origin'),
      pairingRequest('expired', 'expired'),
      pairingRequest('revoked', 'revoked'),
      pairingRequest('stale', 'stale'),
      pairingRequest('offline', 'offline'),
    ],
    trustedDeviceRegistry: [trustedRegistryEntry()],
    householdDeviceDecisions: [householdDecision()],
    trustedDeviceIds: [childDeviceId],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: 'v0.9',
      selectedChildDeviceId: childDeviceId,
      routeId,
      pairingId: 'pairing-child-device-1',
      trustState: 'paired',
      reachability: 'stale',
      readyForControl: false,
      staleAt: checkedAt,
      offlineAt: null,
    },
    controllerAuthority: 'active-controller',
    observerAuthority: 'observer',
    routeRequirementLabels: ['allowed-origin', 'target-device-match', 'non-replayed-intent'],
    auditCheckLabels: ['wrong-origin', 'wrong-device', 'replayed', 'stale', 'revoked', 'offline'],
    honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
  };
}

function canonicalHouseholdDevice() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: childDeviceId,
    displayName: 'Mia Windows PC',
    classification: 'child-agent',
    roleBadges: ['child-agent', 'portal', 'parent-controller'],
    enrollable: true,
    discoveryState: 'paired',
    trustState: 'paired',
    routeId,
    routeState: 'local-network',
    networkMode: 'local-network',
    sourceLabels: ['trusted-registry'],
    networkIdentity: {
      hostname: null,
      ipAddresses: [],
      macAddress: null,
      macVendor: null,
      networkInterfaces: [],
      reachability: 'stale',
      confidence: 'manual-required',
      staleAt: checkedAt,
      offlineAt: null,
      evidenceRecords: [trustedRegistryEvidenceRecord()],
    },
    childAgentInventory: {
      deviceName: 'Mia Windows PC',
      platform: 'windows',
      os: 'windows',
      cpuModel: null,
      cpuCores: null,
      memoryTotal: null,
      gpuModel: null,
      gpuDriver: null,
      gpuMemory: null,
      nvidiaSmi: null,
      networkInterfaces: [],
      capabilities: ['direct-websocket', 'device-inventory', 'pairing-route'],
      roleState: 'implemented',
      routeState: 'local-network',
      pairingTrustState: 'paired',
    },
    policyTargetSurfaces: ['devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai'],
  };
}

function trustedRegistryEvidenceRecord() {
  return {
    schemaVersion: 'v0.9',
    evidenceId: 'lan-evidence-trusted-registry-child-device-1',
    source: 'trusted-registry',
    evidenceKind: 'trusted-registry',
    deviceId: childDeviceId,
    value: childDeviceId,
    normalizedValue: childDeviceId,
    firstSeenAt: checkedAt,
    lastSeenAt: checkedAt,
    expiresAt: null,
    confidence: 'manual-required',
    mergeKey: `trusted:${childDeviceId}`,
    note: null,
  } as const;
}

function discoveredDevice(reachability: unknown) {
  return {
    schemaVersion: 'v0.9',
    discoveredAt: checkedAt,
    childProfile: { childProfileId: 'child-profile-1', displayName: 'Mia' },
    childDevice: childDeviceRef(),
    agentPeerId: 'child-agent-1',
    routeId,
    networkMode: 'local-network',
    reachability,
    addressRef: 'local-service:child-device-1',
    discoveryStatus: 'websocket-direct',
    discoveryState: reachability,
  };
}

function pairingRequest(pairingState: unknown, rejectionReason: unknown) {
  return {
    schemaVersion: 'v0.9',
    challengeId: `challenge-${String(pairingState)}`,
    childDeviceId,
    parentDeviceId,
    routeId,
    origin: 'http://127.0.0.1:4678',
    pairingState,
    rejectionReason,
    issuedAt: checkedAt,
    expiresAt: '2026-06-01T19:50:00.000Z',
  };
}

function householdDecision() {
  return {
    schemaVersion: 'v0.9',
    actionId: 'household-action-1',
    actionKind: 'rename',
    canonicalDeviceId: childDeviceId,
    childProfileId: null,
    displayName: 'Mia Windows PC',
    parentActorId: 'parent-actor-1',
    decidedAt: checkedAt,
    revokedAt: null,
  };
}

function trustedRegistryEntry() {
  return {
    schemaVersion: 'v0.9',
    pairingId: 'pairing-child-device-1',
    childDevice: childDeviceRef(),
    parentDevice: parentDeviceRef(),
    routeId,
    origin: 'http://127.0.0.1:4678',
    proofDigest: 'sha256:household-lan-pairing-proof',
    trustState: 'paired',
    trustedAt: checkedAt,
    expiresAt: '2026-06-01T20:45:00.000Z',
    revokedAt: null,
  };
}

function childDeviceRef() {
  return {
    deviceId: childDeviceId,
    childProfileId: 'child-profile-1',
    label: 'Mia Windows PC',
    platform: 'windows',
  };
}

function parentDeviceRef() {
  return {
    deviceId: parentDeviceId,
    childProfileId: null,
    label: 'Parent Windows PC',
    platform: 'windows',
  };
}

function runtimeEvent(event: unknown, eventRouteId: unknown, evidenceLabel: unknown) {
  return {
    event,
    routeId: eventRouteId,
    proofState: 'ci-mechanical-proof',
    evidenceLabel,
  };
}

function routeSecurity(check: unknown, checkRouteId: unknown, rejectionReason: unknown, evidenceLabel: unknown) {
  return {
    check,
    routeId: checkRouteId,
    rejectionReason,
    proofState: 'ci-mechanical-proof',
    evidenceLabel,
  };
}

function manualGate(gate: unknown, state: unknown) {
  return {
    gate,
    state,
    requiredArtifactSummary: `${String(gate)} remains required before a household LAN product claim`,
  };
}

function rejectedReadModelCandidates() {
  return [
    { ...readModel, sourceProofs: readModel.sourceProofs.slice(1) },
    withPhysicalLanOverclaim(),
    withoutExpiredPairingRequest(),
    { ...readModel, runtimeEvents: readModel.runtimeEvents.slice(1) },
    withoutWrongDeviceRouteSecurity(),
    withoutAddDeviceCanonicalActivityTargetSurface(),
    withPhysicalHostGateOverclaim(),
    {
      ...readModel,
      boundarySummary: { ...readModel.boundarySummary, cloudRelayState: 'ci-mechanical-proof' },
    },
    {
      ...readModel,
      boundarySummary: { ...readModel.boundarySummary, remoteControlState: 'ci-mechanical-proof' },
    },
  ];
}

function withPhysicalLanOverclaim() {
  return {
    ...readModel,
    addDeviceReadModel: {
      ...readModel.addDeviceReadModel,
      physicalHouseholdLanState: 'paired',
    },
  };
}

function withoutWrongDeviceRouteSecurity() {
  return {
    ...readModel,
    routeSecurityChecks: readModel.routeSecurityChecks.filter((check) => check.check !== 'wrong-device'),
  };
}

function withoutAddDeviceCanonicalActivityTargetSurface() {
  return {
    ...readModel,
    addDeviceReadModel: {
      ...readModel.addDeviceReadModel,
      canonicalHouseholdDevices: readModel.addDeviceReadModel.canonicalHouseholdDevices.map((device) => ({
        ...device,
        policyTargetSurfaces: device.policyTargetSurfaces.filter((surface) => surface !== 'activity'),
      })),
    },
  };
}

function withPhysicalHostGateOverclaim() {
  return {
    ...readModel,
    manualProofGates: readModel.manualProofGates.map((gate) =>
      gate.gate === 'two-physical-household-hosts' ? { ...gate, state: 'ci-mechanical-proof' } : gate
    ),
  };
}

function withoutExpiredPairingRequest() {
  return {
    ...readModel,
    addDeviceReadModel: {
      ...readModel.addDeviceReadModel,
      pairingRequests: readModel.addDeviceReadModel.pairingRequests.filter(
        (request) => request.pairingState !== 'expired'
      ),
    },
  };
}
