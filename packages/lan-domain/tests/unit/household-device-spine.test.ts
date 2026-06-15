import { describe, expect, it } from 'vitest';
import { HouseholdDeviceSpineEntrySchema, LanDiscoveryEvidenceRecordSchema } from '../../src/lan-pairing';

describe('canonical household device spine', () => {
  it('accepts one merged child-agent device with inventory and cross-surface targets', () => {
    const parsed = HouseholdDeviceSpineEntrySchema.parse(childAgentDevice());

    expect(parsed.canonicalDeviceId).toBe('lan-physical-mac-54271e97c331');
    expect(parsed.roleBadges).toEqual(['child-agent', 'portal', 'parent-controller']);
    expect(parsed.childAgentInventory?.gpuModel).toBe('GeForce RTX 2070 SUPER');
    expect(parsed.networkIdentity.evidenceRecords.map((record) => record.evidenceKind)).toEqual([
      'ip-address',
      'mac-address',
      'hostname',
      'interface',
      'child-agent-presence',
    ]);
    expect(parsed.policyTargetSurfaces).toEqual([
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

  it('rejects child-agent rows that are missing required policy target surfaces', () => {
    expect(() =>
      HouseholdDeviceSpineEntrySchema.parse({
        ...childAgentDevice(),
        policyTargetSurfaces: ['devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking'],
      })
    ).toThrow(/stable target surfaces/u);
  });

  it('rejects routers that pretend to be enrollable child-agent targets', () => {
    expect(() =>
      HouseholdDeviceSpineEntrySchema.parse({
        ...routerDevice(),
        enrollable: true,
        childAgentInventory: childAgentDevice().childAgentInventory,
      })
    ).toThrow(/non-enrollable/u);
  });

  it('rejects canonical LAN devices without source-backed evidence records', () => {
    expect(() =>
      HouseholdDeviceSpineEntrySchema.parse({
        ...routerDevice(),
        networkIdentity: {
          ...routerDevice().networkIdentity,
          evidenceRecords: [],
        },
      })
    ).toThrow(/evidence record/u);
  });

  it('parses individual LAN evidence records with source, confidence, and merge key', () => {
    const parsed = LanDiscoveryEvidenceRecordSchema.parse(evidenceRecord('hostname', 'GAMEDEV', 'hostname:gamedev'));

    expect(parsed.source).toBe('local-service');
    expect(parsed.confidence).toBe('confirmed');
    expect(parsed.mergeKey).toBe('hostname:gamedev');
  });
});

function childAgentDevice() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: 'lan-physical-mac-54271e97c331',
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
      confidence: 'agent-confirmed',
      staleAt: null,
      offlineAt: null,
      evidenceRecords: [
        evidenceRecord('ip-address', '192.168.2.42', 'ip:192.168.2.42'),
        evidenceRecord('mac-address', '54-27-1e-97-c3-31', 'mac:54271e97c331'),
        evidenceRecord('hostname', 'GAMEDEV', 'hostname:gamedev'),
        evidenceRecord('interface', 'Ethernet 2', 'interface:ethernet2'),
        evidenceRecord('child-agent-presence', 'ocentra-local-service', 'agent:local-dev-agent'),
      ],
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
      evidenceRecords: [
        {
          ...evidenceRecord('ip-address', '192.168.2.1', 'ip:192.168.2.1'),
          source: 'windows-neighbor-table',
          deviceId: 'lan-physical-mac-001122334455',
          confidence: 'strong',
        },
        {
          ...evidenceRecord('mac-address', '00-11-22-33-44-55', 'mac:001122334455'),
          source: 'windows-neighbor-table',
          deviceId: 'lan-physical-mac-001122334455',
          confidence: 'strong',
        },
        {
          ...evidenceRecord('router-classification', 'router', 'router:192.168.2.1'),
          source: 'windows-neighbor-table',
          deviceId: 'lan-physical-mac-001122334455',
          confidence: 'strong',
        },
      ],
    },
    childAgentInventory: null,
    policyTargetSurfaces: ['devices', 'network'],
  } as const;
}

function evidenceRecord(evidenceKind: string, value: string, mergeKey: string) {
  return {
    schemaVersion: 'v0.9',
    evidenceId: `lan-evidence-${evidenceKind}-${value.toLowerCase().replace(/[^a-z0-9]/gu, '')}`,
    source: 'local-service',
    evidenceKind,
    deviceId: 'lan-physical-mac-54271e97c331',
    value,
    normalizedValue: value.toLowerCase(),
    firstSeenAt: '2026-06-01T15:20:00.000Z',
    lastSeenAt: '2026-06-01T15:20:00.000Z',
    expiresAt: null,
    confidence: 'confirmed',
    mergeKey,
    note: null,
  } as const;
}
