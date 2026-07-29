//! Typed service boundary from parent delete authority to child-runtime custody publication.

use ocentra_child_runtime::{
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
    runtime_gate_tombstone::ChildRuntimeTombstonePublicationOutcome,
};
use ocentra_eventing::envelope::EventMetadata;
use ocentra_family_identity_core::household_authority::{
    authorize_household_action, validate_parent_step_up_assertion, HouseholdAuthorityAction,
    HouseholdAuthorityInput, HouseholdAuthorizationFailureReason, HouseholdAuthorizationState,
    ParentStepUpValidationFailureReason, ParentStepUpValidationInput,
};
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    LocalPayloadRetentionAction, StorageCustodyAggregateId, StorageCustodyDecisionId,
    StorageCustodyInput, StorageTombstoneState,
};

#[derive(Clone, Debug)]
pub struct AuthorizedCustodyDeleteCommand {
    pub authority: HouseholdAuthorityInput,
    pub parent_step_up: ParentStepUpValidationInput,
    pub aggregate_id: StorageCustodyAggregateId,
    pub decision_id: StorageCustodyDecisionId,
    pub custody_input: StorageCustodyInput,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthorizedCustodyDeleteError {
    WrongAuthorityAction,
    HouseholdAuthorityRejected(HouseholdAuthorizationFailureReason),
    ParentStepUpRejected(ParentStepUpValidationFailureReason),
    CustodyActionIsNotDelete,
    StorageUnavailable(std::io::ErrorKind),
}

/// Validates the parent owner/delete capability and the action-bound step-up
/// before deriving a custody action. The child-runtime flow then durably writes
/// the intent before attempting the idempotent journal append.
pub async fn publish_authorized_custody_delete(
    flow: &ChildRuntimeTombstoneEventFlow,
    command: AuthorizedCustodyDeleteCommand,
) -> Result<ChildRuntimeTombstonePublicationOutcome, AuthorizedCustodyDeleteError> {
    validate_authority(&command)?;
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        command.aggregate_id,
        command.decision_id,
        command.custody_input,
    ));
    if action.action_plan.local_payload_retention_action != LocalPayloadRetentionAction::Delete
        || action.action_plan.tombstone_state != StorageTombstoneState::Write
    {
        return Err(AuthorizedCustodyDeleteError::CustodyActionIsNotDelete);
    }
    flow.publish_action(action, command.metadata)
        .await
        .map_err(|error| AuthorizedCustodyDeleteError::StorageUnavailable(error.kind()))
}

fn validate_authority(
    command: &AuthorizedCustodyDeleteCommand,
) -> Result<(), AuthorizedCustodyDeleteError> {
    if command.authority.action != HouseholdAuthorityAction::ExportDeleteData
        || command.parent_step_up.action != HouseholdAuthorityAction::ExportDeleteData
    {
        return Err(AuthorizedCustodyDeleteError::WrongAuthorityAction);
    }
    let authority = authorize_household_action(command.authority);
    if authority.authorization_state != HouseholdAuthorizationState::Authorized {
        let Some(reason) = authority.failure_reason else {
            return Err(AuthorizedCustodyDeleteError::StorageUnavailable(
                std::io::ErrorKind::InvalidData,
            ));
        };
        return Err(AuthorizedCustodyDeleteError::HouseholdAuthorityRejected(
            reason,
        ));
    }
    let step_up = validate_parent_step_up_assertion(&command.parent_step_up);
    if !step_up.valid {
        let Some(reason) = step_up.failure_reason else {
            return Err(AuthorizedCustodyDeleteError::StorageUnavailable(
                std::io::ErrorKind::InvalidData,
            ));
        };
        return Err(AuthorizedCustodyDeleteError::ParentStepUpRejected(reason));
    }
    Ok(())
}
