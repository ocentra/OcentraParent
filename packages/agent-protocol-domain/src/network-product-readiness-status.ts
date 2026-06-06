import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const NetworkProductReadinessProtocolText = Schema.String.pipe(Schema.minLength(1));
const NetworkProductReadinessProtocolCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NetworkLiveCaptureCustodyStatusStateSchema = Schema.Literal(
  'CustodyReady',
  'ManualRequired',
  'Unavailable',
  'Degraded'
);
const NetworkLiveCaptureProofStateSchema = Schema.Literal('ProofReady', 'ManualRequired', 'Unavailable', 'Degraded');
const NetworkRawCaptureStorageStateSchema = Schema.Literal('CustodyReady', 'ManualRequired', 'Unavailable', 'Degraded');
const NetworkProductReadinessStatusStateSchema = Schema.Literal('ReadyForPortal', 'ManualRequired', 'Degraded');
const NetworkRemoteDeliveryStatusStateSchema = Schema.Literal(
  'RequirementsSatisfiedButNotImplemented',
  'ManualRequired'
);
const NetworkRemoteLifecycleBlockerCount = 3;
const NetworkLocalAiRuntimeResultBridgeStateSchema = Schema.Literal(
  'ResultReady',
  'RuntimeUnavailable',
  'RuntimeFailed',
  'RuntimeTimedOut',
  'QueueNotReady'
);
const NetworkLocalAiRuntimeResultQueueStatusSchema = Schema.Literal(
  'Queued',
  'NotRecommended',
  'DisabledByParent',
  'ModelUnavailable',
  'QueueUnavailable'
);
const NetworkRiskBudgetStateSchema = Schema.Literal(
  'WithinBudget',
  'MonitorThreshold',
  'AskParentThreshold',
  'WarnChildThreshold',
  'LimitThreshold',
  'BlockThreshold'
);
const NetworkRiskBudgetAgeBandSchema = Schema.Literal(
  'UnderTwelve',
  'ThirteenToFifteen',
  'SixteenToSeventeen',
  'AdultOrUnknown'
);
const NetworkRiskBudgetAdapterProofStateSchema = Schema.Literal('NotNeeded', 'Missing', 'Ready');
const NetworkInterventionStateSchema = Schema.Literal(
  'Ignore',
  'Monitor',
  'AskParent',
  'WarnChild',
  'Limit',
  'Block',
  'ManualRequired'
);
const NetworkPerformanceBenchmarkStateSchema = Schema.Literal('MeetsBenchmarkGate', 'BenchmarkGateExceeded');
const NetworkPerformancePathStateSchema = Schema.Literal(
  'DryRun',
  'ManualRequired',
  'Unsupported',
  'Unavailable',
  'Degraded'
);
const NetworkPerformanceRegressionCodeSchema = Schema.Literal(
  'PacketToDetectionLatencyExceeded',
  'EventThroughputBelowMinimum',
  'QueueDepthExceeded',
  'DroppedEventsObserved',
  'CpuBudgetExceeded',
  'MemoryBudgetExceeded',
  'DiskBudgetExceeded',
  'HighConcurrencyFlowCountBelowMinimum'
);
const NetworkPlatformClaimTargetSchema = Schema.Literal(
  'WindowsFirewall',
  'WindowsWfp',
  'AndroidVpnService',
  'AppleNetworkExtensionMacOs',
  'AppleNetworkExtensionIos',
  'LinuxNftables',
  'LinuxEbpf',
  'LinuxTun'
);
const NetworkPlatformClaimStateSchema = Schema.Literal(
  'Ready',
  'DryRun',
  'ResearchOnly',
  'ManualRequired',
  'Unavailable'
);
const NetworkProductReadinessManualFollowupSchema = withParser(
  Schema.Struct({
    target: NetworkPlatformClaimTargetSchema,
    missing_required_artifacts: Schema.Array(NetworkProductReadinessProtocolText),
  })
);
const NetworkPlatformClaimEntrySchema = withParser(
  Schema.Struct({
    target: NetworkPlatformClaimTargetSchema,
    claim_state: NetworkPlatformClaimStateSchema,
    policy_decision_ref: NetworkProductReadinessProtocolText,
    parent_rule_ref: NetworkProductReadinessProtocolText,
    evidence_refs: Schema.Array(NetworkProductReadinessProtocolText),
    device_or_os_refs: Schema.Array(NetworkProductReadinessProtocolText),
    permission_or_entitlement_refs: Schema.Array(NetworkProductReadinessProtocolText),
    adapter_capability_refs: Schema.Array(NetworkProductReadinessProtocolText),
    missing_required_artifacts: Schema.Array(NetworkProductReadinessProtocolText),
    audit_refs: Schema.Array(NetworkProductReadinessProtocolText),
    adapter_authorized_by_proof: Schema.Boolean,
    enforcement_command_published: Schema.Literal(false),
  })
);
const NetworkProductReadinessMissingArtifactSchema = withParser(
  Schema.Record({
    key: NetworkProductReadinessProtocolText,
    value: NetworkProductReadinessProtocolText,
  })
);

