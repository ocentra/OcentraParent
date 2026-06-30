use ocentra_network_evidence::action_result::{
    plan_network_action_result_state, NetworkActionResultAdapterProofState,
    NetworkActionResultCapabilityState, NetworkActionResultError, NetworkActionResultInput,
    NetworkActionResultRequestedAction, NetworkActionResultTargetKind,
};
use ocentra_network_evidence::adapter_capability_status::{
    NetworkAdapterCapabilityStatusProof, NetworkAdapterCapabilityStatusState,
};
use ocentra_network_evidence::ai_audit::{
    build_network_ai_audit_report, NetworkAiAuditReport, NetworkAiAuditReportError,
    NetworkAiAuditReportInput,
};
use ocentra_network_evidence::ai_detection::{
    evaluate_network_ai_detection_fixtures, NetworkAiDetectionEvaluationError,
    NetworkAiDetectionEvaluationInput, NetworkAiDetectionFixtureCase, NetworkAiDetectionInputKind,
    NetworkAiDetectionLabel, NetworkAiDetectionResult, NetworkAiDetectionRiskLevel,
    NetworkAiDetectionUncertaintyCode,
};
use ocentra_network_evidence::android_vpn_service_gate::{
    NetworkAndroidVpnServiceGateState, NetworkAndroidVpnServiceRequiredArtifact,
};
use ocentra_network_evidence::apple_network_extension_gate::{
    NetworkAppleNetworkExtensionGateState, NetworkAppleNetworkExtensionPlatform,
    NetworkAppleNetworkExtensionRequiredArtifact,
};
use ocentra_network_evidence::bundle::{
    build_network_cross_slice_evidence_bundle, NetworkCrossSliceEvidenceBundle,
    NetworkCrossSliceEvidenceBundleError, NetworkCrossSliceEvidenceBundleInput,
    NetworkCrossSliceEvidenceSource,
};
use ocentra_network_evidence::category::{
    CategoryFreshnessState, CategoryMatchKind, NetworkCategory,
};
use ocentra_network_evidence::dns::types::NetworkEvidenceGrade;
use ocentra_network_evidence::dns_adapter::{
    plan_network_dns_adapter_proof, NetworkDnsAdapterAction, NetworkDnsAdapterCapabilityState,
    NetworkDnsAdapterProofError, NetworkDnsAdapterProofInput, NetworkDnsAdapterProofState,
};
use ocentra_network_evidence::domain::{
    normalize_domain_with_public_suffix, DomainNormalizationError, PublicSuffixModel,
};
use ocentra_network_evidence::linux_adapter_gate::{
    NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateState, NetworkLinuxAdapterKind,
    NetworkLinuxAdapterRequiredArtifact,
};
use ocentra_network_evidence::local_ai_queue::{
    plan_network_local_ai_queue, NetworkLocalAiQueueError, NetworkLocalAiQueueInput,
};
use ocentra_network_evidence::platform_claims::{
    NetworkPlatformClaimEntry, NetworkPlatformClaimState, NetworkPlatformClaimTarget,
};
use ocentra_network_evidence::policy::{
    map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
    NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingError,
    NetworkEvidencePolicyMappingInput, NetworkEvidencePolicyMode,
};
use ocentra_network_evidence::process::{
    NetworkAppInventoryEntry, NetworkProcessAppCorrelationInput, NetworkProcessCorrelationError,
    NetworkProcessSnapshot,
};
use ocentra_network_evidence::risk_budget::{
    evaluate_network_risk_budget_threshold, NetworkRiskBudgetAdapterProofState,
    NetworkRiskBudgetAgeBand, NetworkRiskBudgetEvidenceTier, NetworkRiskBudgetHouseholdPolicy,
    NetworkRiskBudgetPriorEvent, NetworkRiskBudgetSignal, NetworkRiskBudgetThresholdError,
    NetworkRiskBudgetThresholdInput, NetworkRiskBudgetThresholds,
};
use ocentra_network_evidence::windows_firewall_adapter::{
    NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallRequiredArtifact, NetworkWindowsFirewallTargetKind,
};
use ocentra_network_evidence::windows_wfp_gate::{
    NetworkWindowsWfpGateState, NetworkWindowsWfpRequiredArtifact,
};
use ocentra_network_evidence::{
    android_physical_target, android_vpn_service_gate, apple_network_extension_gate, dns,
    linux_adapter_gate, linux_nftables_lab_execution, platform_claims, policy,
    windows_firewall_adapter, windows_firewall_lab_execution, windows_wfp_gate,
};

