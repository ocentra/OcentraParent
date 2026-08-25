//! Private typed authentication material and bounded wire validation.

use crate::{Error, InputFault, Result, MAX_BUFFER_BYTES};

#[path = "tpm_codec_auth_wire.rs"]
mod wire;

const SHA256_DIGEST_BYTES: usize = 32;
const MAX_TPM_NONCE_BYTES: usize = 64;
const MAX_AUTH_VALUE_BYTES: usize = 64;
const MAX_TPM_AUTH_SESSIONS: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Sha256Digest([u8; SHA256_DIGEST_BYTES]);

impl Sha256Digest {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let array = <[u8; SHA256_DIGEST_BYTES]>::try_from(bytes)
            .map_err(|_| Error::InvalidInput(InputFault::TpmCommandShapeInvalid))?;
        Ok(Self(array))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Nonce(Vec<u8>);

impl Nonce {
    pub(crate) fn from_caller_entropy(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() || bytes.len() > MAX_TPM_NONCE_BYTES {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        Ok(Self(bytes.to_vec()))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ProvisionedAuthValue(Vec<u8>);

impl ProvisionedAuthValue {
    pub(crate) fn from_installer_custody(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() || bytes.len() > MAX_AUTH_VALUE_BYTES {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        Ok(Self(bytes.to_vec()))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct AuthorizationArea(Vec<u8>);

impl AuthorizationArea {
    pub(crate) fn from_policy_session_wire(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() || bytes.len() > MAX_BUFFER_BYTES {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        wire::validate_authorization_area(bytes)?;
        Ok(Self(bytes.to_vec()))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
