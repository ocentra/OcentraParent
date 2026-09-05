//! Public v2 parser/verification entry point.

use chrono::{DateTime, Utc};

use crate::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Error, AccountIdentityAuthorityProducerV2Verified,
};

pub fn verify(
    wire: &[u8],
    public_key: &[u8; 65],
    now: DateTime<Utc>,
) -> Result<AccountIdentityAuthorityProducerV2Verified, AccountIdentityAuthorityProducerV2Error> {
    crate::account_identity_authority_producer_v2::verify(wire, public_key, now)
}
