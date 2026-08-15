use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrant;

use super::{
    digest, validation, AuthenticatedDeliveryGrantConsumeError,
    AuthenticatedDeliveryGrantExpectation, AuthenticatedDeliveryGrantTrustedIssuer,
};

pub fn validate_authenticated_delivery_grant(
    grant: &AuthenticatedDeliveryGrant,
    expected: &AuthenticatedDeliveryGrantExpectation,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let (trusted_now, _trusted_now_nanos) = validation::parse_trusted_now(&expected.observed_at)?;
    validation::validate_grant(grant, expected, trusted_issuer, trusted_now)
}

pub fn redacted_delivery_nonce_digest(nonce: &str) -> String {
    digest(nonce)
}
