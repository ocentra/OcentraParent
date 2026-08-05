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
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use super::app_game_adapter_dispatch_result_payload::app_game_adapter_dispatch_result_read_model;
use super::app_game_adapter_execution_readiness_payload::GeneratedAtText;
use crate::enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths};
use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

#[derive(Clone, Debug, PartialEq, Eq)]
struct DispatchText(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DispatchReason(&'static str);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FieldKey(&'static str);

#[derive(Clone, Debug, PartialEq)]
struct FieldPairs(Vec<(&'static str, LogFieldValue)>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DispatchError(&'static str);

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
            DispatchReason(constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY),
        );
    }

    let generated_at: String = timestamp_now();
    let read_model =
        app_game_adapter_dispatch_result_read_model(GeneratedAtText(generated_at.clone()), None);
    let Some(row) = scoped_dispatch_result_row(&read_model) else {
        return dispatch_execute_rejected(
            command,
            DispatchReason(constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY),
        );
    };
    let mut enforcement_command = command;
    enforcement_command.payload = scoped_adapter_enforcement_payload();
    let enforcement_event =
        build_enforcement_audit_report_with_paths(enforcement_command, paths).await;
    if enforcement_event.event != AgentEventName::AgentEnforcementAuditReported {
        return enforcement_event;
    }

    let execute_result = (|| -> Result<DispatchText, DispatchError> {
        let mut result = result_identity_pairs(
            DispatchText(correlation_id.clone()),
            DispatchText(generated_at.clone()),
            row,
        )
        .0;
        result.extend(execution_pairs(&enforcement_event.payload)?.0);
        result.extend(no_claim_pairs().0);
        serde_json::to_string(&fields_from_pairs(result))
            .map(DispatchText)
            .map_err(|_error| DispatchError(constants::error::AGENT_EVENT_SERIALIZES))
    })();

    match execute_result {
        Ok(execute_result) => build_dispatch_executed_event(
            &DispatchText(correlation_id.to_string()),
            target,
            enforcement_event.payload,
            execute_result,
        ),
        Err(reason) => dispatch_execute_rejected_from_parts(
            &DispatchText(correlation_id.to_string()),
            target,
            DispatchReason(reason.0),
        ),
    }
}

fn scoped_dispatch_result_row(
    read_model: &ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel,
) -> Option<&ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultRow>{
    read_model
        .rows
        .iter()
        .find(|row| row.adapter_dispatch_command_result_claimed)
}

fn build_dispatch_executed_event(
    correlation_id: &DispatchText,
    target: ocentra_parent_agent_protocol::transport::AgentPeer,
    mut payload: LogFields,
    execute_result: DispatchText,
) -> AgentEventEnvelope {
    payload.insert(
        constants::field::APP_GAME_ADAPTER_DISPATCH_EXECUTE_RESULT.to_string(),
        LogFieldValue::String(execute_result.0),
    );
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_ADAPTER_DISPATCH_EXECUTED,
        &correlation_id.0,
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

fn result_identity_pairs(
    command_id: DispatchText,
    generated_at: DispatchText,
    row: &AppGameAdapterDispatchResultRow,
) -> FieldPairs {
    FieldPairs(vec![
        (
            constants::field::SCHEMA_VERSION,
            LogFieldValue::Number(f64::from(APP_GAME_SCHEMA_VERSION)),
        ),
        (
            constants::field::COMMAND_ID,
            LogFieldValue::String(command_id.0),
        ),
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(generated_at.0),
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
    ])
}

fn execution_pairs(payload: &LogFields) -> Result<FieldPairs, DispatchError> {
    let status = required_string(payload, FieldKey(constants::field::ENFORCEMENT_STATUS))?;
    let command_name = match payload.get(constants::field::EXECUTION_COMMAND_NAME) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND,
    };
    Ok(FieldPairs(vec![
        (
            constants::field::EXECUTION_COMMAND_NAME,
            LogFieldValue::String(command_name.to_string()),
        ),
        (
            constants::field::EXECUTION_EVENT_NAME,
            LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT.to_string()),
        ),
        (
            constants::field::EXECUTION_RESULT_ID,
            LogFieldValue::String(
                required_string(payload, FieldKey(constants::field::ENFORCEMENT_RESULT_ID))?.0,
            ),
        ),
        (
            constants::field::EXECUTION_STATUS,
            LogFieldValue::String(status.0.clone()),
        ),
        (
            constants::field::EXECUTION_ADAPTER_RESULT_CODE,
            LogFieldValue::String(
                required_string(
                    payload,
                    FieldKey(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
                )?
                .0,
            ),
        ),
        (
            constants::field::EXECUTION_AUDIT_EVENT_ID,
            LogFieldValue::String(
                required_string(
                    payload,
                    FieldKey(constants::field::ENFORCEMENT_AUDIT_EVENT_ID),
                )?
                .0,
            ),
        ),
        (
            constants::field::READBACK_COMMAND_NAME,
            LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_RESULT_READBACK_COMMAND.to_string()),
        ),
        (
            constants::field::ADAPTER_DISPATCH_EXECUTED_CLAIMED,
            LogFieldValue::Boolean(
                status.0.as_str() == constants::enforcement::RESULT_ACTUALLY_ENFORCED,
            ),
        ),
    ]))
}

fn no_claim_pairs() -> FieldPairs {
    FieldPairs(vec![
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
    ])
}

fn dispatch_execute_rejected(
    command: AgentCommandEnvelope,
    reason: DispatchReason,
) -> AgentEventEnvelope {
    dispatch_execute_rejected_from_parts(
        &DispatchText(command.message_id.to_string()),
        command.source,
        reason,
    )
}

fn dispatch_execute_rejected_from_parts(
    correlation_id: &DispatchText,
    target: ocentra_parent_agent_protocol::transport::AgentPeer,
    reason: DispatchReason,
) -> AgentEventEnvelope {
    build_event(
        constants::event_id::COMMAND_REJECTED,
        &correlation_id.0,
        target,
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(reason.0.to_string()),
        )]),
        None,
    )
}

fn required_string(payload: &LogFields, field: FieldKey) -> Result<DispatchText, DispatchError> {
    match payload.get(field.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Ok(DispatchText(value.trim().to_string()))
        }
        _ => Err(DispatchError(
            constants::enforcement::REJECTION_COMMAND_PAYLOAD_INVALID,
        )),
    }
}
