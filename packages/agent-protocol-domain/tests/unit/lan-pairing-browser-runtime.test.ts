import { describe, expect, it } from 'vitest';
import {
  AgentCommand,
  AgentEvent,
  AgentLanBrowserAddDeviceRequestSchema,
  AgentLanBrowserDiscoveryScanRequestSchema,
  AgentLanBrowserRuntimeCommandSchema,
  AgentLanBrowserRuntimeEventSchema,
  AgentLanPairingSupportedWebSocketCommand,
} from '../../src/contracts';
import {
  AgentLanBrowserRuntimeCommandNameLiteral,
  AgentLanBrowserRuntimeEventNameLiteral,
} from '@ocentra-parent/schema-domain/lan-pairing-browser-runtime';
import { LanPairingSchemaVersion } from '@ocentra-parent/schema-domain/lan-pairing-values';

describe('agent protocol browser-first LAN runtime commands', () => {
  it('parses scan and add-device request contracts for service-backed portal commands', () => {
    const scan = AgentLanBrowserDiscoveryScanRequestSchema.parse({
      schemaVersion: LanPairingSchemaVersion.V0_9,
      requestedDiscoverySource: 'local-service',
    });
    const addDevice = AgentLanBrowserAddDeviceRequestSchema.parse({
      schemaVersion: LanPairingSchemaVersion.V0_9,
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
    expect(
      AgentLanBrowserRuntimeCommandSchema.parse(AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan)
    ).toBe(AgentCommand.LanPairingBrowserDiscoveryScan);
    expect(AgentLanBrowserRuntimeCommandSchema.parse(AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest)).toBe(
      AgentCommand.LanPairingAddDeviceRequest
    );
    expect(AgentLanBrowserRuntimeEventSchema.parse(AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported)).toBe(
      AgentEvent.LanPairingAddDeviceReported
    );
    expect(AgentLanPairingSupportedWebSocketCommand.BrowserDiscoveryScan).toBe(
      AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan
    );
    expect(AgentLanPairingSupportedWebSocketCommand.AddDeviceRequest).toBe(
      AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest
    );
  });
});
