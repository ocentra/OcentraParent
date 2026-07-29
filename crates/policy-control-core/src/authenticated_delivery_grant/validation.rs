use chrono::{DateTime, FixedOffset};
use ocentra_family_identity_core::household_authority::{
    requires_parent_step_up, validate_parent_step_up_assertion, HouseholdAuthorityAction,
};

use super::{
    AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuance,
    AuthenticatedDeliveryGrantIssuanceError, DeliveryGrantBindings, DeliveryGrantCapabilityState,
    DeliveryGrantEvidenceState, AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};
use crate::policy_authority::{
    PolicyActionAuthorizationState, PolicyControlDecision, PolicyEnforcementExecutionState,
    PolicyManualReviewState,
};
use crate::policy_contract_helpers::authority::{
    PolicyContractAuthorityDecision, PolicyContractAuthoritySource, PolicyContractAuthorityState,
};
use ocentra_family_identity_core::household_authority::{
    authorize_household_action, HouseholdAuthorizationState,
};

pub(super) fn validate_issuance(
    request: &AuthenticatedDeliveryGrantIssuance<'_>,
    issuer_key_id: &str,
    policy_decision: &PolicyControlDecision,
    policy_authority: &PolicyContractAuthorityDecision,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    validate_household_authority(request)?;
    validate_policy_execution(policy_decision, policy_authority)?;
    validate_execution_constraints(request, policy_decision)?;
    validate_canonical_authorization(request)?;
    validate_parent_step_up(request)?;
    validate_grant_timestamps(&request.bindings)?;
    unsigned_grant(request, issuer_key_id)
        .validate_shape()
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::InvalidBindings)
}

fn validate_household_authority(
    request: &AuthenticatedDeliveryGrantIssuance<'_>,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let authority = authorize_household_action(request.household_authority);
    if authority.authorization_state != HouseholdAuthorizationState::Authorized
        || request.household_authority.action != HouseholdAuthorityAction::ChangePolicy
    {
        return Err(AuthenticatedDeliveryGrantIssuanceError::ParentAuthorityRejected);
    }
    Ok(())
}

fn validate_policy_execution(
    policy_decision: &PolicyControlDecision,
    policy_authority: &PolicyContractAuthorityDecision,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    if policy_decision.action_authorization_state != PolicyActionAuthorizationState::Authorized
        || policy_decision.enforcement_execution_state
            != PolicyEnforcementExecutionState::MayExecute
        || policy_authority.source != PolicyContractAuthoritySource::ParentPolicy
        || policy_authority.state != PolicyContractAuthorityState::Authorized
    {
        return Err(AuthenticatedDeliveryGrantIssuanceError::PolicyNotExecutable);
    }
    Ok(())
}

fn validate_execution_constraints(
    request: &AuthenticatedDeliveryGrantIssuance<'_>,
    policy_decision: &PolicyControlDecision,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    if policy_decision.manual_review_state != PolicyManualReviewState::NotRequired {
        return Err(AuthenticatedDeliveryGrantIssuanceError::ManualReviewRequired);
    }
    if request.capability_state != DeliveryGrantCapabilityState::Available {
        return Err(AuthenticatedDeliveryGrantIssuanceError::CapabilityUnavailable);
    }
    if request.evidence_state != DeliveryGrantEvidenceState::Stable {
        return Err(AuthenticatedDeliveryGrantIssuanceError::EvidenceNotStable);
    }
    (!request.bindings.dry_run)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::DryRunForbidden)
}

fn unsigned_grant(
    request: &AuthenticatedDeliveryGrantIssuance<'_>,
    issuer_key_id: &str,
) -> AuthenticatedDeliveryGrant {
    AuthenticatedDeliveryGrant {
        schema_version: AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
        issuer_key_id: issuer_key_id.to_owned(),
        issuer_actor_id: request.bindings.issuer_actor_id.clone(),
        household_id: request.bindings.household_id.clone(),
        parent_device_id: request.bindings.parent_device_id.clone(),
        child_profile_id: request.bindings.child_profile_id.clone(),
        target_device_id: request.bindings.target_device_id.clone(),
        policy_decision_id: request.bindings.policy_decision_id.clone(),
        policy_version: request.bindings.policy_version.clone(),
        action_id: request.bindings.action_id.clone(),
        capability_id: request.bindings.capability_id.clone(),
        evidence_digest: request.bindings.evidence_digest.clone(),
        payload_digest: request.bindings.payload_digest.clone(),
        payload_length: request.bindings.payload_length,
        dry_run: request.bindings.dry_run,
        nonce: request.bindings.nonce.clone(),
        issued_at: request.bindings.issued_at.clone(),
        expires_at: request.bindings.expires_at.clone(),
        revocation_version: request.bindings.revocation_version.clone(),
        signature: vec![0; 64],
    }
}

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
        && action_device_child_profile_matches(
            validation.action_device_child_profile_id.as_deref(),
            authorization.child_profile_id.as_str(),
        )
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
    let grant_expires_at = parse_rfc3339(&request.bindings.expires_at)?;
    if grant_expires_at > assertion_expires_at {
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

fn action_device_child_profile_matches(
    action_device_child_profile_id: Option<&str>,
    target_child_profile_id: &str,
) -> bool {
    action_device_child_profile_id.is_none()
        || action_device_child_profile_id == Some(target_child_profile_id)
}

pub(super) fn validate_grant_timestamps(
    bindings: &DeliveryGrantBindings,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let issued_at = parse_rfc3339(&bindings.issued_at)?;
    let expires_at = parse_rfc3339(&bindings.expires_at)?;
    if expires_at <= issued_at || expires_at.timestamp_nanos_opt().is_none() {
        return Err(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp);
    }
    Ok(())
}

pub(super) fn validate_freshness_at(
    bindings: &DeliveryGrantBindings,
    trusted_now: &str,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let trusted_now = parse_rfc3339(trusted_now)?;
    if parse_rfc3339(&bindings.expires_at)? <= trusted_now {
        return Err(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp);
    }
    Ok(())
}

fn parse_rfc3339(
    value: &str,
) -> Result<DateTime<FixedOffset>, AuthenticatedDeliveryGrantIssuanceError> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)
}
