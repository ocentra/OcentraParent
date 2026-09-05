//! Lifetime, open, create, and currentness mechanics for AccountIssuer P-256.

#![cfg(windows)]

use super::cng_account_issuer_p256_acl::{valid_service_security, validate_service_binding};
use super::cng_account_issuer_p256_capability::{AccountIssuerP256Key, BoundAccountIssuerP256Key};
use super::cng_account_issuer_p256_security::observe_key;
use crate::{account_issuer_types::AccountIssuerP256Observation, Error, OwnedService, Result};

pub(super) fn observe_bound(
    key: &BoundAccountIssuerP256Key,
) -> Result<AccountIssuerP256Observation> {
    let current = observe_key(key.handles.provider, key.handles.key)?;
    if current != key.baseline || !valid_service_security(current.security(), &key.service_sid) {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(current)
}

pub(super) fn revalidate_bound(key: &BoundAccountIssuerP256Key) -> Result<()> {
    observe_bound(key).map(|_| ())
}

pub(super) fn bind_to_service(
    key: AccountIssuerP256Key,
    service: &OwnedService,
) -> Result<BoundAccountIssuerP256Key> {
    let service_sid = validate_service_binding(service)?;
    let current = observe_key(key.handles.provider, key.handles.key)?;
    if !valid_service_security(current.security(), &service_sid) {
        return Err(Error::CryptoPropertyViolation);
    }
    if !key.baseline.same_immutable_identity(&current)
        || (!key.permits_external_acl_transition && current != key.baseline)
    {
        return Err(Error::CryptoPropertyViolation);
    }
    let baseline = if key.permits_external_acl_transition {
        current
    } else {
        key.baseline
    };
    Ok(BoundAccountIssuerP256Key {
        handles: key.handles,
        baseline,
        service_sid,
    })
}
