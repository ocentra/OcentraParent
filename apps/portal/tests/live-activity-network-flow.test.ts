import { describe, expect, it } from 'vitest';
import type { ActivityNetworkFlowReadModel } from '@ocentra-parent/activity-domain/network-flow';
import { networkEvidenceDrawerSummary, PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { shouldRenderNetworkEvidenceDrawerRoute } from '../src/NetworkEvidenceDrawerRoutePanel';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { NetworkEvidenceDrawerProof } from './network-evidence-drawer-proof-fixture';

describe('portal live activity network flow state', () => {
  registerNetworkFlowReadModelTests();
  registerNetworkFlowNormalizationTests();
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

function registerNetworkFlowNormalizationTests(): void {
  it('normalizes blank optional Windows network fields before branded parsing', () => {
    const state = resolveLiveActivityState([blankOptionalWindowsNetworkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);
    const row = readModel.rows[0];
    const summary = networkEvidenceDrawerSummary(readModel);

    expectNetworkReadModelCounts(readModel, 1);
    expect(row).toBeDefined();
    expect(row?.destinationDomain).toBeNull();
    expect(row?.processName).toBeNull();
    expect(row?.evidence.map((evidence) => evidence.evidenceId)).toEqual([
      NetworkEvidenceDrawerProof.evidenceId,
      NetworkEvidenceDrawerProof.journalEvidenceId,
    ]);
    expect(summary.domainEvidenceRef).toBe(NetworkEvidenceDrawerProof.fields.domainAttributionStatus);
    expect(summary.processRef).toBe(NetworkEvidenceDrawerProof.fields.processAttributionStatus);
    expect(summary.evidenceReferences).toBe(
      `${NetworkEvidenceDrawerProof.evidenceId} | ${NetworkEvidenceDrawerProof.journalEvidenceId}`
    );
  });

  it('normalizes stringified Windows network counters before drawer parsing', () => {
    const state = resolveLiveActivityState([stringifiedWindowsNetworkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);
    const row = readModel.rows[0];
    const summary = networkEvidenceDrawerSummary(readModel);

    expectNetworkReadModelCounts(readModel, 1);
    expect(row?.localEndpoint.port).toBe(NetworkEvidenceDrawerProof.fields.localPort);
    expect(row?.destinationEndpoint.port).toBe(NetworkEvidenceDrawerProof.fields.destinationPort);
    expect(row?.processId).toBe(NetworkEvidenceDrawerProof.fields.pid);
    expect(row?.counters.connectionCount).toBe(1);
    expect(summary.evidenceReferences).toBe(
      `${NetworkEvidenceDrawerProof.evidenceId} | ${NetworkEvidenceDrawerProof.journalEvidenceId}`
    );
  });

  it('projects the latest service row when Windows returns aggregate row counts', () => {
    const state = resolveLiveActivityState([aggregateWindowsNetworkFlowEvent()]);
    const readModel = requireNetworkFlowReadModel(state.networkFlowReadModel);
    const summary = networkEvidenceDrawerSummary(readModel);

    expectNetworkReadModelCounts(readModel, 1);
    expect(readModel.latestEventId).toBe(NetworkEvidenceDrawerProof.eventId);
    expect(summary.evidenceReferences).toBe(
      `${NetworkEvidenceDrawerProof.evidenceId} | ${NetworkEvidenceDrawerProof.journalEvidenceId}`
    );
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

  it('rejects network flow read models with inconsistent tombstone/export counts', () => {
    const event = networkFlowEvent();
    const tombstoneMismatch = resolveLiveActivityState([
      {
        ...event,
        payload: {
          ...event.payload,
          tombstoneRows: 1,
        },
      },
    ]);

    expect(tombstoneMismatch.networkFlowReadModel).toBeNull();
  });
}

function registerNetworkEvidenceDrawerTests(): void {
  it('projects the parent network evidence drawer without unsupported claims', () => {
    const state = resolveLiveActivityState([networkFlowEvent()]);
    const summary = networkEvidenceDrawerSummary(state.networkFlowReadModel);

    expect(summary.evidenceId).toBe(NetworkEvidenceDrawerProof.eventId);
    expect(summary.sourceAdapter).toBe('windows-network-snapshot');
    expect(summary.sourceQuality).toBe('available');
    expect(summary.platformState).toBe('child-device-query-store | available');
    expect(summary.readModelRows).toBe('1 | 1 | 0 | 1');
    expect(summary.localEndpoint).toBe('127.0.0.1 | 4242');
    expect(summary.remoteEndpoint).toBe('203.0.113.10 | 443');
    expect(summary.domainEvidenceRef).toBe(NetworkEvidenceDrawerProof.expected.domainEvidenceRef);
    expect(summary.processRef).toBe(NetworkEvidenceDrawerProof.expected.processRef);
    expect(summary.evidenceReferences).toBe(
      `${NetworkEvidenceDrawerProof.evidenceId} | ${NetworkEvidenceDrawerProof.journalEvidenceId}`
    );
    expect(summary.exactUrlClaim).toBe('Not reported');
    expect(summary.aiAuditRef).toBe('Not reported');
    expect(summary.policyDecisionRef).toBe('Not reported');
    expect(summary.interventionResultRef).toBe('Not reported');
    expect(summary.retentionState).toBe('0 | 1');
    expect(summary.deletedEvidenceReferences).toBe('Not reported');
    expect(summary.degradedState).toBe('available | domain-observed | process-attributed');
  });

  it('mounts the network evidence drawer on canonical network product routes only', () => {
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Activity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.NetworkActivity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Commands)).toBe(false);
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
  expect(row.destinationDomain).toBe(NetworkEvidenceDrawerProof.fields.destinationDomain);
  expect(row.destinationEndpoint.port).toBe(NetworkEvidenceDrawerProof.fields.destinationPort);
  expect(row.processName).toBe(NetworkEvidenceDrawerProof.fields.processName);
  expect(row.evidence.map((evidence) => evidence.evidenceId)).toEqual([
    NetworkEvidenceDrawerProof.evidenceId,
    NetworkEvidenceDrawerProof.journalEvidenceId,
  ]);
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
      latestEventId: NetworkEvidenceDrawerProof.eventId,
      latestObservedAt: '2026-05-21T02:00:00Z',
      latestTombstoneEventId: null,
      latestTombstoneObservedAt: null,
      deletedEvidenceReferenceIds: '',
      observer: NetworkEvidenceDrawerProof.observer,
      adapterId: NetworkEvidenceDrawerProof.fields.adapterId,
      networkProtocol: NetworkEvidenceDrawerProof.fields.networkProtocol,
      tcpState: NetworkEvidenceDrawerProof.fields.tcpState,
      localIp: NetworkEvidenceDrawerProof.fields.localIp,
      localPort: NetworkEvidenceDrawerProof.fields.localPort,
      destinationIp: NetworkEvidenceDrawerProof.fields.destinationIp,
      destinationPort: NetworkEvidenceDrawerProof.fields.destinationPort,
      destinationDomain: NetworkEvidenceDrawerProof.fields.destinationDomain,
      domainAttributionStatus: NetworkEvidenceDrawerProof.fields.domainAttributionStatus,
      processAttributionStatus: NetworkEvidenceDrawerProof.fields.processAttributionStatus,
      processId: NetworkEvidenceDrawerProof.fields.pid,
      processName: NetworkEvidenceDrawerProof.fields.processName,
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

function blankOptionalWindowsNetworkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...networkFlowEvent(),
    eventId: 'evt-network-blank-windows-fields',
    correlationId: 'cmd-network-blank-windows-fields',
    payload: {
      ...networkFlowEvent().payload,
      networkProtocol: '',
      tcpState: '',
      localIp: '',
      destinationIp: '',
      destinationPort: '',
      destinationDomain: '',
      processId: '',
      processName: '',
      bytesSent: '',
      bytesReceived: '',
      firstSeenAt: '',
      lastSeenAt: '',
    },
  });
}

function stringifiedWindowsNetworkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...networkFlowEvent(),
    eventId: 'evt-network-stringified-windows-fields',
    correlationId: 'cmd-network-stringified-windows-fields',
    payload: {
      ...networkFlowEvent().payload,
      limit: '10',
      returned: '1',
      activeRows: '1',
      tombstoneRows: '0',
      exportableRows: '1',
      localPort: String(NetworkEvidenceDrawerProof.fields.localPort),
      destinationPort: String(NetworkEvidenceDrawerProof.fields.destinationPort),
      processId: String(NetworkEvidenceDrawerProof.fields.pid),
      connectionCount: '1',
    },
  });
}

function aggregateWindowsNetworkFlowEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...networkFlowEvent(),
    eventId: 'evt-network-aggregate-windows-fields',
    correlationId: 'cmd-network-aggregate-windows-fields',
    payload: {
      ...networkFlowEvent().payload,
      returned: 10,
      activeRows: 10,
      exportableRows: 10,
    },
  });
}

function networkFlowDigest() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-05-21T02:00:01Z',
    custody: 'child-device-query-store',
    evidence: [
      {
        evidenceId: NetworkEvidenceDrawerProof.evidenceId,
        kind: 'local-db-row',
        digest: NetworkEvidenceDrawerProof.evidenceDigest,
        uri: null,
      },
      {
        evidenceId: NetworkEvidenceDrawerProof.journalEvidenceId,
        kind: 'journal-entry',
        digest: NetworkEvidenceDrawerProof.journalEvidenceDigest,
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
