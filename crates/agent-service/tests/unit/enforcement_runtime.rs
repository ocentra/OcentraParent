#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/command_dispatch_test_support.rs"]
pub mod test_support;
#[path = "../support/test_text.rs"]
mod test_text;

#[path = "../support/activity_capture/mod.rs"]
mod activity_capture;
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/dev_log.rs"]
mod dev_log;
#[path = "../../src/enforcement_api/enforcement_broad_adapter_proof_read_model.rs"]
mod enforcement_broad_adapter_proof_read_model;
#[path = "enforcement_broad_adapter_proof_read_model_tests.rs"]
mod enforcement_broad_adapter_proof_read_model_tests;
#[path = "../../src/enforcement_browser_domain_adapter_app_control_proof_states.rs"]
mod enforcement_browser_domain_adapter_app_control_proof_states;
#[path = "../../src/enforcement_browser_domain_adapter_proof_read_model.rs"]
mod enforcement_browser_domain_adapter_proof_read_model;
#[path = "enforcement_browser_domain_adapter_proof_read_model_tests.rs"]
mod enforcement_browser_domain_adapter_proof_read_model_tests;
#[path = "../../src/enforcement_capability.rs"]
mod enforcement_capability;
#[path = "../../src/enforcement_cross_platform_capability_proof_read_model.rs"]
mod enforcement_cross_platform_capability_proof_read_model;
#[path = "enforcement_cross_platform_capability_proof_read_model_tests.rs"]
mod enforcement_cross_platform_capability_proof_read_model_tests;
#[path = "enforcement_integrity_runtime_audit_proof.rs"]
mod enforcement_integrity_runtime_audit_proof;
#[path = "../../src/enforcement_api/enforcement_integrity_runtime_audit_read_model.rs"]
mod enforcement_integrity_runtime_audit_read_model;
#[path = "enforcement_integrity_runtime_audit_read_model_tests.rs"]
mod enforcement_integrity_runtime_audit_read_model_tests;
#[path = "../../src/enforcement_os_adapter_product_proof_read_model.rs"]
mod enforcement_os_adapter_product_proof_read_model;
#[path = "enforcement_os_adapter_product_proof_read_model_tests.rs"]
mod enforcement_os_adapter_product_proof_read_model_tests;
#[path = "../../src/enforcement_payload.rs"]
mod enforcement_payload;
#[path = "../../src/enforcement_policy_dispatch_read_model.rs"]
mod enforcement_policy_dispatch_read_model;
#[path = "enforcement_policy_dispatch_read_model_tests.rs"]
mod enforcement_policy_dispatch_read_model_tests;
#[path = "../../src/enforcement_api/enforcement_pre_action_journal.rs"]
mod enforcement_pre_action_journal;
#[path = "../../src/enforcement_api/enforcement_supported_adapter_runtime_proof_read_model.rs"]
mod enforcement_supported_adapter_runtime_proof_read_model;
#[path = "enforcement_supported_adapter_runtime_proof_read_model_tests.rs"]
mod enforcement_supported_adapter_runtime_proof_read_model_tests;
#[path = "../../src/enforcement_timer_api.rs"]
mod enforcement_timer_api;
#[path = "enforcement_timer_expiry_tests.rs"]
mod enforcement_timer_expiry_tests;
#[path = "../../src/enforcement_timer_payload.rs"]
mod enforcement_timer_payload;
#[path = "../../src/enforcement_timer_report.rs"]
mod enforcement_timer_report;
#[path = "../../src/enforcement_timer_state_file.rs"]
mod enforcement_timer_state_file;
#[path = "enforcement_timer_tests.rs"]
mod enforcement_timer_tests;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/host_identity_read_model.rs"]
mod host_identity_read_model;
#[path = "../../src/enforcement_api/integrity_alert_status_bridge_read_model.rs"]
mod integrity_alert_status_bridge_read_model;
#[path = "integrity_alert_status_bridge_read_model_tests.rs"]
mod integrity_alert_status_bridge_read_model_tests;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../../src/enforcement_api/notification_provider_status_boundary_read_model.rs"]
mod notification_provider_status_boundary_read_model;
#[path = "notification_provider_status_boundary_read_model_tests.rs"]
mod notification_provider_status_boundary_read_model_tests;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../../src/time.rs"]
mod time;
#[path = "../../src/windows_adapter_artifact_gate_read_model.rs"]
mod windows_adapter_artifact_gate_read_model;
#[path = "../../src/windows_adapter_capability_read_model.rs"]
mod windows_adapter_capability_read_model;

