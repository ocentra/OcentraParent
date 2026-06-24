import { describe, expect, it } from 'vitest';
import type { ActivityNetworkFlowReadModel } from '@ocentra-parent/schema-domain/network-flow';
import { PortalRoute } from '@ocentra-parent/schema-domain/portal-contracts';
import { networkEvidenceDrawerSummary } from '@ocentra-parent/portal-domain/network-evidence-drawer';
import { AgentEvent, AgentEventEnvelopeSchema } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import {
  AgentNetworkRuntimeEventSchemaVersion,
  AgentNetworkRuntimeEventType,
} from '@ocentra-parent/schema-domain/network-runtime-events';
import {
  projectLanDiscoverySourceMatrixViewModel,
  shouldRenderNetworkEvidenceDrawerRoute,
} from '../src/NetworkEvidenceDrawerRoutePanel';
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
    if (row === undefined) {
      throw new Error('blank optional Windows network flow row missing');
    }
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
  it('projects the parent network evidence drawer with truthful surfaced refs and unsupported gaps', () => {
    const state = resolveLiveActivityState([
      networkFlowEventWithProductPathRefs(),
      policyPreviewEvent(),
      networkRuntimeEventChainEvent(),
    ]);
    const summary = networkEvidenceDrawerSummary(
      state.networkFlowReadModel,
      networkEvidenceDrawerSummaryContext(state)
    );

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
    expect(summary.analyzerAlertRef).toBe('event.network.analyzer.alert.1');
    expect(summary.detectionResultRef).toBe('event.network.detection.result.1');
    expect(summary.aiAuditRef).toBe('event.ai.analysis.completed.1');
    expect(summary.policyDecisionRef).toBe('event.policy.decision.completed.1');
    expect(summary.interventionResultRef).toBe('event.enforcement.result.observed.1');
    expect(summary.riskBudgetRef).toBe('event.network.risk-budget.1');
    expect(summary.evidenceGrade).toBe('A');
    expect(summary.retentionState).toBe('0 | 1');
    expect(summary.deletedEvidenceReferences).toBe('Not reported');
    expect(summary.degradedState).toBe('available | domain-observed | process-attributed');
  });

  it('falls back to policy preview refs when no runtime refs were streamed', () => {
    const state = resolveLiveActivityState([networkFlowEvent(), policyPreviewEvent()]);
    const summary = networkEvidenceDrawerSummary(
      state.networkFlowReadModel,
      networkEvidenceDrawerSummaryContext(state)
    );

    expect(summary.analyzerAlertRef).toBe('Not reported');
    expect(summary.detectionResultRef).toBe('Not reported');
    expect(summary.aiAuditRef).toBe('local-ai-result.network.preview.1');
    expect(summary.policyDecisionRef).toBe('policy-decision.network.preview.1');
    expect(summary.interventionResultRef).toBe('Not reported');
    expect(summary.evidenceGrade).toBe('A');
    expect(summary.riskBudgetRef).toBe('Not reported');
  });

  it('mounts the network evidence drawer on canonical network product routes only', () => {
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Activity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.NetworkActivity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Commands)).toBe(false);
    expect(shouldRenderNetworkEvidenceDrawerRoute(PortalRoute.Overview)).toBe(false);
  });

  it('projects LAN discovery source-matrix rows, claims, and restart-persisted weak identity truth', () => {
    const projection = projectLanDiscoverySourceMatrixViewModel(lanSourceMatrixReadModel());

    expect(projection).not.toBeNull();
    expect(projection?.generatedAt).toBe('2026-06-23T00:00:00Z');
    expect(projection?.rowSummary).toBe('2 workpacks | 3 sources');
    expect(projection?.statusSummary).toBe('2 implemented | 1 partial | 0 manual required | 0 not implemented');
    expect(projection?.currentSourceSummary).toBe(
      'local-service | windows-neighbor-table | previous-scan-snapshot'
    );
    expect(projection?.persistedSourceSummary).toBe('previous-scan-snapshot (WP 15)');
    expect(projection?.claimsProved).toBe('read-model-source-matrix | weak-sources-fenced');
    expect(projection?.claimsNotProved).toBe('packet-mode-not-implemented | physical-proof-manual-required');
    expect(projection?.implementedSourceRows).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          label: 'previous-scan-snapshot (WP 15)',
          value: expect.stringContaining('restart-persisted'),
        }),
      ])
    );
    expect(projection?.weakSourceRows).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          label: 'previous-scan-snapshot (WP 15)',
          value: expect.stringContaining('weak-identity'),
        }),
        expect.objectContaining({
          label: 'previous-scan-snapshot (WP 15)',
          value: expect.stringContaining('no-route-control'),
        }),
      ])
    );
  });

  it('keeps LAN source-matrix projection absent when no add-device source matrix was reported', () => {
    expect(
      projectLanDiscoverySourceMatrixViewModel({
        scanSummary: {
          sourceLabels: [],
        },
        lanDiscoverySourceMatrix: null,
      })
    ).toBeNull();
    expect(projectLanDiscoverySourceMatrixViewModel(null)).toBeNull();
  });
}