export const AgentNetworkLiveCaptureCustodyStatusSchema = withParser(
  Schema.Struct({
    status_ref: NetworkProductReadinessProtocolText,
    live_capture_proof_ref: NetworkProductReadinessProtocolText,
    raw_capture_storage_proof_ref: NetworkProductReadinessProtocolText,
    state: NetworkLiveCaptureCustodyStatusStateSchema,
    live_capture_state: NetworkLiveCaptureProofStateSchema,
    raw_capture_storage_state: NetworkRawCaptureStorageStateSchema,
    missing_artifacts: Schema.Array(NetworkProductReadinessMissingArtifactSchema),
    capture_ready: Schema.Boolean,
    raw_artifact_storage_authorized: Schema.Boolean,
    driver_invoked: Schema.Literal(false),
    live_capture_executed: Schema.Literal(false),
    raw_artifact_created: Schema.Literal(false),
    remote_upload_enabled: Schema.Literal(false),
    raw_pcap_without_custody_available: Schema.Literal(false),
    exact_url_available: Schema.Literal(false),
    decrypted_payload_available: Schema.Literal(false),
    page_content_available: Schema.Literal(false),
    private_message_available: Schema.Literal(false),
    search_query_available: Schema.Literal(false),
    policy_authority: Schema.Literal(false),
    adapter_authority: Schema.Literal(false),
    enforcement_commands_published: Schema.Literal(0),
  })
);

