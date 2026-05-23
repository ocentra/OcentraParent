import { describe, expect, it } from 'vitest';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentLanPairingSupportedWebSocketCommand,
  AgentProtocolDefaults,
} from '../src/contracts';
import { AgentPairingStateSchema } from '../src/security';

describe('LAN pairing multi-device protocol contracts', () => {
  it('AgentLanPairingSupportedWebSocketCommand: includes route selection but no HTTP discovery claim', () => {
    expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).toEqual([
      'agent.lan-pairing.proof.submit',
      'agent.lan-pairing.route.select',
      'agent.lan-pairing.route.revoke',
      'agent.lan-pairing.status.get',
    ]);
    expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).not.toContain('agent.lan-pairing.discovery.http');
  });

  it('AgentPairingStateSchema: distinguishes unauthenticated, unpaired, and paired LAN states', () => {
    expect(AgentPairingStateSchema.parse('unauthenticated')).toBe('unauthenticated');
    expect(AgentProtocolDefaults.PairingState.Unpaired).toBe('unpaired');
    expect(AgentProtocolDefaults.PairingState.Paired).toBe('paired');
    expect(AgentProtocolDefaults.PairingState.Revoked).toBe('revoked');
    expect(AgentProtocolDefaults.LanSelectedDeviceReachability.Stale).toBe('stale');
    expect(AgentProtocolDefaults.LanSelectedDeviceReachability.Offline).toBe('offline');
  });

  it('AgentCommandEnvelopeSchema: accepts a route select command for a paired child device', () => {
    const parsed = AgentCommandEnvelopeSchema.safeParse({
      schemaVersion: AgentProtocolDefaults.SchemaVersion,
      messageId: 'cmd-lan-route-select-1',
      sentAt: '2026-05-23T21:05:00Z',
      source: AgentProtocolDefaults.Peer.PortalDev,
      target: {
        deviceId: 'child-device-1',
        platform: 'windows',
        route: 'local-network',
      },
      command: AgentCommand.LanPairingRouteSelect,
      payload: {
        [AgentProtocolDefaults.Field.LanIntentId]: 'intent-route-select-1',
        [AgentProtocolDefaults.Field.LanPairingId]: 'pairing-child-1',
        [AgentProtocolDefaults.Field.LanRouteId]: 'lan-route-child-1',
        [AgentProtocolDefaults.Field.LanProofDigest]: 'sha256:proof-child-1',
        [AgentProtocolDefaults.Field.Origin]: 'http://127.0.0.1:4678',
        [AgentProtocolDefaults.Field.StartedAt]: '2026-05-23T21:05:00Z',
        [AgentProtocolDefaults.Field.StaleAt]: '2026-05-23T21:10:00Z',
      },
    });

    expect(parsed.success).toBe(true);
  });
});
