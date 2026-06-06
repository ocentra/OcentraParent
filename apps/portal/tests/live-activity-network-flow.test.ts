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

    expectProductReadinessCustodySummary(summary);
    expectProductReadinessRiskSummary(summary);
    expectProductReadinessPerformanceSummary(summary);
    expectProductReadinessPlatformSummary(summary);
    expectProductReadinessRemoteDeliverySummary(summary);
    expectProductReadinessLocalAiRuntimeResultSummary(summary);
    expectProductReadinessNoClaimSummary(summary);
  });

  it('keeps malformed product-readiness status visible as a parser failure', () => {
    const state = resolveLiveActivityState([malformedNetworkProductReadinessStatusEvent()]);
    const summary = requireNetworkProductReadinessStatus(state.networkProductReadinessStatus);

    expect(summary.parserStatus).toBe('invalid-product-readiness-status');
    expect(summary.readinessStatusRef).toBe('Not reported');
    expect(summary.noClaimBoundary).toBe('Not reported');
  });
}

function expectProductReadinessCustodySummary(summary: NetworkProductReadinessStatusSummary): void {
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
}

function expectProductReadinessRiskSummary(summary: NetworkProductReadinessStatusSummary): void {
  expect(summary.riskEvaluationRef).toBe('network.risk-evaluation.51a');
  expect(summary.riskAgeBand).toBe('UnderTwelve');
  expect(summary.riskBudgetState).toBe('AskParentThreshold');
  expect(summary.riskInterventionState).toBe('AskParent');
  expect(summary.riskTotalPoints).toBe('42');
  expect(summary.riskPointBreakdown).toBe('15 | 27 | 0 | 0 | 40');
  expect(summary.riskCitedSignalRefs).toBe('network.signal.51a');
  expect(summary.riskCitedAuditRefs).toBe('network.audit.51a');
  expect(summary.riskCitedEvidenceRefs).toBe('network.flow-evidence.51a');
  expect(summary.riskCitedParentRuleRefs).toBe('network.parent-rule.51a');
  expect(summary.riskAdapterProofState).toBe('Ready');
  expect(summary.riskBudgetAdvisoryOnly).toBe('true');
}

function expectProductReadinessPerformanceSummary(summary: NetworkProductReadinessStatusSummary): void {
  expect(summary.performanceBenchmarkRunRef).toBe('network.performance.51a');
  expect(summary.performanceState).toBe('MeetsBenchmarkGate');
  expect(summary.performanceRegressionCodes).toBe('Not reported');
  expect(summary.performanceScenarioCounts).toBe('2 | 20 | 2000 | 600 | 1200');
  expect(summary.performanceLatencyMetrics).toBe('80 | 700 | 90 | Not reported');
  expect(summary.performanceThroughputMetrics).toBe('3200 | 4 | 0 | 2100');
  expect(summary.performanceResourceMetrics).toBe('120 | 40000 | 20000');
  expect(summary.performanceQualityMetrics).toBe('0 | 0');
  expect(summary.performancePathStates).toBe('DryRun');
  expect(summary.performanceProductionSloClaimed).toBe('false');
  expect(summary.performanceAdapterExecutionClaimed).toBe('false');
  expect(summary.performanceHostFilteringClaimed).toBe('false');
}

function expectProductReadinessPlatformSummary(summary: NetworkProductReadinessStatusSummary): void {
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
}

