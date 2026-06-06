import {
  parseAgentNetworkProductReadinessStatusEvent,
  type AgentNetworkLiveCaptureCustodyStatus,
  type AgentNetworkLocalAiRuntimeResultStatus,
  type AgentNetworkPlatformClaimEntry,
  type AgentNetworkProductReadinessStatusFailureReason,
  type AgentNetworkProductReadinessStatus,
  type AgentNetworkRemoteDeliveryStatus,
} from '@ocentra-parent/agent-protocol-domain/network-product-readiness-status';
import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  PortalFormatting,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';

export type NetworkProductReadinessStatusSummary = {
  readonly parserStatus: PortalDetailValue;
  readonly custodyStatusRef: PortalDetailValue;
  readonly custodyState: PortalDetailValue;
  readonly liveCaptureState: PortalDetailValue;
  readonly rawCaptureStorageState: PortalDetailValue;
  readonly captureReady: PortalDetailValue;
  readonly rawArtifactStorageAuthorized: PortalDetailValue;
  readonly missingArtifactCount: PortalDetailValue;
  readonly readinessStatusRef: PortalDetailValue;
  readonly readinessState: PortalDetailValue;
  readonly riskEvaluationRef: PortalDetailValue;
  readonly riskChildProfileRef: PortalDetailValue;
  readonly riskHouseholdPolicyRef: PortalDetailValue;
  readonly riskCascadeRef: PortalDetailValue;
  readonly riskAgeBand: PortalDetailValue;
  readonly riskBudgetState: PortalDetailValue;
  readonly riskInterventionState: PortalDetailValue;
  readonly riskTotalPoints: PortalDetailValue;
  readonly riskPointBreakdown: PortalDetailValue;
  readonly riskCitedSignalRefs: PortalDetailValue;
  readonly riskCitedAuditRefs: PortalDetailValue;
  readonly riskCitedEvidenceRefs: PortalDetailValue;
  readonly riskCitedParentRuleRefs: PortalDetailValue;
  readonly riskCitedPriorEventRefs: PortalDetailValue;
  readonly riskAdapterProofState: PortalDetailValue;
  readonly riskBudgetAdvisoryOnly: PortalDetailValue;
  readonly performanceBenchmarkRunRef: PortalDetailValue;
  readonly performanceFixtureSetRef: PortalDetailValue;
  readonly performanceEventHistoryRef: PortalDetailValue;
  readonly performanceResourceSnapshotRef: PortalDetailValue;
  readonly performanceState: PortalDetailValue;
  readonly performanceRegressionCodes: PortalDetailValue;
  readonly performanceScenarioCounts: PortalDetailValue;
  readonly performanceLatencyMetrics: PortalDetailValue;
  readonly performanceThroughputMetrics: PortalDetailValue;
  readonly performanceResourceMetrics: PortalDetailValue;
  readonly performanceQualityMetrics: PortalDetailValue;
  readonly performancePathStates: PortalDetailValue;
  readonly performanceProductionSloClaimed: PortalDetailValue;
  readonly performanceAdapterExecutionClaimed: PortalDetailValue;
  readonly performanceHostFilteringClaimed: PortalDetailValue;
  readonly platformReadyClaims: PortalDetailValue;
  readonly platformDryRunClaims: PortalDetailValue;
  readonly platformResearchOnlyClaims: PortalDetailValue;
  readonly platformManualRequiredClaims: PortalDetailValue;
  readonly platformUnavailableClaims: PortalDetailValue;
  readonly platformManualFollowups: PortalDetailValue;
  readonly portalReadModelReady: PortalDetailValue;
  readonly retentionExportRefsVisible: PortalDetailValue;
  readonly remoteDeliveryStatusRef: PortalDetailValue;
  readonly remoteBrokerStatus: PortalDetailValue;
  readonly remoteFamilyHubStatus: PortalDetailValue;
  readonly remoteCustodyProofRef: PortalDetailValue;
  readonly remoteAuthRefs: PortalDetailValue;
  readonly remoteTransportRefs: PortalDetailValue;
  readonly remoteLifecycleRefs: PortalDetailValue;
  readonly remoteMissingArtifactCounts: PortalDetailValue;
  readonly remoteAcceptedEventTypeCount: PortalDetailValue;
  readonly remoteLocalQueueProof: PortalDetailValue;
  readonly remoteDuplicateProof: PortalDetailValue;
  readonly remoteDeadLetterCount: PortalDetailValue;
  readonly remoteLifecycleBlockerRefs: PortalDetailValue;
  readonly remoteLifecycleFollowupRef: PortalDetailValue;
  readonly remoteLifecycleMissingArtifactCount: PortalDetailValue;
  readonly remoteLifecycleManualRequired: PortalDetailValue;
  readonly remoteExternalTransportImplemented: PortalDetailValue;
  readonly remoteFamilyHubDeliveryImplemented: PortalDetailValue;
  readonly remoteCrossProcessReplayImplemented: PortalDetailValue;
  readonly remoteRetentionDeleteExportImplemented: PortalDetailValue;
  readonly remotePolicyAuthority: PortalDetailValue;
  readonly remoteSideEffectAuthority: PortalDetailValue;
  readonly remoteEnforcementCommandEventCount: PortalDetailValue;
  readonly remoteAdapterActionExecutedCount: PortalDetailValue;
  readonly localAiRuntimeResultStatusRef: PortalDetailValue;
  readonly localAiBridgeState: PortalDetailValue;
  readonly localAiQueueStatus: PortalDetailValue;
  readonly localAiTriggerRef: PortalDetailValue;
  readonly localAiQueueRefs: PortalDetailValue;
  readonly localAiRuntimeRefs: PortalDetailValue;
  readonly localAiModelRefs: PortalDetailValue;
  readonly localAiPromptPolicyRefs: PortalDetailValue;
  readonly localAiParentRuleRefs: PortalDetailValue;
  readonly localAiEvidenceRefs: PortalDetailValue;
  readonly localAiSummaryRefs: PortalDetailValue;
  readonly localAiManagedBrowserExactUrlEvidenceRefs: PortalDetailValue;
  readonly localAiResultRef: PortalDetailValue;
  readonly localAiOutputSummaryRef: PortalDetailValue;
  readonly localAiRuntimeResultObserved: PortalDetailValue;
  readonly localAiAuditInputReady: PortalDetailValue;
  readonly localAiModelOutputAvailable: PortalDetailValue;
  readonly localAiModelExecutionProved: PortalDetailValue;
  readonly localAiRawPcapAvailable: PortalDetailValue;
  readonly localAiExactUrlClaimed: PortalDetailValue;
  readonly localAiDecryptedPayloadAvailable: PortalDetailValue;
  readonly localAiPageContentAvailable: PortalDetailValue;
  readonly localAiPrivateMessageAvailable: PortalDetailValue;
  readonly localAiSearchQueryAvailable: PortalDetailValue;
  readonly localAiRemoteAiUsed: PortalDetailValue;
  readonly localAiPolicyAuthority: PortalDetailValue;
  readonly localAiAdapterAuthority: PortalDetailValue;
  readonly localAiEnforcementCommandsPublished: PortalDetailValue;
  readonly noClaimBoundary: PortalDetailValue;
  readonly platformEntries: readonly NetworkPlatformClaimManifestEntrySummary[];
};

