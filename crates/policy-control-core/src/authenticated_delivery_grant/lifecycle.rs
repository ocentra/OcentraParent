use chrono::{DateTime, FixedOffset, SecondsFormat, Utc};
use ed25519_dalek::Signer;
use ocentra_eventing::ids::{CorrelationId, EventId};
use ocentra_family_identity_core::parent_step_up_proof::{
    authorization_digest, ParentStepUpAuthorizationBinding,
};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantCapabilityAssertion,
    AuthenticatedDeliveryGrantEvidenceAssertion, AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};

use super::issuance_milestone::{
    accepted_issuance_milestone_for, prepared_issuance_milestone_for,
    rejected_issuance_milestone_for, AuthenticatedDeliveryGrantIssuanceMilestone,
};
use super::{
    validation, AuthenticatedDeliveryGrantIssuance, AuthenticatedDeliveryGrantIssuanceError,
    AuthenticatedDeliveryGrantIssuer, DeliveryGrantBindings, DeliveryGrantCapabilityState,
    DeliveryGrantEvidenceState, GrantTargetDeviceId,
};
use crate::policy_contract_helpers::authority::PolicyContractAuthorityDecision;

mod accepted;

const MINIMUM_REMAINING_GRANT_LIFETIME_SECONDS: i64 = 30;

impl AuthenticatedDeliveryGrantIssuer {
    pub(super) fn prepare_issuance(
        &self,
        request: AuthenticatedDeliveryGrantIssuance<'_>,
        fallback_correlation_id: CorrelationId,
    ) -> Result<
        (CorrelationId, AuthenticatedDeliveryGrant),
        (CorrelationId, AuthenticatedDeliveryGrantIssuanceError),
    > {
        let (request, resolved_decision, policy_authority, correlation_id) = self
            .verify_and_bind_request(request)
            .map_err(|error| (fallback_correlation_id, error))?;
        validation::validate_issuance(
            &request,
            &self.issuer_key_id,
            &resolved_decision,
            &policy_authority,
        )
        .map_err(|error| (correlation_id.clone(), error))?;
        validation::validate_freshness_at(&request.bindings, &self.trusted_now())
            .map_err(|error| (correlation_id.clone(), error))?;
        Ok((correlation_id, self.sign_grant(request.bindings)))
    }

