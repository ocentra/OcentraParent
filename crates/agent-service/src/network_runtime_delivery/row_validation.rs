use chrono::DateTime;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation;
use serde::de::DeserializeOwned;
use serde_json::Value;

const ERROR_DETAIL_SEPARATOR: &str = ": ";

pub(super) struct ValidatedNetworkRuntimeRow {
    pub(super) status: ActivityCaptureCapabilityStatus,
    pub(super) protocol: Option<ActivityNetworkProtocol>,
    pub(super) tcp_state: Option<ActivityNetworkTcpState>,
    pub(super) process_id: Option<u32>,
}

#[derive(Clone, Copy)]
struct PersistedText<'a>(&'a str);

#[derive(Clone, Copy)]
enum ValidationField {
    CapabilityStatus,
    NetworkProtocol,
    TcpState,
    ProcessId,
    ObservedAt,
}

pub(super) fn validate(
    row: &ActivityNetworkFlowObservation,
) -> Result<ValidatedNetworkRuntimeRow, EventingError> {
    let status = parse_required(
        ValidationField::CapabilityStatus,
        PersistedText(row.capability_status.as_str()),
    )?;
    validate_observed_at(PersistedText(row.observed_at.as_str()))?;
    let protocol = row
        .protocol
        .as_deref()
        .map(|value| parse_required(ValidationField::NetworkProtocol, PersistedText(value)))
        .transpose()?;
    let tcp_state = row
        .tcp_state
        .as_deref()
        .map(|value| parse_required(ValidationField::TcpState, PersistedText(value)))
        .transpose()?;
    let process_id = row
        .process_id
        .map(|value| {
            u32::try_from(value).map_err(|error| {
                invalid_value(
                    ValidationField::ProcessId,
                    InvalidPersistedValue(value.to_string()),
                    error,
                )
            })
        })
        .transpose()?;
    Ok(ValidatedNetworkRuntimeRow {
        status,
        protocol,
        tcp_state,
        process_id,
    })
}

fn parse_required<T>(field: ValidationField, value: PersistedText<'_>) -> Result<T, EventingError>
where
    T: DeserializeOwned,
{
    serde_json::from_value(Value::String(value.0.to_owned()))
        .map_err(|error| invalid_value(field, InvalidPersistedValue(value.0.to_owned()), error))
}

fn validate_observed_at(value: PersistedText<'_>) -> Result<(), EventingError> {
    DateTime::parse_from_rfc3339(value.0)
        .map(|_| ())
        .map_err(|error| {
            invalid_value(
                ValidationField::ObservedAt,
                InvalidPersistedValue(value.0.to_owned()),
                error,
            )
        })
}

struct InvalidPersistedValue(String);

fn invalid_value(
    field: ValidationField,
    value: InvalidPersistedValue,
    error: impl std::fmt::Display,
) -> EventingError {
    let field = match field {
        ValidationField::CapabilityStatus => constants::field::CAPABILITY_STATUS,
        ValidationField::NetworkProtocol => constants::field::NETWORK_PROTOCOL,
        ValidationField::TcpState => constants::field::TCP_STATE,
        ValidationField::ProcessId => constants::field::PROCESS_ID,
        ValidationField::ObservedAt => constants::field::OBSERVED_AT,
    };
    let mut value = value.0;
    value.push_str(ERROR_DETAIL_SEPARATOR);
    value.push_str(&error.to_string());
    EventingError::InvalidValue { field, value }
}
