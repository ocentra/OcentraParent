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
    browser_policy_request::{kind_for_command, parse_browser_policy_request},
    browser_policy_runtime::BrowserPolicyRuntime,
    event_builder::build_event,
    time::timestamp_now,
};

const EVENT_METADATA_BY_STATUS: [[BrowserPolicyEventMetadata; 5]; 2] = [
    [
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_REPORTED,
            event_name: AgentEventName::AgentBrowserPolicyReported,
        },
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_PREVIEWED,
            event_name: AgentEventName::AgentBrowserPolicyPreviewed,
        },
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_PATCH_ACCEPTED,
            event_name: AgentEventName::AgentBrowserPolicyPatchAccepted,
        },
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_REPLACE_ACCEPTED,
            event_name: AgentEventName::AgentBrowserPolicyReplaceAccepted,
        },
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_ROLLBACK_ACCEPTED,
            event_name: AgentEventName::AgentBrowserPolicyRollbackAccepted,
        },
    ],
    [
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_REPORTED,
            event_name: AgentEventName::AgentBrowserPolicyReported,
        },
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_PREVIEWED,
            event_name: AgentEventName::AgentBrowserPolicyPreviewed,
        },
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_PATCH_REJECTED,
            event_name: AgentEventName::AgentBrowserPolicyPatchRejected,
        },
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_REPLACE_REJECTED,
            event_name: AgentEventName::AgentBrowserPolicyReplaceRejected,
        },
        BrowserPolicyEventMetadata {
            event_id: constants::event_id::BROWSER_POLICY_ROLLBACK_REJECTED,
            event_name: AgentEventName::AgentBrowserPolicyRollbackRejected,
        },
    ],
];

pub async fn build_browser_policy_event(
    runtime: BrowserPolicyRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let response = match parse_browser_policy_request(&command) {
        Ok(request) => runtime.handle_request(request).await,
        Err(reason) => invalid_request_response(&command, reason),
    };
    let metadata = event_metadata_for_response(response.kind, response.status);
    build_event(
        metadata.event_id,
        &command.message_id,
        command.source.clone(),
        metadata.event_name.clone(),
        severity_for_response(response.status),
        browser_policy_response_payload(&response),
        None,
    )
}

#[derive(Clone)]
struct BrowserPolicyEventMetadata {
    event_id: &'static str,
    event_name: AgentEventName,
}

fn invalid_request_response(
    command: &AgentCommandEnvelope,
    reason: BrowserPolicyRejectionReason,
) -> BrowserPolicyUpdateResponse {
    BrowserPolicyUpdateResponse {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        request_id: command.message_id.clone(),
        kind: kind_for_command(command),
        status: BrowserPolicyUpdateStatus::Rejected,
        policy: None,
        effective_policy: None,
        capability_registry: Some(
            crate::browser_policy_compiler::browser_policy_capability_registry(
                crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
                    generated_at: timestamp_now::<String>().as_str(),
                },
            ),
        ),
        rejection_reason: Some(reason),
        audit_event_id: None,
        message: Some(constants::browser_policy::MESSAGE_INVALID_REQUEST.to_string()),
    }
}

fn event_metadata_for_response(
    kind: BrowserPolicyUpdateKind,
    status: BrowserPolicyUpdateStatus,
) -> &'static BrowserPolicyEventMetadata {
    &EVENT_METADATA_BY_STATUS[status as usize][kind as usize]
}

fn severity_for_response(status: BrowserPolicyUpdateStatus) -> LogLevel {
    match status {
        BrowserPolicyUpdateStatus::Accepted => LogLevel::Info,
        BrowserPolicyUpdateStatus::Rejected => LogLevel::Warn,
    }
}
