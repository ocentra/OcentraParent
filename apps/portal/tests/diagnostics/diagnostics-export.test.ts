import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { ActivityEventKind } from '@ocentra-parent/schema-domain/evidence-kinds';
import { PortalConnectionState } from '@ocentra-parent/schema-domain/portal-contracts';
import { createPortalRuntimeState } from '../../src/portal-state';
import { buildDiagnosticsExport } from '../../src/diagnostics-export';

describe('portal diagnostics export', () => {
  it('copies connection, health, event, and read-model summaries without raw service payload dumps', () => {
    const state = createPortalRuntimeState(AgentProtocolDefaults.WebSocketUrl);
    state.connectionState = PortalConnectionState.Connected;
    state.events.unshift(networkFlowEvent(), recentSummaryEvent(), healthEvent());

    const copied = JSON.parse(buildDiagnosticsExport(state));

    expect(copied.schemaVersion).toBe(1);
    expect(copied.agent.agentUrl).toBe('ws://127.0.0.1:4477/api/dev/ws');
    expect(copied.agent.connectionState).toBe('connected');
    expect(copied.health.online).toBe(true);
    expect(copied.health.transport).toBe('websocket');
    expect(copied.events[0].eventId).toBe('evt-network');
    expect(copied.events[0].payload).toBeUndefined();
    expect(copied.activity.recentSummary.mostRecentSubjectName).toBe('notepad.exe');
    expect(copied.activity.activityMemoryGraphReadModel).toBeNull();
    expect(copied.activity.networkFlowReadModel.rows[0].destinationDomain).toBe('example-network.test');
    expect(copied.activity.networkLinuxNftablesLabStatus).toBeNull();
    expect(copied.activity.networkWindowsFirewallLabStatus).toBeNull();
    expect(copied.activity.networkWindowsWfpGateStatus).toBeNull();
  });
});

function healthEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-health',
    correlationId: 'cmd-health',
    sentAt: '2026-05-20T20:45:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.HealthReported,
    severity: 'info',
    payload: {
      online: true,
      transport: 'websocket',
      privatePayloadExample: 'not copied',
    },
    snapshot: null,
  });
}

function recentSummaryEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-recent',
    correlationId: 'cmd-recent',
    sentAt: '2026-05-20T20:45:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityRecentSummaryReported,
    severity: 'info',
    payload: {
      limit: 25,
      returned: 1,
      firstObservedAt: '2026-05-20T20:44:59Z',
      lastObservedAt: '2026-05-20T20:44:59Z',
      lastEventId: 'activity-event-1',
      mostRecentKind: ActivityEventKind.ProcessObserved,
      mostRecentObserver: 'windows-process',
      mostRecentSubjectKind: 'process',
      mostRecentSubjectId: 'process-1',
      mostRecentSubjectName: 'notepad.exe',
    },
    snapshot: null,
  });
}

function networkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network',
    correlationId: 'cmd-network',
    sentAt: '2026-05-20T20:45:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.NetworkFlowReadModelReported,
    severity: 'info',
    payload: {
      generatedAt: '2026-05-20T20:45:02Z',
      custody: 'child-device-query-store',
      limit: 10,
      returned: 1,
      activeRows: 1,
      tombstoneRows: 0,
      exportableRows: 1,
      capabilityStatus: 'available',
      latestEventId: 'activity-network-flow-1',
      latestObservedAt: '2026-05-20T20:45:01Z',
      latestTombstoneEventId: null,
      latestTombstoneObservedAt: null,
      deletedEvidenceReferenceIds: '',
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
      firstSeenAt: '2026-05-20T20:45:01Z',
      lastSeenAt: '2026-05-20T20:45:01Z',
      activityDigest: JSON.stringify(networkFlowDigest()),
    },
    snapshot: null,
  });
}

function networkFlowDigest() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-05-20T20:45:02Z',
    custody: 'child-device-query-store',
    evidence: [
      {
        evidenceId: 'network-evidence-1',
        kind: 'local-db-row',
        digest: null,
        uri: null,
      },
    ],
    topProcesses: [],
    topDestinations: [],
    unusualIndicators: [],
  };
}
