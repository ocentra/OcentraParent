use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameUnknownCandidateKind {
    NewInventoryApp,
    UnknownProcess,
    PortableExecutable,
    Installer,
    LauncherGameCandidate,
    GameLikeExecutable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameUnknownCandidateSource {
    Inventory,
    Process,
    Foreground,
    Installer,
    Launcher,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameUnknownClassification {
    UnknownApp,
    PossibleGame,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownInventoryCandidateContext {
    pub candidate_id: String,
    pub device_ref: String,
    pub local_user_ref: String,
    pub observed_at_epoch_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownCandidateInput {
    pub candidate_id: String,
    pub subject_ref: String,
    pub device_ref: String,
    pub local_user_ref: String,
    pub kind: AppGameUnknownCandidateKind,
    pub source: AppGameUnknownCandidateSource,
    pub classification: AppGameUnknownClassification,
    pub observed_at_epoch_ms: u64,
    pub evidence_refs: Vec<String>,
    pub category_candidate_ref: Option<String>,
    pub child_status_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownCandidate {
    pub candidate_id: String,
    pub subject_ref: String,
    pub device_ref: String,
    pub local_user_ref: String,
    pub kind: AppGameUnknownCandidateKind,
    pub source: AppGameUnknownCandidateSource,
    pub classification: AppGameUnknownClassification,
    pub observed_at_epoch_ms: u64,
    pub evidence_refs: Vec<String>,
    pub category_candidate_ref: Option<String>,
    pub child_status_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownApprovalRequestInput {
    pub request_id: String,
    pub transition_id: String,
    pub candidate: AppGameUnknownCandidate,
    pub child_reason_refs: Vec<String>,
    pub expires_at_epoch_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownApprovalRequest {
    pub request_id: String,
    pub candidate: AppGameUnknownCandidate,
    pub child_reason_refs: Vec<String>,
    pub expires_at_epoch_ms: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameUnknownParentResponse {
    AllowOnce,
    AllowTarget,
    AllowCategory,
    AskChildWhy,
    Deny,
    BlockIfSupported,
    ReportOnly,
    Override,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameUnknownAdapterCapabilityState {
    Supported,
    Unproven,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownApprovalResponseInput {
    pub transition_id: String,
    pub request_id: String,
    pub actor_ref: String,
    pub response: AppGameUnknownParentResponse,
    pub capability_state: AppGameUnknownAdapterCapabilityState,
    pub evidence_refs: Vec<String>,
    pub child_reason_refs: Vec<String>,
    pub child_status_refs: Vec<String>,
    pub audit_ref: String,
    pub override_ref: Option<String>,
    pub occurred_at_epoch_ms: u64,
    pub decision_expires_at_epoch_ms: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownApprovalExpiryInput {
    pub transition_id: String,
    pub request_id: String,
    pub audit_ref: String,
    pub occurred_at_epoch_ms: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameUnknownApprovalStatus {
    Pending,
    AwaitingChildReason,
    AllowedOnce,
    AllowedTarget,
    AllowedCategory,
    Denied,
    BlockApproved,
    ManualRequired,
    ReportOnly,
    Overridden,
    Expired,
}

impl AppGameUnknownApprovalStatus {
    pub fn accepts_parent_response(self) -> bool {
        matches!(self, Self::Pending | Self::AwaitingChildReason)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameUnknownApprovalPersistenceState {
    Replayable,
    Replayed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameUnknownAdapterDispatchState {
    NotDispatched,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownApprovalSnapshot {
    pub request: AppGameUnknownApprovalRequest,
    pub status: AppGameUnknownApprovalStatus,
    pub response: Option<AppGameUnknownParentResponse>,
    pub actor_ref: Option<String>,
    pub evidence_refs: Vec<String>,
    pub child_reason_refs: Vec<String>,
    pub child_status_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub override_ref: Option<String>,
    pub decision_expires_at_epoch_ms: Option<u64>,
    pub updated_at_epoch_ms: u64,
    pub persistence_state: AppGameUnknownApprovalPersistenceState,
    pub adapter_dispatch_state: AppGameUnknownAdapterDispatchState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameUnknownApprovalWriteReceipt {
    pub sequence: u64,
    pub replayed: bool,
    pub synchronized: bool,
    pub snapshot: AppGameUnknownApprovalSnapshot,
}

#[derive(Debug)]
pub enum AppGameUnknownApprovalError {
    InvalidField { field: &'static str },
    InvalidTransition { reason: &'static str },
    RequestNotFound { request_id: String },
    DuplicateTransition { transition_id: String },
    Journal(EventingError),
}

impl std::fmt::Display for AppGameUnknownApprovalError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidField { field } => {
                write!(formatter, "invalid app/game approval field: {field}")
            }
            Self::InvalidTransition { reason } => {
                write!(formatter, "invalid app/game approval transition: {reason}")
            }
            Self::RequestNotFound { request_id } => {
                write!(
                    formatter,
                    "app/game approval request not found: {request_id}"
                )
            }
            Self::DuplicateTransition { transition_id } => {
                write!(
                    formatter,
                    "app/game approval transition conflicts: {transition_id}"
                )
            }
            Self::Journal(error) => write!(formatter, "app/game approval journal error: {error}"),
        }
    }
}

impl std::error::Error for AppGameUnknownApprovalError {}

impl From<EventingError> for AppGameUnknownApprovalError {
    fn from(error: EventingError) -> Self {
        Self::Journal(error)
    }
}
