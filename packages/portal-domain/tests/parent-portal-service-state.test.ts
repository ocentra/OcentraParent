import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PARENT_PORTAL_SERVICE_STATE, resolveParentPortalServiceState } from '../src/contracts';

describe('portal service-backed parent portal state', () => {
  it('uses real service events as the overview and manage row source', () => {
    const state = serviceBackedState();
    expect(state.content.modes.parentOverview.rowSource).toBe('api');
    expect(state.content.modes.parentManage.rowSource).toBe('api');
    expect(state.userEntry?.label).toBe('Local agent');
    expect(state.parentPortalRows.map((row) => row.label)).toEqual([
      'Local agent',
      'LAN discovery',
      'Device pairing',
      'Browser activity',
      'Activity reports',
      'Network tracking',
    ]);
  });

  it('maps LAN and device service events into visible row readiness', () => {
    const state = serviceBackedState();

    expect(state.parentPortalRows[1]).toMatchObject({
      readyCount: 2,
      gapCount: 0,
      trend: 'paired',
    });
    expect(state.parentPortalRows[2]).toMatchObject({
      readyCount: 3,
      gapCount: 0,
      trend: 'online',
    });
  });

  it('keeps unavailable local-service states visible instead of falling back to sample rows', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [],
    });

    expect(state.content.modes.parentOverview.rowSource).toBe('api');
    expect(state.userEntry?.trend).toBe('LOCAL');
    expect(state.parentPortalRows[1]).toMatchObject({
      label: 'LAN discovery',
      readyCount: 0,
      gapCount: 1,
      trend: 'not-reported',
    });
    expect(state.parentPortalRows[3]).toMatchObject({
      label: 'Browser activity',
      readyCount: 0,
      gapCount: 1,
      trend: 'not-reported',
    });
  });
});

function serviceBackedState() {
  return resolveParentPortalServiceState({
    connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
    events: [
      lanStatusEvent(),
      payloadEvent(AgentEvent.BrowserManagedStatusReported, {
        [AgentProtocolDefaults.Field.ManagedState]: 'managed',
      }),
      payloadEvent(AgentEvent.ActivityRecentSummaryReported, {
        [AgentProtocolDefaults.Field.Returned]: 2,
      }),
      payloadEvent(AgentEvent.NetworkFlowReadModelReported, {
        [AgentProtocolDefaults.Field.Returned]: 3,
        [AgentProtocolDefaults.Field.CapabilityStatus]: 'ready',
      }),
    ],
  });
}

function lanStatusEvent(): AgentEventEnvelope {
  return payloadEvent(AgentEvent.LanPairingStatusReported, {
    [AgentProtocolDefaults.Field.LanDiscoveryState]: 'paired',
    [AgentProtocolDefaults.Field.LanPairingState]: 'paired',
    [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 2,
    [AgentProtocolDefaults.Field.LanSelectedChildDeviceId]: 'child-device-1',
    [AgentProtocolDefaults.Field.LanSelectedDeviceReachability]: 'online',
  });
}

function payloadEvent(event: AgentEventName, payload: AgentProtocolLogFields): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: `evt-${event}`,
    correlationId: `cmd-${event}`,
    sentAt: '2026-06-01T13:55:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event,
    severity: 'info',
    payload,
    snapshot: null,
  });
}
