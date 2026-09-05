//! Managed profile storage is intentionally unavailable until a dependency-
//! owned protected-custody adapter can authenticate an owner receipt/key and
//! retain no-follow, handle-bound root/profile identity. This module exposes
//! no path, metadata, or mutation fallback while that adapter is absent.

use super::BrowserManagedProfileStoreError;

pub(crate) fn profile_store_error_reason(error: &BrowserManagedProfileStoreError) -> &'static str {
    match error {
        BrowserManagedProfileStoreError::ProtectedCustodyAdapterUnavailable => {
            ocentra_parent_agent_protocol::constants::browser::
                PROFILE_STORE_REASON_PROTECTED_CUSTODY_ADAPTER_UNAVAILABLE
        }
    }
}
