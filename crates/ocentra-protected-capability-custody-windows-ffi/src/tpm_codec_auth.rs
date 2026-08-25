//! Private TPM authentication state and fixed SHA-256 mechanics.

use crate::tpm::codec_types::handles::SessionHandle;
use crate::tpm::TPM_SHA256_BYTES;
use crate::{Error, Result};
use sha2::{Digest, Sha256};
use std::sync::atomic::{compiler_fence, Ordering};

#[path = "tpm_codec_auth_wire.rs"]
mod wire;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct Sha256Digest([u8; TPM_SHA256_BYTES]);

impl Sha256Digest {
    pub(crate) fn hash(parts: &[&[u8]]) -> Self {
        let mut hasher = Sha256::new();
        for part in parts {
            hasher.update(part);
        }
        Self(hasher.finalize().into())
    }

    pub(crate) fn as_bytes(&self) -> &[u8; TPM_SHA256_BYTES] {
        &self.0
    }
}

pub(crate) struct SecretNonce([u8; TPM_SHA256_BYTES]);

impl SecretNonce {
    pub(crate) fn from_os_random(bytes: [u8; TPM_SHA256_BYTES]) -> Self {
        Self(bytes)
    }

    pub(crate) fn from_tpm(bytes: &[u8]) -> Result<Self> {
        let value = <[u8; TPM_SHA256_BYTES]>::try_from(bytes).map_err(|_| Error::MalformedTpm)?;
        Ok(Self(value))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; TPM_SHA256_BYTES] {
        &self.0
    }
}

impl Drop for SecretNonce {
    fn drop(&mut self) {
        clear_bytes(&mut self.0);
    }
}

pub(crate) struct SecretSessionKey(Vec<u8>);

impl SecretSessionKey {
    pub(crate) fn unsalted_unbound() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Drop for SecretSessionKey {
    fn drop(&mut self) {
        clear_bytes(self.0.as_mut_slice());
    }
}

pub(crate) struct AuthorizationArea(Vec<u8>);

impl AuthorizationArea {
    pub(crate) fn policy(
        session: &SessionHandle,
        nonce_caller: &SecretNonce,
        attributes: u8,
    ) -> Result<Self> {
        wire::encode_policy_authorization(session, nonce_caller, attributes).map(Self)
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for AuthorizationArea {
    fn drop(&mut self) {
        clear_bytes(self.0.as_mut_slice());
    }
}

pub(crate) fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut difference = 0u8;
    for (left_byte, right_byte) in left.iter().zip(right) {
        difference |= left_byte ^ right_byte;
    }
    difference == 0
}

pub(crate) fn clear_bytes(bytes: &mut [u8]) {
    for byte in bytes {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}
