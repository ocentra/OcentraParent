import { describe, expect, it } from 'vitest';
import {
  LanBrowserAddDeviceReadModelSchema,
  LanProductionHouseholdProofCapabilitySchema,
  LanProductionHouseholdProofSummarySchema,
} from '../src/lan-pairing';

const generatedAt = '2026-06-02T09:40:00.000Z';

describe('LAN production household proof contracts', () => {
  it('parses an honest production LAN household proof summary with manual and not implemented gaps', () => {
    const parsed = LanProductionHouseholdProofSummarySchema.parse(productionSummary());

    expect(parsed.statusRows.map((row) => row.capability)).toEqual([
      'signed-lan-hello',
      'signed-lan-heartbeat',
      'passive-neighbor-discovery',
      'router-neighbor-discovery',
      'mdns-name-discovery',
      'ssdp-name-discovery',
      'router-dhcp-name-discovery',
      'trusted-registry',
      'parent-assignment',
      'parent-rename',
      'parent-ignore',
      'parent-revocation',
      'route-custody',
      'stale-selected-device',
      'offline-selected-device',
      'relay-route',
      'cache-route',
      'second-physical-child-agent',
      'android-child-agent-parity',
      'ios-child-agent-parity',
      'store-signing',
    ]);
    expect(parsed.manualProofRequired).toContain('signed-lan-hello');
    expect(parsed.manualProofRequired).toContain('second-physical-child-agent');
    expect(parsed.notImplemented).toEqual(['relay-route', 'cache-route']);
    expect(parsed.claimsNotProved).toEqual([
      'physical household LAN readiness remains manual-required until two physical child-agent hosts and router/firewall artifacts are attached',
      'signed LAN hello and heartbeat remain manual-required until a second installed child agent signs them',
      'cloud relay routing storage and authentication are not implemented in this LAN proof',
      'Android child-agent parity remains manual-required until real device permission and transport artifacts are attached',
      'iOS child-agent parity remains manual-required until entitlement device and transport artifacts are attached',
      'store signing remains manual-required until signing store and release artifacts are attached',
    ]);
  });

  it('rejects summaries that overclaim signed discovery or omit relay and cache gaps', () => {
    const signedOverclaim = LanProductionHouseholdProofSummarySchema.safeParse({
      ...productionSummary(),
      statusRows: productionSummary().statusRows.map((row) =>
        row.capability === 'signed-lan-hello' ? { ...row, proofState: 'ci-mechanical-proof' } : row
      ),
    });
    const missingCacheGap = LanProductionHouseholdProofSummarySchema.safeParse({
      ...productionSummary(),
      notImplemented: ['relay-route'],
    });

    expect(signedOverclaim.success).toBe(false);
    expect(missingCacheGap.success).toBe(false);
  });

  it('extends the LAN add-device read model with an optional production household proof summary', () => {
    const parsed = LanBrowserAddDeviceReadModelSchema.parse({
      ...addDeviceReadModel(),
      productionHouseholdProof: productionSummary(),
    });

    expect(parsed.productionHouseholdProof?.statusRows).toHaveLength(21);
    expect(parsed.productionHouseholdProof?.manualProofRequired).toContain('ios-child-agent-parity');
  });

  it('keeps the production capability vocabulary explicit', () => {
    for (const capability of productionSummary().statusRows.map((row) => row.capability)) {
      expect(LanProductionHouseholdProofCapabilitySchema.parse(capability)).toBe(capability);
    }
    expect(LanProductionHouseholdProofCapabilitySchema.safeParse('production-ready-lan').success).toBe(false);
  });
});

function productionSummary() {
  return {
    schemaVersion: 'v0.9',
    generatedAt,
    statusRows: [
      manual('signed-lan-hello', 'second child-agent signed hello artifact'),
      manual('signed-lan-heartbeat', 'second child-agent signed heartbeat artifact'),
      ci('passive-neighbor-discovery', 'discovered'),
      ci('router-neighbor-discovery', 'discovered'),
      manual('mdns-name-discovery', 'mDNS packet or parser artifact'),
      manual('ssdp-name-discovery', 'SSDP packet or parser artifact'),
      manual('router-dhcp-name-discovery', 'router DHCP artifact'),
      ci('trusted-registry', 'paired'),
      ci('parent-assignment', 'discovered'),
      ci('parent-rename', 'discovered'),
      ci('parent-ignore', 'discovered'),
      ci('parent-revocation', 'revoked'),
      ci('route-custody', 'paired'),
      ci('stale-selected-device', 'stale'),
      ci('offline-selected-device', 'offline'),
      unavailable('relay-route'),
      unavailable('cache-route'),
      manual('second-physical-child-agent', 'two physical hosts proof JSON'),
      manual('android-child-agent-parity', 'real Android child-agent device artifact'),
      manual('ios-child-agent-parity', 'real iOS entitlement and device artifact'),
      manual('store-signing', 'signing and store artifact'),
    ],
    manualProofRequired: [
      'signed-lan-hello',
      'signed-lan-heartbeat',
      'mdns-name-discovery',
      'ssdp-name-discovery',
      'router-dhcp-name-discovery',
      'second-physical-child-agent',
      'android-child-agent-parity',
      'ios-child-agent-parity',
      'store-signing',
    ],
    notImplemented: ['relay-route', 'cache-route'],
    claimsProved: [
      'passive Windows neighbor evidence is represented in typed LAN read-model state',
      'trusted registry, route custody, stale/offline, and parent decisions are represented in typed LAN read-model state',
    ],
    claimsNotProved: [
      'physical household LAN readiness remains manual-required until two physical child-agent hosts and router/firewall artifacts are attached',
      'signed LAN hello and heartbeat remain manual-required until a second installed child agent signs them',
      'cloud relay routing storage and authentication are not implemented in this LAN proof',
      'Android child-agent parity remains manual-required until real device permission and transport artifacts are attached',
      'iOS child-agent parity remains manual-required until entitlement device and transport artifacts are attached',
      'store signing remains manual-required until signing store and release artifacts are attached',
    ],
  } as const;
}

function ci(capability: string, discoveryState: string) {
  return {
    schemaVersion: 'v0.9',
    capability,
    discoveryState,
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'rust-service-read-model',
    evidenceLabel: `${capability} service state`,
    requiredArtifactSummary: null,
  } as const;
}

function manual(capability: string, requiredArtifactSummary: string) {
  return {
    schemaVersion: 'v0.9',
    capability,
    discoveryState: 'manual-required',
    proofState: 'manual-required',
    runtimeOwner: 'manual-proof',
    evidenceLabel: `${capability} remains manual-required`,
    requiredArtifactSummary,
  } as const;
}

function unavailable(capability: string) {
  return {
    schemaVersion: 'v0.9',
    capability,
    discoveryState: 'unavailable',
    proofState: 'not-implemented',
    runtimeOwner: 'manual-proof',
    evidenceLabel: `${capability} not implemented`,
    requiredArtifactSummary: null,
  } as const;
}

function addDeviceReadModel() {
  return {
    schemaVersion: 'v0.9',
    generatedAt,
    discoverySource: 'local-service',
    addDeviceState: 'pending',
    localServiceDiscoveryState: 'pending',
    physicalHouseholdLanState: 'discovered',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: 'v0.9',
      sourceLabels: ['local-service', 'windows-neighbor-table'],
      scannedDeviceCount: 1,
      agentDeviceCount: 1,
      passiveDeviceCount: 1,
      infrastructureDeviceCount: 1,
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
    auditCheckLabels: ['wrong-origin'],
    honestNonClaims: ['cloud-relay-not-implemented'],
  } as const;
}
