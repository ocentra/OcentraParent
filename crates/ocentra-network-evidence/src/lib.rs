#![allow(
    clippy::map_err_ignore,
    clippy::needless_pass_by_value,
    clippy::redundant_clone
)]

pub mod action_result;
pub mod adapter_capability_status;
pub mod ai_audit;
pub mod ai_detection;
pub mod android_physical_target;
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
pub mod linux_nftables_lab_execution;
pub mod live_capture;
pub mod live_capture_execution;
pub mod local_ai_queue;
pub mod local_platform_probe;
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
pub mod windows_firewall_lab_execution;
pub mod windows_wfp_gate;
pub mod zeek;

mod adapter_capability_status_values;
mod platform_claim_values;
mod process_support;

#[cfg(test)]
mod tests;

use action_result::{
    plan_network_action_result_state, NetworkActionResultAdapterProofState,
    NetworkActionResultCapabilityState, NetworkActionResultError, NetworkActionResultInput,
    NetworkActionResultProof, NetworkActionResultRequestedAction, NetworkActionResultTargetKind,
};
use adapter_capability_status::{
    NetworkAdapterCapabilityStatusProof, NetworkAdapterCapabilityStatusState,
};
use ai_audit::{
    build_network_ai_audit_report, NetworkAiAuditReport, NetworkAiAuditReportError,
    NetworkAiAuditReportInput,
};
use ai_detection::{
    evaluate_network_ai_detection_fixtures, NetworkAiDetectionEvaluationError,
    NetworkAiDetectionEvaluationInput, NetworkAiDetectionEvaluationProof,
    NetworkAiDetectionFixtureCase, NetworkAiDetectionInputKind, NetworkAiDetectionLabel,
    NetworkAiDetectionResult, NetworkAiDetectionRiskLevel, NetworkAiDetectionUncertaintyCode,
};
use android_vpn_service_gate::{
    NetworkAndroidVpnServiceGateProof, NetworkAndroidVpnServiceGateState,
    NetworkAndroidVpnServiceRequiredArtifact,
};
use apple_network_extension_gate::{
    NetworkAppleNetworkExtensionGateProof, NetworkAppleNetworkExtensionGateState,
    NetworkAppleNetworkExtensionPlatform, NetworkAppleNetworkExtensionRequiredArtifact,
};
use bundle::{
    build_network_cross_slice_evidence_bundle, NetworkCrossSliceEvidenceBundle,
    NetworkCrossSliceEvidenceBundleError, NetworkCrossSliceEvidenceBundleInput,
    NetworkCrossSliceEvidenceSource,
};
use category::{CategoryFreshnessState, CategoryMatchKind, DomainCategoryLookup, NetworkCategory};
use dns::types::NetworkEvidenceGrade;
use dns_adapter::{
    plan_network_dns_adapter_proof, NetworkDnsAdapterAction, NetworkDnsAdapterCapabilityState,
    NetworkDnsAdapterProof, NetworkDnsAdapterProofError, NetworkDnsAdapterProofInput,
    NetworkDnsAdapterProofState,
};
use domain::{normalize_domain_with_public_suffix, DomainNormalizationError, PublicSuffixModel};
use linux_adapter_gate::{
    NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateState, NetworkLinuxAdapterKind,
    NetworkLinuxAdapterRequiredArtifact,
};
use local_ai_queue::{
    plan_network_local_ai_queue, NetworkLocalAiQueueError, NetworkLocalAiQueueInput,
    NetworkLocalAiQueuePlan,
};
use platform_claims::{
    NetworkPlatformClaimEntry, NetworkPlatformClaimState, NetworkPlatformClaimTarget,
};
use policy::{
    map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
    NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
    NetworkEvidencePolicyMappingInput, NetworkEvidencePolicyMode,
};
use process::{
    NetworkAppInventoryEntry, NetworkProcessAppCorrelationInput, NetworkProcessCorrelationError,
    NetworkProcessSnapshot,
};
use risk_budget::{
    evaluate_network_risk_budget_threshold, NetworkRiskBudgetAdapterProofState,
    NetworkRiskBudgetAgeBand, NetworkRiskBudgetEvaluation, NetworkRiskBudgetEvidenceTier,
    NetworkRiskBudgetHouseholdPolicy, NetworkRiskBudgetPriorEvent, NetworkRiskBudgetSignal,
    NetworkRiskBudgetThresholdError, NetworkRiskBudgetThresholdInput, NetworkRiskBudgetThresholds,
};
use windows_firewall_adapter::{
    NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallRequiredArtifact, NetworkWindowsFirewallTargetKind,
};
use windows_wfp_gate::{
    NetworkWindowsWfpGateProof, NetworkWindowsWfpGateState, NetworkWindowsWfpRequiredArtifact,
};