export const AgentNetworkProductReadinessStatusSchema = withParser(
  Schema.Struct({
    status_ref: NetworkProductReadinessProtocolText,
    portal_read_model_ref: NetworkProductReadinessProtocolText,
    retention_export_ref: NetworkProductReadinessProtocolText,
    readiness_state: NetworkProductReadinessStatusStateSchema,
    risk_evaluation_ref: NetworkProductReadinessProtocolText,
    risk_child_profile_ref: NetworkProductReadinessProtocolText,
    risk_household_policy_ref: NetworkProductReadinessProtocolText,
    risk_budget_ref: NetworkProductReadinessProtocolText,
    risk_cascade_ref: NetworkProductReadinessProtocolText,
    risk_age_band: NetworkRiskBudgetAgeBandSchema,
    risk_budget_state: NetworkRiskBudgetStateSchema,
    risk_intervention_state: NetworkInterventionStateSchema,
    risk_total_points: NetworkProductReadinessProtocolCount,
    risk_age_profile_points: NetworkProductReadinessProtocolCount,
    risk_active_signal_points: NetworkProductReadinessProtocolCount,
    risk_prior_event_points: NetworkProductReadinessProtocolCount,
    risk_safe_behavior_credit_applied_points: NetworkProductReadinessProtocolCount,
    risk_triggered_threshold_points: NetworkProductReadinessProtocolCount,
    risk_cited_signal_refs: Schema.Array(NetworkProductReadinessProtocolText),
    risk_cited_audit_refs: Schema.Array(NetworkProductReadinessProtocolText),
    risk_cited_evidence_refs: Schema.Array(NetworkProductReadinessProtocolText),
    risk_cited_parent_rule_refs: Schema.Array(NetworkProductReadinessProtocolText),
    risk_cited_prior_event_refs: Schema.Array(NetworkProductReadinessProtocolText),
    risk_adapter_proof_state: NetworkRiskBudgetAdapterProofStateSchema,
    risk_budget_advisory_only: Schema.Boolean,
    performance_benchmark_run_ref: NetworkProductReadinessProtocolText,
    performance_fixture_set_ref: NetworkProductReadinessProtocolText,
    performance_event_history_ref: NetworkProductReadinessProtocolText,
    performance_resource_snapshot_ref: NetworkProductReadinessProtocolText,
    performance_state: NetworkPerformanceBenchmarkStateSchema,
    performance_regression_codes: Schema.Array(NetworkPerformanceRegressionCodeSchema),
    performance_scenario_count: NetworkProductReadinessProtocolCount,
    performance_fixture_count: NetworkProductReadinessProtocolCount,
    performance_packet_count: NetworkProductReadinessProtocolCount,
    performance_flow_count: NetworkProductReadinessProtocolCount,
    performance_event_count: NetworkProductReadinessProtocolCount,
    performance_max_packet_to_summary_latency_ms: NetworkProductReadinessProtocolCount,
    performance_max_packet_to_detection_latency_ms: NetworkProductReadinessProtocolCount,
    performance_max_detection_to_cascade_latency_ms: NetworkProductReadinessProtocolCount,
    performance_max_cascade_to_command_latency_ms: Schema.Union(NetworkProductReadinessProtocolCount, Schema.Null),
    performance_event_throughput_per_second: NetworkProductReadinessProtocolCount,
    performance_max_cpu_millis: NetworkProductReadinessProtocolCount,
    performance_max_memory_peak_kib: NetworkProductReadinessProtocolCount,
    performance_total_disk_written_bytes: NetworkProductReadinessProtocolCount,
    performance_max_queue_depth: NetworkProductReadinessProtocolCount,
    performance_dropped_event_count: NetworkProductReadinessProtocolCount,
    performance_high_concurrency_flow_count: NetworkProductReadinessProtocolCount,
    performance_false_positive_count: NetworkProductReadinessProtocolCount,
    performance_false_negative_count: NetworkProductReadinessProtocolCount,
    performance_path_states: Schema.Array(NetworkPerformancePathStateSchema),
    performance_realtime_response_claimed: Schema.Literal(false),
    performance_adapter_action_executed: Schema.Literal(false),
    performance_host_filtering_executed: Schema.Literal(false),
    platform_ready_claims: NetworkProductReadinessProtocolCount,
    platform_dry_run_claims: NetworkProductReadinessProtocolCount,
    platform_research_only_claims: NetworkProductReadinessProtocolCount,
    platform_manual_required_claims: NetworkProductReadinessProtocolCount,
    platform_unavailable_claims: NetworkProductReadinessProtocolCount,
    platform_manual_followups: Schema.Array(NetworkProductReadinessManualFollowupSchema),
    platform_entries: Schema.Array(NetworkPlatformClaimEntrySchema),
    portal_read_model_ready: Schema.Boolean,
    retention_export_refs_visible: Schema.Boolean,
    policy_authority: Schema.Literal(false),
    adapter_authority: Schema.Literal(false),
    ui_policy_authority: Schema.Literal(false),
    live_adapter_execution_claimed: Schema.Literal(false),
    portal_adapter_dispatch_claimed: Schema.Literal(false),
    enforcement_commands_published: Schema.Literal(0),
    production_slo_claimed: Schema.Literal(false),
    exact_url_available: Schema.Literal(false),
    decrypted_payload_available: Schema.Literal(false),
    page_content_available: Schema.Literal(false),
  })
);

