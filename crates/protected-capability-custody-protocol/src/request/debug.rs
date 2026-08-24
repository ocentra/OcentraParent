use std::fmt;

use super::Request;

impl fmt::Debug for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Request")
            .field("version", &self.version)
            .field("nonce", &self.nonce)
            .field("correlation", &self.correlation)
            .field("client_process_epoch", &self.client_process_epoch)
            .field("broker_epoch", &self.broker_epoch)
            .field("broker_key_epoch", &self.broker_key_epoch)
            .field("writer_lease_epoch", &self.writer_lease_epoch)
            .field("watermark", &self.watermark)
            .field(
                "expected_authority_generation",
                &self.expected_authority_generation,
            )
            .field(
                "expected_target_generation",
                &self.expected_target_generation,
            )
            .field("expected_key_generation", &self.expected_key_generation)
            .field(
                "expected_writer_generation",
                &self.expected_writer_generation,
            )
            .field("session_handle", &self.session_handle)
            .field("attestation_digest", &self.attestation_digest)
            .field("kind", &self.kind)
            .field("operation_length", &self.operation.len())
            .field("action", &self.action)
            .field("target", &self.target)
            .field("opaque_token", &"<redacted>")
            .finish()
    }
}
