use rusqlite::Connection;

use crate::parent_presence_store::ParentPresenceStoreError;

pub(super) fn configure_runtime_durability(
    connection: &Connection,
) -> Result<(), ParentPresenceStoreError> {
    connection
        .execute_batch("PRAGMA synchronous = FULL;")
        .map_err(|_error| ParentPresenceStoreError::Unavailable)
}
