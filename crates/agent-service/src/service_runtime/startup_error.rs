use super::NetworkRuntimeStartupError;
use ocentra_parent_agent_protocol::constants;

pub(super) fn network_runtime_startup_error(
    error: ocentra_eventing::error::EventingError,
) -> NetworkRuntimeStartupError {
    match error {
        ocentra_eventing::error::EventingError::InvalidValue { field, value }
            if field == constants::network_flow::NETWORK_RUNTIME_SPINE_FIELD
                && value
                    == constants::network_flow::NETWORK_RUNTIME_SPINE_JOURNAL_PATH_MISMATCH =>
        {
            NetworkRuntimeStartupError::SpineJournalPathMismatch
        }
        _ => NetworkRuntimeStartupError::Spine,
    }
}
