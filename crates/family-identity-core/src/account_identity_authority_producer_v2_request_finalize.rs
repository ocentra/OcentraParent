use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES;

use super::{
    account_identity_authority_producer_v2_verify, AccountIdentityAuthorityProducerV2Error,
    AccountIdentityAuthorityProducerV2Request, AccountIdentityAuthorityProducerV2Transport,
};

impl AccountIdentityAuthorityProducerV2Request {
    /// Finish an owner-created request with a platform signature. The
    /// signature is checked immediately with ring and low-S is enforced before
    /// any transport can be returned.
    pub fn finalize(
        self,
        signature: [u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
    ) -> Result<AccountIdentityAuthorityProducerV2Transport, AccountIdentityAuthorityProducerV2Error>
    {
        self.finalize_preserving(signature)
            .map_err(|(_, error)| error)
    }

    /// Finalize while retaining the owner-created request on every rejected
    /// signature/wire path. The issuer uses the returned request to record a
    /// durable signing failure instead of consuming a prepared reservation.
    pub(crate) fn finalize_preserving(
        self,
        signature: [u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
    ) -> Result<
        AccountIdentityAuthorityProducerV2Transport,
        (Self, AccountIdentityAuthorityProducerV2Error),
    > {
        let (request, signature) = self.verify_preserving(signature)?;
        let wire = match crate::account_identity_authority_envelope_v2::wire(
            request.signing_bytes.clone(),
            signature,
        ) {
            Ok(wire) => wire,
            Err(error) => return Err((request, error)),
        };
        Ok(transport_from_request(request, wire))
    }

    fn verify_preserving(
        self,
        signature: [u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
    ) -> Result<
        (
            Self,
            [u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SIGNATURE_BYTES],
        ),
        (Self, AccountIdentityAuthorityProducerV2Error),
    > {
        match account_identity_authority_producer_v2_verify::verify_signature(
            &self.public_key,
            &self.signing_bytes,
            &signature,
        ) {
            Ok(()) => Ok((self, signature)),
            Err(error) => Err((self, error)),
        }
    }
}

fn transport_from_request(
    request: AccountIdentityAuthorityProducerV2Request,
    wire: Vec<u8>,
) -> AccountIdentityAuthorityProducerV2Transport {
    let AccountIdentityAuthorityProducerV2Request {
        signing_bytes: _,
        public_key: _,
        operation,
        binding,
        payload_digest,
        issued_at,
        expires_at,
    } = request;
    let receipt_id = binding.receipt_id.clone();
    AccountIdentityAuthorityProducerV2Transport {
        wire,
        receipt: ocentra_schema::account_identity_authority_producer_v2::
            AccountIdentityAuthorityProducerV2Receipt {
            receipt_id,
            operation,
            account_id: binding.account_id,
            household_id: binding.household_id,
            service_binding_id: binding.service_binding_id,
            correlation_id: binding.correlation_id,
            idempotency_key: binding.idempotency_key,
            payload_digest,
            key_id: binding.key_id,
            key_generation: binding.key_generation,
            enrollment_generation: binding.enrollment_generation,
            authority_generation: binding.authority_generation,
            session_generation: binding.session_generation,
            issued_at,
            expires_at,
        },
    }
}

impl AccountIdentityAuthorityProducerV2Transport {
    pub fn wire_bytes(&self) -> &[u8] {
        self.wire.as_slice()
    }

    pub fn receipt(
        &self,
    ) -> &ocentra_schema::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Receipt
    {
        &self.receipt
    }

    pub(crate) fn clone_durable(&self) -> Self {
        crate::account_identity_authority_producer_v2::from_durable_transport(
            self.wire.clone(),
            self.receipt.clone(),
        )
    }
}
