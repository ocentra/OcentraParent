use sha2::{Digest, Sha256};

use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport;

use super::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerReceiptLineage, AccountIdentityIssuerRecordedTransport,
};
use super::{AccountIdentityIssuerCurrentness, AccountIdentityIssuerV2KeyRecord};

impl AccountIdentityIssuerRecordedTransport {
    pub fn lineage(&self) -> &AccountIdentityIssuerReceiptLineage {
        &self.lineage
    }

    pub fn into_lineage(self) -> AccountIdentityIssuerReceiptLineage {
        self.lineage
    }

    pub(crate) fn from_verified_currentness(
        currentness: &AccountIdentityIssuerCurrentness,
        key: &AccountIdentityIssuerV2KeyRecord,
        transport: AccountIdentityAuthorityProducerV2Transport,
        replayed: bool,
    ) -> Self {
        let receipt = transport.receipt();
        let authority = currentness.authority();
        let signed_transport_digest = signed_transport_digest(transport.wire_bytes());
        Self {
            lineage: AccountIdentityIssuerReceiptLineage {
                account_id: currentness.account_id().as_str().to_owned(),
                household_id: currentness.household_id().as_str().to_owned(),
                provider: authority.provider().clone(),
                provider_subject: authority.provider_subject().clone(),
                member_id: authority.member_id().as_str().to_owned(),
                device_id: authority.device_id().as_str().to_owned(),
                session_id: authority.session_id().as_str().to_owned(),
                service: ocentra_schema::account_identity_authority_producer_v2::
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE.to_owned(),
                service_binding_id: key.service_binding_id().as_str().to_owned(),
                key_id: key.key_id().as_str().to_owned(),
                key_generation: key.key_generation(),
                enrollment_generation: key.enrollment_generation(),
                authority_generation: currentness.authority_generation(),
                session_generation: currentness.session_generation(),
                correlation_id: receipt.correlation_id.clone(),
                idempotency_key: receipt.idempotency_key.clone(),
                receipt_id: receipt.receipt_id.clone(),
                payload_digest: receipt.payload_digest.clone(),
                signed_transport_digest,
                issued_at: receipt.issued_at.clone(),
                expires_at: receipt.expires_at.clone(),
            },
            transport,
            replayed,
        }
    }
}

fn signed_transport_digest(wire: &[u8]) -> String {
    let value = Sha256::digest(wire);
    let prefix = ocentra_schema::account_identity_authority_producer_v2::
        ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNED_TRANSPORT_DIGEST_PREFIX;
    format!("{prefix}{value:x}")
}