export const AgentNetworkRemoteDeliveryStatusSchema = withParser(
  Schema.Struct({
    status_ref: NetworkProductReadinessProtocolText,
    broker_status: NetworkRemoteDeliveryStatusStateSchema,
    family_hub_status: NetworkRemoteDeliveryStatusStateSchema,
    custody_proof_ref: NetworkProductReadinessProtocolText,
    publisher_auth_ref: NetworkProductReadinessProtocolText,
    subscriber_auth_ref: NetworkProductReadinessProtocolText,
    encryption_ref: NetworkProductReadinessProtocolText,
    retention_policy_ref: NetworkProductReadinessProtocolText,
    replay_plan_ref: NetworkProductReadinessProtocolText,
    deletion_plan_ref: NetworkProductReadinessProtocolText,
    offset_policy_ref: NetworkProductReadinessProtocolText,
    dedupe_policy_ref: NetworkProductReadinessProtocolText,
    transport_config_ref: NetworkProductReadinessProtocolText,
    relay_identity_ref: NetworkProductReadinessProtocolText,
    relay_policy_ref: NetworkProductReadinessProtocolText,
    broker_missing_artifact_count: NetworkProductReadinessProtocolCount,
    family_hub_missing_artifact_count: NetworkProductReadinessProtocolCount,
    accepted_event_type_count: NetworkProductReadinessProtocolCount,
    local_idempotency_queue_proved: Schema.Boolean,
    dropped_event_dead_letter_count: NetworkProductReadinessProtocolCount,
    queued_duplicate_rejected: Schema.Boolean,
    completed_duplicate_rejected: Schema.Boolean,
    cross_process_replay_ref: NetworkProductReadinessProtocolText,
    remote_retention_delete_export_ref: NetworkProductReadinessProtocolText,
    remote_delivery_ack_ref: NetworkProductReadinessProtocolText,
    remote_lifecycle_followup_ref: NetworkProductReadinessProtocolText,
    remote_lifecycle_missing_artifact_count: NetworkProductReadinessProtocolCount,
    remote_lifecycle_manual_required: Schema.Literal(true),
    external_transport_delivery_implemented: Schema.Literal(false),
    family_hub_delivery_implemented: Schema.Literal(false),
    cross_process_replay_implemented: Schema.Literal(false),
    remote_retention_delete_export_propagation_implemented: Schema.Literal(false),
    policy_authority: Schema.Literal(false),
    side_effect_authority: Schema.Literal(false),
    enforcement_command_event_count: Schema.Literal(0),
    adapter_action_executed_count: Schema.Literal(0),
  })
);

export const AgentNetworkLocalAiRuntimeResultStatusSchema = withParser(
  Schema.Struct({
    status_ref: NetworkProductReadinessProtocolText,
    bridge_state: NetworkLocalAiRuntimeResultBridgeStateSchema,
    queue_status: NetworkLocalAiRuntimeResultQueueStatusSchema,
    trigger_ref: NetworkProductReadinessProtocolText,
    queue_job_ref: Schema.Union(NetworkProductReadinessProtocolText, Schema.Null),
    queue_ref: Schema.Union(NetworkProductReadinessProtocolText, Schema.Null),
    model_runtime_ref: Schema.Union(NetworkProductReadinessProtocolText, Schema.Null),
    local_ai_result_ref: Schema.Union(NetworkProductReadinessProtocolText, Schema.Null),
    runtime_reference_id: Schema.Union(NetworkProductReadinessProtocolText, Schema.Null),
    model_reference: Schema.Union(NetworkProductReadinessProtocolText, Schema.Null),
    model_version_ref: Schema.Union(NetworkProductReadinessProtocolText, Schema.Null),
    prompt_template_ref: NetworkProductReadinessProtocolText,
    policy_context_ref: NetworkProductReadinessProtocolText,
    parent_rule_refs: Schema.Array(NetworkProductReadinessProtocolText),
    evidence_refs: Schema.Array(NetworkProductReadinessProtocolText),
    summary_refs: Schema.Array(NetworkProductReadinessProtocolText),
    managed_browser_exact_url_evidence_refs: Schema.Array(NetworkProductReadinessProtocolText),
    output_summary_ref: Schema.Union(NetworkProductReadinessProtocolText, Schema.Null),
    local_runtime_result_observed: Schema.Boolean,
    audit_input_ready: Schema.Boolean,
    local_model_output_available: Schema.Boolean,
    model_execution_proved: Schema.Literal(false),
    raw_pcap_available: Schema.Literal(false),
    exact_url_claimed: Schema.Literal(false),
    decrypted_payload_available: Schema.Literal(false),
    page_content_available: Schema.Literal(false),
    private_message_available: Schema.Literal(false),
    search_query_available: Schema.Literal(false),
    remote_ai_used: Schema.Literal(false),
    policy_authority: Schema.Literal(false),
    adapter_authority: Schema.Literal(false),
    enforcement_commands_published: Schema.Literal(0),
  })
);

