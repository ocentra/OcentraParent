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
    ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason,
    ParentPresenceVerificationInput, ParentPresenceVerificationPort,
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
    ConsumedPresenceDoesNotMatchCommand,
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
    let consumed_presence = consume_authority(&command, parent_presence)?;
    persist_consumed_authorized_custody_delete(flow, consumed_presence, command).await
}

/// Internal command-runtime ingress. Callers must supply the opaque receipt
/// returned by the sealed identity port after it atomically consumed the
/// parent-presence challenge; this function is deliberately not a transport
/// endpoint and cannot validate caller-supplied identity fields.
pub async fn persist_consumed_authorized_custody_delete(
    flow: &ChildRuntimeTombstoneEventFlow,
    consumed_presence: ParentPresenceVerificationAccepted,
    command: AuthorizedCustodyDeleteCommand,
) -> Result<ChildRuntimeTombstonePublicationOutcome, AuthorizedCustodyDeleteError> {
    if consumed_presence.assertion_snapshot() != &command.parent_presence.assertion {
        return Err(AuthorizedCustodyDeleteError::ConsumedPresenceDoesNotMatchCommand);
    }
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
    flow.publish_action(action, command.metadata)
        .await
        .map_err(|error| AuthorizedCustodyDeleteError::StorageUnavailable(error.kind()))
}

fn consume_authority(
    command: &AuthorizedCustodyDeleteCommand,
    parent_presence: &mut ParentPresenceVerificationPort,
) -> Result<ParentPresenceVerificationAccepted, AuthorizedCustodyDeleteError> {
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
        .map_err(AuthorizedCustodyDeleteError::ParentPresenceRejected)
}
