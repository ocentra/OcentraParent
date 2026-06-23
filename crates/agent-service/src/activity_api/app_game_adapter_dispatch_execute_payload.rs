use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::{
    AppGameAdapterDispatchResultRow, APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND,
    APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT,
    APP_GAME_ADAPTER_DISPATCH_RESULT_READBACK_COMMAND,
    APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
};

use super::app_game_adapter_dispatch_result_payload::app_game_adapter_dispatch_result_read_model_with_execution;
use crate::enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths};
use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

pub async fn build_activity_app_game_adapter_dispatch_execute_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_activity_app_game_adapter_dispatch_execute_report_with_paths(
        command,
        EnforcementJournalPaths::from_environment(),
    )
    .await
}

pub(crate) async fn build_activity_app_game_adapter_dispatch_execute_report_with_paths(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
) -> AgentEventEnvelope {
    let target = command.source.clone();
    let correlation_id = command.message_id.clone();
    if command.target.platform != constants::enforcement::PLATFORM_WINDOWS {
        return dispatch_execute_rejected(
            command,
            constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY,
        );
    }

    let generated_at = timestamp_now();
    let read_model =
        app_game_adapter_dispatch_result_read_model_with_execution(&generated_at, None);
    let Some(row) = read_model
        .rows
        .iter()
        .find(|row| row.adapter_dispatch_command_result_claimed)
    else {
        return dispatch_execute_rejected(
            command,
            constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY,
        );
    };
    let mut enforcement_command = command;
    enforcement_command.command = AgentCommandName::AgentEnforcementExecute;
    enforcement_command.payload = scoped_adapter_enforcement_payload();
    let enforcement_event =
        build_enforcement_audit_report_with_paths(enforcement_command, paths).await;
    if enforcement_event.event != AgentEventName::AgentEnforcementAuditReported {
        return enforcement_event;
    }

    match dispatch_execute_result_payload(
        &correlation_id,
        &generated_at,
        row,
        &enforcement_event.payload,
    ) {
        Ok(execute_result) => build_dispatch_executed_event(
            &correlation_id,
            target,
            enforcement_event.payload,
            execute_result,
        ),
        Err(reason) => dispatch_execute_rejected_from_parts(&correlation_id, target, reason),
    }
}

fn build_dispatch_executed_event(
    correlation_id: &str,
    target: ocentra_parent_agent_protocol::transport::AgentPeer,
    mut payload: LogFields,
    execute_result: String,
) -> AgentEventEnvelope {
    payload.insert(
        constants::field::APP_GAME_ADAPTER_DISPATCH_EXECUTE_RESULT.to_string(),
        LogFieldValue::String(execute_result),
    );
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_ADAPTER_DISPATCH_EXECUTED,
        correlation_id,
        target,
        AgentEventName::AgentActivityAppGameAdapterDispatchExecuted,
        LogLevel::Info,
        payload,
        None,
    )
}

fn scoped_adapter_enforcement_payload() -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::POLICY_DECISION_ID,
            LogFieldValue::String(policy_constants::TEST_DECISION_ID.to_string()),
        ),
        (
            constants::field::POLICY_VERSION,
            LogFieldValue::String(policy_constants::TEST_POLICY_VERSION.to_string()),
        ),
        (
            constants::field::POLICY_ACTION,
            LogFieldValue::String(policy_constants::ACTION_BLOCK.to_string()),
        ),
        (
            constants::field::POLICY_TARGET_TYPE,
            LogFieldValue::String(policy_constants::TARGET_TYPE_PROCESS.to_string()),
        ),
        (
            constants::field::TARGET_ID,
            LogFieldValue::String(constants::enforcement::TEST_PROCESS_TARGET_ID.to_string()),
        ),
        (
            constants::field::POLICY_TARGET_VALUE,
            LogFieldValue::String(constants::enforcement::TEST_PROCESS_TARGET_VALUE.to_string()),
        ),
        (
            constants::field::POLICY_DRY_RUN,
            LogFieldValue::Boolean(false),
        ),
        (
            constants::field::POLICY_REASON_CODES,
            LogFieldValue::String(policy_constants::TEST_REASON_PARENT_BLOCK.to_string()),
        ),
        (
            constants::field::POLICY_RULE_IDS,
            LogFieldValue::String(policy_constants::TEST_BLOCK_RULE_ID.to_string()),
        ),
        (
            constants::field::EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(policy_constants::TEST_EVIDENCE_ID.to_string()),
        ),
        (
            constants::field::REQUESTED_AT,
            LogFieldValue::String(policy_constants::TEST_EVALUATED_AT.to_string()),
        ),
        (
            constants::field::EXPIRES_AT,
            LogFieldValue::String(policy_constants::TEST_EXPIRES_AT.to_string()),
        ),
        (
            constants::field::ENFORCEMENT_ACTION_ID,
            LogFieldValue::String(constants::enforcement::TEST_ACTION_ID.to_string()),
        ),
        (
            constants::field::ENFORCEMENT_RESULT_ID,
            LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
        ),
        (
            constants::field::ENFORCEMENT_AUDIT_EVENT_ID,
            LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
        ),
        (
            constants::field::ENFORCEMENT_TIMER_EVENT_ID,
            LogFieldValue::String(constants::enforcement::TEST_TIMER_EVENT_ID.to_string()),
        ),
        (
            constants::field::PROCESS_ID,
            LogFieldValue::Number(f64::from(u32::MAX)),
        ),
    ])
}

