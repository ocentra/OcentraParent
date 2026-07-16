use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RuntimeRole, SchemaVersion};
use serde::{Deserialize, Serialize};

use crate::constants;

pub const BROWSER_EVIDENCE_SCHEMA_VERSION: u16 = crate::BROWSER_EVIDENCE_SCHEMA_VERSION;

pub mod action_handoff;
pub mod action_handoff_child_status;
pub mod action_handoff_durable;
pub mod action_status;
pub mod delivery;
pub mod social_parent_surface_status_handoff;
pub mod social_provider_receipt;
pub mod social_provider_receipt_durable;
pub mod social_report_writer_delivery_handoff;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        constants::browser::FAMILY_EDGE,
        constants::browser::FAMILY_CHROME,
        constants::browser::FAMILY_BRAVE,
        constants::browser::FAMILY_FIREFOX,
        constants::browser::FAMILY_OPERA,
        constants::browser::FAMILY_UNKNOWN_CHROMIUM,
        constants::browser::FAMILY_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::browser::CHANNEL_STABLE,
        constants::browser::CHANNEL_BETA,
        constants::browser::CHANNEL_DEV,
        constants::browser::CHANNEL_CANARY,
        constants::browser::CHANNEL_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserActiveTabState {
    #[serde(rename = "known-active")]
    KnownActive,
    #[serde(rename = "known-inactive")]
    KnownInactive,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserActiveTabState {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::browser::ACTIVE_STATE_KNOWN_ACTIVE,
        constants::browser::ACTIVE_STATE_KNOWN_INACTIVE,
        constants::browser::ACTIVE_STATE_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY,
        constants::browser::ACTIVE_PROOF_SOURCE_CDP_FOCUS_ACTIVATION,
        constants::browser::ACTIVE_PROOF_SOURCE_MANAGED_EXTENSION_EVENT,
        constants::browser::ACTIVE_PROOF_SOURCE_FOREGROUND_CORRELATION,
        constants::browser::ACTIVE_PROOF_SOURCE_OWNED_SHELL_EVENT,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 10] = [
        constants::browser::CAPABILITY_STATUS_AVAILABLE,
        constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY,
        constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER,
        constants::browser::CAPABILITY_STATUS_UNMANAGED_BROWSER,
        constants::browser::CAPABILITY_STATUS_MANAGED_PROFILE_MISSING,
        constants::browser::CAPABILITY_STATUS_BRIDGE_MISSING,
        constants::browser::CAPABILITY_STATUS_PERMISSION_LIMITED,
        constants::browser::CAPABILITY_STATUS_STALE,
        constants::browser::CAPABILITY_STATUS_ADAPTER_ERROR,
        constants::browser::CAPABILITY_STATUS_DISABLED_BY_PARENT,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::browser::CUSTODY_CHILD_DEVICE_LOCAL,
        constants::browser::CUSTODY_LOCAL_NETWORK_CHILD_AGENT,
        constants::browser::CUSTODY_PARENT_CACHE,
        constants::browser::CUSTODY_PARENT_OWNED_EXPORT,
        constants::browser::CUSTODY_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
    const EVENT_TYPES: [&'static str; 10] = [
        constants::browser::EVENT_BROWSER_EVIDENCE_OBSERVED,
        constants::browser::EVENT_BROWSER_EVIDENCE_JOURNALED,
        constants::browser::EVENT_BROWSER_AI_ANALYSIS_REQUESTED,
        constants::browser::EVENT_BROWSER_AI_ANALYSIS_COMPLETED,
        constants::browser::EVENT_BROWSER_POLICY_EVALUATION_REQUESTED,
        constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED,
        constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED,
        constants::browser::EVENT_BROWSER_INTERVENTION_RESULT_OBSERVED,
        constants::browser::EVENT_BROWSER_AUDIT_ENTRY_COMMITTED,
        constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED,
    ];

    const SUBSCRIBERS: [&'static str; 10] = [
        constants::browser::SUBSCRIBER_BROWSER_EVIDENCE_OBSERVER,
        constants::browser::SUBSCRIBER_BROWSER_EVIDENCE_JOURNAL,
        constants::browser::SUBSCRIBER_BROWSER_AI_REQUEST,
        constants::browser::SUBSCRIBER_BROWSER_AI_COMPLETE,
        constants::browser::SUBSCRIBER_BROWSER_POLICY_REQUEST,
        constants::browser::SUBSCRIBER_BROWSER_POLICY_DECISION,
        constants::browser::SUBSCRIBER_BROWSER_INTERVENTION_COMMAND,
        constants::browser::SUBSCRIBER_BROWSER_INTERVENTION_RESULT,
        constants::browser::SUBSCRIBER_BROWSER_AUDIT_ENTRY,
        constants::browser::SUBSCRIBER_BROWSER_READ_MODEL,
    ];

    const TARGET_HANDLERS: [&'static str; 10] = [
        constants::browser::TARGET_BROWSER_EVIDENCE_OBSERVER,
        constants::browser::TARGET_BROWSER_EVIDENCE_JOURNAL,
        constants::browser::TARGET_BROWSER_AI_ANALYZER,
        constants::browser::TARGET_BROWSER_AI_ANALYZER,
        constants::browser::TARGET_BROWSER_POLICY_ENGINE,
        constants::browser::TARGET_BROWSER_POLICY_ENGINE,
        constants::browser::TARGET_BROWSER_INTERVENTION_ADAPTER,
        constants::browser::TARGET_BROWSER_INTERVENTION_ADAPTER,
        constants::browser::TARGET_BROWSER_AUDIT_WRITER,
        constants::browser::TARGET_BROWSER_READ_MODEL,
    ];

    const ROLE_STRINGS: [&'static str; 10] = [
        constants::eventing_source::ROLE_AGENT,
        constants::eventing_source::ROLE_AGENT,
        constants::eventing_source::ROLE_ANALYZER,
        constants::eventing_source::ROLE_ANALYZER,
        constants::eventing_source::ROLE_DECISION_ENGINE,
        constants::eventing_source::ROLE_DECISION_ENGINE,
        constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
        constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
        constants::eventing_source::ROLE_AUDIT_WRITER,
        constants::eventing_source::ROLE_READ_MODEL,
    ];

    pub fn ordered_chain() -> &'static [Self] {
        &BROWSER_RUNTIME_PHASES
    }

    pub fn event_type(self) -> &'static str {
        Self::EVENT_TYPES[self as usize]
    }

    pub fn subscriber_id(self) -> &'static str {
        Self::SUBSCRIBERS[self as usize]
    }

    pub fn target_handler(self) -> &'static str {
        Self::TARGET_HANDLERS[self as usize]
    }

    pub fn runtime_role(self) -> Result<RuntimeRole, EventingError> {
        RuntimeRole::parse(Self::ROLE_STRINGS[self as usize])
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

fn browser_runtime_aggregate_key(source_ref: impl AsRef<str>) -> String {
    let source_ref = source_ref.as_ref();
    let mut value = String::from(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX);
    value.push_str(source_ref);
    value
}
