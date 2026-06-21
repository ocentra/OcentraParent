use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RuntimeRole, SchemaVersion};
use serde::{Deserialize, Serialize};

use crate::constants;

pub mod action_handoff;
pub mod action_handoff_child_status;
pub mod action_handoff_durable;
pub mod action_status;
pub mod delivery;
pub mod social_parent_surface_status_handoff;
pub mod social_provider_receipt;
pub mod social_provider_receipt_durable;
pub mod social_report_writer_delivery_handoff;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserFamily {
    #[serde(rename = "edge")]
    Edge,
    #[serde(rename = "chrome")]
    Chrome,
    #[serde(rename = "brave")]
    Brave,
    #[serde(rename = "firefox")]
    Firefox,
    #[serde(rename = "opera")]
    Opera,
    #[serde(rename = "unknown-chromium")]
    UnknownChromium,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserFamily {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Edge => constants::browser::FAMILY_EDGE,
            Self::Chrome => constants::browser::FAMILY_CHROME,
            Self::Brave => constants::browser::FAMILY_BRAVE,
            Self::Firefox => constants::browser::FAMILY_FIREFOX,
            Self::Opera => constants::browser::FAMILY_OPERA,
            Self::UnknownChromium => constants::browser::FAMILY_UNKNOWN_CHROMIUM,
            Self::Unknown => constants::browser::FAMILY_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserChannel {
    #[serde(rename = "stable")]
    Stable,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "dev")]
    Dev,
    #[serde(rename = "canary")]
    Canary,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserChannel {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Stable => constants::browser::CHANNEL_STABLE,
            Self::Beta => constants::browser::CHANNEL_BETA,
            Self::Dev => constants::browser::CHANNEL_DEV,
            Self::Canary => constants::browser::CHANNEL_CANARY,
            Self::Unknown => constants::browser::CHANNEL_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserActiveTabState {
    #[serde(rename = "known-active")]
    KnownActive,
    #[serde(rename = "known-inactive")]
    KnownInactive,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserActiveTabState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::KnownActive => constants::browser::ACTIVE_STATE_KNOWN_ACTIVE,
            Self::KnownInactive => constants::browser::ACTIVE_STATE_KNOWN_INACTIVE,
            Self::Unknown => constants::browser::ACTIVE_STATE_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserActiveProofSource {
    #[serde(rename = "target-list-only")]
    TargetListOnly,
    #[serde(rename = "cdp-focus-activation")]
    CdpFocusActivation,
    #[serde(rename = "managed-extension-event")]
    ManagedExtensionEvent,
    #[serde(rename = "foreground-correlation")]
    ForegroundCorrelation,
    #[serde(rename = "owned-shell-event")]
    OwnedShellEvent,
}

impl BrowserActiveProofSource {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::TargetListOnly => constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY,
            Self::CdpFocusActivation => {
                constants::browser::ACTIVE_PROOF_SOURCE_CDP_FOCUS_ACTIVATION
            }
            Self::ManagedExtensionEvent => {
                constants::browser::ACTIVE_PROOF_SOURCE_MANAGED_EXTENSION_EVENT
            }
            Self::ForegroundCorrelation => {
                constants::browser::ACTIVE_PROOF_SOURCE_FOREGROUND_CORRELATION
            }
            Self::OwnedShellEvent => constants::browser::ACTIVE_PROOF_SOURCE_OWNED_SHELL_EVENT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserCapabilityStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "tab-list-only")]
    TabListOnly,
    #[serde(rename = "unsupported-browser")]
    UnsupportedBrowser,
    #[serde(rename = "unmanaged-browser")]
    UnmanagedBrowser,
    #[serde(rename = "managed-profile-missing")]
    ManagedProfileMissing,
    #[serde(rename = "bridge-missing")]
    BridgeMissing,
    #[serde(rename = "permission-limited")]
    PermissionLimited,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "adapter-error")]
    AdapterError,
    #[serde(rename = "disabled-by-parent")]
    DisabledByParent,
}

