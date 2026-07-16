use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_policy_control_core::policy_delivery::{PolicyDeliveryRecord, PolicyDeliveryState};
use ocentra_policy_control_core::policy_request::{ChildPolicyRequest, PolicyTemporaryOverride};

pub fn assert_request_override_shape(
    request: &ChildPolicyRequest,
    temporary_override: Option<&PolicyTemporaryOverride>,
) -> Result<(), EventingError> {
    match request.status {
        PolicyRequestStatus::Approved | PolicyRequestStatus::Modified => {
            assert_resolved_request_override_shape(request, temporary_override)
        }
        PolicyRequestStatus::PreviewOnly
        | PolicyRequestStatus::PendingParentReview
        | PolicyRequestStatus::Denied
        | PolicyRequestStatus::Expired => {
            assert_unresolved_request_override_shape(temporary_override)
        }
        PolicyRequestStatus::ReplayRejected => Err(invalid_request_status_error(
            "policy_request.status",
            request.status,
        )),
    }
}

pub fn assert_request_matches_delivery(
    request: &ChildPolicyRequest,
    delivery: &PolicyDeliveryRecord,
) -> Result<(), EventingError> {
    if !request_accepts_delivery(request.status) {
        return Err(EventingError::InvalidValue {
            field: "policy_control_notification.delivery_state",
            value: request.status.as_protocol_str().to_string(),
        });
    }

    if !delivery_identity_matches_request(request, delivery) {
        return Err(EventingError::InvalidValue {
            field: "policy_control_notification.delivery_id",
            value: delivery.delivery_id.as_str().to_string(),
        });
    }

    if !SUPPORTED_DELIVERY_STATES.contains(&delivery.state) {
        return Err(EventingError::InvalidValue {
            field: "policy_control_notification.delivery_state",
            value: format!("{:?}", delivery.state),
        });
    }

    Ok(())
}

pub fn invalid_request_status_error(
    field: &'static str,
    status: PolicyRequestStatus,
) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: status.as_protocol_str().to_string(),
    }
}

fn assert_resolved_request_override_shape(
    request: &ChildPolicyRequest,
    temporary_override: Option<&PolicyTemporaryOverride>,
) -> Result<(), EventingError> {
    let temporary_override = temporary_override.ok_or_else(|| EventingError::InvalidValue {
        field: "policy_control_notification.override_id",
        value: "missing override for resolved request".to_string(),
    })?;

    if temporary_override.source_request_id != request.request_id {
        return Err(EventingError::InvalidValue {
            field: "policy_control_notification.override_id",
            value: temporary_override.override_id.as_str().to_string(),
        });
    }

    Ok(())
}

fn assert_unresolved_request_override_shape(
    temporary_override: Option<&PolicyTemporaryOverride>,
) -> Result<(), EventingError> {
    if let Some(temporary_override) = temporary_override {
        return Err(EventingError::InvalidValue {
            field: "policy_control_notification.override_id",
            value: temporary_override.override_id.as_str().to_string(),
        });
    }

    Ok(())
}

fn request_accepts_delivery(status: PolicyRequestStatus) -> bool {
    matches!(
        status,
        PolicyRequestStatus::Approved | PolicyRequestStatus::Modified
    )
}

fn delivery_identity_matches_request(
    request: &ChildPolicyRequest,
    delivery: &PolicyDeliveryRecord,
) -> bool {
    request.household_id == delivery.household_id
        && request.policy_version == delivery.policy_version
        && request.source_document_id == delivery.source_document_id
        && request.child_profile_id == delivery.target.child_profile_id
        && request
            .device_id
            .as_ref()
            .is_none_or(|device_id| *device_id == delivery.target.device_id)
}

const SUPPORTED_DELIVERY_STATES: [PolicyDeliveryState; 16] = [
    PolicyDeliveryState::Queued,
    PolicyDeliveryState::Delivering,
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::ExpiredBeforeDelivery,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];
