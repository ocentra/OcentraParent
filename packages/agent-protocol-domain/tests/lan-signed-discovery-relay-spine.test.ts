import { describe, expect, it } from 'vitest';
import {
  AgentLanBrowserAddDeviceReadModelSchema,
  AgentLanSignedDiscoveryRelaySpineSchema,
  AgentProtocolDefaults,
} from '../src/contracts';

const generatedAt = '2026-06-02T11:40:00.000Z';
const routeId = 'lan-route-local-network';

describe('agent protocol signed discovery relay spine', () => {
  it('parses the signed discovery relay spine payload shape', () => {
    const parsed = AgentLanSignedDiscoveryRelaySpineSchema.parse(signedDiscoveryRelaySpine());

    expect(parsed.adapterRows).toHaveLength(8);
    expect(parsed.signedProofRows.map((row) => row.rejectionReason).filter(Boolean)).toEqual([
      'anonymous',
      'expired',
      'replayed',
      'wrong-origin',
      'wrong-device',
      'revoked',
      'stale',
    ]);
    expect(parsed.relayCacheRows.at(-1)?.custodyLabel).toBe('no-ocentra-child-data-custody');
  });

  it('carries the optional spine in the browser add-device read model', () => {
    const parsed = AgentLanBrowserAddDeviceReadModelSchema.parse({
      ...addDeviceReadModel(),
      signedDiscoveryRelaySpine: signedDiscoveryRelaySpine(),
    });

    expect(parsed.signedDiscoveryRelaySpine?.routeSafetyRows.map((row) => row.check)).toContain('wrong-route-rejected');
    expect(parsed.signedDiscoveryRelaySpine?.notImplemented).toContain('relay-route-queued-not-configured');
  });
});

function signedDiscoveryRelaySpine() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    generatedAt,
    adapterRows: adapterRows(),
    signedProofRows: signedProofRows(),
    routeSafetyRows: routeSafetyRows(),
    relayCacheRows: relayCacheRows(),
    manualProofRequired: [
      'mdns-name',
      'ssdp-name',
      'router-dhcp-name',
      'manual-direct-address',
      'signed-child-agent-hello',
      'signed-child-agent-heartbeat',
    ],
    notImplemented: [
      'relay-route-unavailable',
      'relay-route-queued-not-configured',
      'cache-route-unavailable',
      'parent-owned-storage-unavailable',
    ],
    claimsProved: [
      'adapter boundaries are represented for passive, router, manual, and signed child-agent sources',
      'route safety and relay cache custody are represented in typed protocol state',
    ],
    claimsNotProved: [
      'signed child-agent artifacts are still manual-required',
      'physical household LAN proof still requires real second host evidence',
      'relay or cache routes remain unavailable',
      'parent-owned storage remains unavailable',
    ],
  } as const;
}

function adapterRows() {
  return [
    adapterRow('passive-lan-neighbor', 'discovered', 'ci-mechanical-proof', 'strong', 'passive-lan-observation', null),
    adapterRow(
      'router-infrastructure',
      'discovered',
      'ci-mechanical-proof',
      'strong',
      'router-infrastructure-observation',
      null
    ),
    adapterRow(
      'mdns-name',
      'manual-required',
      'manual-required',
      'manual-required',
      'passive-lan-observation',
      'mDNS proof'
    ),
    adapterRow(
      'ssdp-name',
      'manual-required',
      'manual-required',
      'manual-required',
      'passive-lan-observation',
      'SSDP proof'
    ),
    adapterRow(
      'router-dhcp-name',
      'manual-required',
      'manual-required',
      'manual-required',
      'router-infrastructure-observation',
      'DHCP proof'
    ),
    adapterRow(
      'manual-direct-address',
      'manual-required',
      'manual-required',
      'manual-required',
      'manual-parent-entry',
      'manual address proof'
    ),
    adapterRow(
      'signed-child-agent-hello',
      'manual-required',
      'manual-required',
      'manual-required',
      'signed-child-agent-artifact',
      'hello proof'
    ),
    adapterRow(
      'signed-child-agent-heartbeat',
      'manual-required',
      'manual-required',
      'manual-required',
      'signed-child-agent-artifact',
      'heartbeat proof'
    ),
  ] as const;
}

function signedProofRows() {
  return [
    signedProof('signed-hello-manual-required', 'manual-required', 'queued', null, 'manual-required'),
    signedProof('signed-heartbeat-manual-required', 'manual-required', 'queued', null, 'manual-required'),
    signedProof('accepted-signed-child-agent-manual-required', 'manual-required', 'queued', null, 'manual-required'),
    signedProof('unauthenticated-caller-rejected', 'rejected', 'rejected', 'anonymous', 'ci-mechanical-proof'),
    signedProof('expired-signed-proof-rejected', 'expired', 'rejected', 'expired', 'ci-mechanical-proof'),
    signedProof('replayed-signed-proof-rejected', 'rejected', 'rejected', 'replayed', 'ci-mechanical-proof'),
    signedProof('wrong-origin-signed-proof-rejected', 'rejected', 'rejected', 'wrong-origin', 'ci-mechanical-proof'),
    signedProof('wrong-device-signed-proof-rejected', 'rejected', 'rejected', 'wrong-device', 'ci-mechanical-proof'),
    signedProof('revoked-signed-proof-rejected', 'revoked', 'rejected', 'revoked', 'ci-mechanical-proof'),
    signedProof('stale-signed-proof-rejected', 'stale', 'rejected', 'stale', 'ci-mechanical-proof'),
  ] as const;
}