export type NetworkPlatformClaimManifestEntrySummary = {
  readonly target: PortalDetailValue;
  readonly state: PortalDetailValue;
  readonly policyDecisionRef: PortalDetailValue;
  readonly parentRuleRef: PortalDetailValue;
  readonly evidenceRefs: PortalDetailValue;
  readonly deviceOrOsRefs: PortalDetailValue;
  readonly permissionOrEntitlementRefs: PortalDetailValue;
  readonly adapterCapabilityRefs: PortalDetailValue;
  readonly missingRequiredArtifacts: PortalDetailValue;
  readonly auditRefs: PortalDetailValue;
  readonly adapterAuthorizedByProof: PortalDetailValue;
  readonly enforcementCommandPublished: PortalDetailValue;
};

export function parseNetworkProductReadinessStatus(
  event: AgentEventEnvelope | null
): NetworkProductReadinessStatusSummary | null {
  if (event === null) {
    return null;
  }

  const parsed = parseAgentNetworkProductReadinessStatusEvent(event);
  if (!parsed.ok) {
    return failedNetworkProductReadinessStatusSummary(parsed.reason);
  }

  return networkProductReadinessStatusSummary(
    parsed.liveCaptureCustodyStatus,
    parsed.localAiRuntimeResultStatus,
    parsed.productReadinessStatus,
    parsed.remoteDeliveryStatus
  );
}