function requireNetworkFlowReadModel(readModel: ActivityNetworkFlowReadModel | null): ActivityNetworkFlowReadModel {
  expect(readModel).not.toBeNull();
  if (readModel === null) {
    throw new Error('network flow read-model missing');
  }
  return readModel;
}

function networkEvidenceDrawerSummaryContext(state: ReturnType<typeof resolveLiveActivityState>) {
  return {
    networkFlowEventPayload: state.networkFlowEvent?.payload ?? null,
    policyPreviewReadModel: state.policyPreviewReadModel,
    networkRuntimeEventChainStream: state.networkRuntimeEventChainStream,
  };
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

function networkFlowEventWithProductPathRefs() {
  return AgentEventEnvelopeSchema.parse({
    ...networkFlowEvent(),
    eventId: 'evt-network-product-path-refs',
    correlationId: 'cmd-network-product-path-refs',
    payload: {
      ...networkFlowEvent().payload,
      [AgentProtocolDefaults.Field.NetworkProductPathAnalyzerAlertRefs]: 'event.network.analyzer.alert.1',
      [AgentProtocolDefaults.Field.NetworkProductPathAiDetectionRefs]: 'event.network.detection.result.1',
      [AgentProtocolDefaults.Field.NetworkProductPathRiskBudgetRefs]: 'event.network.risk-budget.1',
    },
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

function policyPreviewEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-policy-preview-network',
    correlationId: 'cmd-policy-preview-network',
    sentAt: '2026-05-21T02:00:03Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.PolicyPreviewReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.SchemaVersion]: 1,
      [AgentProtocolDefaults.Field.GeneratedAt]: '2026-05-21T02:00:03Z',
      [AgentProtocolDefaults.Field.Custody]: 'child-device-query-store',
      [AgentProtocolDefaults.Field.Limit]: 10,
      [AgentProtocolDefaults.Field.Returned]: 1,
      [AgentProtocolDefaults.Field.CapabilityStatus]: 'preview-ready',
      [AgentProtocolDefaults.Field.PolicyPreviewId]: 'policy-preview.network.1',
      [AgentProtocolDefaults.Field.LatestEventId]: 'policy-preview.network.event.1',
      [AgentProtocolDefaults.Field.LatestObservedAt]: '2026-05-21T02:00:02Z',
      [AgentProtocolDefaults.Field.PolicyTargetType]: AgentProtocolDefaults.PolicyPreview.TargetType.NetworkDomain,
      [AgentProtocolDefaults.Field.PolicyTargetValue]: NetworkEvidenceDrawerProof.fields.destinationDomain,
      [AgentProtocolDefaults.Field.PolicyDecisionId]: 'policy-decision.network.preview.1',
      [AgentProtocolDefaults.Field.PolicyAction]: AgentProtocolDefaults.PolicyPreview.Action.Block,
      [AgentProtocolDefaults.Field.LocalAiResultId]: 'local-ai-result.network.preview.1',
      [AgentProtocolDefaults.Field.PolicyDryRun]: true,
      [AgentProtocolDefaults.Field.PolicyHandoffState]:
        AgentProtocolDefaults.PolicyPreview.HandoffState.DisabledPreviewOnly,
      [AgentProtocolDefaults.Field.NetworkEvidenceGrade]: AgentProtocolDefaults.PolicyPreview.EvidenceGrade.A,
      [AgentProtocolDefaults.Field.NetworkRequestedPolicyAction]: AgentProtocolDefaults.PolicyPreview.Action.Block,
      [AgentProtocolDefaults.Field.NetworkMappedPolicyAction]: AgentProtocolDefaults.PolicyPreview.Action.Block,
      [AgentProtocolDefaults.Field.NetworkPolicyMappingMode]: AgentProtocolDefaults.PolicyPreview.MappingMode.DryRun,
      [AgentProtocolDefaults.Field.NetworkAdapterActionAuthorized]: false,
      [AgentProtocolDefaults.Field.NetworkEnforcementCommandAuthorized]: false,
    },
    snapshot: null,
  });
}

function networkRuntimeEventChainEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network-runtime-stream',
    correlationId: 'cmd-network-runtime-stream',
    sentAt: '2026-05-21T02:00:04Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.NetworkRuntimeEventChainStreamReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.NetworkRuntimeStreamedEvents]: 3,
      [AgentProtocolDefaults.Field.NetworkRuntimeEventChainStream]: JSON.stringify([
        {
          eventType: AgentNetworkRuntimeEventType.AiAnalysisCompleted,
          payload: {
            schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
            aiAnalysisRef: 'event.ai.analysis.completed.1',
            aiRequestRef: 'event.ai.requested.1',
            previousEventRef: 'event.ai.requested.1',
            advisoryState: 'completed',
            evidenceRefs: [NetworkEvidenceDrawerProof.evidenceId],
            unsupportedClaims: ['decrypted-https-payload'],
          },
        },
        {
          eventType: AgentNetworkRuntimeEventType.PolicyDecisionCompleted,
          payload: {
            schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
            policyDecisionRef: 'event.policy.decision.completed.1',
            policyEvaluationRef: 'event.policy.evaluation.requested.1',
            previousEventRef: 'event.policy.evaluation.requested.1',
            decisionAction: 'block',
            evidenceRefs: [NetworkEvidenceDrawerProof.evidenceId],
            parentRuleRefs: ['policy.rule.network-domain.1'],
            adapterCapabilityRequired: true,
          },
        },
        {
          eventType: AgentNetworkRuntimeEventType.EnforcementResultObserved,
          payload: {
            schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
            enforcementResultRef: 'event.enforcement.result.observed.1',
            enforcementCommandRef: 'event.enforcement.command.issued.1',
            previousEventRef: 'event.enforcement.command.issued.1',
            resultStatus: 'dry-run',
            adapterActionExecuted: false,
            rollbackRef: 'rollback.network.command.1',
            unavailableReasonCode: null,
          },
        },
      ]),
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

function lanSourceMatrixReadModel() {
  return {
    scanSummary: {
      sourceLabels: ['local-service', 'windows-neighbor-table', 'previous-scan-snapshot'],
    },
    lanDiscoverySourceMatrix: {
      generatedAt: '2026-06-23T00:00:00Z',
      workpackRows: [
        {
          workpackId: '04',
          title: 'Passive neighbor discovery',
          discoveryState: 'discovered',
          proofState: 'ci-mechanical-proof',
          runtimeOwner: 'rust-service-read-model',
          status: 'implemented',
          readModelVisible: true,
          requiredArtifactSummary: null,
        },
        {
          workpackId: '15',
          title: 'Household device store',
          discoveryState: 'pending',
          proofState: 'ci-mechanical-proof',
          runtimeOwner: 'rust-service-read-model',
          status: 'partial',
          readModelVisible: true,
          requiredArtifactSummary: null,
        },
      ],
      sourceRows: [
        {
          source: 'windows-neighbor-table',
          workpackId: '04',
          status: 'implemented',
          authority: 'weak-identity',
          runtimePath: 'rust-service-read-model',
          uiSurface: 'devices-lan',
          canConfirmChildAgent: false,
          canAssignChildProfile: false,
          canControlRoute: false,
          requiresSelectedInterface: true,
          persistsAcrossRestart: false,
          evidenceLabel: 'Passive neighbor table row',
          requiredArtifactSummary: null,
        },
        {
          source: 'previous-scan-snapshot',
          workpackId: '15',
          status: 'implemented',
          authority: 'weak-identity',
          runtimePath: 'rust-service-read-model',
          uiSurface: 'devices-lan',
          canConfirmChildAgent: false,
          canAssignChildProfile: false,
          canControlRoute: false,
          requiresSelectedInterface: false,
          persistsAcrossRestart: true,
          evidenceLabel: 'Previous scan continuity hint',
          requiredArtifactSummary: null,
        },
        {
          source: 'service-identity-probe',
          workpackId: '11',
          status: 'partial',
          authority: 'presence-only',
          runtimePath: 'rust-service-read-model',
          uiSurface: 'devices-lan',
          canConfirmChildAgent: false,
          canAssignChildProfile: false,
          canControlRoute: false,
          requiresSelectedInterface: false,
          persistsAcrossRestart: false,
          evidenceLabel: 'Service identity probe presence only',
          requiredArtifactSummary: null,
        },
      ],
      claimsProved: ['read-model-source-matrix', 'weak-sources-fenced'],
      claimsNotProved: ['packet-mode-not-implemented', 'physical-proof-manual-required'],
    },
  } as const;
}
