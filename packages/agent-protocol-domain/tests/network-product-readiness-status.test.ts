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
    expect(parsed.productReadinessStatus.risk_budget_state).toBe('AskParentThreshold');
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
