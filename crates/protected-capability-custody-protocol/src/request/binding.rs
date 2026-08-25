use crate::handshake::UntrustedBrokerHello;
use crate::types::{AuthenticationDomain, BootstrapAuthenticator, ProtocolError};

use super::{authenticated::AuthenticatedRequest, UntrustedRequest};

impl UntrustedRequest {
    pub fn into_authenticated_session(
        self,
        hello: &UntrustedBrokerHello,
        now_unix_millis: u64,
        expected_sequence: u64,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<AuthenticatedRequest, ProtocolError> {
        if !self.is_bound_to(hello)
            || expected_sequence == 0
            || self.sequence() != expected_sequence
        {
            return Err(ProtocolError::InvalidSequence);
        }
        if now_unix_millis == 0
            || now_unix_millis >= self.expires_at_unix_millis()
            || self.expires_at_unix_millis() > hello.session_expires_at_unix_millis()
            || self
                .expires_at_unix_millis()
                .saturating_sub(now_unix_millis)
                > crate::constants::MAX_REQUEST_TTL_MILLIS
        {
            return Err(ProtocolError::InvalidExpiry);
        }
        authenticator.verify(
            AuthenticationDomain::Request,
            &self.request_digest(),
            self.authentication_tag(),
        )?;
        Ok(AuthenticatedRequest::from_verified(self))
    }

    pub fn is_bound_to(&self, hello: &UntrustedBrokerHello) -> bool {
        self.version() == hello.version()
            && self.protocol_generation() == hello.protocol_generation()
            && self.nonce() == hello.client_nonce()
            && self.broker_nonce() == hello.broker_nonce()
            && self.correlation() == hello.correlation()
            && self.client_process_id() == hello.client_process_id()
            && self.client_process_epoch() == hello.client_process_epoch()
            && self.client_session_id() == hello.client_session_id()
            && self.broker_process_id() == hello.broker_process_id()
            && self.broker_session_id() == hello.broker_session_id()
            && self.broker_epoch() == hello.broker_epoch()
            && self.broker_key_epoch() == hello.broker_key_epoch()
            && self.writer_lease_epoch() == hello.writer_lease_epoch()
            && self.watermark() == hello.watermark()
            && self.session_handle() == hello.session_handle()
            && self.transcript_digest() == hello.transcript_digest()
    }
}