fn dispatch_execute_result_payload(
    command_id: &str,
    generated_at: &str,
    row: &AppGameAdapterDispatchResultRow,
    enforcement_payload: &LogFields,
) -> Result<String, &'static str> {
    let mut result = fields_from_pairs(result_identity_pairs(command_id, generated_at, row));
    result.extend(fields_from_pairs(execution_pairs(enforcement_payload)?));
    result.extend(fields_from_pairs(no_claim_pairs()));
    serde_json::to_string(&result).map_err(|_error| constants::error::AGENT_EVENT_SERIALIZES)
}

fn result_identity_pairs(
    command_id: &str,
    generated_at: &str,
    row: &AppGameAdapterDispatchResultRow,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::SCHEMA_VERSION,
            LogFieldValue::Number(f64::from(APP_GAME_SCHEMA_VERSION)),
        ),
        (
            constants::field::COMMAND_ID,
            LogFieldValue::String(command_id.to_string()),
        ),
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(generated_at.to_string()),
        ),
        (
            constants::field::SOURCE_READ_MODEL_ID,
            LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID.to_string()),
        ),
        (
            constants::field::SOURCE_DISPATCH_ROW_ID,
            LogFieldValue::String(row.row_id.clone()),
        ),
        (
            constants::field::SOURCE_PROOF_ENTRY_ID,
            LogFieldValue::String(row.source_proof_entry_id.clone()),
        ),
    ]
}

fn execution_pairs(
    payload: &LogFields,
) -> Result<Vec<(&'static str, LogFieldValue)>, &'static str> {
    let status = required_string(payload, constants::field::ENFORCEMENT_STATUS)?;
    Ok(vec![
        (
            constants::field::EXECUTION_COMMAND_NAME,
            LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND.to_string()),
        ),
        (
            constants::field::EXECUTION_EVENT_NAME,
            LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT.to_string()),
        ),
        (
            constants::field::EXECUTION_RESULT_ID,
            LogFieldValue::String(
                required_string(payload, constants::field::ENFORCEMENT_RESULT_ID)?.to_string(),
            ),
        ),
        (
            constants::field::EXECUTION_STATUS,
            LogFieldValue::String(status.to_string()),
        ),
        (
            constants::field::EXECUTION_ADAPTER_RESULT_CODE,
            LogFieldValue::String(
                required_string(payload, constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE)?
                    .to_string(),
            ),
        ),
        (
            constants::field::EXECUTION_AUDIT_EVENT_ID,
            LogFieldValue::String(
                required_string(payload, constants::field::ENFORCEMENT_AUDIT_EVENT_ID)?.to_string(),
            ),
        ),
        (
            constants::field::READBACK_COMMAND_NAME,
            LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_RESULT_READBACK_COMMAND.to_string()),
        ),
        (
            constants::field::ADAPTER_DISPATCH_EXECUTED_CLAIMED,
            LogFieldValue::Boolean(status == constants::enforcement::RESULT_ACTUALLY_ENFORCED),
        ),
    ])
}

fn no_claim_pairs() -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::BROAD_INSTALLED_APP_BLOCKING_CLAIMED,
            LogFieldValue::Boolean(false),
        ),
        (
            constants::field::CHILD_DEVICE_DELIVERY_CLAIMED,
            LogFieldValue::Boolean(false),
        ),
        (
            constants::field::PLATFORM_ENFORCEMENT_CLAIMED,
            LogFieldValue::Boolean(false),
        ),
        (
            constants::field::PROVIDER_DELIVERY_CLAIMED,
            LogFieldValue::Boolean(false),
        ),
        (
            constants::field::PRIVATE_DIAGNOSTICS_CLAIMED,
            LogFieldValue::Boolean(false),
        ),
    ]
}

fn dispatch_execute_rejected(
    command: AgentCommandEnvelope,
    reason: &'static str,
) -> AgentEventEnvelope {
    dispatch_execute_rejected_from_parts(&command.message_id, command.source, reason)
}

fn dispatch_execute_rejected_from_parts(
    correlation_id: &str,
    target: ocentra_parent_agent_protocol::transport::AgentPeer,
    reason: &'static str,
) -> AgentEventEnvelope {
    build_event(
        constants::event_id::COMMAND_REJECTED,
        correlation_id,
        target,
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(reason.to_string()),
        )]),
        None,
    )
}

fn required_string<'a>(payload: &'a LogFields, field: &str) -> Result<&'a str, &'static str> {
    match payload.get(field) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => Ok(value.trim()),
        _ => Err(constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID),
    }
}
