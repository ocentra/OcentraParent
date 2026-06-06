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
const NetworkRiskBudgetStateSchema = Schema.Literal(
  'WithinBudget',
  'MonitorThreshold',
  'AskParentThreshold',
  'WarnChildThreshold',
  'LimitThreshold',
  'BlockThreshold'
);
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
const NetworkProductReadinessManualFollowupSchema = withParser(
  Schema.Struct({
    target: NetworkPlatformClaimTargetSchema,
    missing_required_artifacts: Schema.Array(NetworkProductReadinessProtocolText),
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
    risk_budget_ref: NetworkProductReadinessProtocolText,
    risk_budget_state: NetworkRiskBudgetStateSchema,
    risk_intervention_state: NetworkInterventionStateSchema,
    risk_total_points: NetworkProductReadinessProtocolCount,
    risk_budget_advisory_only: Schema.Boolean,
    performance_state: NetworkPerformanceBenchmarkStateSchema,
    performance_regression_codes: Schema.Array(NetworkPerformanceRegressionCodeSchema),
    performance_path_states: Schema.Array(NetworkPerformancePathStateSchema),
    platform_ready_claims: NetworkProductReadinessProtocolCount,
    platform_dry_run_claims: NetworkProductReadinessProtocolCount,
    platform_research_only_claims: NetworkProductReadinessProtocolCount,
    platform_manual_required_claims: NetworkProductReadinessProtocolCount,
    platform_unavailable_claims: NetworkProductReadinessProtocolCount,
    platform_manual_followups: Schema.Array(NetworkProductReadinessManualFollowupSchema),
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

export type AgentNetworkLiveCaptureCustodyStatus = Infer<typeof AgentNetworkLiveCaptureCustodyStatusSchema>;
export type AgentNetworkProductReadinessStatus = Infer<typeof AgentNetworkProductReadinessStatusSchema>;

export type AgentNetworkProductReadinessStatusParseResult =
  | {
      readonly ok: true;
      readonly liveCaptureCustodyStatus: AgentNetworkLiveCaptureCustodyStatus;
      readonly productReadinessStatus: AgentNetworkProductReadinessStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-live-capture-custody-status'
        | 'missing-product-readiness-status'
        | 'invalid-live-capture-custody-status-json'
        | 'invalid-product-readiness-status-json'
        | 'invalid-live-capture-custody-status'
        | 'invalid-product-readiness-status';
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

  const parsedLiveCapture = AgentNetworkLiveCaptureCustodyStatusSchema.safeParse(liveCaptureStatus.value);
  if (!parsedLiveCapture.success) {
    return parserFailure('invalid-live-capture-custody-status');
  }

  const parsedProduct = AgentNetworkProductReadinessStatusSchema.safeParse(productStatus.value);
  if (!parsedProduct.success) {
    return parserFailure('invalid-product-readiness-status');
  }

  return {
    ok: true,
    liveCaptureCustodyStatus: parsedLiveCapture.data,
    productReadinessStatus: parsedProduct.data,
  };
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
