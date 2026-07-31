#![forbid(unsafe_code)]

use ed25519_dalek::{SigningKey, VerifyingKey};
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityInput, ParentStepUpValidationInput,
};
use ocentra_family_identity_core::household_authority_proof::HouseholdAuthorityCurrentState;
use ocentra_family_identity_core::parent_step_up_proof::{
    ParentStepUpProofVerifier, VerifiedParentStepUpProof,
};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};

use self::authority::{AuthenticatedDeliveryGrantAuthorityVerifier, SignedAuthorityBindings};
use self::issuance_milestone::EventBusAuthenticatedDeliveryGrantIssuancePublisher;
use crate::policy_authority::PolicyControlDecision;
use crate::policy_contract_helpers::authority::PolicyContractAuthorityDecision;

#[cfg(debug_assertions)]
use std::collections::VecDeque;
#[cfg(debug_assertions)]
use std::sync::{Arc, Mutex};

pub mod authority;
pub mod issuance_milestone;
mod lifecycle;
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
    pub payload_length: usize,
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
    /// Caller context only. Issuance derives its durable audit correlation from
    /// verified authority material and never trusts this value for that chain.
    pub correlation_id: CorrelationId,
    pub household_authority: HouseholdAuthorityInput,
    /// Caller context only. The issuer replaces this with the signed resolved
    /// decision; it is retained for request-shape compatibility.
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
    CorrelationIdRejected,
    MilestonePublicationFailed,
}

pub struct AuthenticatedDeliveryGrantIssuer {
    issuer_key_id: String,
    signing_key: SigningKey,
    authority_verifier: AuthenticatedDeliveryGrantAuthorityVerifier,
    household_authority_current_state: HouseholdAuthorityCurrentState,
    step_up_verifier: ParentStepUpProofVerifier,
    issuance_publisher: Option<EventBusAuthenticatedDeliveryGrantIssuancePublisher>,
    trusted_issuance_now: Option<String>,
    #[cfg(debug_assertions)]
    trusted_issuance_now_sequence: Option<Arc<Mutex<VecDeque<String>>>>,
}

impl AuthenticatedDeliveryGrantIssuer {
    pub fn from_platform_key_with_provenance_verifiers(
        issuer_key_id: impl Into<String>,
        platform_protected_key: [u8; 32],
        authority_key: VerifyingKey,
        household_authority_key: VerifyingKey,
        household_authority_current_state: HouseholdAuthorityCurrentState,
        step_up_key: VerifyingKey,
    ) -> Result<Self, AuthenticatedDeliveryGrantIssuanceError> {
        let issuer_key_id = issuer_key_id.into();
        if issuer_key_id.trim().is_empty()
            || issuer_key_id.len() > AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
        {
            return Err(AuthenticatedDeliveryGrantIssuanceError::InvalidIssuerKeyId);
        }
        let signing_key = SigningKey::from_bytes(&platform_protected_key);
        Ok(Self {
            issuer_key_id,
            signing_key,
            authority_verifier: AuthenticatedDeliveryGrantAuthorityVerifier::new(
                authority_key,
                household_authority_key,
            ),
            household_authority_current_state,
            step_up_verifier: ParentStepUpProofVerifier::new(step_up_key),
            issuance_publisher: None,
            trusted_issuance_now: None,
            #[cfg(debug_assertions)]
            trusted_issuance_now_sequence: None,
        })
    }

    pub fn with_event_bus_issuance_publisher(
        mut self,
        event_bus: EventBus,
    ) -> Result<Self, EventingError> {
        self.issuance_publisher = Some(EventBusAuthenticatedDeliveryGrantIssuancePublisher::new(
            event_bus,
        )?);
        Ok(self)
    }

    pub fn with_household_authority_current_state(
        mut self,
        current_state: HouseholdAuthorityCurrentState,
    ) -> Self {
        self.household_authority_current_state = current_state;
        self
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    #[cfg(debug_assertions)]
    pub fn with_trusted_issuance_now_for_debug_test(
        mut self,
        trusted_now: impl Into<String>,
    ) -> Self {
        self.trusted_issuance_now = Some(trusted_now.into());
        self
    }

    #[cfg(debug_assertions)]
    pub fn with_trusted_issuance_now_sequence_for_debug_test<I, T>(mut self, trusted_now: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.trusted_issuance_now_sequence = Some(Arc::new(Mutex::new(
            trusted_now.into_iter().map(Into::into).collect(),
        )));
        self
    }

    #[cfg(debug_assertions)]
    fn next_trusted_issuance_now_for_debug_test(&self) -> Option<String> {
        self.trusted_issuance_now_sequence
            .as_ref()
            .and_then(|sequence| sequence.lock().ok())
            .and_then(|mut values| values.pop_front())
    }

    #[cfg(not(debug_assertions))]
    fn next_trusted_issuance_now_for_debug_test(&self) -> Option<String> {
        None
    }

    pub fn issue(
        &self,
        request: AuthenticatedDeliveryGrantIssuance<'_>,
    ) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
        let fallback_correlation_id = lifecycle::generated_issuance_correlation_id()?;
        let attempt_id = ocentra_eventing::ids::EventId::generated();
        match self.prepare_issuance(request, fallback_correlation_id) {
            Ok((correlation_id, grant)) => {
                self.finalize_accepted(&correlation_id, &attempt_id, grant)
            }
            Err((correlation_id, error)) => {
                self.finalize_rejected(&correlation_id, &attempt_id, error)
            }
        }
    }

    pub async fn issue_async(
        &self,
        request: AuthenticatedDeliveryGrantIssuance<'_>,
    ) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
        let fallback_correlation_id = lifecycle::generated_issuance_correlation_id()?;
        let attempt_id = ocentra_eventing::ids::EventId::generated();
        match self.prepare_issuance(request, fallback_correlation_id) {
            Ok((correlation_id, grant)) => {
                self.finalize_accepted_async(&correlation_id, &attempt_id, grant)
                    .await
            }
            Err((correlation_id, error)) => {
                self.finalize_rejected_async(&correlation_id, &attempt_id, error)
                    .await
            }
        }
    }
}
