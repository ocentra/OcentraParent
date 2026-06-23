use ocentra_parent_agent_protocol::browser_policy::{
    BrowserPolicyRejectionReason, BrowserPolicyUpdateKind, BrowserPolicyUpdateResponse,
    BrowserPolicyUpdateStatus,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::{
    browser_policy_payload::browser_policy_response_payload,
    browser_policy_request::{
        kind_for_command, parse_browser_policy_request, request_id_from_command,
    },
    browser_policy_runtime::BrowserPolicyRuntime,
    event_builder::build_event,
    time::timestamp_now,
};

pub async fn build_browser_policy_event(
    runtime: BrowserPolicyRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let response = match parse_browser_policy_request(&command) {
        Ok(request) => runtime.handle_request(request).await,
        Err(reason) => invalid_request_response(&command, reason),
    };
    let event_name = event_name_for_response(response.kind, response.status);
    let event_id = event_id_for_response(response.kind, response.status);
    let severity = match response.status {
        BrowserPolicyUpdateStatus::Accepted => LogLevel::Info,
        BrowserPolicyUpdateStatus::Rejected => LogLevel::Warn,
    };
    build_event(
        event_id,
        &command.message_id,
        command.source.clone(),
        event_name,
        severity,
        browser_policy_response_payload(&response),
        None,
    )
}

fn invalid_request_response(
    command: &AgentCommandEnvelope,
    reason: BrowserPolicyRejectionReason,
) -> BrowserPolicyUpdateResponse {
    BrowserPolicyUpdateResponse {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        request_id: request_id_from_command(command),
        kind: kind_for_command(command),
        status: BrowserPolicyUpdateStatus::Rejected,
        policy: None,
        effective_policy: None,
        capability_registry: Some(
            crate::browser_policy_compiler::browser_policy_capability_registry(&timestamp_now()),
        ),
        rejection_reason: Some(reason),
        audit_event_id: None,
        message: Some(constants::browser_policy::MESSAGE_INVALID_REQUEST.to_string()),
    }
}

fn event_id_for_response(
    kind: BrowserPolicyUpdateKind,
    status: BrowserPolicyUpdateStatus,
) -> &'static str {
    match (kind, status) {
        (BrowserPolicyUpdateKind::Preview, _) => constants::event_id::BROWSER_POLICY_PREVIEWED,
        (BrowserPolicyUpdateKind::Patch, BrowserPolicyUpdateStatus::Accepted) => {
            constants::event_id::BROWSER_POLICY_PATCH_ACCEPTED
        }
        (BrowserPolicyUpdateKind::Patch, BrowserPolicyUpdateStatus::Rejected) => {
            constants::event_id::BROWSER_POLICY_PATCH_REJECTED
        }
        (BrowserPolicyUpdateKind::Replace, BrowserPolicyUpdateStatus::Accepted) => {
            constants::event_id::BROWSER_POLICY_REPLACE_ACCEPTED
        }
        (BrowserPolicyUpdateKind::Replace, BrowserPolicyUpdateStatus::Rejected) => {
            constants::event_id::BROWSER_POLICY_REPLACE_REJECTED
        }
        (BrowserPolicyUpdateKind::Rollback, BrowserPolicyUpdateStatus::Accepted) => {
            constants::event_id::BROWSER_POLICY_ROLLBACK_ACCEPTED
        }
        (BrowserPolicyUpdateKind::Rollback, BrowserPolicyUpdateStatus::Rejected) => {
            constants::event_id::BROWSER_POLICY_ROLLBACK_REJECTED
        }
        _ => constants::event_id::BROWSER_POLICY_REPORTED,
    }
}

fn event_name_for_response(
    kind: BrowserPolicyUpdateKind,
    status: BrowserPolicyUpdateStatus,
) -> AgentEventName {
    match (kind, status) {
        (BrowserPolicyUpdateKind::Preview, _) => AgentEventName::AgentBrowserPolicyPreviewed,
        (BrowserPolicyUpdateKind::Patch, BrowserPolicyUpdateStatus::Accepted) => {
            AgentEventName::AgentBrowserPolicyPatchAccepted
        }
        (BrowserPolicyUpdateKind::Patch, BrowserPolicyUpdateStatus::Rejected) => {
            AgentEventName::AgentBrowserPolicyPatchRejected
        }
        (BrowserPolicyUpdateKind::Replace, BrowserPolicyUpdateStatus::Accepted) => {
            AgentEventName::AgentBrowserPolicyReplaceAccepted
        }
        (BrowserPolicyUpdateKind::Replace, BrowserPolicyUpdateStatus::Rejected) => {
            AgentEventName::AgentBrowserPolicyReplaceRejected
        }
        (BrowserPolicyUpdateKind::Rollback, BrowserPolicyUpdateStatus::Accepted) => {
            AgentEventName::AgentBrowserPolicyRollbackAccepted
        }
        (BrowserPolicyUpdateKind::Rollback, BrowserPolicyUpdateStatus::Rejected) => {
            AgentEventName::AgentBrowserPolicyRollbackRejected
        }
        _ => AgentEventName::AgentBrowserPolicyReported,
    }
}
