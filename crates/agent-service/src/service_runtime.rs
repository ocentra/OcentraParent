use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::{
    activity_store_path::network_runtime_journal_path, fields::fields_from_pairs,
    network::NetworkPolicy,
};

#[path = "service_runtime/run.rs"]
mod run;
#[path = "service_runtime/startup_error.rs"]
mod startup_error;

const STARTUP_LOG_CONTEXT_FIELD: &str = "context";
const STARTUP_LOG_CONTEXT_VALUE: &str = "startup";

pub(crate) struct StartupErrorReason(pub(crate) String);

#[derive(Debug)]
pub enum NetworkRuntimeStartupError {
    Spine,
    SpineJournalPathMismatch,
    Reconciliation,
}

pub async fn run_agent_service() {
    run::run_agent_service().await;
}

pub async fn initialize_network_runtime() -> Result<(), NetworkRuntimeStartupError> {
    let runtime_journal_path = network_runtime_journal_path();
    crate::network_runtime_delivery::initialize_network_runtime_spine(&runtime_journal_path)
        .await
        .map_err(startup_error::network_runtime_startup_error)?;
    crate::network_runtime_delivery::reconcile_retained_network_runtime()
        .await
        .map_err(|_| NetworkRuntimeStartupError::Reconciliation)
}

pub fn startup_log_fields(network: &NetworkPolicy) -> LogFields {
    fields_from_pairs(vec![
        (
            STARTUP_LOG_CONTEXT_FIELD,
            LogFieldValue::String(STARTUP_LOG_CONTEXT_VALUE.to_string()),
        ),
        (
            constants::field::LOCAL_PORT,
            LogFieldValue::Number(f64::from(network.bind_address().port())),
        ),
    ])
}

pub(crate) fn startup_error_log_fields(
    network: &NetworkPolicy,
    reason: StartupErrorReason,
) -> LogFields {
    fields_from_pairs(vec![
        (
            STARTUP_LOG_CONTEXT_FIELD,
            LogFieldValue::String(STARTUP_LOG_CONTEXT_VALUE.to_string()),
        ),
        (
            constants::field::LOCAL_PORT,
            LogFieldValue::Number(f64::from(network.bind_address().port())),
        ),
        (constants::field::REASON, LogFieldValue::String(reason.0)),
    ])
}
