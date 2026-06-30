import { describe, expect, it } from 'vitest';
import {
  AgentLanDiscoveryEventHistoryStateSchema,
  AgentLanSelectedDeviceReadinessSchema,
} from '../../src/agent-lan-add-device';
import { AgentProtocolSchemaVersion } from '../../src/event-primitives';

describe('agent LAN add-device contract', () => {
  it('accepts the explicit agent-offline history state', () => {
    const parsed = AgentLanDiscoveryEventHistoryStateSchema.safeParse('agent-offline');

    expect(parsed.success).toBe(true);
  });

  it('accepts paired online selected readiness without a route as not-ready state', () => {
    const parsed = AgentLanSelectedDeviceReadinessSchema.safeParse({
      schemaVersion: AgentProtocolSchemaVersion,
      selectedChildDeviceId: 'trusted-child-1',
      routeId: null,
      pairingId: 'pairing-child-profile-1',
      trustState: 'paired',
      reachability: 'online',
      readyForControl: false,
      staleAt: null,
      offlineAt: null,
    });

    expect(parsed.success).toBe(true);
  });
});
