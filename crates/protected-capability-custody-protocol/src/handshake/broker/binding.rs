use super::super::{UntrustedBrokerHello, UntrustedClientHello};

impl UntrustedBrokerHello {
    pub fn matches_client(&self, client: &UntrustedClientHello) -> bool {
        self.version == client.version()
            && self.protocol_generation == client.protocol_generation()
            && self.client_nonce == client.nonce()
            && self.correlation == client.correlation()
            && self.client_process_id == client.client_process_id()
            && self.client_process_epoch == client.client_process_epoch()
            && self.client_session_id == client.client_session_id()
    }
}
