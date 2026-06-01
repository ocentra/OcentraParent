import { describe, expect, it } from 'vitest';
import { LanBrowserAddDeviceRuntimeRequestSchema, LanBrowserDiscoveryScanRequestSchema } from '../src/lan-pairing';

describe('parent-domain browser-first LAN runtime requests', () => {
  it('parses service scan and add-device requests without visible portal fixtures', () => {
    const scan = LanBrowserDiscoveryScanRequestSchema.parse({
      schemaVersion: 'v0.9',
      requestedDiscoverySource: 'local-service',
    });
    const addDevice = LanBrowserAddDeviceRuntimeRequestSchema.parse({
      schemaVersion: 'v0.9',
      childDeviceId: 'child-device-1',
      parentDeviceId: 'parent-device-1',
      routeId: 'lan-route-local-network',
      origin: 'http://127.0.0.1:4678',
      issuedAt: '2026-06-01T16:50:00.000Z',
      expiresAt: '2099-06-01T16:55:00.000Z',
    });

    expect(scan.requestedDiscoverySource).toBe('local-service');
    expect(addDevice.parentDeviceId).toBe('parent-device-1');
    expect(addDevice.origin).toBe('http://127.0.0.1:4678');
  });
});
