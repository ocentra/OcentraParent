use std::fmt::Write;

use crate::parent_presence_store::ParentPresenceStoreError;

pub(crate) fn generate_opaque_receipt_ref() -> Result<String, ParentPresenceStoreError> {
    let mut entropy = [0_u8; 32];
    getrandom::fill(&mut entropy).map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    let mut encoded = String::with_capacity(64);
    for byte in entropy {
        write!(&mut encoded, "{byte:02x}")
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    }
    Ok(format!("parent-presence-receipt:{encoded}"))
}
