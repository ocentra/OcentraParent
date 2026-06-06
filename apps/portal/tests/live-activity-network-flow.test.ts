import { describe, expect, it } from 'vitest';
import type { ActivityNetworkFlowReadModel } from '@ocentra-parent/activity-domain/network-flow';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { shouldRenderNetworkEvidenceDrawerRoute } from '../src/NetworkEvidenceDrawerRoutePanel';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { networkEvidenceDrawerSummary } from '../src/network-evidence-drawer';
import type { NetworkProductReadinessStatusSummary } from '../src/network-product-readiness-status';
import type { NetworkRuntimeEventChainSummary } from '../src/network-runtime-event-chain';

describe('portal live activity network flow state', () => {
  defineNetworkFlowReadModelTests();
  defineNetworkRuntimeEventChainTests();
  defineNetworkProductReadinessStatusTests();
  defineNetworkRouteMountTests();
});

function defineNetworkFlowReadModelTests(): void {
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
}

function defineNetworkRuntimeEventChainTests(): void {
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
}

function defineNetworkProductReadinessStatusTests(): void {
  it('projects service product-readiness status into the parent network drawer', () => {
    const state = resolveLiveActivityState([networkProductReadinessStatusEvent()]);
    const summary = requireNetworkProductReadinessStatus(state.networkProductReadinessStatus);

    expect(summary.parserStatus).toBe('true');
    expect(summary.custodyStatusRef).toBe('network.live-capture.custody-status.13a');
    expect(summary.custodyState).toBe('CustodyReady');
    expect(summary.liveCaptureState).toBe('ProofReady');
    expect(summary.rawCaptureStorageState).toBe('CustodyReady');
    expect(summary.captureReady).toBe('true');
    expect(summary.rawArtifactStorageAuthorized).toBe('true');
    expect(summary.missingArtifactCount).toBe('0');
    expect(summary.readinessStatusRef).toBe('network.product-readiness.status.51a');
    expect(summary.readinessState).toBe('ManualRequired');
    expect(summary.riskBudgetState).toBe('AskParentThreshold');
    expect(summary.riskInterventionState).toBe('AskParent');
    expect(summary.performanceState).toBe('MeetsBenchmarkGate');
    expect(summary.performancePathStates).toBe('DryRun');
    expect(summary.platformReadyClaims).toBe('1');
    expect(summary.platformDryRunClaims).toBe('1');
    expect(summary.platformResearchOnlyClaims).toBe('0');
    expect(summary.platformManualRequiredClaims).toBe('1');
    expect(summary.platformUnavailableClaims).toBe('1');
    expect(summary.platformManualFollowups).toBe('WindowsWfp | network.live-capture.permission-proof.13');
    expect(summary.platformEntries).toHaveLength(4);
    expect(summary.platformEntries[0]?.target).toBe('WindowsFirewall');
    expect(summary.platformEntries[0]?.adapterAuthorizedByProof).toBe('true');
    expect(summary.platformEntries[0]?.enforcementCommandPublished).toBe('false');
    expect(summary.platformEntries[1]?.state).toBe('DryRun');
    expect(summary.platformEntries[2]?.target).toBe('WindowsWfp');
    expect(summary.platformEntries[2]?.missingRequiredArtifacts).toBe('network.platform-claim.manual-followup.51a');
    expect(summary.platformEntries[3]?.state).toBe('Unavailable');
    expect(summary.portalReadModelReady).toBe('true');
    expect(summary.retentionExportRefsVisible).toBe('true');
    expect(summary.noClaimBoundary).toBe('false');
  });

  it('keeps malformed product-readiness status visible as a parser failure', () => {
    const state = resolveLiveActivityState([malformedNetworkProductReadinessStatusEvent()]);
    const summary = requireNetworkProductReadinessStatus(state.networkProductReadinessStatus);

    expect(summary.parserStatus).toBe('invalid-product-readiness-status');
    expect(summary.readinessStatusRef).toBe('Not reported');
    expect(summary.noClaimBoundary).toBe('Not reported');
  });
}

function defineNetworkRouteMountTests(): void {
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
}

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

function requireNetworkProductReadinessStatus(
  status: NetworkProductReadinessStatusSummary | null
): NetworkProductReadinessStatusSummary {
  expect(status).not.toBeNull();
  if (status === null) {
    throw new Error('network product-readiness status missing');
  }
  return status;
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
    event: AgentEvent.NetworkFlowReadModelReported,
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
    event: AgentEvent.NetworkFlowReadModelReported,
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
    event: AgentEvent.NetworkRuntimeEventChainStreamReported,
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

function networkProductReadinessStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network-product-readiness',
    correlationId: 'cmd-network-product-readiness',
    sentAt: '2026-06-06T05:50:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.NetworkProductReadinessStatusReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
    },
    snapshot: null,
  });
}

function malformedNetworkProductReadinessStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...networkProductReadinessStatusEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify({
        ...productReadinessStatus(),
        portal_adapter_dispatch_claimed: true,
      }),
    },
  });
}