impl BrowserCapabilityStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Available => constants::browser::CAPABILITY_STATUS_AVAILABLE,
            Self::TabListOnly => constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY,
            Self::UnsupportedBrowser => constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER,
            Self::UnmanagedBrowser => constants::browser::CAPABILITY_STATUS_UNMANAGED_BROWSER,
            Self::ManagedProfileMissing => {
                constants::browser::CAPABILITY_STATUS_MANAGED_PROFILE_MISSING
            }
            Self::BridgeMissing => constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING,
            Self::PermissionLimited => constants::browser::CAPABILITY_STATUS_PERMISSION_LIMITED,
            Self::Stale => constants::browser::CAPABILITY_STATUS_STALE,
            Self::AdapterError => constants::browser::CAPABILITY_STATUS_ADAPTER_ERROR,
            Self::DisabledByParent => constants::browser::CAPABILITY_STATUS_DISABLED_BY_PARENT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserCustodyLabel {
    #[serde(rename = "child-device-local")]
    ChildDeviceLocal,
    #[serde(rename = "local-network-child-agent")]
    LocalNetworkChildAgent,
    #[serde(rename = "parent-cache")]
    ParentCache,
    #[serde(rename = "parent-owned-export")]
    ParentOwnedExport,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserCustodyLabel {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ChildDeviceLocal => constants::browser::CUSTODY_CHILD_DEVICE_LOCAL,
            Self::LocalNetworkChildAgent => constants::browser::CUSTODY_LOCAL_NETWORK_CHILD_AGENT,
            Self::ParentCache => constants::browser::CUSTODY_PARENT_CACHE,
            Self::ParentOwnedExport => constants::browser::CUSTODY_PARENT_OWNED_EXPORT,
            Self::Unavailable => constants::browser::CUSTODY_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserEvidenceRecentSummary {
    pub schema_version: u16,
    pub returned: u64,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub browser_evidence_id: Option<String>,
    pub source_id: Option<String>,
    pub adapter_id: Option<String>,
    pub managed_browser_session_id: Option<String>,
    pub browser_family: Option<String>,
    pub active_state: Option<String>,
    pub active_proof_source: Option<String>,
    pub url: Option<String>,
    pub origin: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub capability_status: Option<String>,
    pub custody_label: Option<String>,
}

const BROWSER_RUNTIME_PHASES: [BrowserRuntimePhase; 10] = [
    BrowserRuntimePhase::EvidenceObserved,
    BrowserRuntimePhase::EvidenceJournaled,
    BrowserRuntimePhase::AiAnalysisRequested,
    BrowserRuntimePhase::AiAnalysisCompleted,
    BrowserRuntimePhase::PolicyEvaluationRequested,
    BrowserRuntimePhase::PolicyDecisionCompleted,
    BrowserRuntimePhase::InterventionCommandIssued,
    BrowserRuntimePhase::InterventionResultObserved,
    BrowserRuntimePhase::AuditEntryCommitted,
    BrowserRuntimePhase::ReadModelProjected,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserRuntimePhase {
    EvidenceObserved,
    EvidenceJournaled,
    AiAnalysisRequested,
    AiAnalysisCompleted,
    PolicyEvaluationRequested,
    PolicyDecisionCompleted,
    InterventionCommandIssued,
    InterventionResultObserved,
    AuditEntryCommitted,
    ReadModelProjected,
}

impl BrowserRuntimePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &BROWSER_RUNTIME_PHASES
    }

    pub fn event_type(self) -> &'static str {
        match self {
            Self::EvidenceObserved => constants::browser::EVENT_BROWSER_EVIDENCE_OBSERVED,
            Self::EvidenceJournaled => constants::browser::EVENT_BROWSER_EVIDENCE_JOURNALED,
            Self::AiAnalysisRequested => constants::browser::EVENT_BROWSER_AI_ANALYSIS_REQUESTED,
            Self::AiAnalysisCompleted => constants::browser::EVENT_BROWSER_AI_ANALYSIS_COMPLETED,
            Self::PolicyEvaluationRequested => {
                constants::browser::EVENT_BROWSER_POLICY_EVALUATION_REQUESTED
            }
            Self::PolicyDecisionCompleted => {
                constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED
            }
            Self::InterventionCommandIssued => {
                constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED
            }
            Self::InterventionResultObserved => {
                constants::browser::EVENT_BROWSER_INTERVENTION_RESULT_OBSERVED
            }
            Self::AuditEntryCommitted => constants::browser::EVENT_BROWSER_AUDIT_ENTRY_COMMITTED,
            Self::ReadModelProjected => constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED,
        }
    }

    pub fn subscriber_id(self) -> &'static str {
        match self {
            Self::EvidenceObserved => constants::browser::SUBSCRIBER_BROWSER_EVIDENCE_OBSERVER,
            Self::EvidenceJournaled => constants::browser::SUBSCRIBER_BROWSER_EVIDENCE_JOURNAL,
            Self::AiAnalysisRequested => constants::browser::SUBSCRIBER_BROWSER_AI_REQUEST,
            Self::AiAnalysisCompleted => constants::browser::SUBSCRIBER_BROWSER_AI_COMPLETE,
            Self::PolicyEvaluationRequested => {
                constants::browser::SUBSCRIBER_BROWSER_POLICY_REQUEST
            }
            Self::PolicyDecisionCompleted => constants::browser::SUBSCRIBER_BROWSER_POLICY_DECISION,
            Self::InterventionCommandIssued => {
                constants::browser::SUBSCRIBER_BROWSER_INTERVENTION_COMMAND
            }
            Self::InterventionResultObserved => {
                constants::browser::SUBSCRIBER_BROWSER_INTERVENTION_RESULT
            }
            Self::AuditEntryCommitted => constants::browser::SUBSCRIBER_BROWSER_AUDIT_ENTRY,
            Self::ReadModelProjected => constants::browser::SUBSCRIBER_BROWSER_READ_MODEL,
        }
    }

    pub fn target_handler(self) -> &'static str {
        match self {
            Self::EvidenceObserved => constants::browser::TARGET_BROWSER_EVIDENCE_OBSERVER,
            Self::EvidenceJournaled => constants::browser::TARGET_BROWSER_EVIDENCE_JOURNAL,
            Self::AiAnalysisRequested | Self::AiAnalysisCompleted => {
                constants::browser::TARGET_BROWSER_AI_ANALYZER
            }
            Self::PolicyEvaluationRequested | Self::PolicyDecisionCompleted => {
                constants::browser::TARGET_BROWSER_POLICY_ENGINE
            }
            Self::InterventionCommandIssued | Self::InterventionResultObserved => {
                constants::browser::TARGET_BROWSER_INTERVENTION_ADAPTER
            }
            Self::AuditEntryCommitted => constants::browser::TARGET_BROWSER_AUDIT_WRITER,
            Self::ReadModelProjected => constants::browser::TARGET_BROWSER_READ_MODEL,
        }
    }

    pub fn runtime_role(self) -> RuntimeRole {
        let value = match self {
            Self::EvidenceObserved | Self::EvidenceJournaled => {
                constants::eventing_source::ROLE_AGENT
            }
            Self::AiAnalysisRequested | Self::AiAnalysisCompleted => {
                constants::eventing_source::ROLE_ANALYZER
            }
            Self::PolicyEvaluationRequested | Self::PolicyDecisionCompleted => {
                constants::eventing_source::ROLE_DECISION_ENGINE
            }
            Self::InterventionCommandIssued | Self::InterventionResultObserved => {
                constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER
            }
            Self::AuditEntryCommitted => constants::eventing_source::ROLE_AUDIT_WRITER,
            Self::ReadModelProjected => constants::eventing_source::ROLE_READ_MODEL,
        };
        match RuntimeRole::parse(value) {
            Ok(role) => role,
            Err(_) => std::process::abort(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrowserRuntimeEventPayload {
    pub phase: BrowserRuntimePhase,
    pub source_ref: String,
    pub evidence_ref: String,
    pub capability_status: String,
    pub custody_label: String,
    pub query_visibility: String,
    pub degraded_reason: Option<String>,
    pub journal_ref: Option<String>,
    pub ai_request_ref: Option<String>,
    pub ai_analysis_ref: Option<String>,
    pub policy_evaluation_ref: Option<String>,
    pub policy_decision_ref: Option<String>,
    pub policy_preview_id: Option<String>,
    pub action_intent_id: Option<String>,
    pub intervention_command_ref: Option<String>,
    pub intervention_result_ref: Option<String>,
    pub audit_entry_ref: Option<String>,
    pub read_model_ref: Option<String>,
    pub previous_phase_ref: Option<String>,
    pub exact_url_claimed: bool,
    pub ai_authority: bool,
    pub policy_authority: bool,
    pub dry_run: bool,
    pub adapter_dispatch_claimed: bool,
    pub intervention_command_allowed: bool,
    pub observed_at: String,
}

impl DomainEvent for BrowserRuntimeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(browser_runtime_aggregate_key(&self.source_ref))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::browser::IDEMPOTENCY_BROWSER_RUNTIME_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.evidence_ref);
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

fn browser_runtime_aggregate_key(source_ref: &str) -> String {
    let mut value = String::from(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX);
    value.push_str(source_ref);
    value
}
