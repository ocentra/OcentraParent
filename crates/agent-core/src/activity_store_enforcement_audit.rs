use ocentra_parent_agent_protocol::{constants, LogFields};
use rusqlite::{params, Connection};

use crate::{ActivityStore, ActivityStoreError};

impl ActivityStore {
    pub fn latest_enforcement_audit_fields(&self) -> Result<Option<LogFields>, ActivityStoreError> {
        latest_enforcement_audit_fields(&self.connection)
    }
}

fn latest_enforcement_audit_fields(
    connection: &Connection,
) -> Result<Option<LogFields>, ActivityStoreError> {
    let mut statement =
        connection.prepare(constants::sqlite::SELECT_LATEST_ENFORCEMENT_AUDIT_ACTIVITY)?;
    let mut rows = statement.query(params![
        constants::activity_event_kind::ENFORCEMENT_AUDIT_RECORDED
    ])?;
    match rows.next()? {
        Some(row) => {
            let fields_json: String = row.get(0)?;
            Ok(Some(serde_json::from_str::<LogFields>(&fields_json)?))
        }
        None => Ok(None),
    }
}
