//! Lifetime, open, create, and currentness mechanics for AccountIssuer P-256.

#![cfg(windows)]

use super::cng_account_issuer_p256_capability::AccountIssuerP256Key;
use super::cng_account_issuer_p256_security::{
    observe_key, valid_service_security, validate_service_binding,
};
use crate::{account_issuer_types::AccountIssuerP256Observation, Error, OwnedService, Result};
pub(super) fn observe(key: &AccountIssuerP256Key) -> Result<AccountIssuerP256Observation> {
    observe_key(key.provider, key.key)
}

pub(super) fn revalidate(key: &AccountIssuerP256Key) -> Result<()> {
    let current = observe(key)?;
    if let Some(expected) = &key.observation {
        if &current != expected {
            return Err(Error::CryptoPropertyViolation);
        }
    }
    if let Some(service_sid) = &key.service_sid {
        if !valid_service_security(current.security(), service_sid) {
            return Err(Error::CryptoPropertyViolation);
        }
    }
    Ok(())
}

pub(super) fn bind_to_service(
    key: &mut AccountIssuerP256Key,
    service: &OwnedService,
) -> Result<()> {
    let service_sid = validate_service_binding(service)?;
    let current = observe(key)?;
    if !valid_service_security(current.security(), &service_sid) {
        return Err(Error::CryptoPropertyViolation);
    }
    if let Some(expected) = &key.observation {
        if expected.public_key_sec1() != current.public_key_sec1()
            || expected.key_name() != current.key_name()
            || expected.algorithm() != current.algorithm()
        {
            return Err(Error::CryptoPropertyViolation);
        }
    }
    key.observation = Some(current);
    key.service_sid = Some(service_sid);
    Ok(())
}
