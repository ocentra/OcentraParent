use super::super::signer_authority_types::LanSignedChildAuthorityBindingRef;

pub(super) fn binding_is_well_formed(binding: &LanSignedChildAuthorityBindingRef<'_>) -> bool {
    binding.authority_generation != 0
        && [
            binding.pairing_id,
            binding.child_device_id,
            binding.target_device_id,
            binding.install_id,
            binding.family_hash,
            binding.parent_device_id,
            binding.route_id,
            binding.registry_proof_digest,
        ]
        .iter()
        .all(|value| validate_registry_authority_identifier(value))
        && is_lower_hex(binding.public_key_id, 32)
        && is_lower_hex(binding.public_key_sha256, 64)
}

pub(super) fn validate_registry_authority_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
}

pub(super) fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
