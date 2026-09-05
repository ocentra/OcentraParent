use super::types::{
    BrowserActivityReadModelAgentServiceSnapshot, BrowserEvidenceReadModelAgentServiceSnapshot,
    BrowserInterventionReadModelAgentServiceSnapshot,
    BrowserInventoryReadModelAgentServiceSnapshot, BrowserManagedStatusAgentServiceSnapshot,
};
use super::*;
use ocentra_parent_agent_protocol::activity_surface::ActivityBrowserReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::{AgentEventEnvelope, AgentEventName};

use super::payload_fields::serialized_enum_label;
use super::snapshots_network::response_json_payload_field;
use super::transport::rejection_message;

pub(crate) fn browser_managed_status_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<BrowserManagedStatusAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    expect_event(
        &response_event,
        &AgentEventName::AgentBrowserManagedStatusReported,
        "browser managed status",
    )?;
    let event = events.last().cloned().ok_or_else(|| {
        "agent-service browser managed status result did not include a response event".to_string()
    })?;
    let status = serde_json::from_value(response_json_payload_field(
        &response_event,
        constants::field::BROWSER_MANAGED_STATUS_JSON,
    )?)
    .map_err(|error| format!("agent-service browser managed status parse failed: {error}"))?;
    Ok(BrowserManagedStatusAgentServiceSnapshot { event, status })
}

pub(crate) fn browser_activity_read_model_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<BrowserActivityReadModelAgentServiceSnapshot, String> {
    let read_model = super::snapshots_tracking::activity_surface_snapshot_from_result::<
        ActivityBrowserReadModel,
    >(
        result,
        &AgentEventName::AgentActivityBrowserReadModelReported,
        constants::activity_surface::READ_MODEL_BROWSER,
        "browser",
    )?;
    Ok(BrowserActivityReadModelAgentServiceSnapshot { read_model })
}

pub(crate) fn browser_inventory_read_model_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<BrowserInventoryReadModelAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    expect_event(
        &response_event,
        &AgentEventName::AgentBrowserInventoryReadModelReported,
        "browser inventory read model",
    )?;
    let event = events.last().cloned().ok_or_else(|| {
        "agent-service browser inventory read model result did not include a response event"
            .to_string()
    })?;
    let read_model = serde_json::from_value(response_json_payload_field(
        &response_event,
        constants::field::BROWSER_INVENTORY_READ_MODEL_JSON,
    )?)
    .map_err(|error| format!("agent-service browser inventory read model parse failed: {error}"))?;
    Ok(BrowserInventoryReadModelAgentServiceSnapshot { event, read_model })
}

pub(crate) fn browser_evidence_read_model_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<BrowserEvidenceReadModelAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    expect_event(
        &response_event,
        &AgentEventName::AgentBrowserEvidenceRecentReported,
        "browser evidence read model",
    )?;
    let event = events.last().cloned().ok_or_else(|| {
        "agent-service browser evidence read model result did not include a response event"
            .to_string()
    })?;
    let read_model = serde_json::from_value(response_json_payload_field(
        &response_event,
        constants::field::BROWSER_EVIDENCE_READ_MODEL_JSON,
    )?)
    .map_err(|error| format!("agent-service browser evidence read model parse failed: {error}"))?;
    Ok(BrowserEvidenceReadModelAgentServiceSnapshot { event, read_model })
}

pub(crate) fn browser_intervention_read_model_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<BrowserInterventionReadModelAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    expect_event(
        &response_event,
        &AgentEventName::AgentBrowserInterventionReadModelReported,
        "browser intervention read model",
    )?;
    let event = events.last().cloned().ok_or_else(|| {
        "agent-service browser intervention read model result did not include a response event"
            .to_string()
    })?;
    let read_model = serde_json::from_value(response_json_payload_field(
        &response_event,
        constants::field::BROWSER_INTERVENTION_READ_MODEL_JSON,
    )?)
    .map_err(|error| {
        format!("agent-service browser intervention read model parse failed: {error}")
    })?;
    Ok(BrowserInterventionReadModelAgentServiceSnapshot { event, read_model })
}

fn expect_event(
    response_event: &AgentEventEnvelope,
    expected: &AgentEventName,
    label: &str,
) -> Result<(), String> {
    if &response_event.event == expected {
        return Ok(());
    }
    Err(format!(
        "agent-service expected {} for {label}, received {}",
        serialized_enum_label(expected),
        serialized_enum_label(&response_event.event)
    ))
}
