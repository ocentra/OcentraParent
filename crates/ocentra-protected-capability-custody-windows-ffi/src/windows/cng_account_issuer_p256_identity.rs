//! Immutable TPM/provider/key identity admission for AccountIssuer P-256.

#![cfg(windows)]

use super::cng_account_issuer_p256_capability::{
    ACCOUNT_ISSUER_ALGORITHM_GROUP_NAME, ACCOUNT_ISSUER_ALGORITHM_NAME, ACCOUNT_ISSUER_KEY_NAME,
};
use crate::account_issuer_types::AccountIssuerP256Observation;
use crate::{WindowsText, MAX_WIDE_CHARS};
use windows_sys::Win32::Security::Cryptography::{
    BCRYPT_RSAPUBLIC_MAGIC, NCRYPT_ALLOW_SIGNING_FLAG, NCRYPT_IMPL_HARDWARE_FLAG,
    NCRYPT_IMPL_REMOVABLE_FLAG, NCRYPT_IMPL_SOFTWARE_FLAG, NCRYPT_PCP_SIGNATURE_KEY,
    NCRYPT_PROV_HANDLE,
};

const TPM_MARKER: &str = "TPM";
const TPM2_VERSION_PREFIX: &str = "2.0";

pub(super) fn valid_identity(
    observation: &AccountIssuerP256Observation,
    observed_provider: NCRYPT_PROV_HANDLE,
    expected_provider: NCRYPT_PROV_HANDLE,
) -> bool {
    let implementation_mask =
        NCRYPT_IMPL_HARDWARE_FLAG | NCRYPT_IMPL_SOFTWARE_FLAG | NCRYPT_IMPL_REMOVABLE_FLAG;
    let exact_properties = (
        observation.key_name.as_str().as_bytes(),
        observation.algorithm.as_str().as_bytes(),
        observation.algorithm_group.as_str().as_bytes(),
        observation.implementation_type & implementation_mask,
        observation.export_policy,
        observation.key_usage,
        observation.pcp_key_usage_policy,
        observation.key_length_bits,
        observed_provider,
    );
    exact_properties
        == (
            ACCOUNT_ISSUER_KEY_NAME,
            ACCOUNT_ISSUER_ALGORITHM_NAME,
            ACCOUNT_ISSUER_ALGORITHM_GROUP_NAME,
            NCRYPT_IMPL_HARDWARE_FLAG,
            0,
            NCRYPT_ALLOW_SIGNING_FLAG,
            NCRYPT_PCP_SIGNATURE_KEY,
            256,
            expected_provider,
        )
        && observation.provider_version != 0
        && valid_stable_text(&observation.unique_name)
        && valid_tpm2_platform(&observation.platform_type)
        && valid_selected_ek_public_blob(&observation.ek_public)
        && valid_tpm2_sha256_name(&observation.tpm2b_name)
}

fn valid_stable_text(value: &WindowsText) -> bool {
    let text = value.as_str();
    (
        text.is_empty(),
        text.len() > MAX_WIDE_CHARS,
        text.trim() == text,
    ) == (false, false, true)
        && text.chars().all(|character| !character.is_control())
}

fn valid_tpm2_platform(value: &WindowsText) -> bool {
    let text = value.as_str();
    let ascii_text = text
        .bytes()
        .all(|byte| byte.is_ascii_graphic() || byte == b' ');
    if (valid_stable_text(value), text.len() <= 256, ascii_text) != (true, true, true) {
        return false;
    }
    let normalized = text.to_ascii_uppercase();
    let Some(tpm_index) = normalized.find(TPM_MARKER) else {
        return false;
    };
    let version = normalized[tpm_index + 3..]
        .trim_start_matches(|character: char| !character.is_ascii_digit());
    version.starts_with(TPM2_VERSION_PREFIX)
}

fn valid_tpm2_sha256_name(value: &[u8]) -> bool {
    const TPM2_SHA256_NAME_BYTES: usize = 36;
    const TPM2_SHA256_NAME_BODY_BYTES: u16 = 34;
    const TPM_ALG_SHA256: u16 = 0x000b;
    (
        value.len(),
        value.get(..2),
        value.get(2..4),
        value
            .get(4..)
            .map(|digest| digest.iter().any(|byte| *byte != 0)),
    ) == (
        TPM2_SHA256_NAME_BYTES,
        Some(TPM2_SHA256_NAME_BODY_BYTES.to_be_bytes().as_slice()),
        Some(TPM_ALG_SHA256.to_be_bytes().as_slice()),
        Some(true),
    )
}

/// Deliberate compatibility boundary: this packet admits only the TPM 2.0
/// RSA-2048 EK public blob with exponent 65537. Other EK profiles fail closed.
fn valid_selected_ek_public_blob(value: &[u8]) -> bool {
    const EK_RSA_BITS: u32 = 2048;
    const EK_MODULUS_BYTES: usize = (EK_RSA_BITS / 8) as usize;
    const HEADER_BYTES: usize = 24;
    const EXPONENT: [u8; 3] = [1, 0, 1];
    let expected_length = HEADER_BYTES + EXPONENT.len() + EK_MODULUS_BYTES;
    if value.len() != expected_length {
        return false;
    }
    let header = (
        read_u32(value, 0),
        read_u32(value, 4),
        read_u32(value, 8),
        read_u32(value, 12),
        read_u32(value, 16),
        read_u32(value, 20),
    );
    let modulus = &value[HEADER_BYTES + EXPONENT.len()..];
    header
        == (
            BCRYPT_RSAPUBLIC_MAGIC,
            EK_RSA_BITS,
            EXPONENT.len() as u32,
            EK_MODULUS_BYTES as u32,
            0,
            0,
        )
        && value[HEADER_BYTES..HEADER_BYTES + EXPONENT.len()] == EXPONENT
        && modulus.first().is_some_and(|byte| byte & 0x80 != 0)
        && modulus.iter().any(|byte| *byte != 0)
}

fn read_u32(value: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        value[offset],
        value[offset + 1],
        value[offset + 2],
        value[offset + 3],
    ])
}
