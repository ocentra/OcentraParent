use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::fields::fields_from_pairs;

use super::enforcement_command_execution::EnforcementJournalBuildError;

struct FieldPair {
    key: &'static str,
    value: LogFieldValue,
}

pub(super) fn build_enforcement_report_payload(
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
    status: &ActivityIngestStatus,
    active_state: Option<&ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState>,
) -> Result<LogFields, EnforcementJournalBuildError> {
    let mut payload = enforcement_journal_fields(outcome)?;
    for pair in status_field_pairs(status, outcome) {
        payload.insert(pair.key.to_string(), pair.value);
    }
    for pair in timer_status_field_pairs(outcome, active_state)? {
        payload.insert(pair.key.to_string(), pair.value);
    }
    Ok(payload)
}

pub(super) fn enforcement_journal_fields(
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
) -> Result<LogFields, EnforcementJournalBuildError> {
    let mut fields = base_enforcement_field_pairs(outcome);
    fields.extend(serialized_enforcement_field_pairs(outcome)?);
    fields.extend(timer_enforcement_field_pairs(outcome)?);
    Ok(fields_from_pairs(
        fields
            .into_iter()
            .map(|pair| (pair.key, pair.value))
            .collect(),
    ))
}

fn base_enforcement_field_pairs(
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
) -> Vec<FieldPair> {
    vec![
        FieldPair {
            key: constants::field::POLICY_DECISION_ID,
            value: LogFieldValue::String(outcome.action.policy_decision_id.clone()),
        },
        FieldPair {
            key: constants::field::POLICY_ACTION,
            value: LogFieldValue::String(
                outcome.action.policy_action.as_protocol_str().to_string(),
            ),
        },
        FieldPair {
            key: constants::field::POLICY_TARGET_TYPE,
            value: LogFieldValue::String(
                outcome
                    .action
                    .target
                    .target_type
                    .as_protocol_str()
                    .to_string(),
            ),
        },
        FieldPair {
            key: constants::field::POLICY_TARGET_VALUE,
            value: LogFieldValue::String(outcome.action.target.target_value.clone()),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_ACTION_ID,
            value: LogFieldValue::String(outcome.action.action_id.clone()),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_RESULT_ID,
            value: LogFieldValue::String(outcome.result.result_id.clone()),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_AUDIT_EVENT_ID,
            value: LogFieldValue::String(outcome.audit_event.audit_event_id.clone()),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_STATUS,
            value: LogFieldValue::String(outcome.result.status.as_protocol_str().to_string()),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE,
            value: LogFieldValue::String(
                outcome
                    .result
                    .adapter_result_code
                    .as_protocol_str()
                    .to_string(),
            ),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_ROLLBACK_STATE,
            value: LogFieldValue::String(
                outcome.result.rollback_state.as_protocol_str().to_string(),
            ),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_CAPABILITY_STATE,
            value: LogFieldValue::String(
                outcome
                    .result
                    .capability
                    .capability_state
                    .as_protocol_str()
                    .to_string(),
            ),
        },
        FieldPair {
            key: constants::field::EVIDENCE_REFERENCE_IDS,
            value: evidence_reference_ids(outcome),
        },
    ]
}

fn serialized_enforcement_field_pairs(
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
) -> Result<Vec<FieldPair>, EnforcementJournalBuildError> {
    Ok(vec![
        FieldPair {
            key: constants::field::ENFORCEMENT_ACTION,
            value: LogFieldValue::String(
                serde_json::to_string(&outcome.action).map_err(agent_event_serializes_error)?,
            ),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_RESULT,
            value: LogFieldValue::String(
                serde_json::to_string(&outcome.result).map_err(agent_event_serializes_error)?,
            ),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_AUDIT_EVENT,
            value: LogFieldValue::String(
                serde_json::to_string(&outcome.audit_event)
                    .map_err(agent_event_serializes_error)?,
            ),
        },
    ])
}

fn timer_enforcement_field_pairs(
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
) -> Result<Vec<FieldPair>, EnforcementJournalBuildError> {
    Ok(vec![FieldPair {
        key: constants::field::ENFORCEMENT_TIMER_EVENT,
        value: optional_timer_event(outcome)?,
    }])
}

fn status_field_pairs(
    status: &ActivityIngestStatus,
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
) -> Vec<FieldPair> {
    vec![
        FieldPair {
            key: constants::field::DATABASE_READY,
            value: LogFieldValue::Boolean(status.database_ready),
        },
        FieldPair {
            key: constants::field::EVENTS_INGESTED,
            value: LogFieldValue::Number(status.events_ingested as f64),
        },
        FieldPair {
            key: constants::field::EVENTS_STORED,
            value: LogFieldValue::Number(status.events_stored as f64),
        },
        FieldPair {
            key: constants::field::ENFORCEMENT_JOURNAL_EVENT_ID,
            value: LogFieldValue::String(outcome.audit_event.audit_event_id.clone()),
        },
    ]
}

fn timer_status_field_pairs(
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
    active_state: Option<&ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState>,
) -> Result<Vec<FieldPair>, EnforcementJournalBuildError> {
    let mut fields = Vec::new();
    if let Some(timer) = &outcome.timer_event {
        fields.push(FieldPair {
            key: constants::field::ENFORCEMENT_TIMER_EVENT_ID,
            value: LogFieldValue::String(timer.timer_event_id.clone()),
        });
        fields.push(FieldPair {
            key: constants::field::ENFORCEMENT_TIMER_EVENT_KIND,
            value: LogFieldValue::String(timer.timer_event_kind.as_protocol_str().to_string()),
        });
    }
    fields.push(FieldPair {
        key: constants::field::ENFORCEMENT_TIMER_STATE,
        value: optional_timer_state(active_state)?,
    });
    Ok(fields)
}

fn optional_timer_event(
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
) -> Result<LogFieldValue, EnforcementJournalBuildError> {
    match &outcome.timer_event {
        Some(timer) => Ok(LogFieldValue::String(
            serde_json::to_string(timer).map_err(agent_event_serializes_error)?,
        )),
        None => Ok(LogFieldValue::Null(())),
    }
}

fn optional_timer_state(
    active_state: Option<&ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState>,
) -> Result<LogFieldValue, EnforcementJournalBuildError> {
    match active_state {
        Some(state) => Ok(LogFieldValue::String(
            serde_json::to_string(state).map_err(agent_event_serializes_error)?,
        )),
        None => Ok(LogFieldValue::Null(())),
    }
}

fn evidence_reference_ids(
    outcome: &ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome,
) -> LogFieldValue {
    let mut separator = [0; 4];
    LogFieldValue::String(
        outcome
            .action
            .evidence_references
            .iter()
            .map(|reference| reference.evidence_reference_id.as_str())
            .collect::<Vec<_>>()
            .join(constants::delimiter::LIST.encode_utf8(&mut separator)),
    )
}

fn agent_event_serializes_error(_: serde_json::Error) -> EnforcementJournalBuildError {
    EnforcementJournalBuildError::Serialize
}
