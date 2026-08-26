//! Durable delivery-attempt boundary.
//!
//! Delivery completion is intentionally not represented by a public trait or
//! boolean. The owner can claim an outbox row and durably record failure; a
//! later protected adapter must present a verified receipt to transition the
//! row to acknowledged.

use ocentra_family_identity_core::account_identity_authority_issuer_client::
    account_identity_authority_issuer_client_types::AccountIdentityIssuerOutboxClaim;
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_DELIVERY_ERROR;

#[derive(Debug)]
pub enum AccountIssuerDeliveryError {
    OwnerUnavailable,
    Rejected,
}

impl std::fmt::Display for AccountIssuerDeliveryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(ACCOUNT_ISSUER_DELIVERY_ERROR)
    }
}

impl std::error::Error for AccountIssuerDeliveryError {}

pub struct DeliveryClaim {
    pub(crate) inner: AccountIdentityIssuerOutboxClaim,
}

pub struct DeliveryFailure {
    pub(crate) message: String,
}
