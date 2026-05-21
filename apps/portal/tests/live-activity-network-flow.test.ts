import { describe, expect, it } from 'vitest';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolveLiveActivityState } from '../src/live-activity-state';

describe('portal live activity network flow state', () => {
  it('parses real service network flow read-model payload fields', () => {
    const state = resolveLiveActivityState([networkFlowEvent()]);

    expect(state.networkFlowReadModel?.returned).toBe(1);
    expect(state.networkFlowReadModel?.rows[0]?.destinationDomain).toBe('example-network.test');
    expect(state.networkFlowReadModel?.rows[0]?.destinationEndpoint.port).toBe(443);
    expect(state.networkFlowReadModel?.rows[0]?.processName).toBe('notepad.exe');
  });

  it('keeps empty network flow read models visible without inventing destinations', () => {
    const state = resolveLiveActivityState([emptyNetworkFlowEvent()]);

    expect(state.networkFlowReadModel?.returned).toBe(0);
    expect(state.networkFlowReadModel?.rows).toEqual([]);
    expect(state.networkFlowReadModel?.capabilityStatus).toBe('no-network-observations');
  });
});

function networkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network',
    correlationId: 'cmd-network',
    sentAt: '2026-05-21T02:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.network.flow.read-model.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T02:00:01Z',
      custody: 'child-device-query-store',
      limit: 10,
      returned: 1,
      capabilityStatus: 'available',
      latestEventId: 'activity-network-flow-1',
      latestObservedAt: '2026-05-21T02:00:00Z',
      observer: 'windows-network',
      adapterId: 'windows-network-snapshot',
      networkProtocol: 'tcp',
      tcpState: 'established',
      localIp: '127.0.0.1',
      localPort: 4242,
      destinationIp: '203.0.113.10',
      destinationPort: 443,
      destinationDomain: 'example-network.test',
      domainAttributionStatus: 'domain-observed',
      processAttributionStatus: 'process-attributed',
      processId: 4242,
      processName: 'notepad.exe',
      connectionCount: 1,
      bytesSent: null,
      bytesReceived: null,
      firstSeenAt: '2026-05-21T02:00:00Z',
      lastSeenAt: '2026-05-21T02:00:00Z',
    },
    snapshot: null,
  });
}

function emptyNetworkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network',
    correlationId: 'cmd-network',
    sentAt: '2026-05-21T02:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.network.flow.read-model.reported',
    severity: 'info',
    payload: {
      generatedAt: '2026-05-21T02:00:01Z',
      custody: 'child-device-query-store',
      limit: 10,
      returned: 0,
      capabilityStatus: 'no-network-observations',
      latestEventId: null,
      latestObservedAt: null,
      observer: null,
      adapterId: null,
      networkProtocol: null,
      tcpState: null,
      localIp: null,
      localPort: null,
      destinationIp: null,
      destinationPort: null,
      destinationDomain: null,
      domainAttributionStatus: null,
      processAttributionStatus: null,
      processId: null,
      processName: null,
      connectionCount: null,
      bytesSent: null,
      bytesReceived: null,
      firstSeenAt: null,
      lastSeenAt: null,
    },
    snapshot: null,
  });
}
