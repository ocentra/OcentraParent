//! Account-owned P-256 v2 current-authority producer.
//!
//! This module accepts only a family-created `VerifiedAccountIdentityAuthority`
//! and a durable public-key record. It creates an unsigned request for the
//! later platform signer; the absence of that signer cannot result in a
//! process-generated key or a successful authority response.

use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Binding, AccountIdentityAuthorityProducerV2Claims,
    AccountIdentityAuthorityProducerV2Operation, AccountIdentityAuthorityProducerV2Receipt,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES,
};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

#[path = "account_identity_authority_producer_v2_sec1.rs"]
pub(super) mod sec1;

#[path = "account_identity_authority_producer_v2_request.rs"]
mod account_identity_authority_producer_v2_request;
#[path = "account_identity_authority_producer_v2_request_finalize.rs"]
mod account_identity_authority_producer_v2_request_finalize;
#[path = "account_identity_authority_producer_v2_request_surface.rs"]
mod account_identity_authority_producer_v2_request_surface;
#[path = "account_identity_authority_producer_v2_time.rs"]
mod account_identity_authority_producer_v2_time;
#[path = "account_identity_authority_producer_v2_verified.rs"]
mod account_identity_authority_producer_v2_verified;
#[path = "account_identity_authority_producer_v2_verify.rs"]
mod account_identity_authority_producer_v2_verify;

#[derive(Debug)]
pub enum AccountIdentityAuthorityProducerV2Error {
    AuthorityExpired,
    AuthorityInvalid,
    InvalidKeyId,
    InvalidPublicKey,
    InvalidSignature,
    InvalidWire,
    SignatureInvalid,
    UnsupportedOperation,
}

impl std::fmt::Display for AccountIdentityAuthorityProducerV2Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("account_identity_authority_producer_v2_error")
    }
}

impl std::error::Error for AccountIdentityAuthorityProducerV2Error {}

pub struct AccountIdentityAuthorityProducerV2Request {
    signing_bytes: Vec<u8>,
    public_key: [u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    operation: AccountIdentityAuthorityProducerV2Operation,
    binding: AccountIdentityAuthorityProducerV2Binding,
    payload_digest: String,
    issued_at: String,
    expires_at: String,
}

#[derive(Clone)]
pub struct AccountIdentityAuthorityProducerV2Transport {
    wire: Vec<u8>,
    receipt: AccountIdentityAuthorityProducerV2Receipt,
}

pub struct AccountIdentityAuthorityProducerV2Verified {
    operation: AccountIdentityAuthorityProducerV2Operation,
    receipt_id: String,
    key_id: String,
    service_binding_id: String,
    key_generation: u64,
    enrollment_generation: u64,
    authority_generation: u64,
    session_generation: u64,
    correlation_id: String,
    idempotency_key: String,
    issued_at: String,
    expires_at: String,
    payload_digest: String,
    claims: AccountIdentityAuthorityProducerV2Claims,
}

pub struct AccountIdentityAuthorityProducerV2VerifiedReceipt {
    receipt: AccountIdentityAuthorityProducerV2Receipt,
}

pub(crate) struct AccountIdentityAuthorityProducerV2IssueInput<'a> {
    pub(crate) authority: &'a VerifiedAccountIdentityAuthority,
    pub(crate) key_id: &'a str,
    pub(crate) key_generation: u64,
    pub(crate) enrollment_generation: u64,
    pub(crate) public_key: &'a [u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    pub(crate) service_binding_id: &'a str,
    pub(crate) correlation_id: &'a str,
    pub(crate) idempotency_key: &'a str,
    pub(crate) issued_at: DateTime<Utc>,
}

pub(crate) fn from_durable_transport(
    wire: Vec<u8>,
    receipt: AccountIdentityAuthorityProducerV2Receipt,
) -> AccountIdentityAuthorityProducerV2Transport {
    AccountIdentityAuthorityProducerV2Transport { wire, receipt }
}

pub(crate) fn issue_request(
    input: &AccountIdentityAuthorityProducerV2IssueInput<'_>,
) -> Result<AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Error> {
    account_identity_authority_producer_v2_request::issue_request(input)
}

pub(crate) fn acknowledge_request(
    receipt: &AccountIdentityAuthorityProducerV2Receipt,
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    now: DateTime<Utc>,
) -> Result<AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Error> {
    account_identity_authority_producer_v2_request::acknowledge_request(receipt, public_key, now)
}

pub fn verify(
    wire: &[u8],
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    now: DateTime<Utc>,
) -> Result<AccountIdentityAuthorityProducerV2Verified, AccountIdentityAuthorityProducerV2Error> {
    account_identity_authority_producer_v2_verify::verify(wire, public_key, now)
}

pub fn verify_receipt(
    wire: &[u8],
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
    now: DateTime<Utc>,
) -> Result<
    AccountIdentityAuthorityProducerV2VerifiedReceipt,
    AccountIdentityAuthorityProducerV2Error,
> {
    account_identity_authority_producer_v2_verify::verify_receipt(wire, public_key, now)
}

pub fn expected_key_id(
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
) -> String {
    account_identity_authority_producer_v2_verify::expected_key_id(public_key)
}

pub(crate) fn validate_public_key(
    public_key: &[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_PUBLIC_KEY_BYTES],
) -> Result<(), AccountIdentityAuthorityProducerV2Error> {
    account_identity_authority_producer_v2_verify::validate_public_key(public_key)
}
