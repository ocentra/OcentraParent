use chrono::{DateTime, FixedOffset};
use ocentra_family_identity_core::household_authority::{
    requires_parent_step_up, validate_parent_step_up_assertion, HouseholdAuthorityAction,
};

use super::{
    AuthenticatedDeliveryGrantIssuance, AuthenticatedDeliveryGrantIssuanceError,
    DeliveryGrantBindings,
};

pub(super) fn validate_canonical_authorization(
    request: &AuthenticatedDeliveryGrantIssuance<'_>,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let authorization = &request.canonical_authorization;
    let bindings = &request.bindings;
    let matches = authorization.issuer_actor_id.as_str() == bindings.issuer_actor_id
        && authorization.household_id.as_str() == bindings.household_id
        && authorization.parent_device_id.as_str() == bindings.parent_device_id
        && authorization.child_profile_id.as_str() == bindings.child_profile_id
        && authorization.target_device_id.as_str() == bindings.target_device_id
        && authorization.policy_decision_id.as_str() == bindings.policy_decision_id
        && authorization.policy_version.as_str() == bindings.policy_version
        && authorization.action_id.as_str() == bindings.action_id
        && authorization.capability_id.as_str() == bindings.capability_id
        && authorization.evidence_digest.as_str() == bindings.evidence_digest
        && authorization.payload_digest.as_str() == bindings.payload_digest
        && authorization.nonce.as_str() == bindings.nonce
        && authorization.revocation_version.as_str() == bindings.revocation_version;
    matches
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch)
}

pub(super) fn validate_parent_step_up(
    request: &AuthenticatedDeliveryGrantIssuance<'_>,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let authorization = &request.canonical_authorization;
    let step_up = &request.parent_step_up;
    let validation = &step_up.validation;
    let matches_context = requires_parent_step_up(request.household_authority.action)
        && validation.action == HouseholdAuthorityAction::ChangePolicy
        && validation.family_id == authorization.household_id.as_str()
        && validation.parent_account_id == authorization.issuer_actor_id.as_str()
        && validation.action_device_id == authorization.parent_device_id.as_str()
        && validation.action_device_child_profile_id.as_deref()
            == Some(authorization.child_profile_id.as_str())
        && validation.target_child_profile_id.as_deref()
            == Some(authorization.child_profile_id.as_str())
        && validation.expected_nonce.as_deref() == Some(authorization.nonce.as_str())
        && step_up.target_device_id == authorization.target_device_id;
    if !matches_context {
        return Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected);
    }

    let observed_at = parse_rfc3339(&validation.observed_at)?;
    let issued_at = parse_rfc3339(&request.bindings.issued_at)?;
    if observed_at != issued_at {
        return Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected);
    }
    let Some(assertion) = validation.assertion.as_ref() else {
        return Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected);
    };
    let assertion_expires_at = parse_rfc3339(&assertion.expires_at)?;
    if assertion_expires_at <= observed_at {
        return Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected);
    }

    let mut normalized_validation = validation.clone();
    normalized_validation.observed_at = observed_at.to_rfc3339();
    if let Some(normalized_assertion) = normalized_validation.assertion.as_mut() {
        normalized_assertion.expires_at = assertion_expires_at.to_rfc3339();
    }
    validate_parent_step_up_assertion(&normalized_validation)
        .valid
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
}

pub(super) fn validate_grant_timestamps(
    bindings: &DeliveryGrantBindings,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let issued_at = parse_rfc3339(&bindings.issued_at)?;
    let expires_at = parse_rfc3339(&bindings.expires_at)?;
    (expires_at > issued_at)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)
}

fn parse_rfc3339(
    value: &str,
) -> Result<DateTime<FixedOffset>, AuthenticatedDeliveryGrantIssuanceError> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)
}
