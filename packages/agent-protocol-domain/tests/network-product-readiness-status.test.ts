import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentEventEnvelopeSchema, AgentProtocolDefaults } from '../src/contracts';
import {
  parseAgentNetworkProductReadinessStatusEvent,
  type AgentNetworkLiveCaptureCustodyStatus,
  type AgentNetworkLocalAiRuntimeResultStatus,
  type AgentNetworkProductReadinessStatus,
  type AgentNetworkRemoteDeliveryStatus,
} from '../src/network-product-readiness-status';

describe('network product readiness status protocol adapter', () => {
  it('parses the service-backed live-capture custody and product readiness status payloads', () => {
    const parsed = parseAgentNetworkProductReadinessStatusEvent(productReadinessEvent());

    expect(parsed.ok).toBe(true);
    if (!parsed.ok) {
      throw new Error(parsed.reason);
    }
    assertLiveCaptureCustodyStatus(parsed.liveCaptureCustodyStatus);
    assertLocalAiRuntimeResultStatus(parsed.localAiRuntimeResultStatus);
    assertProductReadinessStatus(parsed.productReadinessStatus);
    assertRemoteDeliveryStatus(parsed.remoteDeliveryStatus);
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
    expect(parseAgentNetworkProductReadinessStatusEvent(missingLocalAiRuntimeResultStatusEvent())).toEqual({
      ok: false,
      reason: 'missing-local-ai-runtime-result-status',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(invalidLocalAiRuntimeResultJsonEvent())).toEqual({
      ok: false,
      reason: 'invalid-local-ai-runtime-result-status-json',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(localAiRuntimeResultClaimRegressionEvent())).toEqual({
      ok: false,
      reason: 'invalid-local-ai-runtime-result-status',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(missingRemoteDeliveryStatusEvent())).toEqual({
      ok: false,
      reason: 'missing-remote-delivery-status',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(invalidRemoteDeliveryJsonEvent())).toEqual({
      ok: false,
      reason: 'invalid-remote-delivery-status-json',
    });
    expect(parseAgentNetworkProductReadinessStatusEvent(remoteDeliveryClaimRegressionEvent())).toEqual({
      ok: false,
      reason: 'invalid-remote-delivery-status',
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

it('rejects local AI runtime result state/ref mismatches', () => {
  expect(parseAgentNetworkProductReadinessStatusEvent(resultReadyMissingOutputSummaryEvent())).toEqual({
    ok: false,
    reason: 'invalid-local-ai-runtime-result-status',
  });
  expect(parseAgentNetworkProductReadinessStatusEvent(runtimeUnavailableOutputSummaryEvent())).toEqual({
    ok: false,
    reason: 'invalid-local-ai-runtime-result-status',
  });
  expect(parseAgentNetworkProductReadinessStatusEvent(queueNotReadyResultRefsEvent())).toEqual({
    ok: false,
    reason: 'invalid-local-ai-runtime-result-status',
  });
});

it('rejects remote delivery lifecycle and broker proof mismatches', () => {
  expect(parseAgentNetworkProductReadinessStatusEvent(remoteDeliveryMissingArtifactCountMismatchEvent())).toEqual({
    ok: false,
    reason: 'invalid-remote-delivery-status',
  });
  expect(parseAgentNetworkProductReadinessStatusEvent(remoteDeliveryLifecycleRefRegressionEvent())).toEqual({
    ok: false,
    reason: 'invalid-remote-delivery-status',
  });
  expect(parseAgentNetworkProductReadinessStatusEvent(remoteDeliveryBrokerRequirementCountMismatchEvent())).toEqual({
    ok: false,
    reason: 'invalid-remote-delivery-status',
  });
  expect(parseAgentNetworkProductReadinessStatusEvent(remoteDeliveryDuplicateProofRegressionEvent())).toEqual({
    ok: false,
    reason: 'invalid-remote-delivery-status',
  });
  expect(parseAgentNetworkProductReadinessStatusEvent(remoteDeliveryDurableEnvelopeReadinessRegressionEvent())).toEqual(
    {
      ok: false,
      reason: 'invalid-remote-delivery-status',
    }
  );
  expect(parseAgentNetworkProductReadinessStatusEvent(remoteDeliveryDurableEnvelopeRefRegressionEvent())).toEqual({
    ok: false,
    reason: 'invalid-remote-delivery-status',
  });
});

function assertLiveCaptureCustodyStatus(status: AgentNetworkLiveCaptureCustodyStatus) {
  expect(status.status_ref).toBe('network.live-capture.custody-status.13a');
  expect(status.state).toBe('CustodyReady');
  expect(status.live_capture_executed).toBe(false);
  expect(status.exact_url_available).toBe(false);
}

function assertProductReadinessStatus(status: AgentNetworkProductReadinessStatus) {
  expect(status.status_ref).toBe('network.product-readiness.status.51a');
  expect(status.readiness_state).toBe('ManualRequired');
  expect(status.risk_evaluation_ref).toBe('network.risk-evaluation.51a');
  expect(status.risk_age_band).toBe('UnderTwelve');
  expect(status.risk_budget_state).toBe('AskParentThreshold');
  expect(status.risk_total_points).toBe(42);
  expect(status.risk_cited_evidence_refs).toEqual(['network.flow-evidence.51a']);
  expect(status.risk_adapter_proof_state).toBe('Ready');
  expect(status.risk_budget_advisory_only).toBe(true);
  expect(status.performance_benchmark_run_ref).toBe('network.performance.51a');
  expect(status.performance_packet_count).toBe(2000);
  expect(status.performance_event_throughput_per_second).toBe(3200);
  expect(status.performance_realtime_response_claimed).toBe(false);
  expect(status.performance_adapter_action_executed).toBe(false);
  expect(status.performance_host_filtering_executed).toBe(false);
  expect(status.platform_entries).toHaveLength(4);
  expect(status.platform_entries[0]?.target).toBe('WindowsFirewall');
  expect(status.platform_entries[0]?.adapter_authorized_by_proof).toBe(true);
  expect(status.platform_entries[2]?.target).toBe('WindowsWfp');
  expect(status.platform_entries[2]?.missing_required_artifacts).toEqual([
    'network.platform-claim.manual-followup.51a',
  ]);
  expect(status.portal_adapter_dispatch_claimed).toBe(false);
  expect(status.enforcement_commands_published).toBe(0);
}

function assertLocalAiRuntimeResultStatus(status: AgentNetworkLocalAiRuntimeResultStatus) {
  expect(status.status_ref).toBe('network.local-ai.runtime-result.status.33b');
  expect(status.bridge_state).toBe('ResultReady');
  expect(status.queue_status).toBe('Queued');
  expect(status.trigger_ref).toBe('network.local-ai.trigger.33b');
  expect(status.queue_job_ref).toBe('network.local-ai.queue-job.33b');
  expect(status.model_runtime_ref).toBe('network.local-ai.model-runtime.33b');
  expect(status.local_ai_result_ref).toBe('network.local-ai.result.33b');
  expect(status.output_summary_ref).toBe('network.local-ai.output-summary.33b');
  expect(status.managed_browser_exact_url_evidence_refs).toEqual([
    'network.local-ai.managed-browser-exact-url-evidence.33b',
  ]);
  expect(status.local_runtime_result_observed).toBe(true);
  expect(status.audit_input_ready).toBe(true);
  expect(status.local_model_output_available).toBe(true);
  expect(status.model_execution_proved).toBe(false);
  expect(status.raw_pcap_available).toBe(false);
  expect(status.exact_url_claimed).toBe(false);
  expect(status.decrypted_payload_available).toBe(false);
  expect(status.page_content_available).toBe(false);
  expect(status.private_message_available).toBe(false);
  expect(status.search_query_available).toBe(false);
  expect(status.remote_ai_used).toBe(false);
  expect(status.policy_authority).toBe(false);
  expect(status.adapter_authority).toBe(false);
  expect(status.enforcement_commands_published).toBe(0);
}

function assertRemoteDeliveryStatus(status: AgentNetworkRemoteDeliveryStatus) {
  expect(status.status_ref).toBe('network.remote-delivery.status.10c');
  expect(status.broker_status).toBe('RequirementsSatisfiedButNotImplemented');
  expect(status.family_hub_status).toBe('RequirementsSatisfiedButNotImplemented');
  expect(status.custody_proof_ref).toBe('broker.network.custody-proof.1');
  expect(status.publisher_auth_ref).toBe('broker.network.publisher-auth.1');
  expect(status.subscriber_auth_ref).toBe('broker.network.subscriber-auth.1');
  expect(status.relay_identity_ref).toBe('family-hub.network.identity.1');
  expect(status.broker_missing_artifact_count).toBe(0);
  expect(status.family_hub_missing_artifact_count).toBe(0);
  expect(status.accepted_event_type_count).toBe(3);
  expect(status.local_idempotency_queue_proved).toBe(true);
  expect(status.dropped_event_dead_letter_count).toBe(1);
  expect(status.queued_duplicate_rejected).toBe(true);
  expect(status.completed_duplicate_rejected).toBe(true);
  expect(status.cross_process_replay_ref).toBe('broker.network.cross-process-replay.manual-required.10d');
  expect(status.remote_retention_delete_export_ref).toBe(
    'broker.network.remote-retention-delete-export.manual-required.10d'
  );
  expect(status.remote_delivery_ack_ref).toBe('family-hub.network.delivery-ack.manual-required.10d');
  expect(status.remote_lifecycle_followup_ref).toBe('network.remote-delivery.lifecycle-followup.10d');
  expect(status.remote_lifecycle_missing_artifact_count).toBe(3);
  expect(status.remote_lifecycle_manual_required).toBe(true);
  expect(status.durable_envelope_schema_ref).toBe('broker.network.durable-envelope.schema.10e');
  expect(status.durable_envelope_journal_ref).toBe('broker.network.durable-envelope.journal-readiness.10e');
  expect(status.durable_envelope_replay_readiness_ref).toBe('broker.network.durable-envelope.replay-readiness.10e');
  expect(status.durable_envelope_delete_export_readiness_ref).toBe(
    'broker.network.durable-envelope.delete-export-readiness.10e'
  );
  expect(status.durable_envelope_support_status_ref).toBe(
    'network.remote-delivery.durable-envelope.support-status.10e'
  );
  expect(status.durable_envelope_ready).toBe(true);
  expect(status.durable_envelope_missing_artifact_count).toBe(0);
  expect(status.external_transport_delivery_implemented).toBe(false);
  expect(status.family_hub_delivery_implemented).toBe(false);
  expect(status.cross_process_replay_implemented).toBe(false);
  expect(status.remote_retention_delete_export_propagation_implemented).toBe(false);
  expect(status.provider_delivery_implemented).toBe(false);
  expect(status.child_device_delivery_implemented).toBe(false);
  expect(status.product_ready_claimed).toBe(false);
  expect(status.policy_authority).toBe(false);
  expect(status.side_effect_authority).toBe(false);
  expect(status.enforcement_command_event_count).toBe(0);
  expect(status.adapter_action_executed_count).toBe(0);
}

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
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
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
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
    },
  });
}

function invalidJsonEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: '{',
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
    },
  });
}

function missingLocalAiRuntimeResultStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
    },
  });
}

function invalidLocalAiRuntimeResultJsonEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: '{',
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
    },
  });
}

function missingRemoteDeliveryStatusEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
    },
  });
}

function invalidRemoteDeliveryJsonEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: '{',
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
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
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
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
    },
  });
}

function localAiRuntimeResultClaimRegressionEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify({
        ...localAiRuntimeResultStatus(),
        model_execution_proved: true,
        remote_ai_used: true,
        enforcement_commands_published: 1,
      }),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
    },
  });
}

function resultReadyMissingOutputSummaryEvent() {
  return localAiRuntimeResultStatusEvent({
    output_summary_ref: null,
    local_model_output_available: false,
  });
}

function runtimeUnavailableOutputSummaryEvent() {
  return localAiRuntimeResultStatusEvent({
    bridge_state: 'RuntimeUnavailable',
    output_summary_ref: 'network.local-ai.output-summary.33b',
    audit_input_ready: true,
    local_model_output_available: true,
  });
}

function queueNotReadyResultRefsEvent() {
  return localAiRuntimeResultStatusEvent({
    bridge_state: 'QueueNotReady',
    queue_status: 'ModelUnavailable',
    local_runtime_result_observed: false,
    audit_input_ready: false,
    local_model_output_available: false,
  });
}

function localAiRuntimeResultStatusEvent(statusPatch: Partial<ReturnType<typeof localAiRuntimeResultStatus>>) {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify({
        ...localAiRuntimeResultStatus(),
        ...statusPatch,
      }),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
    },
  });
}

