use crate::household_authority::HouseholdAuthorityAction;
use ocentra_eventing::error::EventingError;

pub(crate) fn observer_permission_failure_reason(
    granted_scopes: &[HouseholdAuthorityAction],
    is_write_blocked: bool,
) -> Option<EventingError> {
    if granted_scopes.is_empty() {
        return Some(EventingError::EmptyValue {
            field: "family_identity.observer_permission.granted_scopes",
        });
    }

    if !is_write_blocked {
        return Some(EventingError::InvalidValue {
            field: "family_identity.observer_permission.is_write_blocked",
            value: String::from("false"),
        });
    }

    if !observer_scopes_allowed(granted_scopes) {
        return Some(EventingError::InvalidValue {
            field: "family_identity.observer_permission.granted_scopes",
            value: granted_scopes
                .iter()
                .map(|scope| format!("{scope:?}"))
                .collect::<Vec<_>>()
                .join(","),
        });
    }

    None
}

pub(crate) fn parent_member_role_failure(
    role: crate::family_identity::HouseholdRole,
) -> Option<EventingError> {
    if matches!(
        role,
        crate::family_identity::HouseholdRole::ParentOwner
            | crate::family_identity::HouseholdRole::CoParentGuardian
            | crate::family_identity::HouseholdRole::Observer
    ) {
        None
    } else {
        Some(EventingError::InvalidValue {
            field: "family_identity.parent_member.role",
            value: format!("{role:?}"),
        })
    }
}

pub(crate) fn device_registration_child_profile_failure(
    device_registration: &crate::family_identity::DeviceRegistration,
    child_profile: &crate::family_identity::ChildProfile,
) -> Option<EventingError> {
    if device_registration.child_id != child_profile.child_id {
        return Some(EventingError::InvalidValue {
            field: "family_identity.device_registration.child_id",
            value: device_registration.child_id.to_string(),
        });
    }

    if device_registration.household_id != child_profile.household_id {
        return Some(EventingError::InvalidValue {
            field: "family_identity.device_registration.household_id",
            value: device_registration.household_id.to_string(),
        });
    }

    None
}

pub(crate) fn parent_controller_lease_reusable_failure(
    lease: &crate::family_identity::ParentControllerLease,
) -> Option<EventingError> {
    if lease.revocation_state != crate::household_authority::ParentControllerLeaseState::Active {
        return Some(EventingError::InvalidValue {
            field: "family_identity.parent_controller_lease.revocation_state",
            value: format!("{:?}", lease.revocation_state),
        });
    }

    None
}

fn observer_scopes_allowed(granted_scopes: &[HouseholdAuthorityAction]) -> bool {
    granted_scopes.iter().copied().all(observer_scope_allowed)
}

fn observer_scope_allowed(scope: HouseholdAuthorityAction) -> bool {
    matches!(
        scope,
        HouseholdAuthorityAction::ViewChildStatus | HouseholdAuthorityAction::StartRemoteView
    )
}