export function emptyNetworkProductReadinessStatusSummary(): NetworkProductReadinessStatusSummary {
  return {
    parserStatus: notReported(),
    ...emptyLiveCaptureCustodySummary(),
    ...emptyRiskReadinessSummary(),
    ...emptyPerformanceReadinessSummary(),
    ...emptyPlatformReadinessSummary(),
    portalReadModelReady: notReported(),
    retentionExportRefsVisible: notReported(),
    ...emptyRemoteDeliverySummary(),
    ...emptyLocalAiRuntimeResultSummary(),
    noClaimBoundary: notReported(),
    platformEntries: [],
  };
}

function emptyLiveCaptureCustodySummary() {
  return {
    custodyStatusRef: notReported(),
    custodyState: notReported(),
    liveCaptureState: notReported(),
    rawCaptureStorageState: notReported(),
    captureReady: notReported(),
    rawArtifactStorageAuthorized: notReported(),
    missingArtifactCount: notReported(),
  };
}

function emptyRiskReadinessSummary() {
  return {
    readinessStatusRef: notReported(),
    readinessState: notReported(),
    riskEvaluationRef: notReported(),
    riskChildProfileRef: notReported(),
    riskHouseholdPolicyRef: notReported(),
    riskCascadeRef: notReported(),
    riskAgeBand: notReported(),
    riskBudgetState: notReported(),
    riskInterventionState: notReported(),
    riskTotalPoints: notReported(),
    riskPointBreakdown: notReported(),
    riskCitedSignalRefs: notReported(),
    riskCitedAuditRefs: notReported(),
    riskCitedEvidenceRefs: notReported(),
    riskCitedParentRuleRefs: notReported(),
    riskCitedPriorEventRefs: notReported(),
    riskAdapterProofState: notReported(),
    riskBudgetAdvisoryOnly: notReported(),
  };
}

function emptyPerformanceReadinessSummary() {
  return {
    performanceBenchmarkRunRef: notReported(),
    performanceFixtureSetRef: notReported(),
    performanceEventHistoryRef: notReported(),
    performanceResourceSnapshotRef: notReported(),
    performanceState: notReported(),
    performanceRegressionCodes: notReported(),
    performanceScenarioCounts: notReported(),
    performanceLatencyMetrics: notReported(),
    performanceThroughputMetrics: notReported(),
    performanceResourceMetrics: notReported(),
    performanceQualityMetrics: notReported(),
    performancePathStates: notReported(),
    performanceProductionSloClaimed: notReported(),
    performanceAdapterExecutionClaimed: notReported(),
    performanceHostFilteringClaimed: notReported(),
  };
}

function emptyPlatformReadinessSummary() {
  return {
    platformReadyClaims: notReported(),
    platformDryRunClaims: notReported(),
    platformResearchOnlyClaims: notReported(),
    platformManualRequiredClaims: notReported(),
    platformUnavailableClaims: notReported(),
    platformManualFollowups: notReported(),
  };
}

