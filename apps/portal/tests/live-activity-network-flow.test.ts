import { describe, expect, it } from 'vitest';
import type { ActivityNetworkFlowReadModel } from '@ocentra-parent/activity-domain/network-flow';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import { shouldRenderNetworkEvidenceDrawerRoute } from '../src/NetworkEvidenceDrawerRoutePanel';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { networkEvidenceDrawerSummary } from '../src/network-evidence-drawer';
import type { NetworkRuntimeEventChainSummary } from '../src/network-runtime-event-chain';

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
    expect(summary.sourceAdapter).toBe('windows-network-snapshot');
    expect(summary.sourceQuality).toBe('available');
    expect(summary.localEndpoint).toBe('127.0.0.1 | 4242');
    expect(summary.remoteEndpoint).toBe('203.0.113.10 | 443');
    expect(summary.domainEvidenceRef).toBe('example-network.test | domain-observed');
    expect(summary.processRef).toBe('notepad.exe | 4242 | process-attributed');
    expect(summary.evidenceReferences).toBe('network-evidence-1 | network-journal-1');
    expect(summary.exactUrlClaim).toBe('Not reported');
    expect(summary.aiAuditRef).toBe('Not reported');
    expect(summary.policyDecisionRef).toBe('Not reported');
    expect(summary.interventionResultRef).toBe('Not reported');
    expect(summary.retentionState).toBe('Not reported');
  });

  it('projects service runtime event-chain refs into the parent network drawer', () => {
    const state = resolveLiveActivityState([networkFlowEvent(), networkRuntimeEventChainEvent()]);
    const eventChain = requireNetworkRuntimeEventChain(state.networkRuntimeEventChain);
    const summary = networkEvidenceDrawerSummary(state.networkFlowReadModel, eventChain);

    expect(summary.eventHistoryRef).toContain('event.ai.analysis.completed.1');
    expect(summary.eventHistoryRef).toContain('event.portal.read-model.updated.1');
    expect(summary.aiAuditRef).toBe('event.ai.analysis.completed.1 | manual-review-required');
    expect(summary.policyDecisionRef).toBe('event.policy.decision.completed.1 | manual-review');
    expect(summary.interventionResultRef).toBe(
      'event.enforcement.result.observed.1 | manual-required | manual-required'
    );
    expect(summary.auditRef).toBe('event.audit.entry.committed.1 | committed');
    expect(summary.retentionState).toBe('1 | 0 | 1 | network-evidence-deleted-1');
    expect(summary.evidenceGrade).toBe('C');
    expect(summary.confidence).toBe('0.5');
    expect(summary.manualRequiredState).toBe('manual-required-state');
    expect(summary.unavailableState).toBe('manual-required');
    expect(summary.exactUrlClaim).toBe('Not reported');
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

function requireNetworkRuntimeEventChain(
  eventChain: NetworkRuntimeEventChainSummary | null
): NetworkRuntimeEventChainSummary {
  expect(eventChain).not.toBeNull();
  if (eventChain === null) {
    throw new Error('network runtime event-chain missing');
  }
  return eventChain;
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

function networkRuntimeEventChainEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network-runtime-chain',
    correlationId: 'cmd-network-runtime-chain',
    sentAt: '2026-05-21T02:00:02Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.network.runtime.event-chain.stream.reported',
    severity: 'info',
    payload: {
      networkRuntimeObservedRows: 1,
      networkRuntimeStreamedEvents: 6,
      networkRuntimeFailedRows: 0,
      networkRuntimeManualRequiredRows: 1,
      networkRuntimeEnforcementCommandEvents: 0,
      activeRows: 1,
      tombstoneRows: 0,
      exportableRows: 1,
      deletedEvidenceReferenceIds: 'network-evidence-deleted-1',
      networkRuntimeEventChainStream: JSON.stringify(networkRuntimeEventChainEntries()),
    },
    snapshot: null,
  });
}

function networkRuntimeEventChainEntries() {
  return [
    networkClassifiedEntry(),
    aiAnalysisCompletedEntry(),
    policyDecisionCompletedEntry(),
    enforcementResultObservedEntry(),
    auditEntryCommittedEntry(),
    portalReadModelUpdatedEntry(),
  ];
}

function networkClassifiedEntry() {
  return {
    eventType: 'network.activity.classified',
    eventRef: 'event.network.activity.classified.1',
    payload: {
      schemaVersion: 1,
      classificationEventRef: 'event.network.activity.classified.1',
      previousEventRef: 'event.network.domain.observed.1',
      evidenceRefs: ['network-evidence-1'],
      activityKind: 'unknown',
      confidence: 0.5,
      evidenceGrade: 'C',
      uncertaintyCodes: ['ip-only'],
    },
  };
}

function aiAnalysisCompletedEntry() {
  return {
    eventType: 'ai.analysis.completed',
    eventRef: 'event.ai.analysis.completed.1',
    payload: {
      schemaVersion: 1,
      aiAnalysisRef: 'event.ai.analysis.completed.1',
      aiRequestRef: 'event.ai.analysis.requested.1',
      previousEventRef: 'event.ai.analysis.requested.1',
      advisoryState: 'manual-review-required',
      evidenceRefs: ['network-evidence-1'],
      unsupportedClaims: ['decrypted-https-payload'],
    },
  };
}

function policyDecisionCompletedEntry() {
  return {
    eventType: 'policy.decision.completed',
    eventRef: 'event.policy.decision.completed.1',
    payload: {
      schemaVersion: 1,
      policyDecisionRef: 'event.policy.decision.completed.1',
      policyEvaluationRef: 'event.policy.evaluation.requested.1',
      previousEventRef: 'event.policy.evaluation.requested.1',
      decisionAction: 'manual-review',
      evidenceRefs: ['network-evidence-1'],
      parentRuleRefs: ['parent-rule.network.review.1'],
      adapterCapabilityRequired: false,
    },
  };
}

function enforcementResultObservedEntry() {
  return {
    eventType: 'enforcement.result.observed',
    eventRef: 'event.enforcement.result.observed.1',
    payload: {
      schemaVersion: 1,
      enforcementResultRef: 'event.enforcement.result.observed.1',
      enforcementCommandRef: 'event.enforcement.command.issued.1',
      previousEventRef: 'event.policy.decision.completed.1',
      resultStatus: 'manual-required',
      adapterActionExecuted: false,
      rollbackRef: null,
      unavailableReasonCode: 'manual-required',
    },
  };
}

function auditEntryCommittedEntry() {
  return {
    eventType: 'audit.entry.committed',
    eventRef: 'event.audit.entry.committed.1',
    payload: {
      schemaVersion: 1,
      auditEntryRef: 'event.audit.entry.committed.1',
      previousEventRef: 'event.enforcement.result.observed.1',
      policyDecisionRef: 'event.policy.decision.completed.1',
      enforcementCommandRef: null,
      enforcementResultRef: 'event.enforcement.result.observed.1',
      evidenceRefs: ['network-evidence-1'],
      auditOutcome: 'committed',
    },
  };
}

function portalReadModelUpdatedEntry() {
  return {
    eventType: 'portal.read_model.updated',
    eventRef: 'event.portal.read-model.updated.1',
    payload: {
      schemaVersion: 1,
      readModelRef: 'event.portal.read-model.updated.1',
      previousEventRef: 'event.audit.entry.committed.1',
      auditEntryRef: 'event.audit.entry.committed.1',
      updateKind: 'manual-required-state',
      visibleManualRequired: true,
      visibleUnavailable: false,
    },
  };
}
