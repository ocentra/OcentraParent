//! Exact service identity and key security-descriptor admission.

#![cfg(windows)]

use super::cng_account_issuer_p256_capability::BROKER_SERVICE_NAME;
use crate::{Error, OwnedService, Result, SecurityDescriptorObservation};
use windows_sys::Win32::Foundation::GENERIC_ALL;

const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1;
const ALLOWED_ACE_TYPE: u8 = 0;
const TRUSTED_INSTALLER_SID: &[u8] = &[
    1, 6, 0, 0, 0, 0, 0, 5, 80, 0, 0, 0, 181, 137, 251, 56, 25, 132, 194, 203, 92, 108, 35, 109,
    87, 0, 119, 110, 192, 2, 100, 135,
];
const SYSTEM_SID: &[u8] = &[1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
const EVERYONE_SID: &[u8] = &[1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
const AUTHENTICATED_USERS_SID: &[u8] = &[1, 1, 0, 0, 0, 0, 0, 5, 11, 0, 0, 0];
const BUILTIN_ADMINISTRATORS_SID: &[u8] = &[1, 2, 0, 0, 0, 0, 0, 5, 32, 0, 0, 0, 32, 2, 0, 0];
const BUILTIN_USERS_SID: &[u8] = &[1, 2, 0, 0, 0, 0, 0, 5, 32, 0, 0, 0, 33, 2, 0, 0];

pub(super) fn valid_service_security(
    security: &SecurityDescriptorObservation,
    service_sid: &[u8],
) -> bool {
    let header = (
        service_sid.is_empty(),
        security.owner_sid(),
        security.owner_was_defaulted(),
        security.dacl_is_present(),
        security.dacl_was_defaulted(),
        security.dacl_is_protected(),
        security.dacl().len(),
    );
    if header != (false, TRUSTED_INSTALLER_SID, false, true, false, true, 1) {
        return false;
    }
    let Some(ace) = security.dacl().first() else {
        return false;
    };
    (
        ace.ace_type(),
        ace.flags(),
        ace.access_mask(),
        ace.sid(),
        ace.raw().is_empty(),
        is_broad_sid(ace.sid()),
    ) == (ALLOWED_ACE_TYPE, 0, GENERIC_ALL, service_sid, false, false)
}

pub(super) fn validate_service_binding(service: &OwnedService) -> Result<Vec<u8>> {
    let observation = service.observation()?;
    let identity = (
        observation.service_name().as_str().as_bytes(),
        observation.service_sid_type(),
    );
    if identity != (BROKER_SERVICE_NAME, SERVICE_SID_TYPE_UNRESTRICTED) {
        return Err(Error::CryptoPropertyViolation);
    }
    service.service_sid()
}

pub(super) fn valid_base_security(security: &SecurityDescriptorObservation) -> bool {
    let header = (
        security.owner_sid().is_empty(),
        security.owner_was_defaulted(),
        security.dacl_is_present(),
        security.dacl_was_defaulted(),
        security.dacl_is_protected(),
        security.dacl().is_empty(),
    );
    header == (false, false, true, false, true, false)
        && security.dacl().iter().all(|ace| {
            (ace.ace_type(), ace.flags(), is_broad_sid(ace.sid())) == (ALLOWED_ACE_TYPE, 0, false)
        })
}

fn is_broad_sid(sid: &[u8]) -> bool {
    [
        SYSTEM_SID,
        EVERYONE_SID,
        AUTHENTICATED_USERS_SID,
        BUILTIN_ADMINISTRATORS_SID,
        BUILTIN_USERS_SID,
    ]
    .contains(&sid)
}
