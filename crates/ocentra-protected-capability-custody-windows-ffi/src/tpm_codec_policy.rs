//! Private policy, NV-public, and transient-object codec inputs.

use super::auth::Sha256Digest;
use super::handles::{NonNullHandle, NvIndex};
use crate::{Error, InputFault, Result};

const MAX_POLICY_SIGNATURE_BYTES: usize = 4096;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct NvPublicDefinition {
    pub(crate) index: NvIndex,
    pub(crate) attributes: u32,
    pub(crate) auth_policy: Sha256Digest,
    pub(crate) data_size: u16,
}

impl NvPublicDefinition {
    pub(crate) fn from_enrollment(
        index: NvIndex,
        attributes: u32,
        auth_policy: Sha256Digest,
        data_size: u16,
    ) -> Result<Self> {
        if attributes == 0 || data_size == 0 {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        Ok(Self {
            index,
            attributes,
            auth_policy,
            data_size,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ExternalObject {
    sensitive: Vec<u8>,
    public: Vec<u8>,
}

impl ExternalObject {
    pub(crate) fn from_tpm2b_fields(sensitive: &[u8], public: &[u8]) -> Result<Self> {
        if public.is_empty()
            || sensitive.len() > u16::MAX as usize
            || public.len() > u16::MAX as usize
        {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        Ok(Self {
            sensitive: sensitive.to_vec(),
            public: public.to_vec(),
        })
    }

    pub(crate) fn sensitive(&self) -> &[u8] {
        &self.sensitive
    }

    pub(crate) fn public(&self) -> &[u8] {
        &self.public
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PolicySignature(Vec<u8>);

impl PolicySignature {
    pub(crate) fn from_verified_signature(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty()
            || bytes.len() > MAX_POLICY_SIGNATURE_BYTES
            || !is_sha256_signature(bytes)
        {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        Ok(Self(bytes.to_vec()))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PolicyOrDigests(Vec<Sha256Digest>);

impl PolicyOrDigests {
    pub(crate) fn from_enrollment(digests: Vec<Sha256Digest>) -> Result<Self> {
        if !(2..=super::super::TPM_MAX_POLICY_OR_DIGESTS).contains(&digests.len()) {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        Ok(Self(digests))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Sha256Digest> {
        self.0.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PolicySessionStart {
    pub(crate) tpm_key: NonNullHandle,
    pub(crate) bind: NonNullHandle,
    pub(crate) nonce_caller: super::auth::Nonce,
}

impl PolicySessionStart {
    pub(crate) fn from_enrollment(
        tpm_key: NonNullHandle,
        bind: NonNullHandle,
        nonce_caller: super::auth::Nonce,
    ) -> Self {
        Self {
            tpm_key,
            bind,
            nonce_caller,
        }
    }
}

fn is_sha256_signature(bytes: &[u8]) -> bool {
    let mut cursor = 0usize;
    let Ok(algorithm) = take_u16(bytes, &mut cursor) else {
        return false;
    };
    let result = match algorithm {
        0x0014 | 0x0016 => {
            let Ok(hash) = take_u16(bytes, &mut cursor) else {
                return false;
            };
            hash == super::super::TPM_ALG_SHA256
                && take_tpm2b(bytes, &mut cursor).is_ok_and(|value| !value.is_empty())
        }
        0x0018 | 0x001a | 0x001b | 0x001c => {
            let Ok(hash) = take_u16(bytes, &mut cursor) else {
                return false;
            };
            hash == super::super::TPM_ALG_SHA256
                && take_tpm2b(bytes, &mut cursor).is_ok_and(|value| !value.is_empty())
                && take_tpm2b(bytes, &mut cursor).is_ok_and(|value| !value.is_empty())
        }
        _ => false,
    };
    result && cursor == bytes.len()
}

fn take_u16(bytes: &[u8], cursor: &mut usize) -> Result<u16> {
    let end = cursor.checked_add(2).ok_or(Error::MalformedTpm)?;
    if end > bytes.len() {
        return Err(Error::MalformedTpm);
    }
    let value = u16::from_be_bytes([bytes[*cursor], bytes[*cursor + 1]]);
    *cursor = end;
    Ok(value)
}

fn take_tpm2b<'a>(bytes: &'a [u8], cursor: &mut usize) -> Result<&'a [u8]> {
    let length = usize::from(take_u16(bytes, cursor)?);
    let end = cursor.checked_add(length).ok_or(Error::MalformedTpm)?;
    if end > bytes.len() {
        return Err(Error::MalformedTpm);
    }
    let value = &bytes[*cursor..end];
    *cursor = end;
    Ok(value)
}
