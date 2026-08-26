//! Domain-bound SHA-256 and fixed-width low-S ECDSA signing.

#![cfg(windows)]

use super::cng_account_issuer_p256_capability::ACCOUNT_ISSUER_SIGNING_DOMAIN;
use crate::account_issuer_types::AccountIssuerP256Signature;
use crate::{Error, Result, MAX_BUFFER_BYTES};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use windows_sys::Win32::Security::Cryptography::{
    NCryptSignHash, NCRYPT_KEY_HANDLE, NCRYPT_SILENT_FLAG,
};

const P256_SIGNATURE_BYTES: usize = 64;
const P256_SCALAR_BYTES: usize = 32;
const P256_ORDER: [u8; P256_SCALAR_BYTES] = [
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xbc, 0xe6, 0xfa, 0xad, 0xa7, 0x17, 0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2, 0xfc, 0x63, 0x25, 0x51,
];
const P256_HALF_ORDER: [u8; P256_SCALAR_BYTES] = [
    0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xde, 0x73, 0x7d, 0x56, 0xd3, 0x8b, 0xcf, 0x42, 0x79, 0xdc, 0xe5, 0x61, 0x7e, 0x31, 0x92, 0xa8,
];

pub(super) fn sign_domain_bound_request(
    key: NCRYPT_KEY_HANDLE,
    canonical_payload: &[u8],
    service_bound: bool,
) -> Result<AccountIssuerP256Signature> {
    if !service_bound
        || canonical_payload.len() > MAX_BUFFER_BYTES
        || !canonical_payload.starts_with(ACCOUNT_ISSUER_SIGNING_DOMAIN)
    {
        return Err(Error::CryptoPropertyViolation);
    }
    let digest = Sha256::digest(canonical_payload);
    let mut signature = [0_u8; P256_SIGNATURE_BYTES];
    let mut written = 0_u32;
    let status = unsafe {
        NCryptSignHash(
            key,
            core::ptr::null(),
            digest.as_ptr(),
            u32::try_from(digest.len())?,
            signature.as_mut_ptr(),
            u32::try_from(signature.len())?,
            &mut written,
            NCRYPT_SILENT_FLAG,
        )
    };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    if usize::try_from(written)? != P256_SIGNATURE_BYTES {
        return Err(Error::CryptoPropertyViolation);
    }
    canonicalize_low_s(&mut signature)?;
    Ok(AccountIssuerP256Signature(signature))
}

fn canonicalize_low_s(signature: &mut [u8; P256_SIGNATURE_BYTES]) -> Result<()> {
    let (r, s) = signature.split_at_mut(P256_SCALAR_BYTES);
    if r.iter().all(|byte| *byte == 0)
        || s.iter().all(|byte| *byte == 0)
        || compare(s, &P256_ORDER) != Ordering::Less
    {
        return Err(Error::CryptoPropertyViolation);
    }
    if compare(s, &P256_HALF_ORDER) == Ordering::Greater {
        subtract_order(s);
    }
    if s.iter().all(|byte| *byte == 0) || compare(s, &P256_HALF_ORDER) == Ordering::Greater {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(())
}

fn compare(left: &[u8], right: &[u8]) -> Ordering {
    left.iter().cmp(right.iter())
}

fn subtract_order(value: &mut [u8]) {
    let mut borrow = 0_i16;
    for index in (0..value.len()).rev() {
        let difference = i16::from(P256_ORDER[index]) - i16::from(value[index]) - borrow;
        if difference < 0 {
            value[index] = (difference + 256) as u8;
            borrow = 1;
        } else {
            value[index] = difference as u8;
            borrow = 0;
        }
    }
}
