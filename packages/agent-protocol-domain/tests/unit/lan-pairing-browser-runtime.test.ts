import { describe, expect, it } from 'vitest';
import {
  AgentCommand,
  AgentEvent,
  AgentLanBrowserAddDeviceRequestSchema,
  AgentLanBrowserDiscoveryScanRequestSchema,
  AgentLanBrowserRuntimeCommandSchema,
  AgentLanBrowserRuntimeEventSchema,
  AgentLanPairingSupportedWebSocketCommand,
  AgentProtocolDefaults,
} from '../../src/contracts';

describe('agent protocol browser-first LAN runtime commands', () => {
  it('parses scan and add-device request contracts for service-backed portal commands', () => {
    const scan = AgentLanBrowserDiscoveryScanRequestSchema.parse({
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      requestedDiscoverySource: 'local-service',
    });
    const addDevice = AgentLanBrowserAddDeviceRequestSchema.parse({
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      childDeviceId: 'child-device-1',
      parentDeviceId: 'parent-device-1',
      routeId: 'lan-route-local-network',
      origin: 'http://127.0.0.1:4678',
      issuedAt: '2026-06-01T16:50:00.000Z',
      expiresAt: '2099-06-01T16:55:00.000Z',
    });

    expect(scan.requestedDiscoverySource).toBe('local-service');
    expect(addDevice.childDeviceId).toBe('child-device-1');
    expect(addDevice.routeId).toBe('lan-route-local-network');
    expect(AgentLanBrowserRuntimeCommandSchema.parse('agent.lan-pairing.browser-discovery.scan')).toBe(
      AgentCommand.LanPairingBrowserDiscoveryScan
    );
    expect(AgentLanBrowserRuntimeCommandSchema.parse('agent.lan-pairing.add-device.request')).toBe(
      AgentCommand.LanPairingAddDeviceRequest
    );
    expect(AgentLanBrowserRuntimeEventSchema.parse('agent.lan-pairing.add-device.reported')).toBe(
      AgentEvent.LanPairingAddDeviceReported
    );
    expect(AgentLanPairingSupportedWebSocketCommand.BrowserDiscoveryScan).toBe(
      'agent.lan-pairing.browser-discovery.scan'
    );
    expect(AgentLanPairingSupportedWebSocketCommand.AddDeviceRequest).toBe('agent.lan-pairing.add-device.request');
  });
});
