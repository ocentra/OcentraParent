//! Owner-facing AccountIssuer v2 command and result contracts.

use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2CorrelationId,
    AccountIdentityAuthorityProducerV2IdempotencyKey,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};

use ocentra_family_identity_core::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Request;
use ocentra_protected_capability_custody_core::account_issuer::{
    AccountIssuerP256Signer, AccountIssuerP256SignerError, AccountIssuerSignerCapability,
};

pub const PRODUCER: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION;
pub const AUDIENCE: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE;
pub const SERVICE: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCurrentAuthorityCommand {
    correlation_id: AccountIdentityAuthorityProducerV2CorrelationId,
    idempotency_key: AccountIdentityAuthorityProducerV2IdempotencyKey,
}

impl IssueCurrentAuthorityCommand {
    pub fn new(
        correlation_id: AccountIdentityAuthorityProducerV2CorrelationId,
        idempotency_key: AccountIdentityAuthorityProducerV2IdempotencyKey,
    ) -> Self {
        Self {
            correlation_id,
            idempotency_key,
        }
    }

    pub(crate) fn correlation_id(&self) -> &AccountIdentityAuthorityProducerV2CorrelationId {
        &self.correlation_id
    }

    pub(crate) fn idempotency_key(&self) -> &AccountIdentityAuthorityProducerV2IdempotencyKey {
        &self.idempotency_key
    }
}

/// Owner-created bridge state for the protected P-256 signer.
///
/// The family request remains private inside this value until the owner has
/// received the exact protected signing capability. Callers never receive the
/// family request, a raw signer, or a platform handle.
pub struct PreparedAccountIssuerV2Request {
    request: AccountIdentityAuthorityProducerV2Request,
}

impl PreparedAccountIssuerV2Request {
    pub(crate) fn from_request(request: AccountIdentityAuthorityProducerV2Request) -> Self {
        Self { request }
    }

    /// Return the exact binding selected by the Account-owned request.
    pub fn binding(&self) -> &ocentra_schema::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Binding{
        self.request.binding()
    }

    /// Ask the Account-specific protected core adapter to sign this exact
    /// owner-created request. The family request remains private here and is
    /// retained for owner-side verification and durable finalization.
    pub fn sign_with(
        &self,
        signer: &AccountIssuerP256Signer,
    ) -> Result<AccountIssuerSignerCapability, AccountIssuerP256SignerError> {
        signer.sign_request(&self.request)
    }

    pub(crate) fn into_parts(self) -> AccountIdentityAuthorityProducerV2Request {
        self.request
    }
}