function expectProductReadinessRemoteDeliverySummary(summary: NetworkProductReadinessStatusSummary): void {
  expect(summary.remoteDeliveryStatusRef).toBe('network.remote-delivery.status.10c');
  expect(summary.remoteBrokerStatus).toBe('RequirementsSatisfiedButNotImplemented');
  expect(summary.remoteFamilyHubStatus).toBe('RequirementsSatisfiedButNotImplemented');
  expect(summary.remoteCustodyProofRef).toBe('broker.network.custody-proof.1');
  expect(summary.remoteAuthRefs).toBe('broker.network.publisher-auth.1 | broker.network.subscriber-auth.1');
  expect(summary.remoteTransportRefs).toBe(
    'broker.network.encryption.1 | broker.network.config.1 | family-hub.network.identity.1 | family-hub.network.relay-policy.1'
  );
  expect(summary.remoteLifecycleRefs).toBe(
    'broker.network.retention-policy.1 | broker.network.replay-plan.1 | broker.network.deletion-plan.1 | broker.network.offset-policy.1 | broker.network.dedupe-policy.1 | broker.network.cross-process-replay.manual-required.10d | broker.network.remote-retention-delete-export.manual-required.10d | family-hub.network.delivery-ack.manual-required.10d'
  );
  expect(summary.remoteMissingArtifactCounts).toBe('0 | 0 | 3');
  expect(summary.remoteAcceptedEventTypeCount).toBe('3');
  expect(summary.remoteLocalQueueProof).toBe('true');
  expect(summary.remoteDuplicateProof).toBe('true | true');
  expect(summary.remoteDeadLetterCount).toBe('1');
  expect(summary.remoteLifecycleBlockerRefs).toBe(
    'broker.network.cross-process-replay.manual-required.10d | broker.network.remote-retention-delete-export.manual-required.10d | family-hub.network.delivery-ack.manual-required.10d'
  );
  expect(summary.remoteLifecycleFollowupRef).toBe('network.remote-delivery.lifecycle-followup.10d');
  expect(summary.remoteLifecycleMissingArtifactCount).toBe('3');
  expect(summary.remoteLifecycleManualRequired).toBe('true');
  expect(summary.remoteDurableEnvelopeRefs).toBe(
    'broker.network.durable-envelope.schema.10e | broker.network.durable-envelope.journal-readiness.10e | broker.network.durable-envelope.replay-readiness.10e | broker.network.durable-envelope.delete-export-readiness.10e | network.remote-delivery.durable-envelope.support-status.10e'
  );
  expect(summary.remoteDurableEnvelopeReady).toBe('true');
  expect(summary.remoteDurableEnvelopeMissingArtifactCount).toBe('0');
  expect(summary.remoteExternalTransportImplemented).toBe('false');
  expect(summary.remoteFamilyHubDeliveryImplemented).toBe('false');
  expect(summary.remoteCrossProcessReplayImplemented).toBe('false');
  expect(summary.remoteRetentionDeleteExportImplemented).toBe('false');
  expect(summary.remoteProviderDeliveryImplemented).toBe('false');
  expect(summary.remoteChildDeviceDeliveryImplemented).toBe('false');
  expect(summary.remoteProductReadyClaimed).toBe('false');
  expect(summary.remotePolicyAuthority).toBe('false');
  expect(summary.remoteSideEffectAuthority).toBe('false');
  expect(summary.remoteEnforcementCommandEventCount).toBe('0');
  expect(summary.remoteAdapterActionExecutedCount).toBe('0');
}

function expectProductReadinessLocalAiRuntimeResultSummary(summary: NetworkProductReadinessStatusSummary): void {
  expect(summary.localAiRuntimeResultStatusRef).toBe('network.local-ai.runtime-result.status.33b');
  expect(summary.localAiBridgeState).toBe('ResultReady');
  expect(summary.localAiQueueStatus).toBe('Queued');
  expect(summary.localAiTriggerRef).toBe('network.local-ai.trigger.33b');
  expect(summary.localAiQueueRefs).toBe('network.local-ai.queue-job.33b | network.local-ai.queue.33b');
  expect(summary.localAiRuntimeRefs).toBe('network.local-ai.model-runtime.33b | network.local-ai.runtime-ref.33b');
  expect(summary.localAiModelRefs).toBe('network.local-ai.model.33b | network.local-ai.model-version.33b');
  expect(summary.localAiPromptPolicyRefs).toBe(
    'network.local-ai.prompt-template.33b | network.local-ai.policy-context.33b'
  );
  expect(summary.localAiParentRuleRefs).toBe('policy.rule.network-domain.1');
  expect(summary.localAiEvidenceRefs).toBe('network.local-ai.managed-browser-exact-url-evidence.33b');
  expect(summary.localAiSummaryRefs).toBe('network.local-ai.network-summary.33b | network.local-ai.screen-summary.33b');
  expect(summary.localAiManagedBrowserExactUrlEvidenceRefs).toBe(
    'network.local-ai.managed-browser-exact-url-evidence.33b'
  );
  expect(summary.localAiResultRef).toBe('network.local-ai.result.33b');
  expect(summary.localAiOutputSummaryRef).toBe('network.local-ai.output-summary.33b');
  expect(summary.localAiRuntimeResultObserved).toBe('true');
  expect(summary.localAiAuditInputReady).toBe('true');
  expect(summary.localAiModelOutputAvailable).toBe('true');
  expect(summary.localAiModelExecutionProved).toBe('false');
  expect(summary.localAiRawPcapAvailable).toBe('false');
  expect(summary.localAiExactUrlClaimed).toBe('false');
  expect(summary.localAiDecryptedPayloadAvailable).toBe('false');
  expect(summary.localAiPageContentAvailable).toBe('false');
  expect(summary.localAiPrivateMessageAvailable).toBe('false');
  expect(summary.localAiSearchQueryAvailable).toBe('false');
  expect(summary.localAiRemoteAiUsed).toBe('false');
  expect(summary.localAiPolicyAuthority).toBe('false');
  expect(summary.localAiAdapterAuthority).toBe('false');
  expect(summary.localAiEnforcementCommandsPublished).toBe('0');
}