function liveCaptureCustodyStatus() {
  return {
    status_ref: 'network.live-capture.custody-status.13a',
    live_capture_proof_ref: 'network.live-capture.proof.13',
    raw_capture_storage_proof_ref: 'network.raw-capture.storage.03a',
    state: 'CustodyReady',
    live_capture_state: 'ProofReady',
    raw_capture_storage_state: 'CustodyReady',
    missing_artifacts: [],
    capture_ready: true,
    raw_artifact_storage_authorized: true,
    driver_invoked: false,
    live_capture_executed: false,
    raw_artifact_created: false,
    remote_upload_enabled: false,
    raw_pcap_without_custody_available: false,
    exact_url_available: false,
    decrypted_payload_available: false,
    page_content_available: false,
    private_message_available: false,
    search_query_available: false,
    policy_authority: false,
    adapter_authority: false,
    enforcement_commands_published: 0,
  };
}

function productReadinessStatus() {
  return {
    status_ref: 'network.product-readiness.status.51a',
    portal_read_model_ref: 'network.portal-read-model.51a',
    retention_export_ref: 'network.retention-export.51a',
    readiness_state: 'ManualRequired',
    risk_budget_ref: 'network.risk-budget.51a',
    risk_budget_state: 'AskParentThreshold',
    risk_intervention_state: 'AskParent',
    risk_total_points: 42,
    risk_budget_advisory_only: true,
    performance_state: 'MeetsBenchmarkGate',
    performance_regression_codes: [],
    performance_path_states: ['DryRun'],
    platform_ready_claims: 1,
    platform_dry_run_claims: 1,
    platform_research_only_claims: 0,
    platform_manual_required_claims: 1,
    platform_unavailable_claims: 1,
    platform_manual_followups: [
      {
        target: 'WindowsWfp',
        missing_required_artifacts: ['network.live-capture.permission-proof.13'],
      },
    ],
    platform_entries: platformEntries(),
    portal_read_model_ready: true,
    retention_export_refs_visible: true,
    policy_authority: false,
    adapter_authority: false,
    ui_policy_authority: false,
    live_adapter_execution_claimed: false,
    portal_adapter_dispatch_claimed: false,
    enforcement_commands_published: 0,
    production_slo_claimed: false,
    exact_url_available: false,
    decrypted_payload_available: false,
    page_content_available: false,
  };
}

function platformEntries() {
  return [
    {
      target: 'WindowsFirewall',
      claim_state: 'Ready',
      policy_decision_ref: 'network.policy-decision.51a',
      parent_rule_ref: 'network.parent-rule.51a',
      evidence_refs: ['network.flow-evidence.51a'],
      device_or_os_refs: ['windows-device.51a'],
      permission_or_entitlement_refs: ['network.live-capture.permission-proof.13'],
      adapter_capability_refs: ['network.adapter-capability.51a'],
      missing_required_artifacts: [],
      audit_refs: ['network.audit.51a'],
      adapter_authorized_by_proof: true,
      enforcement_command_published: false,
    },
    {
      target: 'WindowsFirewall',
      claim_state: 'DryRun',
      policy_decision_ref: 'network.policy-decision.51a',
      parent_rule_ref: 'network.parent-rule.51a',
      evidence_refs: ['network.flow-evidence.51a'],
      device_or_os_refs: ['windows-device.51a'],
      permission_or_entitlement_refs: ['network.live-capture.permission-proof.13'],
      adapter_capability_refs: ['network.adapter-capability.51a'],
      missing_required_artifacts: [],
      audit_refs: ['network.audit.51a'],
      adapter_authorized_by_proof: false,
      enforcement_command_published: false,
    },
    {
      target: 'WindowsWfp',
      claim_state: 'ManualRequired',
      policy_decision_ref: 'network.policy-decision.51a',
      parent_rule_ref: 'network.parent-rule.51a',
      evidence_refs: ['network.flow-evidence.51a'],
      device_or_os_refs: ['windows-wfp-device.51a'],
      permission_or_entitlement_refs: [],
      adapter_capability_refs: ['network.wfp-capability.51a'],
      missing_required_artifacts: ['network.platform-claim.manual-followup.51a'],
      audit_refs: ['network.wfp-audit.51a'],
      adapter_authorized_by_proof: false,
      enforcement_command_published: false,
    },
    {
      target: 'AppleNetworkExtensionIos',
      claim_state: 'Unavailable',
      policy_decision_ref: 'network.policy-decision.51a',
      parent_rule_ref: 'network.parent-rule.51a',
      evidence_refs: ['network.flow-evidence.51a'],
      device_or_os_refs: ['ios-device.51a'],
      permission_or_entitlement_refs: [],
      adapter_capability_refs: [],
      missing_required_artifacts: [],
      audit_refs: ['network.ios-audit.51a'],
      adapter_authorized_by_proof: false,
      enforcement_command_published: false,
    },
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
