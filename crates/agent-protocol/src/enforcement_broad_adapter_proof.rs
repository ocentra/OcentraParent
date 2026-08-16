use serde::{Deserialize, Serialize};

use crate::{constants::enforcement_broad_adapter_proof as proof, ParentPlatform};

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08BroadAdapterRuntimeSurface {
    #[serde(rename = "windows-owned-process-and-timer-runtime-boundary")]
    WindowsOwnedProcessAndTimerRuntimeBoundary,
    #[serde(rename = "windows-managed-browser-session-runtime-boundary")]
    WindowsManagedBrowserSessionRuntimeBoundary,
    #[serde(rename = "windows-broad-installed-app-runtime-gate")]
    WindowsBroadInstalledAppRuntimeGate,
    #[serde(rename = "windows-network-domain-runtime-gate")]
    WindowsNetworkDomainRuntimeGate,
    #[serde(rename = "windows-managed-browser-exact-url-runtime-gate")]
    WindowsManagedBrowserExactUrlRuntimeGate,
    #[serde(rename = "windows-unmanaged-browser-exact-evidence-runtime-gap")]
    WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
    #[serde(rename = "linux-host-runtime-unavailable")]
    LinuxHostRuntimeUnavailable,
    #[serde(rename = "macos-host-runtime-manual-gate")]
    MacosHostRuntimeManualGate,
    #[serde(rename = "android-mobile-runtime-manual-gate")]
    AndroidMobileRuntimeManualGate,
    #[serde(rename = "ios-mobile-runtime-manual-gate")]
    IosMobileRuntimeManualGate,
}

impl V08BroadAdapterRuntimeSurface {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::SURFACE_OWNED_PROCESS_TIMER,
                proof::SURFACE_MANAGED_BROWSER_SESSION,
                proof::SURFACE_BROAD_INSTALLED_APP_GATE,
                proof::SURFACE_NETWORK_DOMAIN_GATE,
                proof::SURFACE_MANAGED_EXACT_URL_GATE,
                proof::SURFACE_UNMANAGED_EXACT_EVIDENCE_GAP,
                proof::SURFACE_LINUX_UNAVAILABLE,
                proof::SURFACE_MACOS_MANUAL_GATE,
                proof::SURFACE_ANDROID_MANUAL_GATE,
                proof::SURFACE_IOS_MANUAL_GATE,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08BroadAdapterRuntimeClaimState {
    #[serde(rename = "implemented-boundary")]
    ImplementedBoundary,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

impl V08BroadAdapterRuntimeClaimState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::CLAIM_IMPLEMENTED_BOUNDARY,
                proof::CLAIM_MANUAL_REQUIRED,
                proof::CLAIM_UNAVAILABLE,
                proof::CLAIM_NOT_CLAIMED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08BroadAdapterRuntimeEvidenceState {
    #[serde(rename = "composite-runtime-proof")]
    CompositeRuntimeProof,
    #[serde(rename = "manual-artifact-required")]
    ManualArtifactRequired,
    #[serde(rename = "target-unavailable")]
    TargetUnavailable,
    #[serde(rename = "not-implemented")]
    NotImplemented,
}

impl V08BroadAdapterRuntimeEvidenceState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::EVIDENCE_COMPOSITE_RUNTIME_PROOF,
                proof::EVIDENCE_MANUAL_ARTIFACT_REQUIRED,
                proof::EVIDENCE_TARGET_UNAVAILABLE,
                proof::EVIDENCE_NOT_IMPLEMENTED,
            ]
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08BroadAdapterRuntimeProofEntry {
    pub schema_version: String,
    pub proof_entry_id: String,
    pub runtime_surface: V08BroadAdapterRuntimeSurface,
    pub platform: ParentPlatform,
    pub product_claim_state: V08BroadAdapterRuntimeClaimState,
    pub evidence_state: V08BroadAdapterRuntimeEvidenceState,
    pub source_proof_ids: Vec<String>,
    pub linked_proof_commands: Vec<String>,
    pub linked_proof_artifacts: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub broad_installed_app_blocking_claimed: bool,
    pub network_domain_blocking_claimed: bool,
    pub managed_browser_exact_url_claimed: bool,
    pub unmanaged_browser_exact_evidence_claimed: bool,
    pub unsupported_platform_claimed: bool,
    pub mobile_privilege_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08BroadAdapterRuntimeProofReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<V08BroadAdapterRuntimeProofEntry>,
}