    fn verify_and_bind_request<'a>(
        &self,
        mut request: AuthenticatedDeliveryGrantIssuance<'a>,
    ) -> Result<
        (
            AuthenticatedDeliveryGrantIssuance<'a>,
            crate::policy_authority_resolved_decision::ResolvedPolicyDecision,
            PolicyContractAuthorityDecision,
            CorrelationId,
        ),
        AuthenticatedDeliveryGrantIssuanceError,
    > {
        let (bindings, assertions, household_authority, resolved_decision, policy_authority) = self
            .authority_verifier
            .verify(&request.signed_authority_bindings)?;
        let correlation_id = request
            .signed_authority_bindings
            .trusted_issuance_correlation_id()?;
        let (step_up_validation, target_device_id, step_up_assertions) = self
            .step_up_verifier
            .verify(&request.verified_parent_step_up_proof)
            .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)?;
        if assertions != step_up_assertions {
            return Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch);
        }
        let expected_step_up_digest = authorization_digest(ParentStepUpAuthorizationBinding {
            household_id: &bindings.household_id,
            parent_actor_id: &bindings.issuer_actor_id,
            parent_device_id: &bindings.parent_device_id,
            child_profile_id: &bindings.child_profile_id,
            target_device_id: &bindings.target_device_id,
            action_id: &bindings.action_id,
            capability_id: &bindings.capability_id,
            evidence_digest: &bindings.evidence_digest,
            payload_digest: &bindings.payload_digest,
        });
        if request.verified_parent_step_up_proof.authorization_digest != expected_step_up_digest {
            return Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch);
        }
        if resolved_decision.decision_id.as_str() != bindings.policy_decision_id {
            return Err(AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch);
        }
        request.bindings = bindings;
        request.household_authority = household_authority.input();
        request.parent_step_up.validation = step_up_validation;
        request.parent_step_up.target_device_id = GrantTargetDeviceId::parse(target_device_id)
            .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)?;
        request.capability_state = match assertions.capability {
            AuthenticatedDeliveryGrantCapabilityAssertion::Available => {
                DeliveryGrantCapabilityState::Available
            }
            AuthenticatedDeliveryGrantCapabilityAssertion::Unavailable => {
                DeliveryGrantCapabilityState::Unavailable
            }
        };
        request.evidence_state = match assertions.evidence {
            AuthenticatedDeliveryGrantEvidenceAssertion::Stable => {
                DeliveryGrantEvidenceState::Stable
            }
            AuthenticatedDeliveryGrantEvidenceAssertion::Unstable => {
                DeliveryGrantEvidenceState::Unstable
            }
        };
        Ok((request, resolved_decision, policy_authority, correlation_id))
    }

    fn sign_grant(&self, bindings: DeliveryGrantBindings) -> AuthenticatedDeliveryGrant {
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
            payload_length: bindings.payload_length,
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
        grant
    }

    pub(super) fn finalize_accepted(
        &self,
        correlation_id: &CorrelationId,
        attempt_id: &EventId,
        grant: AuthenticatedDeliveryGrant,
    ) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
        accepted::finalize(self, correlation_id, attempt_id, grant)
    }

    pub(super) async fn finalize_accepted_async(
        &self,
        correlation_id: &CorrelationId,
        attempt_id: &EventId,
        grant: AuthenticatedDeliveryGrant,
    ) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
        accepted::finalize_async(self, correlation_id, attempt_id, grant).await
    }

    pub(super) fn finalize_rejected(
        &self,
        correlation_id: &CorrelationId,
        attempt_id: &EventId,
        error: AuthenticatedDeliveryGrantIssuanceError,
    ) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
        self.publish_issuance_milestone(
            correlation_id,
            attempt_id,
            rejected_issuance_milestone_for(error),
        )?;
        Err(error)
    }

    pub(super) async fn finalize_rejected_async(
        &self,
        correlation_id: &CorrelationId,
        attempt_id: &EventId,
        error: AuthenticatedDeliveryGrantIssuanceError,
    ) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
        self.publish_issuance_milestone_async(
            correlation_id,
            attempt_id,
            rejected_issuance_milestone_for(error),
        )
        .await?;
        Err(error)
    }

    fn publish_issuance_milestone(
        &self,
        correlation_id: &CorrelationId,
        attempt_id: &EventId,
        milestone: AuthenticatedDeliveryGrantIssuanceMilestone,
    ) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
        let publisher = self
            .issuance_publisher
            .as_ref()
            .ok_or(AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)?;
        publisher
            .publish(correlation_id.clone(), attempt_id.clone(), milestone)
            .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)
    }

    async fn publish_issuance_milestone_async(
        &self,
        correlation_id: &CorrelationId,
        attempt_id: &EventId,
        milestone: AuthenticatedDeliveryGrantIssuanceMilestone,
    ) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
        let publisher = self
            .issuance_publisher
            .as_ref()
            .ok_or(AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)?;
        publisher
            .publish_async(correlation_id.clone(), attempt_id.clone(), milestone)
            .await
            .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)
    }

    fn trusted_now(&self) -> String {
        self.next_trusted_issuance_now_for_debug_test()
            .unwrap_or_else(|| {
                self.trusted_issuance_now
                    .clone()
                    .unwrap_or_else(|| Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true))
            })
    }
}

pub(super) fn generated_issuance_correlation_id(
) -> Result<CorrelationId, AuthenticatedDeliveryGrantIssuanceError> {
    CorrelationId::parse(format!(
        "authenticated-delivery-grant:issuance:rejection:{}",
        EventId::generated().as_str()
    ))
    .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::CorrelationIdRejected)
}

fn validate_minimum_remaining_lifetime(
    grant: &AuthenticatedDeliveryGrant,
    trusted_now: &str,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let trusted_now = DateTime::<FixedOffset>::parse_from_rfc3339(trusted_now)
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)?;
    let expires_at = DateTime::<FixedOffset>::parse_from_rfc3339(&grant.expires_at)
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)?;
    ((expires_at - trusted_now).num_seconds() >= MINIMUM_REMAINING_GRANT_LIFETIME_SECONDS)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp)
}