export type AgentNetworkLiveCaptureCustodyStatus = Infer<typeof AgentNetworkLiveCaptureCustodyStatusSchema>;
export type AgentNetworkLocalAiRuntimeResultStatus = Infer<typeof AgentNetworkLocalAiRuntimeResultStatusSchema>;
export type AgentNetworkPlatformClaimEntry = Infer<typeof NetworkPlatformClaimEntrySchema>;
export type AgentNetworkProductReadinessStatus = Infer<typeof AgentNetworkProductReadinessStatusSchema>;
export type AgentNetworkRemoteDeliveryStatus = Infer<typeof AgentNetworkRemoteDeliveryStatusSchema>;

export type AgentNetworkProductReadinessStatusParseResult =
  | {
      readonly ok: true;
      readonly liveCaptureCustodyStatus: AgentNetworkLiveCaptureCustodyStatus;
      readonly localAiRuntimeResultStatus: AgentNetworkLocalAiRuntimeResultStatus;
      readonly productReadinessStatus: AgentNetworkProductReadinessStatus;
      readonly remoteDeliveryStatus: AgentNetworkRemoteDeliveryStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-live-capture-custody-status'
        | 'missing-local-ai-runtime-result-status'
        | 'missing-product-readiness-status'
        | 'missing-remote-delivery-status'
        | 'invalid-live-capture-custody-status-json'
        | 'invalid-local-ai-runtime-result-status-json'
        | 'invalid-product-readiness-status-json'
        | 'invalid-remote-delivery-status-json'
        | 'invalid-live-capture-custody-status'
        | 'invalid-local-ai-runtime-result-status'
        | 'invalid-product-readiness-status'
        | 'invalid-remote-delivery-status';
    };
export type AgentNetworkProductReadinessStatusFailureReason = Extract<
  AgentNetworkProductReadinessStatusParseResult,
  { readonly ok: false }
>['reason'];

export function parseAgentNetworkProductReadinessStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkProductReadinessStatusParseResult {
  if (event.event !== AgentEvent.NetworkProductReadinessStatusReported) {
    return parserFailure('wrong-event');
  }

  const liveCaptureStatus = parseJsonField(
    event,
    AgentProtocolDefaults.Field.NetworkLiveCaptureCustodyStatus,
    'missing-live-capture-custody-status',
    'invalid-live-capture-custody-status-json'
  );
  if (!liveCaptureStatus.ok) {
    return liveCaptureStatus;
  }

  const productStatus = parseJsonField(
    event,
    AgentProtocolDefaults.Field.NetworkProductReadinessStatus,
    'missing-product-readiness-status',
    'invalid-product-readiness-status-json'
  );
  if (!productStatus.ok) {
    return productStatus;
  }

  const localAiRuntimeResultStatus = parseJsonField(
    event,
    AgentProtocolDefaults.Field.NetworkLocalAiRuntimeResultStatus,
    'missing-local-ai-runtime-result-status',
    'invalid-local-ai-runtime-result-status-json'
  );
  if (!localAiRuntimeResultStatus.ok) {
    return localAiRuntimeResultStatus;
  }

  const remoteDeliveryStatus = parseJsonField(
    event,
    AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus,
    'missing-remote-delivery-status',
    'invalid-remote-delivery-status-json'
  );
  if (!remoteDeliveryStatus.ok) {
    return remoteDeliveryStatus;
  }

  const parsedLiveCapture = AgentNetworkLiveCaptureCustodyStatusSchema.safeParse(liveCaptureStatus.value);
  if (!parsedLiveCapture.success) {
    return parserFailure('invalid-live-capture-custody-status');
  }

  const parsedProduct = AgentNetworkProductReadinessStatusSchema.safeParse(productStatus.value);
  if (!parsedProduct.success) {
    return parserFailure('invalid-product-readiness-status');
  }
  if (!platformEntryCountsMatch(parsedProduct.data)) {
    return parserFailure('invalid-product-readiness-status');
  }

  const parsedLocalAiRuntimeResult = parseLocalAiRuntimeResultStatus(localAiRuntimeResultStatus.value);
  if (!parsedLocalAiRuntimeResult.ok) {
    return parsedLocalAiRuntimeResult;
  }

  const parsedRemoteDelivery = parseRemoteDeliveryStatus(remoteDeliveryStatus.value);
  if (!parsedRemoteDelivery.ok) {
    return parsedRemoteDelivery;
  }

  return {
    ok: true,
    liveCaptureCustodyStatus: parsedLiveCapture.data,
    localAiRuntimeResultStatus: parsedLocalAiRuntimeResult.data,
    productReadinessStatus: parsedProduct.data,
    remoteDeliveryStatus: parsedRemoteDelivery.data,
  };
}

