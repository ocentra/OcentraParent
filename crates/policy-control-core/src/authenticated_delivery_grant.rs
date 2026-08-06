#![forbid(unsafe_code)]

use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use ocentra_family_identity_core::household_authority::{
    authorize_household_action, HouseholdAuthorityAction, HouseholdAuthorityInput,
    HouseholdAuthorizationState,
};
use ocentra_parent_agent_protocol::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};

use crate::policy_authority::{
    PolicyActionAuthorizationState, PolicyControlDecision, PolicyEnforcementExecutionState,
};
use crate::policy_contract_helpers::authority::{
    PolicyContractAuthorityDecision, PolicyContractAuthoritySource, PolicyContractAuthorityState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryGrantBindings {
    pub issuer_actor_id: String,
    pub household_id: String,
    pub parent_device_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
    pub policy_decision_id: String,
    pub policy_version: String,
    pub action_id: String,
    pub capability_id: String,
    pub evidence_digest: String,
    pub payload_digest: String,
    pub dry_run: bool,
    pub nonce: String,
    pub issued_at: String,
    pub expires_at: String,
    pub revocation_version: String,
}

pub struct AuthenticatedDeliveryGrantIssuance<'a> {
    pub household_authority: HouseholdAuthorityInput,
    pub policy_decision: &'a PolicyControlDecision,
    pub policy_authority: &'a PolicyContractAuthorityDecision,
    pub capability_available: bool,
    pub evidence_stable: bool,
    pub bindings: DeliveryGrantBindings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatedDeliveryGrantIssuanceError {
    InvalidIssuerKeyId,
    ParentAuthorityRejected,
    PolicyNotExecutable,
    CapabilityUnavailable,
    EvidenceNotStable,
    DryRunForbidden,
    InvalidBindings,
}

pub struct AuthenticatedDeliveryGrantIssuer {
    issuer_key_id: String,
    signing_key: SigningKey,
}

impl AuthenticatedDeliveryGrantIssuer {
    pub fn from_platform_key(
        issuer_key_id: impl Into<String>,
        platform_protected_key: [u8; 32],
    ) -> Result<Self, AuthenticatedDeliveryGrantIssuanceError> {
        let issuer_key_id = issuer_key_id.into();
        if issuer_key_id.trim().is_empty() {
            return Err(AuthenticatedDeliveryGrantIssuanceError::InvalidIssuerKeyId);
        }
        Ok(Self {
            issuer_key_id,
            signing_key: SigningKey::from_bytes(&platform_protected_key),
        })
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn issue(
        &self,
        request: AuthenticatedDeliveryGrantIssuance<'_>,
    ) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
        validate_issuance(&request)?;
        let bindings = request.bindings;
        let mut grant = AuthenticatedDeliveryGrant {
            schema_version: AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
            issuer_key_id: self.issuer_key_id.clone(),
            issuer_actor_id: bindings.issuer_actor_id,
            household_id: bindings.household_id,
            parent_device_id: bindings.parent_device_id,
            child_profile_id: bindings.child_profile_id,
            target_device_id: bindings.target_device_id,
            policy_decision_id: bindings.policy_decision_id,
            policy_version: bindings.policy_version,
            action_id: bindings.action_id,
            capability_id: bindings.capability_id,
            evidence_digest: bindings.evidence_digest,
            payload_digest: bindings.payload_digest,
            dry_run: bindings.dry_run,
            nonce: bindings.nonce,
            issued_at: bindings.issued_at,
            expires_at: bindings.expires_at,
            revocation_version: bindings.revocation_version,
            signature: vec![0; 64],
        };
        grant.signature = self
            .signing_key
            .sign(&grant.signing_bytes())
            .to_bytes()
            .to_vec();
        Ok(grant)
    }
}

fn validate_issuance(
    request: &AuthenticatedDeliveryGrantIssuance<'_>,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let authority = authorize_household_action(request.household_authority);
    if authority.authorization_state != HouseholdAuthorizationState::Authorized
        || request.household_authority.action != HouseholdAuthorityAction::ChangePolicy
    {
        return Err(AuthenticatedDeliveryGrantIssuanceError::ParentAuthorityRejected);
    }
    if request.policy_decision.action_authorization_state
        != PolicyActionAuthorizationState::Authorized
        || request.policy_decision.enforcement_execution_state
            != PolicyEnforcementExecutionState::MayExecute
        || request.policy_authority.source != PolicyContractAuthoritySource::ParentPolicy
        || request.policy_authority.state != PolicyContractAuthorityState::Authorized
    {
        return Err(AuthenticatedDeliveryGrantIssuanceError::PolicyNotExecutable);
    }
    if !request.capability_available {
        return Err(AuthenticatedDeliveryGrantIssuanceError::CapabilityUnavailable);
    }
    if !request.evidence_stable {
        return Err(AuthenticatedDeliveryGrantIssuanceError::EvidenceNotStable);
    }
    if request.bindings.dry_run {
        return Err(AuthenticatedDeliveryGrantIssuanceError::DryRunForbidden);
    }
    let unsigned = AuthenticatedDeliveryGrant {
        schema_version: AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
        issuer_key_id: "issuer".to_owned(),
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
        dry_run: request.bindings.dry_run,
        nonce: request.bindings.nonce.clone(),
        issued_at: request.bindings.issued_at.clone(),
        expires_at: request.bindings.expires_at.clone(),
        revocation_version: request.bindings.revocation_version.clone(),
        signature: vec![0; 64],
    };
    unsigned
        .validate_shape()
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::InvalidBindings)
}
