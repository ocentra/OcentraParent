use std::fmt;

use crate::constants;

use super::super::BrokerHello;

impl fmt::Debug for BrokerHello {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_BROKER_HELLO)
            .field(constants::DEBUG_FIELD_VERSION, &self.version)
            .field(constants::DEBUG_FIELD_NONCE, &self.client_nonce)
            .field(constants::DEBUG_FIELD_BROKER_NONCE, &self.broker_nonce)
            .field(constants::DEBUG_FIELD_CORRELATION, &self.correlation)
            .field(
                constants::DEBUG_FIELD_CLIENT_PROCESS_EPOCH,
                &self.client_process_epoch,
            )
            .field(constants::DEBUG_FIELD_BROKER_EPOCH, &self.broker_epoch)
            .field(
                constants::DEBUG_FIELD_BROKER_KEY_EPOCH,
                &self.broker_key_epoch,
            )
            .field(
                constants::DEBUG_FIELD_WRITER_LEASE_EPOCH,
                &self.writer_lease_epoch,
            )
            .field(constants::DEBUG_FIELD_WATERMARK, &self.watermark)
            .field(
                constants::DEBUG_FIELD_AUTHORITY_GENERATION,
                &self.authority_generation,
            )
            .field(
                constants::DEBUG_FIELD_TARGET_GENERATION,
                &self.target_generation,
            )
            .field(constants::DEBUG_FIELD_KEY_GENERATION, &self.key_generation)
            .field(
                constants::DEBUG_FIELD_WRITER_GENERATION,
                &self.writer_generation,
            )
            .field(constants::DEBUG_FIELD_SESSION_HANDLE, &self.session_handle)
            .field(
                constants::DEBUG_FIELD_ATTESTATION_DIGEST,
                &self.attestation_digest,
            )
            .finish()
    }
}