function emptyRemoteDeliverySummary() {
  return {
    remoteDeliveryStatusRef: notReported(),
    remoteBrokerStatus: notReported(),
    remoteFamilyHubStatus: notReported(),
    remoteCustodyProofRef: notReported(),
    remoteAuthRefs: notReported(),
    remoteTransportRefs: notReported(),
    remoteLifecycleRefs: notReported(),
    remoteMissingArtifactCounts: notReported(),
    remoteAcceptedEventTypeCount: notReported(),
    remoteLocalQueueProof: notReported(),
    remoteDuplicateProof: notReported(),
    remoteDeadLetterCount: notReported(),
    remoteLifecycleBlockerRefs: notReported(),
    remoteLifecycleFollowupRef: notReported(),
    remoteLifecycleMissingArtifactCount: notReported(),
    remoteLifecycleManualRequired: notReported(),
    remoteExternalTransportImplemented: notReported(),
    remoteFamilyHubDeliveryImplemented: notReported(),
    remoteCrossProcessReplayImplemented: notReported(),
    remoteRetentionDeleteExportImplemented: notReported(),
    remotePolicyAuthority: notReported(),
    remoteSideEffectAuthority: notReported(),
    remoteEnforcementCommandEventCount: notReported(),
    remoteAdapterActionExecutedCount: notReported(),
  };
}

function emptyLocalAiRuntimeResultSummary() {
  return {
    localAiRuntimeResultStatusRef: notReported(),
    localAiBridgeState: notReported(),
    localAiQueueStatus: notReported(),
    localAiTriggerRef: notReported(),
    localAiQueueRefs: notReported(),
    localAiRuntimeRefs: notReported(),
    localAiModelRefs: notReported(),
    localAiPromptPolicyRefs: notReported(),
    localAiParentRuleRefs: notReported(),
    localAiEvidenceRefs: notReported(),
    localAiSummaryRefs: notReported(),
    localAiManagedBrowserExactUrlEvidenceRefs: notReported(),
    localAiResultRef: notReported(),
    localAiOutputSummaryRef: notReported(),
    localAiRuntimeResultObserved: notReported(),
    localAiAuditInputReady: notReported(),
    localAiModelOutputAvailable: notReported(),
    localAiModelExecutionProved: notReported(),
    localAiRawPcapAvailable: notReported(),
    localAiExactUrlClaimed: notReported(),
    localAiDecryptedPayloadAvailable: notReported(),
    localAiPageContentAvailable: notReported(),
    localAiPrivateMessageAvailable: notReported(),
    localAiSearchQueryAvailable: notReported(),
    localAiRemoteAiUsed: notReported(),
    localAiPolicyAuthority: notReported(),
    localAiAdapterAuthority: notReported(),
    localAiEnforcementCommandsPublished: notReported(),
  };
}

function failedNetworkProductReadinessStatusSummary(
  reason: AgentNetworkProductReadinessStatusFailureReason
): NetworkProductReadinessStatusSummary {
  return {
    ...emptyNetworkProductReadinessStatusSummary(),
    parserStatus: detailFromValue(reason),
  };
}

function networkProductReadinessStatusSummary(
  custody: AgentNetworkLiveCaptureCustodyStatus,
  localAi: AgentNetworkLocalAiRuntimeResultStatus,
  product: AgentNetworkProductReadinessStatus,
  remote: AgentNetworkRemoteDeliveryStatus
): NetworkProductReadinessStatusSummary {
  return {
    parserStatus: detailFromValue(true),
    ...liveCaptureCustodySummary(custody),
    ...riskReadinessSummary(product),
    ...performanceReadinessSummary(product),
    ...platformReadinessSummary(product),
    ...remoteDeliverySummary(remote),
    ...localAiRuntimeResultSummary(localAi),
    portalReadModelReady: detailFromValue(product.portal_read_model_ready),
    retentionExportRefsVisible: detailFromValue(product.retention_export_refs_visible),
    noClaimBoundary: detailFromValue(noClaimBoundaryUpgraded(custody, localAi, product, remote)),
    platformEntries: product.platform_entries.map(platformEntrySummary),
  };
}