use ocentra_network_evidence::{
    action_result::{
        NetworkActionResultBoundaryReason, NetworkActionResultRequiredArtifact,
        NetworkActionResultState,
    },
    adapter_capability_status::{
        build_network_adapter_capability_status, NetworkAdapterCapabilityStatusError,
        NetworkAdapterCapabilityStatusInput,
    },
    ai_audit::{
        NetworkAiAuditNarrativeState, NetworkAiAuditRecommendationKind,
        NetworkAiAuditUncertaintyCode,
    },
    ai_detection::{
        NetworkAiDetectionDriftState, NetworkAiDetectionEvaluationState,
        NetworkAiDetectionPrecisionState, NetworkAiDetectionRecallState,
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
        NetworkCascadeSource, NetworkCascadeSourceKind, NetworkEvidenceCascadeError,
        NetworkEvidenceCascadeInput,
    },
    category::{
        evaluate_category_source_update, lookup_domain_category, CategorySourceCustody,
        CategoryUpdateDecision, DomainCategoryDatabase, DomainCategoryRecord, DomainCategorySource,
    },
    classifier::{
        classify_social_video_game_activity, BrowserClassifierConfirmation, CdnClassifierHint,
        NetworkActivityClassifierInput, NetworkClassifierBasis, NetworkClassifierError,
        ProcessClassifierHint,
    },
    dns::{
        message::parse_dns_message,
        replay_dns_observations,
        types::{DnsQueryType, DnsRecordData, NetworkReplayError},
    },
    dns_adapter::{NetworkDnsAdapterBoundaryReason, NetworkDnsAdapterRequiredArtifact},
    encrypted_dns::{
        detect_encrypted_dns_candidate, detect_quic_http3_limitation, EncryptedDnsProtocol,
    },
    fixtures::{
        dns_query_frame_fixture, dns_query_pcap_fixture, dns_query_replay_expected,
        dns_response_payload_fixture, icmp_echo_frame_fixture, tcp_syn_frame_fixture,
        visibility::{
            http_host_request_fixture, quic_initial_payload_fixture,
            tls_client_hello_no_sni_fixture, tls_client_hello_sni_fixture,
        },
    },
    flow::{aggregate_network_flows, aggregate_pcap_flows, NetworkFlowPacket, NetworkFlowProtocol},
    http::parse_http_host,
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
        NetworkLiveCaptureExecutionInput, NetworkLiveCaptureExecutionRequiredArtifact,
        NetworkLiveCaptureExecutionSource, NetworkLiveCaptureExecutionState,
    },
    local_ai_queue::{NetworkLocalAiQueueInputKind, NetworkLocalAiQueueStatus},
    local_platform_probe::{
        build_network_local_platform_probe_proof, NetworkLocalPlatformProbeError,
        NetworkLocalPlatformProbeHost, NetworkLocalPlatformProbeInput,
        NetworkLocalPlatformProbeObservation, NetworkLocalPlatformProbeState,
        NetworkLocalPlatformProbeUnsupportedClaims,
    },
    managed_browser::{
        correlate_managed_browser_activity, ManagedBrowserCorrelation,
        ManagedBrowserCorrelationBasis, ManagedBrowserCorrelationError,
        ManagedBrowserCorrelationInput, ManagedBrowserCorrelationState, ManagedBrowserPageEvidence,
        NetworkManagedBrowserFlowEvidence,
    },
    notification::{
        map_network_parent_notification_candidate, NetworkParentNotificationCandidateError,
        NetworkParentNotificationCandidateInput, NetworkParentNotificationDeliveryState,
        NetworkParentNotificationSeverity,
    },
    packet::{
        parse_network_packet,
        types::{IpProtocol, PacketParseError, TransportPacketMetadata},
    },
    pcap::{parse_pcap_packets, PcapReplayError},
    performance::{
        evaluate_network_performance_benchmark, NetworkPerformanceBenchmarkError,
        NetworkPerformanceBenchmarkInput, NetworkPerformanceBenchmarkRow,
        NetworkPerformanceBenchmarkState, NetworkPerformanceBenchmarkThresholds,
        NetworkPerformancePathState, NetworkPerformanceRegressionCode,
        NetworkPerformanceScenarioType,
    },
    pipeline::{
        prove_network_end_to_end_pipeline, NetworkEndToEndPipelineError,
        NetworkEndToEndPipelineInput, NetworkEndToEndPipelineRefs,
        NetworkEndToEndUnsupportedClaims,
    },
    platform_claims::{NetworkPlatformClaimManifestProof, NetworkPlatformClaimManualFollowup},
    process::{
        correlate_process_app_activity, NetworkFlowProcessObservation,
        NetworkProcessAppCorrelation, NetworkProcessCorrelationBasis,
        NetworkProcessCorrelationState, NetworkProcessCorrelationUncertainty,
    },
    raw_capture_storage::{
        plan_network_raw_capture_storage,
        types::{
            NetworkRawCaptureStorageError, NetworkRawCaptureStorageInput,
            NetworkRawCaptureStorageRequiredArtifact, NetworkRawCaptureStorageState,
        },
    },
    readiness::{
        evaluate_network_readiness_proof, NetworkHardeningReadinessProof,
        NetworkReadinessFindingCode, NetworkReadinessGate, NetworkReadinessProofError,
        NetworkReadinessProofInput, NetworkReadinessState, NetworkRetentionReadinessProof,
        NetworkRolloutReadinessProof, NetworkSupportReadinessProof,
    },
    risk_budget::{NetworkInterventionState, NetworkRiskBudgetState},
    screen_summary::{
        plan_network_screen_summary_trigger, NetworkScreenSummaryPrivacyMode,
        NetworkScreenSummaryTriggerError, NetworkScreenSummaryTriggerInput,
        NetworkScreenSummaryTriggerPlan, NetworkScreenSummaryTriggerStatus,
    },
    signature_alert::{
        ingest_network_signature_alerts, NetworkSignatureAlertFixtureRow,
        NetworkSignatureAlertIngestionError, NetworkSignatureAlertIngestionInput,
        NetworkSignatureAlertSeverity, NetworkSignatureAlertSource, NetworkSignatureAlertState,
    },
    tls::parse_tls_client_hello_sni,
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
        NetworkZeekHttpEvidence, NetworkZeekLogKind, NetworkZeekTlsEvidence,
        NetworkZeekVisibilityState,
    },
};

#[path = "unit/mod.rs"]
mod unit_tests;