function platformEntryCountsMatch(product: AgentNetworkProductReadinessStatus): boolean {
  return (
    platformEntryCount(product, 'Ready') === product.platform_ready_claims &&
    platformEntryCount(product, 'DryRun') === product.platform_dry_run_claims &&
    platformEntryCount(product, 'ResearchOnly') === product.platform_research_only_claims &&
    platformEntryCount(product, 'ManualRequired') === product.platform_manual_required_claims &&
    platformEntryCount(product, 'Unavailable') === product.platform_unavailable_claims
  );
}

function platformEntryCount(
  product: AgentNetworkProductReadinessStatus,
  claimState: AgentNetworkPlatformClaimEntry['claim_state']
): number {
  return product.platform_entries.filter((entry) => entry.claim_state === claimState).length;
}

function parseLocalAiRuntimeResultStatus(value: unknown):
  | {
      readonly ok: true;
      readonly data: AgentNetworkLocalAiRuntimeResultStatus;
    }
  | Extract<AgentNetworkProductReadinessStatusParseResult, { readonly ok: false }> {
  const parsed = AgentNetworkLocalAiRuntimeResultStatusSchema.safeParse(value);
  if (!parsed.success || !localAiRuntimeResultShapeMatches(parsed.data)) {
    return parserFailure('invalid-local-ai-runtime-result-status');
  }
  return { ok: true, data: parsed.data };
}

function localAiRuntimeResultShapeMatches(status: AgentNetworkLocalAiRuntimeResultStatus): boolean {
  if (status.bridge_state === 'QueueNotReady') {
    return queueNotReadyShapeMatches(status);
  }

  if (!queuedRuntimeShapeMatches(status)) {
    return false;
  }

  if (status.bridge_state === 'ResultReady') {
    return resultReadyShapeMatches(status);
  }

  return nonCompleteRuntimeShapeMatches(status);
}

function queueNotReadyShapeMatches(status: AgentNetworkLocalAiRuntimeResultStatus): boolean {
  return (
    status.queue_status !== 'Queued' &&
    resultRefsAreNull(status) &&
    status.managed_browser_exact_url_evidence_refs.length === 0 &&
    status.output_summary_ref === null &&
    status.local_runtime_result_observed === false &&
    status.audit_input_ready === false &&
    status.local_model_output_available === false
  );
}

function queuedRuntimeShapeMatches(status: AgentNetworkLocalAiRuntimeResultStatus): boolean {
  return (
    status.queue_status === 'Queued' && resultRefsArePresent(status) && status.local_runtime_result_observed === true
  );
}

function resultReadyShapeMatches(status: AgentNetworkLocalAiRuntimeResultStatus): boolean {
  return (
    status.output_summary_ref !== null &&
    status.audit_input_ready === true &&
    status.local_model_output_available === true
  );
}

