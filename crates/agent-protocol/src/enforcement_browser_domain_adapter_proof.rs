use serde::{Deserialize, Serialize};

use crate::{constants::v08_browser_domain_adapter_proof as proof, ParentPlatform};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08BrowserDomainAdapterProofSurface {
    #[serde(rename = "windows-managed-browser-intervention-state")]
    WindowsManagedBrowserInterventionState,
    #[serde(rename = "windows-managed-browser-exact-url-manual")]
    WindowsManagedBrowserExactUrlManual,
    #[serde(rename = "windows-unmanaged-browser-terminate-boundary")]
    WindowsUnmanagedBrowserTerminateBoundary,
    #[serde(rename = "windows-unmanaged-browser-warn-noop")]
    WindowsUnmanagedBrowserWarnNoop,
    #[serde(rename = "windows-unmanaged-browser-exact-evidence-not-claimed")]
    WindowsUnmanagedBrowserExactEvidenceNotClaimed,
    #[serde(rename = "windows-network-domain-filter-manual")]
    WindowsNetworkDomainFilterManual,
    #[serde(rename = "windows-network-domain-adapter-unavailable")]
    WindowsNetworkDomainAdapterUnavailable,
    #[serde(rename = "windows-audit-visibility-boundary")]
    WindowsAuditVisibilityBoundary,
    #[serde(rename = "windows-restart-recovery-visibility-boundary")]
    WindowsRestartRecoveryVisibilityBoundary,
    #[serde(rename = "windows-browser-policy-rollback-visibility")]
    WindowsBrowserPolicyRollbackVisibility,
    #[serde(rename = "linux-browser-domain-adapter-unavailable")]
    LinuxBrowserDomainAdapterUnavailable,
    #[serde(rename = "macos-browser-domain-adapter-unavailable")]
    MacosBrowserDomainAdapterUnavailable,
    #[serde(rename = "android-browser-domain-adapter-manual")]
    AndroidBrowserDomainAdapterManual,
    #[serde(rename = "ios-browser-domain-adapter-manual")]
    IosBrowserDomainAdapterManual,
}