#[cfg(test)]
use self::{
    action_result::{
        NetworkActionResultBoundaryReason, NetworkActionResultRequiredArtifact,
        NetworkActionResultState,
    },
    adapter_capability_status::{
        build_network_adapter_capability_status, NetworkAdapterCapabilityStatusEntry,
        NetworkAdapterCapabilityStatusError, NetworkAdapterCapabilityStatusInput,
    },
    ai_audit::{
        NetworkAiAuditNarrativeState, NetworkAiAuditRecommendation,
        NetworkAiAuditRecommendationKind, NetworkAiAuditUncertaintyCode,
    },
    ai_detection::{
        NetworkAiDetectionDriftState, NetworkAiDetectionEvaluationState,
        NetworkAiDetectionPrecisionState, NetworkAiDetectionRecallState,
    },
    android_physical_target::prove_network_android_physical_target,
    android_physical_target::types::{
        NetworkAndroidPhysicalTargetBoundaryReason, NetworkAndroidPhysicalTargetError,
        NetworkAndroidPhysicalTargetExpected, NetworkAndroidPhysicalTargetField,
        NetworkAndroidPhysicalTargetInput, NetworkAndroidPhysicalTargetMismatch,
        NetworkAndroidPhysicalTargetObserved, NetworkAndroidPhysicalTargetProof,
        NetworkAndroidPhysicalTargetState, NetworkAndroidPhysicalTargetUnsupportedClaims,
    },
    android_vpn_service_gate::{
        plan_network_android_vpn_service_gate, NetworkAndroidVpnServiceCapabilityState,
        NetworkAndroidVpnServiceGateBoundaryReason, NetworkAndroidVpnServiceGateError,
        NetworkAndroidVpnServiceGateInput,
    },
    app_game_session::{
        correlate_app_game_foreground_session, NetworkAppGameEvidenceKind,
        NetworkAppGameForegroundState, NetworkAppGameSessionCorrelation,
        NetworkAppGameSessionCorrelationBasis, NetworkAppGameSessionCorrelationError,
        NetworkAppGameSessionCorrelationInput, NetworkAppGameSessionCorrelationState,
        NetworkAppGameStoredSessionEvidence,
    },
    apple_network_extension_gate::{
        plan_network_apple_network_extension_gate, NetworkAppleNetworkExtensionCapabilityState,
        NetworkAppleNetworkExtensionGateBoundaryReason, NetworkAppleNetworkExtensionGateError,
        NetworkAppleNetworkExtensionGateInput,
    },
    cascade::{
        route_network_evidence_cascade, NetworkCascadeNextCheck, NetworkCascadeSignalStrength,
        NetworkCascadeSource, NetworkCascadeSourceKind, NetworkEvidenceCascadeDecision,
        NetworkEvidenceCascadeError, NetworkEvidenceCascadeInput,
    },
    category::{
        evaluate_category_source_update, lookup_domain_category, CategorySourceCustody,
        CategoryUpdateDecision, DomainCategoryDatabase, DomainCategoryError, DomainCategoryRecord,
        DomainCategorySource,
    },
    classifier::{
        classify_social_video_game_activity, BrowserClassifierConfirmation, CdnClassifierHint,
        NetworkActivityClassification, NetworkActivityClassifierInput, NetworkClassifierBasis,
        NetworkClassifierError, ProcessClassifierHint,
    },
    dns::{
        message::parse_dns_message,
        replay_dns_observations,
        types::{
            DnsMessage, DnsObservation, DnsQueryType, DnsQuestion, DnsRecordData,
            DnsResourceRecord, NetworkReplayError, NetworkReplaySummary,
        },
    },
    dns_adapter::{NetworkDnsAdapterBoundaryReason, NetworkDnsAdapterRequiredArtifact},
    domain::NormalizedDomainEvidence,
    encrypted_dns::{
        detect_encrypted_dns_candidate, detect_quic_http3_limitation, EncryptedDnsCandidate,
        EncryptedDnsProtocol, QuicVisibilityLimitation,
    },
    fixtures::{
        dns_query_frame_fixture, dns_query_pcap_fixture, dns_query_replay_expected,
        dns_response_payload_fixture, icmp_echo_frame_fixture, tcp_syn_frame_fixture,
        visibility::{
            http_host_request_fixture, quic_initial_payload_fixture,
            tls_client_hello_no_sni_fixture, tls_client_hello_sni_fixture,
        },
    },
    flow::{
        aggregate_network_flows, aggregate_pcap_flows, flow_packets_from_pcap, NetworkFlowError,
        NetworkFlowKey, NetworkFlowPacket, NetworkFlowProtocol, NetworkFlowSession,
        NetworkFlowSummary,
    },
    http::{parse_http_host, HttpHostObservation, HttpHostParseError},
    linux_adapter_gate::{
        plan_network_linux_adapter_gate, NetworkLinuxAdapterCapabilityState,
        NetworkLinuxAdapterGateBoundaryReason, NetworkLinuxAdapterGateError,
        NetworkLinuxAdapterGateInput,
    },
    linux_nftables_lab_execution::{
        prove_network_linux_nftables_lab_execution,
        types::{
            NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
            NetworkLinuxNftablesLabExecutionError, NetworkLinuxNftablesLabExecutionInput,
            NetworkLinuxNftablesLabExecutionProof, NetworkLinuxNftablesLabExecutionState,
            NetworkLinuxNftablesLabUnsupportedClaims,
        },
    },
    live_capture::{
        plan_network_live_capture_proof, NetworkLiveCapturePlatform, NetworkLiveCaptureProof,
        NetworkLiveCaptureProofError, NetworkLiveCaptureProofInput, NetworkLiveCaptureProofState,
        NetworkLiveCaptureRequiredArtifact,
    },
    live_capture_execution::{
        prove_network_live_capture_execution, NetworkLiveCaptureExecutionError,
        NetworkLiveCaptureExecutionInput, NetworkLiveCaptureExecutionProof,
        NetworkLiveCaptureExecutionRequiredArtifact, NetworkLiveCaptureExecutionSource,
        NetworkLiveCaptureExecutionState,
    },
    local_ai_queue::{
        NetworkLocalAiQueueInputKind, NetworkLocalAiQueueJob, NetworkLocalAiQueueStatus,
    },
    local_platform_probe::{
        build_network_local_platform_probe_proof, NetworkLocalPlatformProbeError,
        NetworkLocalPlatformProbeHost, NetworkLocalPlatformProbeInput,
        NetworkLocalPlatformProbeObservation, NetworkLocalPlatformProbeProof,
        NetworkLocalPlatformProbeState, NetworkLocalPlatformProbeUnsupportedClaims,
    },
    managed_browser::{
        correlate_managed_browser_activity, ManagedBrowserCorrelation,
        ManagedBrowserCorrelationBasis, ManagedBrowserCorrelationError,
        ManagedBrowserCorrelationInput, ManagedBrowserCorrelationState, ManagedBrowserPageEvidence,
        NetworkManagedBrowserFlowEvidence,
    },
    notification::{
        map_network_parent_notification_candidate, NetworkParentNotificationCandidate,
        NetworkParentNotificationCandidateError, NetworkParentNotificationCandidateInput,
        NetworkParentNotificationDeliveryState, NetworkParentNotificationSeverity,
    },
    packet::{
        parse_network_packet,
        types::{
            EthernetFrameMetadata, IpProtocol, Ipv4PacketMetadata, PacketParseError,
            ParsedNetworkPacket, TransportPacketMetadata, UdpPayloadView,
        },
        udp_payload_from_ethernet_ipv4,
    },
    pcap::{parse_pcap_packets, PcapPacket, PcapReplayError},
    performance::{
        evaluate_network_performance_benchmark, NetworkPerformanceBenchmarkError,
        NetworkPerformanceBenchmarkInput, NetworkPerformanceBenchmarkProof,
        NetworkPerformanceBenchmarkRow, NetworkPerformanceBenchmarkState,
        NetworkPerformanceBenchmarkThresholds, NetworkPerformancePathState,
        NetworkPerformanceRegressionCode, NetworkPerformanceScenarioType,
    },
    pipeline::{
        prove_network_end_to_end_pipeline, NetworkEndToEndPipelineError,
        NetworkEndToEndPipelineInput, NetworkEndToEndPipelineProof, NetworkEndToEndPipelineRefs,
        NetworkEndToEndUnsupportedClaims, NetworkRetentionDeleteExportProof,
    },
    platform_claims::{
        build_network_platform_claim_manifest, NetworkPlatformClaimManifestError,
        NetworkPlatformClaimManifestInput, NetworkPlatformClaimManifestProof,
        NetworkPlatformClaimManualFollowup, NetworkPlatformClaimProofSource,
        NetworkPlatformUnsupportedClaims,
    },
    process::{
        correlate_process_app_activity, NetworkFlowProcessObservation,
        NetworkProcessAppCorrelation, NetworkProcessCorrelationBasis,
        NetworkProcessCorrelationState, NetworkProcessCorrelationUncertainty,
    },
    raw_capture_storage::{
        plan_network_raw_capture_storage,
        types::{
            NetworkRawCaptureStorageError, NetworkRawCaptureStorageInput,
            NetworkRawCaptureStorageProof, NetworkRawCaptureStorageRequiredArtifact,
            NetworkRawCaptureStorageState,
        },
    },
    readiness::{
        evaluate_network_readiness_proof, NetworkHardeningReadinessProof,
        NetworkReadinessFindingCode, NetworkReadinessGate, NetworkReadinessProof,
        NetworkReadinessProofError, NetworkReadinessProofInput, NetworkReadinessState,
        NetworkRetentionReadinessProof, NetworkRolloutReadinessProof, NetworkSupportReadinessProof,
    },
    risk_budget::{NetworkInterventionState, NetworkRiskBudgetState},
    screen_summary::{
        plan_network_screen_summary_trigger, NetworkScreenSummaryPrivacyMode,
        NetworkScreenSummaryTriggerError, NetworkScreenSummaryTriggerInput,
        NetworkScreenSummaryTriggerJob, NetworkScreenSummaryTriggerPlan,
        NetworkScreenSummaryTriggerStatus,
    },
    signature_alert::{
        ingest_network_signature_alerts, NetworkAnalyzerAlertRecord,
        NetworkSignatureAlertFixtureRow, NetworkSignatureAlertIngestionError,
        NetworkSignatureAlertIngestionInput, NetworkSignatureAlertIngestionProof,
        NetworkSignatureAlertSeverity, NetworkSignatureAlertSource, NetworkSignatureAlertState,
    },
    tls::{parse_tls_client_hello_sni, TlsClientHelloError, TlsClientHelloVisibility},
    transfer::{
        classify_remote_torrent_download_activity, NetworkTransferActivityKind,
        NetworkTransferBasis, NetworkTransferClassification, NetworkTransferClassifierError,
        NetworkTransferClassifierInput, NetworkTransferIndicator, NetworkTransferIndicatorEvidence,
        NetworkTransferUncertainty,
    },
    tunnel::{
        classify_vpn_proxy_tunnel_activity, NetworkTunnelBasis, NetworkTunnelClassification,
        NetworkTunnelClassifierError, NetworkTunnelClassifierInput, NetworkTunnelIndicator,
        NetworkTunnelIndicatorEvidence, NetworkTunnelKind,
    },
    unmanaged_browser::{
        correlate_unmanaged_browser_activity, UnmanagedBrowserCorrelation,
        UnmanagedBrowserCorrelationBasis, UnmanagedBrowserCorrelationError,
        UnmanagedBrowserCorrelationInput, UnmanagedBrowserCorrelationState,
        UnmanagedBrowserProcessKind,
    },
    windows_firewall_adapter::{
        plan_network_windows_firewall_adapter_proof, NetworkWindowsFirewallAdapterAction,
        NetworkWindowsFirewallAdapterProofError, NetworkWindowsFirewallAdapterProofInput,
        NetworkWindowsFirewallBoundaryReason, NetworkWindowsFirewallCapabilityState,
    },
    windows_firewall_lab_execution::{
        prove_network_windows_firewall_lab_execution,
        types::{
            NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabCommandKind,
            NetworkWindowsFirewallLabExecutionError, NetworkWindowsFirewallLabExecutionInput,
            NetworkWindowsFirewallLabExecutionProof, NetworkWindowsFirewallLabExecutionState,
            NetworkWindowsFirewallLabUnsupportedClaims,
        },
    },
    windows_wfp_gate::{
        plan_network_windows_wfp_gate, NetworkWindowsWfpGateBoundaryReason,
        NetworkWindowsWfpGateCapabilityState, NetworkWindowsWfpGateError,
        NetworkWindowsWfpGateInput,
    },
    zeek::{
        generate_network_zeek_analyzer_proof, NetworkZeekAnalyzerComparisonArtifact,
        NetworkZeekAnalyzerComparisonState, NetworkZeekAnalyzerError, NetworkZeekAnalyzerInput,
        NetworkZeekAnalyzerProof, NetworkZeekConnectionRow, NetworkZeekDnsRow,
        NetworkZeekHttpEvidence, NetworkZeekHttpRow, NetworkZeekLogKind, NetworkZeekTlsEvidence,
        NetworkZeekTlsRow, NetworkZeekVisibilityState,
    },
};
