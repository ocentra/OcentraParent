use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    enforcement_boundary::{authorize_enforcement_boundary, evaluate_enforcement_boundary},
};
use ocentra_parent_agent_protocol::{
    enforcement::AppGameTimerSessionBinding, logging::LogFields, transport::AgentCommandEnvelope,
};

use crate::enforcement_payload::{
    parse_enforcement_command_payload, EnforcementCommandPayload, EnforcementText,
};
use crate::time::timestamp_now;

use super::{
    activity_capture_store_error,
    adapter_outcome::{adapter_outcome_for_request, final_input},
    eventing_audit::record_eventing_enforcement_audit,
    provenance::{record_audit_provenance, EnforcementAuditProvenance},
    record_enforcement_audit,
    rejected_audit::record_rejected_enforcement_audit,
    retry::recover_completed_enforcement_command,
    EnforcementCommandExecutionError, EnforcementJournalBuildError, EnforcementJournalPaths,
};
use crate::enforcement_api::{
    enforcement_pre_action_journal::journal_before_action_outcome,
    enforcement_report_payload::build_enforcement_report_payload,
};

pub(super) async fn execute_enforcement_command(
    command: AgentCommandEnvelope,
    paths: EnforcementJournalPaths,
    provenance: Option<EnforcementAuditProvenance>,
    app_game_session: Option<AppGameTimerSessionBinding>,
) -> Result<LogFields, EnforcementCommandExecutionError> {
    let correlation_id = EnforcementText(command.message_id.clone());
    let sent_at = EnforcementText(command.sent_at.clone());
    let request = parse_enforcement_command_payload(&command, &sent_at)
        .map_err(EnforcementCommandExecutionError::PayloadRejection)?;
    if let Some(recovered) = recover_completed_enforcement_command(
        &correlation_id,
        &sent_at,
        &request,
        &paths,
        provenance,
    )
    .await
    .map_err(EnforcementCommandExecutionError::RetryRecovery)?
    {
        return Ok(recovered.payload);
    }
    execute_new_command(
        correlation_id,
        sent_at,
        request,
        paths,
        provenance,
        app_game_session,
    )
    .await
}

async fn execute_new_command(
    correlation_id: EnforcementText,
    sent_at: EnforcementText,
    request: EnforcementCommandPayload,
    paths: EnforcementJournalPaths,
    provenance: Option<EnforcementAuditProvenance>,
    app_game_session: Option<AppGameTimerSessionBinding>,
) -> Result<LogFields, EnforcementCommandExecutionError> {
    let authorization = authorize(&correlation_id, &sent_at, &request, &paths).await?;
    let before = journal_before_action_outcome(&request, &authorization.action, sent_at.clone());
    record_eventing_enforcement_audit(
        &correlation_id,
        &sent_at,
        &request,
        &before,
        &paths,
        JournalDispatchPhase::BeforeDispatch,
    )
    .await?;
    record_enforcement_audit(&request, &before, &paths, None).await?;
    let completed_at: EnforcementText = timestamp_now();
    let adapter_outcome = adapter_outcome_for_request(
        &request,
        &authorization.action,
        authorization.adapter_request.as_ref(),
        &completed_at,
    )?;
    let mut outcome = evaluate_enforcement_boundary(final_input(
        request.input.clone(),
        adapter_outcome,
        &completed_at,
    ))
    .map_err(EnforcementCommandExecutionError::BoundaryRejection)?;
    let append = record_eventing_enforcement_audit(
        &correlation_id,
        &sent_at,
        &request,
        &outcome,
        &paths,
        JournalDispatchPhase::AfterDispatch,
    )
    .await?;
    outcome.audit_event.journal_sequence = Some(append.sequence.to_string());
    let status = record_enforcement_audit(&request, &outcome, &paths, provenance).await?;
    let active_state = crate::enforcement_timer_state_file::store_active_timer_state_for_outcome_with_app_game_session(
        &outcome,
        &paths.timer_state_path,
        completed_at.0.as_str(),
        app_game_session,
    )
    .await
    .map_err(activity_capture_store_error)?;
    let mut payload = build_enforcement_report_payload(&outcome, &status, active_state.as_ref())
        .map_err(EnforcementCommandExecutionError::Journal)?;
    record_audit_provenance(&mut payload, provenance);
    persist_report_payload(
        &paths,
        &EnforcementText(outcome.audit_event.audit_event_id.clone()),
        &payload,
    )
    .await?;
    Ok(payload)
}

async fn authorize(
    correlation_id: &EnforcementText,
    sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
    paths: &EnforcementJournalPaths,
) -> Result<
    ocentra_parent_agent_core::enforcement_boundary::EnforcementAuthorizationOutcome,
    EnforcementCommandExecutionError,
> {
    match authorize_enforcement_boundary(request.input.clone()) {
        Ok(authorization) => Ok(authorization),
        Err(rejection) => {
            record_rejected_enforcement_audit(
                correlation_id,
                sent_at,
                request,
                rejection,
                sent_at,
                paths,
            )
            .await?;
            Err(EnforcementCommandExecutionError::BoundaryRejection(
                rejection,
            ))
        }
    }
}

async fn persist_report_payload(
    paths: &EnforcementJournalPaths,
    audit_event_id: &EnforcementText,
    payload: &LogFields,
) -> Result<(), EnforcementJournalBuildError> {
    let store_path = paths.store_path.clone();
    let audit_event_id = audit_event_id.0.clone();
    let payload = payload.clone();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(store_path).map_err(activity_capture_store_error)?;
        store
            .replace_enforcement_audit_fields_by_event_id(&audit_event_id, &payload)
            .map_err(activity_capture_store_error)
    })
    .await
    .map_err(activity_capture_store_error)?
}