impl V08BrowserDomainAdapterProofSurface {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::WindowsManagedBrowserInterventionState => proof::SURFACE_MANAGED_INTERVENTION,
            Self::WindowsManagedBrowserExactUrlManual => proof::SURFACE_MANAGED_EXACT_URL,
            Self::WindowsUnmanagedBrowserTerminateBoundary => proof::SURFACE_UNMANAGED_TERMINATE,
            Self::WindowsUnmanagedBrowserWarnNoop => proof::SURFACE_UNMANAGED_WARN,
            Self::WindowsUnmanagedBrowserExactEvidenceNotClaimed => {
                proof::SURFACE_UNMANAGED_EXACT_EVIDENCE
            }
            Self::WindowsNetworkDomainFilterManual => proof::SURFACE_NETWORK_FILTER_MANUAL,
            Self::WindowsNetworkDomainAdapterUnavailable => {
                proof::SURFACE_NETWORK_ADAPTER_UNAVAILABLE
            }
            Self::WindowsAuditVisibilityBoundary => proof::SURFACE_AUDIT_VISIBILITY,
            Self::WindowsRestartRecoveryVisibilityBoundary => proof::SURFACE_RESTART_RECOVERY,
            Self::WindowsBrowserPolicyRollbackVisibility => proof::SURFACE_BROWSER_POLICY_ROLLBACK,
            Self::LinuxBrowserDomainAdapterUnavailable => proof::SURFACE_LINUX_ADAPTER,
            Self::MacosBrowserDomainAdapterUnavailable => proof::SURFACE_MACOS_ADAPTER,
            Self::AndroidBrowserDomainAdapterManual => proof::SURFACE_ANDROID_ADAPTER,
            Self::IosBrowserDomainAdapterManual => proof::SURFACE_IOS_ADAPTER,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08BrowserDomainAdapterProofCapabilityName {
    #[serde(rename = "app-time-limit")]
    AppTimeLimit,
    #[serde(rename = "local-storage")]
    LocalStorage,
    #[serde(rename = "managed-browser-control")]
    ManagedBrowserControl,
    #[serde(rename = "network-domain-blocking")]
    NetworkDomainBlocking,
    #[serde(rename = "network-extension")]
    NetworkExtension,
    #[serde(rename = "unmanaged-browser-detection")]
    UnmanagedBrowserDetection,
    #[serde(rename = "vpn-dns-filtering")]
    VpnDnsFiltering,
}

impl V08BrowserDomainAdapterProofCapabilityName {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::AppTimeLimit => proof::CAPABILITY_APP_TIME_LIMIT,
            Self::LocalStorage => proof::CAPABILITY_LOCAL_STORAGE,
            Self::ManagedBrowserControl => proof::CAPABILITY_MANAGED_BROWSER_CONTROL,
            Self::NetworkDomainBlocking => proof::CAPABILITY_NETWORK_DOMAIN_BLOCKING,
            Self::NetworkExtension => proof::CAPABILITY_NETWORK_EXTENSION,
            Self::UnmanagedBrowserDetection => proof::CAPABILITY_UNMANAGED_BROWSER_DETECTION,
            Self::VpnDnsFiltering => proof::CAPABILITY_VPN_DNS_FILTERING,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08BrowserDomainAdapterProofCapabilityStatus {
    #[serde(rename = "implemented")]
    Implemented,
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-implemented")]
    NotImplemented,
}

impl V08BrowserDomainAdapterProofCapabilityStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Implemented => proof::STATUS_IMPLEMENTED,
            Self::Supported => proof::STATUS_SUPPORTED,
            Self::ManualRequired => proof::STATUS_MANUAL_REQUIRED,
            Self::Unavailable => proof::STATUS_UNAVAILABLE,
            Self::NotImplemented => proof::STATUS_NOT_IMPLEMENTED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08BrowserDomainAdapterProofEvidenceKind {
    #[serde(rename = "managed-browser")]
    ManagedBrowser,
    #[serde(rename = "unmanaged-browser")]
    UnmanagedBrowser,
    #[serde(rename = "network-domain")]
    NetworkDomain,
    #[serde(rename = "audit")]
    Audit,
    #[serde(rename = "restart-recovery")]
    RestartRecovery,
    #[serde(rename = "rollback")]
    Rollback,
    #[serde(rename = "unsupported-target")]
    UnsupportedTarget,
}

impl V08BrowserDomainAdapterProofEvidenceKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ManagedBrowser => proof::EVIDENCE_MANAGED_BROWSER,
            Self::UnmanagedBrowser => proof::EVIDENCE_UNMANAGED_BROWSER,
            Self::NetworkDomain => proof::EVIDENCE_NETWORK_DOMAIN,
            Self::Audit => proof::EVIDENCE_AUDIT,
            Self::RestartRecovery => proof::EVIDENCE_RESTART_RECOVERY,
            Self::Rollback => proof::EVIDENCE_ROLLBACK,
            Self::UnsupportedTarget => proof::EVIDENCE_UNSUPPORTED_TARGET,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08BrowserDomainAdapterProofClaimState {
    #[serde(rename = "implemented-boundary")]
    ImplementedBoundary,
    #[serde(rename = "degraded-boundary")]
    DegradedBoundary,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

impl V08BrowserDomainAdapterProofClaimState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ImplementedBoundary => proof::CLAIM_IMPLEMENTED_BOUNDARY,
            Self::DegradedBoundary => proof::CLAIM_DEGRADED_BOUNDARY,
            Self::ManualRequired => proof::CLAIM_MANUAL_REQUIRED,
            Self::Unavailable => proof::CLAIM_UNAVAILABLE,
            Self::NotClaimed => proof::CLAIM_NOT_CLAIMED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08BrowserDomainAdapterExecutionState {
    #[serde(rename = "executes-real-service")]
    ExecutesRealService,
    #[serde(rename = "returns-degraded-noop")]
    ReturnsDegradedNoop,
    #[serde(rename = "returns-manual-required")]
    ReturnsManualRequired,
    #[serde(rename = "returns-unavailable")]
    ReturnsUnavailable,
    #[serde(rename = "not-invoked")]
    NotInvoked,
}

impl V08BrowserDomainAdapterExecutionState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ExecutesRealService => proof::EXECUTES_REAL_SERVICE,
            Self::ReturnsDegradedNoop => proof::RETURNS_DEGRADED_NOOP,
            Self::ReturnsManualRequired => proof::RETURNS_MANUAL_REQUIRED,
            Self::ReturnsUnavailable => proof::RETURNS_UNAVAILABLE,
            Self::NotInvoked => proof::NOT_INVOKED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08WindowsAppControlReadinessState {
    #[serde(rename = "readiness-check")]
    ReadinessCheck,
    #[serde(rename = "audit-only")]
    AuditOnly,
    #[serde(rename = "enforced")]
    Enforced,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "failed")]
    Failed,
}

impl V08WindowsAppControlReadinessState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ReadinessCheck => proof::APP_CONTROL_READINESS_CHECK,
            Self::AuditOnly => proof::APP_CONTROL_AUDIT_ONLY,
            Self::Enforced => proof::APP_CONTROL_ENFORCED,
            Self::ManualRequired => proof::APP_CONTROL_MANUAL_REQUIRED,
            Self::Unavailable => proof::APP_CONTROL_UNAVAILABLE,
            Self::Failed => proof::APP_CONTROL_FAILED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08WindowsAppControlPolicyMutationState {
    #[serde(rename = "detect-only")]
    DetectOnly,
    #[serde(rename = "audit-only-visible")]
    AuditOnlyVisible,
    #[serde(rename = "create-update-manual-required")]
    CreateUpdateManualRequired,
    #[serde(rename = "manual-setup-required")]
    ManualSetupRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "failed")]
    Failed,
}

