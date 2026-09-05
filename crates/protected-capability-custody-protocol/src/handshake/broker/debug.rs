use std::fmt;

use crate::constants;

use super::super::UntrustedBrokerHello;

impl fmt::Debug for UntrustedBrokerHello {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_BROKER_HELLO)
            .field(constants::DEBUG_FIELD_VERSION, &self.version)
            .field(
                constants::DEBUG_FIELD_PROTOCOL_GENERATION,
                &self.protocol_generation,
            )
            .field(constants::DEBUG_FIELD_NONCE, &self.client_nonce)
            .field(constants::DEBUG_FIELD_BROKER_NONCE, &self.broker_nonce)
            .field(constants::DEBUG_FIELD_CORRELATION, &self.correlation)
            .field(
                constants::DEBUG_FIELD_CLIENT_PROCESS_ID,
                &self.client_process_id,
            )
            .field(
                constants::DEBUG_FIELD_CLIENT_PROCESS_EPOCH,
                &self.client_process_epoch,
            )
            .field(
                constants::DEBUG_FIELD_CLIENT_SESSION_ID,
                &self.client_session_id,
            )
            .field(
                constants::DEBUG_FIELD_BROKER_PROCESS_ID,
                &self.broker_process_id,
            )
            .field(
                constants::DEBUG_FIELD_BROKER_SESSION_ID,
                &self.broker_session_id,
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
            .field(constants::DEBUG_FIELD_SESSION_HANDLE, &self.session_handle)
            .field(
                constants::DEBUG_FIELD_ATTESTATION_DIGEST,
                &self.attestation_digest,
            )
            .field(constants::DEBUG_FIELD_AUTHENTICATOR, &self.authenticator)
            .field(
                constants::DEBUG_FIELD_EXPIRES_AT_UNIX_MILLIS,
                &self.session_expires_at_unix_millis,
            )
            .finish()
    }
}
