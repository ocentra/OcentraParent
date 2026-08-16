use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, PolicyRequestParentResolutionRequest, PolicyRequestParentResolutionResult,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use super::types::{AuditEventId, CommandId, RejectionReason, SnapshotError};
use super::{apply, result, snapshot};

const INVALID_REQUEST: &str = "invalid-request";

pub(crate) async fn execute(command: &AgentCommandEnvelope) -> PolicyRequestParentResolutionResult {
    let Some(request) = parse_request(command) else {
        return reject_unparsed(command);
    };
    if request.schema_version != AGENT_PROTOCOL_SCHEMA_VERSION {
        return reject_invalid_schema(&request);
    }

    let snapshot = match snapshot::load(&request).await {
        Ok(snapshot) => snapshot,
        Err(error) => return reject_snapshot(&request, error),
    };
    apply::resolve(command, &request, snapshot).await
}

fn parse_request(command: &AgentCommandEnvelope) -> Option<PolicyRequestParentResolutionRequest> {
    command
        .payload
        .get(constants::field::POLICY_REQUEST_PARENT_RESOLUTION_REQUEST)
        .and_then(|value| match value {
            LogFieldValue::String(text) => serde_json::from_str(text).ok(),
            _ => None,
        })
}

fn reject_unparsed(command: &AgentCommandEnvelope) -> PolicyRequestParentResolutionResult {
    result::rejected(
        CommandId(command.message_id.clone()),
        AuditEventId(String::new()),
        None,
        PolicyRequestStatus::PreviewOnly,
        RejectionReason(INVALID_REQUEST.to_string()),
        false,
    )
}

fn reject_invalid_schema(
    request: &PolicyRequestParentResolutionRequest,
) -> PolicyRequestParentResolutionResult {
    result::rejected(
        CommandId(request.command_id.clone()),
        AuditEventId(request.confirmed_audit_reference_id.clone()),
        None,
        PolicyRequestStatus::PreviewOnly,
        RejectionReason(INVALID_REQUEST.to_string()),
        false,
    )
}

fn reject_snapshot(
    request: &PolicyRequestParentResolutionRequest,
    error: SnapshotError,
) -> PolicyRequestParentResolutionResult {
    result::rejected(
        CommandId(request.command_id.clone()),
        AuditEventId(request.confirmed_audit_reference_id.clone()),
        error.request_id,
        error.status,
        error.reason,
        error.lookup_claimed,
    )
}