function expectProductReadinessNoClaimSummary(summary: NetworkProductReadinessStatusSummary): void {
  expect(summary.portalReadModelReady).toBe('true');
  expect(summary.retentionExportRefsVisible).toBe('true');
  expect(summary.noClaimBoundary).toBe('false');
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
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
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
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
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
    ...riskDetails(),
    ...performanceDetails(),
    ...platformDetails(),
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

function localAiRuntimeResultStatus() {
  return {
    status_ref: 'network.local-ai.runtime-result.status.33b',
    bridge_state: 'ResultReady',
    queue_status: 'Queued',
    trigger_ref: 'network.local-ai.trigger.33b',
    queue_job_ref: 'network.local-ai.queue-job.33b',
    queue_ref: 'network.local-ai.queue.33b',
    model_runtime_ref: 'network.local-ai.model-runtime.33b',
    local_ai_result_ref: 'network.local-ai.result.33b',
    runtime_reference_id: 'network.local-ai.runtime-ref.33b',
    model_reference: 'network.local-ai.model.33b',
    model_version_ref: 'network.local-ai.model-version.33b',
    prompt_template_ref: 'network.local-ai.prompt-template.33b',
    policy_context_ref: 'network.local-ai.policy-context.33b',
    parent_rule_refs: ['policy.rule.network-domain.1'],
    evidence_refs: ['network.local-ai.managed-browser-exact-url-evidence.33b'],
    summary_refs: ['network.local-ai.network-summary.33b', 'network.local-ai.screen-summary.33b'],
    managed_browser_exact_url_evidence_refs: ['network.local-ai.managed-browser-exact-url-evidence.33b'],
    output_summary_ref: 'network.local-ai.output-summary.33b',
    local_runtime_result_observed: true,
    audit_input_ready: true,
    local_model_output_available: true,
    model_execution_proved: false,
    raw_pcap_available: false,
    exact_url_claimed: false,
    decrypted_payload_available: false,
    page_content_available: false,
    private_message_available: false,
    search_query_available: false,
    remote_ai_used: false,
    policy_authority: false,
    adapter_authority: false,
    enforcement_commands_published: 0,
  };
}

function remoteDeliveryStatus() {
  return {
    status_ref: 'network.remote-delivery.status.10c',
    broker_status: 'RequirementsSatisfiedButNotImplemented',
    family_hub_status: 'RequirementsSatisfiedButNotImplemented',
    custody_proof_ref: 'broker.network.custody-proof.1',
    publisher_auth_ref: 'broker.network.publisher-auth.1',
    subscriber_auth_ref: 'broker.network.subscriber-auth.1',
    encryption_ref: 'broker.network.encryption.1',
    retention_policy_ref: 'broker.network.retention-policy.1',
    replay_plan_ref: 'broker.network.replay-plan.1',
    deletion_plan_ref: 'broker.network.deletion-plan.1',
    offset_policy_ref: 'broker.network.offset-policy.1',
    dedupe_policy_ref: 'broker.network.dedupe-policy.1',
    transport_config_ref: 'broker.network.config.1',
    relay_identity_ref: 'family-hub.network.identity.1',
    relay_policy_ref: 'family-hub.network.relay-policy.1',
    broker_missing_artifact_count: 0,
    family_hub_missing_artifact_count: 0,
    accepted_event_type_count: 3,
    local_idempotency_queue_proved: true,
    dropped_event_dead_letter_count: 1,
    queued_duplicate_rejected: true,
    completed_duplicate_rejected: true,
    cross_process_replay_ref: 'broker.network.cross-process-replay.manual-required.10d',
    remote_retention_delete_export_ref: 'broker.network.remote-retention-delete-export.manual-required.10d',
    remote_delivery_ack_ref: 'family-hub.network.delivery-ack.manual-required.10d',
    remote_lifecycle_followup_ref: 'network.remote-delivery.lifecycle-followup.10d',
    remote_lifecycle_missing_artifact_count: 3,
    remote_lifecycle_manual_required: true,
    durable_envelope_schema_ref: 'broker.network.durable-envelope.schema.10e',
    durable_envelope_journal_ref: 'broker.network.durable-envelope.journal-readiness.10e',
    durable_envelope_replay_readiness_ref: 'broker.network.durable-envelope.replay-readiness.10e',
    durable_envelope_delete_export_readiness_ref: 'broker.network.durable-envelope.delete-export-readiness.10e',
    durable_envelope_support_status_ref: 'network.remote-delivery.durable-envelope.support-status.10e',
    durable_envelope_ready: true,
    durable_envelope_missing_artifact_count: 0,
    external_transport_delivery_implemented: false,
    family_hub_delivery_implemented: false,
    cross_process_replay_implemented: false,
    remote_retention_delete_export_propagation_implemented: false,
    provider_delivery_implemented: false,
    child_device_delivery_implemented: false,
    product_ready_claimed: false,
    policy_authority: false,
    side_effect_authority: false,
    enforcement_command_event_count: 0,
    adapter_action_executed_count: 0,
  };
}

function riskDetails() {
  return {
    risk_evaluation_ref: 'network.risk-evaluation.51a',
    risk_child_profile_ref: 'child-profile.51a',
    risk_household_policy_ref: 'household-policy.51a',
    risk_budget_ref: 'network.risk-budget.51a',
    risk_cascade_ref: 'network.cascade.51a',
    risk_age_band: 'UnderTwelve',
    risk_budget_state: 'AskParentThreshold',
    risk_intervention_state: 'AskParent',
    risk_total_points: 42,
    risk_age_profile_points: 15,
    risk_active_signal_points: 27,
    risk_prior_event_points: 0,
    risk_safe_behavior_credit_applied_points: 0,
    risk_triggered_threshold_points: 40,
    risk_cited_signal_refs: ['network.signal.51a'],
    risk_cited_audit_refs: ['network.audit.51a'],
    risk_cited_evidence_refs: ['network.flow-evidence.51a'],
    risk_cited_parent_rule_refs: ['network.parent-rule.51a'],
    risk_cited_prior_event_refs: [],
    risk_adapter_proof_state: 'Ready',
    risk_budget_advisory_only: true,
  };
}

function performanceDetails() {
  return {
    performance_benchmark_run_ref: 'network.performance.51a',
    performance_fixture_set_ref: 'network.performance.fixtures.51a',
    performance_event_history_ref: 'network.performance.event-history.51a',
    performance_resource_snapshot_ref: 'network.performance.resource-snapshot.51a',
    performance_state: 'MeetsBenchmarkGate',
    performance_regression_codes: [],
    performance_scenario_count: 2,
    performance_fixture_count: 20,
    performance_packet_count: 2000,
    performance_flow_count: 600,
    performance_event_count: 1200,
    performance_max_packet_to_summary_latency_ms: 80,
    performance_max_packet_to_detection_latency_ms: 700,
    performance_max_detection_to_cascade_latency_ms: 90,
    performance_max_cascade_to_command_latency_ms: null,
    performance_event_throughput_per_second: 3200,
    performance_max_cpu_millis: 120,
    performance_max_memory_peak_kib: 40000,
    performance_total_disk_written_bytes: 20000,
    performance_max_queue_depth: 4,
    performance_dropped_event_count: 0,
    performance_high_concurrency_flow_count: 2100,
    performance_false_positive_count: 0,
    performance_false_negative_count: 0,
    performance_path_states: ['DryRun'],
    performance_realtime_response_claimed: false,
    performance_adapter_action_executed: false,
    performance_host_filtering_executed: false,
  };
}

function platformDetails() {
  return {
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