mod enforcement_api {
    use std::{env, path::PathBuf};

    use ocentra_parent_agent_core::{
        enforcement_adapter::{
            terminate_owned_process, EnforcementAdapterOutcome, OwnedProcessTerminationTarget,
        },
        enforcement_boundary::{
            authorize_enforcement_boundary, evaluate_enforcement_boundary,
            EnforcementBoundaryInput, EnforcementBoundaryOutcome,
        },
    };
    use ocentra_parent_agent_protocol::activity::ActivityEvent;
    use ocentra_parent_agent_protocol::activity::ActivityEventKind;
    use ocentra_parent_agent_protocol::activity::ActivityObserver;
    use ocentra_parent_agent_protocol::activity::ActivitySource;
    use ocentra_parent_agent_protocol::activity::ActivitySubject;
    use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
    use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
    use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::enforcement::EnforcementAdapterKind;
    use ocentra_parent_agent_protocol::enforcement::EnforcementMode;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use ocentra_parent_agent_protocol::logging::LogFields;
    use ocentra_parent_agent_protocol::logging::LogLevel;
    use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
    use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
    use ocentra_parent_agent_protocol::transport::AgentEventName;

    use crate::activity_capture::record_activity_events_to_paths;
    use crate::enforcement_payload::{
        parse_enforcement_command_payload, EnforcementCommandPayload,
    };
    use crate::enforcement_pre_action_journal::journal_before_action_outcome;
    use crate::enforcement_timer_state_file::store_active_timer_state_for_outcome;
    use crate::event_builder::build_event;
    use crate::fields::fields_from_pairs;
    use crate::test_text::TestText;
    use crate::time::timestamp_now;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(crate) struct EnforcementJournalPaths {
        pub journal_path: PathBuf,
        pub key_path: PathBuf,
        pub store_path: PathBuf,
        pub timer_state_path: PathBuf,
    }

