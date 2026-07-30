use super::AuthenticatedDeliveryGrantIssuanceRejection;
use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantIssuanceError;

pub(super) fn from(
    error: AuthenticatedDeliveryGrantIssuanceError,
) -> AuthenticatedDeliveryGrantIssuanceRejection {
    match error {
        AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected => {
            AuthenticatedDeliveryGrantIssuanceRejection::AuthorityProvenance
        }
        AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed => {
            AuthenticatedDeliveryGrantIssuanceRejection::MilestonePublication
        }
        _ => AuthenticatedDeliveryGrantIssuanceRejection::AuthorityProvenance,
    }
}
