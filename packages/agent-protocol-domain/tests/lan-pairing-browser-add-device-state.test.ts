import { describe, expect, it } from 'vitest';
import { AgentLanBrowserAddDeviceReadModelSchema, AgentProtocolDefaults } from '../src/contracts';

describe('agent protocol browser-first LAN add-device state', () => {
  it('parses the service event read model D can consume without portal fixtures', () => {
    const parsed = AgentLanBrowserAddDeviceReadModelSchema.parse({
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      generatedAt: '2026-06-01T15:20:00.000Z',
      discoverySource: 'local-service',
      addDeviceState: AgentProtocolDefaults.LanProductionDiscoveryState.Pending,
      localServiceDiscoveryState: AgentProtocolDefaults.LanProductionDiscoveryState.Pending,
      physicalHouseholdLanState: AgentProtocolDefaults.LanProductionDiscoveryState.ManualRequired,
      cloudRelayState: AgentProtocolDefaults.LanProductionDiscoveryState.Unavailable,
      discoveredDevices: [],
      pairingRequests: [
        {
          schemaVersion: AgentProtocolDefaults.SchemaVersion,
          challengeId: 'challenge-child-device-1-parent-peer-1',
          childDeviceId: 'child-device-1',
          parentDeviceId: 'parent-device-1',
          routeId: 'lan-route-child-1',
          origin: 'http://127.0.0.1:4678',
          pairingState: 'pending',
          rejectionReason: null,
          issuedAt: '2026-06-01T15:20:00.000Z',
          expiresAt: '2026-06-01T15:25:00.000Z',
        },
      ],
      trustedDeviceRegistry: [
        {
          schemaVersion: AgentProtocolDefaults.SchemaVersion,
          pairingId: 'pairing-child-device-1',
          childDevice: {
            deviceId: 'child-device-1',
            childProfileId: null,
            label: 'Mia Windows PC',
            platform: 'windows',
          },
          parentDevice: {
            deviceId: 'parent-device-1',
            childProfileId: null,
            label: 'Parent Windows PC',
            platform: 'windows',
          },
          routeId: 'lan-route-child-1',
          origin: 'http://127.0.0.1:4678',
          proofDigest: 'sha256:lan-proof',
          trustState: 'paired',
          trustedAt: '2026-06-01T15:20:00.000Z',
          expiresAt: '2026-06-01T16:20:00.000Z',
          revokedAt: null,
        },
      ],
      trustedDeviceIds: ['child-device-1'],
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
      routeRequirementLabels: ['allowed-origin', 'target-device-match', 'non-replayed-intent'],
      auditCheckLabels: ['wrong-origin', 'wrong-device', 'replayed', 'stale', 'revoked'],
      honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
    });

    expect(parsed.discoverySource).toBe('local-service');
    expect(parsed.addDeviceState).toBe('pending');
    expect(parsed.physicalHouseholdLanState).toBe('manual-required');
    expect(parsed.cloudRelayState).toBe('unavailable');
    expect(parsed.trustedDeviceRegistry[0]?.childDevice.deviceId).toBe('child-device-1');
    expect(parsed.selectedDeviceReadiness.readyForControl).toBe(false);
    expect(AgentProtocolDefaults.Field.LanAddDeviceReadModel).toBe('addDeviceReadModel');
    expect(AgentProtocolDefaults.Field.LanSelectedDeviceReady).toBe('selectedDeviceReady');
  });
});
