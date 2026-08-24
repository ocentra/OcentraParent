use crate::handshake::BrokerHello;

use super::Request;

impl Request {
    pub fn is_bound_to(&self, hello: &BrokerHello) -> bool {
        self.version == hello.version()
            && self.client_process_epoch == hello.client_process_epoch()
            && self.broker_epoch == hello.broker_epoch()
            && self.broker_key_epoch == hello.broker_key_epoch()
            && self.writer_lease_epoch == hello.writer_lease_epoch()
            && self.watermark == hello.watermark()
            && self.expected_authority_generation == hello.authority_generation()
            && self.expected_target_generation == hello.target_generation()
            && self.expected_key_generation == hello.key_generation()
            && self.expected_writer_generation == hello.writer_generation()
            && self.session_handle == hello.session_handle()
            && self.attestation_digest == hello.attestation_digest()
    }
}