    impl EnforcementJournalPaths {
        pub(crate) fn from_environment() -> Self {
            Self {
                journal_path: crate::activity_store_path::activity_journal_path(),
                key_path: crate::activity_store_path::activity_journal_key_path(),
                store_path: crate::activity_store_path::activity_db_path(),
                timer_state_path: env::var(constants::env_var::AGENT_ENFORCEMENT_TIMER_STATE_PATH)
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| {
                        let mut path = env::temp_dir();
                        path.push(constants::enforcement::TIMER_STATE_FILE_NAME);
                        path
                    }),
            }
        }
    }

    pub(crate) async fn build_enforcement_audit_report_with_paths(
        command: AgentCommandEnvelope,
        paths: EnforcementJournalPaths,
    ) -> AgentEventEnvelope {
        let target = command.source.clone();
        let correlation_id = command.message_id.clone();
        match execute_enforcement_command(command, paths).await {
            Ok(payload) => build_event(
                constants::event_id::ENFORCEMENT_AUDIT_REPORTED,
                &correlation_id,
                target,
                AgentEventName::AgentEnforcementAuditReported,
                LogLevel::Info,
                payload,
                None,
            ),
            Err(reason) => build_event(
                constants::event_id::COMMAND_REJECTED,
                &correlation_id,
                target,
                AgentEventName::AgentCommandRejected,
                LogLevel::Warn,
                fields_from_pairs(vec![(
                    constants::field::REASON,
                    LogFieldValue::String(reason.to_string()),
                )]),
                None,
            ),
        }
    }

    async fn execute_enforcement_command(
        command: AgentCommandEnvelope,
        paths: EnforcementJournalPaths,
    ) -> Result<LogFields, TestText> {
        let observed_at = TestText::from_display(timestamp_now());
        let request = parse_enforcement_command_payload(&command, &observed_at.to_string())
            .map_err(TestText::from_display)?;
        let authorization = authorize_enforcement_boundary(request.input.clone())
            .map_err(|error| TestText::from_display(error.as_protocol_str()))?;
        let before_action_outcome = journal_before_action_outcome(
            &request,
            &authorization.action,
            &observed_at.to_string(),
        );
        record_enforcement_audit(&request, &before_action_outcome, &paths).await?;
        let completed_at = TestText::from_display(timestamp_now());
        let adapter_outcome = match authorization.adapter_request.as_ref() {
            Some(adapter_request) => Some(adapter_outcome_for_request(
                &request,
                &authorization.action,
                adapter_request.adapter_kind,
                adapter_request.mode,
                &completed_at,
            )?),
            None => None,
        };
        let outcome_input = final_input(request.input.clone(), adapter_outcome, &completed_at);
        let mut outcome = evaluate_enforcement_boundary(outcome_input)
            .map_err(|error| TestText::from_display(error.as_protocol_str()))?;
        outcome.audit_event.journal_sequence = Some(outcome.audit_event.audit_event_id.clone());
        let status = record_enforcement_audit(&request, &outcome, &paths).await?;
        let active_state = store_active_timer_state_for_outcome(
            &outcome,
            &paths.timer_state_path,
            &completed_at.to_string(),
        )
        .await
        .map_err(TestText::from_display)?;

        enforcement_report_payload(&outcome, &status, active_state.as_ref())
    }

    fn adapter_outcome_for_request(
        request: &EnforcementCommandPayload,
        action: &ocentra_parent_agent_protocol::enforcement::EnforcementAction,
        adapter_kind: EnforcementAdapterKind,
        mode: EnforcementMode,
        completed_at: &TestText,
    ) -> Result<EnforcementAdapterOutcome, TestText> {
        match (adapter_kind, mode) {
            (EnforcementAdapterKind::ProcessControl, EnforcementMode::TerminateProcess) => {
                let pid = request.process_id.ok_or_else(|| {
                    TestText::from_display(constants::enforcement::REJECTION_PROCESS_ID_REQUIRED)
                })?;
                Ok(terminate_owned_process(
                    OwnedProcessTerminationTarget {
                        pid,
                        expected_process_name: action.target.target_value.clone(),
                    },
                    &completed_at.to_string(),
                ))
            }
            _ => Err(TestText::from_display(
                constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY,
            )),
        }
    }

    fn final_input(
        mut input: EnforcementBoundaryInput,
        adapter_outcome: Option<EnforcementAdapterOutcome>,
        completed_at: &TestText,
    ) -> EnforcementBoundaryInput {
        input.completed_at = Some(completed_at.to_string());
        input.adapter_outcome = adapter_outcome;
        input
    }

    async fn record_enforcement_audit(
        request: &EnforcementCommandPayload,
        outcome: &EnforcementBoundaryOutcome,
        paths: &EnforcementJournalPaths,
    ) -> Result<ActivityIngestStatus, TestText> {
        let event = enforcement_activity_event(request, outcome)?;
        let journal_path = paths.journal_path.clone();
        let key_path = paths.key_path.clone();
        let store_path = paths.store_path.clone();
        tokio::task::spawn_blocking(move || {
            record_activity_events_to_paths(&journal_path, &key_path, &store_path, &[event])
        })
        .await
        .map_err(|_| TestText::from_display(constants::value::ACTIVITY_CAPTURE_STORE_ERROR))?
        .map_err(|_| TestText::from_display(constants::value::ACTIVITY_CAPTURE_STORE_ERROR))
    }

    fn enforcement_activity_event(
        request: &EnforcementCommandPayload,
        outcome: &EnforcementBoundaryOutcome,
    ) -> Result<ActivityEvent, TestText> {
        Ok(ActivityEvent {
            schema_version: ACTIVITY_SCHEMA_VERSION,
            event_id: outcome.audit_event.audit_event_id.clone(),
            observed_at: outcome.audit_event.observed_at.clone(),
            source: ActivitySource {
                device_id: request.device_id.clone(),
                platform: request.platform.clone(),
                observer: ActivityObserver::AgentService,
                source_id: constants::enforcement::SOURCE_ID_AGENT_SERVICE.to_string(),
            },
            kind: ActivityEventKind::EnforcementAuditRecorded,
            subject: ActivitySubject {
                kind: ActivitySubjectKind::Intervention,
                subject_id: outcome.action.action_id.clone(),
                display_name: Some(outcome.action.mode.as_protocol_str().to_string()),
            },
            fields: enforcement_journal_fields(outcome)?,
            evidence: Vec::new(),
        })
    }

    fn enforcement_journal_fields(
        outcome: &EnforcementBoundaryOutcome,
    ) -> Result<LogFields, TestText> {
        let mut fields = LogFields::new();
        fields.insert(
            constants::field::POLICY_DECISION_ID.to_string(),
            LogFieldValue::String(outcome.action.policy_decision_id.clone()),
        );
        fields.insert(
            constants::field::POLICY_ACTION.to_string(),
            LogFieldValue::String(outcome.action.policy_action.as_protocol_str().to_string()),
        );
        fields.insert(
            constants::field::POLICY_TARGET_TYPE.to_string(),
            LogFieldValue::String(
                outcome
                    .action
                    .target
                    .target_type
                    .as_protocol_str()
                    .to_string(),
            ),
        );
        fields.insert(
            constants::field::POLICY_TARGET_VALUE.to_string(),
            LogFieldValue::String(outcome.action.target.target_value.clone()),
        );
        fields.insert(
            constants::field::ENFORCEMENT_ACTION_ID.to_string(),
            LogFieldValue::String(outcome.action.action_id.clone()),
        );
        fields.insert(
            constants::field::ENFORCEMENT_RESULT_ID.to_string(),
            LogFieldValue::String(outcome.result.result_id.clone()),
        );
        fields.insert(
            constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
            LogFieldValue::String(outcome.audit_event.audit_event_id.clone()),
        );
        fields.insert(
            constants::field::ENFORCEMENT_STATUS.to_string(),
            LogFieldValue::String(outcome.result.status.as_protocol_str().to_string()),
        );
        fields.insert(
            constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE.to_string(),
            LogFieldValue::String(
                outcome
                    .result
                    .adapter_result_code
                    .as_protocol_str()
                    .to_string(),
            ),
        );
        fields.insert(
            constants::field::ENFORCEMENT_ROLLBACK_STATE.to_string(),
            LogFieldValue::String(outcome.result.rollback_state.as_protocol_str().to_string()),
        );
        fields.insert(
            constants::field::ENFORCEMENT_CAPABILITY_STATE.to_string(),
            LogFieldValue::String(
                outcome
                    .result
                    .capability
                    .capability_state
                    .as_protocol_str()
                    .to_string(),
            ),
        );
        fields.insert(
            constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
            LogFieldValue::String(evidence_reference_ids(outcome).to_string()),
        );
        fields.insert(
            constants::field::ENFORCEMENT_ACTION.to_string(),
            LogFieldValue::String(
                serde_json::to_string(&outcome.action).map_err(|_| {
                    TestText::from_display(constants::error::AGENT_EVENT_SERIALIZES)
                })?,
            ),
        );
        fields.insert(
            constants::field::ENFORCEMENT_RESULT.to_string(),
            LogFieldValue::String(
                serde_json::to_string(&outcome.result).map_err(|_| {
                    TestText::from_display(constants::error::AGENT_EVENT_SERIALIZES)
                })?,
            ),
        );
        fields.insert(
            constants::field::ENFORCEMENT_AUDIT_EVENT.to_string(),
            LogFieldValue::String(
                serde_json::to_string(&outcome.audit_event).map_err(|_| {
                    TestText::from_display(constants::error::AGENT_EVENT_SERIALIZES)
                })?,
            ),
        );
        fields.insert(
            constants::field::ENFORCEMENT_TIMER_EVENT.to_string(),
            optional_timer_event(outcome)?,
        );
        Ok(fields)
    }

    fn enforcement_report_payload(
        outcome: &EnforcementBoundaryOutcome,
        status: &ActivityIngestStatus,
        active_state: Option<
            &ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState,
        >,
    ) -> Result<LogFields, TestText> {
        let mut payload = enforcement_journal_fields(outcome)?;
        let last_event_id = status.last_event_id.as_ref().map(TestText::from_display);
        payload.insert(
            constants::field::DATABASE_READY.to_string(),
            LogFieldValue::Boolean(status.database_ready),
        );
        payload.insert(
            constants::field::EVENTS_INGESTED.to_string(),
            LogFieldValue::Number(status.events_ingested as f64),
        );
        payload.insert(
            constants::field::EVENTS_STORED.to_string(),
            LogFieldValue::Number(status.events_stored as f64),
        );
        payload.insert(
            constants::field::ENFORCEMENT_JOURNAL_EVENT_ID.to_string(),
            optional_string_value(last_event_id.as_ref()),
        );
        if let Some(timer) = &outcome.timer_event {
            payload.insert(
                constants::field::ENFORCEMENT_TIMER_EVENT_ID.to_string(),
                LogFieldValue::String(timer.timer_event_id.clone()),
            );
            payload.insert(
                constants::field::ENFORCEMENT_TIMER_EVENT_KIND.to_string(),
                LogFieldValue::String(timer.timer_event_kind.as_protocol_str().to_string()),
            );
        }
        payload.insert(
            constants::field::ENFORCEMENT_TIMER_STATE.to_string(),
            optional_timer_state(active_state)?,
        );
        Ok(payload)
    }

    fn optional_timer_event(
        outcome: &EnforcementBoundaryOutcome,
    ) -> Result<LogFieldValue, TestText> {
        match &outcome.timer_event {
            Some(timer) => Ok(LogFieldValue::String(
                serde_json::to_string(timer).map_err(|_| {
                    TestText::from_display(constants::error::AGENT_EVENT_SERIALIZES)
                })?,
            )),
            None => Ok(LogFieldValue::Null(())),
        }
    }

    fn optional_string_value(value: Option<&TestText>) -> LogFieldValue {
        value
            .map(|item| LogFieldValue::String(item.to_string()))
            .unwrap_or(LogFieldValue::Null(()))
    }

    fn optional_timer_state(
        active_state: Option<
            &ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState,
        >,
    ) -> Result<LogFieldValue, TestText> {
        match active_state {
            Some(state) => Ok(LogFieldValue::String(
                serde_json::to_string(state).map_err(|_| {
                    TestText::from_display(constants::error::AGENT_EVENT_SERIALIZES)
                })?,
            )),
            None => Ok(LogFieldValue::Null(())),
        }
    }

    fn evidence_reference_ids(outcome: &EnforcementBoundaryOutcome) -> TestText {
        let mut separator = [0; 4];
        TestText::from_display(
            outcome
                .action
                .evidence_references
                .iter()
                .map(|reference| reference.evidence_reference_id.as_str().to_string())
                .collect::<Vec<_>>()
                .join(constants::delimiter::LIST.encode_utf8(&mut separator)),
        )
    }
}

