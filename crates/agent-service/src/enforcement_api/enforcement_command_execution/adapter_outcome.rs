use ocentra_parent_agent_core::enforcement_adapter::{
    unavailable_adapter_outcome, EnforcementAdapterOutcome, EnforcementUnavailableReason,
};
use ocentra_parent_agent_core::enforcement_boundary::{
    EnforcementAdapterRequest, EnforcementBoundaryInput,
};
use ocentra_parent_agent_protocol::enforcement::{EnforcementAdapterKind, EnforcementMode};

use crate::enforcement_payload::{
    EnforcementCommandPayload, EnforcementPayloadError, EnforcementText,
};

use super::EnforcementCommandExecutionError;

pub(super) fn adapter_outcome_for_request(
    request: &EnforcementCommandPayload,
    action: &ocentra_parent_agent_protocol::enforcement::EnforcementAction,
    adapter_request: Option<&EnforcementAdapterRequest>,
    completed_at: &EnforcementText,
) -> Result<Option<EnforcementAdapterOutcome>, EnforcementCommandExecutionError> {
    match adapter_request {
        Some(adapter_request) => Ok(Some(adapter_outcome_for_kind(
            request,
            action,
            adapter_request.adapter_kind,
            adapter_request.mode,
            completed_at,
        )?)),
        None => Ok(None),
    }
}

fn adapter_outcome_for_kind(
    _request: &EnforcementCommandPayload,
    _action: &ocentra_parent_agent_protocol::enforcement::EnforcementAction,
    adapter_kind: EnforcementAdapterKind,
    mode: EnforcementMode,
    completed_at: &EnforcementText,
) -> Result<EnforcementAdapterOutcome, EnforcementCommandExecutionError> {
    match (adapter_kind, mode) {
        (EnforcementAdapterKind::ProcessControl, EnforcementMode::TerminateProcess) => {
            Ok(unavailable_adapter_outcome(
                EnforcementUnavailableReason::ManualRequired,
                &completed_at.0,
            ))
        }
        _ => Err(EnforcementCommandExecutionError::PayloadRejection(
            EnforcementPayloadError::UnsupportedCapability,
        )),
    }
}

pub(super) fn final_input(
    mut input: EnforcementBoundaryInput,
    adapter_outcome: Option<EnforcementAdapterOutcome>,
    completed_at: &EnforcementText,
) -> EnforcementBoundaryInput {
    input.completed_at = Some(completed_at.0.clone());
    input.adapter_outcome = adapter_outcome;
    input
}
