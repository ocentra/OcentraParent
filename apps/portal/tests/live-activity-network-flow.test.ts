import { describe, expect, it } from 'vitest';
import type { ActivityNetworkFlowReadModel } from '@ocentra-parent/activity-domain/network-flow';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { shouldRenderNetworkEvidenceDrawerRoute } from '../src/NetworkEvidenceDrawerRoutePanel';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { networkEvidenceDrawerSummary } from '../src/network-evidence-drawer';

describe('portal live activity network flow state', () => {
  registerNetworkFlowReadModelTests();
  registerNetworkFlowRejectionTests();
  registerNetworkEvidenceDrawerTests();
});

function registerNetworkFlowReadModelTests(): void {
  it('parses real service network flow read-model payload fields', () => {
    const state = resolveLiveActivityState([networkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);

    expectNetworkReadModelCounts(readModel, 1);
    expectNetworkFlowRow(readModel);
  });

  it('keeps newest buffered network flow read models when sentAt ties', () => {
    const state = resolveLiveActivityState([networkFlowEvent(), tiedOlderEmptyNetworkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);

    expectNetworkReadModelCounts(readModel, 1);
    expectNetworkFlowRow(readModel);
  });

  it('keeps evidence-backed network flow read models when a later empty refresh arrives', () => {
    const state = resolveLiveActivityState([latestEmptyNetworkFlowEvent(), networkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);

    expectNetworkReadModelCounts(readModel, 1);
    expectNetworkFlowRow(readModel);
  });

  it('keeps empty network flow read models visible without inventing destinations', () => {
    const state = resolveLiveActivityState([emptyNetworkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);
    const summary = networkEvidenceDrawerSummary(readModel);

    expectNetworkReadModelCounts(readModel, 0);
    expect(readModel.rows).toEqual([]);
    expect(readModel.capabilityStatus).toBe('no-network-observations');
    expect(summary.evidenceReferences).toBe('Not reported');
    expect(summary.exactUrlClaim).toBe('Not reported');
    expect(summary.platformState).toBe('child-device-query-store | no-network-observations');
    expect(summary.readModelRows).toBe('0 | 0 | 0 | 0');
    expect(summary.degradedState).toBe('no-network-observations');
  });

  it('renders deleted and degraded network status from service-backed read models', () => {
    const state = resolveLiveActivityState([deletedNetworkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);
    const summary = networkEvidenceDrawerSummary(readModel);

    expectNetworkReadModelDeletedCounts(readModel);
    expect(readModel.rows).toEqual([]);
    expect(summary.sourceQuality).toBe('adapter-error');
    expect(summary.platformState).toBe('child-device-query-store | adapter-error');
    expect(summary.readModelRows).toBe('0 | 0 | 1 | 0');
    expect(summary.retentionState).toBe('activity-network-flow-deleted | 2026-05-21T02:05:00Z | network-evidence-1');
    expect(summary.deletedEvidenceReferences).toBe('network-evidence-1');
    expect(summary.degradedState).toBe('adapter-error');
    expect(summary.policyDecisionRef).toBe('Not reported');
    expect(summary.interventionResultRef).toBe('Not reported');
  });
}

function registerNetworkFlowRejectionTests(): void {
  it('rejects active network flow rows when service payload lacks evidence digest', () => {
    const event = networkFlowEvent();
    const state = resolveLiveActivityState([
      {
        ...event,
        payload: {
          ...event.payload,
          activityDigest: null,
        },
      },
    ]);

    expect(state.networkFlowReadModel).toBeNull();
  });

  it('rejects network flow read models with mismatched returned and active counts', () => {
    const event = networkFlowEvent();
    const returnedMismatch = resolveLiveActivityState([
      {
        ...event,
        payload: {
          ...event.payload,
          returned: 2,
        },
      },
    ]);
    const activeMismatch = resolveLiveActivityState([
      {
        ...event,
        payload: {
          ...event.payload,
          activeRows: 2,
        },
      },
    ]);

    expect(returnedMismatch.networkFlowReadModel).toBeNull();
    expect(activeMismatch.networkFlowReadModel).toBeNull();
  });
}

function registerNetworkEvidenceDrawerTests(): void {
  it('projects the parent network evidence drawer without unsupported claims', () => {
    const state = resolveLiveActivityState([networkFlowEvent()]);
    const summary = networkEvidenceDrawerSummary(state.networkFlowReadModel);

    expect(summary.evidenceId).toBe('activity-network-flow-1');
    expect(summary.sourceAdapter).toBe('windows-network-snapshot');
    expect(summary.sourceQuality).toBe('available');
    expect(summary.platformState).toBe('child-device-query-store | available');
    expect(summary.readModelRows).toBe('1 | 1 | 0 | 1');
    expect(summary.localEndpoint).toBe('127.0.0.1 | 4242');
    expect(summary.remoteEndpoint).toBe('203.0.113.10 | 443');
    expect(summary.domainEvidenceRef).toBe('example-network.test | domain-observed');
    expect(summary.processRef).toBe('notepad.exe | 4242 | process-attributed');
    expect(summary.evidenceReferences).toBe('network-evidence-1 | network-journal-1');
    expect(summary.exactUrlClaim).toBe('Not reported');
    expect(summary.aiAuditRef).toBe('Not reported');
    expect(summary.policyDecisionRef).toBe('Not reported');
    expect(summary.interventionResultRef).toBe('Not reported');
    expect(summary.retentionState).toBe('0 | 1');
    expect(summary.deletedEvidenceReferences).toBe('Not reported');
    expect(summary.degradedState).toBe('available | domain-observed | process-attributed');
  });

  it('mounts the network evidence drawer on the activity product route only', () => {
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Activity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Overview)).toBe(false);
  });
}

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

function expectNetworkReadModelDeletedCounts(readModel: ActivityNetworkFlowReadModel): void {
  expect(readModel.returned).toBe(0);
  expect(readModel.activeRows).toBe(0);
  expect(readModel.tombstoneRows).toBe(1);
  expect(readModel.exportableRows).toBe(0);
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

function tiedOlderEmptyNetworkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...emptyNetworkFlowEvent(),
    eventId: 'evt-network-empty-tie',
    correlationId: 'cmd-network-empty-tie',
  });
}

function latestEmptyNetworkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...emptyNetworkFlowEvent(),
    eventId: 'evt-network-empty-later',
    correlationId: 'cmd-network-empty-later',
    sentAt: '2026-05-21T02:00:02Z',
  });
}

function deletedNetworkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network-deleted',
    correlationId: 'cmd-network-deleted',
    sentAt: '2026-05-21T02:05:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.network.flow.read-model.reported',
    severity: 'warn',
    payload: {
      generatedAt: '2026-05-21T02:05:01Z',
      custody: 'child-device-query-store',
      limit: 10,
      returned: 0,
      activeRows: 0,
      tombstoneRows: 1,
      exportableRows: 0,
      capabilityStatus: 'adapter-error',
      latestEventId: null,
      latestObservedAt: null,
      latestTombstoneEventId: 'activity-network-flow-deleted',
      latestTombstoneObservedAt: '2026-05-21T02:05:00Z',
      deletedEvidenceReferenceIds: 'network-evidence-1',
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
