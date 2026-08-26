//! Owner-facing AccountIssuer v2 command and result contracts.

use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2CorrelationId,
    AccountIdentityAuthorityProducerV2IdempotencyKey,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
};

pub const PRODUCER: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SCHEMA_VERSION;
pub const AUDIENCE: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_AUDIENCE;
pub const SERVICE: &str = ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountIssuerReceiptIdError {
    Invalid,
}

impl std::fmt::Display for AccountIssuerReceiptIdError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(
            ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_ISSUER_COMMAND_INVALID,
        )
    }
}

impl std::error::Error for AccountIssuerReceiptIdError {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountIssuerReceiptId(String);

impl AccountIssuerReceiptId {
    pub fn from_bytes(value: Vec<u8>) -> Result<Self, AccountIssuerReceiptIdError> {
        if value.is_empty() || value.len() > 1_024 {
            return Err(AccountIssuerReceiptIdError::Invalid);
        }
        let value = String::from_utf8(value).map_err(|_| AccountIssuerReceiptIdError::Invalid)?;
        if value.trim().is_empty() {
            return Err(AccountIssuerReceiptIdError::Invalid);
        }
        Ok(Self(value))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

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
