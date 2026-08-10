use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, PolicyRequestParentResolutionRequest, PolicyRequestParentResolutionResult,
};
use ocentra_policy_control_core::policy_request::{ChildPolicyRequest, PolicyRequestResolution};
use std::fmt::{Debug, Display};

use crate::activity_store_path::activity_db_path;

use super::types::{AuditEventId, ErrorMessage, RequestIdText, ResolutionError};

use super::audit;

const ACTIVITY_STORE_OPEN_ERROR: &str = "activity-store-open";
const ACTIVITY_STORE_LOOKUP_ERROR: &str = "activity-store-lookup";
const ACTIVITY_STORE_TASK_ERROR: &str = "activity-store-task";

#[derive(Clone, Copy)]
enum StoreErrorKind {
    Open,
    Lookup,
    Task,
}

pub(crate) async fn load_audit_fields(
    event_id: AuditEventId,
) -> Result<Option<LogFields>, ResolutionError> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(&path).map_err(|error| {
            ResolutionError::from_message(store_debug_error(StoreErrorKind::Open, error))
        })?;
        store
            .enforcement_audit_fields_by_event_id(&event_id.0)
            .map_err(|error| {
                ResolutionError::from_message(store_debug_error(StoreErrorKind::Lookup, error))
            })
    })
    .await
    .map_err(|error| {
        ResolutionError::from_message(store_display_error(StoreErrorKind::Task, error))
    })?
}

pub(crate) async fn load_previous_resolution_fields(
    request_id: RequestIdText,
) -> Result<Option<LogFields>, ResolutionError> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(&path).map_err(|error| {
            ResolutionError::from_message(store_debug_error(StoreErrorKind::Open, error))
        })?;
        store
            .latest_matching_enforcement_audit_fields(|fields| {
                canonical_request_id(fields)
                    .as_ref()
                    .is_some_and(|candidate| candidate.0 == request_id.0)
            })
            .map_err(|error| {
                ResolutionError::from_message(store_debug_error(StoreErrorKind::Lookup, error))
            })
    })
    .await
    .map_err(|error| {
        ResolutionError::from_message(store_display_error(StoreErrorKind::Task, error))
    })?
}

fn canonical_request_id(fields: &LogFields) -> Option<RequestIdText> {
    let LogFieldValue::String(serialized) =
        fields.get(constants::policy_control::request::FIELD_CANONICAL_RESOLVED_REQUEST_JSON)?
    else {
        return None;
    };
    serde_json::from_str::<ChildPolicyRequest>(serialized)
        .ok()
        .map(|request| RequestIdText(request.request_id.as_str().to_string()))
}

fn store_debug_error(kind: StoreErrorKind, error: impl Debug) -> ErrorMessage {
    let mut message = kind.to_string();
    message.push(':');
    message.push(' ');
    message.push_str(&format!("{error:?}"));
    ErrorMessage(message)
}

fn store_display_error(kind: StoreErrorKind, error: impl Display) -> ErrorMessage {
    let mut message = kind.to_string();
    message.push(':');
    message.push(' ');
    message.push_str(&error.to_string());
    ErrorMessage(message)
}

impl Display for StoreErrorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Open => ACTIVITY_STORE_OPEN_ERROR,
            Self::Lookup => ACTIVITY_STORE_LOOKUP_ERROR,
            Self::Task => ACTIVITY_STORE_TASK_ERROR,
        };
        formatter.write_str(value)
    }
}

pub(crate) async fn persist_resolution(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestParentResolutionRequest,
    resolution: &PolicyRequestResolution,
    result: &PolicyRequestParentResolutionResult,
) -> bool {
    let Some(event) = audit::build_event(command, request, resolution, result) else {
        return false;
    };
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        ActivityStore::open(&path)
            .and_then(|store| store.ingest_events(&[event]))
            .is_ok()
    })
    .await
    .unwrap_or(false)
}
