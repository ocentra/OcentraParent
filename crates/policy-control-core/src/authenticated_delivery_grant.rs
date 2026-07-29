#![forbid(unsafe_code)]

use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use ocentra_family_identity_core::household_authority::{
    authorize_household_action, HouseholdAuthorityAction, HouseholdAuthorityInput,
    HouseholdAuthorizationState, ParentStepUpValidationInput,
};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};

use self::authority::{AuthenticatedDeliveryGrantAuthorityVerifier, SignedAuthorityBindings};
use self::step_up::{ParentStepUpProofVerifier, VerifiedParentStepUpProof};
use crate::policy_authority::{
    PolicyActionAuthorizationState, PolicyControlDecision, PolicyEnforcementExecutionState,
    PolicyManualReviewState,
};
use crate::policy_contract_helpers::authority::{
    PolicyContractAuthorityDecision, PolicyContractAuthoritySource, PolicyContractAuthorityState,
};

pub mod authority;
pub mod step_up;
mod validation;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

macro_rules! canonical_grant_identifier {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(String);

        impl $name {
            pub fn parse(
                value: impl Into<String>,
            ) -> Result<Self, AuthenticatedDeliveryGrantIssuanceError> {
                let value = value.into();
                (!value.trim().is_empty())
                    .then_some(Self(value))
                    .ok_or(AuthenticatedDeliveryGrantIssuanceError::InvalidAuthorizationSnapshot)
            }

            fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

canonical_grant_identifier!(GrantIssuerActorId);
canonical_grant_identifier!(GrantHouseholdId);
canonical_grant_identifier!(GrantParentDeviceId);
canonical_grant_identifier!(GrantChildProfileId);
canonical_grant_identifier!(GrantTargetDeviceId);
canonical_grant_identifier!(GrantPolicyDecisionId);
canonical_grant_identifier!(GrantPolicyVersion);
canonical_grant_identifier!(GrantActionId);
canonical_grant_identifier!(GrantCapabilityId);
canonical_grant_identifier!(GrantEvidenceDigest);
canonical_grant_identifier!(GrantPayloadDigest);
canonical_grant_identifier!(GrantNonce);
canonical_grant_identifier!(GrantRevocationVersion);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDeliveryGrantAuthorization {
    pub issuer_actor_id: GrantIssuerActorId,
    pub household_id: GrantHouseholdId,
    pub parent_device_id: GrantParentDeviceId,
    pub child_profile_id: GrantChildProfileId,
    pub target_device_id: GrantTargetDeviceId,
    pub policy_decision_id: GrantPolicyDecisionId,
    pub policy_version: GrantPolicyVersion,
    pub action_id: GrantActionId,
    pub capability_id: GrantCapabilityId,
    pub evidence_digest: GrantEvidenceDigest,
    pub payload_digest: GrantPayloadDigest,
    pub nonce: GrantNonce,
    pub revocation_version: GrantRevocationVersion,
}

#[derive(Debug, Clone)]
pub struct ParentStepUpGrantAuthorization {
    pub validation: ParentStepUpValidationInput,
    pub target_device_id: GrantTargetDeviceId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryGrantCapabilityState {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryGrantEvidenceState {
    Stable,
    Unstable,
}

pub struct AuthenticatedDeliveryGrantIssuance<'a> {
    pub household_authority: HouseholdAuthorityInput,
    pub policy_decision: &'a PolicyControlDecision,
    pub policy_authority: &'a PolicyContractAuthorityDecision,
    pub canonical_authorization: CanonicalDeliveryGrantAuthorization,
    pub parent_step_up: ParentStepUpGrantAuthorization,
    pub capability_state: DeliveryGrantCapabilityState,
    pub evidence_state: DeliveryGrantEvidenceState,
    pub bindings: DeliveryGrantBindings,
    pub signed_authority_bindings: SignedAuthorityBindings,
    pub verified_parent_step_up_proof: VerifiedParentStepUpProof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatedDeliveryGrantIssuanceError {
    InvalidIssuerKeyId,
    ParentAuthorityRejected,
    ParentStepUpRejected,
    PolicyNotExecutable,
    ManualReviewRequired,
    CapabilityUnavailable,
    EvidenceNotStable,
    DryRunForbidden,
    AuthorizationBindingMismatch,
    InvalidAuthorizationSnapshot,
    InvalidTimestamp,
    InvalidBindings,
    AuthorityProvenanceRejected,
}

pub struct AuthenticatedDeliveryGrantIssuer {
    issuer_key_id: String,
    signing_key: SigningKey,
    authority_verifier: AuthenticatedDeliveryGrantAuthorityVerifier,
    step_up_verifier: ParentStepUpProofVerifier,
}

impl AuthenticatedDeliveryGrantIssuer {
    pub fn from_platform_key_with_provenance_verifiers(
        issuer_key_id: impl Into<String>,
        platform_protected_key: [u8; 32],
        authority_key: VerifyingKey,
        step_up_key: VerifyingKey,
    ) -> Result<Self, AuthenticatedDeliveryGrantIssuanceError> {
        let issuer_key_id = issuer_key_id.into();
        if issuer_key_id.trim().is_empty() {
            return Err(AuthenticatedDeliveryGrantIssuanceError::InvalidIssuerKeyId);
        }
        let signing_key = SigningKey::from_bytes(&platform_protected_key);
        Ok(Self {
            issuer_key_id,
            signing_key,
            authority_verifier: AuthenticatedDeliveryGrantAuthorityVerifier::new(authority_key),
            step_up_verifier: ParentStepUpProofVerifier::new(step_up_key),
        })
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn issue(
        &self,
        request: AuthenticatedDeliveryGrantIssuance<'_>,
    ) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
        let mut request = request;
        request.bindings = self
            .authority_verifier
            .verify(&request.signed_authority_bindings)?;
        request.parent_step_up.validation = self
            .step_up_verifier
            .verify(&request.verified_parent_step_up_proof)?;
        validate_issuance(&request, &self.issuer_key_id)?;
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
    issuer_key_id: &str,
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
    if request.policy_decision.manual_review_state != PolicyManualReviewState::NotRequired {
        return Err(AuthenticatedDeliveryGrantIssuanceError::ManualReviewRequired);
    }
    if request.capability_state != DeliveryGrantCapabilityState::Available {
        return Err(AuthenticatedDeliveryGrantIssuanceError::CapabilityUnavailable);
    }
    if request.evidence_state != DeliveryGrantEvidenceState::Stable {
        return Err(AuthenticatedDeliveryGrantIssuanceError::EvidenceNotStable);
    }
    if request.bindings.dry_run {
        return Err(AuthenticatedDeliveryGrantIssuanceError::DryRunForbidden);
    }
    validation::validate_canonical_authorization(request)?;
    validation::validate_parent_step_up(request)?;
    validation::validate_grant_timestamps(&request.bindings)?;
    let unsigned = AuthenticatedDeliveryGrant {
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