function liveCaptureCustodySummary(custody: AgentNetworkLiveCaptureCustodyStatus) {
  return {
    custodyStatusRef: detailFromValue(custody.status_ref),
    custodyState: detailFromValue(custody.state),
    liveCaptureState: detailFromValue(custody.live_capture_state),
    rawCaptureStorageState: detailFromValue(custody.raw_capture_storage_state),
    captureReady: detailFromValue(custody.capture_ready),
    rawArtifactStorageAuthorized: detailFromValue(custody.raw_artifact_storage_authorized),
    missingArtifactCount: detailFromValue(custody.missing_artifacts.length),
  };
}

function riskReadinessSummary(product: AgentNetworkProductReadinessStatus) {
  return {
    readinessStatusRef: detailFromValue(product.status_ref),
    readinessState: detailFromValue(product.readiness_state),
    riskEvaluationRef: detailFromValue(product.risk_evaluation_ref),
    riskChildProfileRef: detailFromValue(product.risk_child_profile_ref),
    riskHouseholdPolicyRef: detailFromValue(product.risk_household_policy_ref),
    riskCascadeRef: detailFromValue(product.risk_cascade_ref),
    riskAgeBand: detailFromValue(product.risk_age_band),
    riskBudgetState: detailFromValue(product.risk_budget_state),
    riskInterventionState: detailFromValue(product.risk_intervention_state),
    riskTotalPoints: detailFromValue(product.risk_total_points),
    riskPointBreakdown: joinedDetail([
      product.risk_age_profile_points,
      product.risk_active_signal_points,
      product.risk_prior_event_points,
      product.risk_safe_behavior_credit_applied_points,
      product.risk_triggered_threshold_points,
    ]),
    riskCitedSignalRefs: joinedDetail(product.risk_cited_signal_refs),
    riskCitedAuditRefs: joinedDetail(product.risk_cited_audit_refs),
    riskCitedEvidenceRefs: joinedDetail(product.risk_cited_evidence_refs),
    riskCitedParentRuleRefs: joinedDetail(product.risk_cited_parent_rule_refs),
    riskCitedPriorEventRefs: joinedDetail(product.risk_cited_prior_event_refs),
    riskAdapterProofState: detailFromValue(product.risk_adapter_proof_state),
    riskBudgetAdvisoryOnly: detailFromValue(product.risk_budget_advisory_only),
  };
}

function performanceReadinessSummary(product: AgentNetworkProductReadinessStatus) {
  return {
    performanceBenchmarkRunRef: detailFromValue(product.performance_benchmark_run_ref),
    performanceFixtureSetRef: detailFromValue(product.performance_fixture_set_ref),
    performanceEventHistoryRef: detailFromValue(product.performance_event_history_ref),
    performanceResourceSnapshotRef: detailFromValue(product.performance_resource_snapshot_ref),
    performanceState: detailFromValue(product.performance_state),
    performanceRegressionCodes: joinedDetail(product.performance_regression_codes),
    performanceScenarioCounts: joinedDetail([
      product.performance_scenario_count,
      product.performance_fixture_count,
      product.performance_packet_count,
      product.performance_flow_count,
      product.performance_event_count,
    ]),
    performanceLatencyMetrics: joinedNullableDetail([
      product.performance_max_packet_to_summary_latency_ms,
      product.performance_max_packet_to_detection_latency_ms,
      product.performance_max_detection_to_cascade_latency_ms,
      product.performance_max_cascade_to_command_latency_ms,
    ]),
    performanceThroughputMetrics: joinedDetail([
      product.performance_event_throughput_per_second,
      product.performance_max_queue_depth,
      product.performance_dropped_event_count,
      product.performance_high_concurrency_flow_count,
    ]),
    performanceResourceMetrics: joinedDetail([
      product.performance_max_cpu_millis,
      product.performance_max_memory_peak_kib,
      product.performance_total_disk_written_bytes,
    ]),
    performanceQualityMetrics: joinedDetail([
      product.performance_false_positive_count,
      product.performance_false_negative_count,
    ]),
    performancePathStates: joinedDetail(product.performance_path_states),
    performanceProductionSloClaimed: detailFromValue(product.production_slo_claimed),
    performanceAdapterExecutionClaimed: detailFromValue(product.performance_adapter_action_executed),
    performanceHostFilteringClaimed: detailFromValue(product.performance_host_filtering_executed),
  };
}

