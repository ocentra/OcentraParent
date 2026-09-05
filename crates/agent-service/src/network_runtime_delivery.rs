use std::sync::Arc;

use ocentra_eventing::{error::EventingError, replay::ReplayFilter};
use tokio::sync::OnceCell;

use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    network_capture::NetworkObservation,
    network_event_runtime::{
        NetworkRuntimeJournalPath, NetworkRuntimeJournalState, NetworkRuntimeReport,
        NetworkRuntimeSpine,
    },
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel,
};

use crate::activity_capture_network_observation::NetworkCaptureObservation;
#[path = "network_runtime_delivery/projection.rs"]
mod projection;
#[path = "network_runtime_delivery/reconciliation.rs"]
mod reconciliation;
#[path = "network_runtime_delivery/row_validation.rs"]
mod row_validation;
struct NetworkRuntimeSpineState {
    spine: Arc<NetworkRuntimeSpine>,
    journal_path: NetworkRuntimeJournalPath,
}

static NETWORK_RUNTIME_SPINE: OnceCell<NetworkRuntimeSpineState> = OnceCell::const_new();
const ALL_RETAINED_NETWORK_ROWS_LIMIT: u64 = i64::MAX as u64;
const ERROR_DETAIL_SEPARATOR: &str = ": ";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct NetworkRuntimeServiceDeliveryReport {
    pub(crate) observed_rows: usize,
    pub(crate) delivered_rows: usize,
    pub(crate) failed_rows: usize,
    pub(crate) publish_reports: usize,
    pub(crate) stored_events: usize,
    pub(crate) dead_letters: usize,
    pub(crate) manual_required_rows: usize,
    pub(crate) enforcement_command_events: usize,
    pub(crate) journal_state: NetworkRuntimeJournalState,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct NetworkRuntimeProjectionRowCounts {
    pub(crate) delivered_rows: usize,
    pub(crate) failed_rows: usize,
    pub(crate) manual_required_rows: usize,
}

pub(crate) async fn publish_captured_network_observations(
    observations: &[NetworkCaptureObservation],
) -> Result<(), EventingError> {
    let spine = shared_network_runtime_spine().await?;
    let mut failed_source_event_ids = Vec::new();
    for captured in observations {
        if let Err(error) = spine
            .publish_observation_chain_for_source_event(
                &captured.source_event_id,
                captured.observation.clone(),
                &captured.observed_at,
            )
            .await
        {
            failed_source_event_ids.push((captured.source_event_id.clone(), error));
        }
    }
    if failed_source_event_ids.is_empty() {
        Ok(())
    } else {
        let failed_rows = failed_source_event_ids.len();
        Err(EventingError::InvalidValue {
            field: constants::network_flow::NETWORK_RUNTIME_CAPTURE_PUBLISH_FIELD,
            value: failed_rows.to_string(),
        })
    }
}

pub(crate) async fn reconcile_retained_network_runtime() -> Result<(), EventingError> {
    let read_model = tokio::task::spawn_blocking(|| {
        let path = crate::activity_store_path::activity_db_path();
        let store = ActivityStore::open(path).map_err(|error| {
            startup_reconciliation_error(&StartupReconciliationField::ACTIVITY_STORE, error)
        })?;
        store
            .network_flow_read_model(
                ALL_RETAINED_NETWORK_ROWS_LIMIT,
                &crate::time::timestamp_now::<String>(),
            )
            .map_err(|error| {
                startup_reconciliation_error(&StartupReconciliationField::READ_MODEL, error)
            })
    })
    .await
    .map_err(|error| startup_reconciliation_error(&StartupReconciliationField::JOIN, error))??;

    let observations = read_model
        .rows
        .iter()
        .rev()
        .map(|row| {
            Ok(NetworkCaptureObservation {
                source_event_id: row.event_id.clone(),
                observed_at: row.observed_at.clone(),
                observation: network_runtime_observation_from_row(row)?,
            })
        })
        .collect::<Result<Vec<_>, EventingError>>()?;
    reconciliation::publish_missing_observations(observations).await
}

struct StartupReconciliationField(&'static str);

impl StartupReconciliationField {
    const ACTIVITY_STORE: Self =
        Self(constants::network_flow::NETWORK_RUNTIME_ACTIVITY_STORE_FIELD);
    const READ_MODEL: Self =
        Self(constants::network_flow::NETWORK_RUNTIME_ACTIVITY_STORE_READ_MODEL_FIELD);
    const JOIN: Self = Self(constants::network_flow::NETWORK_RUNTIME_ACTIVITY_STORE_JOIN_FIELD);
}

fn startup_reconciliation_error(
    field: &StartupReconciliationField,
    error: impl std::fmt::Debug,
) -> EventingError {
    let mut value =
        String::from(constants::network_flow::NETWORK_RUNTIME_STARTUP_RECONCILIATION_FAILURE);
    value.push_str(ERROR_DETAIL_SEPARATOR);
    value.push_str(&format!("{error:?}"));
    EventingError::InvalidValue {
        field: field.0,
        value,
    }
}

pub(crate) async fn read_network_runtime_delivery_for_read_model(
    read_model: &ActivityNetworkFlowReadModel,
) -> NetworkRuntimeServiceDeliveryReport {
    let mut delivery = NetworkRuntimeServiceDeliveryReport {
        observed_rows: read_model.rows.len(),
        journal_state: NetworkRuntimeJournalState::UnavailableManualRequired,
        ..NetworkRuntimeServiceDeliveryReport::default()
    };
    match durable_network_runtime_projection(read_model).await {
        Ok(report) => delivery.record_projection(read_model, &report),
        Err(_) => delivery.failed_rows = delivery.observed_rows,
    }
    delivery
}

pub(crate) async fn initialize_network_runtime_spine(
    path: &NetworkRuntimeJournalPath,
) -> Result<(), EventingError> {
    if let Some(state) = NETWORK_RUNTIME_SPINE.get() {
        return ensure_network_runtime_journal_path(state, path);
    }
    let spine = Arc::new(NetworkRuntimeSpine::with_durable_journal(path).await?);
    let state = NetworkRuntimeSpineState {
        spine,
        journal_path: path.clone(),
    };
    if NETWORK_RUNTIME_SPINE.set(state).is_err() {
        return NETWORK_RUNTIME_SPINE
            .get()
            .map(|state| ensure_network_runtime_journal_path(state, path))
            .unwrap_or_else(|| {
                Err(EventingError::InvalidValue {
                    field: constants::network_flow::NETWORK_RUNTIME_SPINE_FIELD,
                    value: constants::network_flow::NETWORK_RUNTIME_SPINE_INIT_FAILURE.to_string(),
                })
            });
    }
    Ok(())
}

fn ensure_network_runtime_journal_path(
    state: &NetworkRuntimeSpineState,
    path: &NetworkRuntimeJournalPath,
) -> Result<(), EventingError> {
    if state.journal_path == *path {
        Ok(())
    } else {
        Err(EventingError::InvalidValue {
            field: constants::network_flow::NETWORK_RUNTIME_SPINE_FIELD,
            value: constants::network_flow::NETWORK_RUNTIME_SPINE_JOURNAL_PATH_MISMATCH.to_string(),
        })
    }
}

pub(crate) async fn shared_network_runtime_spine() -> Result<Arc<NetworkRuntimeSpine>, EventingError>
{
    NETWORK_RUNTIME_SPINE
        .get()
        .map(|state| Arc::clone(&state.spine))
        .ok_or_else(|| EventingError::InvalidValue {
            field: constants::network_flow::NETWORK_RUNTIME_SPINE_FIELD,
            value: constants::network_flow::NETWORK_RUNTIME_SPINE_NOT_INITIALIZED.to_string(),
        })
}

pub(crate) async fn durable_network_runtime_projection(
    read_model: &ActivityNetworkFlowReadModel,
) -> Result<NetworkRuntimeReport, EventingError> {
    let spine = shared_network_runtime_spine().await?;
    let projection = spine.replay_projection(ReplayFilter::all()).await?;
    let retained_event_ids = projection::retained_event_ids_for_rows(read_model);
    Ok(NetworkRuntimeReport {
        publish_reports: Vec::new(),
        stored_events: projection
            .records
            .into_iter()
            .map(|record| record.envelope)
            .filter(|event| retained_event_ids.contains(&event.event_id))
            .collect(),
        dead_letters: Vec::new(),
        handled_phases: Vec::new(),
        journal_state: NetworkRuntimeJournalState::Durable,
    })
}

pub(crate) fn network_runtime_observation_from_row(
    row: &ActivityNetworkFlowObservation,
) -> Result<NetworkObservation, EventingError> {
    let validated = row_validation::validate(row)?;
    let status = validated.status;
    if status != ActivityCaptureCapabilityStatus::Available {
        return Ok(NetworkObservation::degraded(status));
    }

    let associated_pid_count =
        row.associated_pid_count
            .ok_or_else(|| EventingError::InvalidValue {
                field: constants::field::ASSOCIATED_PID_COUNT,
                value: constants::network_flow::NETWORK_RUNTIME_STARTUP_RECONCILIATION_FAILURE
                    .to_string(),
            })?;
    Ok(NetworkObservation {
        status,
        protocol: validated.protocol,
        local_ip: row.local_endpoint.ip.clone(),
        local_port: row.local_endpoint.port,
        destination_ip: row.destination_endpoint.ip.clone(),
        destination_port: row.destination_endpoint.port,
        destination_domain: row.destination_domain.clone(),
        tcp_state: validated.tcp_state,
        pid: validated.process_id,
        process_name: row.process_name.clone(),
        associated_pid_count,
    })
}

pub(crate) fn network_runtime_projection_row_counts(
    read_model: &ActivityNetworkFlowReadModel,
    report: &NetworkRuntimeReport,
) -> NetworkRuntimeProjectionRowCounts {
    projection::network_runtime_projection_row_counts(read_model, report)
}

impl NetworkRuntimeServiceDeliveryReport {
    fn record_projection(
        &mut self,
        read_model: &ActivityNetworkFlowReadModel,
        report: &NetworkRuntimeReport,
    ) {
        self.observed_rows = read_model.rows.len();
        let counts = projection::network_runtime_projection_row_counts(read_model, report);
        self.delivered_rows = counts.delivered_rows;
        self.failed_rows = counts.failed_rows;
        self.stored_events = report.stored_events.len();
        self.journal_state = report.journal_state;
        self.manual_required_rows = counts.manual_required_rows;
        self.enforcement_command_events = projection::count_event_type(
            report,
            projection::EventNameRef(constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED),
        );
    }
}
