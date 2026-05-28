use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    BrowserPolicyUpdateKind, LogFieldValue, LogLevel,
};

use crate::{browser_policy_payload::browser_policy_scaffold_payload, event_builder::build_event};

pub fn build_browser_policy_scaffold_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let kind = kind_for_command(&command.command);
    build_event(
        event_id_for_command(&command.command),
        &command.message_id,
        command.source.clone(),
        event_name_for_command(&command.command),
        LogLevel::Warn,
        browser_policy_scaffold_payload(request_id_from_command(&command), kind),
        None,
    )
}

fn request_id_from_command(command: &AgentCommandEnvelope) -> String {
    match command
        .payload
        .get(constants::field::BROWSER_POLICY_REQUEST)
    {
        Some(LogFieldValue::String(text)) => serde_json::from_str::<serde_json::Value>(text)
            .ok()
            .and_then(|value| {
                value
                    .get(constants::field::BROWSER_POLICY_REQUEST_ID)
                    .and_then(|request_id| request_id.as_str().map(ToString::to_string))
            })
            .unwrap_or_else(|| command.message_id.clone()),
        _ => command.message_id.clone(),
    }
}

fn kind_for_command(command: &AgentCommandName) -> BrowserPolicyUpdateKind {
    match command {
        AgentCommandName::AgentBrowserPolicyPreview => BrowserPolicyUpdateKind::Preview,
        AgentCommandName::AgentBrowserPolicyPatch => BrowserPolicyUpdateKind::Patch,
        AgentCommandName::AgentBrowserPolicyReplace => BrowserPolicyUpdateKind::Replace,
        AgentCommandName::AgentBrowserPolicyRollback => BrowserPolicyUpdateKind::Rollback,
        _ => BrowserPolicyUpdateKind::Get,
    }
}

fn event_id_for_command(command: &AgentCommandName) -> &'static str {
    match command {
        AgentCommandName::AgentBrowserPolicyPreview => {
            constants::event_id::BROWSER_POLICY_PREVIEWED
        }
        AgentCommandName::AgentBrowserPolicyPatch => {
            constants::event_id::BROWSER_POLICY_PATCH_REJECTED
        }
        AgentCommandName::AgentBrowserPolicyReplace => {
            constants::event_id::BROWSER_POLICY_REPLACE_REJECTED
        }
        AgentCommandName::AgentBrowserPolicyRollback => {
            constants::event_id::BROWSER_POLICY_ROLLBACK_REJECTED
        }
        _ => constants::event_id::BROWSER_POLICY_REPORTED,
    }
}

fn event_name_for_command(command: &AgentCommandName) -> AgentEventName {
    match command {
        AgentCommandName::AgentBrowserPolicyPreview => AgentEventName::AgentBrowserPolicyPreviewed,
        AgentCommandName::AgentBrowserPolicyPatch => {
            AgentEventName::AgentBrowserPolicyPatchRejected
        }
        AgentCommandName::AgentBrowserPolicyReplace => {
            AgentEventName::AgentBrowserPolicyReplaceRejected
        }
        AgentCommandName::AgentBrowserPolicyRollback => {
            AgentEventName::AgentBrowserPolicyRollbackRejected
        }
        _ => AgentEventName::AgentBrowserPolicyReported,
    }
}