function routeSafetyRows() {
  return [
    routeSafety('trusted-registry-restart-recovery', routeId, 'paired', 'accepted', null, 'parent-local-service'),
    routeSafety('selected-route-custody', routeId, 'paired', 'accepted', null, 'parent-local-service'),
    routeSafety('stale-selected-device-rejected', routeId, 'stale', 'rejected', 'stale', 'parent-local-service'),
    routeSafety('offline-selected-device-rejected', routeId, 'offline', 'rejected', 'offline', 'parent-local-service'),
    routeSafety('wrong-route-rejected', routeId, 'rejected', 'rejected', 'wrong-device', 'parent-local-service'),
    routeSafety('revoked-route-rejected', routeId, 'revoked', 'rejected', 'revoked', 'parent-local-service'),
    routeSafety('parent-assign-decision-audited', routeId, 'discovered', 'accepted', null, 'parent-local-service'),
    routeSafety('parent-rename-decision-audited', routeId, 'discovered', 'accepted', null, 'parent-local-service'),
    routeSafety('parent-ignore-decision-audited', routeId, 'discovered', 'accepted', null, 'parent-local-service'),
    routeSafety('parent-restore-decision-audited', routeId, 'discovered', 'accepted', null, 'parent-local-service'),
    routeSafety('parent-trust-decision-audited', routeId, 'paired', 'accepted', null, 'parent-local-service'),
    routeSafety('parent-revoke-decision-audited', routeId, 'revoked', 'accepted', null, 'parent-local-service'),
  ] as const;
}

function relayCacheRows() {
  return [
    relayCache(
      'relay-route-unavailable',
      'unavailable',
      'unavailable',
      'not-implemented',
      'no-ocentra-child-data-custody'
    ),
    relayCache(
      'relay-route-queued-not-configured',
      'queued-not-configured',
      'pending',
      'not-implemented',
      'no-ocentra-child-data-custody'
    ),
    relayCache(
      'cache-route-unavailable',
      'unavailable',
      'unavailable',
      'not-implemented',
      'no-ocentra-child-data-custody'
    ),
    relayCache(
      'parent-owned-storage-unavailable',
      'unavailable',
      'unavailable',
      'not-implemented',
      'parent-owned-storage-unavailable'
    ),
    relayCache(
      'ocentra-child-data-custody-not-claimed',
      'local-first',
      'unavailable',
      'ci-mechanical-proof',
      'no-ocentra-child-data-custody'
    ),
  ] as const;
}

function adapterRow(
  adapter: unknown,
  discoveryState: unknown,
  proofState: unknown,
  sourceConfidence: unknown,
  custodyLabel: unknown,
  requiredArtifactSummary: unknown
) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    adapter,
    discoveryState,
    proofState,
    sourceConfidence,
    custodyLabel,
    runtimeOwner: proofState === 'manual-required' ? 'manual-proof' : 'rust-service-read-model',
    evidenceLabel: `${String(adapter)} adapter boundary`,
    requiredArtifactSummary,
  };
}

function signedProof(
  check: unknown,
  discoveryState: unknown,
  responseState: unknown,
  rejectionReason: unknown,
  proofState: unknown
) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    check,
    discoveryState,
    responseState,
    rejectionReason,
    proofState,
    runtimeOwner: proofState === 'manual-required' ? 'manual-proof' : 'rust-service-read-model',
    evidenceLabel: `${String(check)} signed proof state`,
  };
}

function routeSafety(
  check: unknown,
  route: unknown,
  discoveryState: unknown,
  responseState: unknown,
  rejectionReason: unknown,
  custodyLabel: unknown
) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    check,
    routeId: route,
    discoveryState,
    responseState,
    rejectionReason,
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'rust-service-read-model',
    custodyLabel,
    evidenceLabel: `${String(check)} route safety state`,
  };
}

function relayCache(
  check: unknown,
  decisionState: unknown,
  discoveryState: unknown,
  proofState: unknown,
  custodyLabel: unknown
) {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    check,
    decisionState,
    discoveryState,
    proofState,
    runtimeOwner: proofState === 'ci-mechanical-proof' ? 'rust-service-read-model' : 'manual-proof',
    custodyLabel,
    evidenceLabel: `${String(check)} relay cache state`,
  };
}

function addDeviceReadModel() {
  return {
    schemaVersion: AgentProtocolDefaults.SchemaVersion,
    generatedAt,
    discoverySource: 'local-service',
    addDeviceState: AgentProtocolDefaults.LanProductionDiscoveryState.Pending,
    localServiceDiscoveryState: AgentProtocolDefaults.LanProductionDiscoveryState.Pending,
    physicalHouseholdLanState: AgentProtocolDefaults.LanProductionDiscoveryState.ManualRequired,
    cloudRelayState: AgentProtocolDefaults.LanProductionDiscoveryState.Unavailable,
    scanSummary: {
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      sourceLabels: ['local-service'],
      scannedDeviceCount: 0,
      agentDeviceCount: 0,
      passiveDeviceCount: 0,
      infrastructureDeviceCount: 0,
      unsupportedDeviceCount: 0,
    },
    discoveredDevices: [],
    canonicalHouseholdDevices: [],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    householdDeviceDecisions: [],
    trustedDeviceIds: [],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      selectedChildDeviceId: null,
      routeId: null,
      pairingId: null,
      trustState: 'unpaired',
      reachability: 'offline',
      readyForControl: false,
      staleAt: null,
      offlineAt: null,
    },
    controllerAuthority: 'active-controller',
    observerAuthority: 'observer',
    routeRequirementLabels: ['allowed-origin'],
    auditCheckLabels: ['anonymous', 'wrong-origin', 'wrong-device'],
    honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
  } as const;
}
