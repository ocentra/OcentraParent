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
