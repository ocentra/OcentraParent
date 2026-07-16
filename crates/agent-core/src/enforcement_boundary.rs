use ocentra_parent_agent_protocol::activity::policy::PolicyDecision;
use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementAdapterKind, EnforcementAuditEvent, EnforcementCapabilityStatus,
    EnforcementIntent, EnforcementMode, EnforcementResult, EnforcementTimerEvent,
    EnforcementTimerEventKind,
};

#[path = "enforcement_boundary/enforcement_action.rs"]
mod enforcement_action;
#[path = "enforcement_boundary/enforcement_adapter_kind.rs"]
mod enforcement_adapter_kind;
#[path = "enforcement_boundary/enforcement_adapter_request.rs"]
mod enforcement_adapter_request;
#[path = "enforcement_boundary/enforcement_audit.rs"]
mod enforcement_audit;
mod enforcement_authorization;
#[path = "enforcement_boundary/enforcement_evaluation.rs"]
mod enforcement_evaluation;
#[path = "enforcement_boundary/enforcement_mode.rs"]
mod enforcement_mode;
#[path = "enforcement_boundary/enforcement_result.rs"]
mod enforcement_result;
#[path = "enforcement_boundary/enforcement_result_capability.rs"]
mod enforcement_result_capability;
#[path = "enforcement_boundary/enforcement_result_parts.rs"]
mod enforcement_result_parts;
mod enforcement_timer_event;
mod enforcement_unavailable_status;
#[path = "enforcement_boundary/enforcement_validation.rs"]
mod enforcement_validation;

pub type EnforcementAuthorizationOutcome =
    enforcement_authorization::EnforcementAuthorizationOutcome;

pub fn authorize_enforcement_boundary(
    input: EnforcementBoundaryInput,
) -> Result<EnforcementAuthorizationOutcome, EnforcementBoundaryRejection> {
    enforcement_authorization::authorize_enforcement_boundary(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnforcementBoundaryInput {
    pub intent: EnforcementIntent,
    pub decision: PolicyDecision,
    pub capability: EnforcementCapabilityStatus,
    pub action_id: String,
    pub result_id: String,
    pub audit_event_id: String,
    pub timer_event_id: String,
    pub rollback_token: Option<String>,
    pub policy_version: String,
    pub requested_at: String,
    pub completed_at: Option<String>,
    pub adapter_outcome: Option<super::enforcement_adapter::EnforcementAdapterOutcome>,
    pub timer_event_kind: Option<EnforcementTimerEventKind>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementBoundaryOutcome {
    pub action: EnforcementAction,
    pub result: EnforcementResult,
    pub audit_event: EnforcementAuditEvent,
    pub timer_event: Option<EnforcementTimerEvent>,
    pub adapter_request: Option<EnforcementAdapterRequest>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementAdapterRequest {
    pub action_id: String,
    pub adapter_kind: EnforcementAdapterKind,
    pub mode: EnforcementMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnforcementBoundaryRejection {
    PolicyDecisionIdMismatch,
    PolicyActionMismatch,
    PolicyTargetMismatch,
    MissingPolicyEvidenceReference,
    UnsupportedEnforcementCapability,
    AdapterResultRequired,
}

impl EnforcementBoundaryRejection {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::PolicyDecisionIdMismatch => enforcement_constants::REJECTION_DECISION_ID_MISMATCH,
            Self::PolicyActionMismatch => {
                enforcement_constants::REJECTION_POLICY_ACTION_NOT_ENFORCEABLE
            }
            Self::PolicyTargetMismatch => enforcement_constants::REJECTION_TARGET_MISMATCH,
            Self::MissingPolicyEvidenceReference => {
                enforcement_constants::REJECTION_MISSING_EVIDENCE
            }
            Self::UnsupportedEnforcementCapability => {
                enforcement_constants::REJECTION_UNSUPPORTED_CAPABILITY
            }
            Self::AdapterResultRequired => enforcement_constants::REJECTION_ADAPTER_RESULT_REQUIRED,
        }
    }
}

pub fn evaluate_enforcement_boundary(
    input: EnforcementBoundaryInput,
) -> Result<EnforcementBoundaryOutcome, EnforcementBoundaryRejection> {
    enforcement_evaluation::evaluate_enforcement_boundary(input)
}
