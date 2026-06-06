import {
  parseAgentNetworkProductReadinessStatusEvent,
  type AgentNetworkLiveCaptureCustodyStatus,
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
  readonly riskBudgetState: PortalDetailValue;
  readonly riskInterventionState: PortalDetailValue;
  readonly performanceState: PortalDetailValue;
  readonly performancePathStates: PortalDetailValue;
  readonly platformReadyClaims: PortalDetailValue;
  readonly platformDryRunClaims: PortalDetailValue;
  readonly platformResearchOnlyClaims: PortalDetailValue;
  readonly platformManualRequiredClaims: PortalDetailValue;
  readonly platformUnavailableClaims: PortalDetailValue;
  readonly platformManualFollowups: PortalDetailValue;
  readonly portalReadModelReady: PortalDetailValue;
  readonly retentionExportRefsVisible: PortalDetailValue;
  readonly noClaimBoundary: PortalDetailValue;
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
    riskBudgetState: notReported(),
    riskInterventionState: notReported(),
    performanceState: notReported(),
    performancePathStates: notReported(),
    platformReadyClaims: notReported(),
    platformDryRunClaims: notReported(),
    platformResearchOnlyClaims: notReported(),
    platformManualRequiredClaims: notReported(),
    platformUnavailableClaims: notReported(),
    platformManualFollowups: notReported(),
    portalReadModelReady: notReported(),
    retentionExportRefsVisible: notReported(),
    noClaimBoundary: notReported(),
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
    custodyStatusRef: detailFromValue(custody.status_ref),
    custodyState: detailFromValue(custody.state),
    liveCaptureState: detailFromValue(custody.live_capture_state),
    rawCaptureStorageState: detailFromValue(custody.raw_capture_storage_state),
    captureReady: detailFromValue(custody.capture_ready),
    rawArtifactStorageAuthorized: detailFromValue(custody.raw_artifact_storage_authorized),
    missingArtifactCount: detailFromValue(custody.missing_artifacts.length),
    readinessStatusRef: detailFromValue(product.status_ref),
    readinessState: detailFromValue(product.readiness_state),
    riskBudgetState: detailFromValue(product.risk_budget_state),
    riskInterventionState: detailFromValue(product.risk_intervention_state),
    performanceState: detailFromValue(product.performance_state),
    performancePathStates: joinedDetail(product.performance_path_states),
    platformReadyClaims: detailFromValue(product.platform_ready_claims),
    platformDryRunClaims: detailFromValue(product.platform_dry_run_claims),
    platformResearchOnlyClaims: detailFromValue(product.platform_research_only_claims),
    platformManualRequiredClaims: detailFromValue(product.platform_manual_required_claims),
    platformUnavailableClaims: detailFromValue(product.platform_unavailable_claims),
    platformManualFollowups: platformManualFollowups(product),
    portalReadModelReady: detailFromValue(product.portal_read_model_ready),
    retentionExportRefsVisible: detailFromValue(product.retention_export_refs_visible),
    noClaimBoundary: detailFromValue(noClaimBoundaryUpgraded(custody, product)),
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
  ];
}

function joinedDetail(values: readonly unknown[]): PortalDetailValue {
  const normalized = values.filter(isReportedValue).map((value) => String(value));
  if (normalized.length === 0) {
    return notReported();
  }
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
