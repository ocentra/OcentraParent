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

use adapter_capability_status::{
    NetworkAdapterCapabilityStatusProof, NetworkAdapterCapabilityStatusState,
};
use android_vpn_service_gate::{
    NetworkAndroidVpnServiceGateProof, NetworkAndroidVpnServiceGateState,
    NetworkAndroidVpnServiceRequiredArtifact,
};
use apple_network_extension_gate::{
    NetworkAppleNetworkExtensionGateProof, NetworkAppleNetworkExtensionGateState,
    NetworkAppleNetworkExtensionPlatform, NetworkAppleNetworkExtensionRequiredArtifact,
};
use category::{CategoryFreshnessState, CategoryMatchKind, DomainCategoryLookup, NetworkCategory};
use dns::types::NetworkEvidenceGrade;
use domain::{normalize_domain_with_public_suffix, DomainNormalizationError, PublicSuffixModel};
use linux_adapter_gate::{
    NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateState, NetworkLinuxAdapterKind,
    NetworkLinuxAdapterRequiredArtifact,
};
use platform_claims::{NetworkPlatformClaimState, NetworkPlatformClaimTarget};
use policy::{
    NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping, NetworkEvidencePolicyMode,
};
use process::{
    NetworkAppInventoryEntry, NetworkProcessAppCorrelationInput, NetworkProcessCorrelationError,
    NetworkProcessSnapshot,
};
use windows_firewall_adapter::{
    NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallRequiredArtifact, NetworkWindowsFirewallTargetKind,
};
use windows_wfp_gate::{
    NetworkWindowsWfpGateProof, NetworkWindowsWfpGateState, NetworkWindowsWfpRequiredArtifact,
};
