use sha2::{Digest, Sha256};

use crate::account_issuer_contract::ACCOUNT_ISSUER_MAX_WIRE_BYTES;
use crate::account_issuer_session::{
    SessionBinding, ACCOUNT_ISSUER_SESSION_RECEIPT_DOMAIN, ACCOUNT_ISSUER_SESSION_REQUEST_DOMAIN,
};

pub(crate) fn request(
    binding: &SessionBinding,
    request_wire: &[u8],
) -> [u8; crate::constants::REQUEST_DIGEST_BYTES] {
    let mut canonical = Vec::with_capacity(512 + request_wire.len());
    append_binding(&mut canonical, binding);
    append_wire(&mut canonical, request_wire);
    digest(ACCOUNT_ISSUER_SESSION_REQUEST_DOMAIN, &canonical)
}

pub(crate) fn receipt(
    binding: &SessionBinding,
    request_digest: [u8; crate::constants::REQUEST_DIGEST_BYTES],
    receipt_wire: &[u8],
) -> [u8; crate::constants::RESPONSE_DIGEST_BYTES] {
    let mut canonical =
        Vec::with_capacity(512 + ACCOUNT_ISSUER_MAX_WIRE_BYTES.min(receipt_wire.len()));
    append_binding(&mut canonical, binding);
    canonical.extend_from_slice(&request_digest);
    append_wire(&mut canonical, receipt_wire);
    digest(ACCOUNT_ISSUER_SESSION_RECEIPT_DOMAIN, &canonical)
}

fn append_binding(canonical: &mut Vec<u8>, binding: &SessionBinding) {
    canonical.extend_from_slice(&binding.version.value().to_be_bytes());
    canonical.extend_from_slice(&binding.protocol_generation.value().to_be_bytes());
    canonical.extend_from_slice(binding.client_nonce.as_bytes());
    canonical.extend_from_slice(binding.broker_nonce.as_bytes());
    canonical.extend_from_slice(binding.correlation.as_bytes());
    canonical.extend_from_slice(&binding.client_process_id.to_be_bytes());
    canonical.extend_from_slice(&binding.client_process_epoch.to_be_bytes());
    canonical.extend_from_slice(&binding.client_session_id.to_be_bytes());
    canonical.extend_from_slice(&binding.broker_process_id.to_be_bytes());
    canonical.extend_from_slice(&binding.broker_session_id.to_be_bytes());
    canonical.extend_from_slice(&binding.broker_epoch.to_be_bytes());
    canonical.extend_from_slice(&binding.broker_key_epoch.to_be_bytes());
    canonical.extend_from_slice(&binding.writer_lease_epoch.to_be_bytes());
    canonical.extend_from_slice(&binding.watermark.to_be_bytes());
    canonical.extend_from_slice(binding.session_handle.as_bytes());
    canonical.extend_from_slice(binding.transcript_digest.as_bytes());
    canonical.extend_from_slice(&binding.sequence.to_be_bytes());
    canonical.extend_from_slice(&binding.expires_at_unix_millis.to_be_bytes());
}

fn append_wire(canonical: &mut Vec<u8>, wire: &[u8]) {
    canonical.extend_from_slice(&(wire.len() as u64).to_be_bytes());
    canonical.extend_from_slice(wire);
}

fn digest(domain: &[u8], canonical: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&(domain.len() as u32).to_be_bytes());
    hasher.update(domain);
    hasher.update(&(canonical.len() as u32).to_be_bytes());
    hasher.update(canonical);
    hasher.finalize().into()
}
