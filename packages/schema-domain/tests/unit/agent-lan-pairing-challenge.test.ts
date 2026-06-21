import { describe, expect, it } from 'vitest';
import { AgentLanPairingChallengeRequestSchema } from '../../src/agent-lan-pairing-challenge';
import { AgentProtocolSchemaVersion } from '../../src/event-primitives';

describe('agent LAN pairing challenge request contract', () => {
  it('accepts the typed LAN challenge request shape', () => {
    const parsed = AgentLanPairingChallengeRequestSchema.safeParse({
      schemaVersion: AgentProtocolSchemaVersion,
      childDeviceId: 'child-device-1',
      parentDeviceId: 'parent-device-1',
      routeId: 'lan-route-child-1',
      origin: 'http://127.0.0.1:4678',
      issuedAt: '2026-06-20T19:40:00Z',
      expiresAt: '2026-06-20T19:45:00Z',
    });

    expect(parsed.success).toBe(true);
  });

  it('rejects empty branded values', () => {
    const parsed = AgentLanPairingChallengeRequestSchema.safeParse({
      schemaVersion: AgentProtocolSchemaVersion,
      childDeviceId: '',
      parentDeviceId: 'parent-device-1',
      routeId: '',
      origin: '',
      issuedAt: '2026-06-20T19:40:00Z',
      expiresAt: '2026-06-20T19:45:00Z',
    });

    expect(parsed.success).toBe(false);
  });
});