function platformReadinessSummary(product: AgentNetworkProductReadinessStatus) {
  return {
    platformReadyClaims: detailFromValue(product.platform_ready_claims),
    platformDryRunClaims: detailFromValue(product.platform_dry_run_claims),
    platformResearchOnlyClaims: detailFromValue(product.platform_research_only_claims),
    platformManualRequiredClaims: detailFromValue(product.platform_manual_required_claims),
    platformUnavailableClaims: detailFromValue(product.platform_unavailable_claims),
    platformManualFollowups: platformManualFollowups(product),
  };
}

function remoteDeliverySummary(remote: AgentNetworkRemoteDeliveryStatus) {
  return {
    remoteDeliveryStatusRef: detailFromValue(remote.status_ref),
    remoteBrokerStatus: detailFromValue(remote.broker_status),
    remoteFamilyHubStatus: detailFromValue(remote.family_hub_status),
    remoteCustodyProofRef: detailFromValue(remote.custody_proof_ref),
    remoteAuthRefs: joinedDetail([remote.publisher_auth_ref, remote.subscriber_auth_ref]),
    remoteTransportRefs: joinedDetail([
      remote.encryption_ref,
      remote.transport_config_ref,
      remote.relay_identity_ref,
      remote.relay_policy_ref,
    ]),
    remoteLifecycleRefs: joinedDetail([
      remote.retention_policy_ref,
      remote.replay_plan_ref,
      remote.deletion_plan_ref,
      remote.offset_policy_ref,
      remote.dedupe_policy_ref,
      remote.cross_process_replay_ref,
      remote.remote_retention_delete_export_ref,
      remote.remote_delivery_ack_ref,
    ]),
    remoteMissingArtifactCounts: joinedDetail([
      remote.broker_missing_artifact_count,
      remote.family_hub_missing_artifact_count,
      remote.remote_lifecycle_missing_artifact_count,
    ]),
    remoteAcceptedEventTypeCount: detailFromValue(remote.accepted_event_type_count),
    remoteLocalQueueProof: detailFromValue(remote.local_idempotency_queue_proved),
    remoteDuplicateProof: joinedDetail([remote.queued_duplicate_rejected, remote.completed_duplicate_rejected]),
    remoteDeadLetterCount: detailFromValue(remote.dropped_event_dead_letter_count),
    remoteLifecycleBlockerRefs: joinedDetail([
      remote.cross_process_replay_ref,
      remote.remote_retention_delete_export_ref,
      remote.remote_delivery_ack_ref,
    ]),
    remoteLifecycleFollowupRef: detailFromValue(remote.remote_lifecycle_followup_ref),
    remoteLifecycleMissingArtifactCount: detailFromValue(remote.remote_lifecycle_missing_artifact_count),
    remoteLifecycleManualRequired: detailFromValue(remote.remote_lifecycle_manual_required),
    remoteExternalTransportImplemented: detailFromValue(remote.external_transport_delivery_implemented),
    remoteFamilyHubDeliveryImplemented: detailFromValue(remote.family_hub_delivery_implemented),
    remoteCrossProcessReplayImplemented: detailFromValue(remote.cross_process_replay_implemented),
    remoteRetentionDeleteExportImplemented: detailFromValue(
      remote.remote_retention_delete_export_propagation_implemented
    ),
    remotePolicyAuthority: detailFromValue(remote.policy_authority),
    remoteSideEffectAuthority: detailFromValue(remote.side_effect_authority),
    remoteEnforcementCommandEventCount: detailFromValue(remote.enforcement_command_event_count),
    remoteAdapterActionExecutedCount: detailFromValue(remote.adapter_action_executed_count),
  };
}

