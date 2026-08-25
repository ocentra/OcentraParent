use ocentra_parent_agent_protocol::browser::BrowserCustodyLabel;
use ocentra_parent_agent_protocol::browser_intervention::BrowserInterventionReadModel;
use ocentra_parent_agent_protocol::browser_intervention::BrowserInterventionRow;
use ocentra_parent_agent_protocol::browser_intervention::BROWSER_INTERVENTION_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::browser_intervention_values::{
    BrowserBoundaryState, BrowserExactUrlClaimState, BrowserInterventionCapabilityState,
    BrowserInterventionDeliveryState, BrowserUnmanagedDetectionState,
    BrowserUnmanagedFallbackActionState,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_unmanaged_enforcement::BrowserUnmanagedEnforcementState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::{BrowserInterventionAction, BrowserInterventionOutcome};
use rusqlite::{params, Connection, Row};

use crate::{ActivityStore, ActivityStoreError};

#[path = "activity_store_browser_intervention_fallback.rs"]
mod activity_store_browser_intervention_fallback;
mod fields;
use fields::{
    browser_boundary_state_field, browser_channel_field, browser_family_field, custody_label_field,
    decision_source_field, exact_url_claim_state_field, intervention_action_field,
    intervention_capability_field, intervention_delivery_state_field, intervention_mechanism_field,
    intervention_outcome_field, intervention_target_type_field, query_visibility_field,
    string_field, string_list_field, u32_field, unmanaged_detection_state_field,
    unmanaged_enforcement_field,
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
        .filter_map(|row| browser_intervention_read_row_from_store(&row))
        .collect::<Vec<_>>();
    let latest = read_rows.first();
    let managed_session_intervention_capability = latest
        .map(|row| row.managed_session_intervention_capability)
        .unwrap_or(BrowserInterventionCapabilityState::NeedsManagedSession);
    let unmanaged_browser_enforcement = latest
        .map(|row| row.unmanaged_browser_enforcement)
        .unwrap_or(BrowserUnmanagedEnforcementState::RequiresOsAppControl);
    let unmanaged_fallback_action = latest
        .map(activity_store_browser_intervention_fallback::top_level_unmanaged_fallback_action)
        .unwrap_or(BrowserUnmanagedFallbackActionState::Unavailable);
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
        unmanaged_fallback_action,
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
    unmanaged_fallback_action: BrowserUnmanagedFallbackActionState,
    intervention: BrowserInterventionRow,
}

struct BrowserInterventionDerivedFields {
    managed_browser_session_id: Option<String>,
    profile_id: Option<String>,
    process_id: Option<u32>,
    requested_url: Option<String>,
    observed_url: Option<String>,
    browser_boundary_state: BrowserBoundaryState,
    exact_url_claim_state: BrowserExactUrlClaimState,
    unmanaged_detection_state: BrowserUnmanagedDetectionState,
    unmanaged_fallback_action: BrowserUnmanagedFallbackActionState,
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
    row: &BrowserInterventionStoreRow,
) -> Option<BrowserInterventionReadRow> {
    let fields = &row.fields;
    let managed_session_intervention_capability =
        intervention_capability_field(fields).unwrap_or(BrowserInterventionCapabilityState::Ready);
    let unmanaged_browser_enforcement = unmanaged_enforcement_field(fields)
        .unwrap_or(BrowserUnmanagedEnforcementState::MonitorOnly);
    let derived = browser_intervention_derived_fields(fields, &unmanaged_browser_enforcement);
    let intervention = browser_intervention_row_from_fields(row, fields, &derived)?;

    Some(BrowserInterventionReadRow {
        event_id: row.event_id.clone(),
        observed_at: row.observed_at.clone(),
        managed_session_intervention_capability,
        unmanaged_browser_enforcement,
        unmanaged_fallback_action: derived.unmanaged_fallback_action,
        intervention,
    })
}

fn browser_intervention_derived_fields(
    fields: &LogFields,
    unmanaged_browser_enforcement: &BrowserUnmanagedEnforcementState,
) -> BrowserInterventionDerivedFields {
    let managed_browser_session_id =
        string_field(fields, constants::field::MANAGED_BROWSER_SESSION_ID);
    let profile_id = string_field(fields, constants::field::PROFILE_ID);
    let process_id = u32_field(fields, constants::field::PROCESS_ID);
    let requested_url = string_field(fields, constants::field::REQUESTED_URL);
    let observed_url = string_field(fields, constants::field::OBSERVED_URL);
    let browser_boundary_state = browser_boundary_state_field(fields)
        .unwrap_or_else(|| inferred_browser_boundary_state(&managed_browser_session_id));
    let exact_url_claim_state = exact_url_claim_state_field(fields).unwrap_or_else(|| {
        inferred_exact_url_claim_state(
            &browser_boundary_state,
            &managed_browser_session_id,
            &requested_url,
            &observed_url,
        )
    });
    let unmanaged_detection_state = unmanaged_detection_state_field(fields)
        .unwrap_or_else(|| inferred_unmanaged_detection_state(&browser_boundary_state));
    let unmanaged_fallback_action =
        string_field(fields, constants::field::UNMANAGED_FALLBACK_ACTION)
            .and_then(|value| BrowserUnmanagedFallbackActionState::from_protocol_str(&value))
            .unwrap_or_else(|| {
                inferred_unmanaged_fallback_action(
                    &browser_boundary_state,
                    unmanaged_browser_enforcement,
                    &unmanaged_detection_state,
                    &intervention_action_field(fields),
                    &intervention_outcome_field(fields),
                )
            });

    BrowserInterventionDerivedFields {
        managed_browser_session_id,
        profile_id,
        process_id,
        requested_url,
        observed_url,
        browser_boundary_state,
        exact_url_claim_state,
        unmanaged_detection_state,
        unmanaged_fallback_action,
    }
}

fn browser_intervention_row_from_fields(
    row: &BrowserInterventionStoreRow,
    fields: &LogFields,
    derived: &BrowserInterventionDerivedFields,
) -> Option<BrowserInterventionRow> {
    let exact_url_visible = matches!(
        derived.browser_boundary_state,
        BrowserBoundaryState::ManagedSession
    ) && derived.managed_browser_session_id.is_some()
        && matches!(
            derived.exact_url_claim_state,
            BrowserExactUrlClaimState::ExactUrlProven
        );
    Some(BrowserInterventionRow {
        schema_version: BROWSER_INTERVENTION_SCHEMA_VERSION,
        browser_intervention_id: string_field(fields, constants::field::BROWSER_INTERVENTION_ID)?,
        observed_at: row.observed_at.clone(),
        source_id: string_field(fields, constants::field::SOURCE_ID)?,
        device_id: row.device_id.clone(),
        browser_family: browser_family_field(fields),
        browser_channel: browser_channel_field(fields),
        managed_browser_session_id: derived.managed_browser_session_id.clone(),
        profile_id: derived.profile_id.clone(),
        process_id: derived.process_id,
        intervention_action_id: string_field(
            fields,
            constants::field::BROWSER_INTERVENTION_ACTION_ID,
        ),
        intervention_audit_id: string_field(
            fields,
            constants::field::BROWSER_INTERVENTION_AUDIT_ID,
        ),
        evidence_reference_ids: string_list_field(fields, constants::field::EVIDENCE_REFERENCE_IDS),
        policy_decision_id: string_field(fields, constants::field::POLICY_DECISION_ID),
        decision_source: decision_source_field(fields)?,
        intervention_action: intervention_action_field(fields)?,
        intervention_target_type: intervention_target_type_field(fields)?,
        intervention_target_value: string_field(
            fields,
            constants::field::INTERVENTION_TARGET_VALUE,
        )?,
        requested_url: exact_url_visible
            .then(|| derived.requested_url.clone())
            .flatten(),
        observed_url: exact_url_visible
            .then(|| derived.observed_url.clone())
            .flatten(),
        intervention_mechanism: intervention_mechanism_field(fields)?,
        intervention_outcome: intervention_outcome_field(fields)?,
        browser_boundary_state: derived.browser_boundary_state,
        exact_url_claim_state: derived.exact_url_claim_state,
        unmanaged_detection_state: derived.unmanaged_detection_state,
        unmanaged_fallback_action: derived.unmanaged_fallback_action,
        child_delivery_state: intervention_delivery_state_field(fields)
            .unwrap_or(BrowserInterventionDeliveryState::NotDelivered),
        reason: string_field(fields, constants::field::REASON),
        custody_label: custody_label_field(fields).unwrap_or(BrowserCustodyLabel::ChildDeviceLocal),
        query_visibility: query_visibility_field(fields)
            .unwrap_or(BrowserQueryVisibilityLabel::LiveLocal),
    })
}

fn inferred_browser_boundary_state(
    managed_browser_session_id: &Option<String>,
) -> BrowserBoundaryState {
    if managed_browser_session_id.is_some() {
        BrowserBoundaryState::ManagedSession
    } else {
        BrowserBoundaryState::Unknown
    }
}

fn inferred_exact_url_claim_state(
    browser_boundary_state: &BrowserBoundaryState,
    managed_browser_session_id: &Option<String>,
    requested_url: &Option<String>,
    observed_url: &Option<String>,
) -> BrowserExactUrlClaimState {
    if matches!(browser_boundary_state, BrowserBoundaryState::ManagedSession)
        && managed_browser_session_id.is_some()
        && requested_url.is_some()
        && observed_url.is_some()
    {
        BrowserExactUrlClaimState::ExactUrlProven
    } else {
        BrowserExactUrlClaimState::NotClaimed
    }
}

fn inferred_unmanaged_detection_state(
    browser_boundary_state: &BrowserBoundaryState,
) -> BrowserUnmanagedDetectionState {
    match browser_boundary_state {
        BrowserBoundaryState::ManagedSession => BrowserUnmanagedDetectionState::None,
        BrowserBoundaryState::UnmanagedBrowserProcess
        | BrowserBoundaryState::BrowserLikeProcess => BrowserUnmanagedDetectionState::Detected,
        BrowserBoundaryState::Unsupported | BrowserBoundaryState::Unknown => {
            BrowserUnmanagedDetectionState::Unavailable
        }
    }
}

fn inferred_unmanaged_fallback_action(
    browser_boundary_state: &BrowserBoundaryState,
    unmanaged_browser_enforcement: &BrowserUnmanagedEnforcementState,
    unmanaged_detection_state: &BrowserUnmanagedDetectionState,
    intervention_action: &Option<BrowserInterventionAction>,
    intervention_outcome: &Option<BrowserInterventionOutcome>,
) -> BrowserUnmanagedFallbackActionState {
    activity_store_browser_intervention_fallback::inferred_unmanaged_fallback_action(
        browser_boundary_state,
        unmanaged_browser_enforcement,
        unmanaged_detection_state,
        intervention_action,
        intervention_outcome,
    )
}
