use crate::policy_constants;
use serde::{Deserialize, Serialize};

pub const POLICY_DRY_RUN_SCHEMA_VERSION: &str = policy_constants::CONTRACT_SCHEMA_VERSION_V0_6;

fn protocol_lookup<T: Copy, const N: usize>(
    value: impl AsRef<str>,
    variants: [(&'static str, T); N],
) -> Option<T> {
    let value = value.as_ref();
    variants
        .into_iter()
        .find_map(|(protocol, variant)| (value == protocol).then_some(variant))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ParentActorRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "guardian")]
    Guardian,
    #[serde(rename = "system")]
    System,
}

impl ParentActorRole {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        policy_constants::ACTOR_ROLE_PARENT,
        policy_constants::ACTOR_ROLE_GUARDIAN,
        policy_constants::ACTOR_ROLE_SYSTEM,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActorReference {
    pub actor_id: String,
    pub role: ParentActorRole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        policy_constants::EVIDENCE_KIND_JOURNAL_EVENT,
        policy_constants::EVIDENCE_KIND_QUERY_STORE_SUMMARY,
        policy_constants::EVIDENCE_KIND_ACTIVITY_EVENT,
        policy_constants::EVIDENCE_KIND_POLICY_DECISION,
        policy_constants::EVIDENCE_KIND_LOCAL_AI_RESULT,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        policy_constants::ACTION_ALLOW,
        policy_constants::ACTION_WARN,
        policy_constants::ACTION_BLOCK,
        policy_constants::ACTION_TIME_LIMIT,
        policy_constants::ACTION_ASK_PARENT,
        policy_constants::ACTION_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 10] = [
        policy_constants::TARGET_TYPE_APP,
        policy_constants::TARGET_TYPE_PROCESS,
        policy_constants::TARGET_TYPE_WINDOW,
        policy_constants::TARGET_TYPE_DOMAIN,
        policy_constants::TARGET_TYPE_SITE,
        policy_constants::TARGET_TYPE_CATEGORY,
        policy_constants::TARGET_TYPE_VIDEO,
        policy_constants::TARGET_TYPE_CHANNEL,
        policy_constants::TARGET_TYPE_ACTIVITY_TYPE,
        policy_constants::TARGET_TYPE_DEVICE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }

    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (policy_constants::TARGET_TYPE_APP, Self::App),
                (policy_constants::TARGET_TYPE_PROCESS, Self::Process),
                (policy_constants::TARGET_TYPE_WINDOW, Self::Window),
                (policy_constants::TARGET_TYPE_DOMAIN, Self::Domain),
                (policy_constants::TARGET_TYPE_SITE, Self::Site),
                (policy_constants::TARGET_TYPE_CATEGORY, Self::Category),
                (policy_constants::TARGET_TYPE_VIDEO, Self::Video),
                (policy_constants::TARGET_TYPE_CHANNEL, Self::Channel),
                (
                    policy_constants::TARGET_TYPE_ACTIVITY_TYPE,
                    Self::ActivityType,
                ),
                (policy_constants::TARGET_TYPE_DEVICE, Self::Device),
            ],
        )
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
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        policy_constants::HANDOFF_NOT_REQUESTED,
        policy_constants::HANDOFF_DISABLED,
        policy_constants::HANDOFF_PENDING,
        policy_constants::HANDOFF_HANDED_OFF,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