function localAiRuntimeResultSummary(localAi: AgentNetworkLocalAiRuntimeResultStatus) {
  return {
    localAiRuntimeResultStatusRef: detailFromValue(localAi.status_ref),
    localAiBridgeState: detailFromValue(localAi.bridge_state),
    localAiQueueStatus: detailFromValue(localAi.queue_status),
    localAiTriggerRef: detailFromValue(localAi.trigger_ref),
    localAiQueueRefs: joinedDetail([localAi.queue_job_ref, localAi.queue_ref]),
    localAiRuntimeRefs: joinedDetail([localAi.model_runtime_ref, localAi.runtime_reference_id]),
    localAiModelRefs: joinedDetail([localAi.model_reference, localAi.model_version_ref]),
    localAiPromptPolicyRefs: joinedDetail([localAi.prompt_template_ref, localAi.policy_context_ref]),
    localAiParentRuleRefs: joinedDetail(localAi.parent_rule_refs),
    localAiEvidenceRefs: joinedDetail(localAi.evidence_refs),
    localAiSummaryRefs: joinedDetail(localAi.summary_refs),
    localAiManagedBrowserExactUrlEvidenceRefs: joinedDetail(localAi.managed_browser_exact_url_evidence_refs),
    localAiResultRef: detailFromValue(localAi.local_ai_result_ref),
    localAiOutputSummaryRef: detailFromValue(localAi.output_summary_ref),
    localAiRuntimeResultObserved: detailFromValue(localAi.local_runtime_result_observed),
    localAiAuditInputReady: detailFromValue(localAi.audit_input_ready),
    localAiModelOutputAvailable: detailFromValue(localAi.local_model_output_available),
    localAiModelExecutionProved: detailFromValue(localAi.model_execution_proved),
    localAiRawPcapAvailable: detailFromValue(localAi.raw_pcap_available),
    localAiExactUrlClaimed: detailFromValue(localAi.exact_url_claimed),
    localAiDecryptedPayloadAvailable: detailFromValue(localAi.decrypted_payload_available),
    localAiPageContentAvailable: detailFromValue(localAi.page_content_available),
    localAiPrivateMessageAvailable: detailFromValue(localAi.private_message_available),
    localAiSearchQueryAvailable: detailFromValue(localAi.search_query_available),
    localAiRemoteAiUsed: detailFromValue(localAi.remote_ai_used),
    localAiPolicyAuthority: detailFromValue(localAi.policy_authority),
    localAiAdapterAuthority: detailFromValue(localAi.adapter_authority),
    localAiEnforcementCommandsPublished: detailFromValue(localAi.enforcement_commands_published),
  };
}

function platformEntrySummary(entry: AgentNetworkPlatformClaimEntry): NetworkPlatformClaimManifestEntrySummary {
  return {
    target: detailFromValue(entry.target),
    state: detailFromValue(entry.claim_state),
    policyDecisionRef: detailFromValue(entry.policy_decision_ref),
    parentRuleRef: detailFromValue(entry.parent_rule_ref),
    evidenceRefs: joinedDetail(entry.evidence_refs),
    deviceOrOsRefs: joinedDetail(entry.device_or_os_refs),
    permissionOrEntitlementRefs: joinedDetail(entry.permission_or_entitlement_refs),
    adapterCapabilityRefs: joinedDetail(entry.adapter_capability_refs),
    missingRequiredArtifacts: joinedDetail(entry.missing_required_artifacts),
    auditRefs: joinedDetail(entry.audit_refs),
    adapterAuthorizedByProof: detailFromValue(entry.adapter_authorized_by_proof),
    enforcementCommandPublished: detailFromValue(entry.enforcement_command_published),
  };
}

function platformManualFollowups(product: AgentNetworkProductReadinessStatus): PortalDetailValue {
  return joinedDetail(
    product.platform_manual_followups.map((followup) =>
      [followup.target, ...followup.missing_required_artifacts].join(PortalFormatting.EventDetailSeparator)
    )
  );
}

