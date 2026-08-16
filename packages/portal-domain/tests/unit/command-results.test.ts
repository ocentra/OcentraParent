import { describe, expect, it } from 'vitest';
import { PortalCommandResultEvents, isCommandResultEvent, latestCommandResult } from '../../src/command-results';
import {
  PortalAgentEvent as AgentEvent,
  type PortalAgentEventName as AgentEventName,
  type PortalRouteEventRecord,
} from '../../src/portal-contract-adapter';

describe('portal command result contract', () => {
  it('recognizes portal-selectable command result events from the canonical list', () => {
    expect(PortalCommandResultEvents).toContain(AgentEvent.ActivityTrackingReadModelReported);
    expect(PortalCommandResultEvents).toContain(AgentEvent.LanPairingStatusReported);
    expect(isCommandResultEvent(AgentEvent.ActivityTrackingReadModelReported)).toBe(true);
    expect(isCommandResultEvent(AgentEvent.LanPairingStatusReported)).toBe(true);
    expect(isCommandResultEvent(AgentEvent.LanPairingBrowserDiscoveryReported)).toBe(false);
  });

  it('selects the latest matching command result without parsing event shapes in UI', () => {
    const health = event(AgentEvent.HealthReported, 'health-1');
    const tracking = event(AgentEvent.ActivityTrackingReadModelReported, 'tracking-1');

    expect(latestCommandResult([health, tracking], AgentEvent.ActivityTrackingReadModelReported)).toBe(tracking);
    expect(latestCommandResult([health], AgentEvent.ActivityTrackingReadModelReported)).toBeNull();
  });
});

function event(eventName: AgentEventName, eventId: string): PortalRouteEventRecord {
  return {
    eventId,
    correlationId: `${eventId}-correlation`,
    sentAt: '2026-06-12T00:00:00.000Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: eventName,
    severity: 'info',
    payload: {},
    snapshot: null,
  };
}
