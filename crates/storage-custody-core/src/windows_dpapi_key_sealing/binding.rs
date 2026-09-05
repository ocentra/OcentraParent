use ocentra_family_identity_core::trust_bootstrap::PersistedPlatformKeyUnsealingCredential;
use sha2::{Digest, Sha256};

use super::DpapiKeySealingContext;

pub(super) fn authorization_binding(
    context: &DpapiKeySealingContext,
    credential: &PersistedPlatformKeyUnsealingCredential,
    device_local_binding: &[u8; 32],
) -> [u8; 32] {
    Sha256::digest(canonical_binding_bytes(
        context,
        credential,
        device_local_binding,
    ))
    .into()
}

fn canonical_binding_bytes(
    context: &DpapiKeySealingContext,
    credential: &PersistedPlatformKeyUnsealingCredential,
    device_local_binding: &[u8; 32],
) -> Vec<u8> {
    let fields = [
        format!(
            "ocentra-device-trust-dpapi-v{}",
            super::SEALED_KEY_FORMAT_VERSION
        ),
        context.family_id.clone(),
        context.trust_subject.clone(),
        context.device_ref.clone(),
        context.device_role.clone(),
        context.lifecycle_generation.to_string(),
        context.installation_binding_generation.to_string(),
        credential.trust_bootstrap_ref().to_owned(),
        credential.device_trust_ref().as_str().to_owned(),
        hex_encode(device_local_binding),
    ];
    let mut encoded = Vec::new();
    for field in fields {
        let field = hex_encode(field.as_bytes());
        encoded.extend_from_slice(format!("{:016x}:", field.len()).as_bytes());
        encoded.extend_from_slice(field.as_bytes());
    }
    encoded
}

pub(super) fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}
