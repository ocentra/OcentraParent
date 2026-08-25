use std::fmt;

use crate::constants;

use super::UntrustedResponse;

impl fmt::Debug for UntrustedResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_RESPONSE)
            .field(constants::DEBUG_FIELD_VERSION, &self.version())
            .field(
                constants::DEBUG_FIELD_PROTOCOL_GENERATION,
                &self.protocol_generation(),
            )
            .field(constants::DEBUG_FIELD_NONCE, &self.nonce())
            .field(constants::DEBUG_FIELD_BROKER_NONCE, &self.broker_nonce())
            .field(constants::DEBUG_FIELD_CORRELATION, &self.correlation())
            .field(
                constants::DEBUG_FIELD_CLIENT_PROCESS_ID,
                &self.client_process_id(),
            )
            .field(
                constants::DEBUG_FIELD_CLIENT_SESSION_ID,
                &self.client_session_id(),
            )
            .field(
                constants::DEBUG_FIELD_BROKER_PROCESS_ID,
                &self.broker_process_id(),
            )
            .field(
                constants::DEBUG_FIELD_BROKER_SESSION_ID,
                &self.broker_session_id(),
            )
            .field(
                constants::DEBUG_FIELD_SESSION_HANDLE,
                &self.session_handle(),
            )
            .field(
                constants::DEBUG_FIELD_TRANSCRIPT_DIGEST,
                &self.transcript_digest(),
            )
            .field(constants::DEBUG_FIELD_SEQUENCE, &self.sequence())
            .field(constants::DEBUG_FIELD_REQUEST_KIND, &self.request_kind())
            .field(constants::DEBUG_FIELD_STATUS, &self.status())
            .field(
                constants::DEBUG_FIELD_OPAQUE_TOKEN,
                &constants::DEBUG_REDACTED,
            )
            .finish()
    }
}
