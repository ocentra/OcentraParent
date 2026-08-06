use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use rusqlite::{params, Connection};

use crate::{ActivityStore, ActivityStoreError};

impl ActivityStore {
    pub fn latest_enforcement_audit_fields(&self) -> Result<Option<LogFields>, ActivityStoreError> {
        latest_enforcement_audit_fields(&self.connection)
    }

    pub fn latest_matching_enforcement_audit_fields(
        &self,
        mut predicate: impl FnMut(&LogFields) -> bool,
    ) -> Result<Option<LogFields>, ActivityStoreError> {
        latest_matching_enforcement_audit_fields(&self.connection, &mut predicate)
    }

    pub fn recent_enforcement_audit_fields(
        &self,
        limit: u64,
    ) -> Result<Vec<LogFields>, ActivityStoreError> {
        recent_enforcement_audit_fields(&self.connection, limit)
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

fn latest_matching_enforcement_audit_fields(
    connection: &Connection,
    predicate: &mut impl FnMut(&LogFields) -> bool,
) -> Result<Option<LogFields>, ActivityStoreError> {
    let mut statement =
        connection.prepare(constants::sqlite::SELECT_ENFORCEMENT_AUDIT_ACTIVITY_DESC)?;
    let mut rows = statement.query(params![
        constants::activity_event_kind::ENFORCEMENT_AUDIT_RECORDED
    ])?;
    while let Some(row) = rows.next()? {
        let fields_json: String = row.get(0)?;
        let fields = serde_json::from_str::<LogFields>(&fields_json)?;
        if predicate(&fields) {
            return Ok(Some(fields));
        }
    }
    Ok(None)
}

fn recent_enforcement_audit_fields(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<LogFields>, ActivityStoreError> {
    if limit == 0 {
        return Ok(Vec::new());
    }

    let mut statement =
        connection.prepare(constants::sqlite::SELECT_ENFORCEMENT_AUDIT_ACTIVITY_DESC)?;
    let mut rows = statement.query(params![
        constants::activity_event_kind::ENFORCEMENT_AUDIT_RECORDED
    ])?;
    let mut fields = Vec::new();
    while let Some(row) = rows.next()? {
        let fields_json: String = row.get(0)?;
        fields.push(serde_json::from_str::<LogFields>(&fields_json)?);
        if fields.len() as u64 == limit {
            break;
        }
    }
    Ok(fields)
}
