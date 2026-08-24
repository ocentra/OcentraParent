use super::super::{BrokerHello, ClientHello};

impl BrokerHello {
    pub fn matches_client(&self, client: &ClientHello) -> bool {
        self.version == client.version()
            && self.client_nonce == client.nonce()
            && self.correlation == client.correlation()
            && self.client_process_epoch == client.client_process_epoch()
    }
}
