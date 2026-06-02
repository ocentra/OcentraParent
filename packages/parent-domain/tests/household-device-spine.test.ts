import { describe, expect, it } from 'vitest';
import { HouseholdDeviceSpineReadModelSchema } from '../src/household-device-spine';

const generatedAt = '2026-06-01T20:50:00.000Z';

describe('canonical household device spine', () => {
  it('accepts one merged child-agent device with inventory and cross-surface targets', () => {
    const parsed = HouseholdDeviceSpineReadModelSchema.parse({
      schemaVersion: 'v0.9',
      generatedAt,
      devices: [childAgentDevice()],
    });

    expect(parsed.devices).toHaveLength(1);
    expect(parsed.devices[0]?.canonicalDeviceId).toBe('child-device-1');
    expect(parsed.devices[0]?.roleBadges).toEqual(['child-agent', 'portal', 'parent-controller']);
    expect(parsed.devices[0]?.childAgentInventory?.gpuModel).toBe('GeForce RTX 2070 SUPER');
    expect(parsed.devices[0]?.policyTargetSurfaces).toEqual([
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
  });

  it('rejects duplicate canonical physical device rows', () => {
    expect(() =>
      HouseholdDeviceSpineReadModelSchema.parse({
        schemaVersion: 'v0.9',
        generatedAt,
        devices: [childAgentDevice(), { ...childAgentDevice(), displayName: 'Duplicate' }],
      })
    ).toThrow(/one canonical row/u);
  });

  it('rejects routers that pretend to be enrollable child-agent targets', () => {
    expect(() =>
      HouseholdDeviceSpineReadModelSchema.parse({
        schemaVersion: 'v0.9',
        generatedAt,
        devices: [
          {
            ...routerDevice(),
            enrollable: true,
            childAgentInventory: childAgentDevice().childAgentInventory,
          },
        ],
      })
    ).toThrow(/non-child-agent/u);
  });
});

function childAgentDevice() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: 'child-device-1',
    displayName: 'GAMEDEV',
    classification: 'child-agent',
    roleBadges: ['child-agent', 'portal', 'parent-controller'],
    enrollable: true,
    discoveryState: 'paired',
    trustState: 'paired',
    routeId: 'lan-route-local-network',
    routeState: 'local-network',
    networkMode: 'local-network',
    sourceLabels: ['local-service', 'network-neighbor', 'trusted-registry'],
    networkIdentity: {
      hostname: 'GAMEDEV',
      ipAddresses: ['192.168.2.42'],
      macAddress: '54-27-1e-97-c3-31',
      macVendor: null,
      networkInterfaces: ['Ethernet 2'],
      reachability: 'online',
      confidence: 'mac-ip-match',
      staleAt: null,
      offlineAt: null,
    },
    childAgentInventory: {
      deviceName: 'GAMEDEV',
      platform: 'windows',
      os: 'windows',
      cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
      cpuCores: '12 cores / 24 logical',
      memoryTotal: '63 GiB',
      gpuModel: 'GeForce RTX 2070 SUPER',
      gpuDriver: '456.71',
      gpuMemory: '8192 MiB',
      nvidiaSmi: 'GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM',
      networkInterfaces: ['Ethernet 2'],
      capabilities: ['direct-websocket', 'device-inventory', 'pairing-route'],
      roleState: 'implemented',
      routeState: 'local-network',
      pairingTrustState: 'paired',
    },
    policyTargetSurfaces: ['devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai'],
  } as const;
}

function routerDevice() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: 'lan-physical-mac-001122334455',
    displayName: 'LAN 192.168.2.1',
    classification: 'network-infrastructure',
    roleBadges: [],
    enrollable: false,
    discoveryState: 'discovered',
    trustState: 'unpaired',
    routeId: null,
    routeState: 'unavailable',
    networkMode: 'local-network',
    sourceLabels: ['network-neighbor'],
    networkIdentity: {
      hostname: null,
      ipAddresses: ['192.168.2.1'],
      macAddress: '00-11-22-33-44-55',
      macVendor: null,
      networkInterfaces: ['Ethernet 2'],
      reachability: 'online',
      confidence: 'network-neighbor',
      staleAt: null,
      offlineAt: null,
    },
    childAgentInventory: null,
    policyTargetSurfaces: ['devices', 'network'],
  } as const;
}
