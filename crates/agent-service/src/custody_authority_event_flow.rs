//! Typed service boundary from parent delete authority to child-runtime custody publication.

use ocentra_child_runtime::{
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
    runtime_gate_tombstone::ChildRuntimeTombstonePublicationOutcome,
};
use ocentra_eventing::envelope::EventMetadata;
use ocentra_family_identity_core::household_authority::{
    authorize_household_action, HouseholdAuthorityAction, HouseholdAuthorityInput,
    HouseholdAuthorizationFailureReason, HouseholdAuthorizationState,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    LocalPayloadRetentionAction, StorageCustodyAggregateId, StorageCustodyDecisionId,
    StorageCustodyInput, StorageTombstoneState,
};

#[derive(Clone, Debug)]
pub struct AuthorizedCustodyDeleteCommand {
    pub authority: HouseholdAuthorityInput,
    pub parent_presence: ParentPresenceVerificationInput,
    pub aggregate_id: StorageCustodyAggregateId,
    pub decision_id: StorageCustodyDecisionId,
    pub custody_input: StorageCustodyInput,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthorizedCustodyDeleteError {
    WrongAuthorityAction,
    HouseholdAuthorityRejected(HouseholdAuthorizationFailureReason),
    ParentPresenceRejected(ParentPresenceVerificationFailureReason),
    CustodyActionIsNotDelete,
    StorageUnavailable(std::io::ErrorKind),
}

/// Validates the parent owner/delete capability and the action-bound step-up
/// before deriving a custody action. The child-runtime flow then durably writes
/// the intent before attempting the idempotent journal append.
pub async fn publish_authorized_custody_delete(
    flow: &ChildRuntimeTombstoneEventFlow,
    parent_presence: &mut ParentPresenceVerificationPort,
    command: AuthorizedCustodyDeleteCommand,
) -> Result<ChildRuntimeTombstonePublicationOutcome, AuthorizedCustodyDeleteError> {
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        command.aggregate_id.clone(),
        command.decision_id.clone(),
        command.custody_input.clone(),
    ));
    if action.action_plan.local_payload_retention_action != LocalPayloadRetentionAction::Delete
        || action.action_plan.tombstone_state != StorageTombstoneState::Write
    {
        return Err(AuthorizedCustodyDeleteError::CustodyActionIsNotDelete);
    }
    validate_authority(&command, parent_presence)?;
    flow.publish_action(action, command.metadata)
        .await
        .map_err(|error| AuthorizedCustodyDeleteError::StorageUnavailable(error.kind()))
}

fn validate_authority(
    command: &AuthorizedCustodyDeleteCommand,
    parent_presence: &mut ParentPresenceVerificationPort,
) -> Result<(), AuthorizedCustodyDeleteError> {
    if command.authority.action != HouseholdAuthorityAction::ExportDeleteData
        || command.parent_presence.assertion.action != HouseholdAuthorityAction::ExportDeleteData
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
    parent_presence
        .verify_and_consume(command.parent_presence.clone())
        .map_err(AuthorizedCustodyDeleteError::ParentPresenceRejected)?;
    Ok(())
}
