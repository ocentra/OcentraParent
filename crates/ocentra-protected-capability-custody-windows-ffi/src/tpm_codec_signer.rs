//! Canonical public-only RSA signer and fixed-width policy signature.

use super::auth::Sha256Digest;
use crate::tpm::{
    FIXED_RSA_BITS, FIXED_RSA_BYTES, TPM_ALG_NULL, TPM_ALG_RSA, TPM_ALG_RSAPSS, TPM_ALG_SHA256,
};
use crate::{Error, InputFault, Result};

const EXTERNAL_SIGNER_ATTRIBUTES: u32 = 0x0004_0440;

/// Canonical RSA-3072/PSS-SHA256 public verifier loaded into the TPM.
#[derive(Clone, Eq, PartialEq)]
pub struct TpmPolicySignerPublic {
    public: Vec<u8>,
    name: [u8; 34],
}

impl TpmPolicySignerPublic {
    /// Construct the one permitted external-key shape from a CNG-observed RSA
    /// modulus. The exponent, scheme, hash and TPM object attributes are fixed.
    pub fn from_rsa3072_modulus(modulus: &[u8]) -> Result<Self> {
        if modulus.len() != FIXED_RSA_BYTES
            || modulus[0] & 0x80 == 0
            || modulus.last().is_none_or(|last| last & 1 == 0)
        {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        let mut public = Vec::with_capacity(24 + FIXED_RSA_BYTES);
        push_u16(&mut public, TPM_ALG_RSA);
        push_u16(&mut public, TPM_ALG_SHA256);
        push_u32(&mut public, EXTERNAL_SIGNER_ATTRIBUTES);
        push_tpm2b(&mut public, &[])?;
        push_u16(&mut public, TPM_ALG_NULL);
        push_u16(&mut public, TPM_ALG_RSAPSS);
        push_u16(&mut public, TPM_ALG_SHA256);
        push_u16(&mut public, FIXED_RSA_BITS);
        push_u32(&mut public, 0);
        push_tpm2b(&mut public, modulus)?;
        let digest = Sha256Digest::hash(&[&public]);
        let mut name = [0u8; 34];
        name[..2].copy_from_slice(&TPM_ALG_SHA256.to_be_bytes());
        name[2..].copy_from_slice(digest.as_bytes());
        Ok(Self { public, name })
    }

    pub(crate) fn public(&self) -> &[u8] {
        &self.public
    }

    pub(crate) fn name(&self) -> &[u8; 34] {
        &self.name
    }
}

/// One fixed-width RSA-PSS-SHA256 signature produced by the enrolled PCP key.
pub struct TpmPolicySignature([u8; FIXED_RSA_BYTES]);

impl TpmPolicySignature {
    pub fn from_rsa_pss_sha256(bytes: &[u8]) -> Result<Self> {
        let signature = <[u8; FIXED_RSA_BYTES]>::try_from(bytes)
            .map_err(|_| Error::InvalidInput(InputFault::TpmCommandShapeInvalid))?;
        Ok(Self(signature))
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>> {
        let mut wire = Vec::with_capacity(6 + FIXED_RSA_BYTES);
        push_u16(&mut wire, TPM_ALG_RSAPSS);
        push_u16(&mut wire, TPM_ALG_SHA256);
        push_tpm2b(&mut wire, &self.0)?;
        Ok(wire)
    }
}

fn push_u16(output: &mut Vec<u8>, value: u16) {
    output.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_be_bytes());
}

fn push_tpm2b(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    output.extend_from_slice(&u16::try_from(bytes.len())?.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}
