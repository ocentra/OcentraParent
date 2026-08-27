//! Fixed AccountIssuer v2 transport contract.

use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2CorrelationId,
    AccountIdentityAuthorityProducerV2IdempotencyKey,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_OUTER_DOMAIN,
};

use crate::types::ProtocolError;

pub const ACCOUNT_ISSUER_PROTOCOL_VERSION: u16 = 2;
pub const ACCOUNT_ISSUER_TRANSPORT_DOMAIN: &[u8] =
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_OUTER_DOMAIN;
pub const ACCOUNT_ISSUER_SERVICE: &str = "ocentra.account-authority-producer.cloudflare.v2";
pub const ACCOUNT_ISSUER_MAX_FIELD_BYTES: usize = 1_024;
pub const ACCOUNT_ISSUER_MAX_INNER_BYTES: usize = 64 * 1_024;
pub const ACCOUNT_ISSUER_MAX_WIRE_BYTES: usize = 128 * 1_024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerField(String);

impl AccountIssuerField {
    pub fn from_wire(bytes: Vec<u8>) -> Result<Self, ProtocolError> {
        if bytes.is_empty() {
            return Err(ProtocolError::EmptyField);
        }
        if bytes.len() > ACCOUNT_ISSUER_MAX_FIELD_BYTES {
            return Err(ProtocolError::FieldTooLarge);
        }
        let value = String::from_utf8(bytes).map_err(|_| ProtocolError::InvalidDiscriminant(0))?;
        if value.trim().is_empty()
            || value
                .chars()
                .any(|character| character <= '\u{001f}' || character == '\u{007f}')
        {
            return Err(ProtocolError::EmptyField);
        }
        Ok(Self(value))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn parse_correlation_id(
        &self,
    ) -> Result<AccountIdentityAuthorityProducerV2CorrelationId, ProtocolError> {
        AccountIdentityAuthorityProducerV2CorrelationId::parse(self.0.clone())
            .map_err(|_| ProtocolError::InvalidDiscriminant(0))
    }

    pub fn parse_idempotency_key(
        &self,
    ) -> Result<AccountIdentityAuthorityProducerV2IdempotencyKey, ProtocolError> {
        AccountIdentityAuthorityProducerV2IdempotencyKey::parse(self.0.clone())
            .map_err(|_| ProtocolError::InvalidDiscriminant(0))
    }
}
