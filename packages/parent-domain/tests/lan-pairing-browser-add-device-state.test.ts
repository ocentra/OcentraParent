import { describe, expect, it } from 'vitest';
import { LanBrowserAddDeviceReadModelSchema, LanPairingProductionDiscoveryStates } from '../src/lan-pairing';

const generatedAt = '2026-06-01T15:20:00.000Z';

describe('browser-first LAN add-device read model', () => {
  it('parses service-backed local discovery with honest physical LAN boundaries', () => {
    const parsed = LanBrowserAddDeviceReadModelSchema.parse(readModelFixture());

    expect(parsed.discoverySource).toBe('local-service');
    expect(parsed.physicalHouseholdLanState).toBe('manual-required');
    expect(parsed.cloudRelayState).toBe('unavailable');
    expect(parsed.scanSummary).toMatchObject({
      sourceLabels: ['local-service'],
      scannedDeviceCount: 1,
      agentDeviceCount: 1,
    });
    expect(parsed.trustedDeviceRegistry[0]?.childDevice.deviceId).toBe('child-device-1');
    expect(parsed.trustedDeviceIds).toEqual(['child-device-1']);
    expect(parsed.selectedDeviceReadiness.readyForControl).toBe(false);
    expect(parsed.auditCheckLabels).toEqual(['wrong-origin', 'wrong-device', 'replayed', 'stale', 'revoked']);
    expect(LanPairingProductionDiscoveryStates.Rejected).toBe('rejected');
    expect(LanPairingProductionDiscoveryStates.Expired).toBe('expired');
    expect(LanPairingProductionDiscoveryStates.ManualRequired).toBe('manual-required');
  });

  it('rejects duplicate canonical household device rows in the add-device read model', () => {
    expect(() =>
      LanBrowserAddDeviceReadModelSchema.parse({
        ...readModelFixture(),
        canonicalHouseholdDevices: [canonicalHouseholdDevice(), canonicalHouseholdDevice()],
      })
    ).toThrow(/one canonical row/u);
  });
});

function readModelFixture() {
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
    discoveredDevices: [discoveredDevice()],
    canonicalHouseholdDevices: [canonicalHouseholdDevice()],
    pairingRequests: [pairingRequest()],
    trustedDeviceRegistry: [trustedRegistryEntry()],
    trustedDeviceIds: ['child-device-1'],
    revokedDeviceIds: [],
    selectedDeviceReadiness: selectedReadiness(),
    controllerAuthority: 'active-controller',
    observerAuthority: 'observer',
    routeRequirementLabels: ['allowed-origin', 'target-device-match', 'non-replayed-intent'],
    auditCheckLabels: ['wrong-origin', 'wrong-device', 'replayed', 'stale', 'revoked'],
    honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
  };
}

function canonicalHouseholdDevice() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: 'child-device-1',
    displayName: 'Mia Windows PC',
    classification: 'child-agent',
    roleBadges: ['child-agent'],
    enrollable: true,
    discoveryState: 'paired',
    trustState: 'paired',
    routeId: 'lan-route-child-1',
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
      staleAt: generatedAt,
      offlineAt: null,
    },
    childAgentInventory: null,
    policyTargetSurfaces: ['devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai'],
  } as const;
}

function discoveredDevice() {
  return {
    schemaVersion: 'v0.9',
    discoveredAt: generatedAt,
    childProfile: { childProfileId: 'child-profile-1', displayName: 'Mia' },
    childDevice: childDeviceRef(),
    agentPeerId: 'child-agent-1',
    routeId: 'lan-route-child-1',
    networkMode: 'local-network',
    reachability: 'stale',
    addressRef: 'local-service:child-device-1',
    discoveryStatus: 'websocket-direct',
    discoveryState: 'stale',
  };
}

function pairingRequest() {
  return {
    schemaVersion: 'v0.9',
    challengeId: 'challenge-child-device-1-parent-peer-1',
    childDeviceId: 'child-device-1',
    parentDeviceId: 'parent-device-1',
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    pairingState: 'pending',
    rejectionReason: null,
    issuedAt: generatedAt,
    expiresAt: '2026-06-01T15:25:00.000Z',
  };
}

function selectedReadiness() {
  return {
    schemaVersion: 'v0.9',
    selectedChildDeviceId: null,
    routeId: null,
    pairingId: null,
    trustState: 'unpaired',
    reachability: 'offline',
    readyForControl: false,
    staleAt: null,
    offlineAt: null,
  };
}

function childDeviceRef() {
  return {
    deviceId: 'child-device-1',
    childProfileId: 'child-profile-1',
    label: 'Mia Windows PC',
    platform: 'windows',
  };
}

function parentDeviceRef() {
  return {
    deviceId: 'parent-device-1',
    childProfileId: null,
    label: 'Parent Windows PC',
    platform: 'windows',
  };
}

function trustedRegistryEntry() {
  return {
    schemaVersion: 'v0.9',
    pairingId: 'pairing-child-device-1',
    childDevice: childDeviceRef(),
    parentDevice: parentDeviceRef(),
    routeId: 'lan-route-child-1',
    origin: 'http://127.0.0.1:4678',
    proofDigest: 'sha256:lan-proof',
    trustState: 'paired',
    trustedAt: generatedAt,
    expiresAt: '2026-06-01T16:20:00.000Z',
    revokedAt: null,
  };
}
