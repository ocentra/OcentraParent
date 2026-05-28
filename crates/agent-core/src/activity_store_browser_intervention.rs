use ocentra_parent_agent_protocol::{
    constants, BrowserBoundaryState, BrowserCustodyLabel, BrowserExactUrlClaimState,
    BrowserInterventionCapabilityState, BrowserInterventionReadModel, BrowserInterventionRow,
    BrowserQueryVisibilityLabel, BrowserUnmanagedDetectionState, BrowserUnmanagedEnforcementState,
    LogFields, BROWSER_INTERVENTION_SCHEMA_VERSION,
};
use rusqlite::{params, Connection, Row};

use crate::{ActivityStore, ActivityStoreError};

mod fields;
use fields::{
    browser_boundary_state_field, browser_channel_field, browser_family_field, custody_label_field,
    decision_source_field, exact_url_claim_state_field, intervention_action_field,
    intervention_capability_field, intervention_mechanism_field, intervention_outcome_field,
    intervention_target_type_field, query_visibility_field, string_field, u32_field,
    unmanaged_detection_state_field, unmanaged_enforcement_field,
};

impl ActivityStore {
    pub fn browser_intervention_read_model(
        &self,
        limit: u64,
        generated_at: &str,
    ) -> Result<BrowserInterventionReadModel, ActivityStoreError> {
        browser_intervention_read_model(&self.connection, limit, generated_at)
    }
}

pub(crate) fn browser_intervention_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<BrowserInterventionReadModel, ActivityStoreError> {
    let rows = browser_intervention_store_rows(connection, limit)?;
    let read_rows = rows
        .into_iter()
        .filter_map(browser_intervention_read_row_from_store)
        .collect::<Vec<_>>();
    let latest = read_rows.first();
    let managed_session_intervention_capability = latest
        .map(|row| row.managed_session_intervention_capability.clone())
        .unwrap_or(BrowserInterventionCapabilityState::NeedsManagedSession);
    let unmanaged_browser_enforcement = latest
        .map(|row| row.unmanaged_browser_enforcement.clone())
        .unwrap_or(BrowserUnmanagedEnforcementState::RequiresOsAppControl);
    let latest_event_id = latest.map(|row| row.event_id.clone());
    let latest_observed_at = latest.map(|row| row.observed_at.clone());
    let intervention_rows = read_rows
        .into_iter()
        .map(|row| row.intervention)
        .collect::<Vec<_>>();

    Ok(BrowserInterventionReadModel {
        schema_version: BROWSER_INTERVENTION_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        limit,
        returned: intervention_rows.len() as u64,
        latest_event_id,
        latest_observed_at,
        managed_session_intervention_capability,
        unmanaged_browser_enforcement,
        rows: intervention_rows,
    })
}

struct BrowserInterventionStoreRow {
    event_id: String,
    observed_at: String,
    device_id: String,
    fields: LogFields,
}

struct BrowserInterventionReadRow {
    event_id: String,
    observed_at: String,
    managed_session_intervention_capability: BrowserInterventionCapabilityState,
    unmanaged_browser_enforcement: BrowserUnmanagedEnforcementState,
    intervention: BrowserInterventionRow,
}

fn browser_intervention_store_rows(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<BrowserInterventionStoreRow>, ActivityStoreError> {
    let mut statement =
        connection.prepare(constants::sqlite::SELECT_RECENT_BROWSER_INTERVENTION_ACTIVITY)?;
    let rows = statement.query_map(
        params![
            constants::activity_event_kind::BROWSER_INTERVENTION_APPLIED,
            limit as i64
        ],
        browser_intervention_store_row_from_sqlite,
    )?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

fn browser_intervention_store_row_from_sqlite(
    row: &Row<'_>,
) -> rusqlite::Result<BrowserInterventionStoreRow> {
    let fields_json: String = row.get(3)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json)
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;

    Ok(BrowserInterventionStoreRow {
        event_id: row.get(0)?,
        observed_at: row.get(1)?,
        device_id: row.get(2)?,
        fields,
    })
}

fn browser_intervention_read_row_from_store(
    row: BrowserInterventionStoreRow,
) -> Option<BrowserInterventionReadRow> {
    let fields = &row.fields;
    let managed_session_intervention_capability =
        intervention_capability_field(fields).unwrap_or(BrowserInterventionCapabilityState::Ready);
    let unmanaged_browser_enforcement = unmanaged_enforcement_field(fields)
        .unwrap_or(BrowserUnmanagedEnforcementState::MonitorOnly);
    let intervention = BrowserInterventionRow {
        schema_version: BROWSER_INTERVENTION_SCHEMA_VERSION,
        browser_intervention_id: string_field(fields, constants::field::BROWSER_INTERVENTION_ID)?,
        observed_at: row.observed_at.clone(),
        source_id: string_field(fields, constants::field::SOURCE_ID)?,
        device_id: row.device_id,
        browser_family: browser_family_field(fields),
        browser_channel: browser_channel_field(fields),
        managed_browser_session_id: string_field(
            fields,
            constants::field::MANAGED_BROWSER_SESSION_ID,
        ),
        profile_id: string_field(fields, constants::field::PROFILE_ID),
        process_id: u32_field(fields, constants::field::PROCESS_ID),
        policy_decision_id: string_field(fields, constants::field::POLICY_DECISION_ID),
        decision_source: decision_source_field(fields)?,
        intervention_action: intervention_action_field(fields)?,
        intervention_target_type: intervention_target_type_field(fields)?,
        intervention_target_value: string_field(
            fields,
            constants::field::INTERVENTION_TARGET_VALUE,
        )?,
        requested_url: string_field(fields, constants::field::REQUESTED_URL),
        observed_url: string_field(fields, constants::field::OBSERVED_URL),
        intervention_mechanism: intervention_mechanism_field(fields)?,
        intervention_outcome: intervention_outcome_field(fields)?,
        browser_boundary_state: browser_boundary_state_field(fields)
            .unwrap_or(BrowserBoundaryState::ManagedSession),
        exact_url_claim_state: exact_url_claim_state_field(fields)
            .unwrap_or(BrowserExactUrlClaimState::ExactUrlProven),
        unmanaged_detection_state: unmanaged_detection_state_field(fields)
            .unwrap_or(BrowserUnmanagedDetectionState::None),
        reason: string_field(fields, constants::field::REASON),
        custody_label: custody_label_field(fields).unwrap_or(BrowserCustodyLabel::ChildDeviceLocal),
        query_visibility: query_visibility_field(fields)
            .unwrap_or(BrowserQueryVisibilityLabel::LiveLocal),
    };

    Some(BrowserInterventionReadRow {
        event_id: row.event_id,
        observed_at: row.observed_at,
        managed_session_intervention_capability,
        unmanaged_browser_enforcement,
        intervention,
    })
}
