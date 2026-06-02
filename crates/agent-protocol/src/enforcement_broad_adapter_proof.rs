use serde::{Deserialize, Serialize};

use crate::{constants::enforcement_broad_adapter_proof as proof, ParentPlatform};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::WindowsOwnedProcessAndTimerRuntimeBoundary => proof::SURFACE_OWNED_PROCESS_TIMER,
            Self::WindowsManagedBrowserSessionRuntimeBoundary => {
                proof::SURFACE_MANAGED_BROWSER_SESSION
            }
            Self::WindowsBroadInstalledAppRuntimeGate => proof::SURFACE_BROAD_INSTALLED_APP_GATE,
            Self::WindowsNetworkDomainRuntimeGate => proof::SURFACE_NETWORK_DOMAIN_GATE,
            Self::WindowsManagedBrowserExactUrlRuntimeGate => proof::SURFACE_MANAGED_EXACT_URL_GATE,
            Self::WindowsUnmanagedBrowserExactEvidenceRuntimeGap => {
                proof::SURFACE_UNMANAGED_EXACT_EVIDENCE_GAP
            }
            Self::LinuxHostRuntimeUnavailable => proof::SURFACE_LINUX_UNAVAILABLE,
            Self::MacosHostRuntimeManualGate => proof::SURFACE_MACOS_MANUAL_GATE,
            Self::AndroidMobileRuntimeManualGate => proof::SURFACE_ANDROID_MANUAL_GATE,
            Self::IosMobileRuntimeManualGate => proof::SURFACE_IOS_MANUAL_GATE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::ImplementedBoundary => proof::CLAIM_IMPLEMENTED_BOUNDARY,
            Self::ManualRequired => proof::CLAIM_MANUAL_REQUIRED,
            Self::Unavailable => proof::CLAIM_UNAVAILABLE,
            Self::NotClaimed => proof::CLAIM_NOT_CLAIMED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::CompositeRuntimeProof => proof::EVIDENCE_COMPOSITE_RUNTIME_PROOF,
            Self::ManualArtifactRequired => proof::EVIDENCE_MANUAL_ARTIFACT_REQUIRED,
            Self::TargetUnavailable => proof::EVIDENCE_TARGET_UNAVAILABLE,
            Self::NotImplemented => proof::EVIDENCE_NOT_IMPLEMENTED,
        }
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
