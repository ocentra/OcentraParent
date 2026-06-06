import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema, AgentProtocolDefaults } from '../src/contracts';
import { parseAgentNetworkProductReadinessStatusEvent } from '../src/network-product-readiness-status';

describe('network product readiness status protocol adapter', () => {
  it('parses the service-backed live-capture custody and product readiness status payloads', () => {
    const parsed = parseAgentNetworkProductReadinessStatusEvent(productReadinessEvent());

    expect(parsed.ok).toBe(true);
    if (!parsed.ok) {
      throw new Error(parsed.reason);
    }
    expect(parsed.liveCaptureCustodyStatus.status_ref).toBe('network.live-capture.custody-status.13a');
    expect(parsed.liveCaptureCustodyStatus.state).toBe('CustodyReady');
    expect(parsed.liveCaptureCustodyStatus.live_capture_executed).toBe(false);
    expect(parsed.liveCaptureCustodyStatus.exact_url_available).toBe(false);
    expect(parsed.productReadinessStatus.status_ref).toBe('network.product-readiness.status.51a');
    expect(parsed.productReadinessStatus.readiness_state).toBe('ManualRequired');
    expect(parsed.productReadinessStatus.risk_evaluation_ref).toBe('network.risk-evaluation.51a');
    expect(parsed.productReadinessStatus.risk_age_band).toBe('UnderTwelve');
    expect(parsed.productReadinessStatus.risk_budget_state).toBe('AskParentThreshold');
    expect(parsed.productReadinessStatus.risk_total_points).toBe(42);
    expect(parsed.productReadinessStatus.risk_cited_evidence_refs).toEqual(['network.flow-evidence.51a']);
    expect(parsed.productReadinessStatus.risk_adapter_proof_state).toBe('Ready');
    expect(parsed.productReadinessStatus.risk_budget_advisory_only).toBe(true);
    expect(parsed.productReadinessStatus.performance_benchmark_run_ref).toBe('network.performance.51a');
    expect(parsed.productReadinessStatus.performance_packet_count).toBe(2000);
    expect(parsed.productReadinessStatus.performance_event_throughput_per_second).toBe(3200);
    expect(parsed.productReadinessStatus.performance_realtime_response_claimed).toBe(false);
    expect(parsed.productReadinessStatus.performance_adapter_action_executed).toBe(false);
    expect(parsed.productReadinessStatus.performance_host_filtering_executed).toBe(false);
    expect(parsed.productReadinessStatus.platform_entries).toHaveLength(4);
    expect(parsed.productReadinessStatus.platform_entries[0]?.target).toBe('WindowsFirewall');
    expect(parsed.productReadinessStatus.platform_entries[0]?.adapter_authorized_by_proof).toBe(true);
    expect(parsed.productReadinessStatus.platform_entries[2]?.target).toBe('WindowsWfp');
    expect(parsed.productReadinessStatus.platform_entries[2]?.missing_required_artifacts).toEqual([
      'network.platform-claim.manual-followup.51a',
    ]);
    expect(parsed.productReadinessStatus.portal_adapter_dispatch_claimed).toBe(false);
    expect(parsed.productReadinessStatus.enforcement_commands_published).toBe(0);
  });

  it('rejects wrong events and malformed payloads without inventing status rows', () => {
    expect(parseAgentNetworkProductReadinessStatusEvent(wrongEvent()).ok).toBe(false);
    expect(parseAgentNetworkProductReadinessStatusEvent(missingProductStatusEvent())).toEqual({
      ok: false,
      reason: 'missing-product-readiness-status',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(invalidJsonEvent())).toEqual({
      ok: false,
      reason: 'invalid-product-readiness-status-json',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(invalidProductStatusEvent())).toEqual({
      ok: false,
      reason: 'invalid-product-readiness-status',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(claimRegressionEvent())).toEqual({
      ok: false,
      reason: 'invalid-product-readiness-status',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(unknownReadinessStateEvent())).toEqual({
      ok: false,
      reason: 'invalid-product-readiness-status',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(platformEntryCommandRegressionEvent())).toEqual({
      ok: false,
      reason: 'invalid-product-readiness-status',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(platformCountRegressionEvent())).toEqual({
      ok: false,
      reason: 'invalid-product-readiness-status',
    });
  });
});

function productReadinessEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network-product-readiness',
    correlationId: 'cmd-network-product-readiness',
    sentAt: '2026-06-06T05:50:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event: AgentEvent.NetworkProductReadinessStatusReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
    },
    snapshot: null,
  });
}

function wrongEvent() {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: 'evt-network-product-readiness',
    correlationId: 'cmd-network-product-readiness',
    sentAt: '2026-06-06T05:50:00Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event: AgentEvent.HealthReported,
    severity: 'info',
    payload: {},
    snapshot: null,
  });
}

function missingProductStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
    },
  });
}

function invalidJsonEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: '{',
    },
  });
}

function invalidProductStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify({
        ...productReadinessStatus(),
        readiness_state: null,
      }),
    },
  });
}

function claimRegressionEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify({
        ...productReadinessStatus(),
        exact_url_available: true,
        portal_adapter_dispatch_claimed: true,
        performance_host_filtering_executed: true,
      }),
    },
  });
}

function unknownReadinessStateEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify({
        ...productReadinessStatus(),
        readiness_state: 'ReadyMaybe',
      }),
    },
  });
}

function platformEntryCommandRegressionEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify({
        ...productReadinessStatus(),
        platform_entries: [
          {
            ...platformEntries()[0],
            enforcement_command_published: true,
          },
        ],
      }),
    },
  });
}

function platformCountRegressionEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify({
        ...productReadinessStatus(),
        platform_ready_claims: 2,
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
