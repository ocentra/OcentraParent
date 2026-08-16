use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryInput;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::constants;

#[path = "enforcement_payload/field_access.rs"]
pub(crate) mod field_access;
#[path = "enforcement_payload/parsing.rs"]
pub(crate) mod parsing;
#[path = "enforcement_payload/policy_action.rs"]
pub(crate) mod policy_action;
#[path = "enforcement_payload/policy_target_type.rs"]
pub(crate) mod policy_target_type;
#[path = "enforcement_payload/process_id.rs"]
pub(crate) mod process_id;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementDeviceRefText(pub(crate) String);

impl std::fmt::Display for EnforcementDeviceRefText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementText(pub(crate) String);

impl<T> From<T> for EnforcementText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for EnforcementText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementFieldKey(pub(crate) &'static str);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EnforcementPayloadError {
    CommandPayloadInvalid,
    PolicyVersionRequired,
    ReasonCodeRequired,
    RuleIdRequired,
    MissingEvidence,
    ProcessIdRequired,
    UnsupportedCapability,
}

impl std::fmt::Display for EnforcementPayloadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::CommandPayloadInvalid => {
                constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID
            }
            Self::PolicyVersionRequired => {
                constants::enforcement::REJECTION_POLICY_VERSION_REQUIRED
            }
            Self::ReasonCodeRequired => constants::enforcement::REJECTION_REASON_CODE_REQUIRED,
            Self::RuleIdRequired => constants::enforcement::REJECTION_RULE_ID_REQUIRED,
            Self::MissingEvidence => constants::enforcement::REJECTION_MISSING_EVIDENCE,
            Self::ProcessIdRequired => constants::enforcement::REJECTION_PROCESS_ID_REQUIRED,
            Self::UnsupportedCapability => constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EnforcementCommandPayload {
    pub input: EnforcementBoundaryInput,
    pub process_id: Option<u32>,
    pub device_id: EnforcementDeviceRefText,
    pub platform: String,
    pub source_peer_id: EnforcementText,
    pub target_route: EnforcementText,
}

#[derive(Clone, Debug, PartialEq)]
struct EnforcementPolicyPayload {
    policy_decision_id: String,
    policy_version: String,
    target_id: String,
    target_value: String,
    target_type: PolicyTargetType,
    action: PolicyAction,
    dry_run: bool,
    reason_codes: Vec<String>,
    rule_ids: Vec<String>,
    evidence_references: Vec<ParentEvidenceReference>,
    expires_at: Option<String>,
    local_ai_result_id: Option<String>,
    requested_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EnforcementPayloadIds {
    action_id: String,
    result_id: String,
    audit_event_id: String,
    timer_event_id: String,
    intent_id: String,
    rollback_token: Option<String>,
}

pub(crate) fn parse_enforcement_command_payload(
    command: &ocentra_parent_agent_protocol::transport::AgentCommandEnvelope,
    observed_at: &EnforcementText,
) -> Result<EnforcementCommandPayload, EnforcementPayloadError> {
    parsing::parse_enforcement_command_payload(command, observed_at)
}
