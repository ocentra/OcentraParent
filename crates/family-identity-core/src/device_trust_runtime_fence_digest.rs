use sha2::{Digest, Sha256};

use super::DeviceTrustRuntimeFenceTarget;

pub(super) fn outcome_digest(
    operation_id: &str,
    reservation_ref: &str,
    target: &DeviceTrustRuntimeFenceTarget,
) -> String {
    let mut digest = Sha256::new();
    digest.update(b"ocentra-device-trust-runtime-fence-v1\0");
    for identity in [
        operation_id,
        reservation_ref,
        &target.family_id,
        &target.trust_subject,
        &target.parent_device_id,
        &target.child_device_id,
        &target.installation_id,
        &target.signer_key_id,
        &target.signer_key_sha256,
    ] {
        digest.update(identity.as_bytes());
        digest.update([0]);
    }
    for generation in [
        target.action_code as u64,
        target.lifecycle_generation,
        target.installation_binding_generation,
        target.authority_generation,
    ] {
        digest.update(generation.to_be_bytes());
    }
    let bytes = digest.finalize();
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push_str(&format!("{byte:02x}"));
    }
    encoded
}
