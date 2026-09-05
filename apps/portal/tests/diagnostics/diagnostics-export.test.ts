import { describe, expect, it } from 'vitest';
import { applyParentRouteSnapshot, createPortalRuntimeState } from '../../src/portal-state';
import { buildDiagnosticsExport } from '../../src/diagnostics-export';
import {
  ParentBridgeConnectionState,
  type ParentRouteEventSnapshot,
  type ParentRouteLiveActivitySnapshot,
} from '../../generated/parent-ui-bridge';
import { networkFlowReadModelSnapshot } from '../live-activity/live-activity-state-test-support';

describe('portal diagnostics export', () => {
  it('copies connection, health, event, and read-model summaries without raw service payload dumps', () => {
    const state = createPortalRuntimeState();
    applyDevicesSnapshot(state, {
      recentSummary: recentSummarySnapshot('notepad.exe'),
      networkFlowReadModel: networkFlowReadModelSnapshot(),
    });
    state.connectionState = ParentBridgeConnectionState.Connected;
    state.events.unshift(networkFlowEvent(), recentSummaryEvent(), healthEvent());

    const copied = JSON.parse(buildDiagnosticsExport(state));

    expect(copied.schemaVersion).toBe(1);
    expect(copied.agent.agentUrl).toBe('host-bridge://tauri-parent');
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

  it('uses route snapshot live activity when raw agent events are absent', () => {
    const state = createPortalRuntimeState();
    applyDevicesSnapshot(state, {
      recentSummary: recentSummarySnapshot('Child Laptop'),
      networkFlowReadModel: networkFlowReadModelSnapshot(),
    });

    const copied = JSON.parse(buildDiagnosticsExport(state));

    expect(copied.activity.recentSummary.mostRecentSubjectName).toBe('Child Laptop');
    expect(copied.activity.networkFlowReadModel.rows[0].destinationDomain).toBe('example-network.test');
  });
});

function applyDevicesSnapshot(
  state: ReturnType<typeof createPortalRuntimeState>,
  liveActivity?: Pick<ParentRouteLiveActivitySnapshot, 'recentSummary' | 'networkFlowReadModel'>
): void {
  applyParentRouteSnapshot(state, {
    schemaVersion: 1,
    route: 'devices',
    generatedAt: '2026-05-20T20:44:58Z',
    seasonLabel: 'LOCAL',
    lastUpdated: '2026-05-20T20:44:58Z',
    connectionState: 'connected',
    commandEnabled: true,
    agentEndpoint: 'host-bridge://tauri-parent',
    dataSource: 'rust-read-model',
    diagnosticPanelsEnabled: false,
    parentPortalShellStatus: {
      routeLabel: 'Devices',
      parentAccessState: 'proof-missing',
      globalConnectionState: 'manual-required',
      routeCapabilityState: 'available',
      dataSourceLabel: 'rust-read-model',
      cards: [],
    },
    ...(liveActivity === undefined ? {} : { liveActivity }),
    summary: {
      title: 'Devices',
      routeCapability: 'available',
      parentAccess: 'proof-missing',
      household: 'proof-missing',
      childDevice: 'proof-missing',
    },
  });
}

function recentSummarySnapshot(mostRecentSubjectName: string) {
  return {
    schemaVersion: 1,
    limit: 10,
    returned: 1,
    mostRecentSubjectName,
  } as const;
}

function healthEvent() {
  return {
    eventId: 'evt-health',
    correlationId: 'cmd-health',
    sentAt: '2026-05-20T20:45:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    event: 'agent.health.reported',
    severity: 'info',
    payload: {
      online: true,
      transport: 'websocket',
      privatePayloadExample: 'not copied',
    },
    snapshot: null,
  } satisfies ParentRouteEventSnapshot;
}

function recentSummaryEvent() {
  return {
    eventId: 'evt-recent',
    correlationId: 'cmd-recent',
    sentAt: '2026-05-20T20:45:01Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    event: 'activity.recent.summary.reported',
    severity: 'info',
    payload: {
      limit: 25,
      returned: 1,
      firstObservedAt: '2026-05-20T20:44:59Z',
      lastObservedAt: '2026-05-20T20:44:59Z',
      lastEventId: 'activity-event-1',
      mostRecentObserver: 'windows-process',
      mostRecentSubjectKind: 'process',
      mostRecentSubjectId: 'process-1',
      mostRecentSubjectName: 'notepad.exe',
    },
    snapshot: null,
  } satisfies ParentRouteEventSnapshot;
}

function networkFlowEvent() {
  return {
    eventId: 'evt-network',
    correlationId: 'cmd-network',
    sentAt: '2026-05-20T20:45:02Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    event: 'activity.network.flow.read-model.reported',
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
  } satisfies ParentRouteEventSnapshot;
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
