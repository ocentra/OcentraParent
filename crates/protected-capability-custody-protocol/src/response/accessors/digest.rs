use sha2::{Digest, Sha256};

use crate::request::authenticated::AuthenticatedRequest;
use crate::types::{AuthenticationDomain, BootstrapAuthenticator, ProtocolError};

use super::super::{ObservedGenerations, UntrustedResponse};

impl UntrustedResponse {
    pub(crate) fn is_bound_to(&self, request: &AuthenticatedRequest) -> bool {
        let request = request.as_untrusted();
        self.session == request.session
            && self.request_kind == request.kind()
            && self.request_digest == request.request_digest()
            && self.status.is_compatible_with(request.kind())
    }

    pub fn verify_authenticated_session(
        &self,
        request: &AuthenticatedRequest,
        now_unix_millis: u64,
        authenticator: &BootstrapAuthenticator,
    ) -> Result<(), ProtocolError> {
        if !self.is_bound_to(request) {
            return Err(ProtocolError::AuthenticationFailed);
        }
        if now_unix_millis == 0 || now_unix_millis >= self.expires_at_unix_millis() {
            return Err(ProtocolError::InvalidExpiry);
        }
        authenticator.verify(
            AuthenticationDomain::Response,
            &self.response_digest(),
            self.authentication_tag(),
        )
    }

    pub fn response_digest(&self) -> [u8; crate::constants::RESPONSE_DIGEST_BYTES] {
        let mut canonical = Vec::with_capacity(384);
        canonical.extend_from_slice(&self.version().value().to_be_bytes());
        canonical.extend_from_slice(&self.protocol_generation().value().to_be_bytes());
        canonical.extend_from_slice(self.nonce().as_bytes());
        canonical.extend_from_slice(self.broker_nonce().as_bytes());
        canonical.extend_from_slice(self.correlation().as_bytes());
        canonical.extend_from_slice(&self.client_process_id().to_be_bytes());
        canonical.extend_from_slice(&self.client_process_epoch().to_be_bytes());
        canonical.extend_from_slice(&self.client_session_id().to_be_bytes());
        canonical.extend_from_slice(&self.broker_process_id().to_be_bytes());
        canonical.extend_from_slice(&self.broker_session_id().to_be_bytes());
        canonical.extend_from_slice(&self.broker_epoch().to_be_bytes());
        canonical.extend_from_slice(&self.broker_key_epoch().to_be_bytes());
        canonical.extend_from_slice(&self.writer_lease_epoch().to_be_bytes());
        canonical.extend_from_slice(&self.watermark().to_be_bytes());
        canonical.extend_from_slice(self.session_handle().as_bytes());
        canonical.extend_from_slice(self.transcript_digest().as_bytes());
        canonical.extend_from_slice(&self.sequence().to_be_bytes());
        canonical.extend_from_slice(&self.expires_at_unix_millis().to_be_bytes());
        canonical.push(self.request_kind() as u8);
        canonical.extend_from_slice(&self.request_digest());
        canonical.push(self.status() as u8);
        append_observed_generations(&mut canonical, self.observed_generations());
        append_opaque_token_digest(&mut canonical, self.opaque_token_digest());
        let mut digest = Sha256::new();
        digest.update((crate::constants::RESPONSE_DIGEST_DOMAIN.len() as u32).to_be_bytes());
        digest.update(crate::constants::RESPONSE_DIGEST_DOMAIN.as_bytes());
        digest.update((canonical.len() as u32).to_be_bytes());
        digest.update(&canonical);
        digest.finalize().into()
    }
}

fn append_observed_generations(canonical: &mut Vec<u8>, generations: Option<ObservedGenerations>) {
    if let Some(generations) = generations {
        canonical.push(1);
        canonical.extend_from_slice(&generations.authority().to_be_bytes());
        canonical.extend_from_slice(&generations.target().to_be_bytes());
        canonical.extend_from_slice(&generations.key().to_be_bytes());
        canonical.extend_from_slice(&generations.writer().to_be_bytes());
    } else {
        canonical.push(0);
    }
}

fn append_opaque_token_digest(canonical: &mut Vec<u8>, digest: Option<[u8; 32]>) {
    if let Some(digest) = digest {
        canonical.push(1);
        canonical.extend_from_slice(&digest);
    } else {
        canonical.push(0);
    }
}
