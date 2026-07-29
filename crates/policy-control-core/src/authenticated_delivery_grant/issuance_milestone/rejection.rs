use super::AuthenticatedDeliveryGrantIssuanceRejection;
use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantIssuanceError;

mod authorization;

pub(super) fn issuance_rejection(
    error: AuthenticatedDeliveryGrantIssuanceError,
) -> AuthenticatedDeliveryGrantIssuanceRejection {
    match error {
        AuthenticatedDeliveryGrantIssuanceError::InvalidIssuerKeyId => {
            AuthenticatedDeliveryGrantIssuanceRejection::IssuerKey
        }
        AuthenticatedDeliveryGrantIssuanceError::ParentAuthorityRejected => {
            AuthenticatedDeliveryGrantIssuanceRejection::ParentAuthority
        }
        AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected => {
            AuthenticatedDeliveryGrantIssuanceRejection::ParentStepUp
        }
        AuthenticatedDeliveryGrantIssuanceError::PolicyNotExecutable => {
            AuthenticatedDeliveryGrantIssuanceRejection::Policy
        }
        AuthenticatedDeliveryGrantIssuanceError::ManualReviewRequired => {
            AuthenticatedDeliveryGrantIssuanceRejection::ManualReview
        }
        AuthenticatedDeliveryGrantIssuanceError::CapabilityUnavailable => {
            AuthenticatedDeliveryGrantIssuanceRejection::Capability
        }
        AuthenticatedDeliveryGrantIssuanceError::EvidenceNotStable => {
            AuthenticatedDeliveryGrantIssuanceRejection::Evidence
        }
        AuthenticatedDeliveryGrantIssuanceError::DryRunForbidden => {
            AuthenticatedDeliveryGrantIssuanceRejection::DryRun
        }
        AuthenticatedDeliveryGrantIssuanceError::AuthorizationBindingMismatch => {
            AuthenticatedDeliveryGrantIssuanceRejection::AuthorizationBinding
        }
        AuthenticatedDeliveryGrantIssuanceError::InvalidAuthorizationSnapshot
        | AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp
        | AuthenticatedDeliveryGrantIssuanceError::InvalidBindings => authorization::from(error),
        AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected
        | AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed => {
            AuthenticatedDeliveryGrantIssuanceRejection::AuthorityProvenance
        }
    }
}
