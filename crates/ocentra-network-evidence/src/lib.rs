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
pub mod parser_policy;
pub mod pcap;
pub mod performance;
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
mod lab_execution_common;
mod platform_claim_values;
mod process_support;

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
    NetworkAiDetectionRiskLevel,
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
use platform_claims::{NetworkPlatformClaimState, NetworkPlatformClaimTarget};
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
