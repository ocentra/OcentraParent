use ocentra_parent_agent_protocol::constants;
use rusqlite::Connection;

use crate::ActivityStoreError;

pub(crate) fn initialize_connection(connection: &Connection) -> Result<(), ActivityStoreError> {
    connection.execute_batch(constants::sqlite::INITIALIZE_ACTIVITY_STORE)?;
    Ok(())
}
