use super::AuthenticatedDeliveryGrantIssuanceRejection;
use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantIssuanceError;

pub(super) fn from(
    error: AuthenticatedDeliveryGrantIssuanceError,
) -> AuthenticatedDeliveryGrantIssuanceRejection {
    match error {
        AuthenticatedDeliveryGrantIssuanceError::InvalidAuthorizationSnapshot => {
            AuthenticatedDeliveryGrantIssuanceRejection::AuthorizationSnapshot
        }
        AuthenticatedDeliveryGrantIssuanceError::CorrelationIdRejected => {
            AuthenticatedDeliveryGrantIssuanceRejection::CorrelationId
        }
        AuthenticatedDeliveryGrantIssuanceError::InvalidTimestamp => {
            AuthenticatedDeliveryGrantIssuanceRejection::Timestamp
        }
        AuthenticatedDeliveryGrantIssuanceError::InvalidBindings => {
            AuthenticatedDeliveryGrantIssuanceRejection::Bindings
        }
        _ => AuthenticatedDeliveryGrantIssuanceRejection::AuthorizationSnapshot,
    }
}