impl V08WindowsAppControlPolicyMutationState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::DetectOnly => proof::APP_CONTROL_POLICY_DETECT_ONLY,
            Self::AuditOnlyVisible => proof::APP_CONTROL_POLICY_AUDIT_ONLY_VISIBLE,
            Self::CreateUpdateManualRequired => {
                proof::APP_CONTROL_POLICY_CREATE_UPDATE_MANUAL_REQUIRED
            }
            Self::ManualSetupRequired => proof::APP_CONTROL_POLICY_MANUAL_SETUP_REQUIRED,
            Self::Unavailable => proof::APP_CONTROL_POLICY_UNAVAILABLE,
            Self::Failed => proof::APP_CONTROL_POLICY_FAILED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08WindowsAppControlRuleIdentityKind {
    #[serde(rename = "publisher")]
    Publisher,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "hash")]
    Hash,
    #[serde(rename = "package")]
    Package,
}

impl V08WindowsAppControlRuleIdentityKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Publisher => proof::APP_CONTROL_IDENTITY_PUBLISHER,
            Self::Path => proof::APP_CONTROL_IDENTITY_PATH,
            Self::Hash => proof::APP_CONTROL_IDENTITY_HASH,
            Self::Package => proof::APP_CONTROL_IDENTITY_PACKAGE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08WindowsAppControlAdminRequirement {
    #[serde(rename = "administrator-required")]
    AdministratorRequired,
    #[serde(rename = "service-permission-required")]
    ServicePermissionRequired,
    #[serde(rename = "manual-operator-required")]
    ManualOperatorRequired,
    #[serde(rename = "not-applicable")]
    NotApplicable,
}

impl V08WindowsAppControlAdminRequirement {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::AdministratorRequired => proof::APP_CONTROL_ADMINISTRATOR_REQUIRED,
            Self::ServicePermissionRequired => proof::APP_CONTROL_SERVICE_PERMISSION_REQUIRED,
            Self::ManualOperatorRequired => proof::APP_CONTROL_MANUAL_OPERATOR_REQUIRED,
            Self::NotApplicable => proof::APP_CONTROL_ADMIN_NOT_APPLICABLE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08WindowsAppControlEventState {
    #[serde(rename = "audit-visible")]
    AuditVisible,
    #[serde(rename = "rollback-visible")]
    RollbackVisible,
    #[serde(rename = "failure-visible")]
    FailureVisible,
    #[serde(rename = "manual-proof-required")]
    ManualProofRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl V08WindowsAppControlEventState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::AuditVisible => proof::APP_CONTROL_EVENT_AUDIT_VISIBLE,
            Self::RollbackVisible => proof::APP_CONTROL_EVENT_ROLLBACK_VISIBLE,
            Self::FailureVisible => proof::APP_CONTROL_EVENT_FAILURE_VISIBLE,
            Self::ManualProofRequired => proof::APP_CONTROL_EVENT_MANUAL_PROOF_REQUIRED,
            Self::Unavailable => proof::APP_CONTROL_EVENT_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08WindowsAppControlProofState {
    pub proof_state_id: String,
    pub readiness_state: V08WindowsAppControlReadinessState,
    pub policy_mutation_state: V08WindowsAppControlPolicyMutationState,
    pub rule_identity_kinds: Vec<V08WindowsAppControlRuleIdentityKind>,
    pub admin_requirement: V08WindowsAppControlAdminRequirement,
    pub event_states: Vec<V08WindowsAppControlEventState>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub app_control_prevention_claimed: bool,
    pub policy_creation_claimed: bool,
    pub policy_update_claimed: bool,
    pub rollback_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08BrowserDomainAdapterProofEntry {
    pub schema_version: String,
    pub proof_entry_id: String,
    pub surface: V08BrowserDomainAdapterProofSurface,
    pub platform: ParentPlatform,
    pub capability: V08BrowserDomainAdapterProofCapabilityName,
    pub capability_status: V08BrowserDomainAdapterProofCapabilityStatus,
    pub evidence_kind: V08BrowserDomainAdapterProofEvidenceKind,
    pub product_claim_state: V08BrowserDomainAdapterProofClaimState,
    pub adapter_execution_state: V08BrowserDomainAdapterExecutionState,
    pub linked_proof_commands: Vec<String>,
    pub linked_proof_artifacts: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub managed_exact_url_claimed: bool,
    pub unmanaged_exact_url_claimed: bool,
    pub network_domain_blocking_claimed: bool,
    pub broad_browser_control_claimed: bool,
    pub unsupported_os_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08BrowserDomainAdapterProofReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub windows_app_control_states: Vec<V08WindowsAppControlProofState>,
    pub entries: Vec<V08BrowserDomainAdapterProofEntry>,
}
