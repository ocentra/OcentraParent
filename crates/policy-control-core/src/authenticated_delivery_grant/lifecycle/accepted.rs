use ocentra_eventing::ids::{CorrelationId, EventId};
use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrant;

use super::super::AuthenticatedDeliveryGrantIssuer;
use super::{
    accepted_issuance_milestone_for, prepared_issuance_milestone_for,
    validate_minimum_remaining_lifetime, AuthenticatedDeliveryGrantIssuanceError,
};

pub(super) fn finalize(
    issuer: &AuthenticatedDeliveryGrantIssuer,
    correlation_id: &CorrelationId,
    attempt_id: &EventId,
    grant: AuthenticatedDeliveryGrant,
) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
    issuer.publish_issuance_milestone(
        correlation_id,
        attempt_id,
        prepared_issuance_milestone_for(&grant),
    )?;
    validate_before_accepted(issuer, correlation_id, attempt_id, &grant)?;
    publish_accepted_or_reject(issuer, correlation_id, attempt_id, &grant)?;
    Ok(grant)
}

pub(super) async fn finalize_async(
    issuer: &AuthenticatedDeliveryGrantIssuer,
    correlation_id: &CorrelationId,
    attempt_id: &EventId,
    grant: AuthenticatedDeliveryGrant,
) -> Result<AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantIssuanceError> {
    issuer
        .publish_issuance_milestone_async(
            correlation_id,
            attempt_id,
            prepared_issuance_milestone_for(&grant),
        )
        .await?;
    validate_before_accepted_async(issuer, correlation_id, attempt_id, &grant).await?;
    publish_accepted_or_reject_async(issuer, correlation_id, attempt_id, &grant).await?;
    Ok(grant)
}

fn validate_before_accepted(
    issuer: &AuthenticatedDeliveryGrantIssuer,
    correlation_id: &CorrelationId,
    attempt_id: &EventId,
    grant: &AuthenticatedDeliveryGrant,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    match validate_remaining_lifetime(issuer, grant) {
        Ok(()) => Ok(()),
        Err(error) => issuer
            .finalize_rejected(correlation_id, attempt_id, error)
            .map(|_grant| ()),
    }
}

fn publish_accepted_or_reject(
    issuer: &AuthenticatedDeliveryGrantIssuer,
    correlation_id: &CorrelationId,
    attempt_id: &EventId,
    grant: &AuthenticatedDeliveryGrant,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    validate_before_accepted(issuer, correlation_id, attempt_id, grant)?;
    issuer
        .publish_issuance_milestone(
            correlation_id,
            attempt_id,
            accepted_issuance_milestone_for(grant),
        )
        .or_else(|_error| {
            issuer
                .finalize_rejected(
                    correlation_id,
                    attempt_id,
                    AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed,
                )
                .map(|_grant| ())
        })
}

async fn validate_before_accepted_async(
    issuer: &AuthenticatedDeliveryGrantIssuer,
    correlation_id: &CorrelationId,
    attempt_id: &EventId,
    grant: &AuthenticatedDeliveryGrant,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    if let Err(error) = validate_remaining_lifetime(issuer, grant) {
        return issuer
            .finalize_rejected_async(correlation_id, attempt_id, error)
            .await
            .map(|_grant| ());
    }
    Ok(())
}

async fn publish_accepted_or_reject_async(
    issuer: &AuthenticatedDeliveryGrantIssuer,
    correlation_id: &CorrelationId,
    attempt_id: &EventId,
    grant: &AuthenticatedDeliveryGrant,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    validate_before_accepted_async(issuer, correlation_id, attempt_id, grant).await?;
    if issuer
        .publish_issuance_milestone_async(
            correlation_id,
            attempt_id,
            accepted_issuance_milestone_for(grant),
        )
        .await
        .is_err()
    {
        return issuer
            .finalize_rejected_async(
                correlation_id,
                attempt_id,
                AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed,
            )
            .await
            .map(|_grant| ());
    }
    Ok(())
}

fn validate_remaining_lifetime(
    issuer: &AuthenticatedDeliveryGrantIssuer,
    grant: &AuthenticatedDeliveryGrant,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    validate_minimum_remaining_lifetime(grant, &issuer.trusted_now())
}