function remoteDeliveryClaimRegressionEvent() {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify({
        ...remoteDeliveryStatus(),
        family_hub_delivery_implemented: true,
      }),
    },
  });
}

function remoteDeliveryMissingArtifactCountMismatchEvent() {
  return remoteDeliveryStatusEvent({
    remote_lifecycle_missing_artifact_count: 2,
  });
}

function remoteDeliveryLifecycleRefRegressionEvent() {
  return remoteDeliveryStatusEvent({
    cross_process_replay_ref: 'broker.network.cross-process-replay.implemented.10d',
  });
}

function remoteDeliveryBrokerRequirementCountMismatchEvent() {
  return remoteDeliveryStatusEvent({
    broker_missing_artifact_count: 1,
  });
}

function remoteDeliveryDuplicateProofRegressionEvent() {
  return remoteDeliveryStatusEvent({
    queued_duplicate_rejected: false,
  });
}

function remoteDeliveryDurableEnvelopeReadinessRegressionEvent() {
  return remoteDeliveryStatusEvent({
    durable_envelope_missing_artifact_count: 1,
    product_ready_claimed: true,
  });
}

function remoteDeliveryDurableEnvelopeRefRegressionEvent() {
  return remoteDeliveryStatusEvent({
    durable_envelope_schema_ref: 'broker.network.durable-envelope.schema.unversioned',
  });
}

function remoteDeliveryStatusEvent(statusPatch: Partial<ReturnType<typeof remoteDeliveryStatus>>) {
  return AgentEventEnvelopeSchema.parse({
    ...productReadinessEvent(),
    payload: {
      [AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus]: JSON.stringify(liveCaptureCustodyStatus()),
      [AgentProtocolDefaults.Field.NetworkProductReadinessStatus]: JSON.stringify(productReadinessStatus()),
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify({
        ...remoteDeliveryStatus(),
        ...statusPatch,
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
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
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
      [AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus]: JSON.stringify(localAiRuntimeResultStatus()),
      [AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus]: JSON.stringify(remoteDeliveryStatus()),
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
