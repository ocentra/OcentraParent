import { describe, expect, it } from 'vitest';
import { LanBrowserAddDeviceReadModelSchema } from '@ocentra-parent/schema-domain/lan-pairing-device';
import {
  LanRelayAdapterKindSchema as LanSignedDiscoveryRelayAdapterKindSchema,
  LanRelayCacheCheckSchema as LanSignedDiscoveryRelayCacheCheckSchema,
  LanRelayRouteSafetyCheckSchema as LanSignedDiscoveryRelayRouteSafetyCheckSchema,
  LanRelaySignedProofCheckSchema as LanSignedDiscoveryRelaySignedProofCheckSchema,
  LanRelaySpineSchema as LanSignedDiscoveryRelaySpineSchema,
} from '@ocentra-parent/schema-domain/lan-relay-spine';

const generatedAt = '2026-06-02T11:40:00.000Z';
const routeId = 'lan-route-local-network';

describe('LAN signed discovery relay spine contracts', () => {
  it('parses the signed discovery adapter, rejection, route safety, and relay cache matrix', () => {
    const parsed = LanSignedDiscoveryRelaySpineSchema.parse(signedDiscoveryRelaySpine());

    expect(parsed.adapterRows.map((row) => row.adapter)).toEqual([
      'passive-lan-neighbor',
      'router-infrastructure',
      'mdns-name',
      'ssdp-name',
      'router-dhcp-name',
      'manual-direct-address',
      'signed-child-agent-hello',
      'signed-child-agent-heartbeat',
    ]);
    expect(parsed.signedProofRows.filter((row) => row.responseState === 'rejected')).toHaveLength(7);
    expect(parsed.routeSafetyRows.map((row) => row.check)).toContain('parent-revoke-decision-audited');
    expect(parsed.relayCacheRows.map((row) => row.check)).toEqual([
      'relay-route-unavailable',
      'relay-route-queued-not-configured',
      'cache-route-unavailable',
      'parent-owned-storage-unavailable',
      'ocentra-child-data-custody-not-claimed',
    ]);
  });

  it('extends the LAN add-device read model with the optional signed discovery relay spine', () => {
    const parsed = LanBrowserAddDeviceReadModelSchema.parse({
      ...addDeviceReadModel(),
      signedDiscoveryRelaySpine: signedDiscoveryRelaySpine(),
    });

    expect(parsed.signedDiscoveryRelaySpine?.manualProofRequired).toContain('signed-child-agent-hello');
    expect(parsed.signedDiscoveryRelaySpine?.notImplemented).toContain('parent-owned-storage-unavailable');
  });

  it('accepts structurally valid overclaim candidates because relay honesty is enforced outside the base schema', () => {
    for (const candidate of [withSignedHelloOverclaim(), withoutWrongRouteRejection(), withRelayOverclaim()]) {
      expect(LanSignedDiscoveryRelaySpineSchema.safeParse(candidate).success).toBe(true);
    }
  });

  it('keeps signed discovery relay vocabularies explicit', () => {
    expect(LanSignedDiscoveryRelayAdapterKindSchema.parse('manual-direct-address')).toBe('manual-direct-address');
    expect(LanSignedDiscoveryRelaySignedProofCheckSchema.parse('wrong-device-signed-proof-rejected')).toBe(
      'wrong-device-signed-proof-rejected'
    );
    expect(LanSignedDiscoveryRelayRouteSafetyCheckSchema.parse('revoked-route-rejected')).toBe(
      'revoked-route-rejected'
    );
    expect(LanSignedDiscoveryRelayCacheCheckSchema.safeParse('cloud-relay-ready').success).toBe(false);
  });
});

function signedDiscoveryRelaySpine() {
  return {
    schemaVersion: 'v0.9',
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
      'passive and router LAN discovery are separated from controllable child-agent signed discovery rows',
      'signed proof rejection states include unauthenticated, expired, replayed, wrong-origin, wrong-device, revoked, and stale outcomes',
      'route safety rows keep registry recovery, selected route custody, and parent decisions explicit',
      'relay and cache rows are local-first and do not claim Ocentra child-data custody',
    ],
    claimsNotProved: [
      'signed child-agent hello and heartbeat artifacts from a second installed device are still manual-required',
      'physical household LAN readiness still requires two real child-agent hosts',
      'relay or cache production routing remains unavailable or not implemented',
      'parent-owned storage is unavailable until a parent-selected storage adapter exists',
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
      'mDNS packet proof'
    ),
    adapterRow(
      'ssdp-name',
      'manual-required',
      'manual-required',
      'manual-required',
      'passive-lan-observation',
      'SSDP packet proof'
    ),
    adapterRow(
      'router-dhcp-name',
      'manual-required',
      'manual-required',
      'manual-required',
      'router-infrastructure-observation',
      'router DHCP proof'
    ),
    adapterRow(
      'manual-direct-address',
      'manual-required',
      'manual-required',
      'manual-required',
      'manual-parent-entry',
      'manual direct address proof'
    ),
    adapterRow(
      'signed-child-agent-hello',
      'manual-required',
      'manual-required',
      'manual-required',
      'signed-child-agent-artifact',
      'signed hello proof'
    ),
    adapterRow(
      'signed-child-agent-heartbeat',
      'manual-required',
      'manual-required',
      'manual-required',
      'signed-child-agent-artifact',
      'signed heartbeat proof'
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
    schemaVersion: 'v0.9',
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
    schemaVersion: 'v0.9',
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
    schemaVersion: 'v0.9',
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
    schemaVersion: 'v0.9',
    check,
    decisionState,
    discoveryState,
    proofState,
    runtimeOwner: proofState === 'ci-mechanical-proof' ? 'rust-service-read-model' : 'manual-proof',
    custodyLabel,
    evidenceLabel: `${String(check)} relay cache state`,
  };
}

function withSignedHelloOverclaim() {
  return {
    ...signedDiscoveryRelaySpine(),
    adapterRows: signedDiscoveryRelaySpine().adapterRows.map((row) =>
      row.adapter === 'signed-child-agent-hello' ? { ...row, proofState: 'ci-mechanical-proof' } : row
    ),
  };
}

function withoutWrongRouteRejection() {
  return {
    ...signedDiscoveryRelaySpine(),
    routeSafetyRows: signedDiscoveryRelaySpine().routeSafetyRows.filter((row) => row.check !== 'wrong-route-rejected'),
  };
}

function withRelayOverclaim() {
  return {
    ...signedDiscoveryRelaySpine(),
    relayCacheRows: signedDiscoveryRelaySpine().relayCacheRows.map((row) =>
      row.check === 'relay-route-unavailable' ? { ...row, proofState: 'ci-mechanical-proof' } : row
    ),
  };
}

function addDeviceReadModel() {
  return {
    schemaVersion: 'v0.9',
    generatedAt,
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
    discoveredDevices: [],
    canonicalHouseholdDevices: [],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    householdDeviceDecisions: [],
    trustedDeviceIds: [],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: 'v0.9',
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