function noClaimBoundaryUpgraded(
  custody: AgentNetworkLiveCaptureCustodyStatus,
  localAi: AgentNetworkLocalAiRuntimeResultStatus,
  product: AgentNetworkProductReadinessStatus,
  remote: AgentNetworkRemoteDeliveryStatus
): boolean {
  return (
    unsupportedCustodyClaims(custody).some(Boolean) ||
    unsupportedLocalAiClaims(localAi).some(Boolean) ||
    unsupportedProductClaims(product).some(Boolean) ||
    unsupportedRemoteDeliveryClaims(remote).some(Boolean)
  );
}

function unsupportedCustodyClaims(custody: AgentNetworkLiveCaptureCustodyStatus): readonly boolean[] {
  return [
    custody.driver_invoked ||
      custody.live_capture_executed ||
      custody.raw_artifact_created ||
      custody.remote_upload_enabled,
    custody.raw_pcap_without_custody_available,
    custody.exact_url_available ||
      custody.decrypted_payload_available ||
      custody.page_content_available ||
      custody.private_message_available ||
      custody.search_query_available,
    custody.policy_authority || custody.adapter_authority,
    custody.enforcement_commands_published > 0,
  ];
}

function unsupportedProductClaims(product: AgentNetworkProductReadinessStatus): readonly boolean[] {
  return [
    product.policy_authority ||
      product.adapter_authority ||
      product.ui_policy_authority ||
      product.live_adapter_execution_claimed ||
      product.portal_adapter_dispatch_claimed,
    product.enforcement_commands_published > 0,
    product.production_slo_claimed ||
      product.exact_url_available ||
      product.decrypted_payload_available ||
      product.page_content_available,
    product.performance_realtime_response_claimed ||
      product.performance_adapter_action_executed ||
      product.performance_host_filtering_executed,
    product.platform_entries.some((entry) => entry.enforcement_command_published),
  ];
}

function unsupportedLocalAiClaims(localAi: AgentNetworkLocalAiRuntimeResultStatus): readonly boolean[] {
  return [
    localAi.model_execution_proved || localAi.raw_pcap_available,
    localAi.exact_url_claimed ||
      localAi.decrypted_payload_available ||
      localAi.page_content_available ||
      localAi.private_message_available ||
      localAi.search_query_available,
    localAi.remote_ai_used,
    localAi.policy_authority || localAi.adapter_authority,
    localAi.enforcement_commands_published > 0,
  ];
}

function unsupportedRemoteDeliveryClaims(remote: AgentNetworkRemoteDeliveryStatus): readonly boolean[] {
  return [
    remote.external_transport_delivery_implemented || remote.family_hub_delivery_implemented,
    remote.cross_process_replay_implemented || remote.remote_retention_delete_export_propagation_implemented,
    remote.policy_authority || remote.side_effect_authority,
    remote.enforcement_command_event_count > 0,
    remote.adapter_action_executed_count > 0,
  ];
}

function joinedDetail(values: readonly unknown[]): PortalDetailValue {
  const normalized = values.filter(isReportedValue).map((value) => String(value));
  if (normalized.length === 0) {
    return notReported();
  }
  return decodePortalDetailValue(normalized.join(PortalFormatting.EventDetailSeparator));
}

function joinedNullableDetail(values: readonly unknown[]): PortalDetailValue {
  if (values.length === 0) {
    return notReported();
  }
  const normalized = values.map((value) => (isReportedValue(value) ? String(value) : String(notReported())));
  return decodePortalDetailValue(normalized.join(PortalFormatting.EventDetailSeparator));
}

function detailFromValue(value: unknown): PortalDetailValue {
  if (!isReportedValue(value)) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function isReportedValue(value: unknown): boolean {
  return value !== undefined && value !== null && String(value).length > 0;
}

function notReported(): PortalDetailValue {
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
}
