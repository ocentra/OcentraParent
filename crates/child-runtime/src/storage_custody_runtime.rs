use ocentra_eventing::envelope::EventMetadata;
use ocentra_family_identity_core::household_authority::HouseholdAuthorityAction;
use ocentra_family_identity_core::household_authority_proof::VerifiedHouseholdAuthority;
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    StorageCustodyAggregateId, StorageCustodyDecisionId, StorageCustodyInput,
    StorageCustodyLocation,
};

use super::{
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
    removal::ChildAgentServiceIdentity,
    runtime_gate_tombstone::ChildRuntimeTombstonePublicationOutcome, ChildAgentServiceError,
};

pub(super) async fn publish_authorized_storage_custody(
    identity: Option<&ChildAgentServiceIdentity>,
    flow: &ChildRuntimeTombstoneEventFlow,
    authority: VerifiedHouseholdAuthority,
    input: StorageCustodyInput,
    metadata: EventMetadata,
) -> Result<ChildRuntimeTombstonePublicationOutcome, ChildAgentServiceError> {
    let identity =
        identity.ok_or_else(|| authorization_error("child service identity is required"))?;
    if !accepts_storage_custody_authority(identity, &authority) {
        return Err(authorization_error(
            "storage custody requires bound ExportDeleteData authority",
        ));
    }
    if input.location != StorageCustodyLocation::ChildDeviceLocal {
        return Err(authorization_error(
            "child runtime custody ingress only accepts child-device-local data",
        ));
    }

    let aggregate_id = StorageCustodyAggregateId::parse(custody_aggregate_ref(identity))
        .map_err(ChildAgentServiceError::Runtime)?;
    let decision_id = StorageCustodyDecisionId::parse(format!(
        "child-storage-custody-decision:{}",
        metadata.correlation_id.as_str()
    ))
    .map_err(ChildAgentServiceError::Runtime)?;
    let decision = storage_custody_decision_recorded_event(aggregate_id, decision_id, input);
    let action = storage_custody_action_planned_event(decision);

    flow.publish_action(action, metadata)
        .await
        .map_err(ChildAgentServiceError::Storage)
}

fn accepts_storage_custody_authority(
    identity: &ChildAgentServiceIdentity,
    authority: &VerifiedHouseholdAuthority,
) -> bool {
    if authority.input().action != HouseholdAuthorityAction::ExportDeleteData {
        return false;
    }
    let Some(binding) = authority.identity_binding() else {
        return false;
    };
    binding.household_id == identity.household_id
        && binding.child_profile_id == identity.child_profile_id
        && binding.target_device_id == identity.target_device_id
}

fn custody_aggregate_ref(identity: &ChildAgentServiceIdentity) -> String {
    format!(
        "child-storage-custody:{}:{}:{}",
        identity.household_id, identity.child_profile_id, identity.target_device_id
    )
}

fn authorization_error(message: &str) -> ChildAgentServiceError {
    ChildAgentServiceError::Storage(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        message,
    ))
}
