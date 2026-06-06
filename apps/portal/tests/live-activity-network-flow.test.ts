import { describe, expect, it } from 'vitest';
import type { ActivityNetworkFlowReadModel } from '@ocentra-parent/activity-domain/network-flow';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { shouldRenderNetworkEvidenceDrawerRoute } from '../src/NetworkEvidenceDrawerRoutePanel';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { networkEvidenceDrawerSummary } from '../src/network-evidence-drawer';

describe('portal live activity network flow state', () => {
  it('parses real service network flow read-model payload fields', () => {
    const state = resolveLiveActivityState([networkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);

    expectNetworkReadModelCounts(readModel, 1);
    expectNetworkFlowRow(readModel);
  });

  it('projects the parent network evidence drawer without unsupported claims', () => {
    const state = resolveLiveActivityState([networkFlowEvent()]);
    const summary = networkEvidenceDrawerSummary(state.networkFlowReadModel);

    expect(summary.evidenceId).toBe('activity-network-flow-1');
    expect(summary.rowCounts).toBe('1 | 1 | 0 | 1');
    expect(summary.sourceAdapter).toBe('windows-network-snapshot');
    expect(summary.sourceQuality).toBe('available');
    expect(summary.localEndpoint).toBe('127.0.0.1 | 4242');
    expect(summary.remoteEndpoint).toBe('203.0.113.10 | 443');
    expect(summary.domainEvidenceRef).toBe('example-network.test | domain-observed');
    expect(summary.processRef).toBe('notepad.exe | 4242 | process-attributed');
    expect(summary.runtimeDelivery).toBe('1 | 1 | 0 | 11');
    expect(summary.runtimeStorage).toBe('11 | 0');
    expect(summary.manualReview).toBe('1');
    expect(summary.enforcementCommands).toBe('0');
    expect(summary.evidenceReferences).toBe('network-evidence-1 | network-journal-1');
    expect(summary.exactUrlClaim).toBe('Not reported');
    expect(summary.aiAuditRef).toBe('Not reported');
    expect(summary.policyDecisionRef).toBe('Not reported');
    expect(summary.interventionResultRef).toBe('Not reported');
    expect(summary.retentionState).toBe('0 | 1');
  });

  it('keeps empty network flow read models visible without inventing destinations', () => {
    const state = resolveLiveActivityState([emptyNetworkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);
    const summary = networkEvidenceDrawerSummary(readModel);

    expectNetworkReadModelCounts(readModel, 0);
    expect(readModel.rows).toEqual([]);
    expect(readModel.capabilityStatus).toBe('no-network-observations');
    expect(readModel.runtimeDelivery).toBeNull();
    expect(summary.rowCounts).toBe('0 | 0 | 0 | 0');
    expect(summary.runtimeDelivery).toBe('Not reported');
    expect(summary.evidenceReferences).toBe('Not reported');
    expect(summary.exactUrlClaim).toBe('Not reported');
  });

  it('mounts the network evidence drawer on the activity product route only', () => {
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Activity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Overview)).toBe(false);
  });
});

function requireNetworkFlowReadModel(readModel: ActivityNetworkFlowReadModel | null): ActivityNetworkFlowReadModel {
  expect(readModel).not.toBeNull();
  if (readModel === null) {
    throw new Error('network flow read-model missing');
  }
  return readModel;
}

function expectNetworkReadModelCounts(readModel: ActivityNetworkFlowReadModel, expectedRows: number): void {
  expect(readModel.returned).toBe(expectedRows);
  expect(readModel.activeRows).toBe(expectedRows);
  expect(readModel.tombstoneRows).toBe(0);
  expect(readModel.exportableRows).toBe(expectedRows);
}

function expectNetworkFlowRow(readModel: ActivityNetworkFlowReadModel): void {
  const row = readModel.rows[0];
  expect(row).toBeDefined();
  if (row === undefined) {
    throw new Error('network flow row missing');
  }
  expect(row.destinationDomain).toBe('example-network.test');
  expect(row.destinationEndpoint.port).toBe(443);
  expect(row.processName).toBe('notepad.exe');
  expect(row.evidence.map((evidence) => evidence.evidenceId)).toEqual(['network-evidence-1', 'network-journal-1']);
}

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
      activeRows: 1,
      tombstoneRows: 0,
      exportableRows: 1,
      capabilityStatus: 'available',
      latestEventId: 'activity-network-flow-1',
      latestObservedAt: '2026-05-21T02:00:00Z',
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
      firstSeenAt: '2026-05-21T02:00:00Z',
      lastSeenAt: '2026-05-21T02:00:00Z',
      activityDigest: JSON.stringify(networkFlowDigest()),
    },
    snapshot: null,
  });
}

function networkFlowDigest() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-05-21T02:00:01Z',
    custody: 'child-device-query-store',
    evidence: [
      {
        evidenceId: 'network-evidence-1',
        kind: 'local-db-row',
        digest: null,
        uri: null,
      },
      {
        evidenceId: 'network-journal-1',
        kind: 'journal-entry',
        digest: null,
        uri: null,
      },
    ],
    topProcesses: [],
    topDestinations: [],
    unusualIndicators: [],
    runtimeDelivery: {
      observedRows: 1,
      deliveredRows: 1,
      failedRows: 0,
      publishReports: 11,
      storedEvents: 11,
      deadLetters: 0,
      manualRequiredRows: 1,
      enforcementCommandEvents: 0,
    },
  };
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
      activeRows: 0,
      tombstoneRows: 0,
      exportableRows: 0,
      capabilityStatus: 'no-network-observations',
      latestEventId: null,
      latestObservedAt: null,
      latestTombstoneEventId: null,
      latestTombstoneObservedAt: null,
      deletedEvidenceReferenceIds: '',
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