function nonCompleteRuntimeShapeMatches(status: AgentNetworkLocalAiRuntimeResultStatus): boolean {
  return (
    status.output_summary_ref === null &&
    status.audit_input_ready === false &&
    status.local_model_output_available === false
  );
}

function parseRemoteDeliveryStatus(value: unknown):
  | {
      readonly ok: true;
      readonly data: AgentNetworkRemoteDeliveryStatus;
    }
  | Extract<AgentNetworkProductReadinessStatusParseResult, { readonly ok: false }> {
  const parsed = AgentNetworkRemoteDeliveryStatusSchema.safeParse(value);
  if (!parsed.success || !remoteDeliveryShapeMatches(parsed.data)) {
    return parserFailure('invalid-remote-delivery-status');
  }
  return { ok: true, data: parsed.data };
}

function remoteDeliveryShapeMatches(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    remoteDeliveryRequirementCountsMatch(status) &&
    remoteDeliveryLifecycleBlockersMatch(status) &&
    remoteDeliveryLocalProofMatches(status)
  );
}

function remoteDeliveryRequirementCountsMatch(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    (status.broker_status !== 'RequirementsSatisfiedButNotImplemented' || status.broker_missing_artifact_count === 0) &&
    (status.family_hub_status !== 'RequirementsSatisfiedButNotImplemented' ||
      status.family_hub_missing_artifact_count === 0)
  );
}

function remoteDeliveryLifecycleBlockersMatch(status: AgentNetworkRemoteDeliveryStatus): boolean {
  const lifecycleBlockerRefs = [
    status.cross_process_replay_ref,
    status.remote_retention_delete_export_ref,
    status.remote_delivery_ack_ref,
  ];
  return (
    status.remote_lifecycle_missing_artifact_count === NetworkRemoteLifecycleBlockerCount &&
    status.remote_lifecycle_missing_artifact_count === lifecycleBlockerRefs.length &&
    lifecycleBlockerRefs.every((ref) => ref.includes('manual-required'))
  );
}

function remoteDeliveryLocalProofMatches(status: AgentNetworkRemoteDeliveryStatus): boolean {
  return (
    status.accepted_event_type_count > 0 &&
    status.local_idempotency_queue_proved === true &&
    status.dropped_event_dead_letter_count > 0 &&
    status.queued_duplicate_rejected === true &&
    status.completed_duplicate_rejected === true
  );
}

function resultRefsArePresent(status: AgentNetworkLocalAiRuntimeResultStatus): boolean {
  return (
    status.queue_job_ref !== null &&
    status.queue_ref !== null &&
    status.model_runtime_ref !== null &&
    status.local_ai_result_ref !== null &&
    status.runtime_reference_id !== null &&
    status.model_reference !== null &&
    status.model_version_ref !== null
  );
}

function resultRefsAreNull(status: AgentNetworkLocalAiRuntimeResultStatus): boolean {
  return (
    status.queue_job_ref === null &&
    status.queue_ref === null &&
    status.model_runtime_ref === null &&
    status.local_ai_result_ref === null &&
    status.runtime_reference_id === null &&
    status.model_reference === null &&
    status.model_version_ref === null
  );
}

function parseJsonField(
  event: AgentEventEnvelope,
  field: string,
  missingReason: Extract<AgentNetworkProductReadinessStatusParseResult, { readonly ok: false }>['reason'],
  invalidReason: Extract<AgentNetworkProductReadinessStatusParseResult, { readonly ok: false }>['reason']
):
  | {
      readonly ok: true;
      readonly value: unknown;
    }
  | Extract<AgentNetworkProductReadinessStatusParseResult, { readonly ok: false }> {
  const raw = event.payload[field];
  if (!isAgentProtocolLogText(raw)) {
    return parserFailure(missingReason);
  }

  try {
    return { ok: true, value: JSON.parse(raw) as unknown };
  } catch {
    return parserFailure(invalidReason);
  }
}

function parserFailure(
  reason: Extract<AgentNetworkProductReadinessStatusParseResult, { readonly ok: false }>['reason']
): Extract<AgentNetworkProductReadinessStatusParseResult, { readonly ok: false }> {
  return {
    ok: false,
    reason,
  };
}
