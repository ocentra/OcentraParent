use sha2::{Digest, Sha256};

use crate::constants::{REQUEST_DIGEST_BYTES, REQUEST_DIGEST_DOMAIN};

use super::{RequestKind, UntrustedRequest};

impl RequestKind {
    pub(crate) fn decode(value: u8) -> Result<Self, crate::types::ProtocolError> {
        match value {
            1 => Ok(Self::Prepare),
            2 => Ok(Self::Commit),
            3 => Ok(Self::Abort),
            4 => Ok(Self::Recover),
            5 => Ok(Self::ResolveAmbiguity),
            other => Err(crate::types::ProtocolError::UnsupportedRequest(other)),
        }
    }

    pub(crate) fn requires_token(self) -> bool {
        matches!(self, Self::Commit | Self::Abort)
    }
}

impl UntrustedRequest {
    pub fn request_digest(&self) -> [u8; REQUEST_DIGEST_BYTES] {
        let mut canonical = Vec::with_capacity(512);
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
        let generations = self.expected_generations();
        canonical.extend_from_slice(&generations.authority().to_be_bytes());
        canonical.extend_from_slice(&generations.target().to_be_bytes());
        canonical.extend_from_slice(&generations.key().to_be_bytes());
        canonical.extend_from_slice(&generations.writer().to_be_bytes());
        canonical.push(self.kind() as u8);
        append_digest_field(&mut canonical, self.operation());
        canonical.push(self.action() as u8);
        canonical.push(self.target().kind() as u8);
        append_digest_field(&mut canonical, self.target().household());
        append_digest_field(&mut canonical, self.target().device());
        append_digest_field(&mut canonical, self.target().target());
        match self.opaque_token_digest() {
            Some(digest) => {
                canonical.push(1);
                canonical.extend_from_slice(&digest);
            }
            None => canonical.push(0),
        }
        let mut digest = Sha256::new();
        digest.update((REQUEST_DIGEST_DOMAIN.len() as u32).to_be_bytes());
        digest.update(REQUEST_DIGEST_DOMAIN.as_bytes());
        digest.update((canonical.len() as u32).to_be_bytes());
        digest.update(&canonical);
        digest.finalize().into()
    }
}

fn append_digest_field(canonical: &mut Vec<u8>, value: &[u8]) {
    canonical.extend_from_slice(&(value.len() as u32).to_be_bytes());
    canonical.extend_from_slice(value);
}
