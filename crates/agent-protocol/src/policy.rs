use serde::{Deserialize, Serialize};

#[path = "constants/policy.rs"]
pub mod policy_constants;

pub const POLICY_DRY_RUN_SCHEMA_VERSION: &str = policy_constants::CONTRACT_SCHEMA_VERSION_V0_6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentActorRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "guardian")]
    Guardian,
    #[serde(rename = "system")]
    System,
}

impl ParentActorRole {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Parent => policy_constants::ACTOR_ROLE_PARENT,
            Self::Guardian => policy_constants::ACTOR_ROLE_GUARDIAN,
            Self::System => policy_constants::ACTOR_ROLE_SYSTEM,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActorReference {
    pub actor_id: String,
    pub role: ParentActorRole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentEvidenceReferenceKind {
    #[serde(rename = "journal-event")]
    JournalEvent,
    #[serde(rename = "query-store-summary")]
    QueryStoreSummary,
    #[serde(rename = "activity-event")]
    ActivityEvent,
    #[serde(rename = "policy-decision")]
    PolicyDecision,
    #[serde(rename = "local-ai-result")]
    LocalAiResult,
}

impl ParentEvidenceReferenceKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::JournalEvent => policy_constants::EVIDENCE_KIND_JOURNAL_EVENT,
            Self::QueryStoreSummary => policy_constants::EVIDENCE_KIND_QUERY_STORE_SUMMARY,
            Self::ActivityEvent => policy_constants::EVIDENCE_KIND_ACTIVITY_EVENT,
            Self::PolicyDecision => policy_constants::EVIDENCE_KIND_POLICY_DECISION,
            Self::LocalAiResult => policy_constants::EVIDENCE_KIND_LOCAL_AI_RESULT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentEvidenceReference {
    pub evidence_reference_id: String,
    pub kind: ParentEvidenceReferenceKind,
    pub observed_at: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "time-limit")]
    TimeLimit,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "unknown")]
    Unknown,
}

impl PolicyAction {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Allow => policy_constants::ACTION_ALLOW,
            Self::Warn => policy_constants::ACTION_WARN,
            Self::Block => policy_constants::ACTION_BLOCK,
            Self::TimeLimit => policy_constants::ACTION_TIME_LIMIT,
            Self::AskParent => policy_constants::ACTION_ASK_PARENT,
            Self::Unknown => policy_constants::ACTION_UNKNOWN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyTargetType {
    #[serde(rename = "app")]
    App,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "activity-type")]
    ActivityType,
    #[serde(rename = "device")]
    Device,
}

impl PolicyTargetType {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::App => policy_constants::TARGET_TYPE_APP,
            Self::Process => policy_constants::TARGET_TYPE_PROCESS,
            Self::Window => policy_constants::TARGET_TYPE_WINDOW,
            Self::Domain => policy_constants::TARGET_TYPE_DOMAIN,
            Self::Site => policy_constants::TARGET_TYPE_SITE,
            Self::Category => policy_constants::TARGET_TYPE_CATEGORY,
            Self::Video => policy_constants::TARGET_TYPE_VIDEO,
            Self::Channel => policy_constants::TARGET_TYPE_CHANNEL,
            Self::ActivityType => policy_constants::TARGET_TYPE_ACTIVITY_TYPE,
            Self::Device => policy_constants::TARGET_TYPE_DEVICE,
        }
    }

    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            policy_constants::TARGET_TYPE_APP => Some(Self::App),
            policy_constants::TARGET_TYPE_PROCESS => Some(Self::Process),
            policy_constants::TARGET_TYPE_WINDOW => Some(Self::Window),
            policy_constants::TARGET_TYPE_DOMAIN => Some(Self::Domain),
            policy_constants::TARGET_TYPE_SITE => Some(Self::Site),
            policy_constants::TARGET_TYPE_CATEGORY => Some(Self::Category),
            policy_constants::TARGET_TYPE_VIDEO => Some(Self::Video),
            policy_constants::TARGET_TYPE_CHANNEL => Some(Self::Channel),
            policy_constants::TARGET_TYPE_ACTIVITY_TYPE => Some(Self::ActivityType),
            policy_constants::TARGET_TYPE_DEVICE => Some(Self::Device),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyTarget {
    pub target_id: String,
    pub target_type: PolicyTargetType,
    pub target_value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRule {
    pub rule_id: String,
    pub target: PolicyTarget,
    pub action: PolicyAction,
    pub schedule_id: Option<String>,
    pub priority: i64,
    pub reason_code: String,
    pub created_by: ParentActorReference,
    pub enabled: bool,
    pub effective_from: Option<String>,
    pub effective_until: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDecisionHandoffState {
    #[serde(rename = "not-requested")]
    NotRequested,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "handed-off")]
    HandedOff,
}

impl PolicyDecisionHandoffState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NotRequested => policy_constants::HANDOFF_NOT_REQUESTED,
            Self::Disabled => policy_constants::HANDOFF_DISABLED,
            Self::Pending => policy_constants::HANDOFF_PENDING,
            Self::HandedOff => policy_constants::HANDOFF_HANDED_OFF,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyDecision {
    pub schema_version: String,
    pub decision_id: String,
    pub action: PolicyAction,
    pub reason_codes: Vec<String>,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub rule_ids: Vec<String>,
    pub local_ai_result_id: Option<String>,
    pub dry_run: bool,
    pub enforcement_handoff_state: PolicyDecisionHandoffState,
    pub expires_at: Option<String>,
}