#[test]
fn link_runtime_helpers_used_by_the_current_harness() {
    let _ = activity_capture::spawn_startup_activity_capture;
    let _ = activity_capture::startup_activity_capture_enabled;
    let _ = activity_capture::startup_activity_capture_enabled_for_value;
    let _ = activity_capture::record_activity_capture_once;
    let _ = activity_capture::record_activity_capture_to_paths;
    let _ = activity_capture::record_activity_capture_to_paths_at;
    let _ = activity_store_path::activity_db_path;
    let _ = activity_store_path::activity_journal_path;
    let _ = activity_store_path::activity_journal_key_path;
    let _ = dev_log::write_agent_info;
    let _ = dev_log::write_agent_warn;
    let _ = dev_log::write_agent_error;
    let _ = dev_log::write_agent_debug;
    let _ = event_builder::portal_peer();
    let sample_json = serde_json::json!({ "link": true });
    let _ = json_contract::serialize_json_string(&sample_json);
    let _ = json_contract::serialize_json_value(sample_json.clone());
    let decoded: serde_json::Value =
        test_invariants::require_json_decode(&sample_json.to_string(), "link");
    let log_field =
        ocentra_parent_agent_protocol::logging::LogFieldValue::String(String::from("value"));
    let _ = test_invariants::require_log_string_field(Some(&log_field), "link");
    let _ = test_invariants::serialize_test_json(&decoded);
    let _ = enforcement_timer_api::build_enforcement_timer_report;
    let _ = time::timestamp_from_epoch_seconds;
    let _ = time::timestamp_after_epoch_seconds;
}
