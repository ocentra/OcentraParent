use std::fmt;

use super::super::BrokerHello;

impl fmt::Debug for BrokerHello {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BrokerHello")
            .field("version", &self.version)
            .field("client_nonce", &self.client_nonce)
            .field("broker_nonce", &self.broker_nonce)
            .field("correlation", &self.correlation)
            .field("client_process_epoch", &self.client_process_epoch)
            .field("broker_epoch", &self.broker_epoch)
            .field("broker_key_epoch", &self.broker_key_epoch)
            .field("writer_lease_epoch", &self.writer_lease_epoch)
            .field("watermark", &self.watermark)
            .field("authority_generation", &self.authority_generation)
            .field("target_generation", &self.target_generation)
            .field("key_generation", &self.key_generation)
            .field("writer_generation", &self.writer_generation)
            .field("session_handle", &self.session_handle)
            .field("attestation_digest", &self.attestation_digest)
            .finish()
    }
}
