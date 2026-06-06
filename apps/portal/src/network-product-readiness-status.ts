import {
  parseAgentNetworkProductReadinessStatusEvent,
  type AgentNetworkLiveCaptureCustodyStatus,
  type AgentNetworkPlatformClaimEntry,
  type AgentNetworkProductReadinessStatusFailureReason,
  type AgentNetworkProductReadinessStatus,
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

  return networkProductReadinessStatusSummary(parsed.liveCaptureCustodyStatus, parsed.productReadinessStatus);
}

export function emptyNetworkProductReadinessStatusSummary(): NetworkProductReadinessStatusSummary {
  return {
    parserStatus: notReported(),
    custodyStatusRef: notReported(),
    custodyState: notReported(),
    liveCaptureState: notReported(),
    rawCaptureStorageState: notReported(),
    captureReady: notReported(),
    rawArtifactStorageAuthorized: notReported(),
    missingArtifactCount: notReported(),
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
    platformReadyClaims: notReported(),
    platformDryRunClaims: notReported(),
    platformResearchOnlyClaims: notReported(),
    platformManualRequiredClaims: notReported(),
    platformUnavailableClaims: notReported(),
    platformManualFollowups: notReported(),
    portalReadModelReady: notReported(),
    retentionExportRefsVisible: notReported(),
    noClaimBoundary: notReported(),
    platformEntries: [],
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
  product: AgentNetworkProductReadinessStatus
): NetworkProductReadinessStatusSummary {
  return {
    parserStatus: detailFromValue(true),
    ...liveCaptureCustodySummary(custody),
    ...riskReadinessSummary(product),
    ...performanceReadinessSummary(product),
    ...platformReadinessSummary(product),
    portalReadModelReady: detailFromValue(product.portal_read_model_ready),
    retentionExportRefsVisible: detailFromValue(product.retention_export_refs_visible),
    noClaimBoundary: detailFromValue(noClaimBoundaryUpgraded(custody, product)),
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
  product: AgentNetworkProductReadinessStatus
): boolean {
  return unsupportedCustodyClaims(custody).some(Boolean) || unsupportedProductClaims(product).some(Boolean);
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
