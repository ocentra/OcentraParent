use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterResultCode, EnforcementResultStatus, EnforcementRollbackState,
    EnforcementUnavailableReason,
};

pub(super) struct EnforcementResultParts {
    pub status: EnforcementResultStatus,
    pub adapter_result_code: EnforcementAdapterResultCode,
    pub completed_at: Option<String>,
    pub unavailable_reason: Option<String>,
    pub failed_reason: Option<String>,
    pub rollback_token: Option<String>,
    pub rollback_state: EnforcementRollbackState,
    pub unavailable_status_reason: Option<EnforcementUnavailableReason>,
}
