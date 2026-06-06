pub mod ai_audit;
pub mod ai_detection;
pub mod android_vpn_service_gate;
pub mod app_game_session;
pub mod apple_network_extension_gate;
pub mod bundle;
pub mod cascade;
pub mod category;
pub mod classifier;
pub mod dns;
pub mod dns_adapter;
pub mod domain;
pub mod encrypted_dns;
pub mod fixtures;
pub mod flow;
pub mod http;
pub mod linux_adapter_gate;
pub mod live_capture;
pub mod local_ai_queue;
pub mod managed_browser;
pub mod notification;
pub mod packet;
pub mod pcap;
pub mod performance;
pub mod pipeline;
pub mod platform_claims;
pub mod policy;
pub mod process;
pub mod raw_capture_storage;
pub mod readiness;
pub mod risk_budget;
pub mod screen_summary;
pub mod signature_alert;
pub mod tls;
pub mod transfer;
pub mod tunnel;
pub mod unmanaged_browser;
pub mod windows_firewall_adapter;
pub mod windows_wfp_gate;
pub mod zeek;

mod platform_claim_values;
mod process_support;

#[cfg(test)]
mod tests;

pub use ai_audit::{
    build_network_ai_audit_report, NetworkAiAuditNarrativeState, NetworkAiAuditRecommendation,
    NetworkAiAuditRecommendationKind, NetworkAiAuditReport, NetworkAiAuditReportError,
    NetworkAiAuditReportInput, NetworkAiAuditUncertaintyCode,
};
pub use ai_detection::{
    evaluate_network_ai_detection_fixtures, NetworkAiDetectionDriftState,
    NetworkAiDetectionEvaluationError, NetworkAiDetectionEvaluationInput,
    NetworkAiDetectionEvaluationProof, NetworkAiDetectionEvaluationState,
    NetworkAiDetectionFixtureCase, NetworkAiDetectionInputKind, NetworkAiDetectionLabel,
    NetworkAiDetectionPrecisionState, NetworkAiDetectionRecallState, NetworkAiDetectionResult,
    NetworkAiDetectionRiskLevel, NetworkAiDetectionUncertaintyCode,
};
pub use android_vpn_service_gate::{
    plan_network_android_vpn_service_gate, NetworkAndroidVpnServiceCapabilityState,
    NetworkAndroidVpnServiceGateBoundaryReason, NetworkAndroidVpnServiceGateError,
    NetworkAndroidVpnServiceGateInput, NetworkAndroidVpnServiceGateProof,
    NetworkAndroidVpnServiceGateState, NetworkAndroidVpnServiceRequiredArtifact,
};
pub use app_game_session::{
    correlate_app_game_foreground_session, NetworkAppGameEvidenceKind,
    NetworkAppGameForegroundState, NetworkAppGameSessionCorrelation,
    NetworkAppGameSessionCorrelationBasis, NetworkAppGameSessionCorrelationError,
    NetworkAppGameSessionCorrelationInput, NetworkAppGameSessionCorrelationState,
    NetworkAppGameStoredSessionEvidence,
};
pub use apple_network_extension_gate::{
    plan_network_apple_network_extension_gate, NetworkAppleNetworkExtensionCapabilityState,
    NetworkAppleNetworkExtensionGateBoundaryReason, NetworkAppleNetworkExtensionGateError,
    NetworkAppleNetworkExtensionGateInput, NetworkAppleNetworkExtensionGateProof,
    NetworkAppleNetworkExtensionGateState, NetworkAppleNetworkExtensionPlatform,
    NetworkAppleNetworkExtensionRequiredArtifact,
};
pub use bundle::{
    build_network_cross_slice_evidence_bundle, NetworkCrossSliceEvidenceBundle,
    NetworkCrossSliceEvidenceBundleError, NetworkCrossSliceEvidenceBundleInput,
    NetworkCrossSliceEvidenceSource,
};
pub use cascade::{
    route_network_evidence_cascade, NetworkCascadeNextCheck, NetworkCascadeSignalStrength,
    NetworkCascadeSource, NetworkCascadeSourceKind, NetworkEvidenceCascadeDecision,
    NetworkEvidenceCascadeError, NetworkEvidenceCascadeInput,
};
pub use category::{
    evaluate_category_source_update, lookup_domain_category, CategoryFreshnessState,
    CategoryMatchKind, CategorySourceCustody, CategoryUpdateDecision, DomainCategoryDatabase,
    DomainCategoryError, DomainCategoryLookup, DomainCategoryRecord, DomainCategorySource,
    NetworkCategory,
};
pub use classifier::{
    classify_social_video_game_activity, BrowserClassifierConfirmation, CdnClassifierHint,
    NetworkActivityClassification, NetworkActivityClassifierInput, NetworkClassifierBasis,
    NetworkClassifierError, ProcessClassifierHint,
};
pub use dns::{
    parse_dns_message, replay_dns_observations, DnsMessage, DnsObservation, DnsQueryType,
    DnsQuestion, DnsRecordData, DnsResourceRecord, NetworkEvidenceGrade, NetworkReplayError,
    NetworkReplaySummary,
};
pub use dns_adapter::{
    plan_network_dns_adapter_proof, NetworkDnsAdapterAction, NetworkDnsAdapterBoundaryReason,
    NetworkDnsAdapterCapabilityState, NetworkDnsAdapterProof, NetworkDnsAdapterProofError,
    NetworkDnsAdapterProofInput, NetworkDnsAdapterProofState, NetworkDnsAdapterRequiredArtifact,
};
pub use domain::{
    normalize_domain_with_public_suffix, DomainNormalizationError, NormalizedDomainEvidence,
    PublicSuffixModel,
};
pub use encrypted_dns::{
    detect_encrypted_dns_candidate, detect_quic_http3_limitation, EncryptedDnsCandidate,
    EncryptedDnsProtocol, QuicVisibilityLimitation,
};
pub use fixtures::{
    dns_query_frame_fixture, dns_query_pcap_fixture, dns_query_replay_expected,
    dns_response_payload_fixture, http_host_request_fixture, icmp_echo_frame_fixture,
    quic_initial_payload_fixture, tcp_syn_frame_fixture, tls_client_hello_no_sni_fixture,
    tls_client_hello_sni_fixture,
};
pub use flow::{
    aggregate_network_flows, aggregate_pcap_flows, flow_packets_from_pcap, NetworkFlowError,
    NetworkFlowKey, NetworkFlowPacket, NetworkFlowProtocol, NetworkFlowSession, NetworkFlowSummary,
};
pub use http::{parse_http_host, HttpHostObservation, HttpHostParseError};
pub use linux_adapter_gate::{
    plan_network_linux_adapter_gate, NetworkLinuxAdapterCapabilityState,
    NetworkLinuxAdapterGateBoundaryReason, NetworkLinuxAdapterGateError,
    NetworkLinuxAdapterGateInput, NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateState,
    NetworkLinuxAdapterKind, NetworkLinuxAdapterRequiredArtifact,
};
pub use live_capture::{
    plan_network_live_capture_proof, NetworkLiveCapturePlatform, NetworkLiveCaptureProof,
    NetworkLiveCaptureProofError, NetworkLiveCaptureProofInput, NetworkLiveCaptureProofState,
    NetworkLiveCaptureRequiredArtifact,
};
pub use local_ai_queue::{
    plan_network_local_ai_queue, NetworkLocalAiQueueError, NetworkLocalAiQueueInput,
    NetworkLocalAiQueueInputKind, NetworkLocalAiQueueJob, NetworkLocalAiQueuePlan,
    NetworkLocalAiQueueStatus,
};
pub use managed_browser::{
    correlate_managed_browser_activity, ManagedBrowserCorrelation, ManagedBrowserCorrelationBasis,
    ManagedBrowserCorrelationError, ManagedBrowserCorrelationInput, ManagedBrowserCorrelationState,
    ManagedBrowserPageEvidence, NetworkManagedBrowserFlowEvidence,
};
pub use notification::{
    map_network_parent_notification_candidate, NetworkParentNotificationCandidate,
    NetworkParentNotificationCandidateError, NetworkParentNotificationCandidateInput,
    NetworkParentNotificationDeliveryState, NetworkParentNotificationSeverity,
};
pub use packet::{
    parse_network_packet, udp_payload_from_ethernet_ipv4, EthernetFrameMetadata, IpProtocol,
    Ipv4PacketMetadata, PacketParseError, ParsedNetworkPacket, TransportPacketMetadata,
    UdpPayloadView,
};
pub use pcap::{parse_pcap_packets, PcapPacket, PcapReplayError};
pub use performance::{
    evaluate_network_performance_benchmark, NetworkPerformanceBenchmarkError,
    NetworkPerformanceBenchmarkInput, NetworkPerformanceBenchmarkProof,
    NetworkPerformanceBenchmarkRow, NetworkPerformanceBenchmarkState,
    NetworkPerformanceBenchmarkThresholds, NetworkPerformancePathState,
    NetworkPerformanceRegressionCode, NetworkPerformanceScenarioType,
};
pub use pipeline::{
    prove_network_end_to_end_pipeline, NetworkEndToEndPipelineError, NetworkEndToEndPipelineInput,
    NetworkEndToEndPipelineProof, NetworkEndToEndPipelineRefs, NetworkEndToEndUnsupportedClaims,
    NetworkRemoteDeliveryHandoffProof, NetworkRemoteDeliveryHandoffRefs,
    NetworkRetentionDeleteExportProof,
};
pub use platform_claims::{
    build_network_platform_claim_manifest, NetworkPlatformClaimEntry,
    NetworkPlatformClaimManifestError, NetworkPlatformClaimManifestInput,
    NetworkPlatformClaimManifestProof, NetworkPlatformClaimManualFollowup,
    NetworkPlatformClaimProofSource, NetworkPlatformClaimState, NetworkPlatformClaimTarget,
    NetworkPlatformUnsupportedClaims,
};
pub use policy::{
    map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
    NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
    NetworkEvidencePolicyMappingInput, NetworkEvidencePolicyMode,
};
pub use process::{
    correlate_process_app_activity, NetworkAppInventoryEntry, NetworkFlowProcessObservation,
    NetworkProcessAppCorrelation, NetworkProcessAppCorrelationInput,
    NetworkProcessCorrelationBasis, NetworkProcessCorrelationError, NetworkProcessCorrelationState,
    NetworkProcessCorrelationUncertainty, NetworkProcessSnapshot,
};
pub use raw_capture_storage::{
    plan_network_raw_capture_storage, NetworkRawCaptureStorageError, NetworkRawCaptureStorageInput,
    NetworkRawCaptureStorageProof, NetworkRawCaptureStorageRequiredArtifact,
    NetworkRawCaptureStorageState,
};
pub use readiness::{
    evaluate_network_readiness_proof, NetworkHardeningReadinessProof, NetworkReadinessFindingCode,
    NetworkReadinessGate, NetworkReadinessProof, NetworkReadinessProofError,
    NetworkReadinessProofInput, NetworkReadinessState, NetworkRetentionReadinessProof,
    NetworkRolloutReadinessProof, NetworkSupportReadinessProof,
};
pub use risk_budget::{
    evaluate_network_risk_budget_threshold, NetworkInterventionState,
    NetworkRiskBudgetAdapterProofState, NetworkRiskBudgetAgeBand, NetworkRiskBudgetEvaluation,
    NetworkRiskBudgetEvidenceTier, NetworkRiskBudgetHouseholdPolicy, NetworkRiskBudgetPriorEvent,
    NetworkRiskBudgetSignal, NetworkRiskBudgetState, NetworkRiskBudgetThresholdError,
    NetworkRiskBudgetThresholdInput, NetworkRiskBudgetThresholds,
};
pub use screen_summary::{
    plan_network_screen_summary_trigger, NetworkScreenSummaryPrivacyMode,
    NetworkScreenSummaryTriggerError, NetworkScreenSummaryTriggerInput,
    NetworkScreenSummaryTriggerJob, NetworkScreenSummaryTriggerPlan,
    NetworkScreenSummaryTriggerStatus,
};
pub use signature_alert::{
    ingest_network_signature_alerts, NetworkAnalyzerAlertRecord, NetworkSignatureAlertFixtureRow,
    NetworkSignatureAlertIngestionError, NetworkSignatureAlertIngestionInput,
    NetworkSignatureAlertIngestionProof, NetworkSignatureAlertSeverity,
    NetworkSignatureAlertSource, NetworkSignatureAlertState,
};
pub use tls::{parse_tls_client_hello_sni, TlsClientHelloError, TlsClientHelloVisibility};
pub use transfer::{
    classify_remote_torrent_download_activity, NetworkTransferActivityKind, NetworkTransferBasis,
    NetworkTransferClassification, NetworkTransferClassifierError, NetworkTransferClassifierInput,
    NetworkTransferIndicator, NetworkTransferIndicatorEvidence, NetworkTransferUncertainty,
};
pub use tunnel::{
    classify_vpn_proxy_tunnel_activity, NetworkTunnelBasis, NetworkTunnelClassification,
    NetworkTunnelClassifierError, NetworkTunnelClassifierInput, NetworkTunnelIndicator,
    NetworkTunnelIndicatorEvidence, NetworkTunnelKind,
};
pub use unmanaged_browser::{
    correlate_unmanaged_browser_activity, UnmanagedBrowserCorrelation,
    UnmanagedBrowserCorrelationBasis, UnmanagedBrowserCorrelationError,
    UnmanagedBrowserCorrelationInput, UnmanagedBrowserCorrelationState,
    UnmanagedBrowserProcessKind,
};
pub use windows_firewall_adapter::{
    plan_network_windows_firewall_adapter_proof, NetworkWindowsFirewallAdapterAction,
    NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallAdapterProofError,
    NetworkWindowsFirewallAdapterProofInput, NetworkWindowsFirewallBoundaryReason,
    NetworkWindowsFirewallCapabilityState, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallRequiredArtifact, NetworkWindowsFirewallTargetKind,
};
pub use windows_wfp_gate::{
    plan_network_windows_wfp_gate, NetworkWindowsWfpGateBoundaryReason,
    NetworkWindowsWfpGateCapabilityState, NetworkWindowsWfpGateError, NetworkWindowsWfpGateInput,
    NetworkWindowsWfpGateProof, NetworkWindowsWfpGateState, NetworkWindowsWfpRequiredArtifact,
};
pub use zeek::{
    generate_network_zeek_analyzer_proof, NetworkZeekAnalyzerComparisonArtifact,
    NetworkZeekAnalyzerComparisonState, NetworkZeekAnalyzerError, NetworkZeekAnalyzerInput,
    NetworkZeekAnalyzerProof, NetworkZeekConnectionRow, NetworkZeekDnsRow, NetworkZeekHttpEvidence,
    NetworkZeekHttpRow, NetworkZeekLogKind, NetworkZeekTlsEvidence, NetworkZeekTlsRow,
    NetworkZeekVisibilityState,
};
